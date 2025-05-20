use crate::geometry::vector::Vector3;

/// 2x2 matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
    pub m: [[f64;2];2],
}

impl Matrix2 {
    /// Construct Matrix2 from elements
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { m: [[a, b], [c, d]] }
    }
    /// Identity matrix
    pub fn identity() -> Self { Self::new(1.0, 0.0, 0.0, 1.0) }
    /// Determinant
    pub fn det(&self) -> f64 { self.m[0][0]*self.m[1][1] - self.m[0][1]*self.m[1][0] }
    /// Inverse
    pub fn inverse(&self) -> Option<Self> {
        let d = self.det();
        if d.abs() < 1e-12 { return None; }
        let inv = 1.0 / d;
        Some(Self::new(
            self.m[1][1] * inv,
            -self.m[0][1] * inv,
            -self.m[1][0] * inv,
            self.m[0][0] * inv,
        ))
    }
}

/// 3x3 matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
    pub m: [[f64;3];3],
}

impl Matrix3 {
    /// Construct Matrix3 from rows
    pub fn new(m: [[f64;3];3]) -> Self { Self { m } }
    /// Identity
    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }
    /// Transpose
    pub fn transpose(&self) -> Self {
        let mut t = [[0.0;3];3];
        for i in 0..3 { for j in 0..3 { t[i][j] = self.m[j][i]; }}
        Self::new(t)
    }
    /// Determinant
    pub fn det(&self) -> f64 {
        let m = &self.m;
        m[0][0]*(m[1][1]*m[2][2] - m[1][2]*m[2][1])
        - m[0][1]*(m[1][0]*m[2][2] - m[1][2]*m[2][0])
        + m[0][2]*(m[1][0]*m[2][1] - m[1][1]*m[2][0])
    }
    /// Inverse
    pub fn inverse(&self) -> Option<Self> {
        let d = self.det(); if d.abs() < 1e-12 { return None; }
        let m = &self.m;
        let cofactor = |i, j| {
            let mut sub = [[0.0;2];2];
            let (si, sj) = ( (0..3).filter(|&x| x!=i).collect::<Vec<_>>(), (0..3).filter(|&y| y!=j).collect::<Vec<_>>() );
            for ii in 0..2 { for jj in 0..2 {
                sub[ii][jj] = m[si[ii]][sj[jj]];
            }}
            let mat2 = Matrix2::new(sub[0][0], sub[0][1], sub[1][0], sub[1][1]);
            ((i+j)%2==0) as i32 as f64 * mat2.det() - (((i+j)%2)!=0) as i32 as f64 * mat2.det()
        };
        let mut inv = [[0.0;3];3];
        for i in 0..3 { for j in 0..3 { inv[j][i] = cofactor(i,j) / d; }}
        Some(Self::new(inv))
    }
    /// Multiply by Vector3
    pub fn mul_vec(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.m[0][0]*v.x + self.m[0][1]*v.y + self.m[0][2]*v.z,
            self.m[1][0]*v.x + self.m[1][1]*v.y + self.m[1][2]*v.z,
            self.m[2][0]*v.x + self.m[2][1]*v.y + self.m[2][2]*v.z,
        )
    }
}

impl std::ops::Mul for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Matrix3 {
        let mut r = [[0.0;3];3];
        for i in 0..3 { for j in 0..3 { for k in 0..3 {
            r[i][j] += self.m[i][k] * rhs.m[k][j];
        }}}
        Matrix3::new(r)
    }
}

impl std::ops::Mul<f64> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: f64) -> Matrix3 {
        let mut r = [[0.0;3];3];
        for i in 0..3 { for j in 0..3 {
            r[i][j] = self.m[i][j] * rhs;
        }}
        Matrix3::new(r)
    }
}

impl std::ops::Mul<Matrix3> for f64 {
    type Output = Matrix3;
    fn mul(self, rhs: Matrix3) -> Matrix3 {
        rhs * self
    }
}

impl std::ops::Add for Matrix3 {
    type Output = Matrix3;
    fn add(self, rhs: Matrix3) -> Matrix3 {
        let mut r = [[0.0;3];3];
        for i in 0..3 { for j in 0..3 {
            r[i][j] = self.m[i][j] + rhs.m[i][j];
        }}
        Matrix3::new(r)
    }
}