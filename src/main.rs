use vial::prelude::*;
use mysql::*;
use mysql::prelude::*;
routes! {
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> impl Responder {
    "<form method='POST' >
    <div class='top-bar'>
        <h1>
            Rust Forum
        </h1>
    </div>
    <li class='row'>
        <a href='/'></a>
        <div class='header'>
            <h4 class='title'>
                Thread 1
            </h4>
        </div>
        <div class='bottom'>
            <p class='timestamp'>11/08/2021</p>
            <p class='comment-count'>7 comments</p>
        </div>
    </li>
    <li class='row'>
        <a href='/item'></a>
        <div class='header'>
            <h4 class='title'>
                Thread 2
            </h4>
        </div>
        <div class='bottom'>
            <p class='timestamp'>11/08/2021</p>
            <p class='comment-count'>7 comments</p>
        </div>
    </li>
    <input type=textarea name='echo'></input>
    <button input='submit' name='thread'>Post Comment</button>
    <div class='top-comment'>
        <p class='user'>Corey T.</p>
        <p class='timestamp'>10/10/2021</p>
    </div>
    <div class='comment-content'>Comment text here</div>
</form>

<style>
body {
    margin: 10px 60px;
}

a {
    text-decoration: none;
    color: black;
}

h4 {
    margin: 0;
}

p {
    margin: 5px 0;
}

.top-bar {
    background-color: pink;
    padding: 0 40px;
}

.main {
    background-color: #F6F6EF;
}

.row {
    padding: 5px 0;
}

.bottom {
    display: flex;
    color: grey;
    font-size: 12px;
}

.timestamp {
    padding-right: 10px;
}
</style>
</body>

"
}

fn post(req: Request) -> impl Responder {
    format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    );
    "INSERT INTO thread (echo)
    VALUES (:echo)"
  
}

fn main() {
    run!("localhost:2000");
}