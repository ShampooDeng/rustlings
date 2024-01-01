// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let a = 5;
    let b = &a;
    println!("{:0>5}", b);
    println!("{}", *b);
    assert_eq!(5, *b);
    // Try to uncomment this line.
    // assert_eq!(5, b);

    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
