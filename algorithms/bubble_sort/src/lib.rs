use std::fmt::Debug;

// This algorithm complexity is O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for _ in 0..v.len(){
        let mut sorted = true;
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}