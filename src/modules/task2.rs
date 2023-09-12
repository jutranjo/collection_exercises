use std::collections::HashSet;

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


pub fn do_task2(){
    let nonlatin = String::from("first test apple barva šestilo čmrlj šeleshamer vrt");
    println!("{nonlatin}");

    let latin = string2pig_latin(&nonlatin);
    let latin2 = string2pig_latin_better(&nonlatin);
    println!("{latin}");
    println!("{latin2}");
}