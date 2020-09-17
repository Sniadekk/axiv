use std::fs::File;

use anyhow::{anyhow, Result};
use chrono::Duration;
use csv::DeserializeRecordsIter;

use crate::data::entities::{generate_room_key, Price};
use crate::data::{DataSource, Hotel, Input, Output, Room};

/// Struct used to enrich input data with the additional data from the rooms and hotels data source
/// It works as an iterator and lazily buffers the data from .csv and into .csv files, so it is able
/// to work with larger amounts of data.
pub struct DataIntegrator<'a> {
    input: DeserializeRecordsIter<'a, File, Input>,
    rooms: DataSource<Room, String>,
    hotels: DataSource<Hotel, String>,
}

impl<'a> DataIntegrator<'a> {
    pub fn new(
        rooms: DataSource<Room, String>,
        hotels: DataSource<Hotel, String>,
        input: DeserializeRecordsIter<'a, File, Input>,
    ) -> Self {
        Self {
            rooms,
            hotels,
            input,
        }
    }
}

/// Iterator that iterates over the input data which is buffered from the input file as the iterator goes.
/// In enriches the input data with additional information about room and hotel.
/// It throws an error if there's no room or hotel found for the specified code for each of them in the input data.
/// Then it calculates the sum of adults and children, date of the checkout, price per person and combines everything into final object.
impl<'a> Iterator for DataIntegrator<'a> {
    type Item = Result<Output>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next().transpose() {
            Ok(Some(item)) => {
                let room_key = generate_room_key(&item.hotel_code, &item.room_code, &item.source);
                let room = match self.rooms.find(&room_key) {
                    Some(room) => room,
                    None => {
                        return Some(Err(anyhow!(format!(
                            "Input links to a non existent room: {:#?}",
                            item
                        ))))
                    }
                };
                let hotel = match self.hotels.find(&item.hotel_code) {
                    Some(hotel) => hotel,
                    None => {
                        return Some(Err(anyhow!(format!(
                            "Input links to a non existent hotel: {:#?}",
                            item
                        ))))
                    }
                };
                // number of adults and children combined
                let pax = item.adults + item.children;
                // price per person
                let price = item.price / pax as Price;
                // combine everything together
                let output = Output {
                    room_type_meal: format!("{} {}", item.room_type, item.meal),
                    room_code: room.room_code.clone(),
                    source: item.source,
                    hotel_name: hotel.name.clone(),
                    city_name: hotel.city.clone(),
                    city_code: item.city_code,
                    hotel_category: hotel.category,
                    pax,
                    adults: item.adults,
                    children: item.children,
                    room_name: room.room_name.clone(),
                    checkin: item.checkin,
                    checkout: item.checkin + Duration::days(1),
                    price,
                };
                Some(Ok(output))
            }
            Err(_) => Some(Err(anyhow!(
                "Input contains data that can't be deserialized!"
            ))),
            Ok(None) => None,
        }
    }
}
