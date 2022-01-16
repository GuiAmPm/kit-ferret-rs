#[derive(PartialEq, Eq)]
pub enum InputState {
    Idle,
    Released,
    Pressed,
    Held
}

impl InputState {
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == InputState::Pressed || *self == InputState::Held
    }

    #[inline(always)]
    pub fn is_up(&self) ->  bool {
        !self.is_down()
    }
}