use diesel::prelude::*;
use diesel_testing::{establish_connection, models::Manekina, schema::manekinas};

fn main() {
    let color = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "turquoise".to_string());

    let connection = &mut establish_connection();

    let results = Manekina::query()
        .filter(manekinas::hair_color.eq(&color))
        .load(connection)
        .expect("error loading manekinas");

    println!("displaying {} manekinas with {color} hair", results.len());

    for manekina in results {
        dbg!(manekina);
    }
}
