use vial::prelude::*;

routes! {
    GET "/info" => |req| format!(
        "<p>Name: {}</p>", req.query("name").unwrap_or("None")
    );
    GET "/" => index;
}

fn index(req: Request) -> impl Responder {
    "<form method='GET'>
        <p>Enter your name: <input type='text' name='name'/></p>
        <input type='submit'/>
    </form>"
}

fn main() {
    run!("localhost:2000");
}