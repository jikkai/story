extern crate iron;
extern crate router;
extern crate regex;
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
use self::regex::Regex;
use self::url::percent_encoding::percent_decode;
use self::hoedown::{ Markdown, Render };
use self::hoedown::renderer::html::{ self, Html };
use self::rss::json::{ ToJson, Json };
use self::hbs::Template;

struct Post {
	id: String,
	title: String,
	date: String,
	tags: String,
	content: String
}

impl ToJson for Post {
	fn to_json(&self) -> Json {
		let mut m = BTreeMap::new();
		m.insert("id".to_string(), self.id.to_json());
		m.insert("title".to_string(), self.title.to_json());
		m.insert("date".to_string(), self.date.to_json());
		m.insert("tags".to_string(), self.tags.to_json());
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

	// 解析文件头
	let regex = Regex::new(r"title[\s\S]*--*\n").unwrap();
	let header = regex.captures(buffer.as_str()).unwrap().at(0).unwrap();

	let regex_date = Regex::new(r"date:\s\S*\s").unwrap();
	let regex_tags = Regex::new(r"tags:\s\[(\S*,?\s?)*\]\n").unwrap();
	let date: Vec<&str> = regex_date.captures(header).unwrap().at(0).unwrap().split("date: ").collect();
	let tags: Vec<&str> = regex_tags.captures(header).unwrap().at(0).unwrap().split("tags: ").collect();

	// 解析文件体
	let regex_content = Regex::new(r"---\n[\s\S]*").unwrap();
	let content = regex_content.captures(buffer.as_str()).unwrap().at(0).unwrap();

	// 解析markdown文件
	let doc = Markdown::new(content.to_string().as_str());
  let mut html = Html::new(html::Flags::empty(), 0);
	let res = html.render(&doc);
	let result = res.to_str().unwrap();

	let post = Post {
		id: id.to_string(),
		title: title.to_string(),
		date: date[1].to_string(),
		tags: tags[1].to_string(),
		content: result.to_string()
	};

	let mut resp = Response::new();
	resp.set_mut(Template::new("post", post.to_json())).set_mut(status::Ok);
	Ok(resp)
}