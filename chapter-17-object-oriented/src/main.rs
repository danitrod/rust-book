use std::collections::HashSet;

// Encapsulation: don't allow the user to access the object's internals
// Use case: struct to cache average value for a number list
// If we need to change list type to a HashSet for example,
// the public API doesn't necessarily change.
// In this case, the `remove` method has to change.
pub struct AveragedCollection {
    // list: Vec<i32>,
    list: HashSet<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            // list: Vec::new(),
            list: HashSet::new(),
            average: 0.,
        }
    }

    pub fn add(&mut self, value: i32) {
        // self.list.push(value);
        self.list.insert(value);
        self.update_average();
    }

    // pub fn remove(&mut self) -> Option<i32> {
    pub fn remove(&mut self, val: i32) -> Option<i32> {
        // let result = self.list.pop();
        let result = self.list.take(&val);
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut ac = AveragedCollection::new();

    ac.add(5);
    ac.add(10);

    println!("avg: {}", ac.average());
}

// Inheritance: Rust doesn't provide straight-forward inheritance for structs,
// but does provide some features such as traits, which can be used to
// do some of the things that inheritance does.

// With generics and traits, one can have polymorphism with Rust.

// Traits are more granular than inheritance: you can share only the
// needed parts between two structures.
