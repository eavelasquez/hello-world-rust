#[macro_use] extern crate rocket;

// Try visiting:
//   http://127.0.0.1:8000
#[get("/")]                   // <- route attribute
fn world() -> &'static str {  // <- request handler
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/Ai/20
#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
}