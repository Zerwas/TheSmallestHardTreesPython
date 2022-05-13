// f(x,x,y) = f(x,y,x) = f(y,x,x) = f(x,x,x)
pub fn quasi_majority_p<T: Eq + Clone + Hash>(v: &[T], w: &[T]) -> bool {
    assert!(v.len() == 3 && w.len() == 3, "length must be equal to 3!");
    match (elem_count(v), elem_count(w)) {
        (Once(x0, _), Once(x1, _)) => x0 == x1,
        (AllEqual(a), Once(x, _)) | (Once(x, _), AllEqual(a)) => a == x,
        _ => false,
    }
}
