use diesel::prelude::*;
use diesel_testing::establish_connection;
use std::env::args;

fn main() {
    use diesel_testing::schema::posts::dsl::*;

    let target = args().nth(1).expect("expected a target to match against");
    let pattern = format!("%{}%", target);

    let mut connection = establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&mut connection)
        .expect("error deleting posts");

    println!("deleted {} posts", num_deleted);
}
