struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        // Create a mutable copy of our list of digits
        let mut res = digits;

        // Loop over our list of digits in reverse order.
        // `iter_mut` will give us a mutable reference to our "digit"
        // and `rev` will reverse the iterators direction, so we're starting
        // at the end of the list.
        for digit in res.iter_mut().rev() {
            // `match` on "is the current digit 9?"
            match *digit == 9 {
                // Since we're adding 1 to the digit, if our current digit is
                // 9, we need to set it to 0 and "carry" the 1 to the next item
                // in the iterator. This will happen in the next cycle of the for loop,
                // since we're going in reverse order, the next digit is either another
                // 9 where we repeat the process and keep carrying the 1, or we add
                // 1 to the next digit
                true => *digit = 0,
                // It's not a 9, so we can add 1 to our current digit and then return the vector
                // we are looping over. Our iterator is in reverse order, but that
                // does not reverse the order of the resulting vector.
                false => {
                    *digit += 1;
                    return res;
                }
            }
        }
        // If we made it to the end of our vector of digits, and the first digit was a 9,
        // to add one to it we need to insert a new element at the beginning of the vector.
        // `insert` will add "1" at position "0" (the beginning).
        res.insert(0, 1);
        // Return the result
        res
    }
}

fn main() {
    let digits: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", Solution::plus_one(digits));
}
