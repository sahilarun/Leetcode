class Solution {
  int longestIncreasingPath(List<List<int>> matrix) {
    if (matrix.isEmpty || matrix[0].isEmpty) return 0;
    
    int m = matrix.length, n = matrix[0].length;
    List<List<int>> dp = List.generate(m, (_) => List.filled(n, 0));
    List<List<int>> directions = [
      [0, 1], [1, 0], [0, -1], [-1, 0]
    ];

    int dfs(int x, int y) {
      if (dp[x][y] != 0) return dp[x][y];

      int maxLen = 1;
      for (var dir in directions) {
        int nx = x + dir[0], ny = y + dir[1];
        if (nx >= 0 && ny >= 0 && nx < m && ny < n && matrix[nx][ny] > matrix[x][y]) {
          maxLen = maxLen > 1 + dfs(nx, ny) ? maxLen : 1 + dfs(nx, ny);
        }
      }

      dp[x][y] = maxLen;
      return maxLen;
    }

    int result = 0;
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        result = result > dfs(i, j) ? result : dfs(i, j);
      }
    }
    return result;
  }
}
