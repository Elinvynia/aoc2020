use std::{fs::File, io::{BufRead, BufReader}, println};

fn main() {
    let file = File::open("inputs/input-5.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut max = 0;
    let mut ids: Vec<u64> = vec![];

    for line in lines {
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_col = 0;
        let mut max_col = 7;

        for ch in line.chars() {
            let row_range = max_row - min_row;
            let row_midpoint = row_range / 2 + min_row;
            if ch == 'F' {
                max_row = row_midpoint
            }
            if ch == 'B' {
                min_row = row_midpoint + 1
            }

            let col_range = max_col - min_col;
            let col_midpoint = col_range / 2 + min_col;
            if ch == 'R' {
                min_col = col_midpoint + 1
            }
            if ch == 'L' {
                max_col = col_midpoint
            }
        }

        let row = (max_row - min_row) / 2 + min_row;
        let col = (max_col - min_col) / 2 + min_col;

        let seat = row * 8 + col;
        if seat > max {
            max = seat;
        }
        if row != 0 && row != 127 {
            ids.push(seat);
        }
    }

    println!("highest id {:?}", max);

    ids.sort();

    let mut previous = ids[0] - 1;

    for id in ids.iter() {
        if id - 1 != previous {
            println!("missing seat {:?}", id - 1);
            break;
        }
        previous = *id;
    }

}
