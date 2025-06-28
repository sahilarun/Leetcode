import 'dart:math';

class RandomizedCollection {
  final _vals = <int>[];
  final _idx = <int, Set<int>>{};
  final _rand = Random();

  RandomizedCollection();

  bool insert(int val) {
    _idx.putIfAbsent(val, () => <int>{});
    _idx[val]!.add(_vals.length);
    _vals.add(val);
    return _idx[val]!.length == 1;
  }

  bool remove(int val) {
    if (!_idx.containsKey(val) || _idx[val]!.isEmpty) return false;
    int i = _idx[val]!.first;
    int last = _vals.last;

    _vals[i] = last;
    _idx[val]!.remove(i);
    _idx[last]!.remove(_vals.length - 1);
    if (i < _vals.length - 1) _idx[last]!.add(i);

    _vals.removeLast();
    if (_idx[val]!.isEmpty) _idx.remove(val);
    return true;
  }

  int getRandom() => _vals[_rand.nextInt(_vals.length)];
}