use crate::ferret_3d::parameter_equation::ParameterEquation;
use crate::ferret_3d::edge_equation::EdgeEquation;

use nalgebra::SVector;

pub struct TriangleEquations<const TPARAMETER_COUNT: usize> {
    pub area2: f32,
    pub e0: EdgeEquation<TPARAMETER_COUNT>,
    pub e1: EdgeEquation<TPARAMETER_COUNT>,
    pub e2: EdgeEquation<TPARAMETER_COUNT>,
    pub a_var: [ParameterEquation<TPARAMETER_COUNT>; TPARAMETER_COUNT]
}

impl<const TPARAMETER_COUNT: usize> TriangleEquations<TPARAMETER_COUNT> {
    pub fn new(v0: &SVector<f32, TPARAMETER_COUNT>, v1: &SVector<f32, TPARAMETER_COUNT>, v2: &SVector<f32, TPARAMETER_COUNT>) -> Self{
        let e0 = EdgeEquation::new(v0, v1);
        let e1 = EdgeEquation::new(v1, v2);
        let e2 = EdgeEquation::new(v2, v0);

        let area2 = e0.c + e1.c + e2.c;

        #[allow(deprecated)] // TODO: find another way to create uninitialized array
        let mut a_var: [ParameterEquation<TPARAMETER_COUNT>; TPARAMETER_COUNT] = unsafe { core::mem::uninitialized() };

        if area2 >= 0.0 {
            let factor = 1.0 / area2;
            for i in 0..TPARAMETER_COUNT {
                a_var[i] = ParameterEquation::new(v0[i], v1[i], v2[i], &e0, &e1, &e2, factor);
            }
        }

        Self {
            area2,
            e0, e1, e2,
            a_var
        }
    }
}