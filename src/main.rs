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

struct List(Vec<i32>);

fn reverse(pair: (bool, i32)) -> (i32,bool) {
    let (boolean, integer) = pair;
    (integer,boolean)
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
    println!("Display: {}", List(vec![6,5,7,4,8]));

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
    println!("Reversed: {:?}",reverse((long_tuple.0, long_tuple.1)));
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