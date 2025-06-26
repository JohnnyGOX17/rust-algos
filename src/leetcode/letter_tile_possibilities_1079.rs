//! # [1079. Letter Tile Possibilities (Medium)](https://leetcode.com/problems/letter-tile-possibilities/description/)
//!
//! You have n  tiles, where each tile has one letter `tiles[i]` printed on it.
//! Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.

fn find_seq(char_count: &mut [u32]) -> i32 {
    let mut total_count = 0;

    // Try each available char
    for i in 0..26 {
        if char_count[i] == 0 {
            continue;
        }

        // Add current char and recurse
        total_count += 1;
        char_count[i] -= 1;
        total_count += find_seq(char_count);
        char_count[i] += 1;
    }

    total_count
}

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut char_count = [0u32; 26];
    for ch in tiles.chars() {
        // Constraints always gives upper-case letters
        char_count[ch as usize - 65] += 1;
    }

    find_seq(&mut char_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tiles = "AAB".to_string();
        assert_eq!(num_tile_possibilities(tiles), 8);
    }

    #[test]
    fn case_2() {
        let tiles = "AAABBC".to_string();
        assert_eq!(num_tile_possibilities(tiles), 188);
    }

    #[test]
    fn case_3() {
        let tiles = "V".to_string();
        assert_eq!(num_tile_possibilities(tiles), 1);
    }
}
