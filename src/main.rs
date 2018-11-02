#![allow(proc_macro_derive_resolution_fallback)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate tera;
extern crate warp;

mod facade;
mod models;
mod schema;

use diesel::prelude::*;
use failure::Fallible;
use tera::Context;
use warp::Filter;

use crate::facade::{Pool, Tera};
use crate::models::{Agent, Game, User};
use crate::schema::{agents, games, users};

fn main() -> Fallible<()> {
    const BIND_ADDRESS: ([u8; 4], u16) = ([0, 0, 0, 0], 8080);
    const STATIC_FILES_DIR: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let tera = Tera::new()?;
    let tera = warp::any().map(move || tera.clone());

    let pool = Pool::new()?;
    let pool = warp::any().map(move || pool.clone());

    let home = warp::path::end()
        .and(tera.clone())
        .and(pool.clone())
        .and_then(|tera: Tera, pool: Pool| {
            let _connection = pool.get()?;

            let context = Context::new();

            tera.render("home.tera.html", &context)
        });

    let users = warp::path("users")
        .and(warp::path::end())
        .and(tera.clone())
        .and(pool.clone())
        .and_then(|tera: Tera, pool: Pool| {
            let connection = pool.get()?;
            let users: Vec<User> = users::table.load(&connection).unwrap();

            let mut context = Context::new();
            context.insert("users", &users);

            tera.render("users.tera.html", &context)
        });

    let agents = warp::path("agents")
        .and(warp::path::end())
        .and(tera.clone())
        .and(pool.clone())
        .and_then(|tera: Tera, pool: Pool| {
            let connection = pool.get()?;
            let agents: Vec<Agent> = agents::table.load(&connection).unwrap();

            let mut context = Context::new();
            context.insert("agents", &agents);

            tera.render("agents.tera.html", &context)
        });

    let games = warp::path("games")
        .and(warp::path::end())
        .and(tera.clone())
        .and(pool.clone())
        .and_then(|tera: Tera, pool: Pool| {
            let connection = pool.get()?;
            let games: Vec<Game> = games::table.load(&connection).unwrap();

            let mut context = Context::new();
            context.insert("games", &games);

            tera.render("games.tera.html", &context)
        });

    let static_files = warp::fs::dir(STATIC_FILES_DIR);

    let routes =
        warp::get2().and(home.or(users).or(agents).or(games).or(static_files));

    warp::serve(routes).run(BIND_ADDRESS);

    Ok(())
}
