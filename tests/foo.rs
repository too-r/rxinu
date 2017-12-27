#![no_std]

#[macro_use]
extern crate utest_macros;

extern crate utest_rxinu;

macro_rules! panic {
    ($($tt:tt)*) => {
        upanic!($($tt)*);
    };
}

#[test]
fn foo() {
    assert!(true);
}
