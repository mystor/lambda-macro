#![feature(phase)]

#[phase(plugin)]
extern crate lambdas;

fn adder(i: int) -> Box<Fn<(int,), int> + 'static> {
    lambda!([i: int] (j: int) -> int {
        i + j
    })
}

fn main() {
    let add1 = adder(1);
    println!("add1(5) = {}", call!(add1, 5));
    println!("add1(10) = {}", call!(add1, 10));

}
