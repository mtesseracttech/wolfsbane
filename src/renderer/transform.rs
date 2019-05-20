use std::fmt;

use straal::{FloatType, Mat2, Mat3, Mat4, Quat, RotationOrder, Vec2, Vec3, Vec4};

use crate::get_model_matrix;

type RelatedTransform<S> = Rc<Cell<Transform<S>>>;

use super::*;
use core::borrow::Borrow;
use std::cell::Cell;
use std::rc::Rc;

pub enum Space {
    Local,
    World,
}

/*struct RelatedNode {
    name: String,
    hash: u64,
}*/

//#[derive(Clone, Debug)]
pub struct Transform<S> {
    position: Vec3<S>,
    world_rotation: Quat<S>,
    local_rotation: Quat<S>,
    scale: Vec3<S>,
    transform: Mat4<S>,
    changed: bool,
    parent: Option<RelatedTransform<S>>,
    children: Vec<RelatedTransform<S>>,
    name: String,
}

#[allow(dead_code)]
impl<S> Transform<S>
where
    S: straal::FloatType<S>,
{
    pub fn default() -> Transform<S> {
        Transform {
            position: Vec3::zero(),
            rotation: Quat::identity(),
            scale: Vec3::one(),
            transform: Mat4::identity(),
            changed: true,
            parent: None,
            children: Vec::new(),
            name: String::from("unnamed"),
        }
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }

    fn get_parent(&mut self) -> Option<RelatedTransform<S>> {
        self.parent.clone()
    }

    fn detach_children(&mut self) {
        unimplemented!();
    }

    fn get_child_by_index(&mut self, index: usize) -> Option<&mut Transform<S>> {
        unimplemented!();
    }

    fn get_child_index_by_name(&mut self, name: &str) -> Option<usize> {
        unimplemented!();
    }

    fn get_child_by_name(&mut self, name: &str) -> Option<&mut Transform<S>> {
        unimplemented!();
    }

    fn is_child_of(&self, parent: Transform<S>) -> bool {
        unimplemented!();
    }

    fn update_matrix(&mut self) {
        let s = Mat4::get_uniform_scale_mat(self.scale);
        let r = Mat4::from(self.rotation);
        let t = Mat4::get_translation_mat(self.position);
        self.transform = s * r * t;
        self.changed = false;
    }

    pub fn get_local_to_world_matrix(&mut self) -> Mat4<S> {
        if self.changed {
            self.update_matrix();
        }
        self.transform
    }

    pub fn get_world_to_local_matrix(&mut self) -> Mat4<S> {
        self.get_local_to_world_matrix().inverse()
    }

    pub fn get_local_position(&self) -> Vec3<S> {
        self.position
    }

    pub fn set_local_position(&mut self, position: Vec3<S>) {
        self.position = position;
        self.changed = true;
    }

    pub fn get_world_position() -> Vec3<S> {
        unimplemented!()
    }

    pub fn get_local_rotation(&self) -> Quat<S> {
        self.rotation
    }

    pub fn set_local_rotation(&mut self, rotation: Quat<S>) {
        self.rotation = rotation;
        self.changed = true;
    }

    pub fn get_world_rotation() -> Quat<S> {
        unimplemented!()
    }

    pub fn get_local_scale(&self) -> Vec3<S> {
        self.scale
    }

    pub fn set_local_scale(&mut self, scale: Vec3<S>) {
        self.scale = scale;
        self.changed = true;
    }

    pub fn get_world_scale() -> Vec3<S> {
        unimplemented!()
    }

    pub fn translate(&mut self, delta_pos: Vec3<S>) {
        self.position += delta_pos;
        self.changed = true;
    }

    pub fn get_right(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * Vec3::right()).normalized()
    }

    pub fn get_left(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * -Vec3::right()).normalized()
    }

    pub fn get_up(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * Vec3::up()).normalized()
    }

    pub fn get_down(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * -Vec3::up()).normalized()
    }

    pub fn get_forward(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * Vec3::forward()).normalized()
    }

    pub fn get_backward(&mut self) -> Vec3<S> {
        (self.get_local_rotation() * -Vec3::forward()).normalized()
    }

    pub fn set_forward(&mut self, fwd: Vec3<S>, up: Vec3<S>) {
        let fwd = fwd.normalized();
        let rht = up.cross(fwd).normalized();
        let up = fwd.cross(rht).normalized();
        self.rotation = Quat::from(Mat3::new_from_vec3s(rht, up, fwd));
        self.changed = true;
    }

    pub fn rotate_around(&mut self, point: Vec3<S>, axis: Vec3<S>, theta: S) {
        unimplemented!();
        //        let mut world_pos = self.get_world_position();
        //        let rotation = Quat::from_angle_axis(axis, theta);
        //        let diff = rotation * (world_pos - point);
        //        let world_pos = point + diff;
        //        self.position = world_pos;
        //        RotateAroundInternal(axis, angle * Mathf.Deg2Rad);
    }

    fn rotate_around_in_world(&mut self, v_world: Vec3<S>, theta: S) {
        unimplemented!();
    }

    fn rotate_around(&mut self, point: Vec3<S>, axis: Vec3<S>, theta: S) {
        let mut world_pos = self.get_world_position();
        let rotation = Quat::from_angle_axis(axis, theta);
    }

    fn rotate_around_internal(&mut self, world_axis: Vec3<S>, rad: S) {
        let mut local_axis = InverseTransformDirection(worldAxis);
        if local_axis.sqrMagnitude > S::DEF_EPSILON {
            local_axis.normalize();
            let q = Quat::get_quat_from_angle_axis_safe(local_axis, rad);
            self.local_rotation = Quaternion.NormalizeSafe(m_LocalRotation * q);
            self.changed = true;
        }
    }

    //fn rotate_around_in_world(&mut self, v_world: Vec3<S>, theta: S) {}

    fn transform_direction(dir: Vec3<S>) -> Vec3<S> {
        unimplemented!()
        //let local_axis =
        //        Vector3 localAxis = InverseTransformDirection(worldAxis);
        //        if (localAxis.sqrMagnitude > Vector3.kEpsilon)
        //        {
        //            localAxis.Normalize();
        //            Quaternion q = Quaternion.AxisAngleToQuaternionSafe(localAxis, rad);
        //            m_LocalRotation = Quaternion.NormalizeSafe(m_LocalRotation * q);
        //            SetDirty();
        //        }
        //return Quaternion.RotateVectorByQuat (GetRotation (), inDirection);
    }

    fn inverse_transform_direction(dir: Vec3<S>) -> Vec3<S> {
        unimplemented!()
        //return Quaternion.RotateVectorByQuat(Quaternion.Inverse(GetRotation()), inDirection);
    }

    fn transform_to_world_space(&mut self, point: Vec3<S>) -> Vec3<S> {
        let v = Vec4::from(point);
        let m = self.get_local_to_world_matrix();
        Vec3::from(m * v)
    }
}

impl<S> fmt::Display for Transform<S>
where
    S: FloatType<S>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transformation:\n\
             Position:{}\n\
             Rotation:{}\n\
             Scale:{}",
            self.position,
            self.rotation.get_euler_angles_obj_upr_deg(),
            self.scale
        )
    }
}
