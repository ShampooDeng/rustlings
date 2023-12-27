// variables3.rs
//
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x: u16;
    x = 65535;
    // 65536 will overflow.
    println!("Number {}", x);
}
