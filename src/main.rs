use std::fmt;

#[derive(Debug)]
struct MyStruct(i32);

#[derive(Debug)]
struct MyDeep(MyStruct);

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

#[derive(Debug)]
struct Point3D(f64, f64, f64);

struct List(Vec<i32>);

fn reverse(pair: (bool, i32)) -> (i32,bool) {
    let (boolean, integer) = pair;
    (integer,boolean)
}

fn analyze(slice: &[i32]) {
    println!("First element: {}", slice[0]);
    println!("Length: {}", slice.len());
}

enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {

    // String Interpolation
    println!("Hello, world!");
    println!("{0}, {1}!","Hello", "World");
    println!("Pi is {:.prec$}", 3.145454254263452, prec=2);

    // Structs
    println!("{:?}", MyStruct(3));
    println!("{:#?}", MyDeep(MyStruct(5)));
    let point = Complex { real: 1.4, imag: 2. };
    println!("Display: {}", point);
    println!("New point Display: {}", Complex { real: 2.0, ..point});
    println!("Debug: {:?}", point);
    println!("Display: {}", List(vec![6,5,7,4,8]));
    let Complex { real, imag } = point;
    println!("Struct destructured | real: {}, imag: {}", real, imag);
    println!("Display: {:?}", Point3D(1.0, 2.0, 10.0));

    // Literals and Operators
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2); // TODO Try changing `1i32` to `1u32` to see why the type is important
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Tuples
    let long_tuple = (true, 1, 2, 2u32, 1i64, -3i32, 4.2f32);
    println!("First and second values of long_tuple are: {0}, {1}", long_tuple.0, long_tuple.1);
    println!("Reversed: {:?}", reverse((long_tuple.0, long_tuple.1)));

    // Arrays and Slices
    let x: [i32; 5] = [0,1,2,3,4];
    let y: [i32; 5] = [0; 5];
    let slice = &y[1..4];
    analyze(&x[1..4]);
    analyze(slice);

    // Enums
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20.2 as i64, y: 80 }; // Casting
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    use crate::WebEvent::*;
    let event = PageLoad;
    match event {
        PageLoad => println!("Page Loaded event"),
        PageUnload => println!("Page Unloaded event"),
        _ => {}
    }

    // Casting
    let decimal = 65.34_f32;
    println!("Display decimal: {} -> {} -> {}", decimal, decimal as u8, decimal as u8 as char);

    // Inference
    let elem = 1u8;
    let mut vec = Vec::new(); // vec type is not defined
    vec.push(elem); // vec type is lately infered here
    println!("Debug vec: {:?}", vec);

    // Aliasing
    type Meters = u32;
    let distance: Meters = 3;
    println!("Display distance: {}", distance);

    // Conversion
    let num = 42;
    println!("Display Complex: {}", Complex::from(num));
    let complex: Complex = 30.into();
    println!("Display Complex: {}", complex);

    // Expressions
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x // This expression will be assigned to `y`
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);

    // Flow of Control - IF
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

    // Flow of Control - LOOP
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
    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer; // This breaks the outer loop
        }
        // println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Display result: {}", result);

    // Flow of Control - WHILE
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    // Flow of Control - FOR and RANGE
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // Flow of Control - Match
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    let pair = (1, 2, 3);
    match pair { // Pattern Matching
        (0, z, _) => println!("Captured {} ", z), // Never matched
        (1, x, y) => println!("Captured {} and {}", x, y),
        _ => (),
    }
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
    match complex {
        Complex { real, imag } if real == 30_f64 => println!("Destructure: real({}), imag({})", real, imag),
        _ => (),
    }
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

    // Flow of Control - IF LET
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} + {:.2}i", self.real, self.imag)
    }
}

impl From<i32> for Complex {
    fn from(num: i32) -> Self {
        Complex { real: num as f64, imag: 0.0_f64 }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f,"]")     
    }
}