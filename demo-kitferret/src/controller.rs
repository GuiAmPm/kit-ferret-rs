use ferret_rs::system::system_traits::ControllerButton;
use ferret_rs::system::ControllerTrait;
use ferret_rs::system::system_traits::ButtonState;

use teensy4_bsp as bsp;
use bsp::hal::iomuxc::{ gpio::Pin, * };
use bsp::hal::gpio;
use bsp::hal::gpio::{ GPIO };

pub struct Controller<Ps0, Ps1, Ps2, Pi0, Pi1, Pi2, Pi3>
where Ps0: Pin, Ps1: Pin, Ps2: Pin, Pi0: Pin, Pi1: Pin, Pi2: Pin, Pi3: Pin
{
    pin_selector_0: GPIO<Ps0, gpio::Output>,
    pin_selector_1: GPIO<Ps1, gpio::Output>,
    pin_selector_2: GPIO<Ps2, gpio::Output>,
    pin_input_0: GPIO<Pi0, gpio::Input>,
    pin_input_1: GPIO<Pi1, gpio::Input>,
    pin_input_2: GPIO<Pi2, gpio::Input>,
    pin_input_3: GPIO<Pi3, gpio::Input>,

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

impl<Ps0, Ps1, Ps2, Pi0, Pi1, Pi2, Pi3> Controller<Ps0, Ps1, Ps2, Pi0, Pi1, Pi2, Pi3>
where Ps0: Pin, Ps1: Pin, Ps2: Pin, Pi0: Pin, Pi1: Pin, Pi2: Pin, Pi3: Pin {
    pub fn init(
        pin_selector_0: Ps0,
        pin_selector_1: Ps1,
        pin_selector_2: Ps2,
        mut pin_input_0: Pi0,
        mut pin_input_1: Pi1,
        mut pin_input_2: Pi2,
        mut pin_input_3: Pi3,
    ) -> Self {
        const PULL_DOWN: Config = Config::zero()
            .set_hysteresis(Hysteresis::Enabled)
            .set_pull_keep(PullKeep::Enabled)
            .set_pull_keep_select(PullKeepSelect::Pull)
            .set_pullupdown(PullUpDown::Pulldown100k);

        configure(&mut pin_input_0, PULL_DOWN);
        configure(&mut pin_input_1, PULL_DOWN);
        configure(&mut pin_input_2, PULL_DOWN);
        configure(&mut pin_input_3, PULL_DOWN);

        let selector0 = GPIO::new(pin_selector_0);
        let selector1 = GPIO::new(pin_selector_1);
        let selector2 = GPIO::new(pin_selector_2);

        let input0 = GPIO::new(pin_input_0);
        let input1 = GPIO::new(pin_input_1);
        let input2 = GPIO::new(pin_input_2);
        let input3 = GPIO::new(pin_input_3);

        Self {
            pin_selector_0: selector0.output(),
            pin_selector_1: selector1.output(),
            pin_selector_2: selector2.output(),
            pin_input_0: input0,
            pin_input_1: input1,
            pin_input_2: input2,
            pin_input_3: input3,

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
            d: ButtonState::Idle
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

    fn update_internal(&mut self) {
        self.pin_selector_0.set();

        self.up = Self::update_state(&self.up, self.pin_input_0.is_set());
        self.right = Self::update_state(&self.right, self.pin_input_1.is_set());
        self.down = Self::update_state(&self.down, self.pin_input_2.is_set());
        self.left = Self::update_state(&self.left, self.pin_input_3.is_set());

        self.pin_selector_0.clear();

        self.pin_selector_1.set();

        self.a = Self::update_state(&self.a, self.pin_input_0.is_set());
        self.b = Self::update_state(&self.b, self.pin_input_1.is_set());
        self.c = Self::update_state(&self.c, self.pin_input_2.is_set());
        self.d = Self::update_state(&self.d, self.pin_input_3.is_set());

        self.pin_selector_1.clear();

        self.pin_selector_2.set();

        self.select = Self::update_state(&self.select, self.pin_input_0.is_set());
        self.start = Self::update_state(&self.start, self.pin_input_1.is_set());
        self.l = Self::update_state(&self.l, self.pin_input_2.is_set());
        self.r = Self::update_state(&self.r, self.pin_input_3.is_set());

        self.pin_selector_2.clear();
    }
}

impl<Ps0, Ps1, Ps2, Pi0, Pi1, Pi2, Pi3> ControllerTrait for Controller<Ps0, Ps1, Ps2, Pi0, Pi1, Pi2, Pi3>
where Ps0: Pin,
    Ps1: Pin,
    Ps2: Pin,
    Pi0: Pin,
    Pi1: Pin,
    Pi2: Pin,
    Pi3: Pin
{
    fn update(&mut self) {
        self.update_internal();
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