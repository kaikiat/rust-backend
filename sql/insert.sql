INSERT INTO solutions (title, description, created_on, modified_on, code)
VALUES ('2090. K Radius Subarray Averages', 'Rust: Sliding Window', CURRENT_TIMESTAMP, NULL,
'
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1;n];        
        let length = 2 * k + 1;
        if length > n as i32 {
            return res
        }
        let mut total: i64 = 0;
        for i in 0..length {
            total += nums[i as usize] as i64;
        }
        let mut left = 0;
        for i in k..(n as i32 -k) {
            res[i as usize] = (total / length as i64) as i32;
            if i + k + 1 >= n as i32 {
                continue;
            }
            total += nums[(i+k) as usize +1] as i64;
            total -= nums[left as usize] as i64;
            left += 1;
        }
        
        res
    }
}
'
);


INSERT INTO solutions (title, description, created_on, modified_on, code)
VALUES ('2328. Number of Increasing Paths in a Grid', 'Rust: Dynamic Programming', CURRENT_TIMESTAMP, NULL,
'
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0;n];m];
        let mod_val: i32 = 1_000_000_007;
        
        fn dfs(i: usize,j: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, m :usize, n :usize, mod_val:i32) -> i32 {
            if dp[i][j] != 0 {
                return dp[i][j];
            }
            let mut answer = 1; 
            let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for dir in directions.iter() {
                let x = dir.0 as usize + i;
                let y = dir.1 as usize + j;
                if x >= 0 && x < m && y >= 0 && y < n && grid[x][y] > grid[i][j] {
                    answer += dfs(x as usize,y as usize,grid,dp,m,n,mod_val) % mod_val;
                }
            }
            dp[i][j] = answer % mod_val;
            return dp[i][j];
        }
        
        let mut result = 0;
        for i in 0..m{
            for j in 0..n{
                result  = (result + dfs(i,j, &grid, &mut dp, m, n, mod_val)) % mod_val;
            }
        }
        result
    }
}'
);



