//! 📚️ The example registry — the ONE array every remodeling example registers in, and the only thing
//! `🎮️commands/🎬️set-active-example` and the manifest read. Before it existed the two committed example
//! leaves were mounted `pub mod`s nothing referenced.
//!
//! ➕️ To add an example: mount its leaf in `📦️packages/🦀️rust/🦀️.rs` and append ONE row to
//! [`REMODELING_EXAMPLES`]. Nothing else in the crate needs touching — the manifest action, the
//! command bridge and the boot document all read this array.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

//#region 🔖️Registry
/// 📚️ One registered example: the id the `setActiveExample` action carries, the committed text, and the
/// picker chrome. `text` is `.dsl.semio` document text for a document example and `.cmd.semio` replay
/// text for a session example — [`REMODELING_EXAMPLE_BOOT_ID`] names the one that boots the editor.
pub struct RemodelingExample {
    pub id: &'static str,
    pub text: &'static str,
    pub icon: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

/// 📚️ Every committed example of this subset, in picker order. **Append-only** — see this module's doc.
pub const REMODELING_EXAMPLES: &[RemodelingExample] = &[
    RemodelingExample {
        id: crate::examples::art_remodeling_demo::ID,
        text: crate::examples::art_remodeling_demo::PRIMARY_TEXT,
        icon: crate::examples::art_remodeling_demo::ICON,
        label_en: "Demo",
        label_de: "Demo",
    },
    RemodelingExample {
        id: crate::examples::app_remodeling_demo_session::ID,
        text: crate::examples::app_remodeling_demo_session::PRIMARY_TEXT,
        icon: crate::examples::app_remodeling_demo_session::ICON,
        label_en: "Demo Session",
        label_de: "Demo-Sitzung",
    },
];

/// 🚀️ The example the editor boots on when its committed text parses — otherwise
/// `default_remodeling_scene()` stands in, exactly the fallback shape `🧱️block`'s
/// `default_block2d_snapshot()` uses.
pub const REMODELING_EXAMPLE_BOOT_ID: &str = "demo";

/// 🔍️ The committed text of one registered example, or `None` for an unregistered id.
pub fn example_text(id: &str) -> Option<&'static str> {
    REMODELING_EXAMPLES.iter().find(|example| example.id == id).map(|example| example.text)
}

/// 🏷️ Picker chrome for one registered example.
pub fn example_label(example: &RemodelingExample) -> LocalizedLabel {
    LocalizedLabel::native(example.label_en, example.label_de)
}

/// 📚️ Every registered example as an `ExampleSource` — the shape `App::example_source` and
/// `SubsetDeclaration.examples` both consume once this plugin's root moves onto the declaration tree
/// (see `create_remodeling_app`'s SDK-gap note).
pub fn example_sources() -> Vec<ExampleSource> {
    REMODELING_EXAMPLES.iter().map(|example| ExampleSource::new(example.id, example_label(example), example.text, example.icon)).collect()
}

/// 🚀️ The boot document: the registered boot example's text parsed as a scene, or the artifact's own
/// default scene when that text is absent or no longer parses.
pub fn boot_snapshot() -> crate::artifacts::remodeling::RemodelingSnapshot {
    example_text(REMODELING_EXAMPLE_BOOT_ID)
        .and_then(|text| crate::artifacts::remodeling::snapshot::text::parse_dsl(text).ok())
        .unwrap_or_else(crate::artifacts::remodeling::default_remodeling_scene)
}
//#endregion 🔖️Registry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn example_ids_are_unique_and_resolvable() {
        let ids: std::collections::BTreeSet<&str> = REMODELING_EXAMPLES.iter().map(|example| example.id).collect();
        assert_eq!(ids.len(), REMODELING_EXAMPLES.len(), "example ids must be unique");
        assert!(ids.contains(REMODELING_EXAMPLE_BOOT_ID), "the boot example must be registered");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_source_carries_its_committed_text() {
        let sources = example_sources();
        assert_eq!(sources.len(), REMODELING_EXAMPLES.len());
        for example in REMODELING_EXAMPLES {
            assert_eq!(example_text(example.id), Some(example.text));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn the_boot_document_parses_the_demo_example() {
        assert_eq!(boot_snapshot(), crate::artifacts::remodeling::snapshot::text::parse_dsl(example_text(REMODELING_EXAMPLE_BOOT_ID).expect("boot example text")).expect("boot example parses"));
    }
}
//#endregion 🧪️Tests
