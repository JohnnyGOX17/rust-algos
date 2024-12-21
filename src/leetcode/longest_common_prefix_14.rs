//! # [14. Longest Common Prefix (Easy)](https://leetcode.com/problems/longest-common-prefix/description/)
//!
//! Write a function to find the longest common prefix string amongst an array of strings. If there
//! is no common prefix, return an empty string `""`.

pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    strs.sort_by_key(|a| a.len());
    let shortest_str = &strs[0];
    for i in 0..shortest_str.len() {
        if strs
            .iter()
            .any(|s| s.chars().nth(i) != shortest_str.chars().nth(i))
        {
            return shortest_str[0..i].to_string();
        }
    }
    shortest_str.to_string()
}

pub fn longest_common_prefix_direct(strs: Vec<String>) -> String {
    let mut retval: Vec<char> = Vec::new();
    let mut idx = 0;

    loop {
        // ok since strs[i] consists of only lowercase English letters
        let mut comp_char = '~';
        let mut matched_char = false;

        for (i, str_i) in strs.iter().enumerate() {
            let pop_val = str_i.chars().nth(idx);
            if let Some(val) = pop_val {
                if i == 0 {
                    comp_char = val;
                    // useful when str length is 1
                    matched_char = true;
                } else if comp_char == val {
                    matched_char = true;
                } else {
                    matched_char = false;
                    break;
                }
            } else {
                matched_char = false;
                break;
            }
        }

        if matched_char {
            retval.push(comp_char);
        } else {
            break;
        }

        idx += 1;
    }

    retval.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let strs = [
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]
        .to_vec();
        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn case_2() {
        let strs = ["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec();
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}
