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
pub struct GameLobbyExpenses {
    #[serde(rename = "game")]
    pub game: Box<crate::models::GameHandle>,
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<crate::models::NamespaceSummary>>,
    #[serde(rename = "expenses")]
    pub expenses: Vec<crate::models::RegionTierExpenses>,
}

impl GameLobbyExpenses {
    pub fn new(game: crate::models::GameHandle, expenses: Vec<crate::models::RegionTierExpenses>) -> GameLobbyExpenses {
        GameLobbyExpenses {
            game: Box::new(game),
            namespaces: None,
            expenses,
        }
    }
}


