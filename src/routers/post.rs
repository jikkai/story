extern crate iron;
extern crate router;
extern crate url;
extern crate hoedown;
extern crate rustc_serialize as rss;
extern crate handlebars_iron as hbs;

use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::url::percent_encoding::percent_decode;
use self::hoedown::{ Markdown, Render };
use self::hoedown::renderer::html::{ self, Html };
use self::rss::json::{ ToJson, Json };
use self::hbs::Template;

struct Post {
	id: String,
	content: String
}

impl ToJson for Post {
	fn to_json(&self) -> Json {
		let mut m = BTreeMap::new();
		m.insert("id".to_string(), self.id.to_json());
		m.insert("content".to_string(), self.content.to_json());
		Json::Object(m)
	}
}

pub fn post(req: &mut Request) -> IronResult<Response> {
	let id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
	let title = percent_decode(id.as_bytes()).decode_utf8().unwrap();

	// 读取markdown文件
	let mut file = File::open(("./src/posts/".to_string() + &title.to_string() + ".md").as_str()).unwrap();
	let mut buffer = String::new();
	file.read_to_string(&mut buffer).unwrap();

	// 解析markdown文件
	let doc = Markdown::new(buffer.to_string().as_str());
  let mut html = Html::new(html::Flags::empty(), 0);
	let res = html.render(&doc);
	let content = res.to_str().unwrap();

	let post = Post {
		id: id.to_string(),
		content: content.to_string()
	};

	let mut resp = Response::new();
	resp.set_mut(Template::new("post", post.to_json())).set_mut(status::Ok);
	Ok(resp)
}