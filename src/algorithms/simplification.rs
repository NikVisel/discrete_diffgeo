use crate::mesh::half_edge::{Mesh3D, VertexId, EdgeId};
use crate::geometry::vector::Vector3;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Mesh simplification algorithms
pub struct Simplification;

#[derive(Copy, Clone, PartialEq)]
struct EdgeCollapse {
    cost: f64,
    edge: EdgeId,
}

impl Eq for EdgeCollapse {}

impl Ord for EdgeCollapse {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for EdgeCollapse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Simplification {
    /// Simplify the mesh by collapsing edges until the target number of vertices is reached.
    pub fn edge_collapse(mesh: &mut Mesh3D<(), ()>, target_vertex_count: usize) {
        let mut heap = BinaryHeap::new();
        let mut edge_costs = vec![f64::INFINITY; mesh.edges.len()];

        // Compute initial edge collapse costs
        for (e_id, edge) in mesh.edges.iter().enumerate() {
            let cost = Self::compute_edge_cost(mesh, e_id);
            edge_costs[e_id] = cost;
            heap.push(EdgeCollapse { cost, edge: e_id });
        }

        while mesh.vertices.len() > target_vertex_count {
            if let Some(EdgeCollapse { cost: _, edge: e_id }) = heap.pop() {
                if edge_costs[e_id] == f64::INFINITY {
                    continue;
                }
                Self::collapse_edge(mesh, e_id);
                edge_costs[e_id] = f64::INFINITY;

                // Update costs for affected edges
                for &he in &mesh.edges[e_id].he {
                    let twin = mesh.halfedges[he].twin;
                    let e_twin = mesh.halfedges[twin].edge;
                    let cost = Self::compute_edge_cost(mesh, e_twin);
                    edge_costs[e_twin] = cost;
                    heap.push(EdgeCollapse { cost, edge: e_twin });
                }
            }
        }
    }

    /// Compute the cost of collapsing an edge
    fn compute_edge_cost(mesh: &Mesh3D<(), ()>, e_id: EdgeId) -> f64 {
        let he = mesh.edges[e_id].he;
        let v0 = mesh.halfedges[he].from;
        let v1 = mesh.halfedges[mesh.halfedges[he].twin].from;
        let p0 = mesh.vertices[v0].attr;
        let p1 = mesh.vertices[v1].attr;
        (p0 - p1).norm()
    }

    /// Collapse an edge by removing one of its vertices and updating the mesh topology
    fn collapse_edge(mesh: &mut Mesh3D<(), ()>, e_id: EdgeId) {
        let he = mesh.edges[e_id].he;
        let v0 = mesh.halfedges[he].from;
        let v1 = mesh.halfedges[mesh.halfedges[he].twin].from;

        // Update vertex positions
        let p0 = mesh.vertices[v0].attr;
        let p1 = mesh.vertices[v1].attr;
        let new_pos = (p0 + p1) * 0.5;
        mesh.vertices[v0].attr = new_pos;

        // Remove vertex v1 and update half-edges
        let mut he = mesh.vertices[v1].he_out;
        loop {
            let next_he = mesh.halfedges[he].next;
            let twin = mesh.halfedges[he].twin;
            let from = mesh.halfedges[twin].from;
            mesh.halfedges[twin].from = v0;
            mesh.halfedges[he].from = v0;
            he = next_he;
            if he == mesh.vertices[v1].he_out {
                break;
            }
        }
        mesh.vertices.remove(v1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3;

    #[test]
    fn test_edge_collapse() {
        // Create a simple mesh with 4 vertices and 2 faces
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

        // Simplify the mesh to 3 vertices
        Simplification::edge_collapse(&mut mesh, 3);

        // Check the number of vertices
        assert_eq!(mesh.vertices.len(), 3);
    }
}
