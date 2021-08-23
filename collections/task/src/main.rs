use rand::prelude::*;
use std::collections::HashMap;
fn main() {
    let mut rng = rand::thread_rng();
    let mut vectr: Vec<i32> = (0..1000).collect();
    vectr.shuffle(&mut rng);
    // println!("{:?}", vectr);
    println!("Mean: {}", get_average(&vectr));
    let sorted = sort_slow(&vectr);
    // println!("Sorted {:?}", sorted);
    let median = get_median(&sorted);
    println!("Median: {}", median);
    // will be mode element
    vectr.push(3);
    let mode = get_mode(&vectr);
    println!("Mode: {}", mode);
    let pig_latin = convert_pig_latin(&String::from("This is some text"));
    println!("Pig latin text: {}", pig_latin)
}

fn get_average(arr: &Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    for i in arr {
        counter += i;
    }
    counter / (arr.len() as i32)
}

fn get_median(arr: &Vec<i32>) -> i32 {
    sort_slow(arr)[(arr.len() / 2) as usize]
}

fn sort_slow(arr: &Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = arr.to_vec();
    let mut swaps = 1;
    let mut prev_swaps = 0;
    while swaps - prev_swaps > 0 {
        prev_swaps = swaps;
        let length = sorted.len();
        for i in 0..length {
            if i == length - 1 {
                break;
            } else if sorted[i] > sorted[i + 1] {
                let before = sorted[i];
                sorted[i] = sorted[i + 1];
                sorted[i + 1] = before;
                swaps += 1;
            }
        }
    }
    sorted
}

// mode = most common element
fn get_mode(arr: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut counter = 0;
    let mut most_common_element = arr[0];
    for (key, value) in map {
        if value > counter {
            counter += 1;
            most_common_element = *key;
        }
    }
    most_common_element
}

pub enum Consonants {
    t(String),
}

fn convert_pig_latin(text: &String) -> String {
    let mut pig_latin = String::from("");
    for word in text.split_whitespace() {
        //let mut word_chars = &word.chars()[];
        let first_character = &word.chars().next();
        let vowel = match &first_character {
            Some('a') => "a",
            Some('e') => "e",
            Some('i') => "i",
            Some('o') => "o",
            Some('u') => "u",
            Some('A') => "a",
            Some('E') => "e",
            Some('I') => "i",
            Some('O') => "o",
            Some('U') => "u",
            _ => "",
        };
        let new_word = match vowel {
            "" => String::from(word) + "-hay",
            _ => word[1..].to_string() + "-" + &word[0..1] + "ay",
        };

        pig_latin = pig_latin + &new_word + " ";
    }
    pig_latin
}
