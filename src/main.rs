#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "RESPONSE from Rocket.rs"
    })
}

#[get("/about")]
fn about() -> &'static str {
    "ABOUT PAGE"
}

#[get("/")]
fn create_profile() -> &'static str {
    "PROFILE PAGE"
}

#[post("/")]
fn profile() -> &'static str {
    "Creating profile..."
}

#[patch("/")]
fn update_profile() -> &'static str {
    "profile updated"
}

#[delete("/")]
fn delete_profile() -> &'static str {
    "Deleting profile..."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
        .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
        .attach(Template::fairing())
}
