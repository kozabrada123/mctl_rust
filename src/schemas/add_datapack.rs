use rocket::serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
/// Schema for adding a datapack
pub struct AddDatapackSchema {
    /// Datapack git url
    pub url: String,
    /// Target branch name
    pub branch: Option<String>,
    /// Name of the datapack
    ///
    /// (Saved as the datapack folder)
    pub name: Option<String>,
}
