mod display;
mod bitmap;
mod pixel;
fn main() {
    println!("Hello, world!");
    display::Display::new(400.0, 400.0, "hello".to_owned());
}
