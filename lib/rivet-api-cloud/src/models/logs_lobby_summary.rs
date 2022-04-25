/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsLobbySummary {
	#[serde(rename = "lobby_id")]
	pub lobby_id: String,
	#[serde(rename = "namespace_id")]
	pub namespace_id: String,
	#[serde(rename = "lobby_group_name_id")]
	pub lobby_group_name_id: String,
	#[serde(rename = "region_id")]
	pub region_id: String,
	#[serde(rename = "create_ts")]
	pub create_ts: i64,
	#[serde(rename = "status")]
	pub status: Box<crate::models::LogsLobbyStatus>,
}

impl LogsLobbySummary {
	pub fn new(
		lobby_id: String,
		namespace_id: String,
		lobby_group_name_id: String,
		region_id: String,
		create_ts: i64,
		status: crate::models::LogsLobbyStatus,
	) -> LogsLobbySummary {
		LogsLobbySummary {
			lobby_id,
			namespace_id,
			lobby_group_name_id,
			region_id,
			create_ts,
			status: Box::new(status),
		}
	}
}