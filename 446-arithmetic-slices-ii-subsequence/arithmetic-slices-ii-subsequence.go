func numberOfArithmeticSlices(nums []int) int {
	n := len(nums)
	dp := make([]map[int]int, n)
	res := 0

	for i := range dp {
		dp[i] = make(map[int]int)
		for j := 0; j < i; j++ {
			diff := nums[i] - nums[j]
			cnt := dp[j][diff]
			dp[i][diff] += cnt + 1
			res += cnt
		}
	}

	return res
}