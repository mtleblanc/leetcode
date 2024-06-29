use std::mem;

impl Solution {
    /**
     * Using topological sort.  Interestingly just using a vector and sorting & dedupping
     * at the point where no more can be added is a bit faster on the submission test cases
     * than using a HashSet.  I didn't try using a BTreeSet.
     */
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        let mut indegree = vec![0; n];
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push(b);
            indegree[b] += 1;
        }
        let mut frontier = indegree
            .iter()
            .enumerate()
            .filter(|(_, &degree)| degree == 0)
            .map(|(i, &_)| i)
            .collect::<Vec<_>>();
        let mut ancestors = vec![Vec::new(); n];
        let mut temp;
        while let Some(considering) = frontier.pop() {
            ancestors[considering].sort_unstable();
            ancestors[considering].dedup();
            // This gets around the borrow checker allowing mutation of other elements
            // it doesn't seem to be any faster than just cloning
            temp = mem::take(&mut ancestors[considering]);
            for &outgoing in graph[considering].iter() {
                ancestors[outgoing].extend(&temp);
                ancestors[outgoing].push(considering as i32);
                indegree[outgoing] -= 1;
                if indegree[outgoing] == 0 {
                    frontier.push(outgoing);
                }
            }
            let _ = mem::replace(&mut ancestors[considering], temp);
        }
        ancestors
    }

    /**
     * Using set unions, adding one edge at a time.  Lots of annoying extra clones
     */
    pub fn get_ancestors_unions(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut ancestors = vec![HashSet::new(); n as usize];
        let mut descendents = vec![HashSet::new(); n as usize];
        for e in edges.into_iter() {
            match e[..] {
                [a, b] => {
                    let a = a as usize;
                    let b = b as usize;
                    let mut to_add = ancestors[a].clone();
                    to_add.insert(a);
                    for &descendent in descendents[b].iter() {
                        ancestors[descendent as usize].extend(&to_add);
                    }
                    ancestors[b].extend(&to_add);
                    to_add = descendents[b].clone();
                    to_add.insert(b);
                    for &ancestor in ancestors[a].iter() {
                        descendents[ancestor].extend(&to_add);
                    }
                    descendents[a].extend(to_add);
                    ancestors[b].insert(a);
                }
                _ => {}
            }
        }
        ancestors
            .into_iter()
            .map(|h| {
                let mut v = h.into_iter().collect::<Vec<_>>();
                v.sort_unstable();
                let mut ret = v.into_iter().map(|x| x as i32).collect::<Vec<_>>();
                ret.dedup();
                ret
            })
            .collect()
    }
}

pub struct Solution {}
