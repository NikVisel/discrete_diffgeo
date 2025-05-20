use crate::operators::traits::Operator;
use crate::mesh::half_edge::{Mesh3D};
use crate::geometry::vector::Vector3;
use crate::geometry::matrix::Matrix3;

/// Per-face Jacobian tensor of a 3D vector field.
/// Computes J such that its columns are the gradient of each vector component.
pub struct Jacobian;

impl Jacobian {
    pub fn compute(mesh: &Mesh3D<(), ()>, field: &[Vector3]) -> Vec<Matrix3> {
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
                result.push(Matrix3::identity());
                continue;
            }
            let e = [p2 - p1, p0 - p2, p1 - p0];
            let fvs = [field[vs[0]], field[vs[1]], field[vs[2]]];
            let mut cols = [Vector3::zero(); 3];
            for comp in 0..3 {
                let mut grad = Vector3::zero();
                for i in 0..3 {
                    let scalar_f = match comp {
                        0 => fvs[i].x,
                        1 => fvs[i].y,
                        _ => fvs[i].z,
                    };
                    grad = grad + n.cross(&e[i]) * scalar_f;
                }
                cols[comp] = grad / norm_n;
            }
            let mat = [
                [cols[0].x, cols[1].x, cols[2].x],
                [cols[0].y, cols[1].y, cols[2].y],
                [cols[0].z, cols[1].z, cols[2].z],
            ];
            result.push(Matrix3::new(mat));
        }
        result
    }
}

impl Operator<Vec<Vector3>, Vec<Matrix3>> for Jacobian {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<Vector3>) -> Vec<Matrix3> {
        Jacobian::compute(mesh, field)
    }
}
