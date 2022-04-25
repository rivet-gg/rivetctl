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
pub struct MatchmakerCaptchaHcaptcha {
	#[serde(rename = "level")]
	pub level: Level,
}

impl MatchmakerCaptchaHcaptcha {
	pub fn new(level: Level) -> MatchmakerCaptchaHcaptcha {
		MatchmakerCaptchaHcaptcha { level }
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
	#[serde(rename = "Easy")]
	Easy,
	#[serde(rename = "Moderate")]
	Moderate,
	#[serde(rename = "Difficult")]
	Difficult,
	#[serde(rename = "AlwaysOn")]
	AlwaysOn,
}

impl Default for Level {
	fn default() -> Level {
		Self::Easy
	}
}