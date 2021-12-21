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

fn compute_has_won(board_mask: &[bool; 25]) -> bool {
    // Check horizontals
    for y in (0..25).step_by(5) {
        let mut winning_row = true;
        for x in 0..5 {
            if board_mask[y + x] == false {
                winning_row = false;
                break;
            }
        }

        if !winning_row {
            continue;
        }

        return true;
    }

    // Check verticals
    for x in 0..5 {
        let mut winning_col = true;
        for y in (0..25).step_by(5) {
            if board_mask[x + y] == false {
                winning_col = false;
                break;
            }
        }

        if !winning_col {
            continue;
        }

        return true;
    }

    return false;
}

fn winning_bingo_score(rolls: &[u8], boards: &[[u8; 25]]) -> u16 {
    let mut board_masks: Vec<[bool; 25]> = vec![[false; 25]; boards.len()];

    for (roll_num, &roll) in rolls.iter().enumerate() {
        for (board_num, board) in boards.iter().enumerate() {
            // Mark number on board
            for i in 0..25 {
                if board[i] == roll {
                    board_masks[board_num][i] = true;

                    // Start checking for victory after 5 numbers have been given
                    if roll_num > 5 && compute_has_won(&board_masks[board_num]) {
                        let mut total: u16 = 0;

                        for j in 0..25 {
                            if board_masks[board_num][j] == false {
                                total += board[j] as u16;
                            }
                        }

                        return total as u16 * roll as u16;
                    } else {
                        break; // Numbers do not repeat themselves on board
                    }
                }
            }
        }
    }

    println!("No winning boards!");
    return 0;
}
