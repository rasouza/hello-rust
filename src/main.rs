#[derive(Debug)]
struct MyStruct(i32);

#[derive(Debug)]
struct MyDeep(MyStruct);

fn main() {

    // String Interpolation
    println!("Hello, world!");
    println!("{0}, {1}!","Hello", "World");
    println!("Pi is {:.prec$}", 3.145454254263452, prec=2);

    // Structs
    println!("{:?}", MyStruct(3));
    println!("{:#?}", MyDeep(MyStruct(5)));
}
