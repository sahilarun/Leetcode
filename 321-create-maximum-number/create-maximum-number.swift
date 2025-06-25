class Solution {
    func maxNumber(_ nums1: [Int], _ nums2: [Int], _ k: Int) -> [Int] {
        func maxSubsequence(_ nums: [Int], _ t: Int) -> [Int] {
            var stack = [Int]()
            var drop = nums.count - t
            for num in nums {
                while drop > 0 && !stack.isEmpty && stack.last! < num {
                    stack.removeLast()
                    drop -= 1
                }
                stack.append(num)
            }
            return Array(stack.prefix(t))
        }
        
        func greater(_ a: [Int], _ i: Int, _ b: [Int], _ j: Int) -> Bool {
            var i = i, j = j
            while i < a.count && j < b.count {
                if a[i] != b[j] {
                    return a[i] > b[j]
                }
                i += 1
                j += 1
            }
            return (a.count - i) > (b.count - j)
        }

        func merge(_ a: [Int], _ b: [Int]) -> [Int] {
            var res = [Int]()
            var i = 0, j = 0
            while i < a.count || j < b.count {
                if greater(a, i, b, j) {
                    res.append(a[i])
                    i += 1
                } else {
                    res.append(b[j])
                    j += 1
                }
            }
            return res
        }

        var best = [Int]()
        let m = nums1.count, n = nums2.count
        for i in max(0, k - n)...min(k, m) {
            let part1 = maxSubsequence(nums1, i)
            let part2 = maxSubsequence(nums2, k - i)
            let candidate = merge(part1, part2)
            if greater(candidate, 0, best, 0) {
                best = candidate
            }
        }
        return best
    }
}
