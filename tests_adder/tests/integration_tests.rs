use tests_adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let res = tests_adder::add(2, 2);
    assert_eq!(4, res);
}
