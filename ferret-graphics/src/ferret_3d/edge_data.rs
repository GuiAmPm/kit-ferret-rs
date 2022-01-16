use crate::ferret_3d::triangle_equations::TriangleEquations;

#[derive(Clone)]
pub struct EdgeData<const TPARAMETER_COUNT: usize> {
    pub x: f32,
    pub y: f32,
    pub ev0: f32,
    pub ev1: f32,
    pub ev2: f32
}

impl<const TPARAMETER_COUNT: usize> EdgeData<TPARAMETER_COUNT> {
    pub fn new(tri_eq: &TriangleEquations<TPARAMETER_COUNT>, x: f32, y: f32) -> Self {
        Self {
            x, y,
            ev0: tri_eq.e0.evaluate(x, y),
            ev1: tri_eq.e1.evaluate(x, y),
            ev2: tri_eq.e2.evaluate(x, y)
        }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            ev0: 0.0,
            ev1: 0.0,
            ev2: 0.0
        }
    }

    pub fn step_x(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>, step_size: f32) {
        self.ev0 = tri_eq.e0.step_x(self.ev0, step_size);
        self.ev1 = tri_eq.e1.step_x(self.ev1, step_size);
        self.ev2 = tri_eq.e2.step_x(self.ev2, step_size);
    }

    pub fn step_x_by_one(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>) {
        self.ev0 = tri_eq.e0.step_x_by_one(self.ev0);
        self.ev1 = tri_eq.e1.step_x_by_one(self.ev1);
        self.ev2 = tri_eq.e2.step_x_by_one(self.ev2);
    }

    pub fn step_y(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>, step_size: f32) {
        self.ev0 = tri_eq.e0.step_y(self.ev0, step_size);
        self.ev1 = tri_eq.e1.step_y(self.ev1, step_size);
        self.ev2 = tri_eq.e2.step_y(self.ev2, step_size);
    }

    pub fn step_y_by_one(&mut self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>) {
        self.ev0 = tri_eq.e0.step_y_by_one(self.ev0);
        self.ev1 = tri_eq.e1.step_y_by_one(self.ev1);
        self.ev2 = tri_eq.e2.step_y_by_one(self.ev2);
    }

    pub fn test(&self, tri_eq: &TriangleEquations<TPARAMETER_COUNT>) -> bool{
        tri_eq.e0.test(self.ev0)
            && tri_eq.e1.test(self.ev1)
            && tri_eq.e2.test(self.ev2)
    }
}