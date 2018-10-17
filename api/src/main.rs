extern crate actix;
extern crate actix_web;
extern crate initials;

use actix_web::{
    http::{Method, StatusCode, header}, server, App,
    State, Path, middleware, Query, HttpResponse,
    HttpRequest, pred, fs::NamedFile, Result,
};
use initials::{AvatarBuilder, AvatarResult};
use std::collections::HashMap;
use std::str;
use std::str::FromStr;
use std::cmp;
use std::env;

// store closure inside App state
struct AppState {
    builder: Box<Fn(&str) -> AvatarBuilder>,
}

// handle favicon
fn handle_favicon(_: &HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

// render static error page for bad request
fn handle_bad_request() -> Result<NamedFile> {
    Ok(NamedFile::open("static/HTTP400.html")?
        .set_status_code(StatusCode::BAD_REQUEST))
}

// render static error page for not found
fn handle_not_found(_: State<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("static/HTTP404.html")?
        .set_status_code(StatusCode::NOT_FOUND))
}

// handle index
fn handle_index(_: State<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
        .header(header::LOCATION, "/img/a")
        .finish()
    )
}

// server port
fn server_port() -> u16 {
    env::var("APP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8000)
}

// serve the avatars using initials crate
fn handle_avatar(
    (param, query, state): (Path<String>, Query<HashMap<String, String>>, State<AppState>),
) -> Result<NamedFile> {
    let name = str::replace(&param, "+", " ");
    let mut builder = (state.builder)(&name);
    for (key, value) in query.iter() {
        let res: AvatarResult = match key.as_ref() {
            "f" => builder.with_font(value),
            "bc" => builder.with_background_color(&format!("#{}", value)),
            "fc" => builder.with_font_color(&format!("#{}", value)),
            "l" => builder.with_length(usize::from_str(value).unwrap_or(2)),
            "w" =>  builder.with_width(u32::from_str(value).unwrap_or(300)),
            "h" => builder.with_height(u32::from_str(value).unwrap_or(300)),
            "cr" => builder.with_contrast_ratio(f32::from_str(value).unwrap_or(5.)),
            _ => Ok(builder)
        };

        builder = match res {
            Ok(val) => val,
            Err(_) => return handle_bad_request(),
        }
    }

    let ext: String = builder.name
        .to_lowercase()
        .chars()
        .take(cmp::min(builder.length, name.len()))
        .collect();
    let image = builder.draw();
    let img_path = format!("static/{}.jpg", ext);

    image.save(&img_path).unwrap();
    Ok(NamedFile::open(img_path)?)
}


fn main() {
    // create the actor system
    let sys = actix::System::new("initials-avatar");

    server::new(|| {
        App::with_state(AppState{ builder: Box::new(|name| AvatarBuilder::new(name)) })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(Method::GET).with(handle_index))
            .resource("/favicon", |r| r.f(handle_favicon))
            .resource("/img/{avatar}", |r| r.method(Method::GET).with(handle_avatar))
            .default_resource(|r| {
                r.method(Method::GET).with(handle_not_found);
                r.route().filter(pred::Not(pred::Get())).f(
                    |_| HttpResponse::MethodNotAllowed());
            })
    })
    .bind(format!("0.0.0.0:{}", server_port()))
    .unwrap()
    .shutdown_timeout(0)
    .start();

    let _ = sys.run();
}