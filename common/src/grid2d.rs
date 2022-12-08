use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid2D<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: FromStr> Grid2D<T>
where
    <T as FromStr>::Err: Debug,
{
    pub fn from_str_delimeted(s: &str, p: &str) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut data = vec![];
        for line in s.trim().lines() {
            rows = line.len();
            for c in line.split(p).filter(|s| !s.is_empty()) {
                data.push(c.parse().unwrap())
            }
            cols += 1;
        }

        Self { rows, cols, data }
    }
}

impl<T: Copy> Grid2D<T> {
    pub fn transpose(&self) -> Self {
        let mut data = vec![];

        let mut i = 0;
        while i < self.data.len() {
            let q = i % self.rows;
            let r = i / self.rows;

            data.push(self.data[q * self.cols + r]);

            i += 1;
        }

        let rows = self.cols;
        let cols = self.rows;

        Self { rows, cols, data }
    }
}

impl<T> Grid2D<T> {
    pub fn map_rows<U, F: Fn(&[T]) -> Vec<U>>(&self, f: F) -> Grid2D<U> {
        let mut data = vec![];

        let mut r = 0;
        while r < self.rows {
            let mut row = f(&self.data[(r * self.cols)..((r + 1) * self.cols)]);
            data.append(&mut row);
            r += 1;
        }

        let rows = self.rows;
        let cols = self.cols;

        Grid2D::<U> { rows, cols, data }
    }

    pub fn values(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: core::ops::BitOr<Output = T> + Copy> core::ops::BitOr for Grid2D<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let rows = self.rows;
        let cols = self.cols;

        let mut data: Vec<T> = vec![];
        let mut i = 0;
        while i < self.data.len() {
            data.push(self.data[i] | rhs.data[i]);
            i += 1;
        }

        Self { rows, cols, data }
    }
}

impl<T: core::ops::Add<Output = T> + Copy> core::ops::Add for Grid2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let rows = self.rows;
        let cols = self.cols;

        let mut data: Vec<T> = vec![];
        let mut i = 0;
        while i < self.data.len() {
            data.push(self.data[i] + rhs.data[i]);
            i += 1;
        }

        Self { rows, cols, data }
    }
}

impl<T: core::ops::Mul<Output = T> + Copy> core::ops::Mul for Grid2D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows = self.rows;
        let cols = self.cols;

        let mut data: Vec<T> = vec![];
        let mut i = 0;
        while i < self.data.len() {
            data.push(self.data[i] * rhs.data[i]);
            i += 1;
        }

        Self { rows, cols, data }
    }
}
