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
pub struct LogsPerfSpan {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "start_ts")]
	pub start_ts: i64,
	#[serde(rename = "finish_ts", skip_serializing_if = "Option::is_none")]
	pub finish_ts: Option<i64>,
	#[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
	pub req_id: Option<String>,
}

impl LogsPerfSpan {
	pub fn new(label: String, start_ts: i64) -> LogsPerfSpan {
		LogsPerfSpan {
			label,
			start_ts,
			finish_ts: None,
			req_id: None,
		}
	}
}