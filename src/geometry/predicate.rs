use crate::geometry::vector::Vector3;

/// 2D orientation: positive if ABC is counter-clockwise in XY plane
pub fn orient2d(a: &Vector3, b: &Vector3, c: &Vector3) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// 3D orientation: signed volume of tetrahedron ABCD
pub fn orient3d(a: &Vector3, b: &Vector3, c: &Vector3, d: &Vector3) -> f64 {
    let ab = Vector3::new(b.x - a.x, b.y - a.y, b.z - a.z);
    let ac = Vector3::new(c.x - a.x, c.y - a.y, c.z - a.z);
    let ad = Vector3::new(d.x - a.x, d.y - a.y, d.z - a.z);
    ab.dot(&ac.cross(&ad))
}