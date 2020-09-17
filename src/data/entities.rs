use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{custom_date, serialize_float};

// I guess there are not many hotels, where you can find rooms for more than 256 people :D
pub type PeopleAmount = u8;
pub type HotelCategory = f32;
pub type Price = f64;

/// It generates key for use in HashMap based on few properties of the room that are available in the input data,
/// so we can distinguish rooms that have few of the same properties, but are not the same.
pub fn generate_room_key(hotel_code: &str, room_code: &str, source: &str) -> String {
    format!("{}-{}-{}", hotel_code, room_code, source)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Room {
    pub hotel_code: String,
    pub source: String,
    pub room_name: String,
    pub room_code: String,
}

impl Room {
    /// Key used in data sources to
    pub fn key(&self) -> String {
        generate_room_key(&self.hotel_code, &self.room_code, &self.source)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Hotel {
    pub id: String,
    pub city_code: String,
    pub name: String,
    pub category: HotelCategory,
    pub country_code: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub city_code: String,
    pub hotel_code: String,
    pub room_type: String,
    pub room_code: String,
    pub meal: String,
    #[serde(with = "custom_date")]
    pub checkin: NaiveDate,
    pub adults: PeopleAmount,
    pub children: PeopleAmount,
    pub price: Price,
    pub source: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Output {
    #[serde(rename(serialize = "room_type meal"))]
    pub room_type_meal: String,
    pub room_code: String,
    pub source: String,
    pub hotel_name: String,
    pub city_name: String,
    pub city_code: String,
    pub hotel_category: HotelCategory,
    pub pax: PeopleAmount,
    pub adults: PeopleAmount,
    pub children: PeopleAmount,
    pub room_name: String,
    #[serde(with = "custom_date")]
    pub checkin: NaiveDate,
    #[serde(with = "custom_date")]
    pub checkout: NaiveDate,
    #[serde(serialize_with = "serialize_float")]
    pub price: Price,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_key() {
        assert_eq!(generate_room_key("HOTEL", "ROOM", "SRC"), "HOTEL-ROOM-SRC");
        assert_eq!(generate_room_key("aaa", "bbb", "ccc"), "aaa-bbb-ccc");
        assert_eq!(generate_room_key("000", "111", "222"), "000-111-222");
    }
}
