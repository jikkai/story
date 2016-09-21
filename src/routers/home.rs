extern crate iron;
extern crate router;
extern crate regex;
extern crate rustc_serialize as rss;
extern crate handlebars_iron as hbs;

use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::{ File, read_dir };
use std::path::Path;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::regex::Regex;
use self::rss::json::{ ToJson, Json };
use self::hbs::Template;

struct Posts {
	list: Json,
	count: usize,
	page: Vec<i32>,
	prev: usize,
	next: usize,
	is_not_start: bool,
	is_not_end: bool
}

impl ToJson for Posts {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("list".to_string(), self.list.to_json());
		m.insert("count".to_string(), self.count.to_json());
		m.insert("page".to_string(), self.page.to_json());
		m.insert("prev".to_string(), self.prev.to_json());
		m.insert("next".to_string(), self.next.to_json());
		m.insert("is_not_start".to_string(), self.is_not_start.to_json());
		m.insert("is_not_end".to_string(), self.is_not_end.to_json());
		Json::Object(m)
	}
}

struct Header {
	title: String,
	date: String,
	tags: String
}

impl ToJson for Header {
	fn to_json(&self) -> Json {
		let mut m: BTreeMap<String, Json> = BTreeMap::new();
		m.insert("title".to_string(), self.title.to_json());
		m.insert("date".to_string(), self.date.to_json());
		m.insert("tags".to_string(), self.tags.to_json());
		Json::Object(m)
	}
}

pub fn home(req: &mut Request) -> IronResult<Response> {
	let page = req.extensions.get::<Router>().unwrap().find("page").unwrap_or("/");
	let mut index = page;
	if page == "/" {
		index = "1";
	}

	// 获取markdown文件目录
	let paths = read_dir(Path::new("./src/posts/")).unwrap();

	let mut mv = vec![];
	for path in paths {
		let dir_entry = path.unwrap();
		let path = dir_entry.path();
		let file_name = path.file_name().unwrap().to_str().unwrap();

		if path.is_dir() == false && file_name.contains(".md") {

			let mut file = File::open(("./src/posts/".to_string() + &file_name).as_str()).unwrap();
			let mut buffer = String::new();
			file.read_to_string(&mut buffer).unwrap();
			
			let regex = Regex::new(r"title[\s\S]*--*\n").unwrap();
			let header = regex.captures(buffer.as_str()).unwrap().at(0).unwrap();

			let regex_date = Regex::new(r"date:\s\S*\s").unwrap();
			let regex_tags = Regex::new(r"tags:\s\[(\S*,?\s?)*\]\n").unwrap();
			let date: Vec<&str> = regex_date.captures(header).unwrap().at(0).unwrap().split("date: ").collect();
			let tags: Vec<&str> = regex_tags.captures(header).unwrap().at(0).unwrap().split("tags: ").collect();

			let v: Vec<&str> = file_name.split(".md").collect();
			let header = Header {
				title: v[0].to_string(),
				date: date[1].to_string(),
				tags: tags[1].to_string()
			};
			mv.push(header);
		}
	}

	mv.sort_by(|a, b| b.date.cmp(&a.date.to_string()));

	let size = index.to_string().parse::<usize>().unwrap() - 1;
	let mut page = mv.len().to_string().parse::<i32>().unwrap();

	if page % 10 == 0 {
		page = page / 10;
	} else {
		page = page / 10 + 1;
	}

	let mut page_v = vec![];
	for i in 1..(page + 1) {
		page_v.push(i);
	}

	let v = mv.split_at(size * 10).1;
	let mut length = 10;
	if v.len() < 10 {
		length = v.len()
	}
	let list = v.split_at(length).0;

	let posts = Posts {
		list: list.to_json(),
		count: mv.len(),
		page: page_v,
		prev: size,
		next: size + 2,
		is_not_start: size != 0,
		is_not_end: size.to_string().parse::<i32>().unwrap() != page - 1
	};

	let mut resp = Response::new();
	resp.set_mut(Template::new("index", posts.to_json())).set_mut(status::Ok);
	Ok(resp)
}