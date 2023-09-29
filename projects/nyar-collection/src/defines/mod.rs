pub struct HelloRequest {}

pub struct HelloResponse {}

/// service world {
///     hello(name: String) -> HelloResponse
/// }
pub trait World {
    // get{}
    async fn hello(&mut self, input: HelloRequest) -> HelloResponse;
}

pub struct HelloServer {
    context: String,
}

impl <'de> Deserialize<'de> for HelloRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any()
    }
}
impl <'de> Visitor<'de> for HelloRequest {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        ContentDeserializer

        todo!()
    }
}



impl World for HelloServer {
    async fn hello(&mut self, name: HelloRequest) -> HelloResponse {
        todo!()
    }
}

use std::fmt::Formatter;
use std::str::FromStr;

use hyper::{Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Deserializer};
use serde::__private::de::ContentDeserializer;
use serde::__private::de::TagOrContentField::Content;
use serde::de::{MapAccess, Visitor};

pub struct RouteDefine {}

pub fn route(req: Request<Body>) {
    let mut response = Response::new(Body::empty());
    let url = req.uri();
    let query = url.query();
    match (req.method(), url.path()) {
        (&Method::GET, "/") => {
            println!("/")
        }
        (&Method::GET, "user", id, "info") => {
            usize::from_str(id)
            println!("/user")
        }
        (&Method::GET, "/") => {}
        (&Method::POST, "/echo") => {
            // we'll be back
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
}
