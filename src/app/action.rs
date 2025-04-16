use crate::app::Direction;
pub enum Action {
    Quit,
    Move(Direction),
    Look(Direction),
    None,
}
