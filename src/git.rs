//! Git segment implementations for repository information display.
//!
//! Uses the git2 crate for native Git operations.

mod branch_status;
mod last_commit;
mod repo_name;
mod tracking;

pub use branch_status::BranchStatusSegment;
pub use last_commit::LastCommitSegment;
pub use repo_name::RepoNameSegment;
pub use tracking::TrackingSegment;

use git2::Repository;

/// Try to open a Git repository at or above the given path.
#[must_use]
pub fn open_repo(path: &std::path::Path) -> Option<Repository> {
    Repository::discover(path).ok()
}
