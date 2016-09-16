extern crate iron;
extern crate router;

use self::iron::{Request, Response, IronResult};
use self::iron::status;
use self::router::{Router};

pub fn post(req: &mut Request) -> IronResult<Response> {
  let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
  Ok(Response::with((status::Ok, *id)))
}