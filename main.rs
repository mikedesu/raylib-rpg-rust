use raylib::prelude::*;

mod sprite;
use crate::sprite::Sprite;

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let render_texture_width = 640;
    let render_texture_height = 360;
    let target_fps = 60;
    let origin_vec = Vector2::zero();
    let font_size = 20;
    let mut frame_count = 0;

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

    let mut human_idle_sprite = Sprite {
        texture: &human_idle_texture,
        width: 32,
        height: 32,
        numcontexts: 4,
        numframes: 16,
        currentframe: 0,
        currentcontext: 0,
        numloops: 0,
        stop_on_last_frame: false,
        is_animating: true,
        src: Rectangle {
            x: 0.0,
            y: 0.0,
            width: 64.0,
            height: 64.0,
        },
    };

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

            let src = human_idle_sprite.src;

            let dst = Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1280.0,
                height: 720.0,
            };

            let rotation = 0.0;

            d.draw_texture_pro(
                human_idle_sprite.texture,
                src,
                dst,
                Vector2::zero(),
                rotation,
                Color::WHITE,
            );

            if frame_count % 10 == 0 {
                human_idle_sprite.currentframe += 1;
                if human_idle_sprite.currentframe >= human_idle_sprite.numframes {
                    human_idle_sprite.currentframe = 0;
                }

                human_idle_sprite.src = Rectangle {
                    x: (human_idle_sprite.currentframe * 64) as f32,
                    y: 0.0,
                    width: 64.0,
                    height: 64.0,
                };
            }

            d.draw_text("evildojo666", text_x, text_y, font_size, font_color);
            d.draw_fps(10, 10);
            frame_count += 1;
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
