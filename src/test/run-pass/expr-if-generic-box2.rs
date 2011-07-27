


// -*- rust -*-
type compare[T] = fn(&T, &T) -> bool ;

fn test_generic[T](expected: &T, not_expected: &T, eq: &compare[T]) {
    let actual: T = if true { expected } else { not_expected };
    assert (eq(expected, actual));
}

fn test_vec() {
    fn compare_vec(v1: &vec[int], v2: &vec[int]) -> bool { ret v1 == v2; }
    let eq = bind compare_vec(_, _);
    test_generic[vec[int]]([1, 2], [2, 3], eq);
}

fn main() { test_vec(); }