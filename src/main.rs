use raylib::prelude::*;

mod game;

use crate::game::sprite::{
    //decr_context, 
    //incr_context, 
    incr_frame, 
    new_sprite, 
};

use crate::game::gd;


fn handle_input(rl: &RaylibHandle, cam2d: &mut Camera2D) {
    if rl.is_key_pressed(KeyboardKey::KEY_Z) {
        cam2d.zoom += 1.0;
    } 
    else if rl.is_key_pressed(KeyboardKey::KEY_X) && cam2d.zoom > 1.0 {
        cam2d.zoom -= 1.0;
    } 
    //else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
    //incr_context(&mut human_idle_sprite);
    //}
    //else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
    //decr_context(&mut human_idle_sprite);
    //}
}



fn main() {
    let mut cam2d = Camera2D { offset: Vector2::zero(), target: Vector2::zero(), rotation: 0.0, zoom: 1.0};
    let origin_vec = Vector2::zero();
    let font_size = 20;
    let mut frame_count = 0;
    let (mut rl, thread) = raylib::init().size(gd::WINDOW_WIDTH, gd::WINDOW_HEIGHT).title(gd::WINDOW_TITLE).build();
    // Create render texture (needs to be mutable)
    let mut render_texture = rl.load_render_texture(&thread, gd::TARGET_WIDTH, gd::TARGET_HEIGHT).unwrap();
    // have to invert the height to flip the texture
    let render_src = Rectangle::new(0.0, 0.0, gd::TARGET_WIDTH as f32, -(gd::TARGET_HEIGHT as f32));
    let render_dst = Rectangle::new(0.0, 0.0, gd::WINDOW_WIDTH as f32, gd::WINDOW_HEIGHT as f32);
    let tx_path = "img/char/human_idle.png";
    let human_idle_texture = rl.load_texture(&thread, tx_path)
        .expect("Failed to load texture");
    rl.set_target_fps(gd::TARGET_FPS);
    let mut human_idle_sprite = new_sprite(&human_idle_texture, 4, 16);
    while !rl.window_should_close() {
        // get input
        handle_input(&rl, &mut cam2d);

        // update state

        // draw frame
        
        // Measure text first
        let text = "evildojo666";
        let text_width = rl.measure_text(text, font_size);
        let _text_x = (640 - text_width) / 2;
        let _text_y = 180 - 10;
        let rotation = 0.0;
        // Draw to texture
        {
            let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
            d.clear_background(Color::BLACK);
            {
                let mut cm = d.begin_mode2D(cam2d);
                //let src = human_idle_sprite.src;
                let dst = Rectangle {x: 0.0, y: 0.0, width: human_idle_sprite.src.width, height: human_idle_sprite.src.height};
                let rotation = 0.0;
                cm.draw_texture_pro(human_idle_sprite.texture, human_idle_sprite.src, dst, Vector2::zero(), rotation, Color::WHITE);
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
        d.draw_texture_pro(&render_texture, render_src, render_dst, origin_vec, rotation, Color::WHITE);
    }
}
