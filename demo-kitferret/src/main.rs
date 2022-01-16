#![no_std]
#![no_main]

mod usb_io;
mod st7735;
mod controller;
mod timer;
mod spi_controller;

use crate::spi_controller::BlockingSpiController;
use crate::controller::Controller;
use crate::st7735::ST7735;
use cortex_m_rt::{entry};
use ferret_rs::GameLoop;
use teensy4_panic as _;
use teensy4_bsp as bsp;
use bsp::hal::gpio::GPIO;
use timer::Timer;

const SCREEN_WIDTH: u16 = 160;
const SCREEN_HEIGHT: u16 = 128;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    usb_io::init().unwrap();

    systick.delay(2000);
    peripherals.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut peripherals.ccm.handle, &mut peripherals.dcdc);

    let pins = bsp::t40::into_pins(peripherals.iomuxc);

    let mut buffer: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT * 2) as usize]
        = [0x00u8; (SCREEN_WIDTH * SCREEN_HEIGHT * 2) as usize];

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::spi::ClockSelect::Pll2,
        bsp::hal::ccm::spi::PrescalarSelect::LPSPI_PODF_0,
    );

    // TODO: use DMA to better performance maybe? https://github.com/mciantyre/teensy4-rs/blob/master/examples/dma_spi.rs
    let mut spi4 = spi4_builder.build(pins.p11, pins.p12, pins.p13);
    spi4.enable_chip_select_0(pins.p10);

    match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(130_000_000)) {
        Ok(()) => {}
        Err(_) => {
            loop {
                core::hint::spin_loop()
            }
        }
    };

    let p9 = GPIO::new(pins.p9).output();
    let p8 = GPIO::new(pins.p8).output();

    let spi_controller = BlockingSpiController::new(spi4);
    let mut st7735 = ST7735::new(spi_controller, p9, p8, true, false, SCREEN_WIDTH, SCREEN_HEIGHT);

    st7735.init(&mut systick);

    // Turn on the Backlight
    GPIO::new(pins.p7).output().set();

    st7735.set_buffer(Some(&mut buffer));
    st7735.set_interlace(true);

    let mut control = Controller::init(
        pins.p0,
        pins.p1,
        pins.p2,
        pins.p3,
        pins.p4,
        pins.p5,
        pins.p6);

    let depth_buffer = &mut [0.0f32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    let mut cfg = peripherals.ccm.perclk.configure(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_1,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let mut gpt1 = peripherals.gpt1.clock(&mut cfg);
    gpt1.set_mode(bsp::hal::gpt::Mode::FreeRunning);
    gpt1.set_enable(true);

    let timer = Timer::new(systick, gpt1);
    let mut game_loop = GameLoop::new(control, st7735, timer, depth_buffer);

    game_loop.start();
}
