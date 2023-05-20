use std::cmp::{max, min};
use std::fmt::Debug;


#[derive(Debug)]
pub struct Meeting {
    start: i32,
    end: i32,
}

impl Meeting {
    pub fn new(start: i32, end: i32) -> Meeting {
        Meeting { start, end }
    }

    fn overlap(&self, other: &Meeting) -> Option<Vec<i32>> {
        let mut intersection = Vec::with_capacity(2);
        if max(self.start, other.start) < min(self.end, other.end) {
            intersection.push(max(self.start, other.start));
            intersection.push(min(self.end, other.end));
            Some(intersection)
        } else {
            None
        }
    }
}

pub fn get_overlaps(
    schedules1: &Vec<Meeting>,
    schedules2: &Vec<Meeting>) -> Vec<Vec<i32>> {

    let mut overlaps = Vec::new();

    for meeting1 in schedules1.iter() {
        for meeting2 in schedules2.iter() {
            match meeting1.overlap(meeting2) {
                Some(overlap) => overlaps.push(overlap),
                None => continue,
            }
        }

    }
    return overlaps
}