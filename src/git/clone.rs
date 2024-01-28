use std::{path::Path, process::Command};

use git2::build::RepoBuilder;
use log::debug;

use crate::{config::Config, errors::RequestError};

/// Attempt to clone a git repo at a given path
pub fn clone(path: &Path, url: String, branch: Option<String>) -> Result<(), RequestError> {
    let processed_url = url
        .replace(" ", "")
        .replace('"', "")
        .replace("'", "")
        .replace("\\", "")
        .replace(";", "")
        .replace("&", "")
        .replace("|", "")
        .trim()
        .to_string();

    let mut repobuilder = RepoBuilder::new();

    if let Some(branch_unw) = branch.clone() {
        let processed_branch = branch_unw
            .clone()
            .replace(" ", "")
            .replace('"', "")
            .replace("'", "")
            .replace("\\", "")
            .replace(";", "")
            .replace("&", "")
            .replace("|", "")
            .trim()
            .to_string();

        repobuilder.branch(&processed_branch);
    }

    debug!(
        "Cloning {} into {:?} (branch {:?})",
        url.clone(),
        path,
        branch.clone()
    );

    let result = repobuilder.clone(&processed_url, path);

    if let Err(e) = result {
        return Err(crate::errors::RequestError::GitError { error: e });
    }

    Ok(())
}
