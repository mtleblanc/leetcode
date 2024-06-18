mod p330_patching_array;
mod p633_sum_of_squares;
mod p826_most_profit_assigning_work;

fn main() {
    assert_eq!(p330_patching_array::Solution::min_patches(vec![1, 3], 6), 1);
    assert_eq!(
        p330_patching_array::Solution::min_patches(vec![1, 5, 10], 20),
        2
    );
    assert_eq!(
        p330_patching_array::Solution::min_patches(vec![1, 2, 2], 5),
        0
    );
    assert_eq!(
        p330_patching_array::Solution::min_patches(vec![1, 2, 31, 33], 2147483647),
        28
    );
    assert_eq!(
        p330_patching_array::Solution::min_patches(
            vec![
                1 << 0,
                1 << 1,
                1 << 2,
                1 << 3,
                1 << 4,
                1 << 5,
                1 << 6,
                1 << 7,
                1 << 8,
                1 << 9,
                1 << 10,
                1 << 11,
                1 << 12,
                1 << 13,
                1 << 14,
                1 << 15,
                1 << 16,
                1 << 17,
                1 << 18,
                1 << 19,
                1 << 20,
                1 << 21,
                1 << 22,
                1 << 23,
                1 << 24,
                1 << 25,
                1 << 26,
                1 << 27,
                1 << 28,
                1 << 29,
                (1 << 30) - 1
            ],
            i32::MAX
        ),
        1
    );
}
