use log::{error, info, warn};
use rocket::routes;

mod config;
mod errors;
mod git;
mod guards;
mod hash;
mod routes;
mod schemas;

extern crate rocket;

#[rocket::launch]
fn rocket() -> _ {
    // Try to load the config once to see if it properly set up
    // This is done to panic right away instead of panicing on the first web request
    let config_result = config::Config::load();

    if let Err(e) = config_result {
        match e {
            config::ConfigError::FsError { e } => {
                eprintln!("Encountered filesystem error when loading config: {}", e);

                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        eprintln!("This is (probably) because you don't have a Config.toml file. Make one and rerun.");
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        eprintln!("This is (probably) because the process doesn't have permission to read your Config.toml file.");
                    }
                    _ => {}
                }

                std::process::exit(2);
            }
            config::ConfigError::ParseError { e } => {
                eprintln!("Encountered toml parse error when loading config: {}", e);

                std::process::exit(2);
            }
        }
    }

    drop(config_result);

    use rocket::fairing::AdHoc;

    rocket::build()
        .mount("/", routes![routes::root::index])
        .mount(
            "/datapacks",
            routes![routes::datapacks::add::post_add_datapack, routes::datapacks::update::post_update_datapack],
        )
        .attach(AdHoc::on_request("Compatibility Normalizer", |req, _| {
            Box::pin(async move {
                if !req.uri().is_normalized() {
                    let normal = req.uri().clone().into_normalized();
                    warn!("Incoming request URI was normalized for compatibility.");
                    info!("{} -> {}", req.uri(), normal);
                    req.set_uri(normal);
                }
            })
        }))
}
