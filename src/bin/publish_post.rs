use diesel::prelude::*;
use diesel_testing::{establish_connection, models::Post};
use std::env::args;

fn main() {
    use diesel_testing::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("invalid id");

    let mut connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(&mut connection)
        .unwrap();

    println!("Published post {}", post.title);
}
