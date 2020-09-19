struct Tuple(isize, isize);

struct Animal {
    weight: usize,
    name: String,
    kind: String,
}

fn main() {
    let struct_tuple = Tuple(1, 2);

    // struct_tuple[0] -- wrong

    let tuple = (1, 2);
    println!("Struct tuple: ({} {})", struct_tuple.0, struct_tuple.1);

    // Same goes for regular tuple
    println!("Tuple: ({} {})", tuple.0, tuple.1);

    // Struct update
    let elephant = Animal {
        weight: 50,
        name: String::from("Dumbo"),
        kind: String::from("Mammal"),
    };

    let mut spider = Animal {
        kind: String::from("Insect"),
        ..elephant
    };

    let name = String::from("Black Widow");
    spider.name = name.clone();

    // Field init shorthand
    let spider2 = Animal { name, ..spider };
}
