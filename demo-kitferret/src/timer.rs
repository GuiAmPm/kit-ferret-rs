use ferret_rs::system::TimerTrait;
use teensy4_bsp::{SysTick, hal::gpt::GPT};

pub struct Timer {
    systick: SysTick,
    system_timer: GPT
}

impl Timer {
    pub fn new(systick: SysTick, system_timer: GPT) -> Self {
        Self {
            systick,
            system_timer
        }
    }
}

impl TimerTrait for Timer {
    fn delay(&mut self, millis: u32) {
        self.systick.delay(millis)
    }

    fn measure<F>(&self, act: F) -> u128
    where F: FnOnce()
    {
        let (_, period) = self.system_timer.time(act);
        period.as_millis()
    }
}