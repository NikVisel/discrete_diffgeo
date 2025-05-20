pub mod prelude {
    //! Common imports for ease of use

    pub use crate::error::Error;
    pub use crate::mesh::half_edge::{Mesh, Mesh3D};
    pub use crate::geometry::vector::Vector3;
    pub use crate::geometry::matrix::{Matrix2, Matrix3};
    pub use crate::operators::traits::Operator;
    pub use crate::operators::{
        Gradient,
        Divergence,
        Curl,
        Laplacian,
        MeanCurvatureNormal,
        GaussianCurvature,
        Jacobian,
        ShapeOperator,
    };
    pub use crate::algorithms::{
        Smoothing,
        Geodesic,
    };
}