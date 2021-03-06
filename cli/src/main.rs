use anyhow::{bail, Context, Result};
use clap::Parser;
use futures_util::stream::{StreamExt, TryStreamExt};
use rand::{thread_rng, Rng};
use std::{
	env,
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicU64, AtomicUsize, Ordering},
		Arc,
	},
	time::{Duration, Instant},
};
use tokio::fs;

const CONCURRENT_UPLOADS: usize = 8;

#[derive(Parser)]
#[clap()]
struct Opts {
	#[clap(subcommand)]
	subcmd: SubCommand,

	#[clap(long, env = "RIVET_CLOUD_API_URL")]
	api_url: Option<String>,

	#[clap(long, env = "RIVET_ACCESS_TOKEN")]
	access_token: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
	Auth {
		#[clap(subcommand)]
		subcmd: AuthSubCommand,
	},
	Build {
		#[clap(subcommand)]
		subcmd: BuildSubCommand,
	},
	Site {
		#[clap(subcommand)]
		subcmd: SiteSubCommand,
	},
}

#[derive(Parser)]
enum AuthSubCommand {
	Token,
}

#[derive(Parser)]
enum BuildSubCommand {
	Push(BuildPushOpts),
}

#[derive(Parser)]
struct BuildPushOpts {
	#[clap(index(1))]
	tag: String,

	#[clap(long)]
	name: Option<String>,
}

#[derive(Parser)]
enum SiteSubCommand {
	Push(SitePushOptions),
}

#[derive(Parser)]
struct SitePushOptions {
	#[clap(index(1))]
	path: String,

	#[clap(long)]
	name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
	let opts = Opts::parse();

	// Read config
	let config_path = home::home_dir()
		.context("missing home dir")?
		.join(".config")
		.join("rivetctl.json");
	let config = match fs::read(&config_path).await {
		Result::Ok(buf) => serde_json::from_slice::<rivetctl::config::Config>(buf.as_slice())?,
		Result::Err(_) => rivetctl::config::Config::default(),
	};

	// Build ctx
	let ctx = rivetctl::ctx::SharedCtx::new(
		config.clone(),
		opts.api_url.clone(),
		opts.access_token.clone(),
	)
	.await?;

	let client = Arc::new(reqwest::Client::new());

