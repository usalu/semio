//! 💼️ Lift file: 1 item(s), about 9 lines, of `⚛️reactor/💼️jobs/🦀️.rs` that belong to crate `actor-host` (WP1).
//!
//! WP4 owns the source file and cuts the items here (cancel_job); WP1 wires this file with one `mod` line in the crate root. Until `actor-host` exists, the source module re-exports the items with `pub use`.
//!
//! 🛑️ WITHDRAWN by WP4 (`RESULT-PLUGIN-WP4.md`, plan defect PD5): nothing is cut here. `cancel_job` reads the `JOBS` static and the `JobSlot` of `reactor-jobs` (rank 35); lifting it into `actor-host` would be an upward edge. The plan's graph moved it because the token `cancel_job` also names the unrelated method `Host::cancel_job`. The module stays empty; WP1 may drop its mount line.
