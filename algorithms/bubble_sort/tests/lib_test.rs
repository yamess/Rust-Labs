use::bubble_sort::bubble_sort;


#[test]
fn test_bubble_sort() {
    let mut v = vec![4, 3, 2, 5, 1];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}