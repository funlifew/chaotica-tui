use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    physics::{attractor::Attractor, simulation::Simulation},
    render::camera::Camera,
};

pub struct App {
    pub simulation: Simulation,
    pub camera: Camera,
    pub running: bool,
    pub paused: bool,
    pub auto_rotate: bool,
    pub steps_per_frame: usize,
    pub selected_parameter: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            simulation: Simulation::new(Attractor::Lorenz),
            camera: Camera::new(),
            running: true,
            paused: false,
            auto_rotate: false,
            steps_per_frame: 20,
            selected_parameter: 0,
        }
    }

    pub fn update(&mut self) {
        if !self.paused {
            self.simulation.update(self.steps_per_frame);

            if self.simulation.unstable {
                self.paused = true;
            }
        }

        if self.auto_rotate {
            self.camera.rotate(0.006, 0.0);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.running = false;
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }

            KeyCode::Char(' ') => {
                if !self.simulation.unstable {
                    self.paused = !self.paused;
                }
            }

            KeyCode::Tab => {
                self.simulation.switch_attractor();

                self.selected_parameter = 0;
                self.camera.reset();
                self.paused = false;
            }

            KeyCode::Char('r') => {
                self.simulation.reset();
                self.paused = false;
            }

            KeyCode::Char('d') => {
                self.simulation.restore_parameters();
                self.paused = false;
            }

            KeyCode::Char('c') => {
                self.camera.reset();
            }

            KeyCode::Char('a') => {
                self.auto_rotate = !self.auto_rotate;
            }

            KeyCode::Left => {
                self.camera.rotate(-0.08, 0.0);
            }

            KeyCode::Right => {
                self.camera.rotate(0.08, 0.0);
            }

            KeyCode::Up => {
                self.camera.rotate(0.0, 0.08);
            }

            KeyCode::Down => {
                self.camera.rotate(0.0, -0.08);
            }

            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.camera.zoom_in();
            }

            KeyCode::Char('-') => {
                self.camera.zoom_out();
            }

            KeyCode::Char(']') => {
                self.steps_per_frame = (self.steps_per_frame + 5).min(200);
            }

            KeyCode::Char('[') => {
                self.steps_per_frame = self.steps_per_frame.saturating_sub(5).max(1);
            }

            KeyCode::Char('p') => {
                let count = self.simulation.parameters.len();

                self.selected_parameter = (self.selected_parameter + 1) % count;
            }

            KeyCode::Char('j') => {
                self.simulation
                    .adjust_parameter(self.selected_parameter, -1.0);

                self.paused = false;
            }

            KeyCode::Char('k') => {
                self.simulation
                    .adjust_parameter(self.selected_parameter, 1.0);

                self.paused = false;
            }

            _ => {}
        }
    }
}
