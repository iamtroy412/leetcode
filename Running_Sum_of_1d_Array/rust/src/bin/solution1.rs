struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // Init a mutable variable that will store our running
        // sum at each step.
        let mut sum = 0;
        // Create an Iterator out of our input vector
        nums.into_iter()
            // Use `map` to create a new iterator based on the output
            // of the closure. In the closure, take the current value
            // from `nums` and add it to the running sum. Return the
            // running sum back to the map which will be the element
            // in the new vector.
            .map(|x| {
                sum += x;
                sum
            })
            // Turn our new Iterator into a collection
            .collect()
    }
}

fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
    println!("{:?}", Solution::running_sum(vec![1, 1, 1, 1, 1]));
    println!("{:?}", Solution::running_sum(vec![3, 1, 2, 10, 1]));
}
