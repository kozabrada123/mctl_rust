use rocket::get;

#[get("/")]
pub fn index() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    format!("Running Minecraft control server ({}) v{}", name, version)
}
