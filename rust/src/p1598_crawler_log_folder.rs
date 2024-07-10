impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for log in logs {
            match &log[..] {
                "../" => depth = 0.max(depth - 1),
                "./" => {}
                _ => depth += 1,
            }
        }
        depth
    }
}

pub struct Solution {}
