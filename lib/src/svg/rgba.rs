use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Debug)]
struct Rgba(u8, u8, u8, u8);

impl Rgba {
    fn interoplate(&self, other: Rgba, factor: f32) -> Rgba {
        let r = self.0 as f32 + factor * (other.0 as f32 - self.0 as f32);
        let g = self.1 as f32 + factor * (other.1 as f32 - self.1 as f32);
        let b = self.2 as f32 + factor * (other.2 as f32 - self.2 as f32);
        let a = self.3 as f32 + factor * (other.3 as f32 - self.3 as f32);
        Rgba(r as u8, g as u8, b as u8, a as u8)
    }
}

impl Add for Rgba {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rgba(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub for Rgba {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Rgba(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Mul<f32> for Rgba {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgba(
            (self.0 as f32 * rhs) as u8,
            (self.1 as f32 * rhs) as u8,
            (self.2 as f32 * rhs) as u8,
            (self.3 as f32 * rhs) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Rgba;

    #[test]
    fn arithmetic() {
        assert_eq!(Rgba(1, 2, 3, 4) + Rgba(4, 3, 2, 1), Rgba(5, 5, 5, 5));
        assert_eq!(Rgba(9, 9, 9, 9) - Rgba(4, 3, 2, 1), Rgba(5, 6, 7, 8));
        assert_eq!(Rgba(5, 10, 20, 50) * 7.5, Rgba(37, 75, 150, 255));
    }

    #[test]
    fn interpolate() {
        assert_eq!(
            Rgba(0, 0, 0, 0).interoplate(Rgba(2, 50, 100, 255), 0.5),
            Rgba(1, 25, 50, 127)
        );

        assert_eq!(
            Rgba(2, 50, 100, 255).interoplate(Rgba(0, 0, 0, 0), 0.5),
            Rgba(1, 25, 50, 127)
        );
    }
}
