//! 📚️ Example `demo` — an empty space index, matching the freshly-created-space default shape.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "layout-grid";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

    #[test]
    async fn bundled_example_parses_as_a_valid_space_index() {
        let document = <SSpaceSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("bundled example parses");
        assert_eq!(document.space_id, "demo-space");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }
}
//#endregion 🧪️Tests
