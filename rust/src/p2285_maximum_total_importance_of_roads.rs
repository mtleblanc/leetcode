impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut counts = vec![0; n as usize];
        for road in roads {
            counts[road[0] as usize] += 1;
            counts[road[1] as usize] += 1;
        }
        counts.sort_unstable();
        counts
            .into_iter()
            .enumerate()
            .map(|(i, count)| (i + 1) as i64 * count as i64)
            .sum()
    }
}

pub struct Solution {}
