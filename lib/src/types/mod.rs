mod contribution_activity;
mod error;

pub use contribution_activity::ContributionActivity;
pub use error::Error;

pub type Result<T> = core::result::Result<T, Error>;
