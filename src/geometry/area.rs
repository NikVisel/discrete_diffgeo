use crate::geometry::vector::Vector3;

/// Compute area of triangle ABC
pub fn triangle(a: &Vector3, b: &Vector3, c: &Vector3) -> f64 {
    let u = Vector3::new(b.x - a.x, b.y - a.y, b.z - a.z);
    let v = Vector3::new(c.x - a.x, c.y - a.y, c.z - a.z);
    0.5 * u.cross(&v).norm()
}

/// Per-vertex barycentric area: one-third of triangle area for each vertex
pub fn barycentric_area(a: &Vector3, b: &Vector3, c: &Vector3) -> [f64;3] {
    let area = triangle(a, b, c) / 3.0;
    [area, area, area]
}

/// Angle between two vectors
fn angle(u: &Vector3, v: &Vector3) -> f64 {
    let dot = u.dot(v);
    let nu = u.norm();
    let nv = v.norm();
    (dot/(nu*nv)).clamp(-1.0,1.0).acos()
}

/// Per-vertex mixed (Voronoi) area of triangle ABC
pub fn mixed_area(a: &Vector3, b: &Vector3, c: &Vector3) -> [f64;3] {
    let area_total = triangle(a, b, c);
    // edge vectors
    let u = *b - *a;
    let v = *c - *a;
    let w = *a - *b;
    let x = *c - *b;
    let y = *a - *c;
    let z = *b - *c;
    // angles
    let alpha = angle(&u, &v);
    let beta = angle(&w, &x);
    let gamma = angle(&y, &z);
    // obtuse triangle case
    if alpha >= std::f64::consts::FRAC_PI_2 {
        return [area_total/2.0, area_total/4.0, area_total/4.0];
    } else if beta >= std::f64::consts::FRAC_PI_2 {
        return [area_total/4.0, area_total/2.0, area_total/4.0];
    } else if gamma >= std::f64::consts::FRAC_PI_2 {
        return [area_total/4.0, area_total/4.0, area_total/2.0];
    }
    // acute triangle: Voronoi region
    let cot_beta = w.dot(&x) / w.cross(&x).norm();
    let cot_gamma = y.dot(&z) / y.cross(&z).norm();
    let cot_alpha = u.dot(&v) / u.cross(&v).norm();
    let area0 = (u.dot(&u)*cot_gamma + v.dot(&v)*cot_beta) / 8.0;
    let area1 = (x.dot(&x)*cot_alpha + w.dot(&w)*cot_gamma) / 8.0;
    let area2 = (y.dot(&y)*cot_beta + z.dot(&z)*cot_alpha) / 8.0;
    [area0, area1, area2]
}
