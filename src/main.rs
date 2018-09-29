#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

//Route
#[get("/")]
fn index() -> &'static str {
    "Hello mongooses"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
