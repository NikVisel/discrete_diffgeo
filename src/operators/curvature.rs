use crate::operators::traits::Operator;
use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use std::f64::consts::PI;

/// Mean curvature normal operator
pub struct MeanCurvatureNormal;
/// Gaussian curvature operator
pub struct GaussianCurvature;

impl MeanCurvatureNormal {
    /// Compute mean curvature normal vector per vertex.
    pub fn compute(mesh: &Mesh3D<(), ()>, positions: &[Vector3]) -> Vec<Vector3> {
        let n = mesh.vertices.len();
        let mut H = vec![Vector3::zero(); n];
        // mixed/Voronoi area per vertex
        let A = mesh.vertex_areas();
        // accumulate cotangent weights
        fn cot(p: Vector3, q: Vector3, r: Vector3) -> f64 {
            let u = p - r;
            let v = q - r;
            u.dot(&v) / u.cross(&v).norm()
        }
        for edge in &mesh.edges {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            let k0 = mesh.halfedges[mesh.halfedges[he0].next].from;
            let k1 = mesh.halfedges[mesh.halfedges[he1].next].from;
            let p_i = positions[i];
            let p_j = positions[j];
            let cot0 = cot(p_i, p_j, positions[k0]);
            let cot1 = cot(p_j, p_i, positions[k1]);
            let w = 0.5 * (cot0 + cot1);
            let diff = p_j - p_i;
            H[i] = H[i] + diff * w;
            H[j] = H[j] - diff * w;
        }
        for i in 0..n {
            H[i] = H[i] / (2.0 * A[i]);
        }
        H
    }
}

impl Operator<Vec<Vector3>, Vec<Vector3>> for MeanCurvatureNormal {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<Vector3>) -> Vec<Vector3> {
        MeanCurvatureNormal::compute(mesh, field)
    }
}

impl GaussianCurvature {
    /// Compute Gaussian curvature per vertex.
    pub fn compute(mesh: &Mesh3D<(), ()>, positions: &[Vector3]) -> Vec<f64> {
        let n = mesh.vertices.len();
        let mut k = vec![2.0 * PI; n];
        // mixed/Voronoi area per vertex
        let a = mesh.vertex_areas();
        fn angle(u: Vector3, v: Vector3) -> f64 {
            let dot = u.dot(&v);
            let nu = u.norm();
            let nv = v.norm();
            (dot / (nu * nv)).clamp(-1.0, 1.0).acos()
        }
        for face in &mesh.faces {
            // accumulate angle deficits (A precomputed)
            let he0 = face.he;
            let he1 = mesh.halfedges[he0].next;
            let he2 = mesh.halfedges[he1].next;
            let vs = [mesh.halfedges[he0].from, mesh.halfedges[he1].from, mesh.halfedges[he2].from];
            let p0 = positions[vs[0]];
            let p1 = positions[vs[1]];
            let p2 = positions[vs[2]];
            let u0 = p1 - p0;
            let v0 = p2 - p0;
            let u1 = p2 - p1;
            let v1 = p0 - p1;
            let u2 = p0 - p2;
            let v2 = p1 - p2;
            let ang0 = angle(u0, v0);
            let ang1 = angle(u1, v1);
            let ang2 = angle(u2, v2);
            k[vs[0]] -= ang0;
            k[vs[1]] -= ang1;
            k[vs[2]] -= ang2;
        }
        for i in 0..n {
            k[i] = k[i] / a[i];
        }
        k
    }
}

impl Operator<Vec<Vector3>, Vec<f64>> for GaussianCurvature {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<Vector3>) -> Vec<f64> {
        GaussianCurvature::compute(mesh, field)
    }
}