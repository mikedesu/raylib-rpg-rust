use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Raylib Project")
        .build();

    while !rl.window_should_close() {
        let text_width = rl.measure_text("Hello World", 20);
        let text_x = (1280 - text_width) / 2;
        let text_y = 360 - 10;
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Hello World", text_x, text_y, 20, Color::WHITE);
    }
}
