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
pub struct InlineResponse2009 {
    #[serde(rename = "lobbies")]
    pub lobbies: Vec<crate::models::LogsLobbySummary>,
}

impl InlineResponse2009 {
    pub fn new(lobbies: Vec<crate::models::LogsLobbySummary>) -> InlineResponse2009 {
        InlineResponse2009 {
            lobbies,
        }
    }
}


