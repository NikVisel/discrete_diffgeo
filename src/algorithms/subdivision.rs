use crate::mesh::half_edge::{Mesh3D, VertexId, EdgeId, FaceId};
use crate::geometry::vector::Vector3;

/// Mesh subdivision algorithms
pub struct Subdivision;

impl Subdivision {
    /// Perform Catmull-Clark subdivision on the mesh
    pub fn catmull_clark(mesh: &mut Mesh3D<(), ()>) {
        let mut new_positions = vec![Vector3::zero(); mesh.vertices.len()];

        // Compute face points
        let mut face_points = vec![Vector3::zero(); mesh.faces.len()];
        for (f_id, face) in mesh.faces.iter().enumerate() {
            let mut sum = Vector3::zero();
            let mut count = 0;
            let mut he = face.he;
            loop {
                sum = sum + mesh.vertices[mesh.halfedges[he].from].attr;
                count += 1;
                he = mesh.halfedges[he].next;
                if he == face.he { break; }
            }
            face_points[f_id] = sum / count as f64;
        }

        // Compute edge points
        let mut edge_points = vec![Vector3::zero(); mesh.edges.len()];
        for (e_id, edge) in mesh.edges.iter().enumerate() {
            let he = edge.he;
            let v0 = mesh.halfedges[he].from;
            let v1 = mesh.halfedges[mesh.halfedges[he].twin].from;
            let f0 = mesh.halfedges[he].face;
            let f1 = mesh.halfedges[mesh.halfedges[he].twin].face;
            edge_points[e_id] = (mesh.vertices[v0].attr + mesh.vertices[v1].attr + face_points[f0] + face_points[f1]) * 0.25;
        }

        // Compute new vertex positions
        for (v_id, vertex) in mesh.vertices.iter().enumerate() {
            let mut sum_face_points = Vector3::zero();
            let mut sum_edge_points = Vector3::zero();
            let mut sum_vertex_points = Vector3::zero();
            let mut n = 0;
            let mut he = vertex.he_out;
            loop {
                let next_he = mesh.halfedges[he].next;
                let twin_he = mesh.halfedges[he].twin;
                let edge = mesh.halfedges[he].edge;
                let face = mesh.halfedges[he].face;
                sum_face_points = sum_face_points + face_points[face];
                sum_edge_points = sum_edge_points + edge_points[edge];
                sum_vertex_points = sum_vertex_points + mesh.vertices[mesh.halfedges[next_he].from].attr;
                n += 1;
                he = mesh.halfedges[twin_he].next;
                if he == vertex.he_out { break; }
            }
            new_positions[v_id] = (sum_face_points + 2.0 * sum_edge_points + (n as f64 - 3.0) * vertex.attr) / n as f64;
        }

        // Update vertex positions
        for (v_id, vertex) in mesh.vertices.iter_mut().enumerate() {
            vertex.attr = new_positions[v_id];
        }
    }

    /// Perform Loop subdivision on the mesh
    pub fn loop_subdivision(mesh: &mut Mesh3D<(), ()>) {
        let mut new_positions = vec![Vector3::zero(); mesh.vertices.len()];

        // Compute edge points
        let mut edge_points = vec![Vector3::zero(); mesh.edges.len()];
        for (e_id, edge) in mesh.edges.iter().enumerate() {
            let he = edge.he;
            let v0 = mesh.halfedges[he].from;
            let v1 = mesh.halfedges[mesh.halfedges[he].twin].from;
            let f0 = mesh.halfedges[he].face;
            let f1 = mesh.halfedges[mesh.halfedges[he].twin].face;
            let mut sum = mesh.vertices[v0].attr + mesh.vertices[v1].attr;
            let mut count = 2;
            if f0 != f1 {
                sum = sum + mesh.vertices[mesh.halfedges[mesh.halfedges[he].next].from].attr;
                sum = sum + mesh.vertices[mesh.halfedges[mesh.halfedges[mesh.halfedges[he].twin].next].from].attr;
                count += 2;
            }
            edge_points[e_id] = sum / count as f64;
        }

        // Compute new vertex positions
        for (v_id, vertex) in mesh.vertices.iter().enumerate() {
            let mut sum_edge_points = Vector3::zero();
            let mut sum_vertex_points = Vector3::zero();
            let mut n = 0;
            let mut he = vertex.he_out;
            loop {
                let next_he = mesh.halfedges[he].next;
                let twin_he = mesh.halfedges[he].twin;
                let edge = mesh.halfedges[he].edge;
                sum_edge_points = sum_edge_points + edge_points[edge];
                sum_vertex_points = sum_vertex_points + mesh.vertices[mesh.halfedges[next_he].from].attr;
                n += 1;
                he = mesh.halfedges[twin_he].next;
                if he == vertex.he_out { break; }
            }
            new_positions[v_id] = (sum_edge_points + 3.0 * vertex.attr) / (n as f64 + 3.0);
        }

        // Update vertex positions
        for (v_id, vertex) in mesh.vertices.iter_mut().enumerate() {
            vertex.attr = new_positions[v_id];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3;

    #[test]
    fn test_catmull_clark() {
        // Create a simple mesh with 4 vertices and 1 face
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0, 1.0, 0.0) },
            Vertex { he_out: 3, attr: Vector3::new(1.0, 1.0, 0.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 1, next: 2, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 0, next: 3, edge: 0, face: 0 },
            HalfEdge { from: 2, twin: 3, next: 0, edge: 1, face: 0 },
            HalfEdge { from: 1, twin: 2, next: 4, edge: 1, face: 0 },
            HalfEdge { from: 3, twin: 5, next: 1, edge: 2, face: 1 },
            HalfEdge { from: 2, twin: 4, next: 3, edge: 2, face: 1 },
        ];
        mesh.edges = vec![
            Edge { he: 0, attr: () },
            Edge { he: 2, attr: () },
            Edge { he: 4, attr: () },
        ];
        mesh.faces = vec![
            Face { he: 0, attr: () },
            Face { he: 4, attr: () },
        ];

        // Perform Catmull-Clark subdivision
        Subdivision::catmull_clark(&mut mesh);

        // Check the number of vertices
        assert_eq!(mesh.vertices.len(), 4);
    }

    #[test]
    fn test_loop_subdivision() {
        // Create a simple mesh with 4 vertices and 1 face
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0, 0.0, 0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0, 0.0, 0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0, 1.0, 0.0) },
            Vertex { he_out: 3, attr: Vector3::new(1.0, 1.0, 0.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from: 0, twin: 1, next: 2, edge: 0, face: 0 },
            HalfEdge { from: 1, twin: 0, next: 3, edge: 0, face: 0 },
            HalfEdge { from: 2, twin: 3, next: 0, edge: 1, face: 0 },
            HalfEdge { from: 1, twin: 2, next: 4, edge: 1, face: 0 },
            HalfEdge { from: 3, twin: 5, next: 1, edge: 2, face: 1 },
            HalfEdge { from: 2, twin: 4, next: 3, edge: 2, face: 1 },
        ];
        mesh.edges = vec![
            Edge { he: 0, attr: () },
            Edge { he: 2, attr: () },
            Edge { he: 4, attr: () },
        ];
        mesh.faces = vec![
            Face { he: 0, attr: () },
            Face { he: 4, attr: () },
        ];

        // Perform Loop subdivision
        Subdivision::loop_subdivision(&mut mesh);

        // Check the number of vertices
        assert_eq!(mesh.vertices.len(), 4);
    }
}
