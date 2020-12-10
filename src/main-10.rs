use std::{fs::File, io::{BufRead, BufReader}, collections::BTreeMap};

fn main() {
    let file = File::open("inputs/input-10.txt").unwrap();
    let lines: Vec<usize> = BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap()).collect();

    part_one(&lines);


    let mut cache: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sorted = lines.clone();
    sorted.push(0);
    sorted.push(3 + *lines.iter().max().unwrap());
    sorted.sort();

    println!("combinations {:?}", rec_find(0, &sorted, &mut cache));
}

fn rec_find(current: usize, graph: &Vec<usize>, cache: &mut BTreeMap<usize, usize>) -> usize {
    if cache.contains_key(&current) {
        return *cache.get(&current).unwrap();
    };

    if current == graph.len() - 1 {
        return 1;
    };

    let mut result = 0;

    for x in (current + 1)..=(current + 3) {
        if x < graph.len() && graph[x] - graph[current] <= 3 {
            result += rec_find(x, &graph, cache);
        }
    }

    cache.insert(current, result);
    return result
}

fn part_one(lines: &Vec<usize>) {
    let mut ones = 0;
    let mut _twos = 0;
    let mut threes = 0;

    let mut value = 0;

    for _ in 0..=lines.len() {
        let mut one = false;
        let mut two = false;
        let mut three = false;
        if lines.contains(&(value + 1)) {
            one = true;
        }
        if lines.contains(&(value + 2)) {
            two = true;
        }
        if lines.contains(&(value + 3)) {
            three = true;
        }
        if one && two && three {
            value += 1;
            ones += 1;
        } else if one && two {
            value += 1;
            ones += 1;
        } else if one && three {
            value += 1;
            ones += 1;
        } else if two && three {
            value += 2;
            _twos += 1;
        } else if one {
            value += 1;
            ones += 1;
        } else if two {
            value += 2;
            _twos += 1;
        } else if three {
            value += 3;
            threes += 1;
        }
    }

    println!("ones {:?}", ones);
    println!("threes {:?}", threes + 1);
}
