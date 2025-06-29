use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        if m <= 2 || n <= 2 {
            return 0;
        }

        let mut visited = vec![vec![false; n]; m];
        let mut heap = BinaryHeap::new();

        for i in 0..m {
            heap.push(Reverse((height_map[i][0], i, 0)));
            heap.push(Reverse((height_map[i][n - 1], i, n - 1)));
            visited[i][0] = true;
            visited[i][n - 1] = true;
        }

        for j in 1..n - 1 {
            heap.push(Reverse((height_map[0][j], 0, j)));
            heap.push(Reverse((height_map[m - 1][j], m - 1, j)));
            visited[0][j] = true;
            visited[m - 1][j] = true;
        }

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut trapped = 0;

        while let Some(Reverse((height, x, y))) = heap.pop() {
            for (dx, dy) in dirs.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] {
                    continue;
                }

                visited[nx][ny] = true;

                let neighbor_height = height_map[nx][ny];
                trapped += (height - neighbor_height).max(0);
                heap.push(Reverse((height.max(neighbor_height), nx, ny)));
            }
        }

        trapped
    }
}
