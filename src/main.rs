extern crate tera;
extern crate warp;

use warp::Filter;

fn main() {
    const BIND_ADDRESS: ([u8; 4], u16) = ([0, 0, 0, 0], 8080);

    let home = warp::path::end().map(|| "Home page");

    let users = warp::path("users")
        .and(warp::path::end())
        .map(|| "Users page");

    let players = warp::path("players")
        .and(warp::path::end())
        .map(|| "Players page");

    let games = warp::path("games")
        .and(warp::path::end())
        .map(|| "Games page");

    let routes = warp::get2().and(home.or(users).or(players).or(games));

    warp::serve(routes).run(BIND_ADDRESS);
}
