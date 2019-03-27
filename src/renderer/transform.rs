use std::fmt;

use super::*;

pub enum Space {
    Local,
    World,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transform {
    position: Vec3n,
    rotation: Quatn,
    scale: Vec3n,
    transform: Mat4n,
    changed: bool,
    parent: Option<Box<Transform>>,
    children: Vec<Transform>,
}

#[allow(dead_code)]
impl Transform {
    pub fn default() -> Transform {
        Transform {
            position: Vec3n::zero(),
            rotation: Quatn::identity(),
            scale: Vec3n::one(),
            transform: Mat4n::identity(),
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
        let s = Mat4n::get_uniform_scale_mat(self.scale);
        let r = Mat4n::from(self.rotation);
        let t = Mat4n::get_translation_mat(self.position);
        self.transform = s * r * t;
        self.changed = false;
    }

    pub fn get_local_to_world_matrix(&mut self) -> Mat4n {
        if self.changed {
            self.update_matrix();
        }
        self.transform
    }

    pub fn get_world_to_local_matrix(&mut self) -> Mat4n {
        self.get_local_to_world_matrix().inverse()
    }


    pub fn get_local_position(&self) -> Vec3n {
        self.position
    }

    pub fn set_local_position(&mut self, position: Vec3n) {
        self.position = position;
        self.changed = true;
    }

    pub fn get_world_position() -> Vec3n {
        unimplemented!()
    }


    pub fn get_local_rotation(&self) -> Quatn {
        self.rotation
    }

    pub fn set_local_rotation(&mut self, rotation: Quatn) {
        self.rotation = rotation;
        self.changed = true;
    }

    pub fn get_world_rotation() -> Quatn {
        unimplemented!()
    }


    pub fn get_local_scale(&self) -> Vec3n {
        self.scale
    }

    pub fn set_local_scale(&mut self, scale: Vec3n) {
        self.scale = scale;
        self.changed = true;
    }

    pub fn get_world_scale() -> Vec3n {
        unimplemented!()
    }

    pub fn translate(&mut self, delta_pos: Vec3n) {
        self.position += delta_pos;
        self.changed = true;
    }

    pub fn rotate_euler(&mut self, pitch: f32, heading: f32, bank: f32) {
        self.rotation *= Quatn::get_quat_flex_euler_deg(pitch, heading, bank, straal::RotationOrder::HBP);
        self.changed = true;
    }

    pub fn rotate_angle_axis(&mut self, theta: f32, n: Vec3n) {
        self.rotation *= Quatn::get_quat_from_angle_axis(theta, n);
        self.changed = true;
    }

    pub fn get_right(&mut self) -> Vec3n {
        (self.get_local_rotation() * Vec3n::right()).normalized()
    }

    pub fn get_left(&mut self) -> Vec3n {
        (self.get_local_rotation() * -Vec3n::right()).normalized()
    }

    pub fn get_up(&mut self) -> Vec3n {
        (self.get_local_rotation() * Vec3n::up()).normalized()
    }

    pub fn get_down(&mut self) -> Vec3n {
        (self.get_local_rotation() * -Vec3n::up()).normalized()
    }

    pub fn get_forward(&mut self) -> Vec3n {
        (self.get_local_rotation() * Vec3n::forward()).normalized()
    }

    pub fn get_backward(&mut self) -> Vec3n {
        (self.get_local_rotation() * -Vec3n::forward()).normalized()
    }


    pub fn set_forward(&mut self, fwd: Vec3n, up: Vec3n) {
        let fwd = fwd.normalized();
        let rht = up.cross(fwd).normalized();
        let up = fwd.cross(rht).normalized();
        self.rotation = Quatn::from(Mat3n::new_from_vec3s(rht, up, fwd));
        self.changed = true;
    }

    pub fn rotate_around(&mut self, point: Vec3n, axis: Vec3n, theta: f32) {
//        let mut world_pos = self.get_world_position();
//        let rotation = Quat::from_angle_axis(axis, theta);
//        let diff = rotation * (world_pos - point);
//        let world_pos = point + diff;
//        self.position = world_pos;
//        RotateAroundInternal(axis, angle * Mathf.Deg2Rad);
    }

    fn rotate_around_in_world(&mut self, v_world: Vec3n, theta: f32) {}

    fn transform_direction(dir: Vec3n) -> Vec3n
    {
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

    fn inverse_transform_direction(dir: Vec3n) -> Vec3n
    {
        unimplemented!()

        //return Quaternion.RotateVectorByQuat(Quaternion.Inverse(GetRotation()), inDirection);
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