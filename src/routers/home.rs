extern crate iron;
extern crate handlebars_iron as hbs;

use self::iron::prelude::*;
use self::iron::status;
use self::hbs::Template;

pub fn home(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	resp.set_mut(Template::new("index", Some(()))).set_mut(status::Ok);
	Ok(resp)
}