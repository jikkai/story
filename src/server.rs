extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
	
use std::path::Path;

use self::iron::prelude::*;
use self::mount::Mount;
use self::router::Router;
use self::staticfile::Static;
use self::hbs::{ HandlebarsEngine, DirectorySource };

use routers;

pub fn server() {
	let mut hbse = HandlebarsEngine::new();

	hbse.add(Box::new(DirectorySource::new("./src/templates/", ".hbs")));
	hbse.reload().unwrap();

	let mut router = Router::new();
	router.get("/", routers::home::home, "home");
	router.get("/post/:id", routers::post::post, "post");

	let mut mount = Mount::new();
	mount.mount("/", router);
	mount.mount("/static/", Static::new(Path::new("./src/static/")));

	let mut chain = Chain::new(mount);
	chain.link_after(hbse);
  chain.link_after(routers::error::Error404);

	Iron::new(chain).http("localhost:4000").unwrap();
}
