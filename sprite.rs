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

//impl Sprite {
//    pub fn new(t: Texture2D, nc: i32, nf: i32) -> Self {
//        Sprite {
//            texture: t,
//            width: t.width / nf,
//            height: t.height / nc,
//            numcontexts: nc,
//            numframes: nf,
//            currentframe: 0,
//            currentcontext: 0,
//            numloops: 0,
//            stop_on_last_frame: false,
//            is_animating: true,
//            src: Rectangle {
//                x: 0.0,
//                y: 0.0,
//                width: (t.width / nf) as f32,
//                height: (t.height / nc) as f32,
//            },
//        }
//    }
//}
