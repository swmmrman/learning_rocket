use rocket::tokio::time::{sleep, Duration};
use rocket::response::content;
use rocket::fs::NamedFile;

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
async fn welcome() -> Option<NamedFile> {
    NamedFile::open("tmpl.html").await.ok()
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
async fn css(css_file: String) -> Option<NamedFile>{
    NamedFile::open(format!("css/{}", css_file)).await.ok()
}

#[get("/js/<js_file>")]
async fn js(js_file: String) -> Option<NamedFile> {
    NamedFile::open(format!("js/{}", js_file)).await.ok()
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