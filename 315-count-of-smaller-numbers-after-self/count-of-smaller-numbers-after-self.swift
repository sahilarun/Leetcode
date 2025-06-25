class FenwickTree {
    private var tree: [Int]
    
    init(_ size: Int) {
        tree = [Int](repeating: 0, count: size + 1)
    }
    
    func update(_ i: Int, _ delta: Int) {
        var i = i + 1
        while i < tree.count {
            tree[i] += delta
            i += i & -i
        }
    }
    
    func query(_ i: Int) -> Int {
        var i = i
        var sum = 0
        while i > 0 {
            sum += tree[i]
            i -= i & -i
        }
        return sum
    }
}

class Solution {
    func countSmaller(_ nums: [Int]) -> [Int] {
        let sorted = Array(Set(nums)).sorted()
        let rank = Dictionary(uniqueKeysWithValues: sorted.enumerated().map { ($1, $0) })
        let tree = FenwickTree(sorted.count)
        
        var result = [Int]()
        for num in nums.reversed() {
            let i = rank[num]!
            result.append(tree.query(i))
            tree.update(i, 1)
        }
        return result.reversed()
    }
}
