//! 🪟️ Lift file: 1 item(s), about 5 lines, of `⚛️reactor/🪟️surfaces/🦀️.rs` that belong to crate `reactor-surfaces` (WP4).
//!
//! WP2 owns the source file and cuts the items here (impl Default for SurfaceContex); WP4 wires this file with one `mod` line in the crate root. Until `reactor-surfaces` exists, the source module re-exports the items with `pub use`.
//!
//! 🛑️ WITHDRAWN by WP4 (`RESULT-PLUGIN-WP4.md`, plan defect PD8): nothing is cut here. `impl Default for SurfaceContexts` is an orphan-rule impl: `Default` and `SurfaceContexts` both live outside `reactor-surfaces`, so the impl stays with the type in `plugin-program` (WP2).
