#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

pub fn execute() {
    println!("Hydrothermal vent danger spots: {}", danger_spots());
}

pub fn danger_spots() -> u16 {
    return 0;
}
