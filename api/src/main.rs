#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate actix;
extern crate actix_web;
extern crate initials;

use actix_web::{
    http::{Method, StatusCode, header}, server, App,
    State, Path, middleware, Query, HttpResponse,
    HttpRequest, pred, fs::NamedFile, Result,
};
use initials::{AvatarBuilder, AvatarResult};
use std::str;
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

// Deserialize the query for Avatar
#[derive(Deserialize)]
struct AvatarInfo {
    // background color
    bc: Option<String>,
    // font color
    fc: Option<String>,
    // font scale
    fs: Option<f32>,
    // text length
    l: Option<usize>,
    // avatar width
    w: Option<u32>,
    // avatar height
    h: Option<u32>,
    // contrast ratio
    cr: Option<f32>,
    // gaussian blur
    gb: Option<f32>
}

// contruct new avatar according to the queries
fn build_avatar(mut builder: AvatarBuilder, query: Query<AvatarInfo>) -> AvatarResult {
    if let Some(ref background_color) = query.bc {
        builder = builder.with_background_color(&format!("#{}", background_color))?
    }

    if let Some(ref font_color) = query.fc {
        builder = builder.with_font_color(&format!("#{}", font_color))?
    }

    builder
        .with_font_scale(query.fs.unwrap_or(150.))?
        .with_length(query.l.unwrap_or(2))?
        .with_width(query.w.unwrap_or(300))?
        .with_height(query.h.unwrap_or(300))?
        .with_contrast_ratio(query.cr.unwrap_or(5.))?
        .with_blur(query.gb.unwrap_or(1.3))
}

// serve the avatars using `initials` crate
fn handle_avatar(
    (param, query, state): (Path<String>, Query<AvatarInfo>, State<AppState>),
) -> Result<NamedFile> {
    let name = str::replace(&param, "+", " ");
    let builder = (state.builder)(&name);
    
    let avatar = match build_avatar(builder, query) {
        Ok(value) => value,
        Err(_) => return handle_bad_request(),
    };

    let ext: String = avatar.name
        .to_lowercase()
        .chars()
        .take(cmp::min(avatar.length, name.len()))
        .collect();
    let image = avatar.draw();
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