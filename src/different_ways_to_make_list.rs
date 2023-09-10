use rand::Rng;

fn main() {
    
    let list_length = 5;
    let mut list_of_integers: Vec<i32> = Vec::new();

    for _ in 0..list_length {
        let new_number=rand::thread_rng().gen_range(1..=100);
        println!("{new_number}");
        list_of_integers.push(new_number);
    }

    for number in list_of_integers{
        print!("{number} ");
    }
    println!("");

    let list_of_integers2: Vec<_> = (0..=list_length).map(|_| {
        rand::thread_rng().gen_range(0..=100)
    }).collect();

    println!("list of integers: {:?}",list_of_integers2);
    
    let list_of_integers2 = (0..=list_length).map(|_| {
        rand::thread_rng().gen_range(0..=100)
    }).collect::<Vec<_>>();

    println!("list of integers: {:?}",list_of_integers2);
        
}





//use std::io;

//use std::cmp::Ordering;
    //let mut list_of_integers: Vec<i32> = Vec::with_capacity(list_length);
    //let list_of_integers: [i32, list_length];
        //list_of_integers.push(new_number)