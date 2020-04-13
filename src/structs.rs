use crate::utils::*;

pub fn basics() {
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
}

pub fn enums() {
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
    use WebEvent::*;
    let event = PageLoad;
    match event {
        PageLoad => println!("Page Loaded event"),
        PageUnload => println!("Page Unloaded event"),
        _ => {}
    }
}