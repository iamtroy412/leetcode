struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // Init two variables, one for the running sum,
        // and another vector to hold our running some at a
        // given position.
        let mut sum = 0;
        let mut res = Vec::new();

        // Create an Iterator out of the values of the `nums` vec
        // and loop through them
        for num in nums.iter() {
            // Add the value we're on to the running sum
            sum += num;
            // Add the current running sum to the new vector
            res.push(sum);
        }

        // Return the results vector
        res
    }
}

fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
    println!("{:?}", Solution::running_sum(vec![1, 1, 1, 1, 1]));
    println!("{:?}", Solution::running_sum(vec![3, 1, 2, 10, 1]));
}
