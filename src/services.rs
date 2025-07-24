use crate::models::{Flight, FlightApiResponse};
use actix_web::Error as ActixError;
use chrono::Local;
use regex::Regex;
use reqwest;

const API_URL: &str = "https://tokyo-haneda.com/app_resource/flight/data/int/hdacfarv.json";

pub struct FlightService;

impl FlightService {
    pub async fn fetch_todays_flights() -> Result<Vec<Flight>, ActixError> {
        let response_text = Self::fetch_api_data().await?;
        let flight_data = Self::parse_api_response(&response_text)?;
        let today = Self::get_today_string();
        let flights = Self::filter_and_convert_flights(flight_data, &today)?;
        
        Ok(Self::sort_flights_by_time(flights))
    }

    async fn fetch_api_data() -> Result<String, ActixError> {
        reqwest::get(API_URL)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
            .text()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)
    }

    fn parse_api_response(response_text: &str) -> Result<FlightApiResponse, ActixError> {
        serde_json::from_str(response_text)
            .map_err(actix_web::error::ErrorInternalServerError)
    }

    fn get_today_string() -> String {
        Local::now().format("%Y/%m/%d").to_string()
    }

    fn filter_and_convert_flights(
        api_response: FlightApiResponse,
        today: &str,
    ) -> Result<Vec<Flight>, ActixError> {
        let date_re = Regex::new(r"\d{4}/\d{2}/\d{2}")
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let time_re = Regex::new(r"\b([01]?\d|2[0-3]):[0-5]\d\b")
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut flights = Vec::new();

        if let Some(flight_list) = api_response.flight_info {
            for flight_data in flight_list {
                // 到着済みのフライトをスキップ
                if let Some(note) = &flight_data.note_jp {
                    if note.contains("到着済み") {
                        continue;
                    }
                }

                let arrival_time_str = Self::get_arrival_time(&flight_data);
                
                // 今日のフライトかチェック
                if let Some(date_match) = date_re.captures(&arrival_time_str) {
                    if date_match.get(0).map(|m| m.as_str()) != Some(today) {
                        continue;
                    }

                    let time_only = time_re
                        .captures(&arrival_time_str)
                        .and_then(|cap| cap.get(0))
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();

                    flights.push(Flight {
                        flight_status: flight_data
                            .note_status
                            .and_then(|status| status.ja)
                            .unwrap_or_default(),
                        arrival_time: time_only,
                        place_of_departure: flight_data.departure_airport.unwrap_or_default(),
                        aircraft_code: flight_data.aircraft_code.unwrap_or_default(),
                        terminal: flight_data.terminal.unwrap_or_default(),
                    });
                }
            }
        }

        Ok(flights)
    }

    fn get_arrival_time(flight_data: &crate::models::FlightData) -> String {
        flight_data
            .changed_time
            .as_ref()
            .filter(|time| !time.is_empty())
            .or(flight_data.scheduled_time.as_ref())
            .unwrap_or(&String::new())
            .clone()
    }

    fn sort_flights_by_time(mut flights: Vec<Flight>) -> Vec<Flight> {
        flights.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
        flights
    }
}
