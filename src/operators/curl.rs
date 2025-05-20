use crate::operators::traits::Operator;
use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;

/// Curl operator: vector field on faces -> vector per vertex
pub struct Curl;

impl Curl {
    /// Compute discrete curl per vertex: sum over incident faces of field × edge
    pub fn compute(mesh: &Mesh3D<(), ()>, field: &[Vector3]) -> Vec<Vector3> {
        let n = mesh.vertices.len();
        let mut curl = vec![Vector3::zero(); n];
        // mixed/Voronoi area per vertex
        let area = mesh.vertex_areas();
        // sum field × edge around each vertex
        for i in 0..n {
            let mut he = mesh.vertices[i].he_out;
            loop {
                let f_id = mesh.halfedges[he].face;
                let F = field[f_id];
                let nex = mesh.halfedges[he].next;
                let j = mesh.halfedges[nex].from;
                let pi = mesh.vertices[i].attr;
                let pj = mesh.vertices[j].attr;
                let e = pj - pi;
                // cross product F × e
                let c = F.cross(&e);
                curl[i] = curl[i] + c;
                let twin = mesh.halfedges[he].twin;
                he = mesh.halfedges[twin].next;
                if he == mesh.vertices[i].he_out { break; }
            }
            // normalize by mixed/Voronoi area
            if area[i] != 0.0 {
                curl[i] = curl[i] / area[i];
            }
        }
        curl
    }
}

impl Operator<Vec<Vector3>, Vec<Vector3>> for Curl {
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Vec<Vector3>) -> Vec<Vector3> {
        Curl::compute(mesh, field)
    }
}