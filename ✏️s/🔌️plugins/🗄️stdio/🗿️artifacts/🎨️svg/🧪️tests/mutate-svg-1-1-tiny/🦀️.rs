//! 🦀️ SVG 1.1/✳️tiny exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! Every scenario copies the real, committed `qr-code.svg` fixture into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `quick-xml`
//! reference implementation through this subset's own oracle module
//! (`../../🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧪️oracle/🦀️component.rs`); `subject` drives this
//! repository's own `SvgSnapshot::import_utf8`/`export_utf8` and `apply_svg_tiny_mutation` over the
//! full 9-kind `SvgTinyMutation` vocabulary. Both results are read back by the SAME independent
//! `project_svg_tiny` before the `semantic-svg-tiny-1-1-v1` profile compares them. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::tiny::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_svg_tiny};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `SvgTinyMutation` variant, mirrored from
/// `../../🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio`.
const KINDS: &[&str] = &["set-snapshot", "stamp-base-profile", "insert-tiny-element", "remove-element", "set-tiny-attribute", "set-text", "set-view-box", "set-transform", "strip-non-tiny"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://qr-code.svg";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("qr-code.svg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind other than
/// `no-mutation` whose parameters leave the semantic projection exactly where it was has not been
/// tested by this scenario at all -- it proves only that the reference library declined to error.
/// Every `Examples` row is chosen against the real artifact's actual content for that reason, and
/// this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_svg_tiny(&bytes)?;
    if kind != "no-mutation" && projection_divergence(&projection, &project_svg_tiny(&input)?).is_none() {
        return Err(format!("{kind:?} left the semantic projection exactly as it found it -- a mutation whose parameters make it a no-op against the real artifact is not a test of that kind"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ⚖️ First point at which two projections diverge, as a character offset into the canonical
/// rendering plus the window around it on both sides -- an equality check whose failure names WHAT
/// changed rather than only that something did.
fn projection_divergence(restored: &Json, original: &Json) -> Option<String> {
    let (left, right): (Vec<char>, Vec<char>) = (restored.to_string().chars().collect(), original.to_string().chars().collect());
    if left == right {
        return None;
    }
    let at = left.iter().zip(right.iter()).position(|(a, b)| a != b).unwrap_or(left.len().min(right.len()));
    let window = |text: &[char]| text.iter().skip(at.saturating_sub(60)).take(160).collect::<String>();
    Some(format!("first divergence at char {at} of {} vs {} -- got …{}… want …{}…", left.len(), right.len(), window(&left), window(&right)))
}

/// ↩️ The inverse law, checked on the ORACLE side rather than deferred to the parity phase: the
/// reference implementation applies the kind and then its own computed inverse, and the restored
/// document's independent projection must equal the REAL original's own. Without this the scenario
/// would only prove that the inverse ran without erroring, which is not what `@mode-property`
/// claims — and with the subject phase blocked, it is the only place the property can be checked
/// today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation_inverse(&input, &ctx.doc_json()?)?;
    let projection = project_svg_tiny(&bytes)?;
    let original = project_svg_tiny(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: the restored drawing's projection differs from the original's -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real drawing and re-serializes it from its own element tree alone, so BOTH
/// halves of the law are checkable here without a subject -- the re-encoded bytes must differ from
/// the input (SVG 1.1 ✳️tiny is XML, not a byte-preserving carrier: the writer re-derives every
/// tag, quote and escape from the tree, so bit-identity would prove the artifact was copied rather
/// than parsed), and the re-encoded drawing's own projection must still equal the input's.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the drawing was parsed rather than copied".to_string());
    }
    let projection = project_svg_tiny(&bytes)?;
    let original = project_svg_tiny(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::base::schema::snapshot::{element_attr, parse_view_box, set_element_attr, view_box_to_string, NodePath, SvgSnapshot, TransformOp, ViewBox};
    use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::tiny::schema::mutations::{apply_svg_tiny_mutation, insert_tiny_element, inverse_svg_tiny_mutation, remove_element, set_snapshot, set_text, set_tiny_attribute, set_transform, set_view_box, stamp_base_profile, strip_non_tiny, SvgTinyMutation};
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::base::schema::snapshot::{XmlAttr, XmlNode};
    use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::tiny::project_svg_tiny;

    //#region 🔖️SpecCodec
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }

    fn str_field(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    fn path_field(value: &Json, key: &str) -> NodePath {
        match value.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(n) => n.max(0.0) as usize,
                    _ => 0,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn view_box_field(value: &Json, key: &str) -> Option<ViewBox> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 4 => {
                let n: Vec<f64> = items
                    .iter()
                    .map(|item| match item {
                        Json::Number(x) => *x,
                        _ => 0.0,
                    })
                    .collect();
                Some(ViewBox { min_x: n[0], min_y: n[1], width: n[2], height: n[3] })
            }
            _ => None,
        }
    }

    fn transform_op_field(value: &Json) -> TransformOp {
        let num = |key: &str| number_field(value, key);
        let opt_num = |key: &str| match value.get(key) {
            Some(Json::Number(n)) => Some(*n),
            _ => None,
        };
        match value.str("kind").as_str() {
            "matrix" => TransformOp::Matrix { a: num("a"), b: num("b"), c: num("c"), d: num("d"), e: num("e"), f: num("f") },
            "translate" => TransformOp::Translate { x: num("x"), y: opt_num("y") },
            "scale" => TransformOp::Scale { x: num("x"), y: opt_num("y") },
            "rotate" => TransformOp::Rotate {
                angle: num("angle"),
                center: match (opt_num("cx"), opt_num("cy")) {
                    (Some(cx), Some(cy)) => Some((cx, cy)),
                    _ => None,
                },
            },
            "skewX" => TransformOp::SkewX { angle: num("angle") },
            _ => TransformOp::SkewY { angle: num("angle") },
        }
    }

    fn transform_field(value: &Json, key: &str) -> Option<Vec<TransformOp>> {
        match value.get(key) {
            Some(Json::Array(items)) => Some(items.iter().map(transform_op_field).collect()),
            _ => None,
        }
    }

    /// 🔎️ The same owned node JSON grammar the oracle side speaks, decoded into the PRODUCTION
    /// `XmlNode` here instead of the oracle's own independent tree type.
    fn json_to_xml_node(value: &Json) -> XmlNode {
        match value.str("kind").as_str() {
            "text" => XmlNode::Text { text: value.str("text") },
            "cdata" => XmlNode::CData { text: value.str("text") },
            "comment" => XmlNode::Comment { text: value.str("text") },
            "pi" => XmlNode::ProcessingInstruction { target: value.str("target"), data: value.str("data") },
            _ => XmlNode::Element { name: value.str("name"), attrs: value.array("attrs").iter().map(|a| XmlAttr { name: a.str("name"), value: a.str("value") }).collect(), children: value.array("children").iter().map(json_to_xml_node).collect() },
        }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `SvgTinyMutation` this
    /// subset declares for it. An undeclared kind is an error, never a silent no-op.
    fn mutation_from_spec(spec: &Json, base: &SvgSnapshot) -> Result<SvgTinyMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "set-snapshot" => {
                let mut snapshot = base.clone();
                if let Some(root) = snapshot.doc.root.as_mut() {
                    if let Some(id) = str_field(&params, "rootId") {
                        set_element_attr(root, "id", Some(id));
                    }
                    if let Some(width) = match params.get("viewBoxWidth") {
                        Some(Json::Number(n)) => Some(*n),
                        _ => None,
                    } {
                        let mut view_box = element_attr(root, "viewBox").and_then(|s| parse_view_box(s).ok()).unwrap_or(ViewBox { min_x: 0.0, min_y: 0.0, width: 0.0, height: 0.0 });
                        view_box.width = width;
                        set_element_attr(root, "viewBox", Some(view_box_to_string(&view_box)));
                    }
                }
                Ok(SvgTinyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            "stamp-base-profile" => Ok(SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: str_field(&params, "baseProfile"), version: str_field(&params, "version") })),
            "insert-tiny-element" => Ok(SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: path_field(&params, "parent"), index: usize_field(&params, "index"), node: json_to_xml_node(params.get("node").unwrap_or(&Json::Null)) })),
            "remove-element" => Ok(SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: path_field(&params, "parent"), index: usize_field(&params, "index") })),
            "set-tiny-attribute" => Ok(SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute {
                path: path_field(&params, "path"),
                name: params.str("name"),
                value: match params.get("value") {
                    Some(Json::String(v)) => Some(v.clone()),
                    _ => None,
                },
            })),
            "set-text" => Ok(SvgTinyMutation::SetText(set_text::SetText { path: path_field(&params, "path"), text: params.str("text") })),
            "set-view-box" => Ok(SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: path_field(&params, "path"), view_box: view_box_field(&params, "viewBox") })),
            "set-transform" => Ok(SvgTinyMutation::SetTransform(set_transform::SetTransform { path: path_field(&params, "path"), transform: transform_field(&params, "transform") })),
            "strip-non-tiny" => Ok(SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {})),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    fn base_snapshot(ctx: &Context) -> Result<SvgSnapshot, String> {
        SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))
    }

    fn outcome_of(snapshot: &SvgSnapshot) -> Result<Outcome, String> {
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_tiny(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_svg_tiny_mutation(&mut snapshot, &mutation);
        outcome_of(&snapshot)
    }

    /// ↩️ The subset's OWN `Mutation::inverse`, reached through the typed vocabulary rather than
    /// re-derived here, so the property under test is the implementation's algebra and not a
    /// transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let undo = inverse_svg_tiny_mutation(&mutation, &base);
        let mut snapshot = base;
        apply_svg_tiny_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_svg_tiny_mutation(&mut snapshot, step);
        }
        outcome_of(&snapshot)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = SvgSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_svg_tiny(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 9 kinds — the scenario id only selects which Examples row's
/// `<id>`/`<params>` doc string the shared handler reads.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
