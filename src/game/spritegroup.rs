use raylib::prelude::*;

use crate::game::sprite::Sprite;

#[allow(dead_code)]
pub struct Spritegroup<'a> {
    pub size: i32,
    pub capacity: i32,
    pub current: i32,
    pub off_x: i32,
    pub off_y: i32,
    pub default_anim: i32,
    pub alpha: i32,
    pub id: i32,
    pub dest: Rectangle,
    pub xmove: Rectangle,
    pub move_rate: f32,
    pub visible: bool,
    pub sprites: Vec<Sprite<'a>>,
}

//impl<'a> Spritegroup<'_> {
impl<'a> Spritegroup<'a> {
    #[allow(dead_code)]
    pub fn new() -> Spritegroup<'a> {
        Spritegroup {
            size: 0,
            capacity: 0,
            current: 0,
            off_x: 0,
            off_y: 0,
            default_anim: 0,
            alpha: 255,
            id: 0,
            dest: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            xmove: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            move_rate: 1.0,
            visible: true,
            sprites: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, s: Sprite<'a> ) {
        self.sprites.push(s);
    }
}
