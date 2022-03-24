use crate::day_five::vent_reading::VentReading;
mod vent_reading;

#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

pub fn execute() {
    println!("Hydrothermal vent danger spots: {}", danger_spots());

    let max_coordinate: usize = 0;

    let vent_readings: Vec<vent_reading::VentReading> = INPUT_FILE
        .lines()
        .filter_map(|s| s.parse::<VentReading>().ok())
        .collect();
}

pub fn danger_spots() -> u16 {
    return 0;
}
