use crate::error::Result;
use crate::mesh::half_edge::Mesh;
use std::path::Path;

/// Trait for loading meshes from files
pub trait LoadMesh<VA, EA, FA> {
    fn load(path: &Path) -> Result<Mesh<VA, EA, FA>>;
}

/// Trait for saving meshes to files
pub trait SaveMesh<VA, EA, FA> {
    fn save(mesh: &Mesh<VA, EA, FA>, path: &Path) -> Result<()>;
}