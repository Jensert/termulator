use color_eyre::Result;

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    style::Color,
    widgets::{
        Paragraph,
        canvas::{Canvas, Line},
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
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug)]
struct Camera {
    pos: Vec3,
    fov: f64,

    yaw: f64,
    pitch: f64,

    move_speed: f64,
    rotate_speed: f64,
}
impl Camera {
    fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -1.0),
            fov: 90.0,

            yaw: 0.0,
            pitch: 0.0,

            move_speed: 0.1,
            rotate_speed: 5.0,
        }
    }

    fn forward(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        Vec3 {
            x: pitch_rad.cos() * yaw_rad.sin(),
            y: pitch_rad.sin(),
            z: pitch_rad.cos() * yaw_rad.cos(),
        }
    }

    fn right(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        Vec3 {
            x: yaw_rad.cos(),
            y: 0.0,
            z: -yaw_rad.sin(),
        }
    }
    fn forward_movement(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();

        Vec3 {
            x: yaw_rad.sin(),
            y: 0.0,
            z: yaw_rad.cos(),
        }
    }

    fn project_vertex(&self, vertex: &Vec3) -> (f64, f64) {
        // first apply view transformation
        let view_space = self.apply_view_transform(*vertex - self.pos);

        // we dont want to project points behind the camera
        if view_space.z <= 0.0 {
            return (10.0, 10.0); // Place points behind camera off-screen
        }

        // Now do perspective projection
        let scale = (self.fov / 2.0).to_radians().tan();
        let aspect_ratio = 9.0 / 16.0;
        let x = (view_space.x / (scale * view_space.z)) * aspect_ratio;
        let y = view_space.y / (scale * view_space.z);
        (x, y)
    }

    fn apply_view_transform(&self, point: Vec3) -> Vec3 {
        // create rotation matrices for yaw and pitch
        let yaw_rad = -self.yaw.to_radians();
        let pitch_rad = -self.pitch.to_radians();

        // First rotate around Y axis (yaw)
        let mut result = Vec3 {
            x: point.x * yaw_rad.cos() + point.z * yaw_rad.sin(),
            y: point.y,
            z: -point.x * yaw_rad.sin() + point.z * yaw_rad.cos(),
        };

        // Then rotate around X axis (pitch)
        result = Vec3 {
            x: result.x,
            y: result.y * pitch_rad.cos() - result.z * pitch_rad.sin(),
            z: result.y * pitch_rad.sin() + result.z * pitch_rad.cos(),
        };

        result
    }
}

struct App {
    should_quit: bool,
    camera: Camera,
}
impl App {
    fn default() -> Self {
        Self {
            should_quit: false,
            camera: Camera::default(),
        }
    }

