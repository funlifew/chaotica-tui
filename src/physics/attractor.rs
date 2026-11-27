use crate::math::vector::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attractor {
    Lorenz,
    Rossler,
    Thomas,
    Aizawa,
    Halvorsen,
    Chen,
}

#[derive(Debug, Clone, Copy)]
pub struct Parameter {
    pub name: &'static str,
    pub default: f64,
    pub step: f64,
}

impl Parameter {
    pub const fn new(name: &'static str, default: f64, step: f64) -> Self {
        Self {
            name,
            default,
            step,
        }
    }
}

static LORENZ_PARAMETERS: [Parameter; 3] = [
    Parameter::new("Sigma", 10.0, 0.5),
    Parameter::new("Rho", 28.0, 1.0),
    Parameter::new("Beta", 8.0 / 3.0, 0.1),
];

static ROSSLER_PARAMETERS: [Parameter; 3] = [
    Parameter::new("a", 0.2, 0.01),
    Parameter::new("b", 0.2, 0.01),
    Parameter::new("c", 5.7, 0.1),
];

static THOMAS_PARAMETERS: [Parameter; 1] = [Parameter::new("b", 0.208186, 0.01)];

static AIZAWA_PARAMETERS: [Parameter; 6] = [
    Parameter::new("a", 0.95, 0.01),
    Parameter::new("b", 0.7, 0.01),
    Parameter::new("c", 0.6, 0.01),
    Parameter::new("d", 3.5, 0.1),
    Parameter::new("e", 0.25, 0.01),
    Parameter::new("f", 0.1, 0.01),
];

static HALVORSEN_PARAMETERS: [Parameter; 1] = [Parameter::new("a", 1.4, 0.05)];

static CHEN_PARAMETERS: [Parameter; 3] = [
    Parameter::new("a", 35.0, 0.5),
    Parameter::new("b", 3.0, 0.1),
    Parameter::new("c", 28.0, 0.5),
];

impl Attractor {
    pub const ALL: [Self; 6] = [
        Self::Lorenz,
        Self::Rossler,
        Self::Thomas,
        Self::Aizawa,
        Self::Halvorsen,
        Self::Chen,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Lorenz => "Lorenz",
            Self::Rossler => "Rossler",
            Self::Thomas => "Thomas",
            Self::Aizawa => "Aizawa",
            Self::Halvorsen => "Halvorsen",
            Self::Chen => "Chen",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Lorenz => Self::Rossler,
            Self::Rossler => Self::Thomas,
            Self::Thomas => Self::Aizawa,
            Self::Aizawa => Self::Halvorsen,
            Self::Halvorsen => Self::Chen,
            Self::Chen => Self::Lorenz,
        }
    }

    pub fn parameters(self) -> &'static [Parameter] {
        match self {
            Self::Lorenz => &LORENZ_PARAMETERS,
            Self::Rossler => &ROSSLER_PARAMETERS,
            Self::Thomas => &THOMAS_PARAMETERS,
            Self::Aizawa => &AIZAWA_PARAMETERS,
            Self::Halvorsen => &HALVORSEN_PARAMETERS,
            Self::Chen => &CHEN_PARAMETERS,
        }
    }

    pub fn default_parameters(self) -> Vec<f64> {
        self.parameters()
            .iter()
            .map(|parameter| parameter.default)
            .collect()
    }

    pub fn initial_position(self) -> Vec3 {
        match self {
            Self::Lorenz => Vec3::new(1.0, 1.0, 1.0),
            Self::Rossler => Vec3::new(0.1, 0.0, 0.0),
            Self::Thomas => Vec3::new(1.0, 0.1, 0.0),
            Self::Aizawa => Vec3::new(0.1, 0.0, 0.0),
            Self::Halvorsen => Vec3::new(-5.0, 0.0, 0.0),
            Self::Chen => Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn time_step(self) -> f64 {
        match self {
            Self::Lorenz => 0.005,
            Self::Rossler => 0.02,
            Self::Thomas => 0.03,
            Self::Aizawa => 0.008,
            Self::Halvorsen => 0.006,
            Self::Chen => 0.003,
        }
    }

    pub fn center(self) -> Vec3 {
        match self {
            Self::Lorenz => Vec3::new(0.0, 0.0, 25.0),
            Self::Rossler => Vec3::new(0.0, 0.0, 5.0),
            Self::Thomas => Vec3::zero(),
            Self::Aizawa => Vec3::new(0.0, 0.0, 0.5),
            Self::Halvorsen => Vec3::new(-3.0, -3.0, -3.0),
            Self::Chen => Vec3::new(0.0, 0.0, 24.0),
        }
    }

    pub fn view_radius(self) -> f64 {
        match self {
            Self::Lorenz => 32.0,
            Self::Rossler => 18.0,
            Self::Thomas => 5.0,
            Self::Aizawa => 2.0,
            Self::Halvorsen => 15.0,
            Self::Chen => 45.0,
        }
    }

    pub fn derivative(self, position: Vec3, p: &[f64]) -> Vec3 {
        let Vec3 { x, y, z } = position;

        match self {
            Self::Lorenz => {
                let (sigma, rho, beta) = (p[0], p[1], p[2]);

                Vec3::new(sigma * (y - x), x * (rho - z) - y, x * y - beta * z)
            }

            Self::Rossler => {
                let (a, b, c) = (p[0], p[1], p[2]);

                Vec3::new(-y - z, x + a * y, b + z * (x - c))
            }

            Self::Thomas => {
                let b = p[0];

                Vec3::new(y.sin() - b * x, z.sin() - b * y, x.sin() - b * z)
            }

            Self::Aizawa => {
                let (a, b, c, d, e, f) = (p[0], p[1], p[2], p[3], p[4], p[5]);

                Vec3::new(
                    (z - b) * x - d * y,
                    d * x + (z - b) * y,
                    c + a * z - z.powi(3) / 3.0 - (x * x + y * y) * (1.0 + e * z)
                        + f * z * x.powi(3),
                )
            }

            Self::Halvorsen => {
                let a = p[0];

                Vec3::new(
                    -a * x - 4.0 * y - 4.0 * z - y * y,
                    -a * y - 4.0 * z - 4.0 * x - z * z,
                    -a * z - 4.0 * x - 4.0 * y - x * x,
                )
            }

            Self::Chen => {
                let (a, b, c) = (p[0], p[1], p[2]);

                Vec3::new(a * (y - x), (c - a) * x - x * z + c * y, x * y - b * z)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorenz_derivative() {
        let system = Attractor::Lorenz;
        let params = system.default_parameters();

        let result = system.derivative(Vec3::new(1.0, 1.0, 1.0), &params);

        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 26.0);

        assert!((result.z - (1.0 - 8.0 / 3.0)).abs() < 1e-10);
    }

    #[test]
    fn attractor_cycle() {
        assert_eq!(Attractor::Chen.next(), Attractor::Lorenz);
    }
}
