//! 🔮️ Mutation oracle for this subset — every `MdMutation` kind performed independently by the
//! registered `comrak` reference implementation (a full CommonMark parser AND renderer, so it can
//! be both the independent AST every mutation edits and the independent reader every projection
//! goes through) so the subject's own mutation has an independent result to be compared against
//! instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. No other subset shares CommonMark's
//! block/inline AST shape, so everything below lives here rather than in a shared `document` family
//! module.
//!
//! ## Writer freedom vs. real information loss
//! `project_md` compares the parsed BLOCK STRUCTURE, never rendered text, and it only ever emits
//! the fields `MdBlock`/`MdInline` themselves carry — every other CommonMark writer choice is
//! dropped by construction, not by an explicit normalization pass: bullet marker character (`-`
//! vs `*`) and ordered-list delimiter (`.` vs `)`), emphasis delimiter (`*em*` vs `_em_`), code
//! fence character/length (`` ``` `` vs `~~~~`), indented-vs-fenced code block source form
//! (`MdBlock::CodeBlock` unifies both, `info` is `None` for what was indented), and hard-break
//! encoding (trailing backslash vs two trailing spaces) are all real CommonMark writer freedom the
//! spec itself leaves unconstrained — see the RFC1951 precedent this repository's own
//! `deflate`/`zlib` oracle module documents for the same principle applied to a binary format.
//!
//! One genuine SCOPE gap, not writer freedom, is worth recording rather than hiding: this
//! subset's own parser (`⚙️engine::parse_markdown_blocks`) explicitly does not resolve
//! reference-style links/images or setext headings (documented on `MdSnapshot`'s own schema file
//! as an honest scope cut, degrading to plain text/paragraph instead), while `comrak` with
//! `Options::default()` resolves both as core CommonMark. Both are absent from the real README
//! fixture this case reads (verified: no `^\[.+\]:` reference definitions, no `^=+$`/`^-+$` setext
//! underlines), so it does not affect this wave's scenarios, but a future fixture that used either
//! construct would show a genuine subject/oracle disagreement here, not a false one.
//!
//! GFM extensions (tables, strikethrough, task lists, footnotes, autolinks, ...) are likewise
//! never enabled (`Options::default()`): this subset's own scope explicitly excludes GFM, and an
//! unenabled comrak parses the same pipe-table/`~~strike~~` syntax as plain paragraph text this
//! subset's own parser does, so the two stay aligned rather than one silently gaining GFM support
//! the other lacks.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`MdMutation`).
//! @see ../🧬️schema/📸️snapshot/🦀️component.rs — the `MdBlock`/`MdInline` shape this module's JSON
//! mirrors field-for-field (same `kind`-tagged, camelCase shape `serde` derives for both enums).

#[cfg(feature = "oracles")]
mod live {
    use comrak::nodes::{AstNode, ListDelimType, ListType, NodeCode, NodeCodeBlock, NodeHeading, NodeHtmlBlock, NodeLink, NodeList, NodeValue};
    use comrak::{format_commonmark, parse_document, Arena, Options};
    use semio_repo_test_host::Json;

    //#region 🔖️Json
    /// 🔎️ `value` read as an array, or empty for anything else — mirrors `Json::array(key)`'s own
    /// "absent means empty" contract but for a value that is not itself an object field (a `path`
    /// or `inlines` array read straight off a mutation's `params`).
    fn as_array(value: Option<&Json>) -> Vec<Json> {
        match value {
            Some(Json::Array(items)) => items.clone(),
            _ => Vec::new(),
        }
    }

