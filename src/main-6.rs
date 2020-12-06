use std::fs;

fn main() {
    let string = fs::read_to_string("inputs/input-6.txt").unwrap();
    let mut answers: Vec<Vec<String>> = string.split("\n\n").map(|s| {
        let split: Vec<String> = s.split("\n").map(|a| a.to_string()).collect();
        split
    }).collect();

    answers.reverse();
    answers[0].pop().unwrap();

    let mut any = 0;
    let mut every = 0;

    for answer in answers {
        let mut unique = vec![];
        let mut count = vec![];
        let should = answer.len();

        for person in answer {
            for ch in person.chars() {
                if !unique.contains(&ch) {
                    unique.push(ch)
                }
                count.push(ch)
            }
        }

        for uniq in unique.iter() {
            if count.iter().filter(|&n| n == uniq).count() == should {
                every += 1
            }
        }

        any += unique.len();

    }

    println!("any - {:?}", any);
    println!("every - {:?}", every);
}

