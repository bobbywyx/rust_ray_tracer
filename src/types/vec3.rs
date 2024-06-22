use std::ops;

use crate::random;

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn zero_vec3() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3(
            random::random_f64(),
            random::random_f64(),
            random::random_f64(),
        )
    }

    pub fn random_with_bounds(min: f64, max: f64) -> Vec3 {
        Vec3(
            random::random_f64_with_bounds(min, max),
            random::random_f64_with_bounds(min, max),
            random::random_f64_with_bounds(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_with_bounds(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - *n * self.dot(n) * 2.0
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-(n).dot(uv)).min(1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = (-((1.0 - r_out_perp.length_squared()).abs().sqrt())) * *n;

        // print!("prep: {:?}\n",r_out_perp);
        // print!("parallel {:?}\n",r_out_parallel);

        // println!("prep len: {}",r_out_perp.length());
        // println!("para len: {}",r_out_parallel.length());
        // println!("sum len: {}",(r_out_parallel+r_out_perp).length());

        return r_out_perp + r_out_parallel;
        // return *n;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;
        Vec3(x, y, z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;
        Vec3(x, y, z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.0 * rhs;
        let y: f64 = self.1 * rhs;
        let z: f64 = self.2 * rhs;
        Vec3(x, y, z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let x: f64 = self * rhs.0;
        let y: f64 = self * rhs.1;
        let z: f64 = self * rhs.2;
        Vec3(x, y, z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let x: f64 = self.0 * rhs.0;
        let y: f64 = self.1 * rhs.1;
        let z: f64 = self.2 * rhs.2;
        Vec3(x, y, z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        let x = -self.0;
        let y = -self.1;
        let z = -self.2;
        Vec3(x, y, z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
