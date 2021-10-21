///! PrecisionHrader

use axum::{body::Full, http::{
    header::{self, HeaderValue},
    Response,
}};
use axum::response::IntoResponse;
use axum::body::Bytes;
//use axum::http:: //http_body::Full;
use std::convert::Infallible;

pub type StringUtf8 = Plain::<String>;
pub type HtmlUtf8 = Html::<String>;
pub type StrUtf8<'a> = Plain::<&'a str>;

#[derive(Clone, Copy, Debug)]
pub struct Plain<T>(pub T);

impl<T> IntoResponse for Plain<T>
where
    T: Into<Full<Bytes>>,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(self.0.into());
        //res.headers_mut().insert(
        //    header::CONTENT_TYPE,
        //    HeaderValue::from_static("text/plain;charset=utf-8"),
        //);
        res
    }
}

impl<T> From<T> for Plain<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Html<T>(pub T);

impl<T> IntoResponse for Html<T>
where
    T: Into<Full<Bytes>>,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let mut res = Response::new(self.0.into());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        res
    }
}

impl<T> From<T> for Html<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
