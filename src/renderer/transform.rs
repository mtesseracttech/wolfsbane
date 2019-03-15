use std::fmt;

use super::*;

pub struct Transform {
    position: straal::Vec3,
    rotation: straal::Mat3,
    scale: straal::Vec3,
    transform: straal::Mat4,
    changed: bool,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            position: straal::Vec3::zero(),
            rotation: straal::Mat3::identity(),
            scale: straal::Vec3::new(1.0, 1.0, 1.0),
            transform: straal::Mat4::identity(),
            changed: true,
        }
    }

    fn update_matrix(&mut self) {
        let scale = straal::Mat4::get_scale_mat(self.scale);
        let rotation = straal::Mat4::from(self.rotation);
        let trans = straal::Mat4::get_translation_mat(self.position);
        println!("{}", rotation);
        self.transform = scale * rotation * trans;
        self.changed = false;
    }

    pub fn get_matrix(&mut self) -> straal::Mat4 {
        if self.changed {
            self.update_matrix();
        }
        self.transform
    }

    pub fn get_scale(&self) -> straal::Vec3 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: straal::Vec3) {
        self.scale = scale;
        self.changed = true;
    }

    pub fn get_position(&self) -> straal::Vec3 {
        self.position
    }

    pub fn set_position(&mut self, pos: straal::Vec3) {
        self.position = pos;
        self.changed = true;
    }

    pub fn get_right(&mut self) -> straal::Vec3 {
        straal::Vec3::from(self.get_matrix().r0).normalized()
    }

    pub fn get_up(&mut self) -> straal::Vec3 {
        straal::Vec3::from(self.get_matrix().r1).normalized()
    }

    pub fn get_forward(&mut self) -> straal::Vec3 {
        straal::Vec3::from(self.get_matrix().r2).normalized()
    }

    pub fn set_forward(&mut self, fwd: straal::Vec3, up: straal::Vec3) {
        let fwd = fwd.normalized();
        let rht = straal::Vec3::cross(&up, &fwd).normalized();
        let up = straal::Vec3::cross(&fwd, &rht).normalized();
        self.rotation = straal::Mat3::new_from_vec3s(rht, up, fwd);
        self.changed = true;
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transformation (Position:{} Rotation:{} Scale:{})", self.position, self.rotation.get_euler_angles(), self.scale)
    }
}