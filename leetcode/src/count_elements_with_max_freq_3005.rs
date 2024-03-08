/// 3005. Count Elements With Maximum Frequency (Easy)
/// https://leetcode.com/problems/count-elements-with-maximum-frequency/description/
///
/// You are given an array nums consisting of positive integers. Return the total frequencies of
/// elements in nums such that those elements all have the maximum frequency. The frequency of an
/// element is the number of occurrences of that element in the array.
/// Constraints:
/// * 1 <= nums.length <= 100
/// * 1 <= nums[i] <= 100
use std::collections::HashMap;

/// This method need only pass through the array once (runs in O(N)) and has a small memory
/// footprint as it need only have a basic array to map frequencies per element
#[allow(clippy::comparison_chain)]
pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    // since the constraints are relatively small (length and value never exceeding 100) we can
    // create a frequency map (or really a basic array) with index/key being the element value and
    // the value being the frequency of that element's occurence
    assert!((1..100).contains(&nums.len()));
    // make the freq map size 101 so we can directly use the element as the index/key
    // (e.g. don't have to subtract 1 to make 0-based index)
    let mut freq_map = vec![0i32; 101];

    // keep track of maximum frequency of a given element
    let mut max_freq = 0;
    // since one or more elements/values may have the same frequency of occurrence, we also keep
    // track of this as an additive counter each time we hit an element with the same frequency
    let mut max_freq_occurrence = 0;

    for &num in nums.iter() {
        // could also add assert here to enforce element in range (1..100) to not exceed index bounds
        let idx = num as usize;
        // increment frequency of given element
        freq_map[idx] += 1;

        if freq_map[idx] > max_freq {
            // new highest frequency found
            max_freq = freq_map[idx];
            // also reset freq of occurence
            max_freq_occurrence = max_freq;
        } else if freq_map[idx] == max_freq {
            // same frequency hit, increment by frequency amount (same as adding value pairs
            // together or keeping track of occurrence counter and multiplying at the end)
            max_freq_occurrence += max_freq;
        }
    }

    max_freq_occurrence
}

/// This method uses a hash map to keep track of frequencies, then iterates through the hash map to
/// find the maximum frequency and number of occurences. Curiously, this has a faster runtime than
/// the above in LC testing (though it has to iterate twice)...
#[allow(clippy::comparison_chain)]
pub fn max_freq_elements_map(nums: Vec<i32>) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    let mut max_freq = 0;
    let mut max_occurrence = 0;

    for n in nums.iter() {
        freq_map
            .entry(*n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for (_, &v) in freq_map.iter() {
        if v > max_freq {
            max_freq = v;
            max_occurrence = 1;
        } else if v == max_freq {
            max_occurrence += 1;
        }
    }

    max_occurrence * max_freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let in_nums = [1, 2, 2, 3, 1, 4].to_vec();
        assert_eq!(max_frequency_elements(in_nums.clone()), 4);
        assert_eq!(max_freq_elements_map(in_nums), 4);
    }

    #[test]
    fn case_2() {
        let in_nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(max_frequency_elements(in_nums.clone()), 5);
        assert_eq!(max_freq_elements_map(in_nums), 5);
    }

    #[test]
    fn case_3() {
        let in_nums = [15].to_vec();
        assert_eq!(max_frequency_elements(in_nums.clone()), 1);
        assert_eq!(max_freq_elements_map(in_nums), 1);
    }
}
