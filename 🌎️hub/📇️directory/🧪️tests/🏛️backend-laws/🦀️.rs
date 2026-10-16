//! 🏛️ Directory backend laws: the per-backend unit laws of the sqlite, postgres and neo4j directories over their real stores.

#[cfg(test)]
#[path = "../../🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs"]
mod sqlite_tests;

#[cfg(test)]
#[path = "../../🐘️postgres/🧪️tests/🔬️unit/🦀️.rs"]
mod postgres_tests;

#[cfg(test)]
#[path = "../../🌐️neo4j/🧪️tests/🔬️unit/🦀️.rs"]
mod neo4j_tests;
