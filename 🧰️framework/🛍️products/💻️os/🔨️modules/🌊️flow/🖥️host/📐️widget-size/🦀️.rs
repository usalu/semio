//! 📐️ Node size of one widget from its display metadata and port layout.

use super::*;

pub fn widget_node_size(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> (f64, f64) {
    let label = widget_label(widget);
    match widget {
        Widget::InputSlider { .. } => {
            let output = IoPortSpec::named("N", "Num", "number", "Number");
            (dag::slider_widget_width(&label, &output), dag::slider_widget_height())
        }
        Widget::InputNote { text, .. } => dag::note_widget_size(text),
        Widget::OutputAction { .. } | Widget::OutputExport { .. } => (dag::io_widget_width(&label), dag::io_widget_height(&label)),
        Widget::InputImage { src, .. } => dag::image_widget_size(src),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_os_infinite_directed_presentation::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(1, 1, false, false))
        }
        Widget::OutputPreview { preview, expanded, .. } => dag::preview_widget_size(&dag_preview_content_from_dict(preview), &expanded.iter().cloned().collect()),
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_os_infinite_directed_presentation::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(inputs.len(), outputs.len(), variadic_inputs, variadic_outputs))
        }
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = semio_framework_os_infinite_directed_presentation::normalize_node_display(&display_name, &abbreviation);
            (dag::computation_node_width(&normalized_name, &inputs, &outputs), dag::computation_node_height(inputs.len(), outputs.len(), false, false))
        }
    }
}
