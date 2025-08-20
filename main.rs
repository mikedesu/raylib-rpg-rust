use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Raylib Project")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_text(
            "Hello World", 
            (1280 - rl.measure_text("Hello World", 20)) / 2, 
            360 - 10, 
            20, 
            Color::WHITE
        );
    }
}
