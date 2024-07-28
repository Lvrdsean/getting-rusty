use lesson14;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, lesson14::add_two(2));
}
