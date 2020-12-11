use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-11.txt").unwrap();
    let mut seats: Vec<Vec<char>> = BufReader::new(file).lines().map(|l| l.unwrap().chars().collect::<Vec<char>>()).collect();
    let mut seats2 = seats.clone();

    let mut change = true;
    while change {
        change = false;
        let mut cloned = seats.clone();
        for (r, row) in seats.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                let neighbours = get_occ_neighbours(&seats, r, c);
                if col == &'L' && neighbours == 0 {
                    cloned[r][c] = '#';
                    change = true;
                }
                if col == &'#' && neighbours >= 4 {
                    cloned[r][c] = 'L';
                    change = true;
                }
            }
        }
        seats = cloned;
    }

    println!("occupied {:?}", seats.iter().map(|v| v.iter().filter(|&x| x == &'#').count()).sum::<usize>());

    let mut change = true;
    while change {
        change = false;
        let mut cloned = seats2.clone();
        for (r, row) in seats2.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                let neighbours = get_see_occ_neighbours(&seats2, r, c);
                if col == &'L' && neighbours == 0 {
                    cloned[r][c] = '#';
                    change = true;
                }
                if col == &'#' && neighbours >= 5 {
                    cloned[r][c] = 'L';
                    change = true;
                }
            }
        }
        seats2 = cloned;
    }

    println!("occupied seen {:?}", seats2.iter().map(|v| v.iter().filter(|&x| x == &'#').count()).sum::<usize>());
}


#[inline(always)]
fn get_occ_neighbours(seats: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut neighbours = vec![];
    let indexes: &[i32] = &[-1, 0, 1];

    for r in indexes {
        for c in indexes {
            if *r == 0 && *c == 0 {
                continue
            }
            if in_bounds(seats, row as i32 + r, col as i32 + c){
                neighbours.push(seats[(row as i32 + *r) as usize][(col as i32 + *c) as usize])
            }
        }
    }

    return neighbours.iter().filter(|&ch| ch == &'#').count()
}


#[inline(always)]
fn get_see_occ_neighbours(seats: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut neighbours = vec![];
    let indexes: &[i32] = &[-1, 0, 1];
    let mut row_check;
    let mut col_check;

    for r in indexes {
        for c in indexes {
            if *r == 0 && *c == 0 {
                continue
            }
            row_check = *r;
            col_check = *c;
            while in_bounds(seats, row as i32 + row_check, col as i32 + col_check) {
                let value = seats[(row as i32 + row_check) as usize][(col as i32 + col_check) as usize];
                if value != '.' {
                    neighbours.push(value);
                    break;
                }
                row_check += r;
                col_check += c;
            }
        }
    }

    return neighbours.iter().filter(|&ch| ch == &'#').count()
}


#[inline(always)]
fn in_bounds(seats: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }

    let max_row = seats.len() as i32;
    let max_col = seats[0].len() as i32;
    if row > max_row - 1 || col > max_col - 1 {
        return false
    }
    return true
}
