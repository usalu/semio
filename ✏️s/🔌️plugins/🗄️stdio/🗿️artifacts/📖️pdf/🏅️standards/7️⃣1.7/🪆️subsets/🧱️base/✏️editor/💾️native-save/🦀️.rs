//! 💾️ Native PDF Save qualification, publication and close.
#[path = "🏭️factory.rs"]
pub mod factory;
#[path = "📤️producer.rs"]
pub mod producer;
#[path = "🧱️program.rs"]
pub mod program;
#[path = "♻️close.rs"]
pub mod close;
#[path = "🧾️job.rs"]
pub mod job;
#[path = "📄️page.rs"]
pub mod page;
#[path = "🪟️window.rs"]
pub mod window;

#[path = "🧬️graph.rs"]
pub mod graph;
#[path = "🔏️proof.rs"]
pub mod exact_graph_proof;
#[path = "🪡️graft.rs"]
pub mod ordinary_graft;

#[path = "🧮️graph-quote.rs"]
pub mod classic_graph_quote;

#[path = "🌊️graph-writer.rs"]
pub mod classic_graph_writer;

#[path = "🗜️stored-flate.rs"]
pub mod classic_stored_flate;

#[cfg(test)]
#[path = "🧪️tests/🔬️original-save/🦀️.rs"]
mod original_save_tests;
