#![no_std]

pub mod system;

use nalgebra::SVector;
use ferret_utils::convert::float_to_string;
use ferret_graphics::ferret_3d::pixel_data::PixelData;
use ferret_graphics::fonts::glcd::GLCD_FONT;
use nalgebra::Vector3;
use nalgebra::Vector4;
use nalgebra::Matrix4;
use ferret_graphics::ferret_3d::vertex_shader_trait::VertexShaderTrait;
use nalgebra::Vector6;
use ferret_graphics::ferret_3d::pixel_shader_trait::PixelShaderTrait;
use ferret_graphics::color::Color;
use ferret_graphics::FerretGraphics;
use ferret_utils::convert::integer_to_string;

use crate::system::ControllerTrait;
use crate::system::ControllerButton;
use crate::system::ButtonState;
use crate::system::ScreenTrait;
use crate::system::TimerTrait;

struct SimpleVertexShader {
    pub model_view_matrix: Matrix4<f32>,
}

impl SimpleVertexShader {
    pub fn new() -> Self {
        Self {
            model_view_matrix: Matrix4::identity()
        }
    }
}

impl VertexShaderTrait<6, 7> for SimpleVertexShader {
    fn process(&self, vec: &Vector6<f32>) -> SVector<f32, 7> {
        let vec4 = Vector4::new(vec.x, vec.y, vec.z, 1.0);
        let vec4_ = self.model_view_matrix * vec4;
        let x = vec4_.x;
        let y = vec4_.y;
        let z = vec4_.z;
        let w = vec4_.w;
        return nalgebra::vector!(x, y, z, w, vec[3], vec[4], vec[5]);
    }
}

struct SimplePixelShader<'a> {
    pub texture: Option<&'a [u8]>,
    pub tex_wid: u16,
    pub tex_hei: u16
}

impl<'a> SimplePixelShader<'a> {
    pub fn new() -> Self {
        Self {
            texture: None,
            tex_wid: 0,
            tex_hei: 0
        }
    }
}

impl<'a> PixelShaderTrait<7> for SimplePixelShader<'a> {

    fn process(&self, v: &PixelData<7>) -> Vector3<f32> {
        if let Some(texture) = self.texture {
            let x = (self.tex_wid as f32 * v.data[4]) as usize;
            let y = (self.tex_hei as f32 * v.data[5]) as usize;

            let x = x.min(self.tex_wid as usize).max(0);
            let y = y.min(self.tex_hei as usize).max(0);

            let index = ((y * (self.tex_wid as usize) + x) * 3).min(texture.len() - 3).max(0);
            let r = texture[index + 0] as f32;
            let g = texture[index + 1] as f32;
            let b = texture[index + 2] as f32;

            Vector3::new(r / 255.0, g / 255.0, b / 255.0)
        } else {
            Vector3::new(v.data[2], v.data[3], v.data[2])
        }
    }
}

pub struct GameLoop<'a, TController, TScreen, TTimer>
where TController: ControllerTrait, TScreen: ScreenTrait
{
    controller: TController,
    graphics: FerretGraphics<'a, TScreen>,
    timer: TTimer
}

