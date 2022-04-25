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
pub struct LobbyGroupRegionIdleLobbies {
	#[serde(rename = "min_idle_lobbies")]
	pub min_idle_lobbies: i32,
	#[serde(rename = "max_idle_lobbies")]
	pub max_idle_lobbies: i32,
}

impl LobbyGroupRegionIdleLobbies {
	pub fn new(min_idle_lobbies: i32, max_idle_lobbies: i32) -> LobbyGroupRegionIdleLobbies {
		LobbyGroupRegionIdleLobbies {
			min_idle_lobbies,
			max_idle_lobbies,
		}
	}
}