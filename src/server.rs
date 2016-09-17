extern crate iron;
extern crate router;

use self::iron::{Iron, Chain};
use self::router::{Router};
use routers;

pub fn server() {
  let mut router = Router::new();
  router.get("/", routers::home::home, "home");
  router.get("/post/:id", routers::post::post, "post");

  let mut chain = Chain::new(router);
  chain.link_after(routers::error::Error404);

  Iron::new(chain).http("localhost:4000").unwrap();
}
