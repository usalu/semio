//! 🖼️ The page exports run on the BUNDLED example, evaluated live: the flow host with the packaged
//! `draw` flow extension its `neuron-kind`s resolve to linked in-process (the served guest receives it
//! as a host contribution instead). Its own test binary: a linked operator pack is process-wide, and
//! the lib tests pin the bare host, where the demo's operators are unknown.
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::io::export::serializers::artifacts::{dxf::v_r12::any as dxf_out, pdf::v1_4::any as pdf_out, png::v1_2::any as png_out, svg::v1_1::any as svg_out};
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::io::generation2d_drawing;
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::io::text::snapshot::default_snapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_framework_os_flow::register_linked_flow_extension_installer("draw", semio_s_plugin_flow_extension_draw::register);
        semio_framework_os_flow::install_flow_extension_manifest("flow-extension-draw", &semio_s_plugin_flow_extension_draw::extension_manifest_json()).expect("draw extension admission");
    });
}

/// 📐️ The example's slider (30) drives a `draw.shape.rect` of width 30 and default height 10 at the
/// origin, outlined by `draw.style.stroke` (black, width 1); every page export carries exactly that
/// rectangle on a page padded by 16 on each side.
#[test]
fn bundled_example_exports_its_evaluated_rectangle_to_every_page_format() {
    installed();
    let example = default_snapshot();
    let drawing = generation2d_drawing(&example).expect("the bundled example evaluates to a drawing");
    assert_eq!((drawing.canvas.width, drawing.canvas.height), (30.0 + 32.0, 10.0 + 32.0));
    let svg = String::from_utf8(svg_out::serialize_bytes(&example).expect("svg")).expect("utf-8");
    assert!(svg.starts_with("<svg") || svg.contains("<svg "), "{svg}");
    assert_eq!(svg.matches("<path").count(), 1, "{svg}");
    assert!(pdf_out::serialize_bytes(&example).expect("pdf").starts_with(b"%PDF-1.4"));
    let png = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::project_png(&png_out::serialize_bytes(&example).expect("png")).expect("decodes as png");
    assert_eq!((png.width, png.height), (62, 42));
    assert!(png.pixels.chunks(4).filter(|px| px[3] > 0).count() >= 2 * (30 + 10), "the rectangle outline is painted");
    let alpha = |x: u32, y: u32| png.pixels[((y * png.width + x) * 4 + 3) as usize];
    assert!(alpha(31, 16) > 0 && alpha(46, 21) > 0, "the outline sits inside the 16-unit page margin");
    assert_eq!((alpha(4, 4), alpha(31, 21), alpha(57, 37)), (0, 0, 0), "margin and interior stay empty");
    assert!(svg.contains("transform=\"matrix(1,0,") && svg.contains(",1,16,16)\""), "the page margin reaches the svg: {svg}");
    let dxf = String::from_utf8(dxf_out::serialize_bytes(&example).expect("dxf")).expect("dxf text");
    assert!(dxf.contains("POLYLINE") || dxf.contains("LWPOLYLINE"), "{dxf}");
    example.retire_cold();
}

/// 🧾️ Every `{operator}@{input}` of the bundled example whose declared default the operator does not record, plus every
/// operator whose kind the installed `draw` extension does not know — empty for a self-describing example.
fn unrecorded_example_defaults() -> Vec<String> {
    use semio_framework_artifact_flow_flow::neural::ColdRetire;
    use semio_framework_artifact_flow_flow::{default_neuron_params_from_info, Widget};
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let example = default_snapshot();
    let unrecorded = example.host_snapshot.widgets.iter().filter_map(|widget| match widget { Widget::Neuron { id, neuron_kind, params, .. } => Some((id, neuron_kind, params)), _ => None }).flat_map(|(id, kind, params)| {
        let declared = default_neuron_params_from_info(infos.get(kind));
        let missing = if infos.contains_key(kind) { declared.keys().filter(|key| params.get(key).is_none()).map(|key| format!("{id}@{key}")).collect() } else { vec![format!("{id}: unknown kind {kind}")] };
        declared.retire_cold();
        missing
    }).collect();
    example.retire_cold();
    unrecorded
}

/// ⚖️ LAW (design §20.9): the bundled example is a self-describing document — each operator records every declared input's
/// default literal in its `params`, exactly as an inserted operator does — so an input edit on the loaded example folds from
/// the snapshot alone and never meets `mutation.target-missing`.
#[test]
fn the_bundled_example_records_every_declared_default_of_its_operators() {
    installed();
    let unrecorded = unrecorded_example_defaults();
    assert!(unrecorded.is_empty(), "every operator of the bundled example records its declared defaults: {unrecorded:?}");
}

/// 🐛️ [DEBUG] Temporary: prints the bundled example with its operators' declared defaults recorded, for the one-time
/// normalization of the example asset (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). Deleted once the asset holds it.
#[test]
fn debug_print_normalized_example() {
    use semio_framework_artifact_flow_flow::neural::ColdRetire;
    use semio_framework_artifact_flow_flow::{default_neuron_params_from_info, Widget};
    installed();
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let mut example = default_snapshot();
    for widget in example.host_snapshot.widgets.iter_mut() {
        if let Widget::Neuron { neuron_kind, params, .. } = widget {
            let declared = default_neuron_params_from_info(infos.get(neuron_kind));
            let recorded = declared.iter().filter(|(key, _)| params.get(key).is_none()).fold(params.clone(), |recorded, (key, value)| recorded.insert(key.clone(), value.clone()));
            std::mem::replace(params, recorded).retire_cold();
            declared.retire_cold();
        }
    }
    eprintln!("[DEBUG] BEGIN demo\n{}\n[DEBUG] END demo", semio_framework_os_kernel::ArtifactDsl::print_dsl(&example));
    example.retire_cold();
}
