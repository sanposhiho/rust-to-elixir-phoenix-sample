#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use rustler::{Encoder, Env, Error, Term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.RustPhxSampleWeb.UserController",
    [
        ("create_user", 2, create_user)
    ],
    None
}

fn create_user<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let name: String = args[0].decode()?;
    let age: i32 = args[1].decode()?;

    let connection = establish_connection();

    let user = store_user(&connection, &name, &age);

    Ok((atoms::ok(), (user.id, user.name, user.age)).encode(env))
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{User, NewUser};

fn store_user<'a>(conn: &PgConnection, name: &'a str, age: &'a i32) -> User {
    use schema::users;

    let new_user = NewUser {
        name: name,
        age:  age,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
