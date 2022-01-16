use nalgebra::SVector;

pub trait VertexShaderTrait<const TVERTEX_IN_SIZE: usize, const TVERTEX_OUT_SIZE: usize> {
    fn process(&self, v: &SVector<f32, TVERTEX_IN_SIZE>) -> SVector<f32, TVERTEX_OUT_SIZE>;
}
