/// 1629. Slowest Key (Easy)
/// <https://leetcode.com/problems/slowest-key/description/>
///
/// A newly designed keypad was tested, where a tester pressed a sequence of n keys, one at a time.
/// You are given a string keysPressed of length n, where keysPressed\[i\] was the ith key pressed in
/// the testing sequence, and a sorted list releaseTimes, where releaseTimes\[i\] was the time the ith
/// key was released. Both arrays are 0-indexed. The 0th key was pressed at the time 0, and every
/// subsequent key was pressed at the exact time the previous key was released. The tester wants to
/// know the key of the keypress that had the longest duration. The ith keypress had a duration of
/// releaseTimes\[i\] - releaseTimes\[i - 1\], and the 0th keypress had a duration of releaseTimes\[0\].
/// Note that the same key could have been pressed multiple times during the test, and these
/// multiple presses of the same key may not have had the same duration. Return the key of the
/// keypress that had the longest duration. If there are multiple such keypresses, return the
/// lexicographically largest key of the keypresses.
pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    // check for constraints
    let n = release_times.len();
    assert_eq!(n, keys_pressed.len());
    assert!((2..1000).contains(&n));
    let in_bytes = keys_pressed.as_bytes();

    // safe to index first element since length constraint above
    let mut longest_duration = release_times[0];
    let mut prev_duration = release_times[0];
    let mut longest_key = in_bytes[0];

    // Basically, instead of building some complex map of key press differentials, just
    // keep track of the longest key press as we iterate through the input arrays. This is a simple
    // subtraction operation, and has a runtime complexity of O(N) since we iterate once through
    // both arrays, and a space complexity of O(1) since we need only constant extra space.
    //
    // NOTE1: can skip first index since longest duration initialized with first index
    // NOTE2: iter() is ok vs into_iter() when you need only borrow not move the values:
    //   https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html/
    for (key_time, &key) in release_times
        .into_iter()
        .skip(1)
        .zip(in_bytes.iter().skip(1))
    {
        let duration = key_time - prev_duration;
        if duration >= longest_duration {
            if duration == longest_duration {
                if key > longest_key {
                    longest_key = key;
                }
            } else {
                longest_duration = duration;
                longest_key = key;
            }
        }

        prev_duration = key_time;
    }

    longest_key as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let in_times = [9, 29, 49, 50].to_vec();
        assert_eq!(slowest_key(in_times, "cbcd".to_string()), 'c');
    }

    #[test]
    fn case_2() {
        let in_times = [12, 23, 36, 46, 62].to_vec();
        assert_eq!(slowest_key(in_times, "spuda".to_string()), 'a');
    }
}
