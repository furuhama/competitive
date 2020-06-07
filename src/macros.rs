#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        std::cmp::max($x, max!($($z),*))
    }}
}

#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        std::cmp::min($x, max!($($z),*))
    }}
}

#[macro_export]
macro_rules! echo {
    ($($e:expr),+) => ( { $(println!("{}", $e))+ } );
}
