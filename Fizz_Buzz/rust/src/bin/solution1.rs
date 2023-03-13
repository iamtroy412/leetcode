// Second attempt - using Rusts `match` and iterator functions

struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        // Starting at 1, create an iterator of i32 up to,
        // and including, the `n` target value.
        (1..=n)
            // `into_iter` is the generic iterator function, and will return
            // either T, &T, or &mut T depending on context.
            // In this case it will be `T: i32`
            .into_iter()
            // `map` is run on an iterator, takes a closure as an argument,
            // and returns a new iterator that runs the closure on each
            // element of the original iterator.
            // `map` is similar to `for` except that it's lazy. If you're looping
            // for some sort of side effect (ie println), it's more idomatic to use
            // the `for` loop.
            // In this problem, we are basically doing a type conversion with `map`.
            // i32 -> String
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
            .map(|x| match (x % 3 == 0, x % 5 == 0) {
                // The above match statement runs on the current elements value,
                // and evaluates the (bool, bool) expressions, and returns a
                // different `String` value depending on the result.
                //
                // The first case, both are true meaning the number is divisible
                // by both 3 and 5, so return the FizzBuzz string.
                (true, true) => "FizzBuzz".to_string(),
                // The number is only divisible by 3, return the Fizz string.
                (true, false) => "Fizz".to_string(),
                // The number is only divisible by 5, return the Buzz string
                (false, true) => "Buzz".to_string(),
                // The number is NOT divisible by 3 or 5, so just return
                // the number as a `String`.
                _ => x.to_string(),
            })
            // Now that we have used `map` to generate our instructions for returning
            // a new Iterator, we need to turn that new Iterator into some type of colletion.
            // `collect` is run on an iterator and turns it into a relevant collection.
            // Because `collect` is generic, it can cause problems with automatic
            // type inference. But in this case note we didn't need to specifcy the
            // new collection type because it can be infered by the results of the `match`
            // and the return value of this function. `Vec<String`.
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(3));
    println!("{:?}", Solution::fizz_buzz(5));
    println!("{:?}", Solution::fizz_buzz(15));
}
