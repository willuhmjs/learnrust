fn main() {
    let items = ["A partridge in a pear tree!", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five gold rings,", "Six geese-a-laying,", "Seven swans-a-swimming,", "Eight maids-a-milking,", "Nine ladies dancing,", "Ten lords-a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];
    for (i, item) in items.iter().enumerate() {
        println!("On the {} day of Christmas my true love gave to me:\n{item}", {i+1});
        let mut x = i;
        while x > 0 {
            x -= 1;
            println!("{}",items[x]);
        }
        println!("\n");
    }
}
