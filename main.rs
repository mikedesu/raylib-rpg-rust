use raylib::prelude::*;

mod sprite;

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let render_texture_width = 640;
    let render_texture_height = 360;
    let target_fps = 60;
    let origin_vec = Vector2::zero();
    let font_size = 20;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raylib Project")
        .build();
    // Create render texture (needs to be mutable)
    let mut render_texture = rl
        .load_render_texture(&thread, render_texture_width, render_texture_height)
        .unwrap();

    // have to invert the height to flip the texture
    let render_src = Rectangle::new(
        0.0,
        0.0,
        render_texture_width as f32,
        -(render_texture_height as f32),
    );

    let render_dst = Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32);
    let human_idle_texture = rl
        .load_texture(&thread, "img/char/human_idle.png")
        .expect("Failed to load texture");

    rl.set_target_fps(target_fps);

    while !rl.window_should_close() {
        // Measure text first
        let text = "evildojo666";
        let text_width = rl.measure_text(text, font_size);
        let text_x = (640 - text_width) / 2;
        let text_y = 180 - 10;
        let font_color = Color::WHITE;
        let rotation = 0.0;
        let render_color = Color::WHITE;
        // Draw to texture
        {
            let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
            d.clear_background(Color::BLACK);
            let src = Rectangle {
                x: 0.0,
                y: 0.0,
                width: human_idle_texture.width as f32,
                height: human_idle_texture.height as f32,
            };
            let dst = Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1280.0,
                height: 720.0,
            };
            let rotation = 0.0;
            d.draw_texture_pro(
                &human_idle_texture,
                src,
                dst,
                Vector2::zero(),
                rotation,
                Color::WHITE,
            );
            d.draw_text("evildojo666", text_x, text_y, font_size, font_color);
            d.draw_fps(10, 10);
        }
        // Draw texture to window
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(
            &render_texture,
            render_src,
            render_dst,
            origin_vec,
            rotation,
            render_color,
        );
    }
}
