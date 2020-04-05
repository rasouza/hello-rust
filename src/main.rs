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
    println!("Debug: {:?}", point);
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