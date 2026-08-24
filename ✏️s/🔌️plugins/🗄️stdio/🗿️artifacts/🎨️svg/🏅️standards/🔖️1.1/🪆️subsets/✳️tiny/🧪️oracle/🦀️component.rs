//! 🔮️ Mutation oracle for `stdio.svg` 1.1/✳️tiny — every mutation kind THIS subset declares,
//! performed by the registered `quick-xml` reference implementation so the subject's own mutation
//! has an independent result to be compared against instead of being checked against its own
//! reading.
//!
//! The vocabulary is per SUBSET. `✳️tiny` is not a version of `✳️any`'s vocabulary: SVG Tiny 1.1
//! (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Tiny 1.1) is a RESTRICTION of Full 1.1,
//! so the operations that make sense inside it are the profile-closed ones — an insert that cannot
//! introduce an element the profile excludes, an attribute set that cannot introduce a forbidden
//! presentation attribute, the profile stamp itself, and the Full→Tiny down-conversion. `✳️any`'s
//! ungated `insert-element`/`set-attribute`/`set-element-name` have no counterpart here, because a
//! document that admits them is no longer a Tiny document.
//!
//! The reference machinery (the quick-xml tree, the `viewBox`/`transform` grammars, the projection)
//! is shared with the sibling `✳️basic` oracle through the `📰markup` family module rather than
//! copied into both — the two profiles restrict ONE schema and genuinely share every parse, write,
//! address and projection step.
//!
//! The blocklists below are transcribed from REC-SVGMobile-20030114 independently of this
//! repository's own `check_svg_tiny_conformance`. Both encode the same normative list; neither is
//! derived from the other.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the vocabulary itself (`SvgTinyMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Live
#[cfg(feature = "oracles")]
mod live {
    use crate::markup::live::*;
    use semio_repo_test_host::Json;

    //#region 🔖️Profile
    /// 🚫 Elements SVG Tiny 1.1 excludes outright. `fe*` filter primitives are matched by prefix:
    /// Tiny forbids the whole filter mechanism, primitives included.
    const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

    /// 🚫 Presentation attributes SVG Tiny 1.1 forbids on ANY element.
    const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

    pub fn is_blocked_element(name: &str) -> bool {
        let ln = local_name(name);
        BLOCKED_ELEMENTS.contains(&ln) || ln.starts_with("fe")
    }

    pub fn is_blocked_attr(name: &str) -> bool {
        BLOCKED_ATTRS.contains(&local_name(name))
    }

    /// 🛡️ The profile gate every authoring mutation passes through: a subtree entering a Tiny
    /// document may not carry an excluded element or an excluded presentation attribute anywhere.
    /// A violation is an error, never a silent acceptance.
    fn gate_subtree(node: &MarkupNode) -> Result<(), String> {
        let mut elements = Vec::new();
        elements_of(node, &mut elements);
        for (name, attrs) in &elements {
            if is_blocked_element(name) {
                return Err(format!("SVG Tiny 1.1 excludes element <{name}> — REC-SVGMobile-20030114 does not retain it"));
            }
            if let Some((attr, _)) = attrs.iter().find(|(key, _)| is_blocked_attr(key)) {
                return Err(format!("SVG Tiny 1.1 forbids attribute '{attr}' on <{name}> anywhere in the document"));
            }
        }
        Ok(())
    }
    //#endregion 🔖️Profile

