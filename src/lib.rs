#![feature(macro_rules)]

#[macro_export]
macro_rules! lambda {
    (
        [$($capt:ident : $cty:ty),+] ($($arg:ident : $aty:ty),*) -> $ty:ty $body:expr
    ) => (
        {
            #[allow(dead_code)]
            struct Closure($($cty),*);
            impl Fn<($($aty ,)*), $ty> for Closure {
                extern "rust-call" fn call(&self, args: ($($aty ,)*)) -> $ty {
                    let ($($arg ,)*) = args; // Expand lambda args
                    let &Closure($($capt),*) = self; // Expand lambda captures
                    $body
                }
            }
            box Closure($($capt),*)
        }
    );
    (
        [] ($($arg:ident : $aty:ty),*) -> $ty:ty $body:expr
    ) => (
        {
            #[allow(dead_code)]
            struct Closure;
            impl Fn<($($aty ,)*), $ty> for Closure {
                extern "rust-call" fn call(&self, args: ($($aty ,)*)) -> $ty {
                    let ($($arg ,)*) = args; // Expand lambda args
                    $body
                }
            }
            box Closure
        }
    )
}

#[macro_export]
macro_rules! call {
    ($lambda:expr $(,$arg:expr)*) => ($lambda.call(($($arg,)*)))
}
