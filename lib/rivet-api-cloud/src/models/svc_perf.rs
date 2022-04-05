/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvcPerf {
    #[serde(rename = "svc_name")]
    pub svc_name: String,
    #[serde(rename = "ts")]
    pub ts: i64,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "req_id")]
    pub req_id: String,
    #[serde(rename = "spans")]
    pub spans: Vec<crate::models::LogsPerfSpan>,
    #[serde(rename = "marks")]
    pub marks: Vec<crate::models::LogsPerfMark>,
}

impl SvcPerf {
    pub fn new(svc_name: String, ts: i64, duration: i64, req_id: String, spans: Vec<crate::models::LogsPerfSpan>, marks: Vec<crate::models::LogsPerfMark>) -> SvcPerf {
        SvcPerf {
            svc_name,
            ts,
            duration,
            req_id,
            spans,
            marks,
        }
    }
}


