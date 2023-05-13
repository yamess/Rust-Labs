use merge_sort::merge_sort;


#[test]
fn test_merge_sort() {
    let v = vec![4, 3, 2, 5, 1];
    let result = merge_sort(v);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}
