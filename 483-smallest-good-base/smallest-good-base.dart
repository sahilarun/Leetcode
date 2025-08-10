class Solution {
  String smallestGoodBase(String n) {
    final numN = BigInt.parse(n);
    final maxM = numN.bitLength - 1;

    for (int m = maxM; m >= 2; m--) {
      final k = _findBase(numN, m);
      if (k != null) return k.toString();
    }
    return (numN - BigInt.one).toString();
  }

  BigInt? _findBase(BigInt n, int m) {
    BigInt left = BigInt.two;
    BigInt right = n - BigInt.one;

    while (left <= right) {
      final mid = (left + right) >> 1;
      final sum = _calc(mid, m);
      if (sum == n) return mid;
      if (sum < n) {
        left = mid + BigInt.one;
      } else {
        right = mid - BigInt.one;
      }
    }
    return null;
  }

  BigInt _calc(BigInt base, int m) {
    BigInt sum = BigInt.one;
    BigInt cur = BigInt.one;
    for (int i = 0; i < m; i++) {
      cur *= base;
      sum += cur;
      if (sum < BigInt.zero) break;
    }
    return sum;
  }
}