// This was my first attempt, most obvious solution.
// Not breaking any records here.

struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        // This vector of Strings will hold our results
        let mut results = Vec::new();

        // Starting at 1, iterate over a list of integers up to,
        // and including, the `n` value.
        for x in 1..=n {
            // If the number is divisible by both 3 and 5
            // add `FizzBuzz` to our results vec
            if x % 3 == 0 && x % 5 == 0 {
                results.push("FizzBuzz".to_owned());
            // If the number is divisible by 3,
            // add `Fizz` to our results vec
            } else if x % 3 == 0 {
                results.push("Fizz".to_owned());
            // If the number is divisible by 5,
            // add `Buzz` to our results vec
            } else if x % 5 == 0 {
                results.push("Buzz".to_owned());
            // Else it doesn't matter what the number is,
            // convert it to a string and add to results vec
            } else {
                results.push(format!("{}", x));
            }
        }

        // Return our result of strings for fizzbuzz
        results
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(3));
    println!("{:?}", Solution::fizz_buzz(5));
    println!("{:?}", Solution::fizz_buzz(15));
}
