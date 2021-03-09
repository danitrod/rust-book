use std::thread;
use std::time::Duration;

// Spawning a thread
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(300));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(300));
//     }

//     // Force spawned thread to finish before exiting
//     handle.join().unwrap();

//     println!("spawned thread is done. exiting");
// }

// Capturing the environment with move
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
