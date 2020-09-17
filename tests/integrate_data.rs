use file_diff::diff;

use axiv::{run, Settings};

#[test]
fn integrate_data() {
    // Almost default settings
    let settings = Settings {
        input: String::from("input.csv"),
        output: String::from("test-output.csv"),
        hotels: String::from("hotels.json"),
        rooms: String::from("room_names.csv"),
    };
    run(&settings).expect("This shouldn't fail");
    // Ensure that our integration tool produces expected output
    assert!(diff("expected.csv", "test-output.csv"));
}
