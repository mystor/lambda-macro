# lambda-macro

Lambdas in rust are confusing. Currently, when you type `|x| { x + i }`, the lambda cannot live past the lifetime of the variable it closes around: `i`. That means that if `i` is a parameter to a function, you can't return a lambda from that function.

To get around this restriction, the values refered to in the lambda need to be "moved" into the closure. `proc`s do this, but can only be run once, as they clean up the moved variables after they are run.

This macro gets around that by instead moving the variables closed over into a heap-allocated struct, and adding a `call` method to that struct.

## Usage
Load the macro into your program:
```rust
#![feature(phase)]

#[phase(plugin)]
extern crate lambdas;

...
```

This imports the `lambda!` macro, which takes the general form of the following:
```rust
lambda!([i: int, j: int]  // closed over variables and types
        (k: int, l: int)  // lambda parameters and types
        -> int            // lambda return types
        { i + j + k + l } // lambda body
): Box<Fn<(int,int), int> + 'static> // Produces a boxed struct implementing the Fn trait
```

A simple example would be a curried adder function
```rust
fn adder(i: int) -> Box<Fn<(int,), int> + 'static> {
    lambda!([i: int] (j: int) -> int {
        i + j
    })
}
```

This could be called as follows:
```rust
fn main() {
    let add1 = adder(1);
    println!("{}", add1.call((5,))); // 6
}
```

This is a bit inconvenient, so there is also a `call!` macro, which allows us to do the following:
```rust
fn main() {
    let add1 = adder(1);
    println!("{}", call!(add1, 5));
}
```

## Implementation
The `lambda!` macro expands into something like the following:
```rust
lambda!([i: int] (j: int) -> int {
    i + j
})
=>
{
    // The struct containing the closed-over variables
    struct Closure(int);
    // The call method for the Closure
    impl Fn<(int,), int> for Closure {
        extern "rust-call" fn call(&self, args: (int,)) -> int {
            let (j,) = args;
            let &Closure(i) = self;
            { i + j } // Your closure body
        }
    }
    // Boxing the value on the heap
    box() Closure(i)
}
```

## Limitations
With the `#![feature(overloaded_calls)]` you should be able to do:
```rust
fn main() {
    let add1 = adder(1);
    println!("{}", add1(5));
}
```

But that is currently not working (see [rust#18349](https://github.com/rust-lang/rust/issues/18349))

In addition, it would be nice if we could infer the types of closed over variables etc., but that is also currently impossible.
