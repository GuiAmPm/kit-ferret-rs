use ferret_rs::GameLoop;

mod sdl2_interface;

use sdl2_interface::SDL2Controller;
use sdl2_interface::SDL2Screen;
use sdl2_interface::SDL2Timer;

pub fn main() -> ! {
    let sdl_context = sdl2::init().expect("Failed to create SDL2 context.");
    let video_subsystem = sdl_context.video().expect("Failed to create video subsystem.");

    let window = video_subsystem
        .window("Ferret - Desktop Demo", 160 * 5, 128 * 5)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().expect("Failed to create canvas.");
    let event_pump = sdl_context.event_pump().expect("Failed to create event pump");

    let controller = SDL2Controller::new(event_pump);
    let mut color_buffer = [0u8; 160 * 128 * 4];
    let screen = SDL2Screen::new(canvas, 160, 128, &mut color_buffer);
    let timer = SDL2Timer::new();

    let depth_buffer = &mut [0.0f32; 160 * 128];

    let mut game_loop =
        GameLoop::new(
            controller,
            screen,
            timer,
            depth_buffer
        );

    game_loop.start();
}