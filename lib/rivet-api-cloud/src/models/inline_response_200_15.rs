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
pub struct InlineResponse20015 {
    #[serde(rename = "did_remove")]
    pub did_remove: bool,
}

impl InlineResponse20015 {
    pub fn new(did_remove: bool) -> InlineResponse20015 {
        InlineResponse20015 {
            did_remove,
        }
    }
}


