use rocket::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyConfig {
    app_key: String,
}
