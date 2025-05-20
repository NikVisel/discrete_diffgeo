

use crate::mesh::half_edge::{Mesh, Vertex, HalfEdge, Edge, Face};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;


pub fn read_ply(path: &Path) -> crate::error::Result<Mesh<[f64;3], (), ()>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut nv = 0usize;
    let mut nf = 0usize;
    // parse header
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 { break; }
        let tokens: Vec<_> = line.split_whitespace().collect();
        match tokens.as_slice() {
            ["element", "vertex", num] => nv = num.parse().unwrap_or(0),
            ["element", "face", num] => nf = num.parse().unwrap_or(0),
            ["end_header"] => break,
            _ => {}
        }
    }
    let mut mesh = Mesh::new();
    // read vertices
    for _ in 0..nv {
        line.clear();
        reader.read_line(&mut line)?;
        let p: Vec<f64> = line.split_whitespace()
            .take(3)
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        mesh.vertices.push(Vertex { he_out: 0, attr: [p[0], p[1], p[2]] });
    }
    // read faces
    let mut face_indices: Vec<[usize; 3]> = Vec::new();
    for _ in 0..nf {
        line.clear();
        reader.read_line(&mut line)?;
        let tok: Vec<_> = line.split_whitespace().collect();
        if tok.len() >= 4 && tok[0] == "3" {
            let i0 = tok[1].parse().unwrap_or(0);
            let i1 = tok[2].parse().unwrap_or(0);
            let i2 = tok[3].parse().unwrap_or(0);
            face_indices.push([i0, i1, i2]);
        }
    }
    // build half-edge topology (same as others)
    let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();
    for inds in face_indices {
        let fid = mesh.faces.len();
        mesh.faces.push(Face { he: 0, attr: () });
        let hb = mesh.halfedges.len();
        for k in 0..3 {
            mesh.halfedges.push(HalfEdge { from: inds[k], twin: 0, next: hb + (k + 1) % 3, edge: 0, face: fid });
        }
        mesh.faces[fid].he = hb;
        for k in 0..3 {
            let curr = hb + k;
            let next = mesh.halfedges[curr].next;
            let from = mesh.halfedges[curr].from;
            let to = mesh.halfedges[next].from;
            if let Some(&opp) = edge_map.get(&(to, from)) {
                mesh.halfedges[curr].twin = opp;
                mesh.halfedges[opp].twin = curr;
                let eid = mesh.halfedges[opp].edge;
                mesh.halfedges[curr].edge = eid;
            } else {
                let eid = mesh.edges.len();
                mesh.edges.push(Edge { he: curr, attr: () });
                mesh.halfedges[curr].edge = eid;
                edge_map.insert((from, to), curr);
            }
        }
    }
    for (i, he) in mesh.halfedges.iter().enumerate() {
        mesh.vertices[he.from].he_out = i;
    }
    Ok(mesh)
}