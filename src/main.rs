use env_logger::Env;
use clap::{Arg, App};

use oc_genblog::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(Env::default().default_filter_or("trace")).init();
    let matches = App::new("oc-genblog")
        .version("0.1.0")
        .about("Generates a static blog from some markdown files.")
        .arg(Arg::with_name("templates")
                .help("name or path of the template files to use")
                .long("templates")
                .short("t")
                .default_value("templates/*"))
        .arg(Arg::with_name("base_url")
                .help("base URL to use for some filters")
                .long("base_url")
                .short("u")
                .default_value(""))
        .arg(Arg::with_name("INPUT")
                .required(true)
                .index(1)
                .help("Markdown file to scan"))
    .get_matches();
    let base_url = matches.value_of("base_url").unwrap();
    process(matches.value_of("templates").unwrap(), matches.value_of("INPUT").unwrap(), base_url)?;

    Ok(())
}
