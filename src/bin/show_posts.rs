use diesel::prelude::*;
use diesel_testing::{establish_connection, models::*};

fn main() {
    use diesel_testing::schema::posts::dsl::*;

    let mut connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(&mut connection)
        .expect("error loading posts");

    println!("displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("------------\n");
        println!("{}", post.body);
    }
}
