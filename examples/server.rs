extern crate iron;
extern crate "handlebars-iron" as hbs;
extern crate "rustc-serialize" as serialize;

use iron::prelude::*;
use iron::{status};
use hbs::{Template, HandlebarsEngine};
use serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

/// this function is for making example data
fn make_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let mut teams = Vec::new();

    for v in vec![("Jiangsu", 43u16), ("Beijing", 27u16), ("Guangzhou", 22u16), ("Shandong", 12u16)].iter() {
        let (name, score) = *v;
        let mut t = BTreeMap::new();
        t.insert("name".to_string(), name.to_json());
        t.insert("score".to_string(), score.to_json());
        teams.push(t)
    }

    data.insert("teams".to_string(), teams.to_json());
    data
}

/// the handler
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_after(HandlebarsEngine::new("./examples/templates/", ".hbs"));
    Iron::new(chain).http("localhost:3000").unwrap();
}
