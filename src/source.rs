pub enum Source {
    GithubUser(String),
    GitlabUser(String),
}

pub trait DataSource {
    fn fetch(&self, source: Source) -> String;
}

struct ReqwestDataSource {}

impl DataSource for ReqwestDataSource {
    fn fetch(&self, source: Source) -> String {
        todo!("Make HTTP request")
    }
}

#[cfg(test)]
pub struct LocalDataSource {}

#[cfg(test)]
impl DataSource for LocalDataSource {
    fn fetch(&self, source: Source) -> String {
        let fixture_path = match source {
            Source::GithubUser(_) => "fixtures/github.html",
            Source::GitlabUser(_) => "fixtures/gitlab.json",
        };

        std::fs::read_to_string(fixture_path).expect("Unable to read file")
    }
}
