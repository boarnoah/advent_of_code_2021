// https://adventofcode.com/2021/day/3
pub fn execute() {
    let input = include_str!("input.txt");

    let sensor_readings: Vec<u16> = input
        .lines()
        .filter_map(|s| u16::from_str_radix(s, 2).ok())
        .collect();

    println!(
        "Power consumption: {0}",
        power_consumption(&sensor_readings)
    );
}

fn power_consumption(sensor_readings: &[u16]) -> u32 {
    for sensor_reading in sensor_readings {
        println!("{0} - {0:b}", sensor_reading)
    }
    println!("{}", sensor_readings.len());
    return 0;
}
