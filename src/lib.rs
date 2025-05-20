// {{ ... }}
pub mod error;
pub mod mesh;
pub mod operators;
pub mod geometry;
pub mod io;
pub mod algorithms;

pub use prelude::*;

/// Prelude: import common traits and types
pub mod prelude {
    //! Common imports for ease of use
    pub use crate::error::Error;
    pub use crate::mesh::half_edge::Mesh;
    pub use crate::operators::traits::Operator;
}
// {{ ... }}