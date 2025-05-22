use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use nalgebra::{DMatrix, DVector};

/// Surface normal estimation using PCA
pub struct NormalEstimation;

impl NormalEstimation {
    /// Estimate normals for each vertex using PCA on the neighborhood
    pub fn estimate_normals(mesh: &Mesh3D<(), ()>) -> Vec<Vector3> {
        let mut normals = vec![Vector3::zero(); mesh.vertices.len()];
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let neighbors = mesh.vertex_neighbors(i);
            let mut positions = Vec::with_capacity(neighbors.len());
            for &nbr in &neighbors {
                positions.push(mesh.vertices[nbr].attr);
            }
            let cov = Self::covariance_matrix(&positions);
            let eig = cov.symmetric_eigen();
            let normal = Vector3::new(eig.eigenvectors[(0, 0)], eig.eigenvectors[(1, 0)], eig.eigenvectors[(2, 0)]);
            normals[i] = normal.normalize();
        }
        normals
    }

    /// Compute the covariance matrix of the vertex positions
    fn covariance_matrix(positions: &[Vector3]) -> DMatrix<f64> {
        let n = positions.len();
        let mut data = Vec::with_capacity(n * 3);
        for p in positions {
            data.push(p.x);
            data.push(p.y);
            data.push(p.z);
        }
        let matrix = DMatrix::from_vec(n, 3, data);
        let mean = matrix.column_mean();
        let centered = matrix - mean;
        let cov = &centered.transpose() * &centered / (n as f64 - 1.0);
        cov
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::half_edge::Vertex;

    #[test]
    fn test_normal_estimation() {
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0, 1.0, 0.0) },
            Vertex { he_out: 3, attr: Vector3::new(0.0, 0.0, 1.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 1, next: 2, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 0, next: 3, edge: 0, face: 0 },
            HalfEdge { from: 2, twin: 3, next: 0, edge: 1, face: 0 },
            HalfEdge { from: 3, twin: 2, next: 1, edge: 1, face: 0 },
        ];
        let normals = NormalEstimation::estimate_normals(&mesh);
        assert_eq!(normals.len(), 4);
        for normal in normals {
            assert!((normal.norm() - 1.0).abs() < 1e-6);
        }
    }
}
