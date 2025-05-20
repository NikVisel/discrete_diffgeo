use crate::operators::traits::Operator;
use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;
use crate::geometry::matrix::Matrix3;

/// Per-vertex shape operator (Weingarten map) as a 3×3 tensor.
/// Input: positions as Vec<Vector3> to compute normals.
pub struct ShapeOperator;

impl ShapeOperator {
    pub fn compute(mesh: &Mesh3D<(), ()>, positions: &[Vector3]) -> Vec<Matrix3> {
        let n = mesh.vertices.len();
        let mut result = vec![Matrix3::identity(); n];
        // compute face normals (unit)
        let mut face_normals = Vec::with_capacity(mesh.faces.len());
        for face in &mesh.faces {
            let h0 = face.he;
            let h1 = mesh.halfedges[h0].next;
            let h2 = mesh.halfedges[h1].next;
            let vs = [mesh.halfedges[h0].from, mesh.halfedges[h1].from, mesh.halfedges[h2].from];
            let p0 = positions[vs[0]];
            let p1 = positions[vs[1]];
            let p2 = positions[vs[2]];
            let u = p1 - p0;
            let v = p2 - p0;
            let normal = u.cross(&v).normalize();
            face_normals.push(normal);
        }
        // compute vertex normals (area-weighted average)
        let mut vertex_normals = vec![Vector3::zero(); n];
        for (f_id, face) in mesh.faces.iter().enumerate() {
            let fnorm = face_normals[f_id];
            for vid in mesh.face_vertices(f_id) {
                vertex_normals[vid] = vertex_normals[vid] + fnorm;
            }
        }
        for vn in &mut vertex_normals {
            *vn = vn.normalize();
        }
        // precompute cot weights per edge half sum
        let mut wmap = vec![0.0; mesh.edges.len()];
        for (e_id, edge) in mesh.edges.iter().enumerate() {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            let k0 = mesh.halfedges[mesh.halfedges[he0].next].from;
            let k1 = mesh.halfedges[mesh.halfedges[he1].next].from;
            let pi = positions[i];
            let pj = positions[j];
            let pk0 = positions[k0];
            let pk1 = positions[k1];
            let cot0 = (pi - pk0).dot(&(pj - pk0)) / (pi - pk0).cross(&(pj - pk0)).norm();
            let cot1 = (pj - pk1).dot(&(pi - pk1)) / (pj - pk1).cross(&(pi - pk1)).norm();
            wmap[e_id] = 0.5 * (cot0 + cot1);
        }
        // Actually, build per-vertex M and C accumulators
        let mut ms = vec![Matrix3::new([[0.0;3];3]); n];
        let mut cs = vec![Matrix3::new([[0.0;3];3]); n];
        for (e_id, edge) in mesh.edges.iter().enumerate() {
            let he0 = edge.he;
            let he1 = mesh.halfedges[he0].twin;
            let i = mesh.halfedges[he0].from;
            let j = mesh.halfedges[he1].from;
            let pi = positions[i];
            let pj = positions[j];
            let e_vec = pj - pi;
            let ni = vertex_normals[i];
            let nj = vertex_normals[j];
            let delta_n = nj - ni;
            let w = wmap[e_id];
            // accumulate
            // M += w * (e ⊗ e)
            let ee = Matrix3::new([
                [e_vec.x * e_vec.x, e_vec.x * e_vec.y, e_vec.x * e_vec.z],
                [e_vec.y * e_vec.x, e_vec.y * e_vec.y, e_vec.y * e_vec.z],
                [e_vec.z * e_vec.x, e_vec.z * e_vec.y, e_vec.z * e_vec.z],
            ]);
            let dn_e = Matrix3::new([
                [delta_n.x * e_vec.x, delta_n.x * e_vec.y, delta_n.x * e_vec.z],
                [delta_n.y * e_vec.x, delta_n.y * e_vec.y, delta_n.y * e_vec.z],
                [delta_n.z * e_vec.x, delta_n.z * e_vec.y, delta_n.z * e_vec.z],
            ]);
            ms[i] = ms[i] + (ee * w);
            cs[i] = cs[i] + (dn_e * w);
            // flip sign and direction for j
            let ej = pi - pj;
            let delta_nj = ni - nj;
            let ee_j = Matrix3::new([
                [ej.x * ej.x, ej.x * ej.y, ej.x * ej.z],
                [ej.y * ej.x, ej.y * ej.y, ej.y * ej.z],
                [ej.z * ej.x, ej.z * ej.y, ej.z * ej.z],
            ]);
            let dn_ej = Matrix3::new([
                [delta_nj.x * ej.x, delta_nj.x * ej.y, delta_nj.x * ej.z],
                [delta_nj.y * ej.x, delta_nj.y * ej.y, delta_nj.y * ej.z],
                [delta_nj.z * ej.x, delta_nj.z * ej.y, delta_nj.z * ej.z],
            ]);
            ms[j] = ms[j] + (ee_j * w);
            cs[j] = cs[j] + (dn_ej * w);
        }
        // finalize shape operator per vertex: S = inv(M) * C
        for i in 0..n {
            if let Some(inv_m) = ms[i].inverse() {
                result[i] = inv_m * cs[i];
            } else {
                result[i] = Matrix3::identity();
            }
        }
        result
    }
}

impl Operator<Vec<Vector3>, Vec<Matrix3>> for ShapeOperator {
    fn apply(&self, mesh: &Mesh3D<(), ()>, positions: &Vec<Vector3>) -> Vec<Matrix3> {
        ShapeOperator::compute(mesh, positions)
    }
}
