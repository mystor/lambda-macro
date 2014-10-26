#![feature(macro_rules)]

#[macro_export]
macro_rules! lambda {
    (
        [$($capt:ident : $cty:ty),*] ($($arg:ident : $aty:ty),*) -> $ty:ty $body:expr
    ) => (
        {
            struct Closure($($cty),*); // Create the closure
            impl Fn<($($aty ,)*), $ty> for Closure { // Implement the fn type
                extern "rust-call" fn call(&self, args: ($($aty ,)*)) -> $ty {
                    let ($($arg ,)*) = args;
                    let &Closure($($capt),*) = self;
                    $body
                }
            }
            box Closure($($capt),*) // Return a box for it
        }
    )
}

#[macro_export]
macro_rules! call {
    ($lambda:expr $(,$arg:expr)*) => ($lambda.call(($($arg,)*)))
}
