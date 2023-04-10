fn main() {
    println!("Hello, world!");

    another_function(-5);
    create_labeled_measurement(10, 'h');
    println!("{}", expressions_are_weird());
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn create_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions_are_weird() -> i32 {
    // return 5;    This is cool but, what if we just do this:
    5
}