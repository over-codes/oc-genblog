#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use tera::{Context, Tera};
use log::{error};
use chrono::{Local, DateTime};

use pulldown_cmark::{Parser, html, Event, Tag};
use regex::Regex;

const RAW_MATCH_ATTRIBUTE: &str = r#"<!--\s*@(?P<name>\S+)\s+"(?P<value>[^"]+)"\s*-->"#;

lazy_static! {
    static ref MATCH_ATTRIBUTE: Regex = Regex::new(RAW_MATCH_ATTRIBUTE).unwrap();
}

struct Attributes {
    attributes: HashMap<String, String>,
}

impl tera::Function for Attributes {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        match args.get("key") {
            Some(val) => match tera::from_value::<String>(val.clone()) {
                Ok(v) => Ok(tera::to_value(self.attributes.get(&v).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            }
            None => Err("oops".into()),
        }
    }

    fn is_safe(&self) -> bool { true }
}

struct StringValue {
    val: String
}

impl tera::Function for StringValue {
    fn call(&self, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        Ok(tera::Value::String(self.val.clone()))
    }

    fn is_safe(&self) -> bool { true }
}

pub fn process(template_path: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // open the specified file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let metadata = fs::metadata(filename)?;
    // load the template directory
    let mut tera = match Tera::new(template_path) {
        Ok(t) => t,
        Err(e) => {
            error!("problem parsing template file: {}", e);
            return Ok(());
        }
    };
    let mut attributes = HashMap::new();
    let mut template = String::from("blog-post.html");
    let mut header_next = false;
    let parser = Parser::new(&contents)
        // extract attributes, removing them from the resulting code
        .filter(|event| {
            match event {
                Event::Html(tag) => {
                    if let Some(matches) = MATCH_ATTRIBUTE.captures(&tag) {
                        match matches.name("name").unwrap().as_str() {
                            "template" => template = String::from(matches.name("value").unwrap().as_str()),
                            _ => {
                                attributes.insert(String::from(matches.name("name").unwrap().as_str()), String::from(matches.name("value").unwrap().as_str()));
                            },
                        }
                        false
                    } else {
                        true
                    }
                },
                Event::Start(Tag::Heading(1)) => {
                    header_next = true;
                    true
                },
                Event::Text(data) => {
                    if header_next && !attributes.contains_key("title") {
                        attributes.insert(String::from("title"), data.to_string());
                    }
                    true
                }
                _ => true,
            }
        });
    let mut html_buff = String::new();
    html::push_html(&mut html_buff, parser);
    tera.register_function("attribute", Attributes{attributes});
    tera.register_function("content", StringValue{val: html_buff});
    tera.register_function("last_modified", StringValue{val: format!("{}", DateTime::<Local>::from(metadata.modified().unwrap()).format("%Y-%m-%d"))});
    let context = Context::new();
    println!("{}", tera.render(&template.clone(), &context)?);
    Ok(())
}