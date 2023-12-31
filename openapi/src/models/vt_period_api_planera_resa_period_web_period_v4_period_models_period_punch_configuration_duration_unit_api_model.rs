/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PunchConfigurationDurationUnitApiModel : Unit of duration of validity of a single punch.

/// Unit of duration of validity of a single punch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PunchConfigurationDurationUnitApiModel {
    #[serde(rename = "hours")]
    Hours,

}

impl ToString for PunchConfigurationDurationUnitApiModel {
    fn to_string(&self) -> String {
        match self {
            Self::Hours => String::from("hours"),
        }
    }
}

impl Default for PunchConfigurationDurationUnitApiModel {
    fn default() -> PunchConfigurationDurationUnitApiModel {
        Self::Hours
    }
}




