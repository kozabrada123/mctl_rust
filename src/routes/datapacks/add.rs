use rocket::{post, serde::json::Json};

use crate::{
    config::Config, errors::RequestError, guards::AuthenticatedUser, schemas::AddDatapackSchema,
};

#[post("/add", data = "<body>")]
/// Add a datapack
pub fn post_add_datapack(
    _user: AuthenticatedUser,
    body: Json<AddDatapackSchema>,
) -> Result<(), RequestError> {
    let config = Config::load().unwrap();

    let mut path = std::path::PathBuf::new()
        .join(config.server_folder.clone())
        .join("world/datapacks");

    if let Some(name_unw) = body.0.name.clone() {
        let processed_name = name_unw
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

        path = path.join(processed_name);
    } else {
        // Find the name from the url
        let mut processed_url = body.url.clone().replace(".git", "");

        if processed_url.ends_with("/") {
            processed_url = processed_url
                .chars()
                .collect::<Vec<char>>()
                .remove(processed_url.len() - 1)
                .to_string();
        }

        let split = processed_url.split("/").collect::<Vec<&str>>();
        let last = split.get(split.len() - 1).unwrap().to_string();

        let repo_name = last.trim().to_lowercase();

        path = path.join(repo_name);
    }

    crate::git::clone::clone(&path, body.url.clone(), body.branch.clone())?;

    Ok(())
}
