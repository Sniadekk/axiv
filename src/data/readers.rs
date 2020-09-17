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
