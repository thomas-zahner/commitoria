#[derive(PartialEq, Eq, Debug)]
pub struct ContributionActivity {}

pub trait DataSource {
    fn fetch() -> ContributionActivity;
}

pub struct Github {}

impl DataSource for Github {
    fn fetch() -> ContributionActivity {
        ContributionActivity {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github() {
        let result = Github::fetch();
        assert_eq!(result, ContributionActivity {});
    }
}
