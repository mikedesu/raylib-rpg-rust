use raylib::prelude::*;
mod game;
use crate::game::gamestate::Gamestate;
use crate::game::scene::SceneKind;
//use crate::game::mprint;
//use crate::game::sprite::{
//decr_context, 
//incr_context, 
//incr_frame, 
//new_sprite, 
//};

use crate::game::gd;

#[allow(dead_code)]
fn handle_input_title(rl: &RaylibHandle, g: &mut Gamestate) {
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        g.set_scene(SceneKind::NewGame);
    }
}


//fn handle_input(rl: &RaylibHandle, g: &mut Gamestate) {
    //if rl.is_key_pressed(KeyboardKey::KEY_Z) { cam2d.zoom += 1.0; } 
    //else if rl.is_key_pressed(KeyboardKey::KEY_X) && cam2d.zoom > 1.0 { cam2d.zoom -= 1.0; } 
    //else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
    //incr_context(&mut human_idle_sprite);
    //}
    //else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
    //decr_context(&mut human_idle_sprite);
    //}
    //match g.get_scene() {
    //    SceneKind::Title => {
    //        draw_title(&mut rl, &thread, &mut render_texture, &mut gamestate);
    //    },
    //    SceneKind::NewGame => {
    // 
    //    }
    //}
//}


fn draw_title(rl: &mut RaylibHandle, thread: &RaylibThread, mut render_texture: &mut RenderTexture2D, g: &mut Gamestate) {
    // Measure text first
    let text = "evildojo666";
    let font_size = 20;
    let text_width = rl.measure_text(text, font_size);
    let text_x = (640 - text_width) / 2;
    let text_y = 180 - 10;
    let rotation = 0.0;
    // Draw to texture
    {
        let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
        d.clear_background(Color::DARKGRAY);
        {
            //let mut cm = d.begin_mode2D(cam2d);
            //let src = human_idle_sprite.src;
            //let dst = Rectangle {x: 0.0, y: 0.0, width: human_idle_sprite.src.width, height: human_idle_sprite.src.height};
            //let rotation = 0.0;
            //cm.draw_texture_pro(human_idle_sprite.texture, human_idle_sprite.src, dst, Vector2::zero(), rotation, Color::WHITE);
            //if frame_count % 10 == 0 {
            //incr_frame(&mut human_idle_sprite);
            //}
        }
        //d.draw_fps(10, 10);
        let s: String = format!("frame_count: {}", g.get_frame_count());
        d.draw_text(s.as_str(), text_x, text_y, font_size, Color::WHITE);
    }
    // Draw texture to window
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    const ORIGIN_VEC: Vector2 = Vector2::zero();
    let render_src = Rectangle::new(0.0, 0.0, gd::TARGET_WIDTH as f32, -(gd::TARGET_HEIGHT as f32));
    let render_dst = Rectangle::new(0.0, 0.0, gd::WINDOW_WIDTH as f32, gd::WINDOW_HEIGHT as f32);
    d.draw_texture_pro(&render_texture, render_src, render_dst, ORIGIN_VEC, rotation, Color::WHITE);
}


fn draw_new_game_scene(rl: &mut RaylibHandle, thread: &RaylibThread, mut render_texture: &mut RenderTexture2D, _g: &mut Gamestate) {
    // Draw to texture
    {
        let mut d = rl.begin_texture_mode(&thread, &mut render_texture);
        d.clear_background(Color::DARKGRAY);
        {
            //let mut cm = d.begin_mode2D(cam2d);
            //let src = human_idle_sprite.src;
            //let dst = Rectangle {x: 0.0, y: 0.0, width: human_idle_sprite.src.width, height: human_idle_sprite.src.height};
            //let rotation = 0.0;
            //cm.draw_texture_pro(human_idle_sprite.texture, human_idle_sprite.src, dst, Vector2::zero(), rotation, Color::WHITE);
            //if frame_count % 10 == 0 {
            //incr_frame(&mut human_idle_sprite);
            //}
        }
        //d.draw_fps(10, 10);
        d.draw_text("New Game", 10, 10, 20, Color::WHITE);
    }

    // Draw texture to window
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    const ORIGIN_VEC: Vector2 = Vector2::zero();
    const RENDER_SRC: Rectangle = Rectangle::new(0.0, 0.0, gd::TARGET_WIDTH as f32, -(gd::TARGET_HEIGHT as f32));
    const RENDER_DST: Rectangle = Rectangle::new(0.0, 0.0, gd::WINDOW_WIDTH as f32, gd::WINDOW_HEIGHT as f32);
    d.draw_texture_pro(&render_texture, RENDER_SRC, RENDER_DST, ORIGIN_VEC, 0.0, Color::WHITE);
}



fn main() {
    let mut gamestate: Gamestate = Gamestate::new();
    let (mut rl, thread) = raylib::init().size(gd::WINDOW_WIDTH, gd::WINDOW_HEIGHT).title(gd::WINDOW_TITLE).build();
    // Create render texture (needs to be mutable)
    let mut render_texture = rl.load_render_texture(&thread, gd::TARGET_WIDTH, gd::TARGET_HEIGHT).unwrap();
    // have to invert the height to flip the texture
    //let tx_path = "img/char/human_idle.png";
    //let human_idle_texture = rl.load_texture(&thread, tx_path)
    //    .expect("Failed to load texture");
    rl.set_target_fps(gd::TARGET_FPS);
    //let mut human_idle_sprite = new_sprite(&human_idle_texture, 4, 16);
    while !rl.window_should_close() {
        //handle_input(&rl, &mut cam2d);
        // update state
        // draw frame
        //if gamestate.get_scene() == SceneKind::Title {
        //    // do stuff
        //}
        // get input
        match gamestate.get_scene() {
            SceneKind::Title => {
                handle_input_title(&rl, &mut gamestate);
            },
            SceneKind::NewGame => {

            }
        }
        // update state
        // draw frame
        //mprint::info(format!("draw_frame"));
        match gamestate.get_scene() {
            SceneKind::Title => {
                //mprint::info(format!("drawing title"));
                draw_title(&mut rl, &thread, &mut render_texture, &mut gamestate);
            },
            SceneKind::NewGame => {
                //mprint::info(format!("drawing new game"));
                draw_new_game_scene(&mut rl, &thread, &mut render_texture, &mut gamestate);        
            }
        }
        gamestate.incr_frame_count();
    }
}
