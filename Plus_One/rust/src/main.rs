struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res =  digits;

        for digit in res.iter_mut().rev() {
            match *digit == 9 {
                true => *digit = 0,
                false => {
                    *digit += 1;
                    return res
                }
            }
        }
        res.insert(0, 1);
        res
    }

}

fn main() {
    let digits: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", Solution::plus_one(digits));

}