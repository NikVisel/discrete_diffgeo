use crate::error::Result;
use crate::io::traits::{LoadMesh, SaveMesh};
use crate::mesh::half_edge::{Mesh, Mesh3D};
use crate::geometry::vector::Vector3;
use std::path::Path;
use std::ffi::{CString, CStr};

include!(concat!(env!("OUT_DIR"), "/usd_bindings.rs"));

/// Support for loading and saving Pixar USD meshes.
pub struct USD;

impl<EA, FA> LoadMesh<Vector3, EA, FA> for Mesh3D<EA, FA> {
    fn load(path: &Path) -> Result<Mesh3D<EA, FA>> {
        let path_str = path.to_str().ok_or(Error::Parse(format!("Invalid path: {:?}", path)))?;
        let cpath = CString::new(path_str).unwrap();
        // Open USD stage
        let stage = unsafe { UsdStage_Open(cpath.as_ptr()) };
        if stage.is_null() {
            return Err(Error::Unsupported(format!("Failed to open USD stage: {}", path_str)));
        }
        // Get default prim and mesh
        let prim = unsafe { UsdStage_GetDefaultPrim(stage) };
        let usd_mesh = unsafe { UsdGeomMesh_Get(stage, prim) };
        // Read points
        let points_attr = unsafe { UsdGeomMesh_GetPointsAttr(usd_mesh) };
        let mut points_arr = VtArrayGfVec3f::new();
        unsafe { UsdAttribute_Get(&points_attr, &mut points_arr) };
        let count = unsafe { VtArrayGfVec3f_size(&points_arr) };
        let mut positions = Vec::with_capacity(count);
        for i in 0..count {
            let v = unsafe { VtArrayGfVec3f_get(&points_arr, i) };
            positions.push(Vector3::new(v.x as f64, v.y as f64, v.z as f64));
        }
        // Read face-vertex counts
        let counts_attr = unsafe { UsdGeomMesh_GetFaceVertexCountsAttr(usd_mesh) };
        let mut counts_arr = VtArraySize::new();
        unsafe { UsdAttribute_Get(&counts_attr, &mut counts_arr) };
        let fc = unsafe { VtArraySize_size(&counts_arr) };
        let mut face_counts = Vec::with_capacity(fc);
        for i in 0..fc {
            face_counts.push(unsafe { VtArraySize_get(&counts_arr, i) } as usize);
        }
        // Read face-vertex indices
        let indices_attr = unsafe { UsdGeomMesh_GetFaceVertexIndicesAttr(usd_mesh) };
        let mut idx_arr = VtArraySize::new();
        unsafe { UsdAttribute_Get(&indices_attr, &mut idx_arr) };
        let ni = unsafe { VtArraySize_size(&idx_arr) };
        let mut indices = Vec::with_capacity(ni);
        for i in 0..ni {
            indices.push(unsafe { VtArraySize_get(&idx_arr, i) } as usize);
        }
        // Build half-edge mesh from positions and faces
        let mesh = Mesh3D::from_vertices_and_faces(positions, &face_counts, &indices);
        Ok(mesh)
    }
}

impl<EA, FA> SaveMesh<Vector3, EA, FA> for Mesh3D<EA, FA> {
    fn save(mesh: &Mesh3D<EA, FA>, path: &Path) -> Result<()> {
        let path_str = path.to_str().ok_or(Error::Parse(format!("Invalid path: {:?}", path)))?;
        let cpath = CString::new(path_str).unwrap();
        // Create new USD stage
        let stage = unsafe { UsdStage_CreateNew(cpath.as_ptr()) };
        if stage.is_null() {
            return Err(Error::Unsupported(format!("Failed to create USD stage: {}", path_str)));
        }
        // Define mesh prim at root
        let root_path = SdfPath::from_literal("/Mesh");
        let usd_mesh = unsafe { UsdGeomMesh_Define(stage, root_path) };
        // Set points
        let mut points_arr = VtArrayGfVec3f::new();
        for v in &mesh.vertices {
            let p = v.attr;
            points_arr.push(GfVec3f { x: p.x as f32, y: p.y as f32, z: p.z as f32 });
        }
        let points_attr = unsafe { UsdGeomMesh_GetPointsAttr(usd_mesh) };
        unsafe { UsdAttribute_Set(&points_attr, &points_arr) };
        // Set face-vertex counts and indices
        let mut counts_arr = VtArraySize::new();
        let mut indices_arr = VtArraySize::new();
        for poly in mesh.face_vertex_counts() {
            counts_arr.push(poly as usize);
        }
        for idx in mesh.face_vertex_indices() {
            indices_arr.push(idx as usize);
        }
        let counts_attr = unsafe { UsdGeomMesh_GetFaceVertexCountsAttr(usd_mesh) };
        unsafe { UsdAttribute_Set(&counts_attr, &counts_arr) };
        let indices_attr = unsafe { UsdGeomMesh_GetFaceVertexIndicesAttr(usd_mesh) };
        unsafe { UsdAttribute_Set(&indices_attr, &indices_arr) };
        // Save stage
        unsafe { UsdStage_Save(stage) };
        Ok(())
    }
}

#[cfg(all(test, feature = "usd"))]
mod tests {
    use tempfile::tempdir;
    use anyhow::Result;
    use crate::prelude::*;
    use crate::io::traits::{LoadMesh, SaveMesh};

    #[test]
    fn usd_roundtrip() -> Result<()> {
        // Simple triangle mesh
        let positions = vec![
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ];
        let face_counts = vec![3];
        let indices = vec![0, 1, 2];
        let mesh = Mesh3D::from_vertices_and_faces(positions.clone(), &face_counts, &indices);

        let dir = tempdir()?;
        let file = dir.path().join("test.usd");
        SaveMesh::save(&mesh, &file)?;
        let loaded = LoadMesh::load(&file)?;

        assert_eq!(loaded.vertices.len(), mesh.vertices.len());
        for (a, b) in loaded.vertices.iter().zip(mesh.vertices.iter()) {
            let pa = a.attr;
            let pb = b.attr;
            assert!((pa - pb).norm() < 1e-6);
        }
        assert_eq!(loaded.face_vertex_counts(), mesh.face_vertex_counts());
        assert_eq!(loaded.face_vertex_indices(), mesh.face_vertex_indices());
        Ok(())
    }
}
