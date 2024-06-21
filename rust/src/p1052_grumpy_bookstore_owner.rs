impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let duration = customers.len().min(grumpy.len());
        let m = minutes.max(0) as usize;
        if customers.len() == 0 || grumpy.len() == 0 {
            return 0;
        }
        let total_customers: i32 = customers.iter().sum();
        let grumpy_customer_prefixes: Vec<i32> = customers
            .into_iter()
            .zip(grumpy.into_iter())
            .map(|(x, y)| x * y)
            .scan(0, |sum, i| {
                *sum += i;
                Some(*sum)
            })
            .collect();
        let mut best = 0;
        for i in 0..duration {
            let current = grumpy_customer_prefixes[i];
            let prev = if i >= m {
                grumpy_customer_prefixes[i-m]
            } else {
                0
            };
            best = best.max(current - prev);
        }
        total_customers - grumpy_customer_prefixes.last().unwrap_or(&0) + best
    }
}

pub struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }
}
