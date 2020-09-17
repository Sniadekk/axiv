use clap::Clap;

#[derive(Clap)]
pub struct Settings {
    /// Path to the input file containing incomplete data
    #[clap(short, default_value = "input.csv")]
    pub input: String,
    /// Path to the file where the outcome of the program will be saved.
    /// This file will be created if it doesn't exist.
    #[clap(short, default_value = "output.csv")]
    pub output: String,
    /// Path to the file where data about rooms is stored.
    /// DataSource will look for data to import there.
    #[clap(short, default_value = "room_names.csv")]
    pub rooms: String,
    /// Path to the file where data about hotels is stored.
    /// DataSource will look for data to import there.
    #[clap(short, default_value = "hotels.json")]
    pub hotels: String,
}
