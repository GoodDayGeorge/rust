


// -*- rust -*-
fn main() {
    let a: vec[int] = [1, 2, 3, 4, 5];
    let b: vec[int] = [6, 7, 8, 9, 0];
    let v: vec[int] = a + b;
    log v.(9);
    assert (v.(0) == 1);
    assert (v.(7) == 8);
    assert (v.(9) == 0);
}