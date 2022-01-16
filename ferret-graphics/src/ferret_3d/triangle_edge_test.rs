use crate::ferret_3d::edge_data::EdgeData;
use crate::ferret_3d::triangle_equations::TriangleEquations;

pub struct TriangleEdgeTest<const TPARAMETER_COUNT: usize>(bool, bool, bool);

impl<const TPARAMETER_COUNT: usize> TriangleEdgeTest<TPARAMETER_COUNT> {
    pub fn new(triangle: &TriangleEquations<TPARAMETER_COUNT>, edge: &EdgeData<TPARAMETER_COUNT>) -> Self {
        Self(
            triangle.e0.test(edge.ev0),
            triangle.e1.test(edge.ev1),
            triangle.e2.test(edge.ev2)
        )
    }

    pub fn all_true(&self) -> bool {
        self.0 && self.1 && self.2
    }

    pub fn all_same(&self) -> bool {
        self.0 == self.1 && self.0 == self.2
    }
}