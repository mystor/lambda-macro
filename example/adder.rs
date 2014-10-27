#![feature(phase)]

#[phase(plugin)]
extern crate lambdas;

fn adder(i: int) -> Box<Fn<(int,), int> + 'static> {
    lambda!([i: int] (j: int) -> int {
        i + j
    })
}

fn no_capture() -> Box<Fn<(int, int), int> + 'static> {
    lambda!([] (i: int, j: int) -> int {
        i + j
    })
}

fn no_args(i: int, j: int) -> Box<Fn<(), int> + 'static> {
    lambda!([i: int, j: int] () -> int {
        i + j
    })
}

fn main() {
    let add1 = adder(1);
    println!("add1(5) = {}", call!(add1, 5));
    println!("add1(10) = {}", call!(add1, 10));
    println!("no_capture()(5, 10) = {}", call!(no_capture(), 5, 10));
    println!("no_args()(1, 2) = {}", call!(no_args(1, 2)));
}
