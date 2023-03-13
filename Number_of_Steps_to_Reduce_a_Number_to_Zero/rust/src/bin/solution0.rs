// My first soltuion for this problem. Again, not the most optimal yet.

struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        // Init two mutable variables, one for counting the number
        // of steps we've taken, and the second for our running
        // total. This starts out as a copy of the initial `num`.
        let mut total = num;
        let mut steps = 0;

        // While our running total is greater than zero,
        // process our running total and count the number
        // of steps
        while total > 0 {
            // If the running total is an even number,
            // increment the steps and divide the running total in half
            if total % 2 == 0 {
                steps += 1;
                total /= 2;
            // If the running total is an odd number,
            // increment the steps and subtract 1 from the running total
            } else {
                steps += 1;
                total -= 1;
            }
        }
        // Return the total number of steps we've taken
        // to get to 0.
        steps
    }
}

fn main() {
    println!("Input: 14, Output: {}", Solution::number_of_steps(14));
    println!("Input: 8, Output: {}", Solution::number_of_steps(8));
    println!("Input: 123, Output: {}", Solution::number_of_steps(123));
}
