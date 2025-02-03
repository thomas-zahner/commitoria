pub(crate) trait ContributionLevel {
    fn get_contrib_level(count: usize) -> usize;
}

/// The way GitLab calculates contribution levels
pub(crate) struct GitlabContributionLevel {}

impl ContributionLevel for GitlabContributionLevel {
    fn get_contrib_level(count: usize) -> usize {
        match count {
            0 => 0,
            c if c < 10 => 1,
            c if c < 20 => 2,
            c if c < 30 => 3,
            _ => 4,
        }
    }
}
