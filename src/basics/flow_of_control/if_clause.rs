pub(super) fn if_return() {
    let n = 5;
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n // This expression returns an `i32`.
        } else {
            println!(", and is a big number, halve the number");
            n / 2 // This expression must return an `i32` as well.
        };
    println!("{} -> {}", n, big_n);
}

pub(super) fn if_let() {
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}