	match opts.subcmd {
		SubCommand::Auth { subcmd } => match subcmd {
			AuthSubCommand::Token { .. } => {
				print!("Auth token: ");

				// Read token from stdin
				let token = tokio::task::spawn_blocking(|| {
					use std::io::BufRead;

					let stdin = std::io::stdin();
					let mut iterator = stdin.lock().lines();
					iterator.next().unwrap().context("token not provided")
				})
				.await??;

				// Create new config
				let mut new_config = config.clone();
				new_config.auth.token = Some(token.trim().to_owned());

				// Create new context without overridden access token to check the token
				let new_ctx =
					rivetctl::ctx::SharedCtx::new(new_config.clone(), opts.api_url.clone(), None)
						.await?;
				let inspect = new_ctx
					.http_client
					.inspect()
					.send()
					.await
					.context("http_client.inspect")?;
				println!("{:?}", inspect);

				// Save new config
				write_config(&new_config, &config_path).await?;
			}
		},
		SubCommand::Build { subcmd } => match subcmd {
			BuildSubCommand::Push(push_opts) => {
				let game_id = infer_game_id(&ctx).await?;

				let tmp_image_file = tempfile::NamedTempFile::new()?;
				let tmp_path = tmp_image_file.into_temp_path();

				// Re-tag and archive the image
				let image_tag_tag = thread_rng()
					.sample_iter(rand::distributions::Alphanumeric)
					.map(char::from)
					.take(16)
					.collect::<String>()
					.to_lowercase();
				let image_tag = format!("rivet-game:{}", image_tag_tag);
				println!("\n\n> Archiving image");
				let tag_cmd = tokio::process::Command::new("docker")
					.arg("image")
					.arg("tag")
					.arg(&push_opts.tag)
					.arg(&image_tag)
					.output()
					.await?;
				if !tag_cmd.status.success() {
					eprintln!("  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}", tag_cmd.status, String::from_utf8_lossy(&tag_cmd.stdout), String::from_utf8_lossy(&tag_cmd.stderr));
					bail!("failed to tag docker image");
				}

				let save_cmd = tokio::process::Command::new("docker")
					.arg("image")
					.arg("save")
					.arg("--output")
					.arg(&tmp_path)
					.arg(&image_tag)
					.output()
					.await?;
				if !save_cmd.status.success() {
					eprintln!("  ! Failed to archive Docker image:\n\nStatus: {}\n\nStdout:\n{}\n\nStderr:\n{}", save_cmd.status, String::from_utf8_lossy(&save_cmd.stdout), String::from_utf8_lossy(&save_cmd.stderr));
					bail!("failed to save docker image");
				}

				// Inspect the image
				let image_file_meta = fs::metadata(&tmp_path).await?;

				// Create build
				let display_name = push_opts
					.name
					.clone()
					.unwrap_or_else(|| push_opts.tag.clone());
				let content_type = "application/x-tar";
				println!(
					"\n\n> Creating build \"{name}\" ({size})",
					name = display_name,
					size = format_file_size(image_file_meta.len())?,
				);
				let build_res = ctx
					.http_client
					.create_game_build()
					.game_id(&game_id)
					.display_name(&display_name)
					.image_tag(&image_tag)
					.image_file(
						rivetctl::model::upload_prepare_file::Builder::default()
							.path("image.tar")
							.content_type(content_type)
							.content_length(image_file_meta.len() as i64)
							.build(),
					)
					.send()
					.await
					.context("http_client.create_game_build")?;

				println!(
					"\n\n> Uploading ({size})",
					size = format_file_size(image_file_meta.len())?,
				);
				upload_file(
					&client,
					&build_res.image_presigned_request().unwrap(),
					tmp_path,
					Some(content_type),
				)
				.await?;

				println!("\n\n> Completing");
				ctx.http_client
					.complete_upload()
					.upload_id(build_res.upload_id().unwrap())
					.send()
					.await
					.context("http_client.complete_upload")?;
			}
		},
		SubCommand::Site { subcmd } => match subcmd {
			SiteSubCommand::Push(push_opts) => {
				let game_id = infer_game_id(&ctx).await?;

				let upload_path = env::current_dir()?.join(push_opts.path);

				let display_name = push_opts.name.clone().unwrap_or_else(|| {
					upload_path
						.file_name()
						.and_then(|n| n.to_str())
						.map(str::to_owned)
						.unwrap_or_else(|| "Site".to_owned())
				});
				println!("\n\n> Creating site \"{}\"", display_name);
				println!("  * Upload path: {}", upload_path.display());

				// Index the directory
				let files = {
					let upload_path = upload_path.clone();
					tokio::task::spawn_blocking(move || prepare_upload_dir(&upload_path))
				}
				.await??;
				let total_len = files
					.iter()
					.fold(0, |acc, x| acc + x.prepared.content_length().unwrap());
				println!(
					"  * Found {count} files ({size})",
					count = files.len(),
					size = format_file_size(total_len as u64)?,
				);

				// Create site
				let site_res = ctx
					.http_client
					.create_game_cdn_site()
					.game_id(&game_id)
					.display_name(&display_name)
					.set_files(Some(files.iter().map(|f| f.prepared.clone()).collect()))
					.send()
					.await
					.context("http_client.create_game_cdn_site")?;

				println!("\n\n> Uploading");
				{
					let counter = Arc::new(AtomicUsize::new(0));
					let counter_bytes = Arc::new(AtomicU64::new(0));
					let presigned_requests = site_res.presigned_requests().unwrap();
					let total = presigned_requests.len();
					let total_bytes = total_len as u64;

					let files = Arc::new(files.clone());
					futures_util::stream::iter(presigned_requests)
						.map(Ok)
						.try_for_each_concurrent(CONCURRENT_UPLOADS, move |presigned_req| {
							let counter = counter.clone();
							let counter_bytes = counter_bytes.clone();
							{
								let files = files.clone();
								let client = client.clone();

								async move {
									// Find the matching prepared file
									let file = files
										.iter()
										.find(|f| f.prepared.path == presigned_req.path)
										.context("missing prepared file")?;

									upload_file(
										&client,
										&presigned_req,
										&file.absolute_path,
										file.prepared.content_type.as_ref(),
									)
									.await?;

									let progress = counter.fetch_add(1, Ordering::SeqCst) + 1;
									let progress_bytes = counter_bytes.fetch_add(
										file.prepared.content_length().unwrap() as u64,
										Ordering::SeqCst,
									) + file.prepared.content_length().unwrap()
										as u64;
									println!(
										"    {}/{} files ({}/{})",
										progress,
										total,
										format_file_size(progress_bytes)?,
										format_file_size(total_bytes)?
									);

									Result::<()>::Ok(())
								}
							}
						})
						.await?;
				}

				println!("\n\n> Completing");
				ctx.http_client
					.complete_upload()
					.upload_id(site_res.upload_id().unwrap())
					.send()
					.await
					.context("http_client.complete_upload")?;
			}
		},
	}

	Ok(())
}

/// Writes a modified config to the file system.
async fn write_config(config: &rivetctl::config::Config, path: &Path) -> Result<()> {
	// Create parent directory
	fs::create_dir_all(&path.parent().context("config path parent")?).await?;

	// Write config
	let config_str = serde_json::to_string(config)?;
	fs::write(path, config_str).await?;

	Ok(())
}

