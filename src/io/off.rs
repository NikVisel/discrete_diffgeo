use crate::error::Result;

use crate::mesh::half_edge::{Mesh, Vertex, HalfEdge, Edge, Face};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;

/// OFF loader: ASCII only, triangular faces
// Removed trait impl for LoadMesh; see read_off below
pub fn load(path: &Path) -> Result<Mesh<[f64;3], (), ()>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim() != "OFF" {
            return Err(crate::error::Error::Parse("Invalid OFF header".into()));
        }
        line.clear();
        reader.read_line(&mut line)?;
        let parts: Vec<_> = line.split_whitespace().collect();
        let nv: usize = parts[0].parse().unwrap_or(0);
        let nf: usize = parts[1].parse().unwrap_or(0);
        // skip ne
        let mut mesh = Mesh::new();
        for _ in 0..nv {
            line.clear();
            reader.read_line(&mut line)?;
            let p: Vec<f64> = line.split_whitespace().map(|s: &str| s.parse::<f64>().unwrap_or(0.0)).collect();
            mesh.vertices.push(Vertex { he_out: 0, attr: [p[0], p[1], p[2]] });
        }
        let mut face_indices: Vec<[usize; 3]> = Vec::new();
        for _ in 0..nf {
            line.clear();
            reader.read_line(&mut line)?;
            let items: Vec<_> = line.split_whitespace().collect();
            if items[0] != "3" { continue; }
            face_indices.push([items[1].parse().unwrap(), items[2].parse().unwrap(), items[3].parse().unwrap()]);
        }
        // build topology (same as OBJ)
        let mut edge_map = HashMap::new();
        for inds in face_indices {
            let fid = mesh.faces.len();
            mesh.faces.push(Face { he: 0, attr: () });
            let hb = mesh.halfedges.len();
            for k in 0..3 {
                mesh.halfedges.push(HalfEdge { from: inds[k], twin: 0, next: hb+(k+1)%3, edge: 0, face: fid });
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

pub fn read_off(path: &Path) -> crate::error::Result<Mesh<[f64;3], (), ()>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    if line.trim() != "OFF" {
        return Err(crate::error::Error::Parse("Invalid OFF header".into()));
    }
    line.clear();
    reader.read_line(&mut line)?;
    let parts: Vec<_> = line.split_whitespace().collect();
    let nv: usize = parts[0].parse().unwrap_or(0);
    let nf: usize = parts[1].parse().unwrap_or(0);
    let mut mesh = Mesh::new();
    for _ in 0..nv {
        line.clear();
        reader.read_line(&mut line)?;
        let p: Vec<f64> = line.split_whitespace().map(|s: &str| s.parse::<f64>().unwrap_or(0.0)).collect();
        mesh.vertices.push(Vertex { he_out: 0, attr: [p[0], p[1], p[2]] });
    }
    let mut face_indices: Vec<[usize; 3]> = Vec::new();
    for _ in 0..nf {
        line.clear();
        reader.read_line(&mut line)?;
        let items: Vec<_> = line.split_whitespace().collect();
        if items[0] != "3" { continue; }
        face_indices.push([
            items[1].parse().unwrap(),
            items[2].parse().unwrap(),
            items[3].parse().unwrap()
        ]);
    }
    let mut edge_map = HashMap::new();
    for inds in face_indices {
        let fid = mesh.faces.len();
        mesh.faces.push(Face { he: 0, attr: () });
        let hb = mesh.halfedges.len();
        for k in 0..3 {
            mesh.halfedges.push(HalfEdge { from: inds[k], twin: 0, next: hb+(k+1)%3, edge: 0, face: fid });
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

pub fn write_off(mesh: &Mesh<[f64;3], (), ()>, path: &Path) -> crate::error::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "OFF")?;
    writeln!(file, "{} {} 0", mesh.vertices.len(), mesh.faces.len())?;
    for v in &mesh.vertices {
        writeln!(file, "{} {} {}", v.attr[0], v.attr[1], v.attr[2])?;
    }
    for f in &mesh.faces {
        let h0 = f.he;
        let h1 = mesh.halfedges[h0].next;
        let h2 = mesh.halfedges[h1].next;
        writeln!(file, "3 {} {} {}", mesh.halfedges[h0].from, mesh.halfedges[h1].from, mesh.halfedges[h2].from)?;
    }
    Ok(())
}