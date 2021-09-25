// tests direcotry is a seperate create
// cargo test runs the unit, integration and doc tests
use adder;

// From submodule in tests directory
mod common;

// No need for #[cfg(test)], because cargo treats tests directory specially
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// ! Run one test in test directory
// cargo test --test name_of_file_without_.rs
