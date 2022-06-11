use std::collections::HashMap;
use rand::Rng;

const SIZE: usize = 10000;

pub fn run() {
    println!("===== FIRST =====");

    // Seed the rand gen
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::new();

    for _ in 1..=SIZE{
        numbers.push(rng.gen_range(1..=100));
    }

    // println!("unsorted {:?}", numbers);
    numbers.sort();
    // println!("sorted {:?}", numbers);

    // median
    let median: u32;
    let middle: usize;
    if SIZE % 2 == 0 {
        // even
        middle = SIZE / 2;
        median = &numbers[middle] + &numbers[middle+1];
    } else {
        // odd
        middle = SIZE / 2;
        median = numbers[middle]; 
        println!("Miiddle {}", middle);
    }

    println!("Median: {}", median);

    match mode(&numbers) {
        Mode::OneMode(m) => println!("Mode: {}", m),
        Mode::NoMode => println!("No Mode"),
        Mode::MultipleModes(modes) => println!("Modes: {:?}", modes)
    }
}

enum Mode {
    NoMode,
    OneMode(u32),
    MultipleModes(Vec<u32>)
}

fn mode(list: &Vec<u32>) -> Mode {
    let mut occur = HashMap::new();
    for num in list {
        let count = occur.entry(*num).or_insert(0);
        *count += 1;
    }

    // check for no mode
    let values: Vec<u32> = occur.values().cloned().collect();
    let start = &values[0];
    let mut no_mode = true;
    for x in 1..values.len() {
        if values[x] != *start {
            no_mode = false; 
            break;
        }
    }

    if no_mode == true {
        return Mode::NoMode;
    }

    let mut greatest: Vec<u32> = vec![];
    for (k, v) in &occur {
        if greatest.is_empty() {
            greatest.push(*k);
        } else {
            if v > &occur[&greatest[0]] {
                greatest = vec![*k];
            } else if v == &occur[&greatest[0]] {
                greatest.push(*k);
            }
        }
    }

    if greatest.len() == 1 {
        Mode::OneMode(greatest[0])
    } else {
        Mode::MultipleModes(greatest)
    }
}
