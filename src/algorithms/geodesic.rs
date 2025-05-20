use crate::mesh::half_edge::{VertexId, Mesh3D};
use crate::geometry::vector::Vector3;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Geodesic distance computations
pub struct Geodesic;

#[derive(Copy, Clone, PartialEq)]
struct State {
    dist: f64,
    v: VertexId,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.partial_cmp(&self.dist).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Geodesic {
    /// Compute shortest-path geodesic distances from `source` to all vertices using Dijkstra's algorithm over edge graph.
    pub fn dijkstra(mesh: &Mesh3D<(), ()>, source: VertexId) -> Vec<f64> {
        let n = mesh.vertices.len();
        let mut dist = vec![f64::INFINITY; n];
        let mut heap = BinaryHeap::new();
        dist[source] = 0.0;
        heap.push(State { dist: 0.0, v: source });
        while let Some(State { dist: d, v }) = heap.pop() {
            if d > dist[v] { continue; }
            // traverse outgoing edges around vertex v
            let mut he = mesh.vertices[v].he_out;
            loop {
                let to = mesh.halfedges[mesh.halfedges[he].twin].from;
                // edge length via Vector3
                let p: Vector3 = mesh.vertices[v].attr;
                let q: Vector3 = mesh.vertices[to].attr;
                let w = (p - q).norm();
                let nd = d + w;
                if nd < dist[to] {
                    dist[to] = nd;
                    heap.push(State { dist: nd, v: to });
                }
                // next outgoing half-edge
                he = mesh.halfedges[mesh.halfedges[he].twin].next;
                if he == mesh.vertices[v].he_out { break; }
            }
        }
        dist
    }
}