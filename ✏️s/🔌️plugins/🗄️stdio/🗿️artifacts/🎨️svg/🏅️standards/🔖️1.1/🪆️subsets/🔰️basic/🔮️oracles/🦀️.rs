//! 🔮️ Mutation oracle for `stdio.svg` 1.1/🔰️basic — every mutation kind THIS subset declares,
//! performed by the registered `quick-xml` reference implementation so the subject's own mutation
//! has an independent result to be compared against instead of being checked against its own
//! reading.
//!
//! The vocabulary is per SUBSET, and `🔰️basic` is not `🔬️tiny` with a longer allow-list. SVG Basic
//! 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Basic 1.1) KEEPS gradients, patterns,
//! masks, opacity and the filter mechanism; what it drops is the set of expensive raster filter
//! primitives and the ability to clip to text. Its two profile-defining operations are therefore
//! about filters and clip paths — `insert-basic-element`, which refuses a subtree carrying an
//! excluded primitive, and the clip-path pair `set-clip-path-reference`/`insert-clip-path-shape`,
//! which refuse a clip path that would clip to text. Neither exists in `🔬️tiny`'s vocabulary, whose
//! profile has no filters and no `clipPath` element at all.
//!
//! The reference machinery is shared with the sibling `🔬️tiny` oracle through the `📰markup` family
//! module rather than copied into both. The blocklists are transcribed from REC-SVGMobile-20030114
//! independently of this repository's own `check_svg_basic_conformance`; both encode the same
//! normative list, neither is derived from the other.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary itself (`SvgBasicMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Live
#[cfg(feature = "oracles")]
mod live {
    use semio_s_plugin_stdio_markup_test_oracle::live::*;
    use semio_repo_test_host::Json;

    //#region 🔖️Profile
    /// 🚫 The expensive raster filter primitives SVG Basic 1.1 excludes. Full 1.1 has them; Basic's
    /// constrained-device target does not.
    const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

    /// ✍️ The SVG text element kinds — a clip path containing one of these clips to text, which
    /// SVG Basic 1.1 does not support.
    const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

