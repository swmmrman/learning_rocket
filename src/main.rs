#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
"<html>
    <body>
        <h1>Hellow World</h1>
    </body>
</html>"
}

#[get("/welcome")]
fn welcome() -> String {
    std::fs::read_to_string("tmpl.html").unwrap()
}

#[launch]
fn rocket( ) -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![welcome])
}