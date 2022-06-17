#[macro_use]
extern crate log;

use std::{
    env::{current_dir, set_var},
    fs::{canonicalize, metadata, read_to_string},
    io::Result,
    path::PathBuf,
};

use actix_web::{
    get,
    web::{Data, Path},
    App, HttpResponse, HttpServer,
};
use clap::Parser as cliParser;
use pulldown_cmark::{html::push_html, Options, Parser};

/// Simple Markdown Server
#[derive(cliParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Document root directory
    #[clap(short, long, value_parser, default_value = ".")]
    directory: String,

    /// Use port number.
    #[clap(short, long, value_parser, default_value = "8380")]
    port: String,
}

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

#[get("/{path:.*}")]
async fn md(path: Path<(String,)>, data: Data<RaData>) -> HttpResponse {
    let path = path.into_inner();
    let file_path = format!("{}/{}.md", data.root, path.0);

    if metadata(file_path.clone()).is_err() {
        error!("[404] /{} -> {}", path.0, file_path);
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

    info!("[200] /{} -> {}", path.0, file_path);
    HttpResponse::Ok().content_type("text/html").body(html_str)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let root = if args.directory != "" {
        let rel_str = PathBuf::from(args.directory);
        canonicalize(&rel_str)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    } else {
        current_dir().unwrap().display().to_string()
    };

    set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Document root: {}", root);
    println!("Running at  http://localhost:{}/", args.port);

    let data = Data::new(RaData { root });

    HttpServer::new(move || App::new().app_data(data.clone()).service(index).service(md))
        .bind(format!("127.0.0.1:{}", args.port))?
        .run()
        .await
}
