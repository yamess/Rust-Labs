use std::fmt::Debug;

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T>{
    // Sort the left half
    // Sort the right half
    // Merge the two halves together
    let len = v.len();
    if len <= 1 {
        return v;
    }
    let mid = len / 2;
    let mut left = merge_sort(v.drain(..mid).collect());
    let mut right = merge_sort(v.drain(..).collect());
    let mut result = Vec::with_capacity(len);
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }
    result.extend(left);
    result.extend(right);
    return result
}