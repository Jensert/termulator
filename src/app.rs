use crate::camera::Camera;
use crate::types::Vec2;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct App {
    pub should_quit: bool,
    pub terminal_size: Vec2,
    pub camera: Camera,
    pub draw_mode: ratatui::symbols::Marker,
    pub render_mode: RenderMode,
}
impl App {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            camera: Camera::default(),
            terminal_size: Vec2 { x: 10.0, y: 10.0 },
            draw_mode: ratatui::symbols::Marker::Braille,
            render_mode: RenderMode::Vertex,
        }
    }

    pub fn get_aspect_ratio(&mut self) -> f64 {
        self.terminal_size.y / self.terminal_size.x * 2.25 // *2.25 to adjust for difference in row and column width / height
    }

    pub fn get_event(&self) -> Result<Option<Event>> {
        if event::poll(core::time::Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn process_event(&self, event: Option<Event>) -> Result<Action> {
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

                    KeyCode::F(n) => match n {
                        1 => Ok(Action::ChangeDrawMode(ratatui::symbols::Marker::Braille)),
                        2 => Ok(Action::ChangeDrawMode(ratatui::symbols::Marker::Dot)),
                        3 => Ok(Action::ChangeDrawMode(ratatui::symbols::Marker::HalfBlock)),
                        4 => Ok(Action::ChangeDrawMode(ratatui::symbols::Marker::Block)),
                        5 => Ok(Action::ChangeDrawMode(ratatui::symbols::Marker::Bar)),

                        8 => Ok(Action::ChangeRenderMode(RenderMode::Vertex)),
                        9 => Ok(Action::ChangeRenderMode(RenderMode::Raycast)),

                        _ => Ok(Action::None),
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
            Some(Event::Resize(x, y)) => Ok(Action::ChangeWindowSize(Vec2 {
                x: x as f64,
                y: y as f64,
            })),
            _ => Ok(Action::None),
        }
    }

    pub fn process_action(&mut self, action: Action) {
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

            Action::ChangeDrawMode(mode) => self.draw_mode = mode,
            Action::ChangeRenderMode(mode) => self.render_mode = mode,

            Action::ChangeWindowSize(size) => {
                self.terminal_size = size;
                self.camera.aspect_ratio = self.get_aspect_ratio()
            }

            Action::None => (),
        }
    }
}

pub enum Action {
    Quit,
    Move(Direction),
    Look(Direction),
    ChangeDrawMode(ratatui::symbols::Marker),
    ChangeRenderMode(RenderMode),
    ChangeWindowSize(Vec2),
    None,
}

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub enum RenderMode {
    Vertex,
    Raycast,
}
