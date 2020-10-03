use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Generic error handling
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Error kind handling
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Unwrap
    let _f = File::open("hello.txt").unwrap(); // Will automatically panic if any error ocurs

    // Expect
    let _f = File::open("hello.txt").expect("Failed to open hello.txt"); // Same as unwrap; choose message
}

use std::io;
use std::io::Read;

// Propagating error
fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using ? operator
fn _read_username_from_file2() -> Result<String, io::Error> {
    // Same functionality as before, more concise
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even more concise
fn _read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
// Even more concsise
fn _read_username_from_file4() -> Result<String, io::Error> {
    // Using the std lib function read_to_string implements the same behavior
    fs::read_to_string("hello.txt")
}

// Using ? in the main function: it must return Result<(), Box<dyn Error>>

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }
