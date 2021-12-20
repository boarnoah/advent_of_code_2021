#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

// https://adventofcode.com/2021/day/4
pub fn execute() {
    println!("Winning bingo score: {0}", winning_bingo_score());
}

fn winning_bingo_score() -> u16 {
    return 0;
}
