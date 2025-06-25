object Solution {
  def countRangeSum(nums: Array[Int], lower: Int, upper: Int): Int = {
    val prefix = new Array[Long](nums.length + 1)
    for (i <- nums.indices) {
      prefix(i + 1) = prefix(i) + nums(i)
    }

    def mergeSort(start: Int, end: Int): Int = {
      if (end - start <= 1) return 0
      val mid = (start + end) / 2
      var count = mergeSort(start, mid) + mergeSort(mid, end)

      var j = mid
      var k = mid
      var t = mid
      val cache = new Array[Long](end - start)
      var r = 0
      var i = start
      var c = 0

      while (i < mid) {
        while (k < end && prefix(k) - prefix(i) < lower) k += 1
        while (j < end && prefix(j) - prefix(i) <= upper) j += 1
        while (t < end && prefix(t) < prefix(i)) {
          cache(c) = prefix(t)
          c += 1
          t += 1
        }
        cache(c) = prefix(i)
        c += 1
        count += j - k
        i += 1
      }

      System.arraycopy(cache, 0, prefix, start, c)
      count
    }

    mergeSort(0, prefix.length)
  }
}
