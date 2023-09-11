use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::vec;

use rand::Rng;

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

fn string2pig_latin(in_string: &String) -> String{

    let vowels = ['a','e','i','o','u'
                 ,'A','E','I','O','U'];

    let mut return_string = String::new();

    for word in in_string.split_whitespace(){
        let added_word = if word.starts_with(vowels){
            let mut new_word: String = word.to_string();
            new_word.push_str("-hay");
            
            new_word
        }
        else {
            let mut words_chars = word.chars();
            let moved_char = words_chars.next();
            let mut new_word: String = words_chars.collect();
            new_word.push('-');

            match moved_char{
                Some(new_char) => new_word.push(new_char),
                None => new_word.push(' '),
            }
            new_word.push_str("ay");

            new_word
        };
        return_string.push_str(&added_word);
        return_string.push(' ');

    }
    return_string
}

fn string2pig_latin_better(in_string: &str) -> String {
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
        .iter()
        .cloned()
        .collect();

    in_string
        .split_whitespace()
        .map(|word| {
            if let Some(first_char) = word.chars().next() {
                if vowels.contains(&first_char) {
                    format!("{}-hay", word)
                } else {
                    let rest: String = word.chars().skip(1).collect();
                    format!("{}-{}ay", rest, first_char)
                }
            } else {
                String::from(word)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn task1(){
    let list_length = 1000;
    let upper_bound = 100;
    let list_of_integers = generate_list(list_length,upper_bound);

    let median = calculate_median(&list_of_integers);
    println!("Median = {}", median);

    let mode = calculate_mode(&list_of_integers);
    println!("mode = {:?}",mode);
}

fn task2(){
    let nonlatin = String::from("first test apple barva šestilo čmrlj šeleshamer vrt");
    println!("{nonlatin}");

    let latin = string2pig_latin(&nonlatin);
    let latin2 = string2pig_latin_better(&nonlatin);
    println!("{latin}");
    println!("{latin2}");
}

fn task3(){
    println!("Enter employee and department. Example: \"Add Sally to Engineering\" or \"Add Amir to Sales\" ");

    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");     
        
        match  input.split_whitespace().collect::<Vec<&str>>().as_slice(){
            ["add", name, "to", department] => {
                println!("name = {}, department = {}",name, department);
                employee_map.entry(department.to_string()).and_modify(|list| list.push(name.to_string())).or_insert(vec![name.to_string()]);
            },
            ["display"] => {
                for (department, names) in &employee_map {
                    println!("department: {}, employees: {:?}",department,names);
                }
            },
            ["quit"] => break,
            _ => println!("Please enter text"),
        }
    }
}

fn main() {

    //Task 1
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    task1();

    //Task 2
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    task2();


    //Task 3
    //Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    //For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    task3();


}
