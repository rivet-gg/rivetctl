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
pub struct CdnSiteSummary {
    #[serde(rename = "site_id")]
    pub site_id: String,
    #[serde(rename = "upload_id")]
    pub upload_id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "create_ts")]
    pub create_ts: i64,
    #[serde(rename = "content_length")]
    pub content_length: i64,
    #[serde(rename = "complete")]
    pub complete: bool,
}

impl CdnSiteSummary {
    pub fn new(site_id: String, upload_id: String, display_name: String, create_ts: i64, content_length: i64, complete: bool) -> CdnSiteSummary {
        CdnSiteSummary {
            site_id,
            upload_id,
            display_name,
            create_ts,
            content_length,
            complete,
        }
    }
}


