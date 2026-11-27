use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Paragraph,
        canvas::{Canvas, Line as CanvasLine, Points},
    },
};

use crate::{app::App, physics::simulation::MAX_TRAIL_LENGTH};

const BACKGROUND: Color = Color::Rgb(7, 11, 19);
const SURFACE: Color = Color::Rgb(15, 23, 36);
const BORDER: Color = Color::Rgb(51, 65, 85);
const GRID: Color = Color::Rgb(24, 37, 51);

const TEXT: Color = Color::Rgb(226, 232, 240);
const MUTED: Color = Color::Rgb(148, 163, 184);

const SUCCESS: Color = Color::Rgb(74, 222, 128);
const WARNING: Color = Color::Rgb(250, 204, 21);
const ERROR: Color = Color::Rgb(248, 113, 113);

const HEAD: Color = Color::Rgb(255, 255, 255);

#[derive(Debug, Clone, Copy)]
struct Viewport {
    x: [f64; 2],
    y: [f64; 2],
}

impl Viewport {
    fn new(points: &[(f64, f64)], radius: f64, aspect: f64, zoom: f64, auto_fit: bool) -> Self {
        let aspect = aspect.max(0.1);

        let mut center_x = 0.0;
        let mut center_y = 0.0;

        let mut half_width = radius;
        let mut half_height = radius;

        if auto_fit && points.len() >= 300 {
            let mut min_x = f64::INFINITY;
            let mut max_x = f64::NEG_INFINITY;

            let mut min_y = f64::INFINITY;
            let mut max_y = f64::NEG_INFINITY;

            for &(x, y) in points {
                min_x = min_x.min(x);
                max_x = max_x.max(x);

                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }

            center_x = (min_x + max_x) * 0.5;
            center_y = (min_y + max_y) * 0.5;

            let minimum = radius * 0.12;

            half_width = ((max_x - min_x) * 0.5).max(minimum);

            half_height = ((max_y - min_y) * 0.5).max(minimum);
        }

        let padding = 1.12;

        let scale = half_height.max(half_width / aspect) * padding / zoom;

        let width = scale * aspect;
        let height = scale;

        Self {
            x: [center_x - width, center_x + width],
            y: [center_y - height, center_y + height],
        }
    }

    fn width(self) -> f64 {
        self.x[1] - self.x[0]
    }

    fn height(self) -> f64 {
        self.y[1] - self.y[0]
    }
}

#[derive(Debug, Clone, Copy)]
struct Palette {
    name: &'static str,
    accent: Color,
    selection: Color,
    trail: [Color; 4],
}

const PALETTES: [Palette; 3] = [
    Palette {
        name: "Aurora",
        accent: Color::Rgb(94, 234, 212),
        selection: Color::Rgb(20, 65, 65),
        trail: [
            Color::Rgb(35, 100, 110),
            Color::Rgb(45, 160, 155),
            Color::Rgb(85, 230, 195),
            Color::Rgb(190, 255, 230),
        ],
    },
    Palette {
        name: "Nebula",
        accent: Color::Rgb(192, 132, 252),
        selection: Color::Rgb(61, 39, 89),
        trail: [
            Color::Rgb(80, 60, 130),
            Color::Rgb(138, 98, 204),
            Color::Rgb(199, 156, 255),
            Color::Rgb(242, 218, 255),
        ],
    },
    Palette {
        name: "Solar",
        accent: Color::Rgb(251, 191, 36),
        selection: Color::Rgb(85, 53, 20),
        trail: [
            Color::Rgb(135, 67, 35),
            Color::Rgb(200, 121, 53),
            Color::Rgb(255, 187, 88),
            Color::Rgb(255, 240, 171),
        ],
    },
];

fn palette(app: &App) -> Palette {
    PALETTES[app.theme_index % PALETTES.len()]
}

fn panel(title: &str, accent: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .title_style(Style::default().fg(accent).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(SURFACE))
}

fn metric(label: &str, value: String, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:<10}", label), Style::default().fg(MUTED)),
        Span::styled(
            value,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ])
}

