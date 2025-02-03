pub(crate) trait ContributionLevel {
    fn get_contrib_level(count: usize) -> (usize, String);
}

/// The way GitLab calculates contribution levels
pub(crate) struct GitlabContributionLevel {}

impl ContributionLevel for GitlabContributionLevel {
    fn get_contrib_level(count: usize) -> (usize, String) {
        match count {
            0 => (0, "No contributions".into()),
            c if c < 10 => (1, "1-9 contributions".into()),
            c if c < 20 => (2, "10-19 contributions".into()),
            c if c < 30 => (3, "20-29 contributions".into()),
            _ => (4, "30+ contributions".into()),
        }
    }
}
