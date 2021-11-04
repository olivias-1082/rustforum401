use vial::prelude::*;


routes! {

}

fn new(_req: Request) -> impl Responder {
    "<form method='POST'>
        <p>Name: <input type='text' name='name'/></p>
        <p>Location: <input type='text' name='location'/></p>
        <p><input type='submit'/></p>
    </form>"
}




fn main() {
    run!("localhost:2000");
}