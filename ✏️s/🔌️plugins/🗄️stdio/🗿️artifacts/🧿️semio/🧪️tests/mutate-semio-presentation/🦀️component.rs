//! 🦀️ Semio PRESENTATION exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR. Recorded no-oracle decision `semio-presentation-mutation-semantics`
//! (`../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧪️oracle/🔣️component.json`):
//! `s.stdio.semio.presentation` is a semio-NATIVE format with no third-party reader or writer, and
//! `python-pptx` — the obvious candidate now that the Python oracle host has landed — was rejected
//! because reaching a `SemioPresentationSnapshot` from pptx bytes needs this repository's own pptx
//! bridge, because it cannot create masters or layouts at all, and because `set-snapshot` has no
//! counterpart in any presentation library.
//!
//! `oracle` therefore reads the committed specification fixtures literally — no recomputation, no
//! reimplementation of mutation semantics — while `subject` drives this repository's own
//! `apply_semio_presentation_mutation` over the full fifteen-kind `SemioPresentationMutation`
//! vocabulary. Both roles read the SAME committed bytes through the host's `Context::fixture_json`,
//! so nothing about a fixture is transcribed into either role's source where it could silently
//! drift.
//!
//! The before-state of every vector is the real committed example artifact
//! `🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio`, and
//! `identity-round-trip` reads that artifact and its `.pack.semio` sibling directly, so the claim
//! that the vectors describe the real deck is checked rather than asserted.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so the subject module
//! below carries its own small, forward-only, hand-written structural JSON decoder built on the
//! framework's dependency-free `protocol::Json` — a mechanical field-by-field decode, never a
//! reimplementation of mutation semantics. The subject half is gated behind the generated host's
//! `sut` feature; the Rust SUBJECT phase is blocked this wave by a concurrent refactor in
//! `semio-framework-job`, so it is written and gated but not run.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never receives two sides to compare and the `oracle` handlers below are
//! the written statement of the reference answer rather than a second running party. Every law this
//! case claims is therefore asserted INSIDE the subject handler, which fails with both documents
//! printed. A handler that merely ran the mutation and returned would report a pass having checked
//! nothing.

use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioPresentationMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️presentation/
/// 🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. The contract's mutation-coverage gate keeps this list honest
/// against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it
/// honest against the enum.
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

/// 🗣️ The real committed example artifact, in both of the subset's own envelopes — read by
/// `identity-round-trip`'s subject role, which is the only role that decodes bytes rather than a
/// committed projection, so both constants belong to the `sut` build alone.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio";
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🗂️ Builds the same `local://` URIs `component.feature` declares, so the fixture-resolution
/// contract has already proved every one of them exists and pinned its digest.
fn before_uri(kind: &str) -> String {
    format!("local://{kind}/⬅️before.json")
}
#[cfg(feature = "sut")]
fn mutation_uri(kind: &str) -> String {
    format!("local://{kind}/🦠️mutation.json")
}
fn after_uri(kind: &str) -> String {
    format!("local://{kind}/➡️after.json")
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally through the host.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let after = ctx.fixture_json(&after_uri(kind))?;
        let bytes = after.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, after))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started, slide and shape order included.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = ctx.fixture_json(&before_uri(kind))?;
        let bytes = before.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, before))
    }
}

/// 🔮️ The round-trip reference answer: the committed canonical snapshot of the real artifact.
fn identity_oracle(ctx: &Context) -> Result<Outcome, String> {
    let expected = ctx.fixture_json(&before_uri("no-mutation"))?;
    let bytes = expected.to_string().into_bytes();
    Ok(Outcome::with_raw(bytes, expected))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{after_uri, before_uri, mutation_uri, DSL_ASSET, PACK_ASSET};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, RunStyle};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, semio_presentation_mutation_inverse, SemioPresentationMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{
        decode_semio_presentation_pack, encode_semio_presentation_pack, parse_semio_presentation_dsl, print_semio_presentation_dsl, PlaceholderKind, SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage,
        SlideShape, SlideTableCell, SlideTableRow,
    };

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
    /// 🧫️ `document::DocBlock` reuse — only the block kinds a real deck's text bodies, table cells
    /// and speaker notes actually carry are decoded; an unknown one is an error, never a silent
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
            DocBlock::PageBreak => object(vec![("kind", text("pageBreak"))]),
            other => panic!("mutate-semio-presentation: no projection for document block {other:?}"),
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
    fn outcome_of(snapshot: &SemioPresentationSnapshot) -> Outcome {
        let projection = snapshot_json(snapshot);
        let bytes = projection.to_string().into_bytes();
        Outcome::with_raw(bytes, projection)
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    fn fixture_for(kind: &str, ctx: &Context) -> Result<(SemioPresentationSnapshot, SemioPresentationMutation, SemioPresentationSnapshot), String> {
        let before = decode_snapshot(&ctx.fixture_json(&before_uri(kind))?);
        let mutation = decode_mutation(&ctx.fixture_json(&mutation_uri(kind))?);
        let after = decode_snapshot(&ctx.fixture_json(&after_uri(kind))?);
        Ok((before, mutation, after))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioPresentationSnapshot, expected: &SemioPresentationSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }

    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — slide ORDER included, which is what separates a real reorder from a rebuild
    /// that happens to keep the same set. The assertion lives here rather than in the comparison
    /// because a recorded no-oracle case runs no oracle role: a handler that merely returned `Ok`
    /// would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut base, mutation, expected) = fixture_for(kind, ctx)?;
            let outcome = apply_semio_presentation_mutation(&mut base, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", outcome.messages()));
            }
            if base != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &base, &expected));
            }
            Ok(outcome_of(&base))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly, the removed slide's position in the deck
    /// included and not merely its presence.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation, _expected) = fixture_for(kind, ctx)?;
            let mut current = base.clone();
            let outcome = apply_semio_presentation_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            for step in &semio_presentation_mutation_inverse(&mutation, &base) {
                let step_outcome = apply_semio_presentation_mutation(&mut current, step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", step_outcome.messages()));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            Ok(outcome_of(&current))
        }
    }

    /// 🔁 The real committed artifact, decoded from BOTH of its envelopes and carried back through
    /// each of them. Nothing here transcribes the deck: the only channel from the committed bytes to
    /// the projection is the subset's own codecs.
    pub fn identity(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed dsl artifact is not utf-8: {error}"))?;
        let from_text = parse_semio_presentation_dsl(&text)?;
        let from_pack = decode_semio_presentation_pack(&ctx.fixture_bytes(PACK_ASSET)?)?;
        if from_text != from_pack {
            return Err("identity-round-trip: the committed dsl and pack envelopes decode to different decks".to_string());
        }
        let repacked = decode_semio_presentation_pack(&encode_semio_presentation_pack(&from_text))?;
        let reparsed = parse_semio_presentation_dsl(&print_semio_presentation_dsl(&repacked))?;
        if reparsed != from_text {
            return Err("identity-round-trip: re-encoding through pack and dsl did not preserve the deck".to_string());
        }
        Ok(outcome_of(&reparsed))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", identity_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity);
    }
    built
}
//#endregion 🔖️Registration
