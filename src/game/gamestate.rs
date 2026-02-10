use crate::game::scene::SceneKind;

use raylib::prelude::*;

pub struct Gamestate {
    frame_count: u32,
    current_scene: SceneKind,
    cam2d: Camera2D
}


impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            frame_count: 0,
            current_scene: SceneKind::Title,
            cam2d: Camera2D { offset: Vector2::zero(), target: Vector2::zero(), rotation: 0.0, zoom: 1.0}
        }
    }

    pub fn incr_frame_count(&mut self) {
        self.frame_count+=1;
    }

    pub fn get_frame_count(&self) -> u32 { self.frame_count }

    pub fn get_scene(&self) -> SceneKind {
        self.current_scene.clone()
    }

    pub fn set_scene(&mut self, s: SceneKind) {
        self.current_scene = s;
    }

    #[allow(dead_code)]
    pub fn get_camera(&mut self) -> &mut Camera2D {
        &mut self.cam2d
    }
}

