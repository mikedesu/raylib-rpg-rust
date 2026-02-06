use raylib::prelude::*;

pub struct Sprite<'a> {
    pub texture: &'a Texture2D,
    pub width: i32,
    pub height: i32,
    pub numcontexts: i32,
    pub numframes: i32,
    pub currentframe: i32,
    pub currentcontext: i32,
    pub numloops: i32,
    pub stop_on_last_frame: bool,
    pub is_animating: bool,
    pub src: Rectangle,
}

pub fn new_sprite(t: &Texture2D, nc: i32, nf: i32) -> Sprite<'_> {
    Sprite {
        texture: t,
        width: t.width / nf,
        height: t.height / nc,
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
            width: (t.width * 2 / nf) as f32,
            height: (t.height * 2 / nc) as f32,
        },
    }
}

pub fn incr_frame(s: &mut Sprite) {
    if s.is_animating {
        s.currentframe += 1;
        if s.stop_on_last_frame && s.currentframe == s.numframes - 1 {
            s.is_animating = false;
            s.src.x = (s.width * s.currentframe) as f32;
        } else if s.currentframe >= s.numframes {
            s.currentframe = 0;
            s.numloops += 1;
        }
        s.src.x = (s.width * s.currentframe) as f32;
    }
}
