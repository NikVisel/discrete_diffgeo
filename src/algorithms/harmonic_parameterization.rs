use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use nalgebra::{DMatrix, DVector};

/// Harmonic parameterization algorithm for surface meshes.
pub struct HarmonicParameterization;

impl HarmonicParameterization {
    /// Compute harmonic parameterization of a mesh.
    /// Positions stored in vertex attr as Vector3.
    pub fn compute(mesh: &Mesh3D<(), ()>) -> Vec<[f64; 2]> {
        let n = mesh.vertices.len();
        let mut uv = vec![[0.0; 2]; n];

        // Identify boundary loop
        let boundary = Self::find_boundary_loop(mesh);
        let m = boundary.len();

        // Map boundary to unit circle
        for (i, &v) in boundary.iter().enumerate() {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / m as f64;
            uv[v] = [theta.cos(), theta.sin()];
        }

        // Assemble Laplacian system
        let mut L = DMatrix::zeros(n, n);
        let mut B = DMatrix::zeros(n, 2);
        for edge in &mesh.edges {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            let w = Self::cotangent_weight(mesh, he0, he1);
            L[(i, j)] = -w;
            L[(j, i)] = -w;
            L[(i, i)] += w;
            L[(j, j)] += w;
        }

        // Set boundary conditions
        for &v in &boundary {
            L[(v, v)] = 1.0;
            B[(v, 0)] = uv[v][0];
            B[(v, 1)] = uv[v][1];
        }

        // Solve linear system
        let L_inv = L.try_inverse().expect("Laplacian matrix is not invertible");
        let U = L_inv * B;

        // Extract UV coordinates
        for i in 0..n {
            uv[i] = [U[(i, 0)], U[(i, 1)]];
        }

        uv
    }

    /// Find boundary loop of the mesh.
    fn find_boundary_loop(mesh: &Mesh3D<(), ()>) -> Vec<usize> {
        let mut boundary = Vec::new();
        for (i, he) in mesh.halfedges.iter().enumerate() {
            if mesh.halfedges[he.twin].face == usize::MAX {
                boundary.push(i);
            }
        }
        boundary
    }

    /// Compute cotangent weight for an edge.
    fn cotangent_weight(mesh: &Mesh3D<(), ()>, he0: usize, he1: usize) -> f64 {
        let i = mesh.halfedges[he0].from;
        let j = mesh.halfedges[he1].from;
        let k0 = mesh.halfedges[mesh.halfedges[he0].next].from;
        let k1 = mesh.halfedges[mesh.halfedges[he1].next].from;
        let pi = mesh.vertices[i].attr;
        let pj = mesh.vertices[j].attr;
        let pk0 = mesh.vertices[k0].attr;
        let pk1 = mesh.vertices[k1].attr;
        let cot0 = Self::cotangent(pi, pj, pk0);
        let cot1 = Self::cotangent(pi, pj, pk1);
        0.5 * (cot0 + cot1)
    }

    /// Compute cotangent of angle at vertex pk in triangle (pi, pj, pk)
    fn cotangent(pi: Vector3, pj: Vector3, pk: Vector3) -> f64 {
        let u = pi - pk;
        let v = pj - pk;
        u.dot(&v) / u.cross(&v).norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::half_edge::{Mesh, Vertex, HalfEdge, Edge, Face};

    #[test]
    fn test_harmonic_parameterization() {
        // Simple square mesh
        let mut mesh = Mesh3D::<(), ()>::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(1.0, 1.0, 0.0) },
            Vertex { he_out: 3, attr: Vector3::new(0.0, 1.0, 0.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 1, next: 2, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 0, next: 3, edge: 0, face: 0 },
            HalfEdge { from: 2, twin: 3, next: 0, edge: 1, face: 0 },
            HalfEdge { from: 3, twin: 2, next: 1, edge: 1, face: 0 },
        ];
        mesh.edges = vec![
            Edge { he: 0, attr: () },
            Edge { he: 2, attr: () },
        ];
        mesh.faces = vec![
            Face { he: 0, attr: () },
        ];

        let uv = HarmonicParameterization::compute(&mesh);
        assert_eq!(uv.len(), 4);
        assert!((uv[0][0] - 1.0).abs() < 1e-6);
        assert!((uv[0][1] - 0.0).abs() < 1e-6);
        assert!((uv[1][0] - 0.0).abs() < 1e-6);
        assert!((uv[1][1] - 1.0).abs() < 1e-6);
        assert!((uv[2][0] - -1.0).abs() < 1e-6);
        assert!((uv[2][1] - 0.0).abs() < 1e-6);
        assert!((uv[3][0] - 0.0).abs() < 1e-6);
        assert!((uv[3][1] - -1.0).abs() < 1e-6);
    }
}
