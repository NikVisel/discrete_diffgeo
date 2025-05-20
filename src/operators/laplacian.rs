use crate::operators::traits::Operator;
use crate::geometry::vector::Vector3;
use crate::mesh::half_edge::Mesh3D;

/// Cotangent Laplace–Beltrami operator: scalar field on vertices -> scalar per vertex
pub struct Laplacian;

impl Laplacian {
    /// Compute cotangent Laplacian of a scalar field.
    /// Positions stored in vertex attr as Vector3.
    pub fn cotan_laplacian(
        mesh: &Mesh3D<(), ()>,
        field: &[f64],
    ) -> Vec<f64> {
        let n = mesh.vertices.len();
        let mut L = vec![0.0; n];
        // mixed/Voronoi per-vertex areas
        let A = mesh.vertex_areas();

        // Sum cotangent weights along edges
        for edge in &mesh.edges {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            // opposite vertices
            let k0 = mesh.halfedges[mesh.halfedges[he0].next].from;
            let k1 = mesh.halfedges[mesh.halfedges[he1].next].from;
            let pi = mesh.vertices[i].attr;
            let pj = mesh.vertices[j].attr;
            let pk0 = mesh.vertices[k0].attr;
            let pk1 = mesh.vertices[k1].attr;
            let cot0 = cotangent(pi, pj, pk0);
            let cot1 = cotangent(pi, pj, pk1);
            let w = 0.5 * (cot0 + cot1);
            L[i] += w * (field[j] - field[i]);
            L[j] += w * (field[i] - field[j]);
        }

        // Normalize by mixed areas
        for i in 0..n {
            L[i] /= A[i];
        }
        L
    }
}

/// Compute cotangent of angle at vertex pk in triangle (pi, pj, pk)
fn cotangent(pi: Vector3, pj: Vector3, pk: Vector3) -> f64 {
    let u = pi - pk;
    let v = pj - pk;
    u.dot(&v) / u.cross(&v).norm()
}

impl Operator<Vec<f64>, Vec<f64>> for Laplacian {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<f64>) -> Vec<f64> {
        Self::cotan_laplacian(mesh, field)
    }
}