pub(super) fn switch() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}

pub(super) fn pattern_matching() {
    let pair = (1, 2, 3);
    match pair { // Pattern Matching
        (0, z, _) => println!("Captured {} ", z), // Never matched
        (1, x, y) => println!("Captured {} and {}", x, y),
        _ => (),
    }
}

pub(super) fn match_reference() {
    let reference = &42;
    match reference {
        &val => println!("The answer is: {}", val)
    }
    let mut num = 30;
    match num {
        ref mut num => {
            *num += 10;
            println!("Reference is: {}", num);
        }
    }
    println!("Reference is: {}", num);
}

pub(super) fn guard() {
    let point = (1, 2);
    match point {
        (x, y) if x > 0 && y > 0 => println!("Destructure: x({}), y({})", x, y),
        _ => println!("Point is negative"),
    }
}

pub(super) fn bind() {
    match 16 {
        0             => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }    
}