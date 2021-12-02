// https://adventofcode.com/2021/day/1
pub fn execute() {
    let input = include_str!("input.txt");

    let depths: Vec<u32> = input.lines().filter_map(|s| s.parse::<u32>().ok()).collect();
    println!("Number of depth increases: {0}", depth_increases(&depths));
    println!("Number of depth increases (sampling with sliding window (of 3)): {0}", depth_increases_window_sampling(&depths, 3));
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

pub fn depth_increases_window_sampling(depths: &[i32], sampling_window: u32) -> u32 {
    return 0;
}