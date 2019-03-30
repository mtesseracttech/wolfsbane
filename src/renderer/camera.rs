use straal::{FloatType, Mat4, Vec3};

use super::*;

pub enum CameraMode {
    FirstPerson,
    LookAt,
    OrbCamera,
}

pub struct Camera<S> {
    transform: Transform<S>,
    view_matrix: Mat4<S>,
    changed: bool,
    mode: CameraMode,
}

impl<S> Camera<S> where S: FloatType<S> {
    fn default() -> Camera<S> {
        Camera {
            transform: Transform::default(),
            view_matrix: Mat4::identity(),
            changed: true,
            mode: CameraMode::FirstPerson,
        }
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix = self.transform.get_local_to_world_matrix().inverse();
        self.changed = false;
    }

    pub fn get_view_matrix(&mut self) -> Mat4<S> {
        if self.changed {
            self.update_view_matrix();
        }
        self.view_matrix
    }

    pub fn look_at(&mut self, dir: Vec3<S>) {
        self.transform.set_forward(dir, Vec3::forward());
        self.changed = true;
    }
}