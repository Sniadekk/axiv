use std::path::Path;

use anyhow::{Context, Result};
use csv::{ReaderBuilder, WriterBuilder};

use crate::data::{
    hotels_reader, rooms_reader, DataIntegrator, DataSource, HotelDataSource, RoomDataSource,
};
pub use crate::settings::Settings;

mod data;
mod settings;

pub fn run(settings: &Settings) -> Result<()> {
    // Create data sources and populate them with data
    let mut hotels: HotelDataSource = DataSource::new();
    hotels.import_from(Path::new(&settings.hotels), &hotels_reader)?;

    let mut rooms: RoomDataSource = DataSource::new();
    rooms.import_from(Path::new(&settings.rooms), &rooms_reader)?;

    // Create reader to read the incomplete input data
    let mut input_buffer = ReaderBuilder::new()
        .delimiter(b'|')
        .from_path(&settings.input)
        .unwrap();

    let input_reader = input_buffer.deserialize();

    let data_integrator = DataIntegrator::new(rooms, hotels, input_reader);

    // Create writer to write the complete output data
    let mut output_writer = WriterBuilder::new()
        .delimiter(b';')
        .from_path(Path::new(&settings.output))?;

    // Iterate over input data, integrate it with data from data sources and save in output file
    for output_res in data_integrator {
        let output = output_res?;
        output_writer
            .serialize(&output)
            .with_context(|| format!("Couldn't serialize {:#?}", &output))?;
    }
    Ok(())
}
