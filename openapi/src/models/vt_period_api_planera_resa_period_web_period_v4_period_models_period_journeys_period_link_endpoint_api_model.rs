/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneysPeriodLinkEndpointApiModel : Information about an endpoint on an access link.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneysPeriodLinkEndpointApiModel {
    /// The 16-digit Västtrafik gid.
    #[serde(rename = "gid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gid: Option<Option<String>>,
    /// The location name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "locationType")]
    pub location_type: crate::models::LocationType,
    /// The WGS84 latitude of the location.
    #[serde(rename = "latitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Option<f64>>,
    /// The WGS84 longitude of the location.
    #[serde(rename = "longitude", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Option<f64>>,
    /// The planned time in RFC 3339 format.
    #[serde(rename = "plannedTime")]
    pub planned_time: String,
    /// The estimated time in RFC 3339 format.
    #[serde(rename = "estimatedTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_time: Option<Option<String>>,
    /// The best known time of the link in RFC 3339 format. Is EstimatedTime if exists, otherwise PlannedTime.
    #[serde(rename = "estimatedOtherwisePlannedTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_otherwise_planned_time: Option<Option<String>>,
    /// An ordered list (most important first) of notes related to the end point.
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<Vec<crate::models::Note>>>,
}

impl JourneysPeriodLinkEndpointApiModel {
    /// Information about an endpoint on an access link.
    pub fn new(name: String, location_type: crate::models::LocationType, planned_time: String) -> JourneysPeriodLinkEndpointApiModel {
        JourneysPeriodLinkEndpointApiModel {
            gid: None,
            name,
            location_type,
            latitude: None,
            longitude: None,
            planned_time,
            estimated_time: None,
            estimated_otherwise_planned_time: None,
            notes: None,
        }
    }
}


