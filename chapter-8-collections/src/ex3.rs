// - 3. Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales”.
// Then let the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.

pub fn ex3() {
  use std::collections::HashMap;

  println!("Welcome to Rustacean Corp!");
  let mut company: HashMap<String, Vec<String>> = HashMap::new();
  loop {
    println!();
    println!("1 - Add employee to department");
    println!("2 - Retrieve list of people in specific department");
    println!("3 - Retrieve list of all people in the company by department");
    let mut inp = number_input();
    while inp > 3 {
      inp = number_input();
    }
    match inp {
      1 => {
        println!("Enter the name of the employee");
        let name = input();
        println!("Enter the department of the employee");
        let department = input();
        if company.contains_key(&department) {
          match company.get_mut(&department) {
            Some(dep_list) => {
              for i in 0..dep_list.len() {
                if name <= dep_list[i] {
                  dep_list.insert(i, name);
                  break;
                } else if i == dep_list.len() - 1 {
                  dep_list.push(name.clone());
                }
              }
            }
            None => panic!("Unexpected error"),
          }
        } else {
          company.insert(department, vec![name]);
        }
      }
      2 => {
        println!("Enter the name of the department");
        let dep = input();
        match company.get(&dep) {
          Some(list) => println!("People in {}: {:?}", dep, list),
          None => println!("Department not found."),
        }
      }
      3 => {
        println!("All people in the company:");
        for (dep, list) in &company {
          for person in list.iter() {
            println!("{} - {}", dep, person);
          }
        }
      }
      _ => (),
    }
  }
}

fn number_input() -> u8 {
  use std::io::stdin;
  eprint!(">>");
  loop {
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect("Failed to read line");
    let inp: u8 = match inp.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        eprint!(">>");
        continue;
      }
    };
    return inp;
  }
}

fn input() -> String {
  use std::io::stdin;
  eprint!(">>");
  loop {
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect("Failed to read line");
    let inp: String = match inp.trim().parse() {
      Ok(s) => s,
      Err(_) => {
        eprint!(">>");
        continue;
      }
    };
    return inp;
  }
}
