// - 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

const STRING: &str = "olá mundo, eu sou uma string em utf-8 usando çá®ãç†é®ê$ especiais";
const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub fn ex2() {
  let mut pig_latin = String::new();

  for w in STRING.split_whitespace() {
    let mut word_chars = w.chars();
    let first_char = match word_chars.next() {
      Some(c) => c,
      None => panic!("Empty string"),
    };
    if VOWELS.contains(&first_char) {
      // Starts with vowel. Push -hay to the end.
      pig_latin.push(first_char);
      for c in word_chars {
        pig_latin.push(c);
      }
      pig_latin.push_str("-hay");
    } else {
      // Starts with consonant. Push -{first_char}ay to the end.
      for c in word_chars {
        pig_latin.push(c);
      }
      pig_latin.push_str(format!("-{}ay", first_char).as_str());
    }
    // Separate the word
    pig_latin.push(' ');
  }

  println!("{}", pig_latin);
}
