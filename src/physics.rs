// All in rads.
pub trait AngularKinematics {
    fn rotate(&self) -> f32;
    fn accelerate(&self, mouse_wheel_line_delta: f32);
}
