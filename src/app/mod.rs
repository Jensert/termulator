mod action;
mod direction;

use crate::app::action::Action;
use crate::app::direction::Direction;
use crate::camera::Camera;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;

pub struct App {
    pub should_quit: bool,
    pub camera: Camera,
}
impl App {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            camera: Camera::default(),
        }
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
            Action::None => (),
        }
    }
}
