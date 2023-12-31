mod modules;
use modules::{task1::do_task1, task2::do_task2, task3::do_task3};

//use modules::task1::do_task1;

fn main() {
    //Task 1
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    do_task1();

    //Task 2
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    do_task2();

    //Task 3
    //Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    //For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    do_task3();
}
