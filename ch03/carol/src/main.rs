fn main() {
    print_song();
}

fn print_song() {
    let days = ["first",
                "second",
                "third",
                "fourth",
                "fifth",
                "sixth",
                "seventh",
                "eighth",
                "ninth",
                "tenth",
                "eleventh",
                "twelfth"];

    let verses = ["A partridge in a pear tree",
                  "Two turtle doves, and",
                  "Three french hens",
                  "Four calling birds",
                  "Five golden rings",
                  "Six geese a-laying",
                  "Seven swans a-swimming",
                  "Eight maids a-milking",
                  "Nine ladies dancing",
                  "Ten lords a-leaping",
                  "Eleven pipers piping",
                  "Twelve drummers drumming"];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", days[day]);
        for n in (0..day+1).rev() {
            println!("{}", verses[n]);
        }
        println!();
    }
}
