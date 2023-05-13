use std::fmt::Debug;

pub fn quick_sort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let p = pivot(arr);
    if arr.len() <= 1 {
        return;
    }
    println!("arr: {:?}", arr);

    let (a, b) = arr.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

pub fn pivot<T: PartialOrd>(arr: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[p] {
            arr.swap(p + 1, i);
            arr.swap(p, p + 1);
            p += 1;
        }
    }
    return p;
}