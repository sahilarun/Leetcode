class Solution {
  bool isRectangleCover(List<List<int>> rectangles) {
    final corners = <String>{};
    int minX = 1 << 30, minY = 1 << 30;
    int maxX = -(1 << 30), maxY = -(1 << 30);
    int totalArea = 0;

    for (var rect in rectangles) {
      int x1 = rect[0], y1 = rect[1], x2 = rect[2], y2 = rect[3];
      minX = x1 < minX ? x1 : minX;
      minY = y1 < minY ? y1 : minY;
      maxX = x2 > maxX ? x2 : maxX;
      maxY = y2 > maxY ? y2 : maxY;

      totalArea += (x2 - x1) * (y2 - y1);

      for (var point in [
        '$x1 $y1',
        '$x1 $y2',
        '$x2 $y1',
        '$x2 $y2',
      ]) {
        if (!corners.add(point)) corners.remove(point);
      }
    }

    final expectedArea = (maxX - minX) * (maxY - minY);
    if (totalArea != expectedArea) return false;

    return corners.length == 4 &&
      corners.contains('$minX $minY') &&
      corners.contains('$minX $maxY') &&
      corners.contains('$maxX $minY') &&
      corners.contains('$maxX $maxY');
  }
}