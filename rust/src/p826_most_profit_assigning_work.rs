impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
        let mut jobs = difficulty.into_iter().zip(profit.into_iter()).collect::<Vec<_>>();
        jobs.sort();
        worker.sort();
        let mut profit = 0;
        let mut best = 0;
        let mut jobs_iter = jobs.into_iter();
        let mut workers_iter = worker.into_iter();
        let mut current_job = jobs_iter.next();
        let mut current_worker = workers_iter.next();
        while current_worker.is_some() {
            if current_job.is_none() || current_worker.unwrap() < current_job.unwrap().0 {
                profit += best;
                current_worker = workers_iter.next();
            } else {
                best = best.max(current_job.unwrap().1);
                current_job = jobs_iter.next();
            }
        }
        profit
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
        assert_eq!(
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]),
            0
        );
    }
}
