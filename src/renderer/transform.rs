use std::fmt;

use straal::{Mat3, Mat4, Quat, Real, Vec3, Vec4};

use super::*;

pub enum Space {
    Local,
    World,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
    transform: Mat4,
    changed: bool,
    parent: Option<Box<Transform>>,
    children: Vec<Transform>,
}

#[allow(dead_code)]
impl Transform {
    pub fn default() -> Transform {
        Transform {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            transform: Mat4::IDENTITY,
            changed: true,
            parent: None,
            children: Vec::new(),
        }
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }

    fn get_parent(&mut self) -> Option<Box<Transform>> {
        unimplemented!()
        //self.parent
    }

    fn detach_children(&mut self) {
        unimplemented!();
        for mut child in self.children {
            let mut parent = child.get_parent();
        }
    }

    fn get_child(&mut self, index: usize) -> Option<&mut Transform> {
        self.children.get_mut(index)
    }


    fn is_child_of(&self, parent: Transform) -> bool {
        unimplemented!();
    }

    fn update_matrix(&mut self) {
        let s = Mat4::get_scale_mat(self.scale);
        let r = Mat4::from(self.rotation);
        let t = Mat4::get_translation_mat(self.position);
        self.transform = s * r * t;
        self.changed = false;
    }

    pub fn get_local_to_world_matrix(&mut self) -> Mat4 {
        if self.changed {
            self.update_matrix();
        }
        self.transform
    }

    pub fn get_world_to_local_matrix(&mut self) -> Mat4 {
        self.get_local_to_world_matrix().inverse()
    }


    pub fn get_local_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_local_position(&mut self, position: Vec3) {
        self.position = position;
        self.changed = true;
    }

    pub fn get_world_position() -> Vec3 {
        unimplemented!()
    }


    pub fn get_local_rotation(&self) -> Quat {
        self.rotation
    }

    pub fn set_local_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.changed = true;
    }

    pub fn get_world_rotation() -> Quat {
        unimplemented!()
    }


    pub fn get_local_scale(&self) -> Vec3 {
        self.scale
    }

    pub fn set_local_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.changed = true;
    }

    pub fn get_world_scale() -> Vec3 {
        unimplemented!()
    }

    pub fn translate(&mut self, delta_pos: Vec3) {
        self.position += delta_pos;
        self.changed = true;
    }

    pub fn rotate_euler(&mut self, pitch: Real, heading: Real, bank: Real) {
        self.rotation *= Quat::from_euler_obj_upr_deg(pitch, heading, bank);
        self.changed = true;
    }

    pub fn rotate_angle_axis(&mut self, n: Vec3, theta: Real) {
        self.rotation *= Quat::from_angle_axis(n, theta);
        self.changed = true;
    }

    pub fn get_right(&mut self) -> Vec3 {
        (self.get_local_rotation() * Vec3::RIGHT).normalized()
    }

    pub fn get_left(&mut self) -> Vec3 {
        (self.get_local_rotation() * -Vec3::RIGHT).normalized()
    }

    pub fn get_up(&mut self) -> Vec3 {
        (self.get_local_rotation() * Vec3::UP).normalized()
    }

    pub fn get_down(&mut self) -> Vec3 {
        (self.get_local_rotation() * -Vec3::UP).normalized()
    }

    pub fn get_forward(&mut self) -> Vec3 {
        (self.get_local_rotation() * Vec3::FORWARD).normalized()
    }

    pub fn get_backward(&mut self) -> Vec3 {
        (self.get_local_rotation() * -Vec3::FORWARD).normalized()
    }


    pub fn set_forward(&mut self, fwd: Vec3, up: Vec3) {
        let fwd = fwd.normalized();
        let rht = Vec3::cross(up, fwd).normalized();
        let up = Vec3::cross(fwd, rht).normalized();
        self.rotation = Quat::from(Mat3::new_from_vec3s(rht, up, fwd));
        self.changed = true;
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Transformation:\n\
               Position:{}\n\
               Rotation:{}\n\
               Scale:{}", self.position, self.rotation.get_euler_angles_obj_upr_deg(), self.scale)
    }
}