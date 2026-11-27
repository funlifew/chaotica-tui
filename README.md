# CHAOTICA

An interactive three-dimensional chaos theory simulator
running entirely inside your terminal, written in Rust.

## Features

- Six chaotic dynamical systems
- Lorenz, Rossler, Thomas, Aizawa, Halvorsen and Chen
- Fourth-order Runge-Kutta numerical integration
- Real-time three-dimensional trajectory visualization
- Unicode Braille rendering
- Interactive orthographic camera
- Adjustable simulation parameters
- Real-time simulation telemetry
- Bounded trajectory history
- Three interactive color themes: Aurora, Nebula and Solar
- Responsive terminal dashboard
- Automatic trajectory fitting
- Connected trajectory rendering
- Age-based trajectory color gradients
- Toggleable viewport grid
- Focus mode for immersive visualization
- Liang-Barsky line clipping
- Color-coded real-time telemetry
- Highlighted parameter selection

## Installation

```bash
git clone https://github.com/funlifew/chaotica-tui.git
cd chaotica-tui
cargo run --release
```

## Controls

| Key | Action |
| --- | --- |
| Tab | Switch attractor |
| Space | Pause or resume |
| Arrow keys | Rotate camera |
| + / - | Zoom |
| A | Toggle automatic rotation |
| R | Reset simulation |
| C | Reset camera |
| D | Restore default parameters |
| P | Select parameter |
| J / K | Adjust parameter |
| [ / ] | Adjust simulation speed |
| T | Cycle color themes |
| G | Toggle viewport grid |
| F | Toggle focus mode |
| M | Toggle automatic viewport fitting |
| Q / Esc | Quit |

## Mathematics

CHAOTICA integrates autonomous ordinary differential
equations using the classical fourth-order Runge-Kutta
method.

The three-dimensional trajectories are projected
onto a two-dimensional terminal canvas using
orthographic projection.

## Technology

- Rust
- Ratatui
- Crossterm

## Development

```bash
cargo fmt
cargo check
cargo clippy --all-targets
cargo test
```

## Limitations

This is an educational numerical simulator.

Chaotic trajectories are sensitive to initial conditions,
numerical precision and integration step sizes.

Not all parameter combinations produce bounded
or chaotic trajectories.

## License

MIT
