use std::collections::VecDeque;

struct SummaryRanges {
    intervals: VecDeque<[i32; 2]>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            intervals: VecDeque::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let mut i = 0;
        while i < self.intervals.len() {
            let [start, end] = self.intervals[i];

            if value >= start && value <= end {
                return; 
            } else if value == end + 1 {
                self.intervals[i][1] = value;
                if i + 1 < self.intervals.len() && self.intervals[i + 1][0] == value + 1 {
                    self.intervals[i][1] = self.intervals[i + 1][1];
                    self.intervals.remove(i + 1);
                }
                return;
            } else if value + 1 == start {
                self.intervals[i][0] = value;
                return;
            } else if value < start {
                self.intervals.insert(i, [value, value]);
                return;
            }
            i += 1;
        }

        self.intervals.push_back([value, value]);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|x| vec![x[0], x[1]]).collect()
    }
}
