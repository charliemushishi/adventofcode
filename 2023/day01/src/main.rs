use std::collections::HashMap;
use std::io;
use std::io::BufRead;

// PART 2 COMPLETE
fn part02(lines: Vec<String>) {

    let numbers_map: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
        .iter()
        .cloned()
        .collect();

    let mut sum = 0;

    for line in &lines {
        let mut first = 0;
        let mut last = 0;
        let mut front = 0;
        let mut back = line.len();

        while front < line.len() && back > 0 && first == 0 {
            for (word, &value) in &numbers_map {
                if line[front..].starts_with(word) && value < 10 {
                    first = value;
                    break;
                }
            }
            front += 1;
        }
        while back > front && last == 0 {
            for (word, &value) in &numbers_map {
                if line[..back].ends_with(word) && value < 10 {
                    last = value;
                    break;
                }
            }
            back -= 1;
        }
        if last == 0 {
            last = first;
        }
        //println!("First: {}, Last: {}", first, last); where are you D8<
        sum += first * 10 + last;
    }
    println!("Part 2!: = {}", sum);
}


//PART 1 COMPLETE
fn part01(lines: &Vec<String>) {

    let sum_of_pair: i32 = lines
        .iter()
        .filter_map(|mess| {
            let num: String = mess
                .chars()
                .filter(|c| c.is_numeric())
                .collect();

            if num.len() > 1 {
                let first_digit = num.chars().next().unwrap();
                let last_digit = num.chars().last().unwrap();
                Some(format!("{}{}", first_digit, last_digit))
            } else if num.len() == 1 {
                Some(format!("{}{}", num, num))
            } else {
                None
            }
        })
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .sum();

    println!("Part 1!: {}", sum_of_pair);
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().filter_map(Result::ok).collect();
    part02(lines.clone());
    part01(&lines);

}
