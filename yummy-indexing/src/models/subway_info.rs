use crate::common::*;


#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult)]
pub struct SubwayInfo {
    pub seq: i32,
    pub subway_line: String,
    pub station_name: String,
    pub station_eng_name: String,
    pub lat: Decimal,
    pub lng: Decimal,
    pub station_load_addr: String
}

#[doc = "Elasticsearch 와 mapping 할 구조체"]
#[derive(Debug, Serialize, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct SubwayInfoEs {
    pub seq: i32,
    pub subway_line: String,
    pub station_name: String,
    pub station_eng_name: String,
    pub location: GeoPoint,
    pub station_load_addr: String
}

#[derive(Debug, Serialize, new)]
pub struct GeoPoint {
    lat: f64,
    lon: f64,
}