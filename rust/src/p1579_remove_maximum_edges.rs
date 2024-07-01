struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    pub fn find(&mut self, mut a: usize) -> usize {
        loop {
            let p = self.parent[a];
            let gp = self.parent[p];
            if gp == p {
                return p;
            }
            self.parent[a] = gp;
            a = gp;
        }
    }

    pub fn union(&mut self, a: usize, b: usize) -> &mut Self {
        let mut a_name = self.find(a);
        let mut b_name = self.find(b);
        if a_name != b_name {
            self.components -= 1;
            if self.size[a_name] < self.size[b_name] {
                std::mem::swap(&mut a_name, &mut b_name);
            }
            self.parent[b_name] = a_name;
            self.size[a_name] += self.size[b_name];
        }
        self
    }
}

impl Solution {
    /**
     * Build connected components using a UnionFind structure, which keeps track of which disjoint
     * set each element is a part of by storing a pointer to a parent in the same set, forming a
     * directed graph whose only cycles are 1 cycles on a representative of each disjoint set
     *
     * A traversable graph with cycles can remove an edge from a cycle and still be traversable,
     * so we can remove edges until both the Type1+Type3 and Type2+Type3 graphs are trees.  A
     * tree on n vertices has n-1 edges.  Similarly a forest with n vertices and m components has
     * n-m edges.  To remove the maximum number of edges, we should remove as few Type3 edges as possible.
     *
     * So the overall logic is
     *   1) Determine the number of Type3 edges we must remove to prevent cycles by finding the number of
     *      components in the Type3 graph
     *   2) Verify that the Type1+Type3 and Type2+Type3 graphs are connected, else return -1
     *
     * There's a few approaches for handling the 3 graphs we must consider, such as making the Type3 UnionFind,
     * cloning it, and then extending the 2 copies into the other 2 graphs, or making a UnionFind for each type
     * then iterating through the Type3 UnionFind and unioning each vertex in the Type1/2 UnionFinds with its
     * Type3 representative.  It seemed simplest to just build all 3 directly.  There are cases where each
     * approach is fastest, but it's a linear cost no matter what.
     */
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut both = UnionFind::new(n);
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let count = edges.len();
        for edge in edges {
            let a = edge[1] as usize - 1;
            let b = edge[2] as usize - 1;
            match edge[0] {
                1 => {
                    alice.union(a, b);
                }
                2 => {
                    bob.union(a, b);
                }
                _ => {
                    both.union(a, b);
                    alice.union(a, b);
                    bob.union(a, b);
                }
            }
        }
        if bob.components > 1 || alice.components > 1 {
            return -1;
        }
        // Term by term calculation.  Here counts[i] was the number of edges of type i+1
        // let needed_both = n - both.components;
        // let needed_alice_or_bob = n - 1 - needed_both;
        // let excess_both = counts[2] - needed_both;
        // let excess_alice = counts[0] - needed_alice_or_bob;
        // let excess_bob = counts[1] - needed_alice_or_bob;
        // (excess_alice + excess_bob + excess_both) as i32
        //
        // But this simplifies to
        // counts[0] + counts[1] + counts[2] + 2 - n - both.components
        // so no need for separate counts

        (count + 2 - n - both.components) as i32
    }

    pub fn max_num_edges_to_remove_bfs(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
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
        (both_count - required_edges)
            + (alice_count + required_edges - n + 1)
            + (bob_count + required_edges - n + 1)
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
