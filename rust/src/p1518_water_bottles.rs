impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = 0;
        let mut remaining = num_bottles;
        while remaining >= num_exchange {
            let exchanged = remaining / num_exchange;
            remaining %= num_exchange;
            remaining += exchanged;
            total += exchanged * num_exchange;
        }
        total + remaining
    }
}

pub struct Solution {}
