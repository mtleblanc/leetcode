impl Solution {
    /*
     * Keep track of when the chef will be free and iterate the array
     */
    pub fn average_waiting_time_for(customers: Vec<Vec<i32>>) -> f64 {
        let mut total_wait_time = 0;
        let mut free_time = 0;
        let count = customers.len() as f64;
        for customer in customers.into_iter() {
            let arrival = customer[0] as i64;
            let time = customer[1] as i64;
            if arrival >= free_time {
                total_wait_time += time;
                free_time = arrival + time;
            } else {
                free_time += time;
                total_wait_time += free_time - arrival;
            }
        }
        (total_wait_time as f64) / count
    }

    pub fn average_waiting_time_scan(customers: Vec<Vec<i32>>) -> f64 {
        let n = customers.len() as f64;
        let wait_times = customers.into_iter().scan(0, |free_time, customer| {
            let arrival = customer[0] as i64;
            let time = customer[1] as i64;
            if arrival >= *free_time {
                *free_time = arrival + time;
                Some(time)
            } else {
                *free_time += time;
                Some(*free_time - arrival)
            }
        });
        wait_times.sum::<i64>() as f64 / n
    }

    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        Self::average_waiting_time_scan(customers)
    }
}

pub struct Solution {}
