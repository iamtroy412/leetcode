struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        // Create a mutable variable to keep track of
        // our wealthiest customer
        let mut largest: i32 = 0;

        // Loop over each customer in the vector of accounts
        for customer in accounts {
            // Create a variable to keep track of the
            // current customers total amount of money
            let mut total_money = 0;

            // Loop over each bank for our given customer
            for bank in customer {
                // Add the current banks value to the
                // current customers total money
                total_money += bank;
            }

            // Compare the customers total amount of money
            // to the largest amount we've seen so far.
            // If bigger, make it the largest
            if total_money > largest {
                largest = total_money
            }
        }

        // Return the largest (wealthiest) amount of money we've seen
        largest
    }
}

fn main() {
    let accounts0 = vec![vec![1, 2, 3], vec![3, 2, 1]];
    let accounts1 = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    let accounts2 = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];

    println!("accounts0: {:?}", accounts0);
    println!("output: {}", Solution::maximum_wealth(accounts0));
    println!("accounts1: {:?}", accounts1);
    println!("output: {}", Solution::maximum_wealth(accounts1));
    println!("accounts2: {:?}", accounts2);
    println!("output: {}", Solution::maximum_wealth(accounts2));
}
