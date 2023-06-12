#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Web Application from Rocket.rs"
    })
}

#[get("/about")]
fn about() -> &'static str {
    "ABOUT - Title"
}

#[get("/")]
fn create_profile() -> &'static str {
    "PROFILE - Title"
}

#[post("/")]
fn profile() -> &'static str {
    "Message: profile created."
}

#[patch("/")]
fn update_profile() -> &'static str {
    "Message: profile updated."
}

#[delete("/")]
fn delete_profile() -> &'static str {
    "Message: profile deleted."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
        .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
        .attach(Template::fairing())
}
