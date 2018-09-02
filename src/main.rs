#[macro_use] extern crate diesel;
extern crate djangohashers;
extern crate dotenv;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_struct_wrapper;
extern crate time;
#[macro_use] extern crate warp;
extern crate uuid;

use warp::Filter;
use warp::filters::body::json;
use warp::filters::method::post;
use warp::reply::json as replyjson;

mod repository;
mod schema;
mod user;

use user::NewUser;

fn main() {
    // POST /api/users
    let users = post(path!("api")
                     .and(path!("users"))
                     .and(json::<NewUser>())
                     .map(|new_user: NewUser| replyjson(&new_user.register())));
    warp::serve(users)
        .run(([127, 0, 0, 1], 8080));
}