fn shortcut(key: &'static str, label: &'static str, accent: Color) -> Vec<Span<'static>> {
    vec![
        Span::styled(
            format!("[{}]", key),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!(" {}  ", label), Style::default().fg(MUTED)),
    ]
}

fn status(app: &App) -> (&'static str, Color) {
    if app.simulation.unstable {
        ("UNSTABLE", ERROR)
    } else if app.paused {
        ("PAUSED", WARNING)
    } else {
        ("RUNNING", SUCCESS)
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let theme = palette(app);

    frame.render_widget(
        Block::default().style(Style::default().bg(BACKGROUND)),
        area,
    );

    if area.width < 56 || area.height < 20 {
        draw_resize_message(frame, area, theme);
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(6),
        ])
        .split(area);

    draw_header(frame, sections[0], app, theme);
    draw_footer(frame, sections[2], theme);

    let compact = area.width < 100 || area.height < 34;

    if app.focus_mode || compact {
        draw_canvas(frame, sections[1], app, theme);
    } else {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(30), Constraint::Length(32)])
            .split(sections[1]);

        draw_canvas(frame, body[0], app, theme);
        draw_sidebar(frame, body[1], app, theme);
    }
}

fn draw_resize_message(frame: &mut Frame, area: Rect, theme: Palette) {
    let message = Paragraph::new(vec![
        Line::from(""),
        Line::styled(
            "CHAOTICA",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::styled(
            "Resize terminal to at least 56x20.",
            Style::default().fg(MUTED),
        ),
        Line::from(""),
        Line::from("Press Q to exit."),
    ])
    .alignment(Alignment::Center)
    .block(panel(" VIEWPORT ", theme.accent));

    frame.render_widget(message, area);
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let simulation = &app.simulation;
    let (state, state_color) = status(app);

    let selected = &simulation.attractor.parameters()[app.selected_parameter];

    let selected_value = simulation.parameters[app.selected_parameter];

    let lines = vec![
        Line::from(vec![
            Span::styled(
                "CHAOTICA",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" / {} / ", simulation.attractor.name(),),
                Style::default().fg(TEXT),
            ),
            Span::styled(
                format!("● {}", state),
                Style::default()
                    .fg(state_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" / {}", theme.name), Style::default().fg(MUTED)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("t: {:.2}", simulation.time),
                Style::default().fg(TEXT),
            ),
            Span::styled(
                format!("  |  {}: {:.4}", selected.name, selected_value,),
                Style::default().fg(theme.accent),
            ),
            Span::styled(
                format!("  |  pts: {}", simulation.trajectory.len(),),
                Style::default().fg(MUTED),
            ),
        ]),
    ];

    let header = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(panel(" CHAOS LAB ", theme.accent));

    frame.render_widget(header, area);
}

fn draw_canvas(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let simulation = &app.simulation;

    let title = if app.auto_fit {
        " PHASE SPACE / AUTO FIT "
    } else {
        " PHASE SPACE / MANUAL "
    };

    let block = panel(title, theme.accent).style(Style::default().bg(BACKGROUND));

    let inner = block.inner(area);

    let aspect = (f64::from(inner.width.max(1)) * 0.5 / f64::from(inner.height.max(1))).max(0.1);

    // Keep projection and viewport zoom separate.
    let camera = crate::render::camera::Camera {
        zoom: 1.0,
        ..app.camera
    };

    let center = simulation.attractor.center();

    let projected: Vec<(f64, f64)> = simulation
        .trajectory
        .iter()
        .map(|&point| camera.project(point, center))
        .collect();

    let viewport = Viewport::new(
        &projected,
        simulation.attractor.view_radius(),
        aspect,
        app.camera.zoom,
        app.auto_fit,
    );

    let canvas = Canvas::default()
        .block(block)
        .background_color(BACKGROUND)
        .marker(Marker::Braille)
        .x_bounds(viewport.x)
        .y_bounds(viewport.y)
        .paint(|ctx| {
            if app.show_grid {
                let divisions = 8;

                for i in 1..divisions {
                    let fraction = i as f64 / divisions as f64;

                    let x = viewport.x[0] + viewport.width() * fraction;

                    let y = viewport.y[0] + viewport.height() * fraction;

                    ctx.draw(&CanvasLine {
                        x1: x,
                        y1: viewport.y[0],
                        x2: x,
                        y2: viewport.y[1],
                        color: GRID,
                    });

                    ctx.draw(&CanvasLine {
                        x1: viewport.x[0],
                        y1: y,
                        x2: viewport.x[1],
                        y2: y,
                        color: GRID,
                    });
                }

                ctx.layer();
            }

            let count = projected.len();

            for (index, segment) in projected.windows(2).enumerate() {
                let color_index = (index * theme.trail.len() / count).min(theme.trail.len() - 1);

                let color = theme.trail[color_index];

                if let Some((start, end)) = clip_segment(segment[0], segment[1], viewport) {
                    ctx.draw(&CanvasLine {
                        x1: start.0,
                        y1: start.1,
                        x2: end.0,
                        y2: end.1,
                        color,
                    });
                }
            }

            if let Some(&head) = projected.last() {
                ctx.draw(&Points {
                    coords: &[head],
                    color: HEAD,
                });
            }
        });

    frame.render_widget(canvas, area);
}

