pub mod flow_of_control;

pub fn string_interpolation() {
    // String Interpolation
    println!("Hello, world!");
    println!("{0}, {1}!","Hello", "World");
    println!("Pi is {:.prec$}", 3.145454254263452, prec=2);
}

pub fn literals() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2); // TODO Try changing `1i32` to `1u32` to see why the type is important
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

pub fn tuples() {
    let long_tuple = (true, 1, 2, 2u32, 1i64, -3i32, 4.2f32);
    println!("First and second values of long_tuple are: {0}, {1}", long_tuple.0, long_tuple.1);
    println!("Reversed: {:?}", helpers::reverse((long_tuple.0, long_tuple.1)));
}

pub fn arrays() {
    let x: [i32; 5] = [0,1,2,3,4];
    let y: [i32; 5] = [0; 5];
    let slice = &y[1..4];
    helpers::analyze(&x[1..4]);
    helpers::analyze(slice);
}

pub fn casting() {
    let decimal = 65.34_f32;
    println!("Display decimal: {} -> {} -> {}", decimal, decimal as u8, decimal as u8 as char);
}

pub fn inference() {
    let elem = 1u8;
    let mut vec = Vec::new(); // vec type is not defined
    vec.push(elem); // vec type is lately infered here
    println!("Debug vec: {:?}", vec);
}

pub fn aliasing() {
    type Meters = u32;
    let distance: Meters = 3;
    println!("Display distance: {}", distance);
}

pub fn expressions() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x // This expression will be assigned to `y`
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
}

pub fn conversion() {
    use crate::utils;

    let num = 42;
    println!("Display Complex: {}", utils::Complex::from(num));
    let complex: utils::Complex = 30.into();
    println!("Display Complex: {}", complex);
}

mod helpers {
    pub(super) fn reverse(pair: (bool, i32)) -> (i32,bool) {
        let (boolean, integer) = pair;
        (integer,boolean)
    }

    pub(super) fn analyze(slice: &[i32]) {
        println!("First element: {}", slice[0]);
        println!("Length: {}", slice.len());
    }
}
