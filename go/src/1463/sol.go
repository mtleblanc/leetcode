func cherryPickup(grid [][]int) int {
    var curDP [][]int
    var nextDP [][]int
    rows := len(grid)
    cols := len(grid[0])
    curDP = make([][]int, cols)
    nextDP = make([][]int, cols)
    for i := range curDP {
        curDP[i] = make([]int, cols)
        nextDP[i] = make([]int, cols)
    }
    
    for j, v := range curDP {
        for k := range v {
            if j == k {
                v[k] = grid[rows - 1][k]
            } else {
                v[k] = grid[rows - 1][k] + grid[rows - 1][j]
            }
        }
    }
    for curRow := rows - 2; curRow >= 0; curRow -- {
        for j, v := range nextDP {
            for k := range v {
                v[k] = 0
                testVal := 0
                for _, l := range []int{-1, 0, 1} {
                    for _, r := range []int{-1, 0, 1} {
                        if j + l < 0 || j + l >= cols || k + r < 0 || k + r >= cols {
                            continue
                        }
                        testVal = curDP[j+l][k+r]
                        if testVal > v[k] {
                            v[k] = testVal
                        }
                    }
                }
                if j == k {
                    v[k] += grid[curRow][j]
                } else {
                    v[k] += grid[curRow][j] + grid[curRow][k]
                }
            }
        }
        curDP, nextDP = nextDP, curDP
    }
    
    return curDP[0][cols - 1]
}
