use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Initialize the HashMap that will store the targets compliments
        // and the location from `nums` where the other compliment is
        // key: target-num=compliment, value: current index
        let mut compliments: HashMap<i32, i32> = HashMap::new();

        // Start iterating over the length of the `nums` array
        for i in 0..nums.len() {
            // Check to see if the value of `nums` at the current index
            // is already in the compliments HashMap
            match compliments.get(&nums[i]) {
                // If the value of `nums` we're at is already in the HashMap,
                // retun the index from `nums` that we found it's compliment (the value)
                // and return the current index of `nums` we're at.
                // This is the solution, the two indicies of `nums` that add up to target.
                Some(&x) => return vec![x, i as i32],
                // If the value of `nums` we're at isn't in the HashMap, that means
                // we haven't seen it's compliment yet, so add it to the HashMap,
                // along with the index we're currently at.
                None => compliments.insert(target - nums[i], i as i32),
            };
        }

        // This point in the code should be unreachable,
        // but we need to return a Vec<i32> of something
        // to satisfy the fn return type.
        return vec![-1, -1];
    }
}

fn main() {
    let res = Solution::two_sum(vec![3, 2, 4], 6);

    println!("Res: {:?}", res);
}
