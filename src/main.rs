

struct RGBColor(u8, u8, u8);

fn main() {
    let red = RGBColor(255,0,0);

    print!("{}, {}, {}", red.0, red.1, red.2)
}
