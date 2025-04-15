use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    style::Color,
    widgets::{
        Paragraph,
        canvas::{self, Canvas, Circle, Line},
    },
};
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug)]
struct Camera {
    pos: Vec3,
    fov: f64,
}
impl Camera {
    fn new(pos: Vec3, fov: f64) -> Self {
        Self { pos, fov }
    }

    fn default() -> Self {
        Self {
            pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            fov: 90.0,
        }
    }
    fn project_vertex(&self, vertex: &Vec3) -> (f64, f64) {
        // Simple perspective divide
        let scale = self.fov.tan();
        let x = (vertex.x / (scale * vertex.z)) * (1920.0 / 1080.0);
        let y = (vertex.y / scale) / vertex.z;
        (x, y)
    }
}

fn main() -> Result<()> {
    let triangle: [Vec3; 3] = [
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
    ];

    let point = Vec3::new(0.0, 0.0, 0.0);

    let mut camera = Camera::default();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &point, &mut camera);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, vertex: &Vec3, camera: &mut Camera) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let vertex_relative = *vertex - camera.pos;
            let vertex_projected = camera.project_vertex(&vertex_relative);
            let area = frame.area();
            let canvas = Canvas::default()
                .x_bounds([-1.0, 1.0])
                .y_bounds([-1.0, 1.0])
                .marker(ratatui::symbols::Marker::Braille)
                .background_color(Color::Blue)
                .paint(|ctx| {
                    ctx.draw(&Circle {
                        x: vertex_projected.0,
                        y: vertex_projected.1,
                        radius: 0.1,
                        color: Color::Red,
                    });
                });

            let debug_info = Paragraph::new(format!("{:?}, {:?}", camera.pos, vertex_projected));

            frame.render_widget(canvas, area);
            frame.render_widget(debug_info, area);
        })?;
        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })
        ) {
            break Ok(());
        }

        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                ..
            })
        ) {
            camera.pos.x -= 0.1;
        }

        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                ..
            })
        ) {
            camera.pos.x += 0.1;
        }

        if matches!(
            event::read()?,
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            })
        ) {
            camera.pos.z -= 0.1;
        }
    }
}
