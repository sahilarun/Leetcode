def get_skyline(buildings)
  events = []
  buildings.each do |l, r, h|
    events << [l, -h]
    events << [r, h]
  end
  events.sort!

  result = []
  counts = Hash.new(0)
  counts[0] = 1
  heights = [0]

  i = 0
  while i < events.length
    x = events[i][0]
    while i < events.length && events[i][0] == x
      _, h = events[i]
      if h < 0
        counts[-h] += 1
        heights << -h
      else
        counts[h] -= 1
        if counts[h] == 0
          heights.delete(h)
        end
      end
      i += 1
    end

    curr = heights.max
    result << [x, curr] if result.empty? || result[-1][1] != curr
  end

  result
end