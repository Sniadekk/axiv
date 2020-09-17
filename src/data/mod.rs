use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use anyhow::Result;

pub use entities::{Hotel, Input, Output, Room};
pub use integrator::DataIntegrator;
pub use readers::{hotels_reader, rooms_reader};

mod entities;
mod integrator;
mod readers;

/// Custom serde for dates that come in the input.
/// It deserializes date from format %Y%m%d (e.g 20190730) to chrono::NaiveDate.
/// It's serialized to format %Y-%m-%d (e.g 2019-07-30).
pub mod custom_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const INPUT_FORMAT: &str = "%Y%m%d";
    const OUTPUT_FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}", date.format(OUTPUT_FORMAT)).as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        NaiveDate::parse_from_str(String::deserialize(deserializer)?.as_str(), INPUT_FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

/// Custom serde for float numbers to ensure that it is always serialized
/// with two decimal points e.g 8.50 instead of 8.5
pub mod pretty_float {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::data::entities::Price;

    pub fn serialize<S>(num: &Price, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{:.2}", num).as_ref())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Price, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<Price>()
            .map_err(serde::de::Error::custom)
    }
}

/// In-memory data source that keeps its data in a HashMap.
/// The data can be imported from many different places and the read/deserialization
/// process is supplied by the Reader which is just a plain function that reads data from the given path
/// and returns it as a Vec<I>. This way we are not strictly tied to one source of data and one way of parsing it.

pub struct DataSource<I, K: Eq + Hash> {
    items: HashMap<K, I>,
}

impl<I, K: Eq + Hash> DataSource<I, K> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Import data from given path, read it with given data reader and save to the self.items
    /// This method is generic, so we are not tied to one particular way of importing the data, because of that
    /// we are able to import data from many different places or file formats.
    /// We just need to provide a function that is able to deserialize the data into type I.
    /// This operation might fail, because the deserialization process may not succeed or the file might not exist.
    pub fn import_from<R>(&mut self, path: &Path, reader: R) -> Result<()>
    where
        R: Fn(&Path) -> Result<Vec<(K, I)>>,
    {
        let items = reader(path)?;
        self.items.extend(items.into_iter());
        Ok(())
    }

    /// Find data in the DataSource by the given key.
    pub fn find(&self, key: &K) -> Option<&I> {
        self.items.get(key)
    }
}
