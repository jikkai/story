extern crate iron;
extern crate router;

use self::iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use self::iron::error::{IronError};
use self::iron::status;
use self::router::{Router, NoRoute};
use routers;

struct Custom404;

impl AfterMiddleware for Custom404 {
  fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
    println!("Hitting custom 404 middleware");

    if let Some(_) = err.error.downcast::<NoRoute>() {
      Ok(Response::with((status::NotFound, "Custom 404 response")))
    } else {
      Err(err)
    }
  }
}

pub fn server() {
  let mut router = Router::new();
  router.get("/", routers::home::home, "home");
  router.get("/post/:id", routers::post::post, "post");

  let mut chain = Chain::new(router);
  chain.link_after(Custom404);

  Iron::new(chain).http("localhost:4000").unwrap();
}
