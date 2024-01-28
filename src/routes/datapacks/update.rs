use rocket::{post, serde::json::Json};

use crate::{
    config::Config, errors::RequestError, guards::AuthenticatedUser, schemas::UpdateDatapackSchema,
};

#[post("/update", data = "<body>")]
/// Update a datapack
pub fn post_update_datapack(
    _user: AuthenticatedUser,
    body: Json<UpdateDatapackSchema>,
) -> Result<(), RequestError> {
    let config = Config::load().unwrap();

    let mut path = std::path::PathBuf::new()
        .join(config.server_folder.clone())
        .join("world/datapacks");

    let processed_name = body
        .0
        .pack
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

    crate::git::pull::pull(&path, body.0.branch)?;

    Ok(())
}
