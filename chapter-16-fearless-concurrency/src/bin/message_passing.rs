use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    let val = String::from("hi2");
    tx.send(val).unwrap();
    // val can't be used anyomre: its ownership has been moved by sending
    // println!("val is {}", val);
  });

  // recv waits till a message is received or channel close blocking the thread;
  // try_recv sees if theres any message and returns immediatly
  let received = rx.recv().unwrap();
  println!("Got: {}", received);

  // Sending with interval
  let (tx, rx) = mpsc::channel();

  // Using multiple producers
  let tx1 = tx.clone();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}
