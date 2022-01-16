use crate::ferret_3d::triangle_equations::TriangleEquations;

#[derive(Clone)]
pub struct PixelData<const TPARAMETER_COUNT: usize> {
    pub data: [f32; TPARAMETER_COUNT]
}

impl<const TPARAMETER_COUNT: usize> PixelData<TPARAMETER_COUNT> {
    pub fn new(tri_eq: &TriangleEquations<TPARAMETER_COUNT>, x: f32, y: f32) -> Self {

        #[allow(deprecated)] // TODO: find another way to create uninitialized array
        let mut data: [f32; TPARAMETER_COUNT] = unsafe { core::mem::uninitialized() };

        for index in 0..TPARAMETER_COUNT {
            data[index] = tri_eq.a_var[index].evaluate(x, y)
        }

        Self {
            data
        }
    }

    pub fn step_x(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>) {
        for index in 0..TPARAMETER_COUNT {
            self.data[index] = tri_eq.a_var[index].step_x(self.data[index], 1.0);
        }
    }

    pub fn step_y(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>) {
        for index in 0..TPARAMETER_COUNT {
            self.data[index] = tri_eq.a_var[index].step_y(self.data[index], 1.0);
        }
    }
}