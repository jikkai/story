extern crate toml;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron as hbs;
	
use std::io::prelude::*;
use std::fs::{ File };
use std::path::Path;

use self::iron::prelude::*;
use self::mount::Mount;
use self::router::Router;
use self::staticfile::Static;
use self::hbs::{ HandlebarsEngine, DirectorySource };

use routers;

pub fn server() {
	// 解析配置文件
	let mut config_file = File::open("./src/Config.toml".to_string()).unwrap();
	let mut buffer = String::new();
	config_file.read_to_string(&mut buffer).unwrap();
	let config_toml = toml::Parser::new(buffer.to_string().as_str()).parse().unwrap();
	let config_basic = config_toml.get("basic").unwrap();

	let config_theme = config_basic.lookup("theme").unwrap();
	let theme = config_theme.as_str().unwrap();

	// 设置模板目录
	let mut hbse = HandlebarsEngine::new();
	hbse.add(Box::new(DirectorySource::new(("./src/themes/".to_string() + theme + "/templates/").as_str(), ".hbs")));
	hbse.reload().unwrap();

	// 设置路由
	let mut router = Router::new();
	router.get("/", routers::home::home, "home");
	router.get("/post/:id", routers::post::post, "post");

	// 设置静态文件
	let mut mount = Mount::new();
	mount.mount("/", router);
	mount.mount("/static/", Static::new(Path::new(("./src/themes/".to_string() + theme + "/static/").as_str())));

	// 追加中间件
	let mut chain = Chain::new(mount);
	chain.link_after(hbse);
  chain.link_after(routers::error::Error404);

	Iron::new(chain).http("localhost:4000").unwrap();
}
