// ! Submodules in Integration Tests
// Files in subdirectories of the tests directory don’t get compiled
// as separate crates or have sections in the test output.
// Shared code for integration test files
// Does not generate it's own create
pub fn setup() {
    // setup code specific to library tests would go here
}

// How to use:
// use adder;

// mod common;

// #[test]
// fn it_adds_two() {
//     common::setup();
//     assert_eq!(4, adder::add_two(2));
// }
