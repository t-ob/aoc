use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }

    pub fn re(&self) -> &T {
        &self.re
    }

    pub fn im(&self) -> &T {
        &self.im
    }
}

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> Complex<T> {
    pub fn norm(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Hash> Hash for Complex<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.re, &self.im).hash(state);
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: std::ops::SubAssign> std::ops::SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Complex<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}
