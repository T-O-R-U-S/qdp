#![feature(proc_macro_hygiene, decl_macro)]

use std::env::args;

#[macro_use]
extern crate rocket;
use rocket::{
    config::{ConfigError},
    response::content::Html,
};
use rocket_contrib::serve::StaticFiles;

use console::style;

#[catch(404)]
fn four_o_four() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
    <style type="text/css">
        * {
            margin: 0;
            background-color: black;
            color: white;
            font-family: 'Lucida Console', monospace;
            font-weight: 900;
        }
        p {
            font-family: 'Inconsolatas', 'Ubuntu Mono', 'JetBrainsMono NF', monospace;
            margin: 5px;
        }
        h1 {
            color: black;
            width: 100%-30px;
            background-color: #9d16db;
            padding: 30px
        }
        h6 {
            margin: 5px;
        }
    </style>
    <h1 style='font-family: monospace'>404 -- file not found</h1>
    <p>There is no default index/root directory in this version of QuickDeploy (0.1.0), so if you tried to access a directory without an index.html file, you will probably end up here.</p>
    <h6>There will probably never be a root directory due to the way Rocket.rs handles URIs paired with the unpredicatbility of user input.</h6>
</html>"#,
    )
}

fn main() -> Result<(), ConfigError> {
    let mut args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!(
            "{0} (Use {1} to serve your local directory)",
            style("Error: You must specify a directory to serve the static files!").red(),
            style(".").magenta()
        );
        return Ok(());
    }

    if args.len() < 3 {
        println!(
            "{}",
            style("Warning: No 3rd argument specified, assuming / as root...").yellow()
        );
        args.push(String::from("/"));
    }

    rocket::ignite()
        .register(catchers![four_o_four])
        .mount(&args[2], StaticFiles::from(&args[1]))
        .launch();
    Ok(())
}
