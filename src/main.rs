use std::{io, process::exit};

use board::{Board, BOARD_SIZE, StoneType};

mod board;

fn main() {
    let mut board: Board = board::Board::created();
    update_screen(&board.board);
    let mut put_stone_count = 1;

    loop {
        let my_stone_type = get_my_turn_stone_color(&mut put_stone_count);
        board.updated_candidate_stone_pos(&my_stone_type);
        update_screen(&board.candidate_board);
        // put black stone
        let (x, y);
        match got_input_pos() {
            Some((x_input, y_input)) => {
                (x, y) = (x_input, y_input)
            },
            None => {
                print_input_error_message();
                continue;
            },
        };

        if was_input_invalid(&mut board, &x, &y) {
            print_input_error_message();
            continue;
        }
        // if input stone position can't turn over any stones, need again
        if !&board.add_input_stone_pos(&x, &y, &my_stone_type) {
            println!("can't turn over any stones, choose another position");
            continue;
        }
        update_screen(&board.board);
        put_stone_count += 1;

        // game set
        if board.is_no_pos_to_put_stones() {
            let (count_black_stones, count_white_stones) = board.game_result();
            print_game_result(count_black_stones, count_white_stones);
            exit(0);
        }
    }
}

fn got_input_pos() -> Option<(usize, usize)> {
    let mut input_xy = String::new();
    io::stdin()
        .read_line(&mut input_xy)
        .expect("failed to read from stdin");
    let mut input_xy_iter = input_xy.split_whitespace();

    let x_str = input_xy_iter.next()?;
    let y_str = input_xy_iter.next()?;

    let x: usize = x_str.parse::<usize>().ok()?;
    let y: usize = y_str.parse::<usize>().ok()?;

    Some((x, y))
}

fn was_input_invalid(board: &mut Board, x: &usize, y: &usize) -> bool {
    let mut is_invalid_input = false;

    // invalid: input position has stone already
    if board.is_pos_has_stone_already(&x, &y) {
        is_invalid_input = true;
    }
    // invalid: input position is outside of board
    if BOARD_SIZE <= *x || BOARD_SIZE <= *y {
        is_invalid_input = true;
    }

    if is_invalid_input {
        return true;
    }

    false
}

// black: odd count, white: even count
fn get_my_turn_stone_color(count: &mut i32) -> StoneType {
    if *count % 2 == 1 { return StoneType::BlackStone }
    StoneType::WhiteStone
}

fn print_input_error_message() {
    println!("Invalid input! Please input again.");
}

fn update_screen(board: &[[& str;BOARD_SIZE];BOARD_SIZE]) {
    // clear the screen
    print!("{}[2J", 27 as char);
    println!();

    // decorate screen: show x position
    print!("  ");
    for x_index in 0..BOARD_SIZE {
        print!("{} ", x_index);
    }
    println!();

    let mut count: String = "1".to_owned();
    // decorate screen: show y position and stones position
    let mut y_index = 0;
    for line in board {
        print!("{} ", y_index);
        y_index +=1;

        for mark in line {
            if mark == &"*" {
                print!("{} ", count);
                let mut count_i32: i32 = count.parse().unwrap();
                count_i32 += 1;
                count = count_i32.to_string();
            } else {
                print!("{} ", mark);
            }
        }
        println!();
    }
}

fn print_game_result(count_black_stones: i32, count_white_stones: i32) {
    let winner: &str = {
        if count_white_stones < count_black_stones { "BLACK" }
        else if count_black_stones < count_white_stones { "WHITE" }
        else { "DRAW" }
    };

    println!("Winner is {}", winner);
    println!("black stones: {}", count_black_stones);
    println!("white stones: {}", count_white_stones);
}