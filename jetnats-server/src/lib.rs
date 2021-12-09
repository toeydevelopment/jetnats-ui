pub mod nats;

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::Build;

pub fn rocket() -> rocket::Rocket<Build> {
    dotenv().ok();
    rocket::build().mount("/api", routes![])
}
