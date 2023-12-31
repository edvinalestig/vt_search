/*
 * Planera Resa
 *
 * Sök och planera resor med Västtrafik
 *
 * The version of the OpenAPI document: v4
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaginationProperties : Pagination information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaginationProperties {
    /// The requested number of results.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The requested offset in the results array.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// The actual number of returned results.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl PaginationProperties {
    /// Pagination information.
    pub fn new() -> PaginationProperties {
        PaginationProperties {
            limit: None,
            offset: None,
            size: None,
        }
    }
}


