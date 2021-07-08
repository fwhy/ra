use actix_web::{
    get,
    web::{Data, Path},
    App as webApp, HttpResponse, HttpServer,
};
use clap::{load_yaml, App};
use pulldown_cmark::{html::push_html, Options, Parser};
use std::{
    env::{current_dir, set_var},
    fs::{canonicalize, metadata, read_to_string},
    io::Result,
    path::PathBuf,
};

#[macro_use]
extern crate log;

struct RaData {
    root: String,
}

#[get("/")]
async fn index(data: Data<RaData>) -> HttpResponse {
    info!("[200] /");
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Document root: {}", data.root))
}

#[get("/{path}")]
async fn md(Path(path): Path<String>, data: Data<RaData>) -> HttpResponse {
    let file_path = format!("{}/{}.md", data.root, path);

    if metadata(file_path.clone()).is_err() {
        error!("[404] /{} -> {}", path, file_path);
        return HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not Found");
    }

    let md_str = read_to_string(file_path.clone()).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md_str, options);
    let mut body_str = String::new();
    push_html(&mut body_str, parser);
    let html_str = format!(
        r#"
        <html>
            <head>
            <meta charset="utf-8">
            </head>
            <body>{}</body>
        </html>
        "#,
        body_str
    );

    info!("[200] /{} -> {}", path, file_path);
    HttpResponse::Ok().content_type("text/html").body(html_str)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let root = if matches.is_present("directory") {
        let rel_str = PathBuf::from(matches.value_of("directory").unwrap().to_string());
        canonicalize(&rel_str)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    } else {
        current_dir().unwrap().display().to_string()
    };

    let port = if matches.is_present("port") {
        matches.value_of("port").unwrap()
    } else {
        "8383"
    };

    set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Document root: {}", root);
    println!("Running at  http://localhost:{}/", port);

    let data = Data::new(RaData { root });

    HttpServer::new(move || {
        webApp::new()
            .app_data(data.clone())
            .service(index)
            .service(md)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
