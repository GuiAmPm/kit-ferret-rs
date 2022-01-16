use nalgebra::SVector;

pub struct EdgeEquation<const TPARAMETER_COUNT: usize> {
    pub a: f32,
    pub b: f32,
    pub c: f32,

    pub tie: bool
}

impl<const TPARAMETER_COUNT: usize> EdgeEquation<TPARAMETER_COUNT> {
    pub fn new(v0: &SVector<f32, TPARAMETER_COUNT>, v1: &SVector<f32, TPARAMETER_COUNT>) -> Self {
        let a = v0[1] - v1[1];
        let b = v1[0] - v0[0];
        let c = - (a * (v0[0] + v1[0]) + b * (v0[1] + v1[1])) * 0.5;

        let tie = if a != 0.0 { a > 0.0 } else { b > 0.0 };

        EdgeEquation {
            a, b, c, tie
        }
    }

    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.a * x + self.b * y + self.c
    }

    pub fn test(&self, v: f32) -> bool {
        v > 0.0 || (v == 0.0 && self.tie)
    }

    pub fn step_x_by_one(&self, v: f32) -> f32 {
        v + self.a
    }

    pub fn step_x(&self, v: f32, step_size: f32) -> f32 {
        v + self.a * step_size
    }

    pub fn step_y_by_one(&self, v: f32) -> f32 {
        v + self.b
    }

    pub fn step_y(&self, v: f32, step_size: f32) -> f32 {
        v + self.b * step_size
    }
}

