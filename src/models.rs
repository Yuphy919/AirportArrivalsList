use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Flight {
    pub flight_status: String,
    pub arrival_time: String,
    pub place_of_departure: String,
    pub aircraft_code: String,
    pub terminal: String,
}

#[derive(Deserialize)]
pub struct FlightApiResponse {
    pub flight_info: Option<Vec<FlightData>>,
}

#[derive(Deserialize)]
pub struct FlightData {
    #[serde(rename = "備考和名称")]
    pub note_jp: Option<String>,
    
    #[serde(rename = "定刻")]
    pub scheduled_time: Option<String>,
    
    #[serde(rename = "変更時刻")]
    pub changed_time: Option<String>,
    
    #[serde(rename = "備考訳名称")]
    pub note_status: Option<FlightStatus>,
    
    #[serde(rename = "出発地空港和名称")]
    pub departure_airport: Option<String>,
    
    #[serde(rename = "機種コード")]
    pub aircraft_code: Option<String>,
    
    #[serde(rename = "ターミナル区分")]
    pub terminal: Option<String>,
}

#[derive(Deserialize)]
pub struct FlightStatus {
    pub ja: Option<String>,
}
