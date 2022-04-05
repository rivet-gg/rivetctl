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
pub struct InlineObject4 {
    #[serde(rename = "hostname")]
    pub hostname: String,
    #[serde(rename = "lobby_ports")]
    pub lobby_ports: Vec<crate::models::LobbyGroupRuntimeDockerPort>,
}

impl InlineObject4 {
    pub fn new(hostname: String, lobby_ports: Vec<crate::models::LobbyGroupRuntimeDockerPort>) -> InlineObject4 {
        InlineObject4 {
            hostname,
            lobby_ports,
        }
    }
}


