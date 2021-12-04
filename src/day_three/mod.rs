// https://adventofcode.com/2021/day/3
pub fn execute() {
    let input = include_str!("input.txt");

    let mut num_bits = 0;
    let sensor_readings: Vec<u16> = input
        .lines()
        .filter_map(|s| match u16::from_str_radix(s, 2) {
            Ok(i) => {
                if s.trim().len() > num_bits {
                    num_bits = s.trim().len()
                }

                return Option::from(i);
            }
            Err(_) => None,
        })
        .collect();

    println!(
        "Power consumption: {0}",
        power_consumption(&sensor_readings, num_bits)
    );
}

fn power_consumption(sensor_readings: &[u16], num_bits: usize) -> u32 {
    let mut bit_counts: Vec<u32> = vec![0; num_bits];

    for sensor_reading in sensor_readings {
        // ex: Pretend u8 for simplicity, For i = 3
        // sensor_reading: 1111 1100
        // mask (1 << 3):  0000 0001 -> 0000 1000
        // AND mask with sensor_reading: 0000 1000, if 3rd bit of reading was 0: 0000 0000

        for i in 0..num_bits {
            let mask = 1 << i;

            if sensor_reading & mask != 0 {
                bit_counts[i] += 1;
            }
        }
    }

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for i in 0..num_bits {
        let mask = 1 << i;

        // If this bit is 1 for more than half the readings, 1 is most significant
        if bit_counts[i] > (sensor_readings.len() / 2) as u32 {
            gamma_rate |= mask
        } else {
            epsilon_rate |= mask
        }
    }

    return gamma_rate * epsilon_rate;
}
