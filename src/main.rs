#[macro_use]
use vial::prelude::*;
use horrorshow::owned_html;
use horrorshow::html;
routes! {
    GET "/" => |_| html! {
        p {
            : "You're looking for this: ";
            a(href="/echo") { : "echo" }
        }
    };
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> impl Responder {
    html! {
        form(method="POST") {
            p {
            : "Type something: ";
                input(type="text", name="echo");
                input(type="submit");
            }
        }
    }
}

fn post(req: Request) -> impl Responder {
    owned_html! {
        h1: req.form("echo")
            .unwrap_or("You didn't say anything!");
    }
}

fn main() {
    vial::run!("localhost:2000").unwrap();
}