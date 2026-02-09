use crate::game::scene::SceneKind;

pub struct Gamestate {
    frame_count: u32,
    current_scene: SceneKind
}


impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            frame_count: 0,
            current_scene: SceneKind::Title
        }
    }

    pub fn incr_frame_count(&mut self) {
        self.frame_count+=1;
    }

    pub fn get_frame_count(&self) -> u32 { self.frame_count }

    pub fn get_scene(&self) -> SceneKind {
        self.current_scene.clone()
    }
}

