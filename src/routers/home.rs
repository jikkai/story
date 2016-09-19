extern crate iron;
extern crate rustc_serialize as rss;
extern crate handlebars_iron as hbs;

use std::collections::BTreeMap;
use std::fs::read_dir;
use std::path::Path;

use self::iron::prelude::*;
use self::iron::status;
use self::rss::json::{ ToJson, Json };
use self::hbs::Template;

struct Posts {
	list: Json
}

impl ToJson for Posts {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("list".to_string(), self.list.to_json());
		Json::Object(m)
	}
}

pub fn home(_: &mut Request) -> IronResult<Response> {
	// 获取markdown文件目录
	let paths = read_dir(Path::new("./src/posts/")).unwrap();

	let mut mv = vec![];
	for path in paths {
		let dir_entry = path.unwrap();
		let path = dir_entry.path();
		let file_name = path.file_name().unwrap().to_str().unwrap();

		let v: Vec<&str> = file_name.split(".md").collect();

		mv.push(v[0].to_string());
	}

	let posts = Posts {
		list: mv.to_json()
	};

	let mut resp = Response::new();
	resp.set_mut(Template::new("index", posts.to_json())).set_mut(status::Ok);
	Ok(resp)
}