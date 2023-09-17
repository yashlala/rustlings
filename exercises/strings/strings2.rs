// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

fn main() {
    // WTF? Understand types of strings.
    // OK I get it now. When you pass something in to a function, rust will try to match the
    // type (like implicit arg type conversion in C). But rust is a bit more formal about it.
    // It just calls "deref" method on the provided argument. Hopefully that function(s) will
    // return something useful! (really `deref()` is overloaded so many options to conver to).
    // When we do `*x` it calls "deref", when we pass as an argument it passes deref, etc.
    // TODO think more.

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
