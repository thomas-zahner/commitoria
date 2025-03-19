use super::rgba::Rgba;

pub(crate) struct ContributionInfo {
    pub(crate) average_count_per_day: f32,
    pub(crate) count_today: usize,
}

pub(crate) trait ContributionColour {
    fn get_colour(&self, info: ContributionInfo) -> Rgba;
}

pub(crate) struct InterpolatedColourStyle {
    inactive_colour: Rgba,
    active_colour: Rgba,
}

impl InterpolatedColourStyle {
    /// This is a function returning a number ranging from 0 to 1,
    /// indicating how active a user was on the given day with `x` amount of contributions,
    /// with an average contribution count `a` over the last year.
    ///
    /// Formula: `-1 / ((1 / a) * x + 1) + 1` which can be simplified as `x / (x + a)`
    fn f(x: f32, a: f32) -> f32 {
        let divisor = x as f32 + a;
        match divisor {
            0.0 => 0.0,
            _ => x as f32 / divisor,
        }
    }
}

/// The way GitLab visualises contribution activity
pub(crate) struct GitlabColourStyle {}

impl ContributionColour for GitlabColourStyle {
    fn get_colour(&self, info: ContributionInfo) -> Rgba {
        const WHITE_SMOKE: Rgba = Rgba::new(236, 236, 239, 255); // #ececefff
        const LAVENDER: Rgba = Rgba::new(210, 220, 255, 255); // #d2dcffff
        const CORNFLOWER_BLUE: Rgba = Rgba::new(121, 146, 245, 255); // #7992f5ff
        const ROYAL_BLUE: Rgba = Rgba::new(78, 101, 205, 255); // #4e65cdff
        const DARKSLATE_BLUE: Rgba = Rgba::new(48, 52, 112, 255); // #303470ff

        match info.count_today {
            0 => WHITE_SMOKE,
            c if c < 10 => LAVENDER,
            c if c < 20 => CORNFLOWER_BLUE,
            c if c < 30 => ROYAL_BLUE,
            _ => DARKSLATE_BLUE,
        }
    }
}

impl ContributionColour for InterpolatedColourStyle {
    fn get_colour(&self, count: ContributionInfo) -> Rgba {
        self.inactive_colour.interpolate(
            self.active_colour.clone(),
            dbg!(Self::f(
                count.count_today as f32,
                count.average_count_per_day
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::svg::rgba::Rgba;

    use super::{ContributionColour, ContributionInfo, InterpolatedColourStyle};

    #[test]
    fn interpolated_colour_style() {
        const INACTIVE: Rgba = Rgba::new(236, 236, 239, 255); // #ececefff
        const ACTIVE: Rgba = Rgba::new(48, 52, 112, 255); // #303470ff

        let style = InterpolatedColourStyle {
            active_colour: ACTIVE,
            inactive_colour: INACTIVE,
        };

        let average_count_per_day = 5.0;
        assert_eq!(
            style.get_colour(ContributionInfo {
                average_count_per_day,
                count_today: 0,
            }),
            INACTIVE
        );

        assert_eq!(
            style.get_colour(ContributionInfo {
                average_count_per_day,
                count_today: 999999,
            }),
            ACTIVE
        );

        assert_eq!(
            style.get_colour(ContributionInfo {
                average_count_per_day: 0.0,
                count_today: 0,
            }),
            INACTIVE
        );
    }
}
