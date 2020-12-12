use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-12.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    part_one(&lines);
    part_two(&lines);
}


fn part_one(lines: &Vec<String>) {
    let mut position: (isize, isize) = (0, 0);
    let mut degrees = 90;

    for line in lines {
        let command = &line[0..1];
        let value: isize = line[1..].parse().unwrap();
        match command {
            "N" => position = (position.0, position.1 + value),
            "S" => position = (position.0, position.1 - value),
            "E" => position = (position.0 + value, position.1),
            "W" => position = (position.0 - value, position.1),
            "R" => {
                degrees += value;
                degrees = degrees % 360;
                degrees = (degrees + 360) % 360;
            },
            "L" => {
                degrees -= value;
                degrees = degrees % 360;
                degrees = (degrees + 360) % 360;
            },
            "F" => {
                match degrees {
                    0 => position = (position.0, position.1 + value),
                    90 => position = (position.0 + value, position.1),
                    180 => position = (position.0, position.1 - value),
                    270 => position = (position.0 - value, position.1),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    println!("first {:?}", position.0.abs() + position.1.abs());
}

fn part_two(lines: &Vec<String>) {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 1;
    let mut wy = 10;

    for line in lines {
        let command = &line[0..1];
        let value: isize = line[1..].parse().unwrap();
        match command {
            "N" => wx += value,
            "S" => wx -= value,
            "E" => wy += value,
            "W" => wy -= value,
            "R" => {
                let (a, b) = rot(wx, wy, value);
                wx = a;
                wy = b;
            },
            "L" => {
                let (a, b) = rot(wx, wy, 360 - value);
                wx = a;
                wy = b;
            },
            "F" => {
                x += wx * value;
                y += wy * value;
            }
            _ => unreachable!(),
        }
    }

    println!("second {:?}", x.abs() + y.abs());
}


fn rot(x: isize, y: isize, d: isize) -> (isize, isize) {
  match d {
    90  => (-y,  x),
    180 => (-x, -y),
    270 => ( y, -x),
    _ => unreachable!(),
  }
}
