#![no_std]

pub mod color;
pub mod fonts;
pub mod ferret_3d;
pub mod rect;
pub mod screen_trait;

use crate::screen_trait::ScreenTrait;
use crate::ferret_3d::pixel_data::PixelData;
use crate::ferret_3d::triangle_edge_test::TriangleEdgeTest;
use crate::ferret_3d::edge_data::EdgeData;
use crate::ferret_3d::triangle_equations::TriangleEquations;
use crate::ferret_3d::vertex_shader_trait::VertexShaderTrait;
use crate::ferret_3d::pixel_shader_trait::PixelShaderTrait;
use crate::color::Color;

use nalgebra::SVector;

pub use rect::Rect;

const BLOCK_SIZE: usize = 8;
const DEPTH_RANGE_FAR: f32 = 1.0;
const DEPTH_RANGE_NEAR: f32 = 0.0;

pub struct FerretGraphics<'a, TScreen>
where
    TScreen: ScreenTrait
{
    screen: TScreen,
    depth_buffer: &'a mut [f32],
    depth_test: bool
}

impl<'a, TScreen> FerretGraphics<'a, TScreen>
where
    TScreen: ScreenTrait
{
    pub fn new(screen: TScreen, depth_buffer: &'a mut [f32]) -> Self {
        Self {
            screen,
            depth_buffer,
            depth_test: false
        }
    }

    pub fn set_depth_test(&mut self, value: bool) {
        self.depth_test = value;
    }

    pub fn clear_color_buffer(&mut self, color: Color) {
        let rgb = color.as_rgb888();
        self.screen.clear(rgb.0, rgb.1, rgb.2);
    }

    pub fn clear_depth_buffer(&mut self, depth: f32) {
        for x in 0..self.depth_buffer.len() {
            self.depth_buffer[x] = depth;
        }
    }

    pub fn draw_string<'b>
    (
        &mut self,
        mut x: u16,
        mut y: u16,
        text: &'b str,
        font: &[u8],
        font_color: Color,
        bg_color: Option<Color>
    )
    {
        let original_x = x;
        for ch in text.chars() {

            if ch == '\n' {
                x = original_x;
                y += 9;
                continue;
            }

            self.draw_char(x, y, ch, font, font_color, bg_color);
            x += 6;

            if x >= self.screen.get_width() {
                break;
            }
        }
    }

    pub fn draw_char_array(
        &mut self,
        mut x: u16,
        mut y: u16,
        ch_array: &[char],
        start: usize,
        font: &[u8],
        font_color: Color,
        bg_color: Option<Color>
    ) {
        // TODO: can we reduce duplication string -> char_array ?
        let original_x = x;
        for ch_index in start..ch_array.len() {
            let ch = ch_array[ch_index];
            if ch == '\n' {
                x = original_x;
                y += 9;
                continue;
            }

            self.draw_char(x, y, ch, font, font_color, bg_color);
            x += 6;

            if x >= self.screen.get_width() {
                break;
            }
        }
    }

    pub fn draw_char(
        &mut self,
        x: u16,
        y: u16,
        c: char,
        font: &[u8],
        font_color: Color,
        bg_color: Option<Color>
    ) {
        let font_color = font_color.as_rgb888();
        let mut mask: u8 = 0x01;

        if let Some(bg_color) = bg_color {
            let bg_color = bg_color.as_rgb888();
            // solid bg
            let mut color: (u8, u8, u8);

            for yc in 0..8 {

                // If out of bounds, stop
                if yc + y >= self.screen.get_height() {
                    break;
                }

                for xc in 0..5 {

                    // If out of bounds, stop
                    if xc + x >=  self.screen.get_width() {
                        break;
                    }

                    if font[c as usize * 5 + xc as usize] & mask != 0 {
                        color = font_color;
                    } else {
                        color = bg_color;
                    }

                    self.screen.set_pixel(
                        x + xc as u16,
                        y + yc as u16,
                        color.0,
                        color.1,
                        color.2
                    );
                }
                mask = mask << 1;
            }
        } else {
            // transparent bg

            for yc in 0..8 {
                for xc in 0..5 {
                    if font[c as usize * 5 + xc] & mask == 0 {
                        continue
                    }

                    self.screen.set_pixel(
                        x + xc as u16,
                        y + yc as u16,
                        font_color.0,
                        font_color.1,
                        font_color.2
                    );
                }
                mask = mask << 1;
            }
        }
    }

    pub fn draw_mesh
        <TVertexShader: VertexShaderTrait<TVECTOR_IN_SIZE, TVECTOR_OUT_SIZE>,
        TPixelShader: PixelShaderTrait<TVECTOR_OUT_SIZE>,
        const TVECTOR_IN_SIZE: usize,
        const TVECTOR_OUT_SIZE: usize>(
            &mut self,
            vertices: &[SVector<f32, TVECTOR_IN_SIZE>],
            indexes: &[(usize, usize, usize)],
            vertex_shader: &TVertexShader,
            pixel_shader: &TPixelShader
        ) {
            for x in 0..indexes.len() {
                let i = indexes[x];

                // TODO: reduce duplicated vertex processing
                let v0 = vertex_shader.process(&vertices[i.0]);
                let v1 = vertex_shader.process(&vertices[i.1]);
                let v2 = vertex_shader.process(&vertices[i.2]);

                if self.does_triangle_clip_completely(v0, v1, v2) {
                     continue;
                } else {
                    let (v0, v1, v2) = self.transform_triangle(v0, v1, v2);

                    self.draw_triangle(
                        pixel_shader,
                        &v0,
                        &v1,
                        &v2);
                }
            }
    }

    fn does_triangle_clip_completely <const TVECTOR_SIZE: usize>
    (
        &self,
        v0: SVector<f32, TVECTOR_SIZE>,
        v1: SVector<f32, TVECTOR_SIZE>,
        v2: SVector<f32, TVECTOR_SIZE>
    ) -> bool
    {
        let x0 = v0[0];
        let y0 = v0[1];
        let z0 = v0[2];
        let w0 = v0[3];

        let x1 = v1[0];
        let y1 = v1[1];
        let z1 = v1[2];
        let w1 = v1[3];

        let x2 = v2[0];
        let y2 = v2[1];
        let z2 = v2[2];
        let w2 = v2[3];

        (w0 < x0 && w1 < x1 && w2 < x2) // Pos X clip
        || (-w0 > x0 && -w1 > x1 && -w2 > x2) // Neg X clip

        || (w0 < y0 && w1 < y1 && w2 < y2) // Pos Y clip
        || (-w0 > y0 && -w1 > y1 && -w2 > y2) // Neg Y clip

        || (w0 < z0 && w1 < z1 && w2 < z2) // Pos Z clip
        || (-w0 > z0 && -w1 > z1 && -w2 > z2) // Neg Z clip
    }

    fn transform_triangle<const TVECTOR_SIZE: usize>
    (
        &mut self,
        v0: SVector<f32, TVECTOR_SIZE>,
        v1: SVector<f32, TVECTOR_SIZE>,
        v2: SVector<f32, TVECTOR_SIZE>
    ) -> (SVector<f32, TVECTOR_SIZE>, SVector<f32, TVECTOR_SIZE>, SVector<f32, TVECTOR_SIZE>)
    {
        let v0 = self.transform_vertex(v0);
        let v1 = self.transform_vertex(v1);
        let v2 = self.transform_vertex(v2);

        (v0, v1, v2)
    }

    fn transform_vertex<const TVECTOR_SIZE: usize>(&self, v: SVector<f32, TVECTOR_SIZE>) -> SVector<f32, TVECTOR_SIZE> {
        let mut transformed_vector = SVector::<f32, TVECTOR_SIZE>::zeros();
        let inv_w = 1.0 / v[3];

        let original_x = v[0];
        let original_y = v[1];
        let original_z = v[2];

        let mut x = original_x * inv_w;
        let mut y = original_y * inv_w;
        let mut z = original_z * inv_w;

        let px = self.screen.get_width() as f32 / 2.0;
        let ox = 0.0 + px;

        let py = self.screen.get_height() as f32 / 2.0;
        let oy = 0.0 + py;

        x = px * x + ox;
        y = py * y + oy;
        z = 0.5 * (DEPTH_RANGE_FAR - DEPTH_RANGE_NEAR) * z + 0.5 * (DEPTH_RANGE_NEAR + DEPTH_RANGE_FAR);

        transformed_vector[0] = x;
        transformed_vector[1] = y;
        transformed_vector[2] = z;

        for index in 3..transformed_vector.len() {
            transformed_vector[index] = v[index];
        }

        transformed_vector
    }

    pub fn present(&mut self) {
        self.screen.update_screen().unwrap();
    }

    fn draw_triangle<TPixelShader: PixelShaderTrait<VECTOR_SIZE>, const VECTOR_SIZE: usize>(
        &mut self,
        pixel_shader: &TPixelShader,
        v0: &SVector<f32, VECTOR_SIZE>,
        v1: &SVector<f32, VECTOR_SIZE>,
        v2: &SVector<f32, VECTOR_SIZE>,
    ) {
        let triangle = TriangleEquations::new(v2, v1, v0);

        if triangle.area2 <= 0.0 {
            return
        }

        let min_x = v0[0].min(v1[0]).min(v2[0]) as usize;
        let max_x = v0[0].max(v1[0]).max(v2[0]) as usize;
        let min_y = v0[1].min(v1[1]).min(v2[1]) as usize;
        let max_y = v0[1].max(v1[1]).max(v2[1]) as usize;

        let min_x = min_x.max(0);
        let max_x = max_x.min(self.screen.get_width() as usize);
        let min_y = min_y.max(0);
        let max_y = max_y.min(self.screen.get_height() as usize);

        let step_size = BLOCK_SIZE - 1;

        let xm = max_x as usize;
        let ym = max_y as usize;

        let step_size = step_size as f32;

        for x in (min_x..xm).step_by(BLOCK_SIZE) {
            for y in (min_y..ym).step_by(BLOCK_SIZE) {
                let mut edge00 = EdgeData::new(&triangle, x as f32 + 1.0, y as f32 + 1.0);

                let mut edge01 = edge00.clone();
                edge01.step_y(&triangle, step_size);

                let mut edge10 = edge00.clone();
                edge10.step_x(&triangle, step_size);

                let mut edge11 = edge01.clone();
                edge11.step_x(&triangle, step_size);

                let e00 = TriangleEdgeTest::new(&triangle, &edge00);
                let e01 = TriangleEdgeTest::new(&triangle, &edge01);
                let e10 = TriangleEdgeTest::new(&triangle, &edge10);
                let e11 = TriangleEdgeTest::new(&triangle, &edge11);

                let e00_all_true = e00.all_true();
                let e01_all_true = e01.all_true();
                let e10_all_true = e10.all_true();
                let e11_all_true = e11.all_true();

                let all_test_false =
                    !e00_all_true
                    && !e01_all_true
                    && !e10_all_true
                    && !e11_all_true;

                let test_edges;

                if all_test_false {
                    test_edges =
                    !e00.all_same()
                    || !e01.all_same()
                    || !e10.all_same()
                    || !e11.all_same();
                } else {
                    let are_all_true =
                    e00_all_true
                    && e01_all_true
                    && e10_all_true
                    && e11_all_true;

                    test_edges = !are_all_true;
                }

                if test_edges {
                    self.draw_block::<TPixelShader, VECTOR_SIZE, true>(pixel_shader, &triangle, &mut edge00, x, y, max_x, max_y);
                } else {
                    self.draw_block::<TPixelShader, VECTOR_SIZE, false>(pixel_shader, &triangle, &mut edge00, x, y, max_x, max_y);
                }
            }
        }
    }

    fn draw_block<TPixelShader: PixelShaderTrait<VECTOR_SIZE>, const VECTOR_SIZE: usize, const TEST_EDGES: bool>(
        &mut self,
        pixel_shader: &TPixelShader,
        triangle: &TriangleEquations<VECTOR_SIZE>,
        edge: &mut EdgeData<VECTOR_SIZE>,
        x: usize,
        y: usize,
        max_x: usize,
        max_y: usize
    ) {
        let mut pixel_out = PixelData::new(&triangle, edge.x, edge.y);

        for y_pos in y..y + BLOCK_SIZE {
            if y_pos >= max_y {
                break;
            }

            let mut pixel_in = pixel_out.clone();
            let mut edge_in = if TEST_EDGES {
                edge.clone()
            } else {
                EdgeData::empty()
            };

            for x_pos in x..x + BLOCK_SIZE {
                if x_pos >= max_x {
                    break;
                }

                let d = 1.0/pixel_in.data[3];
                if (!TEST_EDGES || edge_in.test(&triangle)) && self.depth_test(x_pos as u32, y_pos as u32, d) {

                    let pixel_value = pixel_shader.process(&pixel_in);

                    let r = (pixel_value.x * 255.0) as u8;
                    let g = (pixel_value.y * 255.0) as u8;
                    let b = (pixel_value.z * 255.0) as u8;

                    self.screen.set_pixel(x_pos as u16, y_pos as u16,  r, g, b);
                    self.set_depth_value(x_pos as u32, y_pos as u32, d);
                }

                pixel_in.step_x(&triangle);

                if TEST_EDGES {
                    edge_in.step_x_by_one(&triangle);
                }
            }

            pixel_out.step_y(&triangle);

            if TEST_EDGES {
                edge.step_y_by_one(&triangle);
            }
        }
    }

    fn depth_test(&self, x: u32, y: u32, z: f32) -> bool {
        !self.depth_test || z > self.get_depth_value(x, y)
    }

    fn get_depth_value(&self, x: u32, y: u32) -> f32 {
        let index = y * self.screen.get_width() as u32 + x;
        self.depth_buffer[index as usize]
    }

    fn set_depth_value(&mut self, x: u32, y: u32, value: f32) {
        let index = y * self.screen.get_width() as u32 + x;
        self.depth_buffer[index as usize] = value;
    }
}