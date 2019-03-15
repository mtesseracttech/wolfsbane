use super::*;

pub struct Camera {
    transform: Transform,
    view_matrix: straal::Mat4,
    changed: bool,
}

impl Camera {
    fn update_view_matrix(&mut self) {
        self.view_matrix = self.transform.get_matrix().inverse();
        self.changed = false;
    }

    pub fn get_view_matrix(&mut self) -> straal::Mat4 {
        if self.changed {
            self.update_view_matrix();
        }
        self.view_matrix
    }

    pub fn look_at(&mut self, dir: straal::Vec3) {
        self.transform.set_forward(dir, straal::Vec3::forward());
        self.changed = true;
    }
}