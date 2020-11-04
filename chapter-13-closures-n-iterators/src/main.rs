use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
mod iterators;
use iterators::iterators;
mod counter;

struct Cacher<T, U: Eq + Hash + Copy, V: Copy>
where
    T: Fn(U) -> V,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U: Eq + Hash + Copy, V: Copy> Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures can use variables in their scope
    let x = 4;

    let equal_to_x = |z| z == x; // This wouldn't compile if it was a function

    let y = 4;

    assert!(equal_to_x(y));

    // Closure ownership
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    println!("\nIterators");
    iterators();

    println!("\nCounter");
    counter::main();
}
