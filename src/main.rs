#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello mongooses"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
