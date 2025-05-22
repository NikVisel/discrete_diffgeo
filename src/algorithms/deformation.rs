use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use crate::geometry::matrix::Matrix3;

/// Mesh deformation algorithms
pub struct Deformation;

impl Deformation {
    /// As-rigid-as-possible (ARAP) deformation
    pub fn arap_deformation(mesh: &Mesh3D<(), ()>, positions: &mut [Vector3], iterations: usize) {
        let n = mesh.vertices.len();
        let mut rotations = vec![Matrix3::identity(); n];
        let mut delta = vec![Vector3::zero(); n];
        let area = mesh.vertex_areas();

        for _ in 0..iterations {
            // Compute local rotations
            for i in 0..n {
                let mut sum = Matrix3::identity();
                let mut he = mesh.vertices[i].he_out;
                loop {
                    let nbr = mesh.halfedges[mesh.halfedges[he].twin].from;
                    let p = positions[i];
                    let q = positions[nbr];
                    let pq = q - p;
                    let r = rotations[nbr];
                    sum = sum + r * pq.outer_product(&pq);
                    he = mesh.halfedges[mesh.halfedges[he].twin].next;
                    if he == mesh.vertices[i].he_out { break; }
                }
                let svd = sum.svd(true, true);
                rotations[i] = svd.u.unwrap() * svd.v_t.unwrap();
            }

            // Compute ARAP displacements
            for i in 0..n {
                let mut sum = Vector3::zero();
                let mut cnt = 0.0;
                let mut he = mesh.vertices[i].he_out;
                loop {
                    let nbr = mesh.halfedges[mesh.halfedges[he].twin].from;
                    let p = positions[i];
                    let q = positions[nbr];
                    let r = rotations[nbr];
                    let pq = q - p;
                    sum = sum + r * pq;
                    cnt += 1.0;
                    he = mesh.halfedges[mesh.halfedges[he].twin].next;
                    if he == mesh.vertices[i].he_out { break; }
                }
                if cnt > 0.0 && area[i] != 0.0 {
                    delta[i] = sum / cnt;
                }
            }

            // Apply displacements
            for i in 0..n {
                positions[i] = positions[i] + delta[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::half_edge::Vertex;

    #[test]
    fn test_arap_deformation() {
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0, 1.0, 0.0) },
            Vertex { he_out: 3, attr: Vector3::new(0.0, 0.0, 1.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 0, next: 1, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 1, next: 2, edge: 0, face: 0 },
            HalfEdge { from: 2, twin: 2, next: 0, edge: 1, face: 0 },
            HalfEdge { from: 3, twin: 3, next: 1, edge: 1, face: 0 },
        ];
        let mut positions = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];
        Deformation::arap_deformation(&mesh, &mut positions, 10);
        assert_eq!(positions.len(), 4);
        for pos in positions {
            assert!((pos.norm() - 1.0).abs() < 1e-6);
        }
    }
}
