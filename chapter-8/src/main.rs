fn _vecs() {
    let mut _test = [1, 2, 3];
    // test[5] = 5; Can't assign
    let mut vtest = _test.to_vec();
    vtest[5] = 5; // Lets assign but panics
    let mut v: Vec<i8> = vec![1, 2, 3];
    v.push(-1);
    println!("{:?} {:?}", v, vtest);
}

fn _strings() {
    // push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    // push char
    let mut s = String::from("lo");
    s.push('l');

    // concat strings with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // String slicing
    let hello = "Здравствуйте";
    // UTF-8 characters may have 1 byte or more than 1. That can cause issues when slicing strings.
    // String indexing is not allowed in Rust.
    let _s = &hello[0..4]; // OK: the first two characters are 2 bytes each, there fore s is Зд
    let _s = &hello[0..1]; // Will panic: as the character З is 2 bytes long, only getting 1 char will panic

    // Solution for unknown char size lengths: iterating
    for c in "नमस्ते".chars() {
        // iterating on chars
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        // iterating on bytes
        println!("{}", b);
    }
}

fn _hash_maps() {
    // Creating a HashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Using collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Collect transorms a tuple into many types, here a HashMap. Zip here brings together the two vecs into a tuple.
    let mut _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // HashMaps copy values of types that have the Copy trait, or owns owned values like Strings
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    // field_value is invalid at this point as they were moved. field_name can still be accessed as it was not moved, only referenced.

    // Getting value from HashMap
    let _score = scores.get(&String::from("Blue"));

    // Iterating HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating HashMap with replacement
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // {"Blue": 25}

    // Adding value only if value does not exist with `entry`
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let value = scores.entry(String::from("Blue")).or_insert(50); // Entry always returns a mutable reference to either the existing or the new value

    println!("blue value: {}", value);
    println!("{:?}", scores);

    // Using HashMap for counting words
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // count reference is lost at each loop
    }

    println!("{:?}", map);
}

// mod ex1;
// mod ex2;
mod ex3;
fn main() {
    // ex1::ex1();
    // ex2::ex2();
    ex3::ex3();
}

// Exercises:

// - 1. Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

// - 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

// - 3. Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales”.
// Then let the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.
