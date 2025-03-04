use super::rgba::Rgba;

pub(crate) trait ContributionColour {
    fn get_colour(count: usize) -> Rgba;
}

/// The way GitLab visualises contribution activity
pub(crate) struct GitlabColourStyle {}

impl ContributionColour for GitlabColourStyle {
    fn get_colour(count: usize) -> Rgba {
        let string = match count {
            0 => "#ececefff",
            c if c < 10 => "#d2dcffff",
            c if c < 20 => "#7992f5ff",
            c if c < 30 => "#4e65cdff",
            _ => "#303470ff",
        };

        Rgba::try_from(string.to_owned()).unwrap() // todo: use const instead of unwrap at runtime
    }
}
