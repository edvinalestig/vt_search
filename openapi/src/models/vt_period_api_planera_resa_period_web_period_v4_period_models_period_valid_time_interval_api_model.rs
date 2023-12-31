/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ValidTimeIntervalApiModel : Information specifying the interval when valid journey information is available, i.e. when it is possible to search journeys.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidTimeIntervalApiModel {
    /// The start time of the interval when valid journey information is available, specified in RFC 3339 format.
    #[serde(rename = "validFrom", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<Option<String>>,
    /// The end time of the interval when valid journey information is available, specified in RFC 3339 format.
    #[serde(rename = "validUntil", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<Option<String>>,
}

impl ValidTimeIntervalApiModel {
    /// Information specifying the interval when valid journey information is available, i.e. when it is possible to search journeys.
    pub fn new() -> ValidTimeIntervalApiModel {
        ValidTimeIntervalApiModel {
            valid_from: None,
            valid_until: None,
        }
    }
}


