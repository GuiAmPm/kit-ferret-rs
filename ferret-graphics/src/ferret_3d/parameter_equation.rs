use crate::ferret_3d::edge_equation::EdgeEquation;

#[derive(Copy, Clone)]
pub struct ParameterEquation<const TPARAMETER_COUNT: usize> {
    pub a: f32,
    pub b: f32,
    pub c: f32
}

impl<const TPARAMETER_COUNT: usize> ParameterEquation<TPARAMETER_COUNT> {
    pub fn new(v0: f32,
        v1: f32,
        v2: f32,
        e0: &EdgeEquation<TPARAMETER_COUNT>,
        e1: &EdgeEquation<TPARAMETER_COUNT>,
        e2: &EdgeEquation<TPARAMETER_COUNT>,
        factor: f32
    ) -> Self {
        let a = factor * (v2 * e0.a + v0 * e1.a + v1 * e2.a);
        let b = factor * (v2 * e0.b + v0 * e1.b + v1 * e2.b);
        let c = factor * (v2 * e0.c + v0 * e1.c + v1 * e2.c);

        Self {
            a, b, c
        }
    }

    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.a * x + self.b * y + self.c
    }

    pub fn step_x(&self, v: f32, step_size: f32) -> f32 {
        v + self.a * step_size
    }

    pub fn step_y(&self, v: f32, step_size: f32) -> f32 {
        v + self.b * step_size
    }
}