    /// 🔎️ `key` read as a `usize`, erroring loudly rather than silently defaulting — an
    /// out-of-range or missing index must fail the mutation, never quietly no-op it.
    fn json_usize(json: &Json, key: &str) -> Result<usize, String> {
        match json.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
            _ => Err(format!("mutation params carry no numeric '{key}'")),
        }
    }

    /// 🔎️ `key` read as an optional string: absent key or explicit `null` both mean `None`,
    /// mirroring `Option<String>`'s `skip_serializing_if = "Option::is_none"` on the schema side.
    fn optional_string(json: &Json, key: &str) -> Option<String> {
        match json.get(key) {
            Some(Json::String(value)) => Some(value.clone()),
            _ => None,
        }
    }
    //#endregion 🔖️Json

    //#region 🔖️Build
    /// 🏗️ Builds one `MdBlock`-shaped JSON object into a real comrak block node (leaf or
    /// container, recursively), independent of the subject's own `MdBlock` type — this module
    /// never imports the subject crate at all.
    fn build_block<'a>(arena: &'a Arena<'a>, json: &Json) -> Result<&'a AstNode<'a>, String> {
        match json.str("kind").as_str() {
            "heading" => {
                let level = match json.get("level") {
                    Some(Json::Number(value)) => *value as u8,
                    _ => return Err("heading block carries no numeric 'level'".to_string()),
                };
                let node = arena.alloc(NodeValue::Heading(NodeHeading { level, setext: false, closed: true }).into());
                append_inlines(arena, node, &json.array("inlines"))?;
                Ok(node)
            }
            "paragraph" => {
                let node = arena.alloc(NodeValue::Paragraph.into());
                append_inlines(arena, node, &json.array("inlines"))?;
                Ok(node)
            }
            "list" => {
                let ordered = matches!(json.get("ordered"), Some(Json::Bool(true)));
                let tight = matches!(json.get("tight"), Some(Json::Bool(true)));
                let start = match json.get("start") {
                    Some(Json::Number(value)) => *value as usize,
                    _ => 1,
                };
                let list_type = if ordered { ListType::Ordered } else { ListType::Bullet };
                let padding = if ordered { 3 } else { 2 };
                let meta = NodeList { list_type, marker_offset: 0, padding, start, delimiter: ListDelimType::Period, bullet_char: b'-', tight, is_task_list: false };
                let node = arena.alloc(NodeValue::List(meta).into());
                for item_entry in json.array("items") {
                    let item_node = arena.alloc(NodeValue::Item(meta).into());
                    for block_json in as_array(Some(&item_entry)) {
                        let child = build_block(arena, &block_json)?;
                        item_node.append(child);
                    }
                    node.append(item_node);
                }
                Ok(node)
            }
            "codeBlock" => {
                let info = optional_string(json, "info").unwrap_or_default();
                let mut literal = json.str("literal");
                if !literal.is_empty() && !literal.ends_with('\n') {
                    literal.push('\n');
                }
                Ok(arena.alloc(NodeValue::CodeBlock(Box::new(NodeCodeBlock { fenced: true, fence_char: b'`', fence_length: 3, fence_offset: 0, info, literal, closed: true })).into()))
            }
            "blockQuote" => {
                let node = arena.alloc(NodeValue::BlockQuote.into());
                for block_json in json.array("blocks") {
                    let child = build_block(arena, &block_json)?;
                    node.append(child);
                }
                Ok(node)
            }
            "thematicBreak" => Ok(arena.alloc(NodeValue::ThematicBreak.into())),
            "htmlBlock" => {
                let mut literal = json.str("raw");
                if !literal.ends_with('\n') {
                    literal.push('\n');
                }
                Ok(arena.alloc(NodeValue::HtmlBlock(NodeHtmlBlock { block_type: 6, literal }).into()))
            }
            other => Err(format!("mutation block carries unknown kind {other:?}")),
        }
    }

    /// 🏗️ Builds one `MdInline`-shaped JSON object into a real comrak inline node, recursively.
    fn build_inline<'a>(arena: &'a Arena<'a>, json: &Json) -> Result<&'a AstNode<'a>, String> {
        match json.str("kind").as_str() {
            "text" => Ok(arena.alloc(NodeValue::Text(json.str("text").into()).into())),
            "emphasis" => {
                let node = arena.alloc(NodeValue::Emph.into());
                append_inlines(arena, node, &json.array("inlines"))?;
                Ok(node)
            }
            "strong" => {
                let node = arena.alloc(NodeValue::Strong.into());
                append_inlines(arena, node, &json.array("inlines"))?;
                Ok(node)
            }
            "code" => Ok(arena.alloc(NodeValue::Code(NodeCode { num_backticks: 1, literal: json.str("literal") }).into())),
            "link" => {
                let node = arena.alloc(NodeValue::Link(Box::new(NodeLink { url: json.str("url"), title: optional_string(json, "title").unwrap_or_default() })).into());
                append_inlines(arena, node, &json.array("text"))?;
                Ok(node)
            }
            "image" => {
                let node = arena.alloc(NodeValue::Image(Box::new(NodeLink { url: json.str("url"), title: optional_string(json, "title").unwrap_or_default() })).into());
                let alt = arena.alloc(NodeValue::Text(json.str("alt").into()).into());
                node.append(alt);
                Ok(node)
            }
            "softBreak" => Ok(arena.alloc(NodeValue::SoftBreak.into())),
            "hardBreak" => Ok(arena.alloc(NodeValue::LineBreak.into())),
            "htmlInline" => Ok(arena.alloc(NodeValue::HtmlInline(json.str("raw")).into())),
            other => Err(format!("mutation inline carries unknown kind {other:?}")),
        }
    }

    fn append_inlines<'a>(arena: &'a Arena<'a>, parent: &'a AstNode<'a>, inlines_json: &[Json]) -> Result<(), String> {
        for inline_json in inlines_json {
            parent.append(build_inline(arena, inline_json)?);
        }
        Ok(())
    }
    //#endregion 🔖️Build

    //#region 🔖️Navigate
    /// 🧭️ Mirrors `MdPathStep`'s two variants (`../../🧬️schema/🔺️diff/🦀️component.rs`) without
    /// importing the subject crate: `BlockQuote{index}` steps into the block-quote block at
    /// `index`'s own children; `ListItem{index,item}` steps into the list block at `index`'s
    /// `item`-th `Item` node's children.
    enum PathStep {
        BlockQuote(usize),
        ListItem(usize, usize),
    }

    fn decode_path(params: &Json) -> Result<Vec<PathStep>, String> {
        as_array(params.get("path"))
            .iter()
            .map(|step| match step.str("step").as_str() {
                "blockQuote" => Ok(PathStep::BlockQuote(json_usize(step, "index")?)),
                "listItem" => Ok(PathStep::ListItem(json_usize(step, "index")?, json_usize(step, "item")?)),
                other => Err(format!("path step carries unknown 'step' {other:?}")),
            })
            .collect()
    }

    fn nth_child<'a>(node: &'a AstNode<'a>, index: usize) -> Result<&'a AstNode<'a>, String> {
        node.children().nth(index).ok_or_else(|| format!("index {index} is out of range for its container"))
    }

    /// 🧭️ Walks `path` from `root`, returning the CONTAINER node whose children are the addressed
    /// `Vec<MdBlock>` — `root` itself (whose children are the top-level blocks) when `path` is
    /// empty, exactly mirroring `navigate_container`'s own contract.
    fn navigate<'a>(root: &'a AstNode<'a>, path: &[PathStep]) -> Result<&'a AstNode<'a>, String> {
        let mut current = root;
        for step in path {
            current = match step {
                PathStep::BlockQuote(index) => {
                    let target = nth_child(current, *index)?;
                    if !matches!(target.data.borrow().value, NodeValue::BlockQuote) {
                        return Err(format!("path step blockQuote({index}) does not address a block quote"));
                    }
                    target
                }
                PathStep::ListItem(index, item) => {
                    let list_node = nth_child(current, *index)?;
                    if !matches!(list_node.data.borrow().value, NodeValue::List(_)) {
                        return Err(format!("path step listItem({index}, _) does not address a list"));
                    }
                    nth_child(list_node, *item)?
                }
            };
        }
        Ok(current)
    }

    fn insert_at<'a>(container: &'a AstNode<'a>, index: usize, new_node: &'a AstNode<'a>) {
        match container.children().nth(index) {
            Some(existing) => existing.insert_before(new_node),
            None => container.append(new_node),
        }
    }
    //#endregion 🔖️Navigate

    //#region 🔖️Dispatch
    /// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized
    /// bytes. An unrecognised kind is an error, never a silent no-op: a mutation that is quietly
    /// skipped reports as a passing test.
    pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let arena = Arena::new();
        let options = Options::default();
        let root = parse_document(&arena, text, &options);
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));

        match spec.str("kind").as_str() {
            "" => return Err("mutation spec carries no `kind`".to_string()),
            "no-mutation" => {}
            "set-snapshot" => {
                for child in root.children().collect::<Vec<_>>() {
                    child.detach();
                }
                let snapshot = params.get("snapshot").ok_or("set-snapshot: params carry no 'snapshot'")?;
                for block_json in snapshot.array("blocks") {
                    root.append(build_block(&arena, &block_json)?);
                }
            }
            "insert-block" => {
                let path = decode_path(&params)?;
                let index = json_usize(&params, "index")?;
                let block_json = params.get("block").ok_or("insert-block: params carry no 'block'")?;
                let container = navigate(root, &path)?;
                let new_node = build_block(&arena, block_json)?;
                insert_at(container, index, new_node);
            }
            "remove-block" => {
                let path = decode_path(&params)?;
                let index = json_usize(&params, "index")?;
                let container = navigate(root, &path)?;
                nth_child(container, index)?.detach();
            }
            "replace-block" => {
                let path = decode_path(&params)?;
                let index = json_usize(&params, "index")?;
                let block_json = params.get("block").ok_or("replace-block: params carry no 'block'")?;
                let container = navigate(root, &path)?;
                let target = nth_child(container, index)?;
                let new_node = build_block(&arena, block_json)?;
                target.insert_before(new_node);
                target.detach();
            }
            "set-inlines" => {
                let path = decode_path(&params)?;
                let index = json_usize(&params, "index")?;
                let inlines_json = params.get("inlines").ok_or("set-inlines: params carry no 'inlines'")?;
                let container = navigate(root, &path)?;
                let target = nth_child(container, index)?;
                let retargetable = matches!(target.data.borrow().value, NodeValue::Heading(_) | NodeValue::Paragraph);
                if retargetable {
                    for child in target.children().collect::<Vec<_>>() {
                        child.detach();
                    }
                    append_inlines(&arena, target, &as_array(Some(inlines_json)))?;
                }
                // 🍃 else: graceful no-op, mirroring `MdMutation::diff`'s own documented
                // degrade-gracefully behavior for a `SetInlines` addressed at a non-text block.
            }
            other => return Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", other, input.len())),
        }

        let mut out = String::new();
        format_commonmark(root, &options, &mut out).map_err(|error| format!("comrak could not format the document: {error}"))?;
        Ok(out.into_bytes())
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Projection
    /// 👁️ Projects markdown bytes with the INDEPENDENT `comrak` reader onto this subset's own
    /// `MdSnapshot`-shaped semantic tree: real CommonMark block/inline structure, never raw
    /// rendered text. Called on both the oracle's own re-serialized bytes and (from the case's own
    /// Rust adapter) the subject's re-parsed output, so neither producer is ever checked against
    /// its own reading.
    pub fn project_md(input: &[u8]) -> Result<Json, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let arena = Arena::new();
        let options = Options::default();
        let root = parse_document(&arena, text, &options);
        let blocks: Vec<Json> = root.children().filter_map(project_block).collect();
        Ok(Json::Object(vec![("schema".to_string(), Json::String("stdio.md".to_string())), ("blocks".to_string(), Json::Array(blocks))]))
    }

    fn project_block<'a>(node: &'a AstNode<'a>) -> Option<Json> {
        let value = node.data.borrow().value.clone();
        Some(match value {
            NodeValue::Heading(heading) => {
                let inlines: Vec<Json> = node.children().filter_map(project_inline).collect();
                Json::Object(vec![("kind".to_string(), Json::String("heading".to_string())), ("level".to_string(), Json::Number(heading.level as f64)), ("inlines".to_string(), Json::Array(inlines))])
            }
            NodeValue::Paragraph => {
                let inlines: Vec<Json> = node.children().filter_map(project_inline).collect();
                Json::Object(vec![("kind".to_string(), Json::String("paragraph".to_string())), ("inlines".to_string(), Json::Array(inlines))])
            }
            NodeValue::List(meta) => {
                let ordered = matches!(meta.list_type, ListType::Ordered);
                let items: Vec<Json> = node.children().map(|item| Json::Array(item.children().filter_map(project_block).collect())).collect();
                Json::Object(vec![
                    ("kind".to_string(), Json::String("list".to_string())),
                    ("ordered".to_string(), Json::Bool(ordered)),
                    ("start".to_string(), if ordered { Json::Number(meta.start as f64) } else { Json::Null }),
                    ("tight".to_string(), Json::Bool(meta.tight)),
                    ("items".to_string(), Json::Array(items)),
                ])
            }
            NodeValue::CodeBlock(code) => Json::Object(vec![("kind".to_string(), Json::String("codeBlock".to_string())), ("info".to_string(), if code.info.is_empty() { Json::Null } else { Json::String(code.info.clone()) }), ("literal".to_string(), Json::String(code.literal.clone()))]),
            NodeValue::BlockQuote => {
                let blocks: Vec<Json> = node.children().filter_map(project_block).collect();
                Json::Object(vec![("kind".to_string(), Json::String("blockQuote".to_string())), ("blocks".to_string(), Json::Array(blocks))])
            }
            NodeValue::ThematicBreak => Json::Object(vec![("kind".to_string(), Json::String("thematicBreak".to_string()))]),
            NodeValue::HtmlBlock(html) => Json::Object(vec![("kind".to_string(), Json::String("htmlBlock".to_string())), ("raw".to_string(), Json::String(html.literal.clone()))]),
            // 🍃 Out-of-subset block kinds (GFM tables/footnotes/alerts/...) never arise —
            // `Options::default()` never enables the extensions that produce them — but a
            // defensive omission keeps this projection total rather than panicking if one did.
            _ => return None,
        })
    }

    fn project_inline<'a>(node: &'a AstNode<'a>) -> Option<Json> {
        let value = node.data.borrow().value.clone();
        Some(match value {
            NodeValue::Text(text) => Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text.to_string()))]),
            NodeValue::Emph => Json::Object(vec![("kind".to_string(), Json::String("emphasis".to_string())), ("inlines".to_string(), Json::Array(node.children().filter_map(project_inline).collect()))]),
            NodeValue::Strong => Json::Object(vec![("kind".to_string(), Json::String("strong".to_string())), ("inlines".to_string(), Json::Array(node.children().filter_map(project_inline).collect()))]),
            NodeValue::Code(code) => Json::Object(vec![("kind".to_string(), Json::String("code".to_string())), ("literal".to_string(), Json::String(code.literal.clone()))]),
            NodeValue::Link(link) => Json::Object(vec![
                ("kind".to_string(), Json::String("link".to_string())),
                ("text".to_string(), Json::Array(node.children().filter_map(project_inline).collect())),
                ("url".to_string(), Json::String(link.url.clone())),
                ("title".to_string(), if link.title.is_empty() { Json::Null } else { Json::String(link.title.clone()) }),
            ]),
            NodeValue::Image(image) => Json::Object(vec![
                ("kind".to_string(), Json::String("image".to_string())),
                ("alt".to_string(), Json::String(node.collect_text())),
                ("url".to_string(), Json::String(image.url.clone())),
                ("title".to_string(), if image.title.is_empty() { Json::Null } else { Json::String(image.title.clone()) }),
            ]),
            NodeValue::SoftBreak => Json::Object(vec![("kind".to_string(), Json::String("softBreak".to_string()))]),
            NodeValue::LineBreak => Json::Object(vec![("kind".to_string(), Json::String("hardBreak".to_string()))]),
            NodeValue::HtmlInline(raw) => Json::Object(vec![("kind".to_string(), Json::String("htmlInline".to_string())), ("raw".to_string(), Json::String(raw.clone()))]),
            // 🍃 Out-of-subset inline kinds — see `project_block`'s own note; same reasoning.
            _ => return None,
        })
    }
    //#endregion 🔖️Projection

    //#region 🔖️Inverse
    /// 🧭️ Navigates a `project_md`-shaped JSON document by `path`, returning the addressed
    /// container's blocks as an owned `Vec<Json>` — the projection-side twin of `navigate` above,
    /// operating on JSON instead of a live AST since it reads the ORIGINAL document independently
    /// of whatever `oracle_apply_mutation` goes on to build.
    fn container_at(doc: &Json, path: &[Json]) -> Result<Vec<Json>, String> {
        let mut current = doc.array("blocks");
        for step in path {
            current = match step.str("step").as_str() {
                "blockQuote" => {
                    let index = json_usize(step, "index")?;
                    let block = current.get(index).ok_or_else(|| format!("path: blockQuote index {index} out of range"))?;
                    block.array("blocks")
                }
                "listItem" => {
                    let index = json_usize(step, "index")?;
                    let item = json_usize(step, "item")?;
                    let block = current.get(index).ok_or_else(|| format!("path: listItem index {index} out of range"))?;
                    match block.array("items").get(item) {
                        Some(Json::Array(blocks)) => blocks.clone(),
                        _ => return Err(format!("path: listItem item {item} out of range")),
                    }
                }
                other => return Err(format!("path step carries unknown 'step' {other:?}")),
            };
        }
        Ok(current)
    }

    /// 🔎️ The block at `path`/`index` in the ORIGINAL projected document, or `None` when the
    /// address does not resolve — mirrors `MdMutation::inverse`'s own graceful `None =>
    /// MdMutation::NoMutation` fallback rather than panicking on a stale address.
    fn block_at(doc: &Json, path: &Json, index: usize) -> Result<Option<Json>, String> {
        let path_steps = as_array(Some(path));
        let container = container_at(doc, &path_steps)?;
        Ok(container.get(index).cloned())
    }

    /// ↩️ The spec for the mutation that undoes `spec`, computed from the ORIGINAL document's own
    /// INDEPENDENT `comrak` projection — the same restore-the-prior-value law `MdMutation::inverse`
    /// implements (`InsertBlock` ↔ `RemoveBlock`, `ReplaceBlock`/`SetInlines` restore the prior
    /// value, `SetSnapshot` restores the whole prior document), computed here from data rather than
    /// through that trait: this oracle module has no reachable path to the subject's own
    /// `protocol::Mutation` impl, and mirroring its algebra independently keeps the two
    /// implementations honestly separate.
    pub fn inverse_mutation_spec(original_input: &[u8], spec: &Json) -> Result<Json, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let original = project_md(original_input)?;
        let path = params.get("path").cloned().unwrap_or(Json::Array(Vec::new()));

        let (inverse_kind, inverse_params) = match kind.as_str() {
            "no-mutation" => ("no-mutation".to_string(), Json::Object(Vec::new())),
            "set-snapshot" => ("set-snapshot".to_string(), Json::Object(vec![("snapshot".to_string(), original)])),
            "insert-block" => {
                let index = json_usize(&params, "index")?;
                ("remove-block".to_string(), Json::Object(vec![("path".to_string(), path), ("index".to_string(), Json::Number(index as f64))]))
            }
            "remove-block" => {
                let index = json_usize(&params, "index")?;
                match block_at(&original, &path, index)? {
                    Some(block) => ("insert-block".to_string(), Json::Object(vec![("path".to_string(), path), ("index".to_string(), Json::Number(index as f64)), ("block".to_string(), block)])),
                    None => ("no-mutation".to_string(), Json::Object(Vec::new())),
                }
            }
            "replace-block" => {
                let index = json_usize(&params, "index")?;
                match block_at(&original, &path, index)? {
                    Some(block) => ("replace-block".to_string(), Json::Object(vec![("path".to_string(), path), ("index".to_string(), Json::Number(index as f64)), ("block".to_string(), block)])),
                    None => ("no-mutation".to_string(), Json::Object(Vec::new())),
                }
            }
            "set-inlines" => {
                let index = json_usize(&params, "index")?;
                match block_at(&original, &path, index)? {
                    Some(block) => match block.get("inlines") {
                        Some(inlines) => ("set-inlines".to_string(), Json::Object(vec![("path".to_string(), path), ("index".to_string(), Json::Number(index as f64)), ("inlines".to_string(), inlines.clone())])),
                        None => ("no-mutation".to_string(), Json::Object(Vec::new())),
                    },
                    None => ("no-mutation".to_string(), Json::Object(Vec::new())),
                }
            }
            other => return Err(format!("mutation kind {other:?} has no inverse spec")),
        };
        Ok(Json::Object(vec![("kind".to_string(), Json::String(inverse_kind)), ("params".to_string(), inverse_params)]))
    }
    //#endregion 🔖️Inverse
}

#[cfg(feature = "oracles")]
pub use live::{inverse_mutation_spec, oracle_apply_mutation, project_md};

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all, and every
/// entry point fails loudly. A missing oracle must never degrade into a silently skipped test.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use semio_repo_test_host::Json;
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_md(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn inverse_mutation_spec(_original_input: &[u8], _spec: &Json) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{inverse_mutation_spec, oracle_apply_mutation, project_md};
//#endregion 🔖️Unavailable
