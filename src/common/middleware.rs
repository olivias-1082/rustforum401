use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, AroundMiddleware, Handler};

use common::http::*;
use common::utils::{get_session_obj, is_login, is_admin};

pub struct FlowControl;

impl BeforeMiddleware for FlowControl {

    fn before(&self, _req: &mut Request) -> IronResult<()> {

        Ok(())
    }
}

impl AfterMiddleware for FlowControl {

    fn after(&self, _req: &mut Request, res: Response) -> IronResult<Response> {

        Ok(res)
    }
}

impl AroundMiddleware for FlowControl {

    fn around(self, handler: Box<Handler>) -> Box<Handler> {

        Box::new(move |req: &mut Request| -> IronResult<Response> {

            handler.handle(req)
        })
    }
}

pub fn authorize<F>(handler: F, check_login: bool, check_admin: bool) -> Box<Handler>
    where F: Send + Sync + 'static + Fn(&mut Request) -> IronResult<Response> {

    Box::new(move |req: &mut Request| -> IronResult<Response> {

        if check_login {

            if !is_login(req) { 

                if req.headers.get_raw("X-Requested-With").is_some() {  

                    let mut data = JsonData::new();

                    data.success = false;
                    data.message = "Unauthorized".to_string();

                    return respond_unauthorized_json(&data);
                } else {

                    return redirect_to("/login");
                }
            }
        
        }

        handler(req)
    })
}