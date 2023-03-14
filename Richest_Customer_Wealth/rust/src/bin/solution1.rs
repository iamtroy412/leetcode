struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            // Create an Iterator out of the given list of accounts.
            .iter()
            // The elements of our Iterator, are other Vectors which themselves
            // can be iterated again. The elements represent each customers
            // "banks", and we sum them to get each customers total wealth.
            // This total wealth is the result of the closure and returned back
            // to the map.
            .map(|bank| bank.iter().sum())
            // At this point, we have an Iterator of each customers total wealth
            // wealth_total_of[customer0, customer1, ...]. The answer to this problem
            // is to return the customers wealth that is the greatest, so use
            // `max` on the Iterator to return the largest element value.
            .max()
            // `max` returns an `Option`, so if the iterator is empty, it will
            // return `None`. We're not expecting any empty iterators so just unwrap
            .unwrap()
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
