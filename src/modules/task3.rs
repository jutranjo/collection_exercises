use std::collections::HashMap;
use std::io;
use std::vec;

pub fn do_task3(){
    println!("Enter employee and department. Example: \"Add Sally to Engineering\" or \"Add Amir to Sales\" ");

    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");     

        match input.split_whitespace().collect::<Vec<&str>>().as_slice(){
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