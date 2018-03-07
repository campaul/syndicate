extern crate hyper;
extern crate spellbook;
#[macro_use] extern crate tera;

use std::rc::Rc;

#[derive(Clone)]
struct State {
    tera: Rc<tera::Tera>,
}

fn index(context: spellbook::Context<State>) -> spellbook::Result {
    let tera_context = tera::Context::new();
    let body = try!(context.state.tera.render("index.html", &tera_context));

    Ok(spellbook::Response::new().with_body(body))
}

fn main() {
    let state = State {
        tera: Rc::new(compile_templates!("templates/**/*")),
    };

    let router = spellbook::Router::new().get("/", index);

    spellbook::Server::new(state, router).serve("127.0.0.1:3000");
}
