// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

/**
 This is some type of constant.
- Testing Markdown 1
- Testing Markdown 2

# Some title
some text

## Some Subtitle
clickable link [SOME_CONSTANT]
 **/

const SOME_CONSTANT: i32 = 548;

fn count_to_5() -> i32 {
    let mut count = 0;
    loop {
        if count == 5 {
            break;
        }
        count += 1;
    }
    println!("{}", SOME_CONSTANT);
    count
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
