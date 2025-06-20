def shortest_palindrome(s)
  return s if s.empty?

  rev = s.reverse
  combined = s + "#" + rev
  lps = Array.new(combined.length, 0)
  j = 0
  
  (1...combined.length).each do |i|
    while j > 0 && combined[i] != combined[j]
      j = lps[j - 1]
    end
    
    if combined[i] == combined[j]
      j += 1
    end
    
    lps[i] = j
  end
  overlap = lps[-1]
  rev[0, s.length - overlap] + s
end