fn draw_sidebar(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let parameter_height = app.simulation.parameters.len() as u16 + 2;

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(parameter_height),
            Constraint::Length(8),
            Constraint::Min(7),
        ])
        .split(area);

    draw_parameters(frame, sections[0], app, theme);

    draw_telemetry(frame, sections[1], app, theme);

    draw_camera_info(frame, sections[2], app, theme);
}

fn draw_parameters(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let simulation = &app.simulation;

    let mut lines = Vec::new();

    for (index, definition) in simulation.attractor.parameters().iter().enumerate() {
        let selected = index == app.selected_parameter;

        let value = simulation.parameters[index];

        let marker = if selected { ">" } else { " " };

        let content = format!("{} {:<8} {:>12.5}", marker, definition.name, value,);

        let style = if selected {
            Style::default()
                .fg(theme.accent)
                .bg(theme.selection)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(TEXT)
        };

        lines.push(Line::styled(content, style));
    }

    let widget = Paragraph::new(lines).block(panel(" PARAMETERS ", theme.accent));

    frame.render_widget(widget, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, theme: Palette) {
    let accent = theme.accent;

    let row1 = [
        shortcut("Tab", "System", accent),
        shortcut("Space", "Pause", accent),
        shortcut("Q", "Quit", accent),
    ]
    .concat();

    let row2 = [
        shortcut("Arrows", "Camera", accent),
        shortcut("+/-", "Zoom", accent),
        shortcut("A", "Rotate", accent),
    ]
    .concat();

    let row3 = [
        shortcut("T", "Theme", accent),
        shortcut("G", "Grid", accent),
        shortcut("F", "Focus", accent),
        shortcut("M", "Fit", accent),
    ]
    .concat();

    let row4 = [
        shortcut("P", "Param", accent),
        shortcut("J/K", "Edit", accent),
        shortcut("R/D/C", "Reset", accent),
        shortcut("[/]", "Speed", accent),
    ]
    .concat();

    let lines = vec![
        Line::from(row1),
        Line::from(row2),
        Line::from(row3),
        Line::from(row4),
    ];

    let footer = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(panel(" KEYBOARD CONTROLS ", theme.accent));

    frame.render_widget(footer, area);
}

fn draw_telemetry(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let simulation = &app.simulation;

    let lines = vec![
        metric(
            "X",
            format!("{:+.5}", simulation.position.x),
            Color::Rgb(248, 113, 113),
        ),
        metric(
            "Y",
            format!("{:+.5}", simulation.position.y),
            Color::Rgb(74, 222, 128),
        ),
        metric(
            "Z",
            format!("{:+.5}", simulation.position.z),
            Color::Rgb(96, 165, 250),
        ),
        metric("Time", format!("{:.3}", simulation.time), TEXT),
        metric("dt", format!("{:.4}", simulation.dt), TEXT),
        metric(
            "Trail",
            format!("{}/{}", simulation.trajectory.len(), MAX_TRAIL_LENGTH,),
            theme.accent,
        ),
    ];

    let widget = Paragraph::new(lines).block(panel(" TELEMETRY ", theme.accent));

    frame.render_widget(widget, area);
}

fn draw_camera_info(frame: &mut Frame, area: Rect, app: &App, theme: Palette) {
    let lines = vec![
        metric("Zoom", format!("{:.2}x", app.camera.zoom), TEXT),
        metric(
            "Grid",
            if app.show_grid {
                "ON".to_string()
            } else {
                "OFF".to_string()
            },
            if app.show_grid { SUCCESS } else { MUTED },
        ),
        metric(
            "Auto fit",
            if app.auto_fit {
                "ON".to_string()
            } else {
                "OFF".to_string()
            },
            if app.auto_fit { SUCCESS } else { MUTED },
        ),
        metric(
            "Speed",
            format!("{} steps/f", app.steps_per_frame),
            theme.accent,
        ),
        metric(
            "Rotation",
            if app.auto_rotate {
                "AUTO".to_string()
            } else {
                "MANUAL".to_string()
            },
            TEXT,
        ),
    ];

    let widget = Paragraph::new(lines).block(panel(" VIEW / CAMERA ", theme.accent));

    frame.render_widget(widget, area);
}

fn clip_segment(
    start: (f64, f64),
    end: (f64, f64),
    view: Viewport,
) -> Option<((f64, f64), (f64, f64))> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;

    let mut enter: f64 = 0.0;
    let mut exit: f64 = 1.0;

    let boundaries = [
        (-dx, start.0 - view.x[0]),
        (dx, view.x[1] - start.0),
        (-dy, start.1 - view.y[0]),
        (dy, view.y[1] - start.1),
    ];

    for (p, q) in boundaries {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }

            continue;
        }

        let t = q / p;

        if p < 0.0 {
            enter = enter.max(t);
        } else {
            exit = exit.min(t);
        }

        if enter > exit {
            return None;
        }
    }

    Some((
        (start.0 + enter * dx, start.1 + enter * dy),
        (start.0 + exit * dx, start.1 + exit * dy),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_preserves_aspect_ratio() {
        let viewport = Viewport::new(&[], 10.0, 2.0, 1.0, false);

        let aspect = viewport.width() / viewport.height();

        assert!((aspect - 2.0).abs() < 1e-10);
    }

    #[test]
    fn zoom_reduces_viewport_size() {
        let normal = Viewport::new(&[], 10.0, 1.0, 1.0, false);

        let zoomed = Viewport::new(&[], 10.0, 1.0, 2.0, false);

        assert!((zoomed.width() - normal.width() / 2.0).abs() < 1e-10);
    }

    #[test]
    fn segment_inside_viewport() {
        let viewport = Viewport {
            x: [-10.0, 10.0],
            y: [-10.0, 10.0],
        };

        let result = clip_segment((-2.0, 0.0), (2.0, 0.0), viewport);

        assert_eq!(result, Some(((-2.0, 0.0), (2.0, 0.0))),);
    }

    #[test]
    fn segment_crosses_viewport() {
        let viewport = Viewport {
            x: [-10.0, 10.0],
            y: [-10.0, 10.0],
        };

        let result = clip_segment((-20.0, 0.0), (20.0, 0.0), viewport).unwrap();

        assert!((result.0.0 + 10.0).abs() < 1e-10);
        assert!((result.1.0 - 10.0).abs() < 1e-10);
    }

    #[test]
    fn invisible_segment_is_rejected() {
        let viewport = Viewport {
            x: [-10.0, 10.0],
            y: [-10.0, 10.0],
        };

        assert!(clip_segment((20.0, 20.0), (30.0, 30.0), viewport,).is_none());
    }

    #[test]
    fn auto_fit_centers_trajectory() {
        let points = vec![(5.0, 8.0); 300];

        let viewport = Viewport::new(&points, 10.0, 1.0, 1.0, true);

        let center_x = (viewport.x[0] + viewport.x[1]) / 2.0;

        let center_y = (viewport.y[0] + viewport.y[1]) / 2.0;

        assert!((center_x - 5.0).abs() < 1e-10);
        assert!((center_y - 8.0).abs() < 1e-10);
    }
}
