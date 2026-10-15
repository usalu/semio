//! 🧪️ The served schema against the committed SDL: the rendering is the document the Go twin and `graphql-js` read, and the inventory names exactly the declarations that document makes.

use super::*;

const SDL: &str = include_str!("../../🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql");

#[test]
fn serves_the_committed_sdl() {
    assert_eq!(render_sdl(&build_schema()), SDL);
}

#[test]
fn inventories_exactly_the_declarations_of_the_committed_sdl() {
    let mut declared: Vec<String> = SDL
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let kind = match words.next()? {
                "type" => "object",
                "interface" => "interface",
                "union" => "union",
                "enum" => "enum",
                "input" => "input",
                "scalar" => "scalar",
                _ => return None,
            };
            Some(format!("{kind} {}", words.next()?))
        })
        .collect();
    declared.sort();
    let inventory = schema_inventory(&build_schema());
    let mut inventoried: Vec<String> = inventory["types"].as_array().expect("types").iter().map(|row| format!("{} {}", row["kind"].as_str().expect("kind"), row["name"].as_str().expect("name"))).collect();
    inventoried.sort();
    assert!(!declared.is_empty());
    assert_eq!(inventoried, declared);
}