    pub fn is_blocked_primitive(name: &str) -> bool {
        BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name))
    }

    fn clips_to_text(node: &MarkupNode) -> bool {
        let mut elements = Vec::new();
        elements_of(node, &mut elements);
        elements.iter().any(|(name, _)| TEXT_ELEMENTS.contains(&local_name(name)))
    }

    /// 🔗 The fragment id of a `clip-path="url(#id)"`-shaped value, bare or quoted.
    fn clip_path_ref_id(value: &str) -> Option<&str> {
        let inner = value.trim().strip_prefix("url(")?.strip_suffix(')')?;
        inner.trim().trim_matches(|c| c == '\'' || c == '"').strip_prefix('#')
    }

    /// 🛡️ The profile gate every authoring mutation passes through: a subtree entering a Basic
    /// document may not carry an excluded filter primitive, and may not itself be a clip path that
    /// clips to text.
    fn gate_subtree(node: &MarkupNode) -> Result<(), String> {
        let mut elements = Vec::new();
        elements_of(node, &mut elements);
        for (name, _) in &elements {
            if is_blocked_primitive(name) {
                return Err(format!("SVG Basic 1.1 excludes the raster filter primitive <{name}> — REC-SVGMobile-20030114 does not retain it"));
            }
        }
        if matches!(node, MarkupNode::Element { name, .. } if local_name(name) == "clipPath") && clips_to_text(node) {
            return Err("SVG Basic 1.1 does not support clipping to text — this clipPath carries a text descendant".to_string());
        }
        Ok(())
    }

    /// 🛡️ The clip-path reference gate: a `clip-path="url(#id)"` may only resolve to a clip path
    /// that contains no text descendant.
    fn gate_clip_reference(doc: &MarkupDoc, id: &str) -> Result<(), String> {
        let path = path_of_id(doc, id).ok_or_else(|| format!("clip-path reference: this document declares no element with id {id:?}"))?;
        let node = node_at(doc, &path)?;
        match node {
            MarkupNode::Element { name, .. } if local_name(name) == "clipPath" => {
                if clips_to_text(node) {
                    return Err(format!("SVG Basic 1.1 does not support clipping to text — clipPath #{id} carries a text descendant"));
                }
                Ok(())
            }
            _ => Err(format!("clip-path reference: #{id} is not a clipPath element")),
        }
    }

    /// 🗺️ The path of the `clipPath` named by `clipPathId`, gated on the profile's text rule.
    fn clip_path_target(doc: &MarkupDoc, id: &str) -> Result<Vec<usize>, String> {
        gate_clip_reference(doc, id)?;
        path_of_id(doc, id).ok_or_else(|| format!("clip-path reference: this document declares no element with id {id:?}"))
    }
    //#endregion 🔖️Profile

    //#region 🔖️Forward
    /// 🦠️ Applies one declared kind to `doc` in place. An unrecognised kind is an error, never a
    /// silent no-op: a quietly skipped mutation reports as a passing test.
    pub fn apply(doc: &mut MarkupDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "set-snapshot" => {
                *doc = doc_from_wire(&member(&member(params, "snapshot"), "doc"))?;
                Ok(())
            }
            "patch-snapshot" => {
                *doc = patched_markup(doc, "stdio.svg", &member(params, "patch"))?;
                Ok(())
            }
            "stamp-base-profile" => {
                let root = doc.root.as_mut().ok_or("stamp-base-profile: document has no root element")?;
                set_attr(root, "baseProfile", non_empty_str(params, "baseProfile"));
                set_attr(root, "version", non_empty_str(params, "version"));
                Ok(())
            }
            "insert-basic-element" => {
                let node = node_from_wire(&member(params, "node"))?;
                gate_subtree(&node)?;
                insert_child(doc, &json_to_path(&member(params, "parent")), usize_member(params, "index"), node)
            }
            "remove-element" => remove_child(doc, &json_to_path(&member(params, "parent")), usize_member(params, "index")).map(|_| ()),
            "set-basic-attribute" => {
                let name = params.str("name");
                let value = match params.get("value") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                };
                if local_name(&name) == "clip-path" {
                    if let Some(id) = value.as_deref().and_then(clip_path_ref_id) {
                        gate_clip_reference(doc, id)?;
                    }
                }
                set_attr(node_at_mut(doc, &json_to_path(&member(params, "path")))?, &name, value);
                Ok(())
            }
            "set-clip-path-reference" => {
                let path = json_to_path(&member(params, "path"));
                let value = match non_empty_str(params, "clipPathId") {
                    Some(id) => {
                        gate_clip_reference(doc, &id)?;
                        Some(format!("url(#{id})"))
                    }
                    None => None,
                };
                set_attr(node_at_mut(doc, &path)?, "clip-path", value);
                Ok(())
            }
            "insert-clip-path-shape" => {
                let id = params.str("clipPathId");
                let target = clip_path_target(doc, &id)?;
                let node = node_from_wire(&member(params, "node"))?;
                if clips_to_text(&node) {
                    return Err("SVG Basic 1.1 does not support clipping to text — the inserted shape carries a text element".to_string());
                }
                gate_subtree(&node)?;
                insert_child(doc, &target, usize_member(params, "index"), node)
            }
            "set-text" => match node_at_mut(doc, &json_to_path(&member(params, "path")))? {
                MarkupNode::Text(text) => {
                    *text = params.str("text");
                    Ok(())
                }
                _ => Err("set-text: target is not a text node".into()),
            },
            "set-view-box" => {
                let view_box = view_box_from_wire(&member(params, "viewBox")).map(|value| format_view_box(&value));
                set_attr(node_at_mut(doc, &json_to_path(&member(params, "path")))?, "viewBox", view_box);
                Ok(())
            }
            "set-transform" => {
                let transform = transform_from_wire(&member(params, "transform"))?;
                set_attr(node_at_mut(doc, &json_to_path(&member(params, "path")))?, "transform", transform);
                Ok(())
            }
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Computes and applies this kind's OWN inverse against the already-mutated document,
    /// sourcing whatever the forward mutation discarded from `base` — the document as it stood
    /// before the forward mutation ran. Computed independently of the subject's own algebra so the
    /// property has two producers to disagree.
    pub fn invert(base: &MarkupDoc, mut mutated: MarkupDoc, kind: &str, params: &Json) -> Result<MarkupDoc, String> {
        match kind {
            "set-snapshot" | "patch-snapshot" => Ok(base.clone()),
            "stamp-base-profile" => {
                let prior_profile = node_at(base, &[]).ok().and_then(|node| element_attr(node, "baseProfile")).map(|s| s.to_string());
                let prior_version = node_at(base, &[]).ok().and_then(|node| element_attr(node, "version")).map(|s| s.to_string());
                let root = mutated.root.as_mut().ok_or("inverse stamp-base-profile: document has no root element")?;
                set_attr(root, "baseProfile", prior_profile);
                set_attr(root, "version", prior_version);
                Ok(mutated)
            }
            "insert-basic-element" => {
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
            "set-basic-attribute" => {
                let path = json_to_path(&member(params, "path"));
                let name = params.str("name");
                let prior = node_at(base, &path).ok().and_then(|node| element_attr(node, &name)).map(|s| s.to_string());
                set_attr(node_at_mut(&mut mutated, &path)?, &name, prior);
                Ok(mutated)
            }
            "set-clip-path-reference" => {
                let path = json_to_path(&member(params, "path"));
                let prior = node_at(base, &path).ok().and_then(|node| element_attr(node, "clip-path")).map(|s| s.to_string());
                set_attr(node_at_mut(&mut mutated, &path)?, "clip-path", prior);
                Ok(mutated)
            }
            "insert-clip-path-shape" => {
                let id = params.str("clipPathId");
                let target = path_of_id(&mutated, &id).ok_or_else(|| format!("inverse insert-clip-path-shape: the mutated document has no element with id {id:?}"))?;
                remove_child(&mut mutated, &target, usize_member(params, "index"))?;
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
    include!("🧪️tests/🔬️live-unit/🦀️.rs");
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

/// 👁️ Projects SVG bytes with the INDEPENDENT reader onto the `semantic-svg-basic-1-1-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_svg_basic(bytes: &[u8]) -> Result<Json, String> {
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
pub fn project_svg_basic(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
