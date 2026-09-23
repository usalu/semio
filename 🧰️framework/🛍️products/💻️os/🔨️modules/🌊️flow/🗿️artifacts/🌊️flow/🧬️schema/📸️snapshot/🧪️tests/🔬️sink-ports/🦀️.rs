use super::neural::ColdRetire;
use super::*;

#[test]
fn preview_widget_io_ports_returns_one_input() {
    let widget = Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: OrderedSet::new() };
    let (inputs, outputs, _, _) = widget_io_ports(&widget, &[], &HashMap::new());
    assert_eq!(inputs.len(), 1);
    assert!(outputs.is_empty());
    let node = widget_to_dag_node(&widget, 0, &OrderedMap::new(), &[], &HashMap::new(), (40.0, 28.0));
    assert_eq!(inputs.as_slice(), node.inputs());
    let Widget::OutputPreview { preview, .. } = widget else {
        unreachable!();
    };
    ColdRetire::retire_cold(preview);
}
