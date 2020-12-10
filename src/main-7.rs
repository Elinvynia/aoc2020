use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, println};

fn main() {
    let file = File::open("inputs/input-7.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let mut bags: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    let mut total = 0;

    for line in lines {
        let two: Vec<&str> = line.split(" bags contain ").collect();
        let contains: Vec<&str> = two[1].split(", ").map(|mut s| {
            s = s.trim_end_matches(" bag");
            s = s.trim_end_matches(" bag.");
            s = s.trim_end_matches(" bags");
            s = s.trim_end_matches(" bags.");
            s
        }).collect();
        let mut tuples: Vec<(String, u64)> = vec![];
        for contain in contains.into_iter() {
            let mut c = contain.splitn(2, ' ');
            let number = c.next().unwrap();
            let name = c.next().unwrap();
            if number == "no" {
                tuples.push(("none".into(), 0))
            }
            else {
                tuples.push((name.to_string(), number.parse().unwrap()));
            }
        }

        bags.insert(two[0].into(), tuples);
    }

    for (name, _) in bags.iter() {
        if contains_bag(name, &bags) {
            total += 1
        };
    }

    println!("colors {:?}", total);
    println!("in shiny {:?}", child("shiny gold", &bags));
}


fn contains_bag(name: &str, bags: &HashMap<String, Vec<(String, u64)>>) -> bool {
    let contains = bags.get(name).unwrap();

    if contains.len() == 1 && contains[0].0 == "none" {
        return false
    };

    for (n, _) in contains {
        if n == "shiny gold" {
            return true
        }
    };

    let mut has = false;
    for (n, _) in contains {
        has = has || contains_bag(n, bags)
    }

    return has
}


fn child(name: &str, bags: &HashMap<String, Vec<(String, u64)>>) -> u64 {
    let contains = bags.get(name).unwrap();

    if contains.len() == 1 && contains[0].0 == "none" {
        return 0
    }

    let mut total = 0;
    for (n, amount) in contains {
        total += amount;
        total += amount * child(n, bags);
    }

    return total
}
