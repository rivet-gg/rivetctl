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
pub struct RegionTier {
    #[serde(rename = "tier_name_id")]
    pub tier_name_id: String,
    #[serde(rename = "rivet_cores_numerator")]
    pub rivet_cores_numerator: i32,
    #[serde(rename = "rivet_cores_denominator")]
    pub rivet_cores_denominator: i32,
    #[serde(rename = "cpu")]
    pub cpu: i64,
    #[serde(rename = "memory")]
    pub memory: i64,
    #[serde(rename = "disk")]
    pub disk: i64,
    #[serde(rename = "bandwidth")]
    pub bandwidth: i64,
    #[serde(rename = "price_per_second")]
    pub price_per_second: i64,
}

impl RegionTier {
    pub fn new(tier_name_id: String, rivet_cores_numerator: i32, rivet_cores_denominator: i32, cpu: i64, memory: i64, disk: i64, bandwidth: i64, price_per_second: i64) -> RegionTier {
        RegionTier {
            tier_name_id,
            rivet_cores_numerator,
            rivet_cores_denominator,
            cpu,
            memory,
            disk,
            bandwidth,
            price_per_second,
        }
    }
}


