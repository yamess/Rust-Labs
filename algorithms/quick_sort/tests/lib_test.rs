use quick_sort::{pivot, quick_sort};

#[test]
fn test_pivot() {
    let mut v = vec![4,6,1,19,8,11,13,3];
    let _p = pivot(&mut v);
    for x in 0..v.len() {
        assert_eq!((v[x] < v[_p]), (x < _p));
    }

    quick_sort(&mut v);
    assert_eq!(v, vec![1,3,4,6,8,11,13,19])
}