    fn get_event(&self) -> Result<Option<Event>> {
        if event::poll(core::time::Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    fn process_event(&self, event: Option<Event>) -> Result<Action> {
        match event {
            Some(Event::FocusGained) => Ok(Action::None),
            Some(Event::FocusLost) => Ok(Action::None),
            Some(Event::Key(event)) => {
                if !event.is_press() {
                    return Ok(Action::None);
                }

                match event.code {
                    KeyCode::Char(c) => match c {
                        'Q' => Ok(Action::Quit), // Quit app

                        'a' => Ok(Action::Move(Direction::Left)),
                        'd' => Ok(Action::Move(Direction::Right)),
                        'w' => Ok(Action::Move(Direction::Forward)),
                        's' => Ok(Action::Move(Direction::Backward)),
                        ' ' => Ok(Action::Move(Direction::Up)),
                        'k' => Ok(Action::Move(Direction::Up)),
                        'j' => Ok(Action::Move(Direction::Down)),
                        _ => {
                            println!("{:?}", event);
                            Ok(Action::None)
                        }
                    },

                    KeyCode::Left => Ok(Action::Look(Direction::Left)),
                    KeyCode::Right => Ok(Action::Look(Direction::Right)),
                    KeyCode::Up => Ok(Action::Look(Direction::Up)),
                    KeyCode::Down => Ok(Action::Look(Direction::Down)),
                    _ => {
                        println!("{:?}", event);
                        Ok(Action::None)
                    }
                }
            }
            Some(Event::Mouse(_event)) => Ok(Action::None),
            Some(Event::Paste(_string)) => Ok(Action::None),
            Some(Event::Resize(_x, _y)) => Ok(Action::None),
            _ => Ok(Action::None),
        }
    }

    fn process_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Move(direction) => match direction {
                Direction::Forward => {
                    self.camera.pos =
                        self.camera.pos + self.camera.forward_movement() * self.camera.move_speed
                }
                Direction::Backward => {
                    self.camera.pos =
                        self.camera.pos - self.camera.forward_movement() * self.camera.move_speed
                }
                Direction::Left => {
                    self.camera.pos = self.camera.pos - self.camera.right() * self.camera.move_speed
                }
                Direction::Right => {
                    self.camera.pos = self.camera.pos + self.camera.right() * self.camera.move_speed
                }
                Direction::Up => {
                    self.camera.pos.y = self.camera.pos.y + self.camera.move_speed;
                }
                Direction::Down => {
                    self.camera.pos.y = self.camera.pos.y - self.camera.move_speed;
                }
            },
            Action::Look(direction) => match direction {
                Direction::Up => {
                    self.camera.pitch += self.camera.rotate_speed;
                    self.camera.pitch = self.camera.pitch.clamp(-89.0, 89.0);
                }
                Direction::Down => {
                    self.camera.pitch -= self.camera.rotate_speed;
                    self.camera.pitch = self.camera.pitch.clamp(-89.0, 89.0);
                }
                Direction::Left => {
                    self.camera.yaw -= self.camera.rotate_speed;
                }
                Direction::Right => {
                    self.camera.yaw += self.camera.rotate_speed;
                }
                _ => (), // Skip forward and backward
            },
            Action::None => (),
        }
    }
}

enum Action {
    Quit,
    Move(Direction),
    Look(Direction),
    None,
}
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    std::io::stdout()
        .execute(crossterm::event::EnableMouseCapture)
        .unwrap();
    let terminal = ratatui::init();
    let app = App::default();

    let result = run(app, terminal);

    ratatui::restore();

    result
}

fn run(mut app: App, mut terminal: DefaultTerminal) -> Result<()> {
    let cube_vertices = [
        Vec3::new(-0.5, -0.5, 3.5),
        Vec3::new(0.5, -0.5, 3.5),
        Vec3::new(0.5, 0.5, 3.5),
        Vec3::new(-0.5, 0.5, 3.5),
        Vec3::new(-0.5, -0.5, 2.5),
        Vec3::new(0.5, -0.5, 2.5),
        Vec3::new(0.5, 0.5, 2.5),
        Vec3::new(-0.5, 0.5, 2.5),
    ];

    let cube_edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let canvas = Canvas::default()
                .x_bounds([-1.0, 1.0])
                .y_bounds([-1.0, 1.0])
                .marker(ratatui::symbols::Marker::Braille)
                .background_color(Color::Blue)
                .paint(|ctx| {
                    for &(start_idx, end_idx) in &cube_edges {
                        let start = cube_vertices[start_idx] - app.camera.pos;
                        let end = cube_vertices[end_idx] - app.camera.pos;

                        let (x1, y1) = app.camera.project_vertex(&start);
                        let (x2, y2) = app.camera.project_vertex(&end);
                        let color = Color::White;
                        ctx.draw(&Line {
                            x1,
                            y1,
                            x2,
                            y2,
                            color,
                        });
                    }
                });

            let debug_info = Paragraph::new(format!(
                "pos: {:?} \nyaw: {:.1}, \npitch: {:.1}",
                app.camera.pos, app.camera.yaw, app.camera.pitch
            ));

            frame.render_widget(canvas, area);
            frame.render_widget(debug_info, area);
        })?;

        let event = app.get_event()?;
        let action = app.process_event(event)?;
        app.process_action(action);

        if app.should_quit {
            return Ok(());
        }
    }
}
