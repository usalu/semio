//! 🦀️ Semio PRESENTATION exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-
//! END-TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-presentation-mutate`
//! is the registered oracle `semio-presentation-python-independent` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️presentation/🧪️oracle/🔣️.json`) — an independent Python implementation of the
//! semio presentation carrier, document's own recursive `DocBlock` grammar and all fifteen verbs,
//! written from the committed grammar and protocol documents and living beside this file as
//! `🐍️component.py`. The runner dispatches the oracle role there and the subject role here, and
//! compares the two projections under `@comparison-ordered-json-v1`. Registering oracle handlers
//! here as well would put this repository's own answer on both sides of that comparison, which is
//! the precise failure the platform exists to prevent.
//!
//! **The deck under test is a real one.** `local://🗣️talk.dsl.semio` and its binary twin were derived
//! ONCE from the real committed PowerPoint deck
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🧫️fixtures/🎞️semio-talk.pptx` — a genuine 2020 conference
//! talk with one master, eleven layouts, seven slides, ninety-eight shapes and three embedded PNG
//! parts — by an independent Python OOXML reader built on `zipfile` and `xml.etree`, never through
//! this repository's own pptx bridge. Using that bridge is exactly what the old no-oracle decision
//! refused, and it stays out of the fixture's provenance for the same reason.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! deck with slide and shape ORDER intact, `spec-vector-<kind>` requires the applied deck to be the
//! committed after-snapshot, and `identity-round-trip` requires all four committed encodings — the
//! derived talk deck's and the committed `📽️deck` example's — to be reproduced byte for byte through
//! `law::carrier_is_exact`.
//!
//! The generated host links only `semio-repo-test-host` and, behind `sut`, this subset's own crate —
//! no `serde`, no `serde_json` — so the subject module below carries its own small, forward-only,
//! hand-written structural JSON decoder built on the framework's dependency-free `protocol::Json`. It
//! is a mechanical field-by-field decode, never a reimplementation of mutation semantics, and every
//! input it reads is a fixture the FEATURE declares, so neither adapter holds a transcription that
//! could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioPresentationMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/
/// 🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. The contract's mutation-coverage gate keeps this list honest
/// against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it
/// honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-slide",
    "remove-slide",
    "set-slide-layout",
    "set-slide-notes",
    "insert-shape",
    "remove-shape",
    "set-shape-frame",
    "set-textbox-blocks",
    "insert-master",
    "remove-master",
    "insert-layout",
    "remove-layout",
    "set-layout-master",
];

//#endregion 🔖️Kinds


//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocListItem, DocRun, DocTableCell, DocTableRow, RunStyle};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, semio_presentation_mutation_inverse, SemioPresentationMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{
        decode_semio_presentation_pack, encode_semio_presentation_pack, parse_semio_presentation_dsl, print_semio_presentation_dsl, PlaceholderKind, SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage,
        SlideShape, SlideTableCell, SlideTableRow,
    };

    //#region 🔖️Input
    /// 🎤️ The real derived talk deck — the committed `🎞️semio-talk.pptx` read once by an independent
    /// OOXML reader and written out through the independent Python implementation of this carrier.
    const TALK_DSL: &str = "local://🗣️talk.dsl.semio";
    const TALK_PACK: &str = "local://🎒️talk.pack.semio";
    /// 📽️ The committed example deck, kept because it is the artifact this subset's own
    /// `fixture_honesty_law` pins to `demo_semio_presentation_snapshot()`.
    const DECK_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio";
    const DECK_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🎒️example.pack.semio";

    /// 🧫️ Every fixture URI of one scheme the scenario's steps name, in step order — including the
    /// cells of a step's data table, which is where the specification-vector paths live. The feature
    /// is the single place those paths are written down; both adapters read them from there.
    fn step_uris(ctx: &Context, scheme: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            for token in text.split_whitespace() {
                if token.starts_with(scheme) {
                    found.push(token.to_string());
                }
            }
        }
        for table in &ctx.scenario.data_tables {
            for cell in table.iter().flatten() {
            for token in cell.split_whitespace() {
                if token.starts_with(scheme) {
                    found.push(token.to_string());
                }
            }
            }
        }
        found
    }
    //#endregion 🔖️Input

    //#region 🔖️JsonReaders
    /// 🧫️ Structural readers over the framework's dependency-free `Json`. Every one of them mirrors
    /// one committed payload's own declared shape; none of them knows anything about mutation
    /// semantics.
    fn field<'a>(json: &'a Json, key: &str) -> &'a Json {
        json.get(key).unwrap_or_else(|| panic!("mutate-semio-presentation: fixture is missing the field {key:?}"))
    }
    fn number(json: &Json, key: &str) -> f64 {
        match json.get(key) {
            Some(Json::Number(value)) => *value,
            other => panic!("mutate-semio-presentation: expected a numeric field {key:?}, found {other:?}"),
        }
    }
    fn usize_field(json: &Json, key: &str) -> usize {
        number(json, key) as usize
    }
    fn opt_string(json: &Json, key: &str) -> Option<String> {
        match json.get(key) {
            Some(Json::String(value)) => Some(value.clone()),
            _ => None,
        }
    }
    fn opt_number(json: &Json, key: &str) -> Option<f64> {
        match json.get(key) {
            Some(Json::Number(value)) => Some(*value),
            _ => None,
        }
    }
    fn flag(json: &Json, key: &str) -> bool {
        matches!(json.get(key), Some(Json::Bool(true)))
    }
    //#endregion 🔖️JsonReaders

    //#region 🔖️DecodeDocument
    /// 🧫️ `document::DocBlock` reuse — all eight block kinds, because the real derived deck's
    /// mutation payloads exercise the whole union; an unknown one is an error, never a silent
    /// default that would make a wrong fixture look right.
    fn decode_run_style(json: &Json) -> RunStyle {
        RunStyle {
            bold: flag(json, "bold"),
            italic: flag(json, "italic"),
            underline: flag(json, "underline"),
            size: opt_number(json, "size"),
            font: opt_string(json, "font"),
            color: opt_string(json, "color"),
            link: opt_string(json, "link"),
        }
    }
    fn decode_run(json: &Json) -> DocRun {
        DocRun { text: json.str("text"), style: decode_run_style(field(json, "style")) }
    }
    fn decode_block(json: &Json) -> DocBlock {
        match json.str("kind").as_str() {
            "paragraph" => DocBlock::Paragraph { style_id: opt_string(json, "style_id"), runs: json.array("runs").iter().map(decode_run).collect() },
            "heading" => DocBlock::Heading { level: number(json, "level") as u8, style_id: opt_string(json, "style_id"), runs: json.array("runs").iter().map(decode_run).collect() },
            "list" => DocBlock::List { ordered: flag(json, "ordered"), items: json.array("items").iter().map(|item| DocListItem { blocks: decode_blocks(item, "blocks") }).collect() },
            "table" => DocBlock::Table {
                rows: json
                    .array("rows")
                    .iter()
                    .map(|row| DocTableRow { cells: row.array("cells").iter().map(|cell| DocTableCell { blocks: decode_blocks(cell, "blocks") }).collect() })
                    .collect(),
            },
            "code" => DocBlock::Code { language: opt_string(json, "language"), text: json.str("text") },
            "quote" => DocBlock::Quote { blocks: decode_blocks(json, "blocks") },
            "image" => DocBlock::Image { image_id: json.str("image_id"), alt: json.str("alt"), width: opt_number(json, "width"), height: opt_number(json, "height") },
            "pageBreak" => DocBlock::PageBreak,
            other => panic!("mutate-semio-presentation: no decoder for document block kind {other:?}"),
        }
    }
    fn decode_blocks(json: &Json, key: &str) -> Vec<DocBlock> {
        json.array(key).iter().map(decode_block).collect()
    }
    //#endregion 🔖️DecodeDocument

    //#region 🔖️Decode
    fn decode_frame(json: &Json) -> SlideFrame {
        let origin = field(json, "origin");
        SlideFrame { origin: SemioPoint2 { x: number(origin, "x"), y: number(origin, "y") }, width: number(json, "width"), height: number(json, "height") }
    }
    fn decode_placeholder_kind(json: &Json) -> PlaceholderKind {
        match json.str("kind").as_str() {
            "title" => PlaceholderKind::Title,
            "subtitle" => PlaceholderKind::Subtitle,
            "body" => PlaceholderKind::Body,
            "footer" => PlaceholderKind::Footer,
            "slideNumber" => PlaceholderKind::SlideNumber,
            "dateTime" => PlaceholderKind::DateTime,
            "other" => PlaceholderKind::Other { value: json.str("value") },
            other => panic!("mutate-semio-presentation: unknown placeholder kind {other:?}"),
        }
    }
    fn decode_image(json: &Json) -> SlidePictureImage {
        SlidePictureImage {
            asset_id: json.str("assetId"),
            mime: json.str("mime"),
            bytes: json
                .array("bytes")
                .iter()
                .map(|entry| match entry {
                    Json::Number(value) => *value as u8,
                    other => panic!("mutate-semio-presentation: expected a byte number, found {other:?}"),
                })
                .collect(),
        }
    }
    fn decode_table_rows(json: &Json) -> Vec<SlideTableRow> {
        json.array("rows").iter().map(|row| SlideTableRow { cells: row.array("cells").iter().map(|cell| SlideTableCell { blocks: decode_blocks(cell, "blocks") }).collect() }).collect()
    }
    fn decode_shape(json: &Json) -> SlideShape {
        match json.str("shapeKind").as_str() {
            "textBox" => SlideShape::TextBox { frame: decode_frame(field(json, "frame")), blocks: decode_blocks(json, "blocks") },
            "picture" => SlideShape::Picture { frame: decode_frame(field(json, "frame")), image: decode_image(field(json, "image")) },
            "table" => SlideShape::Table { frame: decode_frame(field(json, "frame")), rows: decode_table_rows(json) },
            "placeholder" => SlideShape::Placeholder { frame: decode_frame(field(json, "frame")), kind: decode_placeholder_kind(field(json, "kind")) },
            other => panic!("mutate-semio-presentation: unknown shape kind {other:?}"),
        }
    }
    fn decode_shapes(json: &Json) -> Vec<SlideShape> {
        json.array("shapes").iter().map(decode_shape).collect()
    }
    fn decode_master(json: &Json) -> SlideMaster {
        SlideMaster { id: json.str("id"), shapes: decode_shapes(json) }
    }
    fn decode_layout(json: &Json) -> SlideLayout {
        SlideLayout { id: json.str("id"), master_id: json.str("masterId"), shapes: decode_shapes(json) }
    }
    fn decode_slide(json: &Json) -> Slide {
        Slide { id: json.str("id"), layout_id: opt_string(json, "layoutId"), shapes: decode_shapes(json), notes: decode_blocks(json, "notes") }
    }
    fn decode_snapshot(json: &Json) -> SemioPresentationSnapshot {
        SemioPresentationSnapshot {
            schema: json.str("schema"),
            masters: json.array("masters").iter().map(decode_master).collect(),
            layouts: json.array("layouts").iter().map(decode_layout).collect(),
            slides: json.array("slides").iter().map(decode_slide).collect(),
        }
    }

    /// 🧫️ `SemioPresentationMutation` is internally tagged on `mutation` with camelCase VARIANT
    /// names, while its struct-variant FIELDS keep their Rust spelling — a container `rename_all` on
    /// an enum renames variants only, which is why `slide_index`/`shape_index`/`layout_id`/
    /// `master_id` are snake_case on the wire even though `SlideLayout::master_id` (a struct) is
    /// `masterId`.
    fn decode_mutation(json: &Json) -> SemioPresentationMutation {
        match json.str("mutation").as_str() {
            "noMutation" => SemioPresentationMutation::NoMutation,
            "setSnapshot" => SemioPresentationMutation::SetSnapshot { snapshot: decode_snapshot(field(json, "snapshot")) },
            "insertSlide" => SemioPresentationMutation::InsertSlide { index: usize_field(json, "index"), slide: decode_slide(field(json, "slide")) },
            "removeSlide" => SemioPresentationMutation::RemoveSlide { index: usize_field(json, "index") },
            "setSlideLayout" => SemioPresentationMutation::SetSlideLayout { index: usize_field(json, "index"), layout_id: opt_string(json, "layout_id") },
            "setSlideNotes" => SemioPresentationMutation::SetSlideNotes { index: usize_field(json, "index"), notes: decode_blocks(json, "notes") },
            "insertShape" => SemioPresentationMutation::InsertShape { slide_index: usize_field(json, "slide_index"), shape_index: usize_field(json, "shape_index"), shape: decode_shape(field(json, "shape")) },
            "removeShape" => SemioPresentationMutation::RemoveShape { slide_index: usize_field(json, "slide_index"), shape_index: usize_field(json, "shape_index") },
            "setShapeFrame" => SemioPresentationMutation::SetShapeFrame { slide_index: usize_field(json, "slide_index"), shape_index: usize_field(json, "shape_index"), frame: decode_frame(field(json, "frame")) },
            "setTextBoxBlocks" => SemioPresentationMutation::SetTextBoxBlocks { slide_index: usize_field(json, "slide_index"), shape_index: usize_field(json, "shape_index"), blocks: decode_blocks(json, "blocks") },
            "insertMaster" => SemioPresentationMutation::InsertMaster { master: decode_master(field(json, "master")) },
            "removeMaster" => SemioPresentationMutation::RemoveMaster { id: json.str("id") },
            "insertLayout" => SemioPresentationMutation::InsertLayout { layout: decode_layout(field(json, "layout")) },
            "removeLayout" => SemioPresentationMutation::RemoveLayout { id: json.str("id") },
            "setLayoutMaster" => SemioPresentationMutation::SetLayoutMaster { id: json.str("id"), master_id: json.str("master_id") },
            other => panic!("mutate-semio-presentation: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Projection
    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }
    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }
    fn optional_text(value: &Option<String>) -> Json {
        match value {
            Some(inner) => Json::String(inner.clone()),
            None => Json::Null,
        }
    }
    fn optional_number(value: &Option<f64>) -> Json {
        match value {
            Some(inner) => Json::Number(*inner),
            None => Json::Null,
        }
    }
    fn run_style_json(style: &RunStyle) -> Json {
        object(vec![
            ("bold", Json::Bool(style.bold)),
            ("italic", Json::Bool(style.italic)),
            ("underline", Json::Bool(style.underline)),
            ("size", optional_number(&style.size)),
            ("font", optional_text(&style.font)),
            ("color", optional_text(&style.color)),
            ("link", optional_text(&style.link)),
        ])
    }
    fn run_json(run: &DocRun) -> Json {
        object(vec![("text", text(&run.text)), ("style", run_style_json(&run.style))])
    }
    fn block_json(block: &DocBlock) -> Json {
        match block {
            DocBlock::Paragraph { style_id, runs } => object(vec![("kind", text("paragraph")), ("style_id", optional_text(style_id)), ("runs", Json::Array(runs.iter().map(run_json).collect()))]),
            DocBlock::Heading { level, style_id, runs } => {
                object(vec![("kind", text("heading")), ("level", Json::Number(f64::from(*level))), ("style_id", optional_text(style_id)), ("runs", Json::Array(runs.iter().map(run_json).collect()))])
            }
            DocBlock::List { ordered, items } => object(vec![
                ("kind", text("list")),
                ("ordered", Json::Bool(*ordered)),
                ("items", Json::Array(items.iter().map(|item| object(vec![("blocks", blocks_json(&item.blocks))])).collect())),
            ]),
            DocBlock::Table { rows } => object(vec![(
                "kind",
                text("table"),
            ), (
                "rows",
                Json::Array(rows.iter().map(|row| object(vec![("cells", Json::Array(row.cells.iter().map(|cell| object(vec![("blocks", blocks_json(&cell.blocks))])).collect()))])).collect()),
            )]),
            DocBlock::Code { language, text: body } => object(vec![("kind", text("code")), ("language", optional_text(language)), ("text", text(body))]),
            DocBlock::Quote { blocks } => object(vec![("kind", text("quote")), ("blocks", blocks_json(blocks))]),
            DocBlock::Image { image_id, alt, width, height } => object(vec![
                ("kind", text("image")),
                ("image_id", text(image_id)),
                ("alt", text(alt)),
                ("width", optional_number(width)),
                ("height", optional_number(height)),
            ]),
            DocBlock::PageBreak => object(vec![("kind", text("pageBreak"))]),
        }
    }
    fn blocks_json(blocks: &[DocBlock]) -> Json {
        Json::Array(blocks.iter().map(block_json).collect())
    }
    fn frame_json(frame: &SlideFrame) -> Json {
        object(vec![
            ("origin", object(vec![("x", Json::Number(frame.origin.x)), ("y", Json::Number(frame.origin.y))])),
            ("width", Json::Number(frame.width)),
            ("height", Json::Number(frame.height)),
        ])
    }
    fn placeholder_kind_json(kind: &PlaceholderKind) -> Json {
        match kind {
            PlaceholderKind::Title => object(vec![("kind", text("title"))]),
            PlaceholderKind::Subtitle => object(vec![("kind", text("subtitle"))]),
            PlaceholderKind::Body => object(vec![("kind", text("body"))]),
            PlaceholderKind::Footer => object(vec![("kind", text("footer"))]),
            PlaceholderKind::SlideNumber => object(vec![("kind", text("slideNumber"))]),
            PlaceholderKind::DateTime => object(vec![("kind", text("dateTime"))]),
            PlaceholderKind::Other { value } => object(vec![("kind", text("other")), ("value", text(value))]),
        }
    }
    fn shape_json(shape: &SlideShape) -> Json {
        match shape {
            SlideShape::TextBox { frame, blocks } => object(vec![("shapeKind", text("textBox")), ("frame", frame_json(frame)), ("blocks", blocks_json(blocks))]),
            SlideShape::Picture { frame, image } => object(vec![
                ("shapeKind", text("picture")),
                ("frame", frame_json(frame)),
                (
                    "image",
                    object(vec![
                        ("assetId", text(&image.asset_id)),
                        ("mime", text(&image.mime)),
                        ("bytes", Json::Array(image.bytes.iter().map(|byte| Json::Number(f64::from(*byte))).collect())),
                    ]),
                ),
            ]),
            SlideShape::Table { frame, rows } => object(vec![
                ("shapeKind", text("table")),
                ("frame", frame_json(frame)),
                ("rows", Json::Array(rows.iter().map(|row| object(vec![("cells", Json::Array(row.cells.iter().map(|cell| object(vec![("blocks", blocks_json(&cell.blocks))])).collect()))])).collect())),
            ]),
            SlideShape::Placeholder { frame, kind } => object(vec![("shapeKind", text("placeholder")), ("frame", frame_json(frame)), ("kind", placeholder_kind_json(kind))]),
        }
    }
    fn shapes_json(shapes: &[SlideShape]) -> Json {
        Json::Array(shapes.iter().map(shape_json).collect())
    }

    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field. Slide and shape ORDER
    /// is load-bearing — this vocabulary addresses both by index — so the projection preserves it.
    pub fn snapshot_json(snapshot: &SemioPresentationSnapshot) -> Json {
        object(vec![
            ("schema", text(&snapshot.schema)),
            ("masters", Json::Array(snapshot.masters.iter().map(|master| object(vec![("id", text(&master.id)), ("shapes", shapes_json(&master.shapes))])).collect())),
            (
                "layouts",
                Json::Array(snapshot.layouts.iter().map(|layout| object(vec![("id", text(&layout.id)), ("masterId", text(&layout.master_id)), ("shapes", shapes_json(&layout.shapes))])).collect()),
            ),
            (
                "slides",
                Json::Array(
                    snapshot
                        .slides
                        .iter()
                        .map(|slide| object(vec![("id", text(&slide.id)), ("layoutId", optional_text(&slide.layout_id)), ("shapes", shapes_json(&slide.shapes)), ("notes", blocks_json(&slide.notes))]))
                        .collect(),
                ),
            ),
        ])
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎤️ The real derived talk deck, parsed through this repository's own DSL codec.
    fn talk(ctx: &Context) -> Result<SemioPresentationSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(TALK_DSL)?).map_err(|error| format!("the derived talk deck is not UTF-8: {error}"))?;
        parse_semio_presentation_dsl(&text)
    }

    /// 📜️ The scenario's own committed mutation payload — the feature owns the vector.
    fn payload(ctx: &Context) -> Result<SemioPresentationMutation, String> {
        let uri = step_uris(ctx, "local://🦠️").into_iter().next().ok_or_else(|| format!("{}: the scenario names no mutation payload", ctx.scenario.id))?;
        Ok(decode_mutation(&ctx.fixture_json(&uri)?))
    }

    fn apply(current: &mut SemioPresentationSnapshot, step: &SemioPresentationMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_presentation_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON both sides
    /// project — trimmed to the deck's shape, because the real talk deck embeds three PNG parts.
    fn disagreement(what: &str, got: &SemioPresentationSnapshot, expected: &SemioPresentationSnapshot) -> String {
        let short = |deck: &SemioPresentationSnapshot| {
            let slides = deck.slides.iter().map(|slide| format!("{}({} shapes,{} notes)", slide.id, slide.shapes.len(), slide.notes.len())).collect::<Vec<_>>().join(",");
            let layouts = deck.layouts.iter().map(|layout| format!("{}->{}", layout.id, layout.master_id)).collect::<Vec<_>>().join(",");
            let masters = deck.masters.iter().map(|master| master.id.clone()).collect::<Vec<_>>().join(",");
            format!("masters=[{masters}] layouts=[{layouts}] slides=[{slides}] digest={}", digest(snapshot_json(deck).to_string().as_bytes()))
        };
        format!("{what}\n     got: {}\nexpected: {}", short(got), short(expected))
    }

    /// 🎯️ One verb applied to the real derived talk deck by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = talk(ctx)?;
        apply(&mut current, &payload(ctx)?, &ctx.scenario.id)?;
        Ok(Outcome::projection(snapshot_json(&current)))
    }

    /// ↩️ The metamorphic inverse law on the real deck: applying the verb and then its OWN computed
    /// inverse must restore it exactly, slide and shape ORDER included — which is what an
    /// index-addressed vocabulary makes load-bearing.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = talk(ctx)?;
        let step = payload(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current);
        for undo in &semio_presentation_mutation_inverse(&step, &base) {
            apply(&mut current, undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the deck", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector, whose
    /// before-state is the committed `📽️deck` example artifact — a THIRD statement of what the verb
    /// means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let uris = step_uris(ctx, "local://");
        if uris.len() < 3 {
            return Err(format!("{}: the scenario names {} specification-vector fixtures, expected three", ctx.scenario.id, uris.len()));
        }
        let mut current = decode_snapshot(&ctx.fixture_json(&uris[0])?);
        let step = decode_mutation(&ctx.fixture_json(&uris[1])?);
        let expected = decode_snapshot(&ctx.fixture_json(&uris[2])?);
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied deck does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(snapshot_json(&current)))
    }

    /// 🔁️ Both committed encodings of BOTH decks, each re-emitted from the parsed document.
    ///
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so
    /// reproducing all four committed files BYTE FOR BYTE is the correct answer here and
    /// `law::reparsed_not_copied` would be exactly backwards — the same reading `mutate-dag-1`
    /// records for `.dag.dsl.semio`. Nor is it a self-comparison any more: the talk deck's two files
    /// were written by the INDEPENDENT Python implementation from the same grammar, and the digests
    /// of what each side emitted are what the runner compares.
    pub fn identity(ctx: &Context) -> Result<Outcome, String> {
        let mut report = Vec::new();
        for (name, dsl_uri, pack_uri) in [("talk", TALK_DSL, TALK_PACK), ("deck", DECK_DSL, DECK_PACK)] {
            let dsl_bytes = ctx.fixture_bytes(dsl_uri)?;
            let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: the committed {name} artifact is not UTF-8: {error}"))?;
            let parsed = parse_semio_presentation_dsl(&text)?;
            let printed = print_semio_presentation_dsl(&parsed);
            carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
            let reparsed = parse_semio_presentation_dsl(&printed)?;
            if reparsed != parsed {
                return Err(disagreement(&format!("identity-round-trip: printing the {name} back to DSL and reparsing it lost content"), &reparsed, &parsed));
            }
            let pack_bytes = ctx.fixture_bytes(pack_uri)?;
            let unpacked = decode_semio_presentation_pack(&pack_bytes)?;
            if unpacked != parsed {
                return Err(disagreement(&format!("identity-round-trip: the {name}'s binary twin decodes to a different deck than its text artifact"), &unpacked, &parsed));
            }
            let repacked_bytes = encode_semio_presentation_pack(&parsed);
            carrier_is_exact(&repacked_bytes, &pack_bytes)?;
            let repacked = decode_semio_presentation_pack(&repacked_bytes)?;
            if repacked != parsed {
                return Err(disagreement(&format!("identity-round-trip: encoding the {name} to a pack and decoding it back lost content"), &repacked, &parsed));
            }
            report.push((
                name.to_string(),
                Json::Object(vec![
                    ("document".to_string(), snapshot_json(&parsed)),
                    ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
                    ("packDigest".to_string(), Json::String(digest(&repacked_bytes))),
                    ("dslLength".to_string(), Json::Number(printed.len() as f64)),
                    ("packLength".to_string(), Json::Number(repacked_bytes.len() as f64)),
                ]),
            ));
        }
        Ok(Outcome::projection(Json::Object(report)))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::identity);
    }
    built
}
//#endregion 🔖️Registration
