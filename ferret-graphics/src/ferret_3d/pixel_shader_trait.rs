use nalgebra::Vector3;

use crate::ferret_3d::pixel_data::PixelData;


pub trait PixelShaderTrait<const TVERTEX_INPUT_SIZE: usize> {
    fn process(&self, pixel_data: &PixelData<TVERTEX_INPUT_SIZE>) -> Vector3<f32>;
}