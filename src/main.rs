//OVERALL TO-DO
//[X] HTML for home
//[ ] HTML for boards
//[ ] HTML for threads
//[ ] HTML for making threads
//[ ] Plug in the database where needed
//[ ] Routing

use vial::prelude::*;
use horrowshow::prelude::*;
use horrorshow::helper::doctype;
use mysql::*;
use mysql::prelude::*;
routes! {
    GET "/" => homePage;
    //The augment for boardPage is a placeholder for the DB entry
    GET "/boards/general" => boardPage(general);
    GET "/boards/general/newThread" => newThreadPage(general);    
    GET "/boards/politics" => boardPage(politics);
    GET "/boards/politics/newThread" => newThreadPage(politics); 
    GET "/boards/tech" => boardPage(tech);
    GET "/boards/tech/newThread" => newThreadPage(tech);     
    GET "/boards/tv" => boardPage(tv);
    GET "/boards/tv/newThread" => newThreadPage(tv); 
    GET "/boards/gaming" => boardPage(gaming);
    GET "/boards/gaming/newThread" => newThreadPage(gaming);     

    GET "/echo" => echo;
    POST "/echo" => post;
}

fn homePage(_: Request) -> impl Responder{
    let homeP = format!("{}", html! {
        : doctype::HTML;
        html{
            body{
                h1(class="Welcome to the Rustic Board!");
                //TO-DO: Login button/display current user
                br;
                br;
                h2(class="Board list");
                p{
                    : a(href="/boards/general", id="General");
                }
                br;
                p{
                    : a(href="/boards/tech", id="Tech");
                }
                br;
                p{
                    : a(href="/boards/gaming", id="Gaming");
                }
                br;
                p{
                    : a(href="/boards/politics", id="Politics");
                }
                br;
                p{
                    : a(href="/boards/tv", id="TV & Movies");
                }
                br;
            }
        }
    });
}
//TO-DO add augment to specify which board is being requested
fn boardPage(_: Request) -> impl Responder{
    let boardP = format!("{}", html! {
        : doctype::HTML;
        html{
            body{
                h1(class="Welcome to the Rustic Board!");
                br;
                a(href="/", id="Home");
                //TO-DO: Login button/display current user
                br;
                //TO-DO Get board name from database
                //h2(class=boardName)
                br;
                h2(class="Threads here");
                //ol{
                    //@ for i in 0..9{ (For subsequent pages, increase the start and end values of i by 10)
                        //p{
                            //a(href=THREAD_URL[i], title=format!(THREAD_TITLE[i] (RIGHT ALIGN)DATETIME[i]));
                        //};
                    //}
                //};
            }
        }
    });
}

fn threadPage(_: Request) -> impl Responder{
    let threadP = format!("{}", html! {
        : doctype::HTML;
        html{
            body{
                h1(class="Welcome to the Rustic Board!");
                br;
                a(href="/", id="Home");                
                //TO-DO: Login button/display current user
                br;
                //TO-DO: Add board name
                //h2(class=boardName);
                br;
                //h3(class=THREAD_TITLE);
                //p(class=AUTHOR, (RIGHT ALLIGN)REPLYID BR DATETIME)
                //ol{
                    //@ for i in 0..9{ (For subsequent pages, increase the start and end values of i by 10)
                        //p{
                            //a(href=THREAD_URL[i], title=format!(THREAD_TITLE[i] (RIGHT ALIGN)DATETIME[i]));
                        //};
                    //}
                //};
                br;
                form{
                    //action=REPLY(),
                    input(type=text, placeholder="Leave a reply");
                };
                
            }
        }
    });
}

fn newThreadPage(_: Request) -> impl Responder{
    let newThreadP = format!("{}", html! {
        : doctype::HTML;
        html{
            body{
                h1(class="Welcome to the Rustic Board!");
                br;
                a(href="/", id="Home");                
                //TO-DO: Login button/display current user
                br;
                br;
                h2(class="Create a thread");
                br;
                p(class="Title:");
                input(type=text);
                br;
                p(class="Text:");
                input(type=text);
                br;
            }
        }
    });
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