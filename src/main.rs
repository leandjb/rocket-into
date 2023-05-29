#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { "HOME PAGE" }

#[get("/about")]
fn about() -> &'static str {
    "ABOUT PAGE"
}

#[get("/profile")]
fn create_profile() -> &'static str {
    "PROFILE PAGE"
}

#[post("/profile")]
fn profile() -> &'static str {
    "Creating profile..."
}

#[patch("/profile")]
fn update_profile() -> &'static str {
    "profile updated"
}

#[delete("/profile")]
fn delete_profile() -> &'static str {
    "Deleting profile..."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about, profile, create_profile, update_profile, delete_profile])
}
