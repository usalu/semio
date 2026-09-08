
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
