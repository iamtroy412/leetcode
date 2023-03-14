use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Create a mutable `HashMap` that will store the characters from the
        // `magazine` and the number of times the char was seen (count).
        // k: 'a', v: 3
        let mut dict = HashMap::new();

        // Populate our HashMap by looping over the characters
        // in the `magazine`
        for c in magazine.chars() {
            // Get an `Entry` enum from the current magazine char
            // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
            dict.entry(c)
                // If the `Entry` is `Occupied`, the `and_modify` method
                // provides in-place mutable access to it
                .and_modify(|count| *count += 1)
                // If the `Entry` is `Vacant`, the `or_insert` method
                // will add the value and return a mutable reference
                // to the value in the entry
                .or_insert(1);
        }

        // Loop over the chars in our `ransom_note`
        for c in ransom_note.chars() {
            // Match our current char to a key in the `magazine` HashMap we
            // created earlier. `get_mut` returns `Some` mutable reference to the
            // HashMaps value, or `None` if they key lookup fails
            match dict.get_mut(&c) {
                // Use a match guard to check if the chars used count
                // is greater than 0. If so, that means we still have this char
                // available from our magazine to use in our ransom note.
                // So subtract one from the number of these chars left.
                Some(count) if *count > 0 => {
                    *count -= 1;
                }
                // This could be two conditions, if the count == 0, meaning we
                // had some of the chars but none are left to use for the ransom note
                // so return false. The other is the Option returns `None` meaning
                // we never had any of the letters to use, so we can't make the
                // ransom note anway, return false.
                _ => return false,
            }
        }

        // At this point we've gone through all of the chars
        // in our `ransom_note` and haven't run out of chars
        // from our `magazine`, so yes we can construct this note.
        // return true
        true
    }
}

fn main() {
    println!("ransomNote = 'a', magazine = 'b'");
    println!(
        "Output: {}",
        Solution::can_construct("a".to_string(), "b".to_string())
    );

    println!("ransomNote = 'aa', magazine = 'ab'");
    println!(
        "Output: {}",
        Solution::can_construct("aa".to_string(), "ab".to_string())
    );

    println!("ransomNote = 'aa', magazine = 'aab'");
    println!(
        "Output: {}",
        Solution::can_construct("aa".to_string(), "aab".to_string())
    );
}
