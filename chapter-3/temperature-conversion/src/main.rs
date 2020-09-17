use std::io;

fn main() {
    // Choose conversion
    eprint!("Type 1 to convert Celsius to Fahrenheit or 2 to convert Fahrenheit to Celsius >>");

    let mut choice: f64;
    loop {
        choice = int_input();

        if choice == 1. || choice == 2. {
            break;
        } else {
            eprint!("Type 1 or 2 >>");
        }
    }

    let input_unit: &str;
    let output_unit: &str;
    if choice == 1. {
        input_unit = "Celsius";
        output_unit = "Fahrenheit";
    } else {
        input_unit = "Fahrenheit";
        output_unit = "Celsius";
    };

    // Temperature input
    eprint!("Type in your temperature in {} >>", input_unit);
    let temp_input = int_input();

    let temp_result: f64;
    if choice == 1. {
        temp_result = temp_input * (9. / 5.) + 32.;
    } else {
        temp_result = (temp_input - 32.) * (5. / 9.);
    }

    println!("Conversion to {}: {:.2}", output_unit, temp_result);
}

fn int_input() -> f64 {
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp: f64 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprint!(">>");
                continue;
            }
        };
        return inp;
    }
}
