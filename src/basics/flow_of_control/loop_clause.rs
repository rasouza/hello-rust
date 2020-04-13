pub(super) fn loop_return() { 
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Display result: {}", result);
}

pub(super) fn break_continue() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

pub(super) fn labels() {
    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer; // This breaks the outer loop
        }
        // println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}