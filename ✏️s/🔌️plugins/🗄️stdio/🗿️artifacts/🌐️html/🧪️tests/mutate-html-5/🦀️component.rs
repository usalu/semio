//! 🦀️ HTML 5 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, derived `🌐️zukunft-bau-entwerfen-mit-bestand.html` fixture (see
//! the feature file's own header for the derivation note) into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `html5ever`/
//! `markup5ever_rcdom` reference implementation
//! (`../../🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `parse_html_document`/`write_html_document`/`apply_html_mutation` over the full 10-kind
//! `HtmlMutation` vocabulary. Both results are read back by the SAME independent `project_html_5`
//! (`html5ever`) before the `semantic-html-v1` profile compares them. The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::html::standards::v5::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_html_5};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `HtmlMutation` variant, mirrored from
/// `../../🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio` (see this file's own header); `kinds_const_matches_enum_variants_in_
/// declaration_order` on the production side and the framework's own catalog-completeness gate on
/// this side are what keep the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-doctype", "insert-node", "remove-node", "set-element-name", "set-attribute", "set-text", "set-comment", "set-raw-text"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🌐️zukunft-bau-entwerfen-mit-bestand.html";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.html"))?;
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
    let projection = project_html_5(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ One handler shared by every `inverse-<kind>` scenario id.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_html_5(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law: `html5ever` fully parses the real document
/// and re-serializes it from its own tree alone (the same "no-mutation" routing `oracle_apply_mutation`
/// already gives every other kind), independent evidence that a full parse/re-serialize is possible
/// before the SUBJECT is held to the same standard below.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let projection = project_html_5(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::mutations::{apply_html_mutation, HtmlMutation};
    use semio_s_plugin_stdio::artifacts::html::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, parse_html_document, write_html_document, HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind, STDIO_HTML_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::html::standards::v5::subsets::any::project_html_5;

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

    fn optional_string(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }

    /// 🏳️ Tri-state attribute value read from a mutation spec's `value` key -- mirrors the oracle's
    /// own `tristate_value` exactly: the key ABSENT means "remove" (`None`), present and `Json::Null`
    /// means "valueless" (`Some(None)`), present and a string means "set to that value" (`Some(Some(v))`).
    fn tristate_value(params: &Json) -> Option<Option<String>> {
        match params.get("value") {
            None => None,
            Some(Json::Null) => Some(None),
            Some(Json::String(text)) => Some(Some(text.clone())),
            Some(_) => Some(None),
        }
    }

    /// 🔎️ The same owned node-spec JSON grammar the oracle side speaks (`{"kind":"element","name":
    /// ...,"attributes":[{"name":...,"value":string|null}],"children":[...]}` |
    /// `{"kind":"text"|"comment","text":...}` | `{"kind":"rawText","parentKind":"script"|"style",
    /// "text":...}`), decoded into the PRODUCTION `HtmlNode` here instead of the oracle's own
    /// independent tree type.
    fn json_to_html_node(value: &Json) -> Result<HtmlNode, String> {
        match value.str("kind").as_str() {
            "element" => Ok(HtmlNode::Element {
                name: value.str("name"),
                attributes: value.array("attributes").iter().map(|attr| HtmlAttr { name: attr.str("name"), value: match attr.get("value") { Some(Json::String(text)) => Some(text.clone()), _ => None } }).collect(),
                children: value.array("children").iter().map(json_to_html_node).collect::<Result<Vec<_>, _>>()?,
            }),
            "text" => Ok(HtmlNode::Text { text: value.str("text") }),
            "comment" => Ok(HtmlNode::Comment { text: value.str("text") }),
            "rawText" => Ok(HtmlNode::RawText { parent_kind: if value.str("parentKind") == "style" { RawTextKind::Style } else { RawTextKind::Script }, text: value.str("text") }),
            other => Err(format!("unknown node kind {other:?}")),
        }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `HtmlMutation` this subset
    /// declares for it.
    fn mutation_from_spec(spec: &Json) -> Result<HtmlMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(HtmlMutation::NoMutation),
            "set-snapshot" => Ok(HtmlMutation::SetSnapshot { snapshot: HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: optional_string(&params, "doctype"), root: json_to_html_node(&params.get("root").cloned().unwrap_or(Json::Null))? } }),
            "set-doctype" => Ok(HtmlMutation::SetDoctype { doctype: optional_string(&params, "doctype") }),
            "insert-node" => Ok(HtmlMutation::InsertNode { parent: usize_path(params.array("parent")), index: usize_field(&params, "index"), node: json_to_html_node(&params.get("node").cloned().unwrap_or(Json::Null))? }),
            "remove-node" => Ok(HtmlMutation::RemoveNode { parent: usize_path(params.array("parent")), index: usize_field(&params, "index") }),
            "set-element-name" => Ok(HtmlMutation::SetElementName { path: usize_path(params.array("path")), name: params.str("name") }),
            "set-attribute" => Ok(HtmlMutation::SetAttribute { path: usize_path(params.array("path")), name: params.str("name"), value: tristate_value(&params) }),
            "set-text" => Ok(HtmlMutation::SetText { path: usize_path(params.array("path")), text: params.str("text") }),
            "set-comment" => Ok(HtmlMutation::SetComment { path: usize_path(params.array("path")), text: params.str("text") }),
            "set-raw-text" => Ok(HtmlMutation::SetRawText { path: usize_path(params.array("path")), text: params.str("text") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// ↩️ `HtmlMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm
    /// (`../../🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`), transplanted
    /// rather than called through the trait, same precedent `mutate-xml-1-0`'s own `inverse_of` gives:
    /// written in closed form so this adapter needs no extra crate dependency beyond
    /// `semio-s-plugin-stdio` itself, and reads `base`'s PRIOR state through the same `node_at`/
    /// `element_attr` navigation helpers the production `inverse()` itself uses.
    fn inverse_of(mutation: &HtmlMutation, base: &HtmlSnapshot) -> HtmlMutation {
        match mutation {
            HtmlMutation::NoMutation => HtmlMutation::NoMutation,
            HtmlMutation::SetSnapshot { .. } => HtmlMutation::SetSnapshot { snapshot: base.clone() },
            HtmlMutation::SetDoctype { .. } => HtmlMutation::SetDoctype { doctype: base.doctype.clone() },
            HtmlMutation::InsertNode { parent, index, .. } => HtmlMutation::RemoveNode { parent: parent.clone(), index: *index },
            HtmlMutation::RemoveNode { parent, index } => match node_at(base, parent) {
                Ok(HtmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => HtmlMutation::InsertNode { parent: parent.clone(), index: *index, node: node.clone() },
                    None => HtmlMutation::NoMutation,
                },
                _ => HtmlMutation::NoMutation,
            },
            HtmlMutation::SetElementName { path, .. } => match node_at(base, path) {
                Ok(HtmlNode::Element { name, .. }) => HtmlMutation::SetElementName { path: path.clone(), name: name.clone() },
                _ => HtmlMutation::NoMutation,
            },
            HtmlMutation::SetAttribute { path, name, .. } => {
                let prior = node_at(base, path).ok().and_then(|node| element_attr(node, name)).cloned();
                HtmlMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior }
            }
            HtmlMutation::SetText { path, .. } => {
                let prior = match node_at(base, path) { Ok(HtmlNode::Text { text }) => text.clone(), _ => String::new() };
                HtmlMutation::SetText { path: path.clone(), text: prior }
            }
            HtmlMutation::SetComment { path, .. } => {
                let prior = match node_at(base, path) { Ok(HtmlNode::Comment { text }) => text.clone(), _ => String::new() };
                HtmlMutation::SetComment { path: path.clone(), text: prior }
            }
            HtmlMutation::SetRawText { path, .. } => {
                let prior = match node_at(base, path) { Ok(HtmlNode::RawText { text, .. }) => text.clone(), _ => String::new() };
                HtmlMutation::SetRawText { path: path.clone(), text: prior }
            }
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let base = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_html_mutation(&mut snapshot, &mutation);
        let bytes = write_html_document(&snapshot).into_bytes();
        let projection = project_html_5(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let base = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_html_mutation(&mut snapshot, &mutation);
        apply_html_mutation(&mut snapshot, &undo);
        let bytes = write_html_document(&snapshot).into_bytes();
        let projection = project_html_5(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `parse_html_document`/
    /// `write_html_document` are this subset's ONLY channel from input to output (HTML is text-native;
    /// there is no separate binary layer over the same model).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let snapshot = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let output = write_html_document(&snapshot).into_bytes();
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_html_5(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 10-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 10 kinds -- the scenario id only selects which fixture row's
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
