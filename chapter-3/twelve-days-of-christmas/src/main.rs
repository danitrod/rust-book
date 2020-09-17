// Lyrics:
// 1 partridge in a pear tree.
// 2 turtle doves,
// 3 French hens,
// 4 calling birds
// 5 gold rings
// 6 geese a-laying
// 7 swans a-swimming
// 8 maids a-milking
// 9 ladies dancing
// 10 lords a-leaping
// 11 pipers piping
// 12 drummers drumming

const LYRIC_OBJECTS: [&str; 12] = [
    "partridge in a pear tree.",
    "turtle doves,",
    "French hens,",
    "calling birds,",
    "gold rings,",
    "geese a-laying,",
    "swans a-swimming,",
    "maids a-milking,",
    "ladies dancing,",
    "lords a-leaping,",
    "pipers piping,",
    "drummers drumming,",
];

const NUMBERS: [&str; 12] = [
    "And a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
    "Twelve",
];

const POSITIONS: [&str; 12] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eight", "Ninth", "Tenth",
    "Eleventh", "Twelfth",
];

fn main() {
    println!(
        "On the {} day of Christmas my true love sent to me\nA {}\n",
        POSITIONS[0], LYRIC_OBJECTS[0]
    );
    for i in 1..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            POSITIONS[i]
        );
        for j in (0..i + 1).rev() {
            println!("{} {}", NUMBERS[j], LYRIC_OBJECTS[j]);
        }
        println!();
    }
}
