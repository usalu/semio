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
/// 📜️ The document every mutation row runs on: the real `word/document.xml` of the real committed
/// `📜️example-readme.docx`, extracted once (unzip, no other edit) — 92 873 bytes, 414 top-level
/// body blocks including a real 37-row/7-column table, and a real XML declaration.
const INPUT: &str = "shared://📰️ooxml-readme-document.xml";
/// 📄️ The minified 747-byte OOXML part this case used to rest on, kept for `identity-round-trip`
/// alone: it is the one committed document on which this repository's writer and `quick-xml` are
/// known to CONVERGE character for character, which is the finding the serialization-form probe was
/// written for and which nothing else here restates.
const MINIFIED_INPUT: &str = "shared://📰️ooxml-word-document.xml";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("readme-document.xml"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🧫️ The same, for the minified part the round-trip scenario additionally reads.
fn mutable_minified_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(MINIFIED_INPUT, Some("word-document.xml"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
///
/// 👁️ The OBSERVABILITY law is asserted here in role: a kind other than `no-mutation` whose
/// parameters leave the semantic projection exactly where it was has not been tested by this
/// scenario at all -- it proves only that the reference library declined to error. Every `Examples`
/// row is chosen against the real document's actual content for that reason, and this check is what
/// keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_xml_1_0(&bytes)?;
    if kind != "no-mutation" && projection_divergence(&projection, &project_xml_1_0(&input)?).is_none() {
        return Err(format!("{kind:?} left the semantic projection exactly as it found it -- a mutation whose parameters make it a no-op against the real document is not a test of that kind"));
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

/// ↩️ The inverse law, ASSERTED on the ORACLE side rather than deferred to the parity phase:
/// `quick-xml` applies the kind and then its own computed inverse, and the restored document's
/// independent projection must equal the REAL original's own. Without this the scenario would only
/// prove that the reference library did not error, which is not what `@mode-property` claims -- and
/// the subject handler asserts the same law on its own side, so neither side can be vacuous.
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

/// 🧪️ The same XML document, rendered differently: one insignificant space inserted before the `>`
/// (or `/>`) that terminates every start tag. XML 1.0 §3.1 admits it exactly there — `STag ::= '<'
/// Name (S Attribute)* S? '>'` and `EmptyElemTag ::= '<' Name (S Attribute)* S? '/>'` — so the
/// perturbed bytes denote the SAME document while being a document no writer would emit. Comments,
/// CDATA sections, processing instructions, the DOCTYPE and the interiors of attribute values are
/// stepped over untouched, because `>` inside any of them is ordinary content.
fn loosen_start_tags(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() + input.len() / 16);
    let mut at = 0usize;
    let skip_past = |out: &mut Vec<u8>, at: &mut usize, terminator: &[u8]| {
        let end = input[*at..].windows(terminator.len()).position(|window| window == terminator).map(|offset| *at + offset + terminator.len()).unwrap_or(input.len());
        out.extend_from_slice(&input[*at..end]);
        *at = end;
    };
    while at < input.len() {
        if input[at] != b'<' {
            out.push(input[at]);
            at += 1;
            continue;
        }
        let rest = &input[at..];
        if rest.starts_with(b"<!--") {
            skip_past(&mut out, &mut at, b"-->");
        } else if rest.starts_with(b"<![CDATA[") {
            skip_past(&mut out, &mut at, b"]]>");
        } else if rest.starts_with(b"<?") || rest.starts_with(b"<!") || rest.starts_with(b"</") {
            skip_past(&mut out, &mut at, b">");
        } else {
            let mut quote: Option<u8> = None;
            let mut cursor = at;
            while cursor < input.len() {
                let byte = input[cursor];
                match quote {
                    Some(open) if byte == open => quote = None,
                    Some(_) => {}
                    None if byte == b'"' || byte == b'\'' => quote = Some(byte),
                    None if byte == b'>' => break,
                    None => {}
                }
                cursor += 1;
            }
            let close = if input[..cursor].ends_with(b"/") { cursor - 1 } else { cursor };
            out.extend_from_slice(&input[at..close]);
            out.push(b' ');
            out.extend_from_slice(&input[close..(cursor + 1).min(input.len())]);
            at = cursor + 1;
        }
    }
    out
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated — and asserted
/// by a probe that a byte copy cannot satisfy.
///
/// ⚠️ The naive form of this law ("the re-encoded bytes must differ from the input") is the wrong
/// assertion for THIS input and was replaced rather than tuned: `shared://📰️ooxml-word-document.xml`
/// is a canonically minified OOXML part with no XML declaration and no inter-element whitespace, and
/// `quick-xml`'s canonical serialization agrees with it character for character. `output == input`
/// here is two minifying writers CONVERGING, which the byte-difference check cannot tell apart from
/// a `read`/`write` shortcut that never parsed anything — it fails on a correct implementation and
/// would pass on an incorrect one the moment a fixture with a declaration were swapped in. It is not
/// evidence either way.
///
/// What IS evidence, and is what this handler now requires: serialization-form invariance. The input
/// is re-rendered by [`loosen_start_tags`] into byte-different markup denoting the same document,
/// and both renderings must re-encode to the SAME bytes. A shortcut that hands its input back
/// returns the two different byte strings unchanged and fails immediately; only an implementation
/// that actually parsed both into one tree and re-derived the output from it can pass. The probe is
/// additionally required to be non-vacuous (the perturbation must really have moved the bytes), and
/// the round trip must still preserve the semantic projection.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let minified = round_trip_oracle_once(&mutable_minified_input(ctx)?, "the minified OOXML part")?;
    let readme = round_trip_oracle_once(&mutable_input(ctx)?, "the README document part")?;
    Ok(Outcome::with_raw(readme.0, Json::Object(vec![("minified".to_string(), minified.1), ("readme".to_string(), readme.1)])))
}

/// 🔁️ The probe itself, over one document.
fn round_trip_oracle_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
    let input = input.to_vec();
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let loosened = loosen_start_tags(&input);
    if loosened == input {
        return Err(format!("the serialization-form probe is vacuous on {what}: perturbing the start tags did not change a single byte, so it cannot distinguish parsing from copying"));
    }
    let from_loosened = oracle_apply_mutation(&loosened, &no_mutation)?;
    if from_loosened != bytes {
        return Err(format!(
            "byte pass-through on {what}: two byte-different renderings of the SAME document re-encoded differently ({} vs {} bytes), so the output is not being re-derived from a parsed tree",
            from_loosened.len(),
            bytes.len()
        ));
    }
    let projection = project_xml_1_0(&bytes)?;
    let original = project_xml_1_0(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated on {what}: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok((bytes, projection))
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
    /// 👁️ The forward mutation, with the same observability law the oracle side asserts: a kind
    /// other than `no-mutation` that left the projection exactly where it was addressed nothing in
    /// the real document.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = XmlSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base.clone();
        apply_xml_mutation(&mut snapshot, &mutation);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_xml_1_0(&bytes)?;
        if kind != "no-mutation" && super::projection_divergence(&projection, &project_xml_1_0(&base.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?)?).is_none() {
            return Err(format!("{kind:?} left the semantic projection exactly as it found it -- the parameters address nothing in the real document"));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The inverse law, asserted on the SUBJECT side too rather than deferred to the parity
    /// phase: apply-then-undo must restore this side's OWN reading of the original document's
    /// projection. Mirrors `super::inverse_oracle` through the same independent `project_xml_1_0`.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = XmlSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = inverse_of(&mutation, &base);
        let original = project_xml_1_0(&base.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?)?;
        let mut snapshot = base;
        let forward = apply_xml_mutation(&mut snapshot, &mutation);
        let backward = apply_xml_mutation(&mut snapshot, &undo);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_xml_1_0(&bytes)?;
        if let Some(divergence) = super::projection_divergence(&projection, &original) {
            return Err(format!(
                "inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}; forward outcome messages {:?}, undo outcome messages {:?}",
                spec.str("kind"),
                forward.messages(),
                backward.messages()
            ));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule, asserted by the SAME serialization-form probe the oracle
    /// side uses and for the same reason: this fixture is canonically minified, so a correct writer
    /// converging on the input byte for byte is the expected outcome and "the bytes must differ"
    /// cannot tell that apart from a `read`/`write` shortcut. Instead the input is re-rendered by
    /// `super::loosen_start_tags` into byte-different markup denoting the same document, and both
    /// renderings must re-encode identically -- which only an implementation that really parsed
    /// them into one snapshot can do. `XmlSnapshot::import_utf8`/`export_utf8` are this subset's
    /// ONLY channel from input to output (XML is text-native; there is no separate binary layer
    /// over the same model).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let minified = round_trip_once(&super::mutable_minified_input(ctx)?, "the minified OOXML part")?;
        let readme = round_trip_once(&mutable_input(ctx)?, "the README document part")?;
        Ok(Outcome::with_raw(readme.0, Json::Object(vec![("minified".to_string(), minified.1), ("readme".to_string(), readme.1)])))
    }

    /// 🔁️ The probe itself, over one document.
    fn round_trip_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
        let input = input.to_vec();
        let output = XmlSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let loosened = super::loosen_start_tags(&input);
        if loosened == input {
            return Err(format!("the serialization-form probe is vacuous on {what}: perturbing the start tags did not change a single byte"));
        }
        let from_loosened = XmlSnapshot::import_utf8(&loosened).map_err(|error| format!("import_utf8 of the perturbed rendering failed: {error}"))?.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        if from_loosened != output {
            return Err(format!("byte pass-through on {what}: two byte-different renderings of the SAME document re-encoded differently ({} vs {} bytes)", from_loosened.len(), output.len()));
        }
        let projection = project_xml_1_0(&output)?;
        if let Some(divergence) = super::projection_divergence(&projection, &project_xml_1_0(&input)?) {
            return Err(format!("round-trip law violated on {what}: decode then re-encode did not preserve the semantic projection -- {divergence}"));
        }
        Ok((output, projection))
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
