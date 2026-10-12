use diesel::prelude::*;
use diesel_testing::{establish_connection, models::Post};
use std::env::args;

fn main() {
    use diesel_testing::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("invalid id");

    let mut connection = establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(&mut connection)
        .optional(); // allows for returning an Option<Post>, otherwise throws an error

    match post {
        Ok(Some(post)) => println!("post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("unable to find post {}", post_id),
        Err(_) => println!("an error occured while fetching post {}", post_id),
    }
}
