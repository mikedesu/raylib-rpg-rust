use raylib::prelude::*;
mod game;
//mod game::gd;
use crate::game::sprite::{
    decr_context, 
    incr_context, 
    incr_frame, 
    new_sprite, 
};
use crate::game::gd;
//use crate::game::gd::{
//    WINDOW_WIDTH,
//    WINDOW_HEIGHT,
//    TARGET_WIDTH,
//    TARGET_HEIGHT
//};

fn main() {
    let mut cam2d = Camera2D { offset: Vector2::zero(), target: Vector2::zero(), rotation: 0.0, zoom: 1.0};
    let target_fps = 60;
    let origin_vec = Vector2::zero();
    let font_size = 20;
    let mut frame_count = 0;
    let win_w = gd::WINDOW_WIDTH;
    let win_h = gd::WINDOW_HEIGHT;
    let win_w_f = win_w as f32;
    let win_h_f = win_h as f32;
    let target_w = gd::TARGET_WIDTH;
    let target_h = gd::TARGET_HEIGHT;
    let target_w_f = target_w as f32;
    let target_h_f = target_h as f32;
    let win_title = "evildojo666 presents";
    let (mut rl, thread) = raylib::init().size(win_w, win_h).title(win_title).build();
    // Create render texture (needs to be mutable)
    let mut render_texture = rl.load_render_texture(&thread, gd::TARGET_WIDTH, gd::TARGET_HEIGHT).unwrap();
    // have to invert the height to flip the texture
    let render_src = Rectangle::new(0.0, 0.0, target_w_f, -target_h_f);
    let render_dst = Rectangle::new(0.0, 0.0, win_w_f, win_h_f);
    let tx_path = "../img/char/human_idle.png";
    let human_idle_texture = rl.load_texture(&thread, tx_path)
        .expect("Failed to load texture");
    rl.set_target_fps(target_fps);
    let mut human_idle_sprite = new_sprite(&human_idle_texture, 4, 16);
    while !rl.window_should_close() {
        // get input
        if rl.is_key_down(KeyboardKey::KEY_Z) {
            cam2d.zoom += 0.1;
        } 
        else if rl.is_key_down(KeyboardKey::KEY_X) {
            if cam2d.zoom > 1.0 {
                cam2d.zoom -= 0.1;
            }
        } 
        else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            incr_context(&mut human_idle_sprite);
        }
        else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            decr_context(&mut human_idle_sprite);
        }

        // update state

        // draw frame

        // Measure text first
        let text = "evildojo666";
        let text_width = rl.measure_text(text, font_size);
        let _text_x = (640 - text_width) / 2;
        let _text_y = 180 - 10;
        let _font_color = Color::WHITE;
        let rotation = 0.0;
        let render_color = Color::WHITE;
        // Draw to texture
        {
            let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
            d.clear_background(Color::BLACK);
            {
                let mut cm = d.begin_mode2D(cam2d);
                let src = human_idle_sprite.src;
                let dst = Rectangle {x: 0.0, y: 0.0, width: src.width, height: src.height};
                let rotation = 0.0;
                cm.draw_texture_pro(human_idle_sprite.texture, src, dst, Vector2::zero(), rotation, Color::WHITE);
                if frame_count % 10 == 0 {
                    incr_frame(&mut human_idle_sprite);
                }
            }
            d.draw_fps(10, 10);
            //d.draw_text("evildojo666", text_x, text_y, font_size, font_color);
            frame_count += 1;
        }
        // Draw texture to window
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(&render_texture, render_src, render_dst, origin_vec, rotation, render_color);
    }
}
