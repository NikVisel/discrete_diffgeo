use crate::operators::traits::Operator;
use crate::mesh::half_edge::{Mesh3D};
use crate::geometry::vector::Vector3;

/// Gradient operator: scalar field on vertices -> vector per face
pub struct Gradient;

impl Gradient {
    /// Compute per-face gradient: ∇f = Σ f_i (N×e_i) / |N|
    pub fn compute(mesh: &Mesh3D<(), ()>, field: &[f64]) -> Vec<Vector3> {
        let mut result = Vec::with_capacity(mesh.faces.len());
        for face in &mesh.faces {
            let h0 = face.he;
            let h1 = mesh.halfedges[h0].next;
            let h2 = mesh.halfedges[h1].next;
            let vs = [mesh.halfedges[h0].from, mesh.halfedges[h1].from, mesh.halfedges[h2].from];
            let p0 = mesh.vertices[vs[0]].attr;
            let p1 = mesh.vertices[vs[1]].attr;
            let p2 = mesh.vertices[vs[2]].attr;
            let u = p1 - p0;
            let v = p2 - p0;
            let n = u.cross(&v);
            let norm_n = n.norm();
            if norm_n == 0.0 {
                result.push(Vector3::zero());
                continue;
            }
            let e = [p2 - p1, p0 - p2, p1 - p0];
            let fvals = [field[vs[0]], field[vs[1]], field[vs[2]]];
            let mut grad = Vector3::zero();
            for i in 0..3 {
                grad = grad + n.cross(&e[i]) * fvals[i];
            }
            result.push(grad / norm_n);
        }
        result
    }
}

impl Operator<Vec<f64>, Vec<Vector3>> for Gradient {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<f64>) -> Vec<Vector3> {
        Gradient::compute(mesh, field)
    }
}