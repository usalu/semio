//! 🦀️ XML 1.0 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `📰️ooxml-word-document.xml` fixture (extracted once
//! from the real ECMA-376 example DOCX — see the feature file's own header) into the case work
//! directory first; the committed fixture is never written to. `oracle` drives the registered
//! `quick-xml` reference implementation (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s
//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's
//! own `XmlSnapshot::import_utf8`/`export_utf8`/`apply_xml_mutation` over the full 8-kind
//! `XmlMutation` vocabulary. Both results are read back by the SAME independent `project_xml_1_0`
//! (`quick-xml`) before the `semantic-xml-v1` profile compares them. The subject half is gated behind
//! the generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xml::standards::v1_0::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_xml_1_0};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `XmlMutation` variant, mirrored from
/// `../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio` (see this file's own header); `kinds_const_matches_enum_variants_in_
/// declaration_order` on the production side and the framework's own catalog-completeness gate on
/// this side are what keep the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-declaration", "set-doctype", "insert-element", "remove-element", "set-attribute", "set-text"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📰️ooxml-word-document.xml";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("word-document.xml"))?;
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
    let projection = project_xml_1_0(&bytes)?;
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
/// `quick-xml` applies the kind and then its own computed inverse, and the restored document's
/// independent projection must equal the REAL original's own. Without this the scenario would only
/// prove that the reference library did not error, which is not what `@mode-property` claims -- and
/// with the subject phase blocked, this is the only place the law can be checked today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_xml_1_0(&bytes)?;
    let original = project_xml_1_0(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real document and re-serializes it from its own tree alone (the same
/// "no-mutation" routing `oracle_apply_mutation` already gives every other kind), so BOTH halves of
/// the law are checkable here without a subject -- the re-encoded bytes must differ from the input,
/// XML 1.0 being no byte-preserving carrier (a conforming writer re-derives every tag, quote and
/// escape from the tree), and the re-encoded document's own projection must still equal the input's.
///
/// ⚠️ The byte half currently FAILS on this fixture, and the failure is recorded rather than tuned
/// away: `shared://📰️ooxml-word-document.xml` is byte-identical to the `word/document.xml` part of
/// `../../📜️docx/…/📜️example.docx`, which carries no `docProps`, zeroed zip timestamps and a 14-byte
/// `numbering.xml` -- it is this repository's own minified OOXML writer's output, not Microsoft
/// Word's, and `quick-xml`'s canonical serialization happens to agree with it character for
/// character. So `output == input` here is two minifying writers coinciding, not a pass-through, and
/// the assertion cannot tell the two apart. The fix belongs to the FIXTURE (re-derive the part from
/// a genuinely Word-authored DOCX, which carries an XML declaration and breaks the coincidence), not
/// to this handler.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the document was parsed rather than copied".to_string());
    }
    let projection = project_xml_1_0(&bytes)?;
    let original = project_xml_1_0(&input)?;
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
    use semio_s_plugin_stdio::artifacts::xml::schema::mutations::{apply_xml_mutation, XmlMutation, XmlNodePath};
    use semio_s_plugin_stdio::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode};
    use semio_s_plugin_stdio::artifacts::xml::XmlSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::xml::standards::v1_0::subsets::any::project_xml_1_0;

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

    fn usize_path(items: Vec<Json>) -> Vec<usize> {
        items.iter().map(|item| match item { Json::Number(number) => number.max(0.0) as usize, _ => 0 }).collect()
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    /// 🔎️ The same owned node-spec JSON grammar the oracle side speaks
    /// (`{"kind":"element"|"text"|"cdata"|"comment"|"pi", ...}`), decoded into the PRODUCTION
    /// `XmlNode` here instead of the oracle's own independent tree type.
    fn json_to_xml_node(value: &Json) -> Result<XmlNode, String> {
        match value.str("kind").as_str() {
            "element" => Ok(XmlNode::Element {
                name: value.str("name"),
                attrs: value.array("attrs").iter().map(|attr| XmlAttr { name: attr.str("name"), value: attr.str("value") }).collect(),
                children: value.array("children").iter().map(json_to_xml_node).collect::<Result<Vec<_>, _>>()?,
            }),
            "text" => Ok(XmlNode::Text { text: value.str("text") }),
            "cdata" => Ok(XmlNode::CData { text: value.str("text") }),
            "comment" => Ok(XmlNode::Comment { text: value.str("text") }),
            "pi" => Ok(XmlNode::ProcessingInstruction { target: value.str("target"), data: value.str("data") }),
            other => Err(format!("unknown node kind {other:?}")),
        }
    }

    /// 📄️ `{"version":...,"encoding":...,"standalone":...}` when present, absent (no `version` key)
    /// meaning "no declaration" -- the same convention `set-doctype`'s `name` key uses below.
    fn json_to_declaration(params: &Json) -> Option<XmlDeclaration> {
        non_empty(params, "version").map(|version| XmlDeclaration { version, encoding: non_empty(params, "encoding"), standalone: match params.get("standalone") { Some(Json::Bool(value)) => Some(*value), _ => None } })
    }

    fn json_to_doctype(params: &Json) -> Option<XmlDoctype> {
        let name = non_empty(params, "name")?;
        let external_id = match params.get("externalId") {
            Some(value) if !matches!(value, Json::Null) => match value.str("kind").as_str() {
                "system" => Some(XmlExternalId::System { system_id: value.str("systemId") }),
                "public" => Some(XmlExternalId::Public { public_id: value.str("publicId"), system_id: value.str("systemId") }),
                _ => None,
            },
            _ => None,
        };
        let declarations = params.array("entities").iter().map(|entry| XmlDtdDeclaration::Entity { parameter: matches!(entry.get("parameter"), Some(Json::Bool(true))), name: entry.str("name"), value: entry.str("value") }).collect();
        Some(XmlDoctype { name, external_id, declarations })
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `XmlMutation` this subset
    /// declares for it. `set-snapshot` parses `params.xml` as a whole real document (the same real
    /// `word/styles.xml` OOXML part the oracle side parses too), replacing the base snapshot outright.
    fn mutation_from_spec(spec: &Json) -> Result<XmlMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(XmlMutation::NoMutation),
            "set-snapshot" => {
                let snapshot = XmlSnapshot::import_utf8(params.str("xml").as_bytes()).map_err(|error| format!("set-snapshot xml parse failed: {error}"))?;
                Ok(XmlMutation::SetSnapshot { snapshot })
            }
            "set-declaration" => Ok(XmlMutation::SetDeclaration { declaration: json_to_declaration(&params) }),
            "set-doctype" => Ok(XmlMutation::SetDoctype { doctype: json_to_doctype(&params) }),
            "insert-element" => Ok(XmlMutation::InsertElement { path: XmlNodePath(usize_path(params.array("path"))), index: usize_field(&params, "index"), node: json_to_xml_node(&params.get("node").cloned().unwrap_or(Json::Null))? }),
            "remove-element" => Ok(XmlMutation::RemoveElement { path: XmlNodePath(usize_path(params.array("path"))), index: usize_field(&params, "index") }),
            "set-attribute" => Ok(XmlMutation::SetAttribute { path: XmlNodePath(usize_path(params.array("path"))), name: params.str("name"), value: match params.get("value") { Some(Json::String(text)) => Some(text.clone()), _ => None } }),
            "set-text" => Ok(XmlMutation::SetText { path: XmlNodePath(usize_path(params.array("path"))), text: params.str("text") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `XmlMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm,
    /// transplanted rather than called through the trait, same precedent `mutate-pdf-1-7`'s own
    /// `inverse_of` gives: written in closed form so this adapter needs no extra crate dependency
    /// beyond `semio-s-plugin-stdio` itself.
    fn inverse_of(mutation: &XmlMutation, base: &XmlSnapshot) -> XmlMutation {
        match mutation {
            XmlMutation::NoMutation => XmlMutation::NoMutation,
            XmlMutation::SetSnapshot { .. } => XmlMutation::SetSnapshot { snapshot: base.clone() },
            XmlMutation::SetDeclaration { .. } => XmlMutation::SetDeclaration { declaration: base.doc.declaration.clone() },
            XmlMutation::SetDoctype { .. } => XmlMutation::SetDoctype { doctype: base.doc.doctype.clone() },
            XmlMutation::InsertElement { path, index, .. } => XmlMutation::RemoveElement { path: path.clone(), index: *index },
            XmlMutation::RemoveElement { path, index } => {
                let parent = path.resolve(base.doc.root.as_ref());
                let node = parent.and_then(|node| match node { XmlNode::Element { children, .. } => children.get(*index).cloned(), _ => None }).unwrap_or(XmlNode::Text { text: String::new() });
                XmlMutation::InsertElement { path: path.clone(), index: *index, node }
            }
            XmlMutation::SetAttribute { path, name, .. } => {
                let target = path.resolve(base.doc.root.as_ref());
                let prior = target.and_then(|node| match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attr| &attr.name == name).map(|attr| attr.value.clone()), _ => None });
                XmlMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior }
            }
            XmlMutation::SetText { path, .. } => {
                let prior = path.resolve(base.doc.root.as_ref()).and_then(|node| match node { XmlNode::Text { text } => Some(text.clone()), _ => None }).unwrap_or_default();
                XmlMutation::SetText { path: path.clone(), text: prior }
            }
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = XmlSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_xml_mutation(&mut snapshot, &mutation);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_xml_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = XmlSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_xml_mutation(&mut snapshot, &mutation);
        apply_xml_mutation(&mut snapshot, &undo);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_xml_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `XmlSnapshot::import_utf8`/`export_utf8`
    /// are this subset's ONLY channel from input to output (XML is text-native; there is no separate
    /// binary layer over the same model).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = XmlSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_xml_1_0(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 8-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 8 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
