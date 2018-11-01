extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate failure;
extern crate tera;
extern crate warp;

mod facade;

use failure::Fallible;
use tera::Context;
use warp::Filter;

use crate::facade::Tera;

fn main() -> Fallible<()> {
    const BIND_ADDRESS: ([u8; 4], u16) = ([0, 0, 0, 0], 8080);
    const STATIC_FILES_DIR: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let tera = Tera::new()?;
    let tera = warp::any().map(move || tera.clone());

    let home = warp::path::end().and(tera.clone()).and_then(|tera: Tera| {
        let context = Context::new();

        tera.render("home.tera.html", &context)
    });

    let users = warp::path("users")
        .and(warp::path::end())
        .and(tera.clone())
        .and_then(|tera: Tera| {
            let context = Context::new();

            tera.render("users.tera.html", &context)
        });

    let agents = warp::path("agents")
        .and(warp::path::end())
        .and(tera.clone())
        .and_then(|tera: Tera| {
            let context = Context::new();

            tera.render("agents.tera.html", &context)
        });

    let games = warp::path("games")
        .and(warp::path::end())
        .and(tera.clone())
        .and_then(|tera: Tera| {
            let context = Context::new();

            tera.render("games.tera.html", &context)
        });

    let static_files = warp::fs::dir(STATIC_FILES_DIR);

    let routes =
        warp::get2().and(home.or(users).or(agents).or(games).or(static_files));

    warp::serve(routes).run(BIND_ADDRESS);

    Ok(())
}
