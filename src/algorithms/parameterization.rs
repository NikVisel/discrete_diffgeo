use crate::mesh::half_edge::Mesh3D;
use crate::geometry::vector::Vector3;

/// 2D parameterization algorithms for surface meshes.

/// Projects 3D positions to 2D by discarding the Z coordinate.
/// Useful as a naive initial guess for more advanced methods.
pub fn project_xy(positions: &[Vector3]) -> Vec<[f64;2]> {
    positions.iter().map(|v| [v.x, v.y]).collect()
}

/// Tutte embedding: map boundary to unit circle and solve Laplacian for interior vertices.
/// Currently unimplemented.
pub fn tutte_parameterization(_mesh: &Mesh3D<(), ()>, _positions: &[Vector3]) -> Vec<[f64;2]> {
    // TODO: identify boundary loop and fix positions on unit circle
    //       assemble Laplacian system and solve sparse linear equations
    unimplemented!()
}