use rand::Rng;
use std::collections::HashMap;

fn generate_list(list_length: i32,upperbound: i32) -> Vec<i32>{
    (0..=list_length-1).map(|_| {
        rand::thread_rng().gen_range(0..=upperbound)
    }).collect()
}

fn calculate_median(input_vec: &Vec<i32>) -> i32 {
    
    let mut sorted_numbers = input_vec.clone();
    sorted_numbers.sort_unstable();
    let vector_length = sorted_numbers.len();
    if vector_length%2 == 0 {
        (sorted_numbers[vector_length/2 -1] + sorted_numbers[vector_length/2])/2
    }
    else {
        sorted_numbers[vector_length/2]
    }
}

fn calculate_mode(numbers: &Vec<i32>) -> Vec<i32>{
    let mut modemap: HashMap<i32, i32> = HashMap::new();

    for element in numbers {
        modemap.entry(*element).and_modify(|e| *e=*e+1).or_insert(0);
    }

    let mut mode: Vec<i32> = Vec::new();

    let mut highest = 0;

    for (_key, &value) in &modemap {
        if value>highest {
            highest = value;
        }
    }
    for (key, value) in modemap {
        if value == highest {
            mode.push(key);
        }
    }

    mode
}

pub fn do_task1(){
    let list_length = 1000;
    let upper_bound = 100;
    let list_of_integers = generate_list(list_length,upper_bound);

    let median = calculate_median(&list_of_integers);
    println!("Median = {}", median);

    let mode = calculate_mode(&list_of_integers);
    println!("mode = {:?}",mode);
}
