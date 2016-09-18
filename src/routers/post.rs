extern crate iron;
extern crate router;
extern crate rustc_serialize as rss;
extern crate handlebars_iron as hbs;

use std::collections::BTreeMap;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::rss::json::{ ToJson, Json };
use self::hbs::Template;

struct Post {
	id: String
}

impl ToJson for Post {
	fn to_json(&self) -> Json {
		let mut m = BTreeMap::new();
		m.insert("id".to_string(), self.id.to_json());
		Json::Object(m)
	}
}

pub fn post(req: &mut Request) -> IronResult<Response> {
	let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
	let post = Post {
		id: id.to_string()
	};

	let mut resp = Response::new();
	resp.set_mut(Template::new("post", post.to_json())).set_mut(status::Ok);
	Ok(resp)
}