/// Prepared file that will be uploaded to S3.
#[derive(Clone)]
struct UploadFile {
	absolute_path: PathBuf,
	prepared: rivetctl::model::UploadPrepareFile,
}

/// Lists all files in a directory and returns the data required to upload them.
fn prepare_upload_dir(base_path: &Path) -> Result<Vec<UploadFile>> {
	use std::path::Component;

	let mut files = Vec::<UploadFile>::new();

	// Walk files while respecting .rivet-cdn-ignore
	let walk = ignore::WalkBuilder::new(base_path)
		.standard_filters(false)
		.add_custom_ignore_filename(".rivet-cdn-ignore")
		.parents(true)
		.build();
	for entry in walk {
		let entry = entry?;
		let file_path = entry.path();
		let file_meta = entry.metadata()?;

		if file_meta.is_file() {
			// Convert path to Unix-style string
			let path_str = entry
				.path()
				.strip_prefix(base_path)?
				.components()
				.filter_map(|c| match c {
					Component::Normal(name) => name.to_str().map(str::to_string),
					_ => None,
				})
				.collect::<Vec<String>>()
				.join("/");

			// Attempt to guess the MIME type
			let content_type = mime_guess::from_path(&file_path)
				.first_raw()
				.map(str::to_string);

			files.push(UploadFile {
				absolute_path: file_path.to_path_buf(),
				prepared: rivetctl::model::upload_prepare_file::Builder::default()
					.path(path_str)
					.set_content_type(content_type)
					.content_length(file_meta.len() as i64)
					.build(),
			});
		}
	}

	Ok(files)
}

/// Uploads a file to a given URL.
async fn upload_file(
	client: &reqwest::Client,
	presigned_req: &rivetctl::model::UploadPresignedRequest,
	path: impl AsRef<Path>,
	content_type: Option<impl ToString>,
) -> Result<()> {
	use tokio::fs::File;
	use tokio_util::codec::{BytesCodec, FramedRead};

	let content_type = content_type.map(|x| x.to_string());

	// Try the upload multiple times since DigitalOcean spaces is incredibly
	// buggy and spotty internet connections may cause issues. This is
	// especially important since we have files that we need to batch upload, so
	// one failing request is bad.
	let mut attempts = 0;
	let upload_time = 'upload: loop {
		// Read file
		let file = File::open(path.as_ref()).await?;
		let file_meta = file.metadata().await?;

		println!(
			"  * {path}: Uploading {size} [{mime}]",
			path = presigned_req.path().unwrap(),
			size = format_file_size(file_meta.len())?,
			mime = content_type.clone().unwrap_or_default(),
		);

		// Create body
		let stream = FramedRead::new(file, BytesCodec::new());
		let body = reqwest::Body::wrap_stream(stream);

		// Upload file
		let start = Instant::now();
		let mut req = client
			.put(presigned_req.url().unwrap())
			.header("content-length", file_meta.len());
		if let Some(content_type) = &content_type {
			req = req.header("content-type", content_type.to_string());
		}
		let res = req.body(body).send().await?;
		if res.status().is_success() {
			let upload_time = start.elapsed();
			break 'upload upload_time;
		} else {
			if attempts > 4 {
				bail!(
					"failed to upload file: {}\n{:?}",
					res.status(),
					res.text().await
				);
			} else {
				attempts += 1;
				println!(
					"  ! Upload failed with status {status}, will retry (attempt #{attempt}): {body:?}",
					attempt = attempts,
					status = res.status(),
					body = res.text().await,
				);
				tokio::time::sleep(Duration::from_secs(5)).await;
				continue 'upload;
			}
		}
	};

	println!(
		"  * {}: Finished in {:.3}s",
		presigned_req.path().unwrap(),
		upload_time.as_secs_f64()
	);

	Ok(())
}

/// Uses the provided token to find the game ID to modify.
async fn infer_game_id(ctx: &rivetctl::ctx::SharedCtx) -> Result<String> {
	let inspect = ctx
		.http_client
		.inspect()
		.send()
		.await
		.context("http_client.inspect")?;
	let game_cloud =
		if let rivetctl::model::AuthAgent::GameCloud(game_cloud) = inspect.agent().unwrap() {
			game_cloud
		} else {
			bail!("invalid token agent");
		};

	Ok(game_cloud.game_id().unwrap().to_string())
}

fn format_file_size(bytes: u64) -> Result<String> {
	use humansize::FileSize;

	let size = format!(
		"{}",
		bytes
			.file_size(humansize::file_size_opts::DECIMAL)
			.ok()
			.context("format file size")?
	);
	Ok(size)
}
