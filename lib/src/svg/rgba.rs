use std::{
    fmt::{self, Display},
    num::ParseIntError,
    ops::{Add, Mul, Sub},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rgba(u8, u8, u8, u8);

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub(crate) fn interpolate(&self, other: Rgba, factor: f32) -> Rgba {
        let r = self.0 as f32 + factor * (other.0 as f32 - self.0 as f32);
        let g = self.1 as f32 + factor * (other.1 as f32 - self.1 as f32);
        let b = self.2 as f32 + factor * (other.2 as f32 - self.2 as f32);
        let a = self.3 as f32 + factor * (other.3 as f32 - self.3 as f32);
        Rgba(
            r.round() as u8,
            g.round() as u8,
            b.round() as u8,
            a.round() as u8,
        )
    }
}

impl From<&Rgba> for String {
    fn from(value: &Rgba) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            value.0, value.1, value.2, value.3
        )
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = String::from(self);
        write!(f, "{}", x)
    }
}

#[derive(Debug, PartialEq)]
pub enum StringToRgbaError {
    InvalidLength,
    NotAscii,
    InvalidHexValue(ParseIntError),
}

impl From<ParseIntError> for StringToRgbaError {
    fn from(error: ParseIntError) -> Self {
        Self::InvalidHexValue(error)
    }
}

/// Try to convert a hexadecimal string to `Rgba`.
/// The string can optionally be prefixed with the `#` character.
impl TryFrom<String> for Rgba {
    type Error = StringToRgbaError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        use StringToRgbaError::*;
        fn convert_to_u8(value: &str) -> Result<u8, ParseIntError> {
            u8::from_str_radix(value, 16)
        }

        if value.get(0..1) == Some(&"#") {
            value = value[1..].into();
        }

        if !value.is_ascii() {
            Err(NotAscii)
        } else if value.len() != 8 {
            Err(InvalidLength)
        } else {
            Ok(Self(
                convert_to_u8(&value[0..2])?,
                convert_to_u8(&value[2..4])?,
                convert_to_u8(&value[4..6])?,
                convert_to_u8(&value[6..8])?,
            ))
        }
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
            Rgba(0, 0, 0, 0).interpolate(Rgba(2, 50, 100, 255), 0.5),
            Rgba(1, 25, 50, 128)
        );

        assert_eq!(
            Rgba(2, 50, 100, 255).interpolate(Rgba(0, 0, 0, 0), 0.5),
            Rgba(1, 25, 50, 128)
        );
    }

    #[test]
    fn into_string() {
        assert_eq!(
            String::from(&Rgba(202, 254, 0, 66)),
            String::from("#cafe0042")
        );
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Rgba::try_from("#cafe0042".to_string()),
            Ok(Rgba(202, 254, 0, 66))
        );
    }

    #[test]
    fn from_string_without_hashtag_prefix() {
        assert_eq!(
            Rgba::try_from("cafe0042".to_string()),
            Ok(Rgba(202, 254, 0, 66))
        );
    }

    #[test]
    fn from_string_uppercase() {
        assert_eq!(
            Rgba::try_from("ABCDEFAB".to_string()),
            Ok(Rgba(171, 205, 239, 171))
        );
    }

    #[test]
    fn double_conversion() {
        assert_eq!(
            String::from(&Rgba::try_from("#cafecafe".to_string()).unwrap()),
            "#cafecafe".to_string()
        );
        assert_eq!(
            Rgba::try_from(String::from(&Rgba(12, 34, 56, 78))).unwrap(),
            Rgba(12, 34, 56, 78)
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            format!("{}", Rgba::try_from("#12345678".to_string()).unwrap()),
            "#12345678".to_string()
        );
    }
}
