use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    widgets::{
        Block, Borders, Paragraph, Wrap,
        canvas::{Canvas, Points},
    },
};

use crate::app::App;

const BACKGROUND: Color = Color::Rgb(8, 14, 23);
const BORDER: Color = Color::Rgb(51, 70, 85);
const ACCENT: Color = Color::Rgb(84, 219, 173);
const MUTED: Color = Color::Rgb(150, 170, 190);

fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(BACKGROUND))
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if area.width < 80 || area.height < 28 {
        let message = Paragraph::new(
            "Resize your terminal to at least 80x28.\n\
             Press Q to quit.",
        )
        .alignment(Alignment::Center)
        .block(panel(" CHAOTICA "));

        frame.render_widget(message, area);
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(area);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Length(29)])
        .split(sections[1]);

    draw_header(frame, sections[0], app);
    draw_canvas(frame, body[0], app);
    draw_sidebar(frame, body[1], app);
    draw_footer(frame, sections[2]);
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        "CHAOTICA v{} | {} Attractor | RK4",
        env!("CARGO_PKG_VERSION"),
        app.simulation.attractor.name(),
    );

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
        .block(panel(""));

    frame.render_widget(header, area);
}

fn draw_canvas(frame: &mut Frame, area: Rect, app: &App) {
    let simulation = &app.simulation;

    let center = simulation.attractor.center();
    let radius = simulation.attractor.view_radius();

    let projected: Vec<(f64, f64)> = simulation
        .trajectory
        .iter()
        .map(|point| app.camera.project(*point, center))
        .collect();

    let width = area.width.saturating_sub(2).max(1);
    let height = area.height.saturating_sub(2).max(1);

    // Approximate terminal character aspect ratio.
    let aspect = (f64::from(width) * 0.5 / f64::from(height)).max(0.1);

    let split = projected.len().saturating_mul(4) / 5;

    let (old_points, recent_points) = projected.split_at(split);

    let head = [app.camera.project(simulation.position, center)];

    let canvas = Canvas::default()
        .block(panel(" PHASE SPACE / 3D "))
        .background_color(BACKGROUND)
        .marker(Marker::Braille)
        .x_bounds([-radius * aspect, radius * aspect])
        .y_bounds([-radius, radius])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: old_points,
                color: Color::Rgb(35, 125, 115),
            });

            ctx.draw(&Points {
                coords: recent_points,
                color: ACCENT,
            });

            ctx.draw(&Points {
                coords: &head,
                color: Color::Yellow,
            });
        });

    frame.render_widget(canvas, area);
}

fn draw_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    let simulation = &app.simulation;

    let status = if simulation.unstable {
        "UNSTABLE"
    } else if app.paused {
        "PAUSED"
    } else {
        "RUNNING"
    };

    let mut info = format!(
        "Status: {}\n\n\
         PARAMETERS\n",
        status,
    );

    for (index, definition) in simulation.attractor.parameters().iter().enumerate() {
        let marker = if index == app.selected_parameter {
            ">"
        } else {
            " "
        };

        info.push_str(&format!(
            "{} {:<7} {:.5}\n",
            marker, definition.name, simulation.parameters[index],
        ));
    }

    info.push_str(&format!(
        "\nPOSITION\n\
         X: {:>11.4}\n\
         Y: {:>11.4}\n\
         Z: {:>11.4}\n\n\
         Time: {:.3}\n\
         Points: {}\n\
         dt: {:.4}\n\
         Steps/frame: {}\n\
         Zoom: {:.2}x\n\
         Auto rotate: {}",
        simulation.position.x,
        simulation.position.y,
        simulation.position.z,
        simulation.time,
        simulation.trajectory.len(),
        simulation.dt,
        app.steps_per_frame,
        app.camera.zoom,
        if app.auto_rotate { "ON" } else { "OFF" },
    ));

    let sidebar = Paragraph::new(info)
        .style(Style::default().fg(MUTED))
        .wrap(Wrap { trim: false })
        .block(panel(" TELEMETRY "));

    frame.render_widget(sidebar, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let controls = "[Tab] System  [Space] Pause  [R] Reset  \
         [A] Rotate  [Q] Quit\n\
         [Arrows] Camera  [+/-] Zoom  \
         [P] Select  [J/K] Edit  [[]/[]] Speed";

    let footer = Paragraph::new(controls)
        .style(Style::default().fg(MUTED))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(panel(" CONTROLS "));

    frame.render_widget(footer, area);
}
