use rocket::tokio::time::{sleep, Duration};
use rocket::response::content;
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
content::RawHtml(
r#"<html>
    <body>
        <h1>Hello World</h1>
    </body>
</html>"#)
}

#[get("/welcome")]
fn welcome() -> content::RawHtml<String> {
    content::RawHtml(
    std::fs::read_to_string("tmpl.html").unwrap()
    )
}

#[get("/delay/<seconds>")]
async fn delay(mut seconds: u64) -> content::RawHtml<String> {
    if seconds > 30 {
        seconds = 30;
    }
    sleep(Duration::from_secs(seconds)).await;
    content::RawHtml(format!("Slept for {} seconds.", seconds))
}
#[get("/css/<css_file>")]
fn css(css_file: String) -> content::RawCss<String>{
    content::RawCss(
        std::fs::read_to_string(format!("css/{}", css_file)).unwrap()
    )
}

#[get("/js/<js_file>")]
fn js(js_file: String) -> content::RawJavaScript<String>{
    content::RawJavaScript(
        std::fs::read_to_string(format!("js/{}", js_file)).unwrap()
    )
}

#[launch]
fn rocket( ) -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![welcome])
        .mount("/", routes![delay])
        .mount("/", routes![css])
        .mount("/", routes![js])
}