use std::process::exit;

#[cfg(all(feature = "sample_input"))]
static INPUT_FILE: &str = include_str!("sample.txt");
#[cfg(not(all(feature = "sample_input")))]
static INPUT_FILE: &str = include_str!("input.txt");

// https://adventofcode.com/2021/day/4
pub fn execute() {
    let mut input_data = INPUT_FILE.lines();
    let rolls: Vec<u8> = match input_data.next() {
        None => {
            println!("No rolls found in first line");
            exit(1)
        }
        Some(rolls) => rolls
            .split(",")
            .filter_map(|s| s.parse::<u8>().ok())
            .collect(),
    };

    let mut boards: Vec<[u8; 25]> = Vec::new();
    let mut num_parsed_values = 0;
    let mut board_buffer: [u8; 25] = [0; 25];

    loop {
        match input_data.next() {
            None => break,
            Some(line) => {
                // Skip empty line above bingo board(s)
                if line.is_empty() {
                    continue;
                }

                line.split(' ')
                    .filter_map(|s| s.parse::<u8>().ok())
                    .for_each(|i| {
                        board_buffer[num_parsed_values] = i;
                        num_parsed_values += 1;
                    });
            }
        }

        if num_parsed_values == 25 {
            boards.push(board_buffer);
            num_parsed_values = 0;
        }
    }

    println!(
        "Winning bingo score: {0}",
        winning_bingo_score(&rolls, &boards)
    );
}

fn winning_bingo_score(rolls: &[u8], boards: &[[u8; 25]]) -> u16 {
    return 0;
}
