mod precision_header;

use std::{
    collections::HashMap,
    convert::Infallible,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use axum::{
    extract::{Extension, Json, Path, Query},
    handler::get,
    http::{
        header::{self, HeaderName},
        Error, HeaderMap, HeaderValue, StatusCode,
    },
    response::Headers,
    service, Router, Server,
};

use precision_header::*;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
async fn search(h: HeaderMap) -> (HeaderMap, StringUtf8) {
    // <Contact>s7and70and0@outllook.org</Contact>
    // <Image height="16" width="16" type="favicon.ico?v=1"></Image>
    // <Developer>John Bookos</Developer>
    // <OutputEncoding>UTF-8</OutputEncoding>
    // <InputEncoding>UTF-8</InputEncoding>
    //<Url type="text/html" method="get" template="http://2962-36-235-73-163.ngrok.io?q={searchTerms}"/>
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
    <ShortName>Search Me</ShortName>
    <Description>search me.</Description>
    <Contact>xxx@outlook.com</Contact>
    <Url type="text/html" method="get" template="http://search.me?q={searchTerms}"/>
    <Url rel="self" type="application/opensearchdescription+xml" template="http://search.me/search.xml" />
    <Url type="application/x-suggestions+json" template="http://search.me/auto/{searchTerms}"/>
</OpenSearchDescription>"#
        .to_string();
    for key in h.keys() {
        println!("{:?}: {:?}", key, h.get(key));
    }
    let mut header = HeaderMap::new();
    //header.insert(
    //    //header::CONTENT_TYPE,
    //    HeaderName::from_str("last-modified").unwrap(),
    //    HeaderValue::from_str("Sun, 17 Oct 2021 08:44:32 GMT").unwrap(),
    //    //HeaderValue::from_str("application/opensearchdescription+xml").unwrap(),
    //);
    println!("call!");
    header.append(
        header::SET_COOKIE,
        HeaderValue::from_str("search=573536568586765").unwrap(),
    );
    //expires: Tue, 19 Oct 2021 11:12:22 GMT
    //last-modified: Sun, 17 Oct 2021 08:44:32 GMT
    (header, xml.into())
}

//https://www.thewindowsclub.com/uri-commands-to-open-microsoft-store-apps
async fn query(
    h: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, HeaderMap, HtmlUtf8) {
    let mut header = HeaderMap::new();
    if let Some(q) = params.get("q") {
        println!("{}", q);
        let uri = if q.as_str() == "mail" {
            format!("mailto:s7and70and0@outlook.com")
        } else if q.as_str() == "settings" {
            format!("ms-call:settings")
        } else if q.as_str() == "map" {
            format!("bingmaps:?cp=40.726966~-74.006076")
        } else if q.as_str() == "msa" {
            format!("ms-actioncenter:")
        } else if q.as_str() == "msc" {
            //ms-default-location
            format!("ms-calculator:")
        } else if q.as_str() == "msclock" {
            format!("ms-clock:")
        } else if q.as_str() == "課表" {
            format!("http://search.me/static/course.jpeg")
        } else {
            format!("https://www.google.com.tw/search?q={}", q)
        };
        header.insert(
            header::LOCATION,
            HeaderValue::from_str(uri.as_str()).unwrap(),
        );
        (StatusCode::FOUND, header, "".to_owned().into())
    } else {
        for key in h.keys() {
            println!("{:?}: {:?}", key, h.get(key));
        }
        header.append(header::SET_COOKIE, HeaderValue::from_str("a=1").unwrap());
        header.append(header::SET_COOKIE, HeaderValue::from_str("b=p").unwrap());
        header.append(header::SET_COOKIE, "c=0".parse().unwrap());
        let html = r#"<!DOCTYPE html>
<html lang="zh-TW">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Search Me</title>
    <link rel="search" type="application/opensearchdescription+xml" href="http://search.me/search.xml" title="LocalHost">
    </head>
    <body>
        ...hi...
    </body>
</html>"#;
        (StatusCode::OK, header, html.to_owned().into())
    }
}

async fn auto(h: HeaderMap) -> &'static str {
    for key in h.keys() {
        println!("{:?}: {:?}", key, h.get(key));
    }
    r#"["abc",["apple","cap","bool"]]"#
}

#[tokio::main]
async fn main() {
    let dirs = ServeDir::new(std::path::Path::new(
        "C:\\Users\\s7and\\Pictures\\web_m_db_search_file\\",
    ))
    .append_index_html_on_directories(true);
    //hide_console_window();
    let app = Router::new()
        .route("/", get(query))
        .route("/auto/:data", get(auto))
        .route("/search.xml", get(search))
        .nest(
            "/dir",
            service::get(dirs).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("找不到檔案 error: {}", error),
                ))
            }),
        )
        .route(
            "/static/course.jpeg",
            service::get(ServeFile::new(
                "C:\\Users\\s7and\\Pictures\\web_m_db_search_file\\course.jpeg",
            ))
            .handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        )
        .into_make_service();

    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 80)))
        .serve(app)
        .await
        .unwrap();
}

fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}
