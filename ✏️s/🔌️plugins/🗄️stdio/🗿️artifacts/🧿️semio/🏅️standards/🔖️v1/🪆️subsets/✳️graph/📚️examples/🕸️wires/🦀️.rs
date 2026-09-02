//! 📚️ Example "wires" for `stdio.semio.graph` — the first real, non-scaffold fixture for this
//! subset. `PRIMARY_TEXT` is meant to be the genuine `SemioGraphSnapshot::print_dsl` output for
//! `snapshot::demo_graph_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️.rs`), asserted
//! byte-identical to it by this subset's own `fixture_honesty_law` (`🚪️io/🦀️.rs`).
//!
//! `🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_graph_snapshot()`, captured via a temporary `[DEBUG]`-prefixed
//! `debug_dump_fixture_bytes` test in `📸️snapshot/🦀️.rs` (now removed) once this subset
//! was mounted into the crate's module tree — verified byte-exact with `wc -c`/`xxd`.
//!
//! Named "wires" (not "graph") to avoid confusion with the unrelated existing `value`-subset
//! example already at `✳️any/📚️examples/🕸️graph/`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "wires";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Wires", "Verdrahtung")
}
pub const ICON: &str = "share-2";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn wires_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
