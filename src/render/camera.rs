use crate::math::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub yaw: f64,
    pub pitch: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw: 0.35,
            pitch: 0.2,
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rotate(&mut self, yaw: f64, pitch: f64) {
        self.yaw = (self.yaw + yaw).rem_euclid(std::f64::consts::TAU);

        self.pitch =
            (self.pitch + pitch).clamp(-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2);
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.1).min(10.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.1).max(0.1);
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn project(&self, position: Vec3, center: Vec3) -> (f64, f64) {
        let point = position - center;

        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();

        let x = point.x * cos_yaw + point.y * sin_yaw;

        let depth = -point.x * sin_yaw + point.y * cos_yaw;

        let y = point.z * cos_pitch - depth * sin_pitch;

        (x * self.zoom, y * self.zoom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_projects_to_origin() {
        let camera = Camera::new();
        let center = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(camera.project(center, center), (0.0, 0.0));
    }

    #[test]
    fn default_axis_projection() {
        let camera = Camera {
            yaw: 0.0,
            pitch: 0.0,
            zoom: 1.0,
        };

        assert_eq!(
            camera.project(Vec3::new(2.0, 3.0, 4.0), Vec3::zero(),),
            (2.0, 4.0)
        );
    }
}
