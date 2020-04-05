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