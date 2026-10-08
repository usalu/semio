use super::table_text;
use crate::catalog::{PlaygroundEntry, Ports};

#[test]
fn table_text_preserves_catalog_order_and_row_wire_format() {
    let first = PlaygroundEntry { variant: "alpha".into(), plugin_id: "plugin-a".into(), ports: Ports { react: 3100, wgpu: 3101 }, ..Default::default() };
    let second = PlaygroundEntry { variant: "beta".into(), plugin_id: "plugin-b".into(), ports: Ports { react: 3200, wgpu: 3201 }, ..Default::default() };

    assert_eq!(table_text(&[first, second]), "alpha\tplugin-a\treact:3100\twgpu:3101\nbeta\tplugin-b\treact:3200\twgpu:3201\n");
}
