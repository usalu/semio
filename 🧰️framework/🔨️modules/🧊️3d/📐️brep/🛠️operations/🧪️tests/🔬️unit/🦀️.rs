//! 🧪️ Companion white-box laws of `semio-framework-3d-brep-construction`.

#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

use semio_framework_3d_testing_support as test_support;

#[path = "../../🧱️primitives/🧪️tests/🔬️unit/🦀️.rs"]
mod primitives_tests;
#[path = "../../🧱️primitives/🧪️tests/🔬️topology-seed/🦀️.rs"]
mod primitives_topology_seed_tests;
#[path = "../../🧱️primitives/🧪️tests/🔬️topology-reachability/🦀️.rs"]
mod primitives_topology_reachability_tests;
#[path = "../../🧱️primitives/🧪️tests/🔬️topology-compaction/🦀️.rs"]
mod primitives_topology_compaction_tests;
#[path = "../../🧵️sew/🧪️tests/🔬️heal/🦀️.rs"]
mod sew_heal_tests;
