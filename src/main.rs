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
fn welcome() ->&'static str {
    "Welcome"
}

#[launch]
fn rocket( ) -> _ {
    rocket::build().mount("/", routes![index]);
    rocket::build().mount("/welcome", routes![welcome])
}