#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { "HOME PAGE" }

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
}