    //#region 🔖️Forward
    /// 🦠️ Applies one declared kind to `doc` in place. An unrecognised kind is an error, never a
    /// silent no-op: a quietly skipped mutation reports as a passing test.
    pub fn apply(doc: &mut MarkupDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),
            "set-snapshot" => {
                let root = doc.root.as_mut().ok_or("set-snapshot: document has no root element")?;
                if let Some(id) = non_empty_str(params, "rootId") {
                    set_attr(root, "id", Some(id));
                }
                if let Some(width) = params.get("viewBoxWidth").and_then(json_number) {
                    let mut view_box = element_attr(root, "viewBox").map(parse_view_box).transpose()?.unwrap_or([0.0, 0.0, 0.0, 0.0]);
                    view_box[2] = width;
                    set_attr(root, "viewBox", Some(format_view_box(&view_box)));
                }
                Ok(())
            }
            "stamp-base-profile" => {
                let root = doc.root.as_mut().ok_or("stamp-base-profile: document has no root element")?;
                set_attr(root, "baseProfile", non_empty_str(params, "baseProfile"));
                set_attr(root, "version", non_empty_str(params, "version"));
                Ok(())
            }
            "insert-tiny-element" => {
                let node = json_to_node(&member(params, "node"));
                gate_subtree(&node)?;
                insert_child(doc, &json_to_path(&member(params, "parent")), usize_member(params, "index"), node)
            }
            "remove-element" => remove_child(doc, &json_to_path(&member(params, "parent")), usize_member(params, "index")).map(|_| ()),
            "set-tiny-attribute" => {
                let name = params.str("name");
                if is_blocked_attr(&name) {
                    return Err(format!("SVG Tiny 1.1 forbids attribute '{name}' anywhere in the document"));
                }
                let value = match params.get("value") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                };
                set_attr(node_at_mut(doc, &json_to_path(&member(params, "path")))?, &name, value);
                Ok(())
            }
            "set-text" => match node_at_mut(doc, &json_to_path(&member(params, "path")))? {
                MarkupNode::Text(text) => {
                    *text = params.str("text");
                    Ok(())
                }
                _ => Err("set-text: target is not a text node".into()),
            },
            "set-view-box" => {
                let node = node_at_mut(doc, &json_to_path(&member(params, "path")))?;
                match params.get("viewBox") {
                    Some(Json::Array(items)) if items.len() == 4 => {
                        let n: Vec<f64> = items.iter().map(|item| json_number(item).unwrap_or(0.0)).collect();
                        set_attr(node, "viewBox", Some(format_view_box(&[n[0], n[1], n[2], n[3]])));
                    }
                    _ => set_attr(node, "viewBox", None),
                }
                Ok(())
            }
            "set-transform" => {
                let node = node_at_mut(doc, &json_to_path(&member(params, "path")))?;
                match params.get("transform") {
                    Some(Json::Array(items)) => {
                        let ops: Vec<MarkupTransformOp> = items.iter().map(json_to_transform_op).collect::<Result<_, String>>()?;
                        set_attr(node, "transform", Some(format_transform_list(&ops)));
                    }
                    _ => set_attr(node, "transform", None),
                }
                Ok(())
            }
            "strip-non-tiny" => {
                let root = doc.root.as_mut().ok_or("strip-non-tiny: document has no root element")?;
                if is_blocked_element(match root {
                    MarkupNode::Element { name, .. } => name,
                    _ => return Err("strip-non-tiny: root is not an element".into()),
                }) {
                    return Err("strip-non-tiny: the root element itself is outside the profile".into());
                }
                retain_elements(root, &mut |child| !matches!(child, MarkupNode::Element { name, .. } if is_blocked_element(name)));
                rewrite_elements(root, &mut |_, attrs| attrs.retain(|(key, _)| !is_blocked_attr(key)));
                Ok(())
            }
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Computes and applies this kind's OWN inverse against the already-mutated document,
    /// sourcing whatever the forward mutation discarded from `base` — the document as it stood
    /// before the forward mutation ran. Mirrors the algebra `../🧬️schema/🧬️mutations/🦀️component.rs`
    /// defines for the subject, computed independently here so the property has two producers to
    /// disagree.
    ///
    /// `set-snapshot` and `strip-non-tiny` both restore the whole prior document: a strip that
    /// removed 335 excluded attributes has no smaller undo, and the subject's own inverse is the
    /// same `SetSnapshot{base}`. Stated rather than hidden.
    pub fn invert(base: &MarkupDoc, mut mutated: MarkupDoc, kind: &str, params: &Json) -> Result<MarkupDoc, String> {
        match kind {
            "no-mutation" => Ok(mutated),
            "set-snapshot" | "strip-non-tiny" => Ok(base.clone()),
            "stamp-base-profile" => {
                let prior_profile = node_at(base, &[]).ok().and_then(|node| element_attr(node, "baseProfile")).map(|s| s.to_string());
                let prior_version = node_at(base, &[]).ok().and_then(|node| element_attr(node, "version")).map(|s| s.to_string());
                let root = mutated.root.as_mut().ok_or("inverse stamp-base-profile: document has no root element")?;
                set_attr(root, "baseProfile", prior_profile);
                set_attr(root, "version", prior_version);
                Ok(mutated)
            }
            "insert-tiny-element" => {
                remove_child(&mut mutated, &json_to_path(&member(params, "parent")), usize_member(params, "index"))?;
                Ok(mutated)
            }
            "remove-element" => {
                let parent = json_to_path(&member(params, "parent"));
                let index = usize_member(params, "index");
                let captured = child_at(base, &parent, index).cloned().ok_or_else(|| format!("inverse remove-element: the original document has no child {index} under {parent:?}"))?;
                insert_child(&mut mutated, &parent, index, captured)?;
                Ok(mutated)
            }
            "set-tiny-attribute" => {
                let path = json_to_path(&member(params, "path"));
                let name = params.str("name");
                let prior = node_at(base, &path).ok().and_then(|node| element_attr(node, &name)).map(|s| s.to_string());
                set_attr(node_at_mut(&mut mutated, &path)?, &name, prior);
                Ok(mutated)
            }
            "set-text" => {
                let path = json_to_path(&member(params, "path"));
                let prior = match node_at(base, &path)? {
                    MarkupNode::Text(text) => text.clone(),
                    _ => return Err("inverse set-text: the original target is not a text node".into()),
                };
                match node_at_mut(&mut mutated, &path)? {
                    MarkupNode::Text(text) => *text = prior,
                    _ => return Err("inverse set-text: target is not a text node".into()),
                }
                Ok(mutated)
            }
            "set-view-box" => {
                let path = json_to_path(&member(params, "path"));
                let prior = node_at(base, &path).ok().and_then(|node| element_attr(node, "viewBox")).map(|s| s.to_string());
                set_attr(node_at_mut(&mut mutated, &path)?, "viewBox", prior);
                Ok(mutated)
            }
            "set-transform" => {
                let path = json_to_path(&member(params, "path"));
                let prior = node_at(base, &path).ok().and_then(|node| element_attr(node, "transform")).map(|s| s.to_string());
                set_attr(node_at_mut(&mut mutated, &path)?, "transform", prior);
                Ok(mutated)
            }
            other => Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let mut doc = parse_markup(input)?;
        apply(&mut doc, kind, params)?;
        write_markup(&doc)
    }

    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let base = parse_markup(input)?;
        let mut mutated = base.clone();
        apply(&mut mutated, kind, params)?;
        write_markup(&invert(&base, mutated, kind, params)?)
    }

    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        write_markup(&parse_markup(input)?)
    }

    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        Ok(project_markup(&parse_markup(bytes)?))
    }
    //#endregion 🔖️Routing

    #[cfg(test)]
    mod tests {
        use super::*;

        fn tiny_doc() -> MarkupDoc {
            parse_markup(br#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1"><g style="fill:#000"><rect x="0" y="0" width="4" height="4"/></g><filter id="f1"><feTurbulence/></filter></svg>"#).expect("parses")
        }

        #[test]
        fn strip_non_tiny_removes_excluded_elements_and_attributes() {
            let mut doc = tiny_doc();
            apply(&mut doc, "strip-non-tiny", &Json::Object(Vec::new())).expect("strip applies");
            let rendered = String::from_utf8(write_markup(&doc).expect("writes")).expect("utf8");
            assert!(!rendered.contains("filter"), "the excluded <filter> subtree must be gone: {rendered}");
            assert!(!rendered.contains("style="), "the excluded style attribute must be gone: {rendered}");
            assert!(rendered.contains("<rect"), "retained geometry must survive: {rendered}");
        }

        #[test]
        fn insert_tiny_element_rejects_an_excluded_subtree() {
            let mut doc = tiny_doc();
            let node = Json::Object(vec![("kind".into(), Json::String("element".into())), ("name".into(), Json::String("linearGradient".into()))]);
            let params = Json::Object(vec![("parent".into(), Json::Array(Vec::new())), ("index".into(), Json::Number(0.0)), ("node".into(), node)]);
            assert!(apply(&mut doc, "insert-tiny-element", &params).is_err(), "a linearGradient is outside SVG Tiny 1.1");
        }

        #[test]
        fn set_tiny_attribute_rejects_a_forbidden_presentation_attribute() {
            let mut doc = tiny_doc();
            let params = Json::Object(vec![("path".into(), Json::Array(Vec::new())), ("name".into(), Json::String("opacity".into())), ("value".into(), Json::String("0.5".into()))]);
            assert!(apply(&mut doc, "set-tiny-attribute", &params).is_err(), "opacity is forbidden anywhere in SVG Tiny 1.1");
        }

        #[test]
        fn unknown_kind_is_an_error_not_a_no_op() {
            let mut doc = tiny_doc();
            assert!(apply(&mut doc, "set-element-name", &Json::Object(Vec::new())).is_err(), "a kind this subset does not declare must never be silently skipped");
        }
    }
}
//#endregion 🔖️Live

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::apply_mutation(input, &spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::apply_mutation_inverse(input, &spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// 🔁️ Decodes with the independent reader and re-encodes from its own tree alone, no mutation
/// applied — the identity round trip.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    live::round_trip(input)
}

/// 👁️ Projects SVG bytes with the INDEPENDENT reader onto the `semantic-svg-tiny-1-1-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_svg_tiny(bytes: &[u8]) -> Result<Json, String> {
    live::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_svg_tiny(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
