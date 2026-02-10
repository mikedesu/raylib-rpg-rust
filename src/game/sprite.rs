use raylib::prelude::*;

use crate::game::mprint;

#[allow(dead_code)]
pub struct Sprite<'a> {
    pub texture: &'a Texture2D,
    pub numcontexts: f32,
    pub numframes: f32,
    pub currentframe: f32,
    pub currentcontext: f32,
    pub numloops: i32,
    pub stop_on_last_frame: bool,
    pub is_animating: bool,
    pub src: Rectangle,
}

impl<'a> Sprite<'_> {
    #[allow(dead_code)]
    pub fn new(t: &Texture2D, nc: f32, nf: f32) -> Sprite<'_> {
        Sprite {
            texture: t,
            numcontexts: nc,
            numframes: nf,
            currentframe: 0.0,
            currentcontext: 0.0,
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
    pub fn incr_frame(&mut self) {
        if self.is_animating {
            self.currentframe += 1.0;
            if self.stop_on_last_frame && self.currentframe == self.numframes - 1.0 {
                self.is_animating = false;
            } else if self.currentframe >= self.numframes {
                self.currentframe = 0.0;
                self.numloops += 1;
            }
            self.src.x = self.src.width * self.currentframe;
        }
    }

    #[allow(dead_code)]
    pub fn set_context(&mut self, ctx: f32) {
        mprint::info(format!("set_context({})", ctx));
        if ctx >= 0.0 && ctx < self.numcontexts {
            self.currentcontext = ctx % self.numcontexts;
            self.src.y = self.src.height * self.currentcontext;
            self.currentframe = 0.0;
            self.src.x = 0.0;
        }
    }

    #[allow(dead_code)]
    pub fn incr_context(&mut self) {
        self.set_context(self.currentcontext + 1.0);
    }

    #[allow(dead_code)]
    pub fn decr_context(&mut self) {
        self.set_context(self.currentcontext - 1.0);
    }
}
