impl Solution {
    pub fn find_the_winner_brute(n: i32, k: i32) -> i32 {
        let k = k as usize;
        let mut friends: Vec<i32> = (0..n).into_iter().collect();
        let mut index = 0;
        while friends.len() > 1 {
            index += k - 1;
            index %= friends.len();
            let removed = friends.remove(index);
            println!("Player {} removed", removed);
        }
        friends[0] + 1
    }

    /*
     * Inductively, the survivor with n players is k
     * after the survivor with n - 1 players, since
     * in the first round, the kth player is removed
     * and there are n - 1 players remaining, starting
     * with player k + 1.  Thus the survivor is
     * find_the_winner(n - 1, k) + k, modulo n
     *
     * With 1 player, player 1 wins, as the base of the
     * induction.  However since we need to compute
     * mod n, it is easier to number the players 0 to
     * n - 1, and then add 1 back at the end to restore
     * the numbering
     *
     * Since this is weak induction, we don't need a
     * full array for dp, just the most recent value
     */
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut ans = 0;
        for i in 2..=n {
            ans += k;
            ans %= i;
        }
        ans + 1
    }

    pub fn find_the_winner_fold(n: i32, k: i32) -> i32 {
        (2..=n).fold(0, |ans, i| (ans + k) % i) + 1
    }
}

pub struct Solution {}
