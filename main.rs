use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Raylib Project")
        .build();

    // Create render texture (needs to be mutable)
    let mut render_texture = rl.load_render_texture(&thread, 640, 360).unwrap();

    while !rl.window_should_close() {
        // Draw to texture
        {
            let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
            d.clear_background(Color::BLACK);
            
            let text_width = rl.measure_text("Hello World", 20);
            let text_x = (640 - text_width) / 2;
            let text_y = 180 - 10;
            d.draw_text("Hello World", text_x, text_y, 20, Color::WHITE);
        }

        // Draw texture to window
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(
            &render_texture,
            Rectangle::new(0.0, 0.0, 640.0, 360.0),
            Rectangle::new(0.0, 0.0, 1280.0, 720.0),
            Vector2::zero(),
            0.0,
            Color::WHITE
        );
    }
}
