use raylib::prelude::*;

use crate::game::mprint;

#[allow(dead_code)]
pub struct Sprite<'a> {
    pub texture: &'a Texture2D,
    //pub width: i32,
    //pub height: i32,
    pub numcontexts: i32,
    pub numframes: i32,
    pub currentframe: i32,
    pub currentcontext: i32,
    pub numloops: i32,
    pub stop_on_last_frame: bool,
    pub is_animating: bool,
    pub src: Rectangle,
}


#[allow(dead_code)]
pub fn new_sprite(t: &Texture2D, nc: i32, nf: i32) -> Sprite<'_> {
    Sprite {
        texture: t,
        //width: 32,
        //height: 32,
        numcontexts: nc,
        numframes: nf,
        currentframe: 0,
        currentcontext: 0,
        numloops: 0,
        stop_on_last_frame: false,
        is_animating: true,
        src: Rectangle {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        },
    }
}


#[allow(dead_code)]
pub fn incr_frame(s: &mut Sprite) {
    if s.is_animating {
        s.currentframe += 1;
        if s.stop_on_last_frame && s.currentframe == s.numframes - 1 {
            s.is_animating = false;
            //s.src.x = s.src.width * s.currentframe as f32;
        } else if s.currentframe >= s.numframes {
            s.currentframe = 0;
            s.numloops += 1;
        }
        s.src.x = s.src.width * s.currentframe as f32;
    }
}


#[allow(dead_code)]
pub fn set_context(s: &mut Sprite, ctx: i32) {
    mprint::info(format!("set_context({})", ctx));
    if ctx >= 0 && ctx < s.numcontexts {
        s.currentcontext = ctx % s.numcontexts;
        s.src.y = s.src.height * s.currentcontext as f32;
        s.currentframe = 0;
        s.src.x = 0.0;
    }
}


#[allow(dead_code)]
pub fn incr_context(s: &mut Sprite) {
    set_context(s, s.currentcontext + 1);
}


#[allow(dead_code)]
pub fn decr_context(s: &mut Sprite) {
    set_context(s, s.currentcontext - 1);
}
