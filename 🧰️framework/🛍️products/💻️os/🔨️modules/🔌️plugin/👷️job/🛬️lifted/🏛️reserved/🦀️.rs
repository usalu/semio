//! 🏛️ Lift file: 2 item(s), about 145 lines, of `⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs` that belong to crate `job` (WP3).
//!
//! WP4 owns the source file and cuts the items here (drive_framework_reserved_registry_job, drive_framework_reserved_registry_job_counted); WP3 wires this file with one `mod` line in the crate root. Until `job` exists, the source module re-exports the items with `pub use`.
//!
//! 🛑️ WITHDRAWN by WP4 (`RESULT-PLUGIN-WP4.md`, plan defect PD6): nothing is cut here. `drive_framework_reserved_registry_job` and its counted twin call `start_job`, `step_job`, `job_admission_demands`, `IoRunControl` and `SqliteSnapshotControl` of `reactor-jobs` (rank 35); lifting them into `job` would be an upward edge. They stay in `reactor-jobs`; the instance composer that calls them needs a `reactor-jobs` dependency (request to WP0 for Appendix H).
