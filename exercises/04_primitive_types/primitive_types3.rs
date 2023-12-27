// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // it seems like that an array is fixed size.
    let mut a: [i32; 100] = [100;100]; // initialize it with the same value.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("{}", a[20])
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
