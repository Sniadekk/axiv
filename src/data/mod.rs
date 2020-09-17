use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use anyhow::Result;
use serde::Serializer;

pub use entities::{Hotel, Input, Output, Room};
pub use integrator::DataIntegrator;
pub use readers::{hotels_reader, rooms_reader};

use crate::data::entities::Price;

mod entities;
mod integrator;
mod readers;

pub type RoomDataSource = DataSource<String, Room>;
pub type HotelDataSource = DataSource<String, Hotel>;

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

pub fn serialize_float<S>(num: &Price, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{:.2}", num).as_ref())
}

/// In-memory data source that keeps its data in a HashMap.
/// The data can be imported from many different places and the read/deserialization
/// process is supplied by the Reader which is just a plain function that reads data from the given path
/// and returns it as a Vec<I>. This way we are not strictly tied to one source of data and one way of parsing it.

pub struct DataSource<K: Eq + Hash, I> {
    items: HashMap<K, I>,
}

impl<I, K: Eq + Hash> DataSource<K, I> {
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

#[cfg(test)]
mod test {
    use std::path::Path;

    use anyhow::Result;
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    use super::*;

    // DataSource

    fn mock_data(_path: &Path) -> Result<Vec<(String, usize)>> {
        Ok(vec![
            (String::from("one"), 1),
            (String::from("two"), 2),
            (String::from("three"), 3),
            (String::from("four"), 4),
            (String::from("five"), 5),
        ])
    }

    #[test]
    fn import_from() -> Result<()> {
        let mut data_source: DataSource<String, usize> = DataSource::new();
        data_source.import_from(Path::new("some_path"), &mock_data)?;
        assert_eq!(data_source.items.len(), 5);
        Ok(())
    }

    #[test]
    fn find() -> Result<()> {
        let mut data_source: DataSource<String, usize> = DataSource::new();
        data_source.import_from(Path::new("some_path"), &mock_data)?;

        assert_eq!(
            data_source
                .find(&String::from("one"))
                .expect("Unable to find item in data source for given key!"),
            &1
        );
        assert_eq!(
            data_source
                .find(&String::from("two"))
                .expect("Unable to find item in data source for given key!"),
            &2
        );
        assert_eq!(
            data_source
                .find(&String::from("three"))
                .expect("Unable to find item in data source for given key!"),
            &3
        );
        assert_eq!(
            data_source
                .find(&String::from("four"))
                .expect("Unable to find item in data source for given key!"),
            &4
        );
        assert_eq!(
            data_source
                .find(&String::from("five"))
                .expect("Unable to find item in data source for given key!"),
            &5
        );

        Ok(())
    }

    // custom_date

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct MockDate {
        #[serde(with = "custom_date")]
        date: NaiveDate,
    }

    #[test]
    fn custom_date_ser() {
        assert_eq!(
            serde_json::to_string(&MockDate {
                date: NaiveDate::from_ymd(2020, 12, 12)
            })
            .expect("Unable to serialize given struct"),
            r#"{"date":"2020-12-12"}"#
        );

        assert_eq!(
            serde_json::to_string(&MockDate {
                date: NaiveDate::from_ymd(1, 1, 1)
            })
            .expect("Unable to serialize given struct"),
            r#"{"date":"0001-01-01"}"#
        );

        assert_eq!(
            serde_json::to_string(&MockDate {
                date: NaiveDate::from_ymd(10, 1, 1)
            })
            .expect("Unable to serialize given struct"),
            r#"{"date":"0010-01-01"}"#
        );

        assert_eq!(
            serde_json::to_string(&MockDate {
                date: NaiveDate::from_ymd(100, 1, 1)
            })
            .expect("Unable to serialize given struct"),
            r#"{"date":"0100-01-01"}"#
        );
    }

    #[test]
    fn custom_date_de() {
        assert_eq!(
            serde_json::from_str::<MockDate>(r#"{"date":"20201212"}"#)
                .expect("Couldn't deserialize given json"),
            MockDate {
                date: NaiveDate::from_ymd(2020, 12, 12)
            }
        );
        assert_eq!(
            serde_json::from_str::<MockDate>(r#"{"date":"00010101"}"#)
                .expect("Couldn't deserialize given json"),
            MockDate {
                date: NaiveDate::from_ymd(1, 1, 1)
            }
        );
        assert_eq!(
            serde_json::from_str::<MockDate>(r#"{"date":"00100101"}"#)
                .expect("Couldn't deserialize given json"),
            MockDate {
                date: NaiveDate::from_ymd(10, 1, 1)
            }
        );
        assert_eq!(
            serde_json::from_str::<MockDate>(r#"{"date":"01000101"}"#)
                .expect("Couldn't deserialize given json"),
            MockDate {
                date: NaiveDate::from_ymd(100, 1, 1)
            }
        );
        assert_eq!(
            serde_json::from_str::<MockDate>(r#"{"date":"10000101"}"#)
                .expect("Couldn't deserialize given json"),
            MockDate {
                date: NaiveDate::from_ymd(1000, 1, 1)
            }
        );
    }
}
