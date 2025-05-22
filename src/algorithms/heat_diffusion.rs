use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use nalgebra::DMatrix;

/// Heat diffusion algorithm
pub struct HeatDiffusion;

impl HeatDiffusion {
    /// Compute heat diffusion on a scalar field over the mesh vertices.
    ///
    /// # Parameters
    /// - `mesh`: half-edge mesh
    /// - `field`: initial scalar field values at vertices
    /// - `time`: diffusion time
    /// - `diffusivity`: diffusion coefficient
    ///
    /// # Returns
    /// - `Vec<f64>`: scalar field values after diffusion
    pub fn diffuse(
        mesh: &Mesh3D<(), ()>,
        field: &[f64],
        time: f64,
        diffusivity: f64,
    ) -> Vec<f64> {
        let n = mesh.vertices.len();
        let mut laplacian = DMatrix::zeros(n, n);
        let mut mass_matrix = DMatrix::zeros(n, n);

        // Build Laplacian and mass matrices
        for edge in &mesh.edges {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            let k0 = mesh.halfedges[mesh.halfedges[he0].next].from;
            let k1 = mesh.halfedges[mesh.halfedges[he1].next].from;
            let pi = mesh.vertices[i].attr;
            let pj = mesh.vertices[j].attr;
            let pk0 = mesh.vertices[k0].attr;
            let pk1 = mesh.vertices[k1].attr;
            let cot0 = cotangent(pi, pj, pk0);
            let cot1 = cotangent(pi, pj, pk1);
            let w = 0.5 * (cot0 + cot1);
            laplacian[(i, j)] = w;
            laplacian[(j, i)] = w;
            laplacian[(i, i)] -= w;
            laplacian[(j, j)] -= w;
        }

        // Build mass matrix
        let areas = mesh.vertex_areas();
        for i in 0..n {
            mass_matrix[(i, i)] = areas[i];
        }

        // Solve heat equation
        let id = DMatrix::identity(n, n);
        let lhs = &mass_matrix + time * diffusivity * &laplacian;
        let rhs = &mass_matrix * DMatrix::from_column_slice(n, 1, field);
        let solution = lhs.lu().solve(&rhs).unwrap();

        solution.column(0).iter().cloned().collect()
    }
}

/// Compute cotangent of angle at vertex pk in triangle (pi, pj, pk)
fn cotangent(pi: Vector3, pj: Vector3, pk: Vector3) -> f64 {
    let u = pi - pk;
    let v = pj - pk;
    u.dot(&v) / u.cross(&v).norm()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3;

    #[test]
    fn test_heat_diffusion() {
        // Simple triangle mesh
        let mut mesh = Mesh3D::<(), ()>::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0, 1.0, 0.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 0, next: 1, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 1, next: 2, edge: 1, face: 0 },
            HalfEdge { from: 2, twin: 2, next: 0, edge: 2, face: 0 },
        ];
        mesh.edges = vec![
            Edge { he: 0, attr: () },
            Edge { he: 1, attr: () },
            Edge { he: 2, attr: () },
        ];
        mesh.faces = vec![ Face { he: 0, attr: () } ];

        let field = vec![1.0, 0.0, 0.0];
        let time = 0.1;
        let diffusivity = 1.0;
        let result = HeatDiffusion::diffuse(&mesh, &field, time, diffusivity);

        assert!((result[0] - 0.75).abs() < 1e-6);
        assert!((result[1] - 0.125).abs() < 1e-6);
        assert!((result[2] - 0.125).abs() < 1e-6);
    }
}
