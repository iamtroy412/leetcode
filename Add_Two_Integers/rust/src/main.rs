// The 'hello world' of problems.

struct Solution {}

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        // Return the sum of the two numbers
        num1 + num2
    }
}

fn main() {
    println!("Input: num1 = 12, num2 = 5");
    println!("Output: {}", Solution::sum(12, 5));

    println!("Input: num1 = -10, num2 = 4");
    println!("Output: {}", Solution::sum(-10, 4));
}
