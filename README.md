# discrete_differential

[![CI](https://github.com/NikVisel/discrete_diffgeo/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/NikVisel/discrete_diffgeo/actions/workflows/ci.yml)


A Rust crate for discrete differential geometry operators and algorithms on half-edge meshes.

## Features

- Mesh I/O: OBJ, OFF, PLY
- Differential Operators: 
  - gradient
  - divergence
  - curl
  - Laplacian
  - mean & Gaussian curvature
  - per-face Jacobian tensor
  - per-vertex shape operator (Weingarten map)
- Algorithms: geodesic distances (Dijkstra), uniform Laplacian smoothing

## Usage Example

```rust
use discrete_differential::prelude::*;

fn main() -> anyhow::Result<()> {
    // Load mesh (Mesh3D alias from prelude)
    let mesh: Mesh3D<(), ()> = Mesh::load_off("mesh.off")?;
    // Extract positions as Vector3
    let positions: Vec<Vector3> = mesh.vertices.iter().map(|v| v.attr).collect();

    // Scalar field example: x-coordinate
    let scalar: Vec<f64> = positions.iter().map(|p| p.x).collect();
    let grad = Gradient.apply(&mesh, &scalar);
    println!("Gradient on first face: {:?}", grad[0]);

    // Vector field divergence
    let div = Divergence.apply(&mesh, &grad);
    println!("Divergence on first vertex: {}", div[0]);

    // Per-face Jacobian of vector field
    let jac = Jacobian.apply(&mesh, &grad);
    println!("Jacobian on first face: {:?}", jac[0]);

    // Per-vertex shape operator
    let shape = ShapeOperator.apply(&mesh, &positions);
    println!("Shape operator at first vertex: {:?}", shape[0]);

    // Curvature
    let Hn = MeanCurvatureNormal.apply(&mesh, &positions);
    let K = GaussianCurvature.apply(&mesh, &positions);
    println!("Mean curvature normal at vertex 0: {:?}", Hn[0]);
    println!("Gaussian curvature at vertex 0: {}", K[0]);

    // Geodesic distances
    let dists = Geodesic::dijkstra(&mesh, 0);
    println!("Geodesic dist to vertex 1: {}", dists[1]);

    // Smoothing
    let mut pos2 = positions.clone();
    Smoothing::laplacian_smoothing(&mesh, &mut pos2, 10, 0.1);
    println!("Smoothed position of vertex 0: {:?}", pos2[0]);

    Ok(())
}

## Pixar USD Support

To enable USD I/O, install Pixar USD so that `pkg-config` (on Linux/macOS) or `vcpkg` (on Windows) can locate the `pxr_usd` library. Then build or test with the `usd` feature:
```bash
# Linux/macOS: install USD via package manager or from source
# Windows: install USD via vcpkg
cargo test --features usd
```
If you need to point to a custom install, set `PKG_CONFIG_PATH` or configure `vcpkg` accordingly. No additional environment variables are required.

## Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.
