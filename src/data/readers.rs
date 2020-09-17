use std::fs::read_to_string;
use std::path::Path;

use anyhow::{Context, Result};
use csv::ReaderBuilder;

use crate::data::{Hotel, Room};

/// Function used to read hotel data from a file which is not a valid json,
/// but each line is a valid json object.
/// It throws an error if the file doesn't exist at specified path or if
/// it encounters data that isn't in the format of the Hotel.
pub fn hotels_reader(path: &Path) -> Result<Vec<(String, Hotel)>> {
    read_to_string(path)
        .with_context(|| "Path to the hotels data is invalid!")?
        .lines()
        .map(|line| {
            serde_json::from_str::<Hotel>(line)
                .map(|hotel| (hotel.id.clone(), hotel))
                .with_context(|| {
                    format!(
                        "Encountered unparsable entity during parsing hotels data at line: {}",
                        line
                    )
                })
        })
        .collect()
}

/// Function used to read rooms data from a CSV file.
/// It throws an error if the file doesn't exist at specified path or if
/// it encounters data that isn't in the format of the Room.
pub fn rooms_reader(path: &Path) -> Result<Vec<(String, Room)>> {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)
        .with_context(|| "Path to the rooms data is invalid!")?;

    csv_reader
        .deserialize::<Room>()
        .map(|res| {
            res.map(|room| (room.key(), room))
                .with_context(|| "Encountered unparsable entity during parsing rooms data.")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_rooms() {
        let data = rooms_reader(Path::new("test_data/room_names.csv"))
            .expect("Couldn't read rooms from given path");

        assert_eq!(
            data,
            vec![
                (
                    String::from("BER00003-BER849-MARR"),
                    Room {
                        hotel_code: String::from("BER00003"),
                        room_code: String::from("BER849"),
                        source: String::from("MARR"),
                        room_name: String::from("Single Standard")
                    }
                ),
                (
                    String::from("BER00003-BER848-MARR"),
                    Room {
                        hotel_code: String::from("BER00003"),
                        room_code: String::from("BER848"),
                        source: String::from("MARR"),
                        room_name: String::from("Deluxe King")
                    }
                ),
                (
                    String::from("BER00003-BER848-DOTW"),
                    Room {
                        hotel_code: String::from("BER00003"),
                        room_code: String::from("BER848"),
                        source: String::from("DOTW"),
                        room_name: String::from("SINGLE DELUXE")
                    }
                ),
                (
                    String::from("BER00002-BER898-GTA"),
                    Room {
                        hotel_code: String::from("BER00002"),
                        room_code: String::from("BER898"),
                        source: String::from("GTA"),
                        room_name: String::from("Standard")
                    }
                ),
                (
                    String::from("BER00002-BER898-IHG"),
                    Room {
                        hotel_code: String::from("BER00002"),
                        room_code: String::from("BER898"),
                        source: String::from("IHG"),
                        room_name: String::from("Einzelzimmer")
                    }
                ),
                (
                    String::from("BER00002-BER848-MARR"),
                    Room {
                        hotel_code: String::from("BER00002"),
                        room_code: String::from("BER848"),
                        source: String::from("MARR"),
                        room_name: String::from("Deluxe King Extra")
                    }
                )
            ]
        )
    }

    #[test]
    fn read_rooms_from_invalid_path() {
        assert_eq!(
            rooms_reader(Path::new("nonexistentfile"))
                .expect_err("This should fail")
                .to_string(),
            "Path to the rooms data is invalid!"
        );
    }

    #[test]
    fn read_rooms_in_invalid_format() {
        assert_eq!(
            rooms_reader(Path::new("test_data/invalid_rooms_data.csv"))
                .expect_err("This should fail")
                .to_string(),
            "Encountered unparsable entity during parsing rooms data.",
        );
    }

    #[test]
    fn read_hotels() {
        let data = hotels_reader(Path::new("test_data/hotels.json"))
            .expect("Couldn't read hotels from given path");

        assert_eq!(
            data,
            vec![
                (
                    String::from("BER00002"),
                    Hotel {
                        id: String::from("BER00002"),
                        city_code: String::from("BER"),
                        name: String::from("Crowne Plaza Berlin City Centre"),
                        category: 4.0,
                        country_code: String::from("DE"),
                        city: String::from("Berlin")
                    }
                ),
                (
                    String::from("BER00003"),
                    Hotel {
                        id: String::from("BER00003"),
                        city_code: String::from("BER"),
                        name: String::from("Berlin Marriott Hotel"),
                        category: 5.0,
                        country_code: String::from("DE"),
                        city: String::from("Berlin")
                    }
                )
            ]
        )
    }

    #[test]
    fn read_hotels_from_invalid_path() {
        assert_eq!(
            hotels_reader(Path::new("nonexistentfile"))
                .expect_err("This should fail")
                .to_string(),
            "Path to the hotels data is invalid!"
        );
    }

    #[test]
    fn read_hotels_in_invalid_format() {
        assert_eq!(
            hotels_reader(Path::new("test_data/invalid_hotels_data.csv"))
                .expect_err("This should fail")
                .to_string(),
            r#"Encountered unparsable entity during parsing hotels data at line: {"id": "BER00003", "city_code": "BER", "country_code": "DE", "city": "Berlin" }"#
        );
    }
}
