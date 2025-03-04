use super::rgba::Rgba;

pub(crate) trait ContributionColour {
    fn get_colour(count: usize) -> Rgba;
}

/// The way GitLab visualises contribution activity
pub(crate) struct GitlabColourStyle {}

impl ContributionColour for GitlabColourStyle {
    fn get_colour(count: usize) -> Rgba {
        const WHITE_SMOKE: Rgba = Rgba::new(236, 236, 239, 255); // #ececefff
        const LAVENDER: Rgba = Rgba::new(210, 220, 255, 255); // #d2dcffff
        const CORNFLOWER_BLUE: Rgba = Rgba::new(121, 146, 245, 255); // #7992f5ff
        const ROYAL_BLUE: Rgba = Rgba::new(78, 101, 205, 255); // #4e65cdff
        const DARKSLATE_BLUE: Rgba = Rgba::new(48, 52, 112, 255); // #303470ff

        match count {
            0 => WHITE_SMOKE,
            c if c < 10 => LAVENDER,
            c if c < 20 => CORNFLOWER_BLUE,
            c if c < 30 => ROYAL_BLUE,
            _ => DARKSLATE_BLUE,
        }
    }
}
