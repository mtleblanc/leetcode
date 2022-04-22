

func divisorGame(n int) bool {
    memo := make([]bool, n)
    memo[0] = false
    for i := 2; i <= n; i ++ {
        canWin := !memo[i-2]
        div := 2
        for !canWin {
            otherDiv := i / div
            if div > otherDiv {
                break
            }
            if i % div == 0 {
                canWin = canWin || !memo[i - div - 1] || !memo[i - otherDiv - 1]
            }
            div ++
        }
        memo[i-1] = canWin
    }
    fmt.Println(memo)
    return memo[n-1]
}
