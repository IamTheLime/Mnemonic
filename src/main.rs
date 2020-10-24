extern crate snippet_storage;
extern crate diesel;

use self::snippet_storage::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use snippet_storage::schema::snippets::dsl::*;

    let connection = establish_connection_sqlite();
    let results = snippets
        .filter(title.eq("pila"))
        .limit(5)
        .load::<Snippet>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {

    }
}
