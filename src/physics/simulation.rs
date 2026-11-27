use std::collections::VecDeque;

use crate::math::{rk4::rk4_step, vector::Vec3};

use super::attractor::Attractor;

pub const MAX_TRAIL_LENGTH: usize = 15_000;

pub struct Simulation {
    pub attractor: Attractor,
    pub position: Vec3,
    pub trajectory: VecDeque<Vec3>,
    pub parameters: Vec<f64>,
    pub time: f64,
    pub dt: f64,
    pub unstable: bool,
}

impl Simulation {
    pub fn new(attractor: Attractor) -> Self {
        let position = attractor.initial_position();

        let mut trajectory = VecDeque::with_capacity(MAX_TRAIL_LENGTH);

        trajectory.push_back(position);

        Self {
            attractor,
            position,
            trajectory,
            parameters: attractor.default_parameters(),
            time: 0.0,
            dt: attractor.time_step(),
            unstable: false,
        }
    }

    pub fn step(&mut self) {
        if self.unstable {
            return;
        }

        let attractor = self.attractor;
        let parameters = &self.parameters;

        let next_position = rk4_step(self.position, self.dt, |position| {
            attractor.derivative(position, parameters)
        });

        if !next_position.is_finite() || next_position.magnitude() > 1_000_000.0 {
            self.unstable = true;
            return;
        }

        self.position = next_position;
        self.time += self.dt;

        if self.trajectory.len() >= MAX_TRAIL_LENGTH {
            self.trajectory.pop_front();
        }

        self.trajectory.push_back(self.position);
    }

    pub fn update(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();

            if self.unstable {
                break;
            }
        }
    }

    pub fn reset(&mut self) {
        self.position = self.attractor.initial_position();

        self.time = 0.0;
        self.unstable = false;
        self.trajectory.clear();
        self.trajectory.push_back(self.position);
    }

    pub fn restore_parameters(&mut self) {
        self.parameters = self.attractor.default_parameters();

        self.reset();
    }

    pub fn switch_attractor(&mut self) {
        *self = Self::new(self.attractor.next());
    }

    pub fn adjust_parameter(&mut self, index: usize, direction: f64) {
        let definitions = self.attractor.parameters();

        if let Some(definitions) = definitions.get(index) {
            let new_value = self.parameters[index] + definitions.step * direction;

            self.parameters[index] = new_value.clamp(-100.0, 100.0);

            self.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_advances_time() {
        let mut simulation = Simulation::new(Attractor::Lorenz);

        simulation.update(100);

        assert!((simulation.time - 0.5).abs() < 1e-10);
    }

    #[test]
    fn simulation_reset() {
        let mut simulation = Simulation::new(Attractor::Lorenz);

        simulation.update(100);
        simulation.reset();

        assert_eq!(simulation.time, 0.0);
        assert_eq!(simulation.trajectory.len(), 1);
    }

    #[test]
    fn default_systems_remain_finite() {
        for attractor in Attractor::ALL {
            let mut simulation = Simulation::new(attractor);

            simulation.update(4000);

            assert!(!simulation.unstable, "{} became unstable", attractor.name(),);
        }
    }

    #[test]
    fn trajectory_is_bounded() {
        let mut simulation = Simulation::new(Attractor::Lorenz);

        simulation.update(MAX_TRAIL_LENGTH + 100);

        assert_eq!(simulation.trajectory.len(), MAX_TRAIL_LENGTH);
    }
}
