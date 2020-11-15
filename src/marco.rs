macro_rules! test {
    ($left: expr; and $right: expr) => (
        println!("{:?} and {:?} is {:?}",stringify!($left),stringify!($right), $left && $right)
    );
    ($left: expr; or $right: expr) => (
        println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right);
    )
}

fn main() {
    test!( 1i32 + 1 ==2i32;and {
        let mut a = 1 + 1;
        2i32 * 2 == 4i32
    });
    test!( 1 + 2 == 3;or 2 + 3 == 4);
}