use crate::pixel::Pixel;

mod bitmap;
mod display;
mod pixel;
fn main() {
    let mut disp = display::Display::new([800, 600], "Software rendering".to_owned());
    disp.start();
    disp.bitmap.fill(0x80);
    disp.bitmap.draw_pixel(100, 100, Pixel::new(255, 0, 0, 255));
    while disp.run() {
        disp.update();
    }
    disp.stop();
}
