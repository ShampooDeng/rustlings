// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// NOTE: fill(vec0) will move the data owned by vec0 to vec inside fill_vec.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // let mut vec1 = fill_vec(vec0.clone());
    // Another way:
    let mut vec2: Vec<i32> = Vec::new();
    vec0.clone_into(&mut vec2);
    let mut vec1 = fill_vec(vec2);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
