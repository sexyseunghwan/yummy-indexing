use crate::common::*;


#[doc = "MySQL 와 맵핑할 구조체"]
#[derive(Debug, FromQueryResult,)]
pub struct SubwayInfo {
    pub seq: i32,
    pub subway_line: String,
    pub station_name: String,
    pub station_eng_name: String,
    pub lat: Decimal,
    pub lng: Decimal,
    pub station_load_addr: String
}

impl SubwayInfo {
    pub fn to_es(&self) -> SubwayInfoEs {
        SubwayInfoEs::new(
            self.seq, 
            self.subway_line.clone(), 
            self.station_name.clone(), 
            self.station_eng_name.clone(), 
             GeoPoint::new(
                self.lat.to_f64().unwrap_or(0.0),
                self.lng.to_f64().unwrap_or(0.0),
            ), 
            self.station_load_addr.clone(),
            self.lat,
            self.lng
        )
    }
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
    pub station_load_addr: String,
    pub lat: Decimal,
    pub lng: Decimal,
}

#[derive(Debug, Serialize, new)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}