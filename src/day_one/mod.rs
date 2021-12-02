// https://adventofcode.com/2021/day/1
pub fn execute() -> u32 {
    let input = include_str!("input.txt");

    let depths = input.lines().map(|i| i.parse::<u32>());
    let mut depth_increases = 0;
    let mut previous_depth = 0;

    for (i, parsed_depth) in depths.enumerate() {
        match parsed_depth {
            Ok(depth) => {
                if previous_depth == 0 {
                    println!("Skipping #{0}, Previous depth unknown.", i)
                } else if depth > previous_depth {
                    depth_increases += 1;
                }
                previous_depth = depth;
            }
            Err(_) => println!("Couldn't parse depth #{0}", i)
        }
    }

    return depth_increases;
}
