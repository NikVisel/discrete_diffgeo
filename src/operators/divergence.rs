use crate::operators::traits::Operator;
use crate::mesh::half_edge::{Mesh3D};
use crate::geometry::vector::Vector3;

/// Divergence operator: vector field on faces -> scalar per vertex
pub struct Divergence;

impl Divergence {
    /// Compute discrete divergence per vertex: sum of face vector · edge over incident faces
    pub fn compute(mesh: &Mesh3D<(), ()>, field: &[Vector3]) -> Vec<f64> {
        let n = mesh.vertices.len();
        let mut div = vec![0.0; n];
        // mixed/Voronoi area per vertex
        let area = mesh.vertex_areas();
        for i in 0..n {
            let mut he = mesh.vertices[i].he_out;
            loop {
                let f_id = mesh.halfedges[he].face;
                let f = field[f_id];
                let nex = mesh.halfedges[he].next;
                let j = mesh.halfedges[nex].from;
                let pi = mesh.vertices[i].attr;
                let pj = mesh.vertices[j].attr;
                let e = pj - pi;
                div[i] += f.dot(&e);
                he = mesh.halfedges[mesh.halfedges[he].twin].next;
                if he == mesh.vertices[i].he_out { break; }
            }
            if area[i] != 0.0 {
                div[i] /= area[i];
            }
        }
        div
    }
}

impl Operator<Vec<Vector3>, Vec<f64>> for Divergence {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<Vector3>) -> Vec<f64> {
        Divergence::compute(mesh, field)
    }
}