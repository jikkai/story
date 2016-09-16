extern crate iron;

use self::iron::{Request, Response, IronResult};
use self::iron::status;

pub fn home(_: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "Home/")))
}