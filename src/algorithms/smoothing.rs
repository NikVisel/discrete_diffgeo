use crate::geometry::vector::Vector3;
use crate::mesh::half_edge::Mesh3D;

/// Mesh smoothing algorithms
pub struct Smoothing;

impl Smoothing {
    /// Perform uniform Laplacian smoothing on vertex positions.
    ///
    /// # Parameters
    /// - `mesh`: half-edge mesh
    /// - `positions`: mutable slice of vertex positions
    /// - `iterations`: number of smoothing iterations
    /// - `alpha`: smoothing factor (0 < alpha < 1)
    pub fn laplacian_smoothing(
        mesh: &Mesh3D<(), ()>,
        positions: &mut [Vector3],
        iterations: usize,
        alpha: f64,
    ) {
        let n = mesh.vertices.len();
        let mut delta = vec![Vector3::zero(); n];
        // mixed/Voronoi area per vertex
        let area = mesh.vertex_areas();

        for _ in 0..iterations {
            // compute Laplacian displacement
            for i in 0..n {
                let mut sum = Vector3::zero();
                let mut cnt = 0.0;
                let mut he = mesh.vertices[i].he_out;
                loop {
                    let nbr = mesh.halfedges[mesh.halfedges[he].twin].from;
                    sum = sum + positions[nbr];
                    cnt += 1.0;
                    he = mesh.halfedges[mesh.halfedges[he].twin].next;
                    if he == mesh.vertices[i].he_out { break; }
                }
                if cnt > 0.0 && area[i] != 0.0 {
                    // uniform neighbor average minus pos, normalized by area
                    let avg = sum / cnt;
                    delta[i] = (avg - positions[i]) / area[i];
                }
            }

            // apply smoothing
            for i in 0..n {
                positions[i] = positions[i] + delta[i] * alpha;
            }
        }
    }
}