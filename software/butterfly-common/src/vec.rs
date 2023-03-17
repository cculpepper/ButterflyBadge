
use core::ops::{ Sub, Add };
use libm::{sinf, cosf, sqrtf};

pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector<const D: usize> {
    value: [f32; D]
}

impl<const D: usize> AsRef<[f32; D]> for Vector<D> {
    fn as_ref(&self) -> &[f32; D] {
        &self.value
    }
}

impl<const D: usize> AsMut<[f32; D]> for Vector<D> {
    fn as_mut(&mut self) -> &mut [f32; D] {
        &mut self.value
    }
}

impl<const D: usize> Vector<D> {

    pub fn pairwise_ref<'a>(&'a self, rhs: &'a Self) -> impl Iterator<Item = (&f32,&f32)> {
        self.value.iter().zip(rhs.value.iter())
    }

    pub fn pairwise_mut<'a,'b>(&'a mut self, rhs: &'b mut Self)
        -> impl Iterator<Item = (&'a mut f32, &'b mut f32)>
    {
        self.as_mut().iter_mut().zip(rhs.value.iter_mut())
    }

    pub fn pairwise(self, rhs: Self) -> impl Iterator<Item = (f32, f32)> {
        self.value.into_iter().zip(rhs.value.into_iter())
    }

}

impl<const D: usize> Sub for Vector<D> {
    type Output = Vector<D>;
    fn sub(mut self, mut rhs: Self) -> Self::Output {
        for (x1,x2) in self.pairwise_mut(&mut rhs)  {
            *x1 -= *x2;
        }
        self
    }
}

impl<const D: usize> Add for Vector<D> {
    type Output = Vector<D>;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        for (x1,x2) in self.pairwise_mut(&mut rhs)  {
            *x1 += *x2;
        }
        self
    }
}

impl<const D: usize> Vector<D>
{
    pub fn zero() -> Vector<D> {
        Self { value: [0.; D] }
    }

    pub fn magnitude(self) -> f32 {
        sqrtf(self.magnitude_sq())
    }

    pub fn magnitude_sq(self) -> f32 {
        self.value.iter().map(|a| a*a).sum()
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    pub fn distance_sq(self, rhs: Self) -> f32 {
        (self - rhs).magnitude_sq()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.pairwise(other).map(|(a1,a2)| a1*a2).sum()
    }

    pub fn scale(mut self, f: f32) -> Self {
        self.value.iter_mut().for_each(|x| *x *= f);
        self
    }

    pub fn scale_inv(self, f: f32) -> Self {
        self.scale(1./f)
    }

    /// negate (get the opposite direction) of the vector
    pub fn neg(mut self) -> Self {
        self.value.iter_mut().for_each(|x| *x *= -1.);
        self
    }

    /// returns the unit vector in the same direction and magnitude of the original vector
    pub fn unit(self) -> Option<(Self, f32)> {
        let magnitude = self.magnitude();

        if magnitude.is_normal() {
            Some((self.scale_inv(magnitude), magnitude))
        } else {
            None
        }
    }

    /// projection of self on rhs
    pub fn projection(self, rhs: Self) -> Self {
        self.scale(self.dot(rhs))
    }

    pub fn compwise_mul(mut self, rhs: Self) -> Self {
        self.compwise_op(rhs, |a,b| a * b);
        self
    }

    pub fn compwise_op(&mut self, rhs: Self, op: impl Fn(f32, f32) -> f32) {
        (0..D).into_iter().for_each(|idx| {
            self.value[idx] = op(self.value[idx], rhs.value[idx]);
        })
    }


}

impl<const D: usize> From<[f32;D]> for Vector<D> {
    fn from(value: [f32;D]) -> Self {
        Self { value }
    }
}

impl<const D: usize> From<Vector<D>> for [f32; D] {
    fn from(value: Vector<D>) -> [f32; D] {
        value.value
    }
}

impl Vector<1> {
    pub fn x(&self) -> f32 { self.value[0] }
}


impl Vector<2> {
    pub fn x(&self) -> f32 { self.value[0] }
    pub fn y(&self) -> f32 { self.value[1] }

    pub const fn new(x: f32, y: f32) -> Self {
        Self { value: [x,y] }
    }
    
    pub fn angled(angle: f32) -> Self {
        Self { value: [ cosf(angle), sinf(angle) ]}
    }

}

impl Vector<3> {
    pub fn x(&self) -> f32 { self.value[0] }
    pub fn y(&self) -> f32 { self.value[1] }
    pub fn z(&self) -> f32 { self.value[2] }

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { value: [x,y,z] }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let Vector { value: [a1,a2,a3]} = self;
        let Vector { value: [b1,b2,b3]} = rhs;

        let i = a2*b3-a3*b2;
        let j = a1*b3-a3*b1;
        let k = a1*b2-a2*b1;
        
        Vec3::new( i,j,k)
    }

}

#[cfg(test)]
mod tests {
    use core::f32::consts::SQRT_2;

    use super::*;

    macro_rules! assert_close {
        ($a:tt,$b:tt,$c:expr) => {
            let d = ($a - $b).abs();
            assert!(d < $c)
        };
    }

    fn test_dist_helper<const D: usize>(p1: Vector<D>, p2: Vector<D>, expect: f32) {
        let lamda = 0.000000001;
        let actual = p1.distance(p2);
        assert_close!(expect, actual, lamda);
    }

    #[test]
    fn test_distance1() {
        let p1  = Vec2::new(0.,0.);
        let p2 = Vec2::new(1.,0.);
        test_dist_helper(p1, p2, 1.)
    }

    #[test]
    fn test_distance2() {
        test_dist_helper(Vec2::new(0., 0.), [1.,1.].into(), SQRT_2);
    }

}
