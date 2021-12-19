#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

// https://adventofcode.com/2021/day/1
pub fn execute() {
    let depths: Vec<u32> = INPUT_FILE
        .lines()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    println!("Number of depth increases: {0}", depth_increases(&depths));
    println!(
        "Number of depth increases (sampling with sliding window (of 3)): {0}",
        depth_increases_window_sampling(&depths, 3)
    );
}

pub fn depth_increases(depths: &[u32]) -> u32 {
    let mut depth_increases = 0;
    let mut previous_depth = &depths[0];

    for depth in depths.iter().skip(1) {
        if *depth > *previous_depth {
            depth_increases += 1;
        }

        previous_depth = depth;
    }

    return depth_increases;
}

pub fn depth_increases_window_sampling(depths: &[u32], sampling_window: usize) -> u32 {
    let mut depth_increases = 0;
    let mut previous_depth: i32 = -1;

    for depth_samples in depths.windows(sampling_window) {
        let current_depths: u32 = depth_samples.iter().sum::<u32>();

        if previous_depth != -1 && current_depths > previous_depth as u32 {
            depth_increases += 1;
        }

        previous_depth = current_depths as i32;
    }

    return depth_increases;
}
