pub fn basics() {
    let base = |n| { 
        println!("Base is: {}", n);
    };
    base(32);
}

pub fn input_argument() {
    let double = |x| 2 * x;
    println!("3 doubled: {}", helpers::apply_to_3(double));
}

mod helpers {
    pub(super) fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {
        f(3)
    }
}