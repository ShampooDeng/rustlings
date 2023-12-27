// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


fn call_me(name:&str){
    //help: function arguments must have a statically known size,
    //borrowed types always have a known size
    println!("How dare you! {}", name);
}

fn main() {
    let user_name = "wm";
    call_me(user_name);
}
