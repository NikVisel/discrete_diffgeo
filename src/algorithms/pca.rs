use nalgebra::{DMatrix, DVector};
use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;

/// Principal Component Analysis (PCA) on mesh vertices
pub struct PCA;

impl PCA {
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

    /// Perform PCA on the vertex positions and return the principal components
    pub fn compute(mesh: &Mesh3D<(), ()>) -> (Vector3, Vector3, Vector3) {
        let positions: Vec<Vector3> = mesh.vertices.iter().map(|v| v.attr).collect();
        let cov = Self::covariance_matrix(&positions);
        let eig = cov.symmetric_eigen();
        let mut eigenvectors = eig.eigenvectors.column_iter().collect::<Vec<_>>();
        eigenvectors.sort_by(|a, b| b.norm().partial_cmp(&a.norm()).unwrap());
        let pc1 = Vector3::new(eigenvectors[0][0], eigenvectors[0][1], eigenvectors[0][2]);
        let pc2 = Vector3::new(eigenvectors[1][0], eigenvectors[1][1], eigenvectors[1][2]);
        let pc3 = Vector3::new(eigenvectors[2][0], eigenvectors[2][1], eigenvectors[2][2]);
        (pc1, pc2, pc3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::half_edge::Vertex;

    #[test]
    fn test_pca() {
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(1.0, 2.0, 3.0) },
            Vertex { he_out: 0, attr: Vector3::new(4.0, 5.0, 6.0) },
            Vertex { he_out: 0, attr: Vector3::new(7.0, 8.0, 9.0) },
        ];
        let (pc1, pc2, pc3) = PCA::compute(&mesh);
        assert!((pc1.norm() - 1.0).abs() < 1e-6);
        assert!((pc2.norm() - 1.0).abs() < 1e-6);
        assert!((pc3.norm() - 1.0).abs() < 1e-6);
    }
}
