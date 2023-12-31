/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JourneyDetailsPeriodTripLegDetailsApiModel : Detailed information about a Public Transport trip leg.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JourneyDetailsPeriodTripLegDetailsApiModel {
    /// The service journey for the trip leg.
    #[serde(rename = "serviceJourneys", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub service_journeys: Option<Option<Vec<crate::models::JourneyDetailsPeriodServiceJourneyApiModel>>>,
    /// The calls on the trip leg.
    #[serde(rename = "callsOnTripLeg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub calls_on_trip_leg: Option<Option<Vec<crate::models::JourneyDetailsPeriodCallDetailsApiModel>>>,
    /// The coordinates for the trip leg.
    #[serde(rename = "tripLegCoordinates", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trip_leg_coordinates: Option<Option<Vec<crate::models::CoordinateApiModel>>>,
    /// The tariff zones that the trip leg traverses.
    #[serde(rename = "tariffZones", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tariff_zones: Option<Option<Vec<crate::models::JourneyDetailsPeriodTariffZoneApiModel>>>,
    /// Flag indicating if the trip leg is cancelled.
    #[serde(rename = "isCancelled", skip_serializing_if = "Option::is_none")]
    pub is_cancelled: Option<bool>,
    /// Flag indicating if the trip leg is partially cancelled.
    #[serde(rename = "isPartCancelled", skip_serializing_if = "Option::is_none")]
    pub is_part_cancelled: Option<bool>,
    #[serde(rename = "occupancy", skip_serializing_if = "Option::is_none")]
    pub occupancy: Option<Box<crate::models::OccupancyInformationApiModel>>,
    /// Index of Leg in Journey
    #[serde(rename = "journeyLegIndex", skip_serializing_if = "Option::is_none")]
    pub journey_leg_index: Option<i32>,
}

impl JourneyDetailsPeriodTripLegDetailsApiModel {
    /// Detailed information about a Public Transport trip leg.
    pub fn new() -> JourneyDetailsPeriodTripLegDetailsApiModel {
        JourneyDetailsPeriodTripLegDetailsApiModel {
            service_journeys: None,
            calls_on_trip_leg: None,
            trip_leg_coordinates: None,
            tariff_zones: None,
            is_cancelled: None,
            is_part_cancelled: None,
            occupancy: None,
            journey_leg_index: None,
        }
    }
}


