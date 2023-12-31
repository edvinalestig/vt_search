/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OccupancyInformationApiModel : Contains information about occupancy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OccupancyInformationApiModel {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<crate::models::OccupancyLevel>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::OccupancyInformationSource>,
}

impl OccupancyInformationApiModel {
    /// Contains information about occupancy.
    pub fn new() -> OccupancyInformationApiModel {
        OccupancyInformationApiModel {
            level: None,
            source: None,
        }
    }
}


