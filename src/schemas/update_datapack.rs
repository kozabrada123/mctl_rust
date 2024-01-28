use rocket::serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
/// Schema for updating a datapack
pub struct UpdateDatapackSchema {
    /// Which pack
    pub pack: String,
    /// Target branch name, if any
    ///
    /// defaults to master
    pub branch: Option<String>,
    // Todo: add commit checkout
}
