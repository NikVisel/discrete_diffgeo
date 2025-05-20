
use crate::mesh::half_edge::{Mesh, Vertex, HalfEdge, Edge, Face};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;


pub fn read_obj(path: &Path) -> crate::error::Result<Mesh<[f64;3], (), ()>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut mesh = Mesh::new();
    let mut face_indices: Vec<[usize; 3]> = Vec::new();
    for line in reader.lines() {
        let l = line.map_err(|e| crate::error::Error::Io(e))?;
        let parts: Vec<_> = l.split_whitespace().collect();
        if parts.is_empty() { continue; }
        match parts[0] {
            "v" if parts.len() >= 4 => {
                let x: f64 = parts[1].parse().map_err(|e: std::num::ParseFloatError| crate::error::Error::Parse(e.to_string()))?;
                let y: f64 = parts[2].parse().map_err(|e: std::num::ParseFloatError| crate::error::Error::Parse(e.to_string()))?;
                let z: f64 = parts[3].parse().map_err(|e: std::num::ParseFloatError| crate::error::Error::Parse(e.to_string()))?;
                mesh.vertices.push(Vertex { he_out: 0, attr: [x, y, z] });
            }
            "f" if parts.len() >= 4 => {
                let idx: Vec<usize> = parts[1..4].iter()
                    .map(|p| p.split('/').next().unwrap().parse::<usize>().map_err(|e: std::num::ParseIntError| crate::error::Error::Parse(e.to_string())))
                    .collect::<crate::error::Result<Vec<_>>>()?;
                face_indices.push([idx[0]-1, idx[1]-1, idx[2]-1]);
            }
            _ => {}
        }
    }
    // Build half-edge topology
    let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();
    for inds in face_indices {
        let face_id = mesh.faces.len();
        mesh.faces.push(Face { he: 0, attr: () });
        let he_base = mesh.halfedges.len();
        // three half-edges
        mesh.halfedges.push(HalfEdge { from: inds[0], twin: 0, next: he_base+1, edge: 0, face: face_id });
        mesh.halfedges.push(HalfEdge { from: inds[1], twin: 0, next: he_base+2, edge: 0, face: face_id });
        mesh.halfedges.push(HalfEdge { from: inds[2], twin: 0, next: he_base,   edge: 0, face: face_id });
        mesh.faces[face_id].he = he_base;
        // link twins and edges
        for k in 0..3 {
            let curr = he_base + k;
            let next_he = mesh.halfedges[curr].next;
            let from = mesh.halfedges[curr].from;
            let to = mesh.halfedges[next_he].from;
            if let Some(&opp) = edge_map.get(&(to, from)) {
                mesh.halfedges[curr].twin = opp;
                mesh.halfedges[opp].twin = curr;
                // reuse edge
                let e_id = mesh.halfedges[opp].edge;
                mesh.halfedges[curr].edge = e_id;
            } else {
                // new edge
                let e_id = mesh.edges.len();
                mesh.edges.push(Edge { he: curr, attr: () });
                mesh.halfedges[curr].edge = e_id;
                edge_map.insert((from, to), curr);
            }
        }
    }
    // assign outgoing half-edge per vertex
    for (he_idx, he) in mesh.halfedges.iter().enumerate() {
        mesh.vertices[he.from].he_out = he_idx;
    }
    Ok(mesh)
}

pub fn write_obj(mesh: &Mesh<[f64;3], (), ()>, path: &Path) -> crate::error::Result<()> {
    let mut file = File::create(path)?;
    // write vertices
    for v in &mesh.vertices {
        writeln!(file, "v {} {} {}", v.attr[0], v.attr[1], v.attr[2])?;
    }
    // write faces
    for face in &mesh.faces {
        let he0 = face.he;
        let he1 = mesh.halfedges[he0].next;
        let he2 = mesh.halfedges[he1].next;
        let a = mesh.halfedges[he0].from + 1;
        let b = mesh.halfedges[he1].from + 1;
        let c = mesh.halfedges[he2].from + 1;
        writeln!(file, "f {} {} {}", a, b, c)?;
    }
    Ok(())
}