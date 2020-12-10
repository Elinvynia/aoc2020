use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-9.txt").unwrap();
    let lines: Vec<u128> = BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let mut target = 0;

    for (index, current) in lines.iter().skip(25).enumerate() {
        let index = index + 25;
        let available = &lines[index - 25..index];
        if !is_sum(available, current) {
            println!("first nonsum is {:?}", current);
            target = *current;
            break;
        }
    }

    let max = lines.len();
    for (index, _) in lines.iter().enumerate() {
        for x in index..max {
            if lines[index..x + 1].iter().sum::<u128>() == target {
                println!("{:?}", &lines[index..x + 1]);
                let mi = lines[index..x + 1].iter().min().unwrap();
                let ma = lines[index..x + 1].iter().max().unwrap();
                println!("min {:?}, max {:?}", mi, ma);
                println!("sum {:?}", mi + ma);
                break;
            };
        }
    }
}


fn is_sum(available: &[u128], num: &u128) -> bool {
    for x in available {
        for y in available {
            if x + y == *num {
                return true
            }
        }
    }

    return false
}
