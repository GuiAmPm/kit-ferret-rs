#[derive(Copy, Clone)]
pub enum ControllerButton {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    Select = 8,
    Start = 9,
    L = 10,
    R = 11
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Idle = 0,
    Released = 1,
    Pressed = 2,
    Held = 3
}

impl ButtonState {
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == ButtonState::Pressed
            || *self == ButtonState::Held
    }

    #[inline(always)]
    pub fn is_up(&self) ->  bool {
        !self.is_down()
    }
}

pub trait ControllerTrait {
    fn update(&mut self);
    fn get_button_status(&self, button: ControllerButton) -> ButtonState;
}

pub trait TimerTrait {
    fn delay(&mut self, millis: u32);
    fn measure<F>(&self, act: F) -> u128 where F: FnOnce();
}