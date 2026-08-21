use crate::args::ParsedArgs;
use crate::catalog::{load_playground_catalog, playgrounds_json_text, PlaygroundEntry};
use std::path::Path;

// #region 🔖️Command
/// 📇️ Lists generated playground registrations as JSON or tabular terminal text.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    if parsed.has_flag("json") {
        print!("{}", playgrounds_json_text(root));
        return 0;
    }
    print!("{}", table_text(&load_playground_catalog(root)));
    0
}
// #endregion 🔖️Command

// #region 🔖️Presentation
fn table_text(catalog: &[PlaygroundEntry]) -> String {
    catalog.iter().map(|row| format!("{}\t{}\treact:{}\twgpu:{}\n", row.variant, row.plugin_id, row.ports.react, row.ports.wgpu)).collect()
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::table_text;
    use crate::catalog::PlaygroundEntry;

    #[test]
    fn table_text_preserves_catalog_order_and_row_wire_format() {
        let mut first = PlaygroundEntry::default();
        first.variant = "alpha".into();
        first.plugin_id = "plugin-a".into();
        first.ports.react = 3100;
        first.ports.wgpu = 3101;
        let mut second = PlaygroundEntry::default();
        second.variant = "beta".into();
        second.plugin_id = "plugin-b".into();
        second.ports.react = 3200;
        second.ports.wgpu = 3201;

        assert_eq!(table_text(&[first, second]), "alpha\tplugin-a\treact:3100\twgpu:3101\nbeta\tplugin-b\treact:3200\twgpu:3201\n");
    }
}
// #endregion 🔖️Tests
