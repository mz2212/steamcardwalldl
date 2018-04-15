#[macro_use]
extern crate clap;

extern crate reqwest;
extern crate select;
extern crate url;

use clap::App;
use select::document::Document;
use select::predicate::{Class, Name};
use url::Url;
use std::io::Read;
use std::io::copy;
use std::fs::OpenOptions;
use std::path::PathBuf;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let app_id = matches.value_of("appid").unwrap(); // Concat the AppId with the baseurl
    let base_page = "https://www.steamcardexchange.net/index.php?gamepage-appid-";
    let page = format!("{}{}", base_page, app_id);
    let mut response = reqwest::get(page.as_str()).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let dom = Document::from(body.as_str());
    let mut game_name = String::new();

    for node in dom.find(Class("game-title")) { // Hunt down the game's title
        game_name = node.find(Name("h1")).next().unwrap().text();
    }
    println!("Downloading wallpapers for: {}", game_name);

    let wplinks = dom.find(Class("element-link-right"));
    for node in wplinks {
        let link = node.attr("href").unwrap();
        let parsed_link = Url::parse(link).unwrap();
        let path_vec = parsed_link.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        let filename = path_vec.last().unwrap();
        let mut path = PathBuf::from(filename);
        path.set_extension("jpg");
        let filename = path.to_str().unwrap();
        println!("{}", filename);

        let mut file = match OpenOptions::new().write(true).create_new(true).open(filename){
            Err(_) => continue,
            Ok(f) => f,
        };

        let mut image = reqwest::get(link).unwrap();
        copy(&mut image, &mut file).unwrap();
    } 
}
