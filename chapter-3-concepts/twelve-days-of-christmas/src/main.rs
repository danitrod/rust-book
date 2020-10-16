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
    "And a partridge in a pear tree.",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five gold rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

const POSITIONS: [&str; 12] = [
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eight", "Ninth", "Tenth",
    "Eleventh", "Twelfth",
];

fn main() {
    println!(
        "On the {} day of Christmas my true love sent to me\nA partridge in a pear tree.\n",
        POSITIONS[0]
    );
    for i in 1..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            POSITIONS[i]
        );
        for j in (0..i + 1).rev() {
            println!("{}", LYRIC_OBJECTS[j]);
        }
        println!();
    }
}
