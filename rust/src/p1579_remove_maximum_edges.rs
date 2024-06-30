use std::collections::HashSet;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut both = vec![Vec::new(); n];
        let mut alice = vec![Vec::new(); n];
        let mut bob = vec![Vec::new(); n];
        let mut alice_count: i32 = 0;
        let mut bob_count: i32 = 0;
        let mut both_count: i32 = 0;

        for edge in edges {
            let a = edge[1] as usize - 1;
            let b = edge[2] as usize - 1;
            match edge[0] {
                1 => {
                    alice[a].push(b);
                    alice[b].push(a);
                    alice_count += 1;
                }
                2 => {
                    bob[a].push(b);
                    bob[b].push(a);
                    bob_count += 1;
                }
                _ => {
                    both[a].push(b);
                    both[b].push(a);
                    both_count += 1;
                }
            }
        }
        /*
         * First figure out how many type 3 edges can be removed without
         * affecting connectivity.  This amounts to just picking a tree
         * for each component, which is n-1 edges for a connected component
         * of n vertices
         */
        let mut unseen: HashSet<usize> = HashSet::from_iter(0..n);
        let mut frontier = Vec::new();
        let mut required_edges = 0;
        while let Some(&seed) = unseen.iter().next() {
            unseen.remove(&seed);
            frontier.push(seed);
            let mut size = 1;
            while let Some(next) = frontier.pop() {
                for edge in both[next].iter() {
                    if unseen.contains(edge) {
                        frontier.push(*edge);
                        size += 1;
                        unseen.remove(edge);
                    }
                }
            }
            required_edges += size - 1;
        }

        /*
         * Now both type-1 and type-3 edges must combine for n-1 edges, likewise type-2 and type-3.
         * The only concern is if either is not traversable to start, so we just need to check that
         */
        frontier.push(0);
        let mut seen = vec![false; n];
        let mut seen_count = 1;
        seen[0] = true;
        while let Some(next) = frontier.pop() {
            for &edge in both[next].iter() {
                if !seen[edge] {
                    frontier.push(edge);
                    seen_count += 1;
                    seen[edge] = true;
                }
            }
            for &edge in alice[next].iter() {
                if !seen[edge] {
                    frontier.push(edge);
                    seen_count += 1;
                    seen[edge] = true;
                }
            }
        }
        if seen_count < n {
            return -1;
        }

        frontier.push(0);
        let mut seen = vec![false; n];
        let mut seen_count = 1;
        seen[0] = true;
        while let Some(next) = frontier.pop() {
            for &edge in both[next].iter() {
                if !seen[edge] {
                    frontier.push(edge);
                    seen_count += 1;
                    seen[edge] = true;
                }
            }
            for &edge in bob[next].iter() {
                if !seen[edge] {
                    frontier.push(edge);
                    seen_count += 1;
                    seen[edge] = true;
                }
            }
        }
        if seen_count < n {
            return -1;
        }

        let n = n as i32;
        ((both_count - required_edges)
            + (alice_count + required_edges - n + 1)
            + (bob_count + required_edges - n + 1))
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::max_num_edges_to_remove(
                4,
                vec![
                    vec![3, 1, 2],
                    vec![3, 2, 3],
                    vec![1, 1, 3],
                    vec![1, 2, 4],
                    vec![1, 1, 2],
                    vec![2, 3, 4]
                ]
            ),
            2
        );
    }
}
