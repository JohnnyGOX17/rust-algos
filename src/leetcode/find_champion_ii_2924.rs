//! # 2924. Find Champion II
//!
//! <https://leetcode.com/problems/find-champion-ii/description/>

/// Traverse DAG to find head node
pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    // In the case of the tournament DAG, the winner is essentially the head node (or in the case
    // of no winner, one of the head nodes)- therefore any time we see the end point of an edge, we
    // can increment that node's "indegree" (the number of edges coming into the vertex of a
    // directed graph).
    let mut node_counts = vec![0; n as usize];
    for edge in edges {
        // Only need the end node of the edge to increment indegree
        node_counts[edge[1] as usize] += 1;
    }

    let mut champ = -1;
    for (i, count) in node_counts.iter().enumerate() {
        if *count == 0 {
            if champ != -1 {
                // Already a winner found
                return -1;
            } else {
                champ = i as i32;
            }
        }
    }
    champ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let n = 3;
        let edges: Vec<Vec<i32>> = [[0, 1].to_vec(), [1, 2].to_vec()].to_vec();
        assert_eq!(find_champion(n, edges), 0);
    }

    #[test]
    fn case_2() {
        let n = 4;
        let edges: Vec<Vec<i32>> = [[0, 2].to_vec(), [1, 3].to_vec(), [1, 2].to_vec()].to_vec();
        assert_eq!(find_champion(n, edges), -1);
    }
}
