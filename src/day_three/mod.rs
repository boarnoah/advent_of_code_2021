#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

// https://adventofcode.com/2021/day/3
pub fn execute() {
    let mut num_bits = 0;
    let sensor_readings: Vec<u16> = INPUT_FILE
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
        "Power consumption: {}",
        power_consumption(&sensor_readings, num_bits)
    );
    println!(
        "Life Support Rating: {}",
        life_support_rating(&sensor_readings, num_bits)
    );
}

fn power_consumption(sensor_readings: &[u16], num_bits: usize) -> u32 {
    let bit_counts: Vec<u32> = count_flagged_bits_by_position(sensor_readings, num_bits);

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for i in 0..num_bits {
        let mask = 1 << i;

        // If this bit is 1 for more than half the readings, 1 is most significant
        if bit_counts[i] > sensor_readings.len() as u32 - bit_counts[i] {
            gamma_rate |= mask
        } else {
            epsilon_rate |= mask
        }
    }

    return gamma_rate * epsilon_rate;
}

fn filter_candidates<RetainCriteria: Fn(u32, u32) -> bool>(
    candidates: &mut Vec<u16>,
    i: usize,
    criteria: RetainCriteria,
) {
    if candidates.len() > 1 {
        let bit_count = count_flagged_bits_in_position(&candidates, i);

        if criteria(bit_count, candidates.len() as u32 - bit_count) {
            candidates.retain(|x| {
                let mask = 1 << i;
                *x & mask != 0
            })
        } else {
            candidates.retain(|x| {
                let mask = 1 << i;
                *x & mask == 0
            })
        }
    }
}

fn life_support_rating(sensor_readings: &[u16], num_bits: usize) -> u32 {
    let mut oxygen_reading_candidates: Vec<u16> = sensor_readings.to_vec();
    let mut co2_reading_candidates = Vec::from(sensor_readings);

    for i in (0..num_bits).rev() {
        if oxygen_reading_candidates.len() > 1 {
            filter_candidates(
                &mut oxygen_reading_candidates,
                i,
                |bit_count, num_candidates| bit_count >= num_candidates,
            );
        }

        if co2_reading_candidates.len() > 1 {
            filter_candidates(
                &mut co2_reading_candidates,
                i,
                |bit_count, num_candidates| bit_count < num_candidates,
            );
        }
    }

    match oxygen_reading_candidates.first() {
        None => 0,
        Some(oxygen_reading) => match co2_reading_candidates.first() {
            None => 0,
            Some(co2_reading) => *oxygen_reading as u32 * *co2_reading as u32,
        },
    }
}

fn count_flagged_bits_in_position(sensor_readings: &[u16], position: usize) -> u32 {
    let mut bit_count: u32 = 0;

    // ex: Pretend u8 for simplicity, For i = 3
    // sensor_reading: 1111 1100
    // mask (1 << 3):  0000 0001 -> 0000 1000
    // AND mask with sensor_reading: 0000 1000, if 3rd bit of reading was 0: 0000 0000
    for sensor_reading in sensor_readings {
        let mask = 1 << position;

        if sensor_reading & mask != 0 {
            bit_count += 1;
        }
    }

    return bit_count;
}

fn count_flagged_bits_by_position(sensor_readings: &[u16], num_bits: usize) -> Vec<u32> {
    let mut bit_counts: Vec<u32> = vec![0; num_bits];

    for i in 0..num_bits {
        bit_counts[i] = count_flagged_bits_in_position(sensor_readings, i)
    }

    return bit_counts;
}
