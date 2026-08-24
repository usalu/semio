//! 🦀️ SVG 1.1 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🔣️qr-code.svg` fixture into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `quick-xml`
//! reference implementation (`../../🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `SvgSnapshot::import_utf8`/`export_utf8`/`apply_svg_mutation` over the full 11-kind `SvgMutation`
//! vocabulary. Both results are read back by the SAME independent `project_svg_1_1` (`quick-xml`)
//! before the `semantic-svg-1-1-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_svg_1_1};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `SvgMutation` variant, mirrored from
/// `../../🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio` (see this file's own header).
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-declaration", "set-doctype", "insert-element", "remove-element", "set-element-name", "set-attribute", "set-text", "set-view-box", "set-transform"];
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
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_svg_1_1(&bytes)?;
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

/// ↩️ The inverse law, ASSERTED on the ORACLE side rather than deferred to the parity phase:
/// `quick-xml` applies the kind and then its own computed inverse, and the restored drawing's
/// independent projection must equal the REAL original's own. Without this the scenario would only
/// prove that the reference library did not error, which is not what `@mode-property` claims -- and
/// with the subject phase blocked, this is the only place the law can be checked today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_svg_1_1(&bytes)?;
    let original = project_svg_1_1(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original drawing's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real drawing and re-serializes it from its own element tree alone, so BOTH
/// halves of the law are checkable here without a subject -- the re-encoded bytes must differ from
/// the input (SVG 1.1 is XML, not a byte-preserving carrier: the writer re-derives every tag, quote
/// and escape from the tree, so bit-identity would prove the artifact was copied rather than
/// parsed), and the re-encoded drawing's own projection must still equal the input's.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the drawing was parsed rather than copied".to_string());
    }
    let projection = project_svg_1_1(&bytes)?;
    let original = project_svg_1_1(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::{apply_svg_mutation, SvgMutation};
    use semio_s_plugin_stdio::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::{element_attr, node_at, parse_transform_list, parse_view_box, set_element_attr, view_box_to_string, NodePath, SvgSnapshot, TransformOp, ViewBox};
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::any::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlNode};
    use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::any::project_svg_1_1;

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
            Some(Json::Array(items)) => items.iter().map(|item| match item { Json::Number(n) => n.max(0.0) as usize, _ => 0 }).collect(),
            _ => Vec::new(),
        }
    }

    fn view_box_field(value: &Json, key: &str) -> Option<ViewBox> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 4 => {
                let n: Vec<f64> = items.iter().map(|item| match item { Json::Number(x) => *x, _ => 0.0 }).collect();
                Some(ViewBox { min_x: n[0], min_y: n[1], width: n[2], height: n[3] })
            }
            _ => None,
        }
    }

    fn transform_op_field(value: &Json) -> TransformOp {
        let num = |key: &str| number_field(value, key);
        let opt_num = |key: &str| match value.get(key) { Some(Json::Number(n)) => Some(*n), _ => None };
        match value.str("kind").as_str() {
            "matrix" => TransformOp::Matrix { a: num("a"), b: num("b"), c: num("c"), d: num("d"), e: num("e"), f: num("f") },
            "translate" => TransformOp::Translate { x: num("x"), y: opt_num("y") },
            "scale" => TransformOp::Scale { x: num("x"), y: opt_num("y") },
            "rotate" => TransformOp::Rotate { angle: num("angle"), center: match (opt_num("cx"), opt_num("cy")) { (Some(cx), Some(cy)) => Some((cx, cy)), _ => None } },
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

    /// 🔎️ The same owned node JSON grammar the oracle side speaks
    /// (`{"kind":"element"|"text"|"cdata"|"comment"|"pi", ...}`), decoded into the PRODUCTION
    /// `XmlNode` here instead of the oracle's own independent `QNode`.
    fn json_to_xml_node(value: &Json) -> XmlNode {
        match value.str("kind").as_str() {
            "text" => XmlNode::Text { text: value.str("text") },
            "cdata" => XmlNode::CData { text: value.str("text") },
            "comment" => XmlNode::Comment { text: value.str("text") },
            "pi" => XmlNode::ProcessingInstruction { target: value.str("target"), data: value.str("data") },
            _ => XmlNode::Element { name: value.str("name"), attrs: value.array("attrs").iter().map(|a| XmlAttr { name: a.str("name"), value: a.str("value") }).collect(), children: value.array("children").iter().map(json_to_xml_node).collect() },
        }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `SvgMutation` this
    /// subset declares for it. `set-snapshot` mirrors the oracle's own extension of "replace the
    /// document" -- only `rootId`/`viewBoxWidth` are spec-driven, applied on top of the currently
    /// decoded `base` snapshot so every other field (declaration, doctype, the rest of the tree)
    /// survives untouched.
    fn mutation_from_spec(spec: &Json, base: &SvgSnapshot) -> Result<SvgMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(SvgMutation::NoMutation),
            "set-snapshot" => {
                let mut snapshot = base.clone();
                if let Some(root) = snapshot.doc.root.as_mut() {
                    if let Some(id) = str_field(&params, "rootId") {
                        set_element_attr(root, "id", Some(id));
                    }
                    if let Some(width) = match params.get("viewBoxWidth") { Some(Json::Number(n)) => Some(*n), _ => None } {
                        let mut vb = element_attr(root, "viewBox").and_then(|s| parse_view_box(s).ok()).unwrap_or(ViewBox { min_x: 0.0, min_y: 0.0, width: 0.0, height: 0.0 });
                        vb.width = width;
                        set_element_attr(root, "viewBox", Some(view_box_to_string(&vb)));
                    }
                }
                Ok(SvgMutation::SetSnapshot { snapshot })
            }
            "set-declaration" => Ok(SvgMutation::SetDeclaration { declaration: str_field(&params, "version").map(|version| XmlDeclaration { version, encoding: str_field(&params, "encoding"), standalone: match params.get("standalone") { Some(Json::Bool(b)) => Some(*b), _ => None } }) }),
            "set-doctype" => Ok(SvgMutation::SetDoctype { doctype: str_field(&params, "doctype").map(|inner| XmlDoctype::from(format!("<!DOCTYPE {inner}>").as_str())) }),
            "insert-element" => Ok(SvgMutation::InsertElement { parent: path_field(&params, "parent"), index: usize_field(&params, "index"), node: json_to_xml_node(params.get("node").unwrap_or(&Json::Null)) }),
            "remove-element" => Ok(SvgMutation::RemoveElement { parent: path_field(&params, "parent"), index: usize_field(&params, "index") }),
            "set-element-name" => Ok(SvgMutation::SetElementName { path: path_field(&params, "path"), name: params.str("name") }),
            "set-attribute" => Ok(SvgMutation::SetAttribute { path: path_field(&params, "path"), name: params.str("name"), value: match params.get("value") { Some(Json::String(v)) => Some(v.clone()), _ => None } }),
            "set-text" => Ok(SvgMutation::SetText { path: path_field(&params, "path"), text: params.str("text") }),
            "set-view-box" => Ok(SvgMutation::SetViewBox { path: path_field(&params, "path"), view_box: view_box_field(&params, "viewBox") }),
            "set-transform" => Ok(SvgMutation::SetTransform { path: path_field(&params, "path"), transform: transform_field(&params, "transform") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `SvgMutation::inverse` in closed form (`../../🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/
    /// 🧬️mutations/🦀️component.rs`'s own `Mutation<SvgSnapshot>::inverse`), transplanted rather than
    /// called through the `protocol::Mutation` trait -- this adapter's generated crate never links
    /// `protocol` directly, only `semio-s-plugin-stdio` and `semio-s-plugin-stdio-test-oracle`.
    fn inverse_of(mutation: &SvgMutation, base: &SvgSnapshot) -> SvgMutation {
        match mutation {
            SvgMutation::NoMutation => SvgMutation::NoMutation,
            SvgMutation::SetSnapshot { .. } => SvgMutation::SetSnapshot { snapshot: base.clone() },
            SvgMutation::SetDeclaration { .. } => SvgMutation::SetDeclaration { declaration: base.doc.declaration.clone() },
            SvgMutation::SetDoctype { .. } => SvgMutation::SetDoctype { doctype: base.doc.doctype.clone() },
            SvgMutation::InsertElement { parent, index, .. } => SvgMutation::RemoveElement { parent: parent.clone(), index: *index },
            SvgMutation::RemoveElement { parent, index } => match node_at(&base.doc, parent) {
                Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => SvgMutation::InsertElement { parent: parent.clone(), index: *index, node: node.clone() },
                    None => SvgMutation::NoMutation,
                },
                _ => SvgMutation::NoMutation,
            },
            SvgMutation::SetElementName { path, .. } => match node_at(&base.doc, path) {
                Ok(XmlNode::Element { name, .. }) => SvgMutation::SetElementName { path: path.clone(), name: name.clone() },
                _ => SvgMutation::NoMutation,
            },
            SvgMutation::SetAttribute { path, name, .. } => SvgMutation::SetAttribute { path: path.clone(), name: name.clone(), value: node_at(&base.doc, path).ok().and_then(|node| element_attr(node, name)).map(|s| s.to_string()) },
            SvgMutation::SetText { path, .. } => {
                let old = match node_at(&base.doc, path) {
                    Ok(XmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                SvgMutation::SetText { path: path.clone(), text: old }
            }
            SvgMutation::SetViewBox { path, .. } => SvgMutation::SetViewBox { path: path.clone(), view_box: node_at(&base.doc, path).ok().and_then(|node| element_attr(node, "viewBox")).and_then(|v| parse_view_box(v).ok()) },
            SvgMutation::SetTransform { path, .. } => SvgMutation::SetTransform { path: path.clone(), transform: node_at(&base.doc, path).ok().and_then(|node| element_attr(node, "transform")).and_then(|v| parse_transform_list(v).ok()) },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_svg_mutation(&mut snapshot, &mutation);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_1_1(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_svg_mutation(&mut snapshot, &mutation);
        apply_svg_mutation(&mut snapshot, &undo);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_1_1(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `import_utf8`/`export_utf8` are this
    /// subset's ONLY channel from input to output (no separate text-DSL layer over the snapshot).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = SvgSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_svg_1_1(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 11-kind sweep for the subject
    /// role without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 11 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
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
