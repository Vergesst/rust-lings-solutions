fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim_matches(|c: char| c.is_whitespace())
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    format!("{} world", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // let mut vec = Vec::<&str>::new();
    // let mut target = input.trim();
    // let mut start: usize = 0;
    // for end in 0..target.len() {
    //     if target[end..end+1] == *" " {
    //         let mut temp = &target[start..end];
    //         vec.push(temp);
    //         start = end + 1;
    //     }
    //     if end == target.len()-1 {
    //         vec.push(&target[start..end+1]);
    //     }
    // }
    // let mut target = String::new();
    // for x in 0..vec.len() {
    //     let mut temp = vec[x];
    //     if temp == "cars" {
    //         temp = "balloons"
    //     }
    //     let _ = target.push_str(temp);
    //     target.push_str(" ");
    // }
    // let n = target.trim();
    // n.to_string()
    input
        .split_whitespace()
        .map(| string | if string == "cars" {"balloons"} else {string})
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}