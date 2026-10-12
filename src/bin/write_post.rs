use diesel_testing::*;
use std::io::{Read, stdin};

fn main() {
    let mut connection = establish_connection();

    let mut body = String::new();
    let mut title = String::new();

    println!("what would you like your title to be?");

    stdin().read_line(&mut title).unwrap();

    let title = title.trim_end();

    println!("\nok! let's write {title} (press ctrl+d when finished)\n",);

    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&mut connection, title, &body);

    println!("\nsaved draft {title} with id {}", post.id);
}
