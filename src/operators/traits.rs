use crate::mesh::half_edge::Mesh3D;

/// A discrete operator on a mesh field.
pub trait Operator<Input, Output> {
    /// Apply the operator on the given field over the mesh.
    fn apply(&self, mesh: &Mesh3D<(), ()>, field: &Input) -> Output;
}