use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, MulAssign, DivAssign, Index, IndexMut};
use rand::RngExt;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e.iter().all(|x| x.abs() < s)
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();          
        Self { e: [rng.random(), rng.random(), rng.random()] }  
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        Self { e: [
            rng.random_range(min..max),     
            rng.random_range(min..max),
            rng.random_range(min..max),
        ]}
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.e[0]*v.e[0] + self.e[1]*v.e[1] + self.e[2]*v.e[2]
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[1]*v.e[2] - self.e[2]*v.e[1],
            self.e[2]*v.e[0] - self.e[0]*v.e[2],
            self.e[0]*v.e[1] - self.e[1]*v.e[0],
        ]}
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::rng();
        loop {
            let p = Vec3::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0]+other.e[0], self.e[1]+other.e[1], self.e[2]+other.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0]-other.e[0], self.e[1]-other.e[1], self.e[2]-other.e[2]] }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0]*other.e[0], self.e[1]*other.e[1], self.e[2]*other.e[2]] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 { e: [self.e[0]*t, self.e[1]*t, self.e[2]*t] }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}
