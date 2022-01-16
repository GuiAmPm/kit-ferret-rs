use std::thread;
use std::time;
use std::time::Duration;

use ferret_rs::system::TimerTrait;
use sdl2::render::Texture;
use ferret_rs::system::ScreenTrait;
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use ferret_rs::system::system_traits::ControllerButton;
use ferret_rs::system::system_traits::ButtonState;
use ferret_rs::system::ControllerTrait;

pub struct SDL2Controller {
    event_pump: EventPump,

    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,

    pub select: ButtonState,
    pub start: ButtonState,
    pub l: ButtonState,
    pub r: ButtonState,

    pub a: ButtonState,
    pub b: ButtonState,
    pub c: ButtonState,
    pub d: ButtonState,
}

impl SDL2Controller {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,

            up: ButtonState::Idle,
            down: ButtonState::Idle,
            left: ButtonState::Idle,
            right: ButtonState::Idle,

            select: ButtonState::Idle,
            start: ButtonState::Idle,
            l: ButtonState::Idle,
            r: ButtonState::Idle,

            a: ButtonState::Idle,
            b: ButtonState::Idle,
            c: ButtonState::Idle,
            d: ButtonState::Idle,
        }
    }

    fn update_state(state: &ButtonState, pressed: bool) -> ButtonState {
        match state {
            ButtonState::Idle | ButtonState::Released =>
                if pressed { ButtonState::Pressed } else { ButtonState::Idle }

            ButtonState::Pressed | ButtonState::Held =>
                if pressed { ButtonState::Held } else { ButtonState::Released }
        }
    }
}

impl ControllerTrait for SDL2Controller {
    fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => {
                            self.up = Self::update_state(&self.up, true);
                        },
                        Keycode::A => {
                            self.left = Self::update_state(&self.left, true);
                        },
                        Keycode::S => {
                            self.down = Self::update_state(&self.down, true);
                        },
                        Keycode::D => {
                            self.right = Self::update_state(&self.right, true);
                        },
                        Keycode::Tab => {
                            self.select = Self::update_state(&self.select, true);
                        },
                        Keycode::Return => {
                            self.start = Self::update_state(&self.start, true);
                        },
                        Keycode::I => {
                            self.a = Self::update_state(&self.a, true);
                        },
                        Keycode::J => {
                            self.d = Self::update_state(&self.d, true);
                        },
                        Keycode::K => {
                            self.c = Self::update_state(&self.c, true);
                        },
                        Keycode::L => {
                            self.b = Self::update_state(&self.b, true);
                        },
                        Keycode::Q => {
                            self.l = Self::update_state(&self.l, true);
                        },
                        Keycode::O => {
                            self.r = Self::update_state(&self.r, true);
                        }
                        _ => ()
                    }
                },

                Event::KeyUp { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::W => {
                            self.up = Self::update_state(&self.up, false);
                        },
                        Keycode::A => {
                            self.left = Self::update_state(&self.left, false);
                        },
                        Keycode::S => {
                            self.down = Self::update_state(&self.down, false);
                        },
                        Keycode::D => {
                            self.right = Self::update_state(&self.right, false);
                        },
                        Keycode::Tab => {
                            self.select = Self::update_state(&self.select, false);
                        },
                        Keycode::Return => {
                            self.start = Self::update_state(&self.start, false);
                        },
                        Keycode::I => {
                            self.a = Self::update_state(&self.a, false);
                        },
                        Keycode::J => {
                            self.d = Self::update_state(&self.d, false);
                        },
                        Keycode::K => {
                            self.c = Self::update_state(&self.c, false);
                        },
                        Keycode::L => {
                            self.b = Self::update_state(&self.b, false);
                        },
                        Keycode::Q => {
                            self.l = Self::update_state(&self.l, false);
                        },
                        Keycode::O => {
                            self.r = Self::update_state(&self.r, false);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    fn get_button_status(&self, button: ControllerButton) -> ButtonState {
        match button {
            // direction buttons
            ControllerButton::Up => self.up,
            ControllerButton::Right => self.right,
            ControllerButton::Down => self.down,
            ControllerButton::Left => self.left,

            // action buttons
            ControllerButton::A => self.a,
            ControllerButton::B => self.b,
            ControllerButton::C => self.c,
            ControllerButton::D => self.d,

            // extra buttons
            ControllerButton::Select => self.select,
            ControllerButton::Start => self.start,
            ControllerButton::L => self.l,
            ControllerButton::R => self.r,
        }
    }
}

pub struct SDL2Screen<'a> {
    canvas: Canvas<sdl2::video::Window>,
    screen_buffer: Texture,
    width: u16,
    height: u16,
    color_buffer: &'a mut [u8]
}

impl<'a> SDL2Screen<'a> {
    pub fn new(canvas: Canvas<sdl2::video::Window>, width: u16, height: u16, color_buffer: &'a mut [u8]) -> Self {
        let screen_buffer = canvas.create_texture(
            PixelFormatEnum::RGBA32,
            TextureAccess::Static,
            width as u32,
            height as u32
          ).unwrap();

        Self {
            canvas,
            screen_buffer,
            width,
            height,
            color_buffer
        }
    }
}

impl<'a> ScreenTrait for SDL2Screen<'a> {
    fn get_width(&self) -> u16 { self.width }
    fn get_height(&self) -> u16 { self.height }

    fn set_pixel(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8) {
        let index = (y * self.get_width() + x) as usize;

        self.color_buffer[index * 4 + 0] = r;
        self.color_buffer[index * 4 + 1] = g;
        self.color_buffer[index * 4 + 2] = b;
        self.color_buffer[index * 4 + 3] = 255;
    }

    fn clear(&mut self, r: u8, g: u8, b: u8) {
        for index in (0..self.color_buffer.len()).step_by(4) {
            self.color_buffer[index + 0] = r;
            self.color_buffer[index + 1] = g;
            self.color_buffer[index + 2] = b;
            self.color_buffer[index + 3] = 255;
        }
    }

    fn update_screen(&mut self) -> std::result::Result<(), ()> {
        self.screen_buffer.update(
            None,
            &self.color_buffer,
            (4 * self.get_width()) as usize
        ).unwrap();

        self.canvas.copy(&self.screen_buffer, None, None).unwrap();
        self.canvas.present();

        Ok(())
    }
}

pub struct SDL2Timer {
}

impl SDL2Timer {
    pub fn new() -> Self {
        Self {}
    }
}

impl TimerTrait for SDL2Timer {
    fn delay(&mut self, millis: u32) {
        thread::sleep(Duration::from_millis(millis as u64));
    }


    fn measure<F>(&self, act: F) -> u128
    where F: FnOnce()
    {
        let before = time::Instant::now();
        act();
        before.elapsed().as_millis()
    }
}