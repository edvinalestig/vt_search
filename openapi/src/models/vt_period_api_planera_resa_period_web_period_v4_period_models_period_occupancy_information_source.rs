/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OccupancyInformationSource : Represents the source from which the occupancy information originate.

/// Represents the source from which the occupancy information originate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OccupancyInformationSource {
    #[serde(rename = "prediction")]
    Prediction,
    #[serde(rename = "realtime")]
    Realtime,

}

impl ToString for OccupancyInformationSource {
    fn to_string(&self) -> String {
        match self {
            Self::Prediction => String::from("prediction"),
            Self::Realtime => String::from("realtime"),
        }
    }
}

impl Default for OccupancyInformationSource {
    fn default() -> OccupancyInformationSource {
        Self::Prediction
    }
}




