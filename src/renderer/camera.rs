use super::*;

pub enum CameraMode {
    FirstPerson,
    LookAt,
    OrbCamera,
}

pub struct Camera {
    transform: Transform,
    view_matrix: Mat4n,
    changed: bool,
    mode: CameraMode,
}

impl Camera {
    fn default() -> Camera {
        Camera {
            transform: Transform::default(),
            view_matrix: Mat4n::identity(),
            changed: true,
            mode: CameraMode::FirstPerson,
        }
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = self.transform.get_local_to_world_matrix().inverse();
        self.changed = false;
    }

    pub fn get_view_matrix(&mut self) -> Mat4n {
        if self.changed {
            self.update_view_matrix();
        }
        self.view_matrix
    }

    pub fn look_at(&mut self, dir: Vec3n) {
        self.transform.set_forward(dir, Vec3n::forward());
        self.changed = true;
    }

    pub fn get_transform(&mut self) -> &mut Transform {
        unimplemented!()
    }
}