impl<'a, TController, TScreen, TTimer> GameLoop<'a, TController, TScreen, TTimer>
where TController: ControllerTrait,
    TScreen: ScreenTrait,
    TTimer: TimerTrait
{
    pub fn new(
        controller: TController,
        screen: TScreen,
        timer: TTimer,
        depth_buffer: &'a mut [f32]
    ) -> Self {
        let graphics = FerretGraphics::new(screen, depth_buffer);

        Self {
            controller,
            graphics,
            timer
        }
    }

    pub fn start(&mut self) -> ! {
        let vertices_0 = [
            Vector6::new(-1.0, -1.0, -1.0, 0.0, 0.0, 0.0), // 0
            Vector6::new(1.0, -1.0, -1.0, 1.0, 0.0, 0.0), // 1
            Vector6::new(-1.0, 1.0, -1.0, 0.0, 1.0, 0.0), // 2
            Vector6::new(1.0, 1.0, -1.0, 1.0, 1.0, 0.0), // 3
            Vector6::new(1.0, -1.0, 1.0, 0.0, 0.0, 1.0), // 4
            Vector6::new(1.0, 1.0, 1.0, 1.0, 0.0, 1.0), // 5
            Vector6::new(-1.0, -1.0, 1.0, 0.0, 1.0, 1.0), // 6
            Vector6::new(-1.0, 1.0, 1.0, 1.0, 1.0, 1.0), // 7

            Vector6::new(-10.0, 10.0, 100.0, 0.0, 0.0, 1.0), // 8
            Vector6::new(10.0, 10.0, 100.0, 1.0, 0.0, 1.0), // 9
            Vector6::new(-10.0, 0.0, 0.0, 0.0, 1.0, 1.0), // 10
            Vector6::new(10.0, 0.0, 0.0, 1.0, 1.0, 1.0), // 11
        ];

        let indexes = [(0, 1, 2), (2, 1, 3),
                        (1, 4, 3), (3, 4, 5),
                        (4, 6, 5), (5, 6, 7),
                        (6, 0, 7), (7, 0, 2),
                        (0, 4, 1), (4, 0, 6),
                        (2, 3, 5), (5, 7, 2),
                        // (8, 9, 10), (10, 9, 8),
        ];

        let texture = include_bytes!("../assets/box.raw");

        let mut vertex_shader = SimpleVertexShader::new();
        let mut pixel_shader = SimplePixelShader::new();

        pixel_shader.texture = Some(texture);
        pixel_shader.tex_wid = 128;
        pixel_shader.tex_hei = 128;

        let mut index = 0;

        let mut rot_x = 0.0f32;
        let mut rot_y = 0.0f32;
        let mut t_x = 0.0;
        let mut t_y = 0.0;
        let mut t_z = 3.0;
        let mut s =  1.0;
        let mut auto_rotate = false;

        let view_matrix = Matrix4::<f32>::identity();

        let fovy = 3.1419 * 90.0 / 180.0;
        let aspect = 160.0/128.0;
        let near = 2.0;
        let far = 100.0;

        let projection_matrix =
            nalgebra::Perspective3::new(aspect, fovy, near, far).into_inner()
            * Matrix4::new_nonuniform_scaling(&nalgebra::vector!(1.0, -1.0, -1.0));

        let mut period = 0;

        self.graphics.set_depth_test(true);

        loop {
            period = self.timer.measure(|| {
                self.controller.update();

                if index == 0 {
                    index = 1;
                } else {
                    index = 0;
                }

                self.graphics.clear_color_buffer(Color (0.0, 0.0, 0.0));

                if self.controller.get_button_status(ControllerButton::R).is_down() {
                    if self.controller.get_button_status(ControllerButton::B).is_down() {
                        rot_y += 0.1;
                    } else if self.controller.get_button_status(ControllerButton::D).is_down() {
                        rot_y -= 0.1;
                    }

                    if self.controller.get_button_status(ControllerButton::A).is_down() {
                        rot_x += 0.1;
                    } else if self.controller.get_button_status(ControllerButton::C).is_down() {
                        rot_x -= 0.1;
                    }

                    if self.controller.get_button_status(ControllerButton::Up).is_down() {
                        t_z += 0.1;
                    } else if self.controller.get_button_status(ControllerButton::Down).is_down() {
                        t_z -= 0.1;
                    }
                } else {
                    if self.controller.get_button_status(ControllerButton::Right).is_down() {
                        t_x += 0.1;
                    } else if self.controller.get_button_status(ControllerButton::Left).is_down() {
                        t_x -= 0.1;
                    }

                    if self.controller.get_button_status(ControllerButton::Up).is_down() {
                        t_y += 0.1;
                    } else if self.controller.get_button_status(ControllerButton::Down).is_down() {
                        t_y -= 0.1;
                    }
                }

                if self.controller.get_button_status(ControllerButton::Start).is_down() {
                    s = 1.0;
                    t_x = 0.0;
                    t_y = 0.0;
                    t_z = 0.0;
                    rot_x = 0.0;
                    rot_y = 0.0;
                }

                if self.controller.get_button_status(ControllerButton::Select) == ButtonState::Pressed {
                    auto_rotate = !auto_rotate;
                }

                if auto_rotate {
                    rot_x += 0.025;
                    rot_y += 0.0125;
                }

                self.graphics.clear_depth_buffer(0.0);

                for x in 0..2 {
                    let model_matrix =
                        Matrix4::<f32>::new_translation(&Vector3::new(t_x, t_y, t_z))
                        * Matrix4::<f32>::from_euler_angles(rot_x, rot_y , 0.0)
                        * Matrix4::<f32>::new_translation(&Vector3::new(x as f32 * 0.5, x as f32 * 0.5, x as f32 * 0.01));

                    let mvp_matrix = projection_matrix * (view_matrix * model_matrix);
                    vertex_shader.model_view_matrix = mvp_matrix;

                    self.graphics.draw_mesh(&vertices_0, &indexes, &vertex_shader, &pixel_shader);
                }

                self.graphics.draw_string(
                    10,
                    10,
                    "This is a test\nsecond line ",
                    &GLCD_FONT,
                    Color(1.0, 1.0, 1.0),
                    None);

                let mut string_buffer = ['\0';20];

                integer_to_string(period, &mut string_buffer, 3);

                self.graphics.draw_char_array(
                    10,
                    27,
                    &string_buffer,
                    0,
                    &GLCD_FONT,
                    Color(1.0, 0.0, 0.0),
                    None
                );

                float_to_string(t_z, &mut string_buffer, 10);
                self.graphics.draw_char_array(10, 100, &string_buffer, 0, &GLCD_FONT, Color(1.0, 1.0, 1.0), None);

                if self.controller.get_button_status(ControllerButton::Start).is_down() {
                    self.graphics.draw_string(
                        10,
                        50,
                        "START",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::Select).is_down() {
                    self.graphics.draw_string(
                        50,
                        50,
                        "SELECT",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::R).is_down() {
                    self.graphics.draw_string(
                        100,
                        50,
                        "R",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::L).is_down() {
                    self.graphics.draw_string(
                        110,
                        50,
                        "L",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::A).is_down() {
                    self.graphics.draw_string(
                        10,
                        60,
                        "A",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::B).is_down() {
                    self.graphics.draw_string(
                        20,
                        60,
                        "B",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::C).is_down() {
                    self.graphics.draw_string(
                        30,
                        60,
                        "C",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::D).is_down() {
                    self.graphics.draw_string(
                        40,
                        60,
                        "D",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::Up).is_down() {
                    self.graphics.draw_string(
                        10,
                        70,
                        "^",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::Right).is_down() {
                    self.graphics.draw_string(
                        20,
                        70,
                        ">",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::Down).is_down() {
                    self.graphics.draw_string(
                        30,
                        70,
                        "V",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                if self.controller.get_button_status(ControllerButton::Left).is_down() {
                    self.graphics.draw_string(
                        40,
                        70,
                        "<",
                        &GLCD_FONT,
                        Color(1.0, 1.0, 1.0),
                        None);
                }

                self.graphics.present();
            });

            if period < 16 {
                self.timer.delay(16 - period as u32);
            }
        }
    }
}