//! Half-edge mesh data structure

use crate::geometry::{area, vector::Vector3};

/// Newtype IDs for mesh entities
pub type VertexId = usize;
pub type HalfEdgeId = usize;
pub type EdgeId = usize;
pub type FaceId = usize;

/// Generic half-edge mesh
#[derive(Debug, Default)]
pub struct Mesh<VA = (), EA = (), FA = ()> {
    pub vertices: Vec<Vertex<VA>>,
    pub halfedges: Vec<HalfEdge>,
    pub edges: Vec<Edge<EA>>,
    pub faces: Vec<Face<FA>>,
}

/// Vertex with outgoing half-edge and attribute
#[derive(Debug)]
pub struct Vertex<VA> {
    pub he_out: HalfEdgeId,
    pub attr: VA,
}

/// Half-edge linking topology
#[derive(Debug)]
pub struct HalfEdge {
    pub from: VertexId,
    pub twin: HalfEdgeId,
    pub next: HalfEdgeId,
    pub edge: EdgeId,
    pub face: FaceId,
}

/// Edge with representative half-edge and attribute
#[derive(Debug)]
pub struct Edge<EA> {
    pub he: HalfEdgeId,
    pub attr: EA,
}

/// Face with one half-edge and attribute
#[derive(Debug)]
pub struct Face<FA> {
    pub he: HalfEdgeId,
    pub attr: FA,
}

impl<VA, EA, FA> Mesh<VA, EA, FA> {
    /// Return the vertex indices of the face with the given face index (assumes triangle)
    pub fn face_vertices(&self, f_id: usize) -> [usize; 3] {
        let face = &self.faces[f_id];
        let h0 = face.he;
        let h1 = self.halfedges[h0].next;
        let h2 = self.halfedges[h1].next;
        [self.halfedges[h0].from, self.halfedges[h1].from, self.halfedges[h2].from]
    }

    /// Create an empty mesh
    pub fn new() -> Self where VA: Default, EA: Default, FA: Default {
        Default::default()
    }
    // For mesh types that do not implement Default, provide a custom constructor as needed.


    /// Return the IDs of faces incident to vertex `vid`.
    pub fn vertex_incident_faces(&self, vid: VertexId) -> Vec<FaceId> {
        let mut faces = Vec::new();
        let start = self.vertices[vid].he_out;
        let mut he = start;
        loop {
            faces.push(self.halfedges[he].face);
            he = self.halfedges[self.halfedges[he].twin].next;
            if he == start { break; }
        }
        faces
    }

    /// Return the IDs of vertices adjacent to vertex `vid`.
    pub fn vertex_neighbors(&self, vid: VertexId) -> Vec<VertexId> {
        let mut nbrs = Vec::new();
        let start = self.vertices[vid].he_out;
        let mut he = start;
        loop {
            let next_he = self.halfedges[he].next;
            nbrs.push(self.halfedges[next_he].from);
            he = self.halfedges[self.halfedges[he].twin].next;
            if he == start { break; }
        }
        nbrs
    }

    /// Return the IDs of edges incident to vertex `vid`.
    pub fn vertex_incident_edges(&self, vid: VertexId) -> Vec<EdgeId> {
        let mut edges = Vec::new();
        let start = self.vertices[vid].he_out;
        let mut he = start;
        loop {
            edges.push(self.halfedges[he].edge);
            he = self.halfedges[self.halfedges[he].twin].next;
            if he == start { break; }
        }
        edges
    }
}

/// 3D mesh with Vector3 attributes
pub type Mesh3D<EA, FA> = Mesh<Vector3, EA, FA>;

impl<EA, FA> Mesh<Vector3, EA, FA> {
    /// Compute per-vertex barycentric area (sum of one-third of each incident face)
    pub fn vertex_areas(&self) -> Vec<f64> {
        let n = self.vertices.len();
        let mut areas = vec![0.0; n];
        for face in &self.faces {
            let h0 = face.he;
            let h1 = self.halfedges[h0].next;
            let h2 = self.halfedges[h1].next;
            let vs = [
                self.halfedges[h0].from,
                self.halfedges[h1].from,
                self.halfedges[h2].from,
            ];
            let pa = self.vertices[vs[0]].attr;
            let pb = self.vertices[vs[1]].attr;
            let pc = self.vertices[vs[2]].attr;
            let ba = area::mixed_area(&pa, &pb, &pc);
            for (i, &vi) in vs.iter().enumerate() {
                areas[vi] += ba[i];
            }
        }
        areas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3;

    #[test]
    fn test_vertex_areas_triangle() {
        // Single triangle (0,0,0), (1,0,0), (0,1,0)
        let mut mesh = Mesh3D::<(),()>::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::new(0.0,0.0,0.0) },
            Vertex { he_out: 1, attr: Vector3::new(1.0,0.0,0.0) },
            Vertex { he_out: 2, attr: Vector3::new(0.0,1.0,0.0) },
        ];
        mesh.halfedges = vec![
            HalfEdge { from:0, twin:0, next:1, edge:0, face:0 },
            HalfEdge { from:1, twin:1, next:2, edge:1, face:0 },
            HalfEdge { from:2, twin:2, next:0, edge:2, face:0 },
        ];
        mesh.edges = vec![
            Edge { he:0, attr:() },
            Edge { he:1, attr:() },
            Edge { he:2, attr:() },
        ];
        mesh.faces = vec![ Face { he:0, attr:() } ];

        let areas = mesh.vertex_areas();
        // Mixed area for right triangle: angles at vertex 0 is 90°, areas = [0.25, 0.125, 0.125]
        assert!((areas[0] - 0.25).abs() < 1e-8);
        assert!((areas[1] - 0.125).abs() < 1e-8);
        assert!((areas[2] - 0.125).abs() < 1e-8);
    }

    #[test]
    fn test_topology_queries_triangle() {
        // Reuse same mesh
        let mut mesh = Mesh3D::<(),()>::new();
        mesh.vertices = vec![
            Vertex { he_out: 0, attr: Vector3::zero() },
            Vertex { he_out: 1, attr: Vector3::zero() },
            Vertex { he_out: 2, attr: Vector3::zero() },
        ];
        mesh.halfedges = vec![
            HalfEdge { from:0, twin:0, next:1, edge:0, face:0 },
            HalfEdge { from:1, twin:1, next:2, edge:1, face:0 },
            HalfEdge { from:2, twin:2, next:0, edge:2, face:0 },
        ];
        mesh.edges = vec![
            Edge { he:0, attr:() },
            Edge { he:1, attr:() },
            Edge { he:2, attr:() },
        ];
        mesh.faces = vec![ Face { he:0, attr:() } ];

        // Incident faces for vertex 0: face 0 thrice
        assert_eq!(mesh.vertex_incident_faces(0), vec![0,0,0]);
        // Neighbors for vertex 0: 1,2,0
        assert_eq!(mesh.vertex_neighbors(0), vec![1,2,0]);
        // Incident edges for vertex 0: 0,1,2
        assert_eq!(mesh.vertex_incident_edges(0), vec![0,1,2]);
    }
}