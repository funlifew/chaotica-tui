use super::vector::Vec3;

pub fn rk4_step<F>(position: Vec3, dt: f64, derivative: F) -> Vec3
where
    F: Fn(Vec3) -> Vec3,
{
    let k1 = derivative(position);
    let k2 = derivative(position + k1 * (dt * 0.5));
    let k3 = derivative(position + k2 * (dt * 0.5));
    let k4 = derivative(position + k3 * dt);

    position + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_velocity() {
        let result = rk4_step(Vec3::zero(), 1.0, |_| Vec3::new(1.0, 2.0, 3.0));

        assert_eq!(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn exponential_growth() {
        let result = rk4_step(Vec3::new(1.0, 0.0, 0.0), 0.1, |p| p);

        let expected = 0.1_f64.exp();

        assert!((result.x - expected).abs() < 1e-6);
    }
}
