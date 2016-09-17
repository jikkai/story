extern crate iron;
extern crate router;

use self::iron::{Request, Response, IronResult, AfterMiddleware};
use self::iron::error::{IronError};
use self::iron::status;
use self::router::{NoRoute};

pub struct Error404;

impl AfterMiddleware for Error404 {
  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    if let Some(_) = err.error.downcast::<NoRoute>() {
      Ok(Response::with((status::NotFound, "404 page not found")))
    } else {
      Err(err)
    }
  }
}