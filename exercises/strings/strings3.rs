// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


fn trim_me(input: &str) -> String { // borrow and check :)
    return input.trim().to_string();
}

fn compose_me(input: &str) -> String {
    return format!("{input} world!"); // The power of macros! Would be nice to have syntax highlighting
}

fn replace_me(input: &str) -> String {
    return input.replace("cars", "balloons"); // JS like string replace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // What doe test macros do?
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
