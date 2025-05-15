use serde::Deserialize;

use super::rgba::Rgba;

/// Information statistics used for calculating the cell colour of a day.
pub(crate) struct ContributionInfo {
    pub(crate) average_count_per_day: f32,
    pub(crate) count_today: usize,
}

/// The different strategies to calculate the colour of a cell
#[derive(Clone, Deserialize)]
pub enum ColourStrategy {
    /// The way GitLab visualises contribution activity
    GitlabStrategy,
    /// Smoothly interpolate from `inactive_colour` to `active_colour`
    InterpolationStrategy {
        inactive_colour: Rgba,
        active_colour: Rgba,
    },
}

impl ColourStrategy {
    pub(crate) fn get_colour(&self, info: ContributionInfo) -> Rgba {
        match self {
            ColourStrategy::GitlabStrategy => get_gitlab_colour(info),
            ColourStrategy::InterpolationStrategy {
                inactive_colour,
                active_colour,
            } => get_interpolated_colour(inactive_colour, active_colour, info),
        }
    }
}

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

fn get_gitlab_colour(info: ContributionInfo) -> Rgba {
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

fn get_interpolated_colour(
    inactive_colour: &Rgba,
    active_colour: &Rgba,
    info: ContributionInfo,
) -> Rgba {
    inactive_colour.interpolate(
        active_colour.clone(),
        f(info.count_today as f32, info.average_count_per_day),
    )
}

#[cfg(test)]
mod tests {
    use super::ColourStrategy::InterpolationStrategy;
    use crate::svg::contribution_colour::ContributionInfo;
    use crate::svg::rgba::Rgba;

    #[test]
    fn interpolation() {
        const INACTIVE: Rgba = Rgba::new(0, 0, 0, 0);
        const ACTIVE: Rgba = Rgba::new(255, 255, 255, 255);

        let style = InterpolationStrategy {
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
