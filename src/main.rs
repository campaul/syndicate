extern crate hyper;
extern crate spellbook;
#[macro_use] extern crate tera;

use spellbook::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
struct State {
    tera: Rc<tera::Tera>,
}

fn index(context: Context<State>) -> Result {
    let tera_context = tera::Context::new();
    let body = try!(context.state.tera.render("index.html", &tera_context));

    Ok(Response::new()
        .with_header(hyper::header::ContentLength(body.len() as u64))
        .with_body(body))
}

fn main() {
    let state = State {
        tera: Rc::new(compile_templates!("templates/**/*")),
    };

    let router = Router::new()
        .get("/", index);

    Spellbook::new(state, router).serve("127.0.0.1:3000");
}
