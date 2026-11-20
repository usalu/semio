//! 🧪️ Companion white-box laws of `semio-ui-contract-values`.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]


#[path = "../🔬️unit/🦀️.rs"]
mod unit;
#[path = "🏟️arena/🦀️.rs"]
mod arena_laws;
#[path = "../../🗺️fixed-map-and-bytes/🛬️receive/🧪️tests/🦀️.rs"]
mod bounded_receiving_tests;
#[path = "../../📋️list/🧪️tests/📋️list/🦀️.rs"]
mod fixed_list_storage_tests;
#[path = "../../⚖️compare/🧪️tests/🦀️.rs"]
mod compare_tests;
#[path = "../../♻️retirement/📋️list/🧪️tests/📋️list/🦀️.rs"]
mod retirement_fixed_list_tests;
#[path = "../../♻️retirement/🧪️tests/♻️retirement/🦀️.rs"]
mod retirement_tests;
#[path = "../../♻️retirement/📮️handback/🧪️tests/📮️handback/🦀️.rs"]
mod handback_tests;
