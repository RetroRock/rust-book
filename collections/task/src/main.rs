use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vectr: Vec<i32> = (0..10000).collect();
    vectr.shuffle(&mut rng);
    println!("{:?}", vectr);
    println!("Mean {}", get_average(&vectr));
    let sorted = sort_slow(&vectr);
    println!("Sorted {:?}", sorted);
}

fn get_average(arr: &Vec<i32>) -> i32 {
    let mut counter: i32 = 0;
    for i in arr {
        counter += i;
    }
    counter / (arr.len() as i32)
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
