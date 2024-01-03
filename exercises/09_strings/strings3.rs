// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    let mut output = String::new();
    let len = input.len();

    for i in 0..len {
        let mut array: [char; 3] = ['&'; 3];
        let mut j = 0usize;

        if i == 0{
            array[0] = '*';
            j = 1;
            let slice = &input[i..i + 2];
            for c in slice.chars() {
                array[j] = c;
                j += 1;
            }
        } else if i == len - 1 {
            array[2] = '*';
            let slice = &input[i-1..];
            for c in slice.chars() {
                array[j] = c;
                j += 1;
            }
        } else {
            let slice = &input[i - 1..i + 2];
            for c in slice.chars() {
                array[j] = c;
                j += 1;
            }
        }

        if array[1] == ' ' && array[0] != ' ' && array[2] != ' '{
            output.push(array[1]);
        } else if array[1] != ' ' {
            output.push(array[1]);
        }
    }
    output
}

fn compose_me(input: &str) -> String {
    let mut output = String::new();
    input.clone_into(&mut output);
    output.push_str(" world!");
    output
}

fn replace_me(input: &str) -> String {
    let mut output = String::new();
    input.clone_into(&mut output);

    if input.rfind("cars") != None {
        output = input.replace("cars", "balloons");
    } else if input.rfind("balloons") != None {
        output = input.replace("balloons", "cars");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!      "), "Hello!");
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
