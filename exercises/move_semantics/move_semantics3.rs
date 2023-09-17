// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    // passes ownership of vec0 into fill_vec
    // gets ownership of that data back in the form of vec1
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// TODO: What does this do to the original value of vec0? WTF?
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // has ownership
    vec.push(22);
    vec.push(44);
    vec.push(66);

    // send ownership back to caller
    vec
}
