extern crate hyper;
extern crate spellbook;
#[macro_use] extern crate tera;

use std::rc::Rc;

use spellbook::{
    Request,
    Response,
    Result,
    Router,
    Spellbook
};

use tera::{
    Tera,
    Context,
};

#[derive(Clone)]
struct Syndicate {
    tera: Rc<Tera>,
}

fn index(app: Rc<Syndicate>, _req: Rc<Request>) -> Result {
    let context = Context::new();
    let body = try!(app.tera.render("index.html", &context));

    Ok(Response::new()
        .with_header(hyper::header::ContentLength(body.len() as u64))
        .with_body(body))
}

fn main() {
    let app = Syndicate {
        tera: Rc::new(compile_templates!("templates/**/*")),
    };

    let router = Router::new()
        .get("/", index);

    Spellbook::new(app, router).serve("127.0.0.1:3000");
}
