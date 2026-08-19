//! 📚️ Example "wires" for `stdio.semio.graph` — the first real, non-scaffold fixture for this
//! subset. `PRIMARY_TEXT` is meant to be the genuine `SemioGraphSnapshot::print_dsl` output for
//! `snapshot::demo_graph_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️component.rs`), asserted
//! byte-identical to it by this subset's own `fixture_honesty_law` (`🚪️io/🦀️component.rs`).
//!
//! `🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_graph_snapshot()`, captured via a temporary `[DEBUG]`-prefixed
//! `debug_dump_fixture_bytes` test in `📸️snapshot/🦀️component.rs` (now removed) once this subset
//! was mounted into the crate's module tree — verified byte-exact with `wc -c`/`xxd`.
//!
//! Named "wires" (not "graph") to avoid confusion with the unrelated existing `value`-subset
//! example already at `✳️any/📚️examples/🕸️graph/`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "wires";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Wires", "Verdrahtung")
}
pub const ICON: &str = "share-2";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    async fn wires_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
