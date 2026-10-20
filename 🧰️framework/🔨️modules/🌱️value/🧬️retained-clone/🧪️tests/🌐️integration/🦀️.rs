//! 🧫️ Public retained-clone integration laws compose published source and cursor protocols.
#![cfg(test)]

pub use semio_framework_value::*;
pub(crate) use semio_framework_value_testing_allocator::observe_retirement_allocations;

#[path="../../🧮️scalar/🧪️tests/🌐️lifecycle/🦀️.rs"]
mod scalar_lifecycle_tests;

#[path="../../🔮️demands/🧪️tests/🦀️.rs"]
mod normal_demand_tests;

#[path="../../../♻️retirement/🧪️tests/⏱️work/🦀️.rs"]
mod work;

#[path="../../🧺️std-collections/🧪️tests/🌐️integration/🦀️.rs"]
mod standard_collection_tests;
