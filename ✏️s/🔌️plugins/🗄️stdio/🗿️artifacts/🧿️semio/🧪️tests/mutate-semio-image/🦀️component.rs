//! 🦀️ Semio IMAGE exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-image-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️image/🧪️oracle/🔣️component.json`): `s.stdio.semio.image` is a semio-NATIVE
//! format with no third-party reader or writer, and `SemioImageSnapshot` is a neutral raster model
//! no decoder emits, so `oracle` here reads the committed, independently handcrafted per-kind
//! specification fixtures — declared in `component.feature` as `asset://` references into their own
//! committed leaf directories and read through the host's `Context::fixture_json` at run time —
//! literally, with no recomputation and no reimplementation of mutation semantics. `subject` drives
//! this repository's own `apply_semio_image_mutation`/`inverse_semio_image_mutation` over the full
//! 13-kind `SemioImageMutation` vocabulary. Both sides project to structural JSON and
//! `ordered-json-v1` compares them.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so the subject module
//! below carries its own small, forward-only, hand-written JSON decoder turning the SAME fixture
//! bytes into real `SemioImageSnapshot`/`SemioImageMutation` values — a mechanical structural
//! decode, never a reimplementation of mutation semantics, and never a hand-transcribed Rust-literal
//! COPY that could silently drift from the committed file. The generated test-host crate carries no
//! `serde_json` dependency (only `semio-repo-test-host` and, behind `sut`, this subset's own crate),
//! so the decoder is built on the framework's own dependency-free `protocol::Json`. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation; the Rust SUBJECT phase is blocked this wave by a concurrent os-kernel
//! refactor, so it is written and gated but not run.
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
/// 🏷️ Mirrors `SemioImageMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-dimensions", "set-colorspace", "set-bit-depth", "set-icc", "insert-frame", "remove-frame", "move-frame", "set-frame-delay", "set-frame-pixels", "set-metadata-entry", "remove-metadata-entry"];

/// 🕳️ The one nullary kind: it owns no `🧬️mutations/<kind>/🧪️tests/` leaf of its own, so its
/// scenarios are declared individually in the feature and read the `move-frame` leaf's committed
/// before-snapshot as the document the identity law is asserted over.
const NULLARY_KIND: &str = "no-mutation";
//#endregion 🔖️Kinds

//#region 🔖️OracleFixtures
/// 🗂️ `(leaf directory, fixture slug)` per kind — the SAME `<dir>`/`<slug>` Examples columns
/// `component.feature` declares as `asset://` fixture references, kept here purely to rebuild the
/// identical URI strings the fixture-resolution contract already validated exist.
fn kind_fixture_dir(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "set-snapshot" => ("📸️set-snapshot", "retargets-the-document-onto-a-grayscale-sixteen-bit-variant"),
        "set-dimensions" => ("📐️set-dimensions", "widens-the-frameless-canvas-to-four-by-two"),
        "set-colorspace" => ("🌈️set-colorspace", "records-the-source-colorspace-as-rgba"),
        "set-bit-depth" => ("🔢️set-bit-depth", "raises-the-source-bit-depth-to-sixteen"),
        "set-icc" => ("🎨️set-icc", "attaches-an-icc-profile-where-there-was-none"),
        "insert-frame" => ("➕️insert-frame", "appends-a-second-frame-at-the-end"),
        "remove-frame" => ("📄remove-frame", "removes-the-leading-frame"),
        "move-frame" => ("🔀️move-frame", "moves-the-last-frame-to-the-front"),
        "set-frame-delay" => ("⏱️set-frame-delay", "slows-the-second-frame-down"),
        "set-frame-pixels" => ("🟪️set-frame-pixels", "repaints-the-only-frame-black"),
        "set-metadata-entry" => ("🏷️set-metadata-entry", "rewrites-the-existing-author-entry"),
        "remove-metadata-entry" => ("🗑️remove-metadata-entry", "removes-the-comment-entry-and-keeps-the-author-entry"),
        NULLARY_KIND => ("🔀️move-frame", "moves-the-last-frame-to-the-front"),
        other => panic!("mutate-semio-image: no fixture registered for kind {other:?}"),
    }
}
fn fixture_uri(kind: &str, leaf: &str) -> String {
    let (dir, slug) = kind_fixture_dir(kind);
    format!("asset://🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/{dir}/🧪️tests/{slug}/{leaf}")
}
fn before_uri(kind: &str) -> String {
    fixture_uri(kind, "📸️snapshot/⬅️before/🔣️component.json")
}
fn mutation_uri(kind: &str) -> String {
    fixture_uri(kind, "🦠️mutation/🔣️component.json")
}
fn after_uri(kind: &str) -> String {
    fixture_uri(kind, "📸️snapshot/➡️after/🔣️component.json")
}
//#endregion 🔖️OracleFixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally through the host.
/// `no-mutation` is the identity, so its reference answer is the committed BEFORE snapshot.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let uri = if kind == NULLARY_KIND { before_uri(kind) } else { after_uri(kind) };
        let answer = ctx.fixture_json(&uri)?;
        let bytes = answer.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, answer))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = ctx.fixture_json(&before_uri(kind))?;
        let bytes = before.to_string().into_bytes();
        Ok(Outcome::with_raw(bytes, before))
    }
}

/// 🔮️ The completeness reference answer: rebuilding the committed document from an empty snapshot
/// must land on the committed document itself, so the reference is that same committed file.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let before = ctx.fixture_json(&before_uri(NULLARY_KIND))?;
    let bytes = before.to_string().into_bytes();
    Ok(Outcome::with_raw(bytes, before))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{after_uri, before_uri, mutation_uri, NULLARY_KIND};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, inverse_semio_image_mutation, SemioImageMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};

    //#region 🔖️Decode
    /// 🧫️ A small, forward-only, hand-written structural decoder — turns the fixture bytes
    /// `Context::fixture_json` reads STRAIGHT from the committed file into real
    /// `SemioImageSnapshot`/`SemioImageMutation` values. It decodes JSON STRUCTURE only, field by
    /// field, mirroring each payload's own declared serde shape; it never invents or reimplements
    /// any mutation SEMANTICS, which still run through the real entry points below.
    fn u32_field(json: &Json, key: &str) -> u32 {
        match json.get(key) {
            Some(Json::Number(value)) => *value as u32,
            other => panic!("mutate-semio-image: expected a numeric field {key:?}, found {other:?}"),
        }
    }
    fn usize_field(json: &Json, key: &str) -> usize {
        u32_field(json, key) as usize
    }
    fn bytes_field(json: &Json, key: &str) -> Vec<u8> {
        json.array(key)
            .iter()
            .map(|entry| match entry {
                Json::Number(value) => *value as u8,
                other => panic!("mutate-semio-image: expected a byte number, found {other:?}"),
            })
            .collect()
    }
    fn optional_bytes_field(json: &Json, key: &str) -> Option<Vec<u8>> {
        match json.get(key) {
            Some(Json::Array(_)) => Some(bytes_field(json, key)),
            _ => None,
        }
    }
    fn decode_colorspace(tag: &str) -> SemioColorspace {
        match tag {
            "rgb" => SemioColorspace::Rgb,
            "rgba" => SemioColorspace::Rgba,
            "grayscale" => SemioColorspace::Grayscale,
            "grayscaleAlpha" => SemioColorspace::GrayscaleAlpha,
            "indexed" => SemioColorspace::Indexed,
            other => panic!("mutate-semio-image: unknown colorspace tag {other:?}"),
        }
    }
    fn decode_frame(json: &Json) -> SemioImageFrame {
        SemioImageFrame { delay_ms: u32_field(json, "delayMs"), rgba8: bytes_field(json, "rgba8") }
    }
    fn decode_metadata_entry(json: &Json) -> SemioImageMetadataEntry {
        SemioImageMetadataEntry { key: json.str("key"), value: json.str("value") }
    }
    fn decode_snapshot(json: &Json) -> SemioImageSnapshot {
        SemioImageSnapshot {
            schema: json.str("schema"),
            width: u32_field(json, "width"),
            height: u32_field(json, "height"),
            colorspace: decode_colorspace(&json.str("colorspace")),
            bit_depth: u32_field(json, "bitDepth") as u8,
            frames: json.array("frames").iter().map(decode_frame).collect(),
            icc: optional_bytes_field(json, "icc"),
            metadata: json.array("metadata").iter().map(decode_metadata_entry).collect(),
        }
    }
    /// 🧫️ The committed mutation fixture is serde's internally-tagged shape (`{"mutation": "…", …}`)
    /// with camelCase variant names — exactly `SemioImageMutation`'s own
    /// `#[serde(tag = "mutation", rename_all = "camelCase")]` declaration.
    fn decode_mutation(json: &Json) -> SemioImageMutation {
        match json.str("mutation").as_str() {
            "noMutation" => SemioImageMutation::NoMutation,
            "setSnapshot" => SemioImageMutation::SetSnapshot { snapshot: decode_snapshot(json.get("snapshot").expect("mutate-semio-image: setSnapshot fixture must carry a snapshot")) },
            "setDimensions" => SemioImageMutation::SetDimensions { width: u32_field(json, "width"), height: u32_field(json, "height") },
            "setColorspace" => SemioImageMutation::SetColorspace { colorspace: decode_colorspace(&json.str("colorspace")) },
            "setBitDepth" => SemioImageMutation::SetBitDepth { bit_depth: u32_field(json, "bitDepth") as u8 },
            "setIcc" => SemioImageMutation::SetIcc { icc: optional_bytes_field(json, "icc") },
            "insertFrame" => SemioImageMutation::InsertFrame { index: usize_field(json, "index"), frame: decode_frame(json.get("frame").expect("mutate-semio-image: insertFrame fixture must carry a frame")) },
            "removeFrame" => SemioImageMutation::RemoveFrame { index: usize_field(json, "index") },
            "moveFrame" => SemioImageMutation::MoveFrame { from: usize_field(json, "from"), to: usize_field(json, "to") },
            "setFrameDelay" => SemioImageMutation::SetFrameDelay { index: usize_field(json, "index"), delay_ms: u32_field(json, "delayMs") },
            "setFramePixels" => SemioImageMutation::SetFramePixels { index: usize_field(json, "index"), rgba8: bytes_field(json, "rgba8") },
            "setMetadataEntry" => SemioImageMutation::SetMetadataEntry { key: json.str("key"), value: json.str("value") },
            "removeMetadataEntry" => SemioImageMutation::RemoveMetadataEntry { key: json.str("key") },
            other => panic!("mutate-semio-image: no decoder for mutation variant {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Fixtures
    /// 🧫️ Reads the SAME committed fixture bytes the oracle role reads. The nullary kind owns no
    /// leaf, so its payload comes from the feature's own doc string instead of a leaf file, and —
    /// mirroring `../🦀️component.rs::mutate_oracle_for` — its expected after-state is the
    /// before-snapshot itself, never the borrowed leaf's `➡️after` file, which belongs to
    /// `move-frame` and would assert the opposite of identity.
    fn fixture_for(kind: &str, ctx: &Context) -> Result<(SemioImageSnapshot, SemioImageMutation, SemioImageSnapshot), String> {
        let before = decode_snapshot(&ctx.fixture_json(&before_uri(kind))?);
        let mutation = if kind == NULLARY_KIND { decode_mutation(&ctx.doc_json()?) } else { decode_mutation(&ctx.fixture_json(&mutation_uri(kind))?) };
        let after = if kind == NULLARY_KIND { before.clone() } else { decode_snapshot(&ctx.fixture_json(&after_uri(kind))?) };
        Ok((before, mutation, after))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioImageSnapshot, expected: &SemioImageSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Projection
    fn colorspace_tag(colorspace: SemioColorspace) -> &'static str {
        match colorspace {
            SemioColorspace::Rgb => "rgb",
            SemioColorspace::Rgba => "rgba",
            SemioColorspace::Grayscale => "grayscale",
            SemioColorspace::GrayscaleAlpha => "grayscaleAlpha",
            SemioColorspace::Indexed => "indexed",
        }
    }
    fn bytes_json(bytes: &[u8]) -> Json {
        Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
    }
    fn frame_json(frame: &SemioImageFrame) -> Json {
        Json::Object(vec![("delayMs".to_string(), Json::Number(frame.delay_ms as f64)), ("rgba8".to_string(), bytes_json(&frame.rgba8))])
    }
    fn metadata_json(entry: &SemioImageMetadataEntry) -> Json {
        Json::Object(vec![("key".to_string(), Json::String(entry.key.clone())), ("value".to_string(), Json::String(entry.value.clone()))])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, matching the committed fixtures field for field.
    fn snapshot_json(snapshot: &SemioImageSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("width".to_string(), Json::Number(snapshot.width as f64)),
            ("height".to_string(), Json::Number(snapshot.height as f64)),
            ("colorspace".to_string(), Json::String(colorspace_tag(snapshot.colorspace).to_string())),
            ("bitDepth".to_string(), Json::Number(snapshot.bit_depth as f64)),
            ("frames".to_string(), Json::Array(snapshot.frames.iter().map(frame_json).collect())),
            ("icc".to_string(), snapshot.icc.as_ref().map(|bytes| bytes_json(bytes)).unwrap_or(Json::Null)),
            ("metadata".to_string(), Json::Array(snapshot.metadata.iter().map(metadata_json).collect())),
        ])
    }
    fn outcome_of(snapshot: &SemioImageSnapshot) -> Outcome {
        let projection = snapshot_json(snapshot);
        let bytes = projection.to_string().into_bytes();
        Outcome::with_raw(bytes, projection)
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — colorspace tag, plane geometry and sample payload alike. The assertion lives
    /// here rather than in the comparison because a recorded no-oracle case runs no oracle role: a
    /// handler that merely returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut base, mutation, expected) = fixture_for(kind, ctx)?;
            let outcome = apply_semio_image_mutation(&mut base, &mutation);
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
    /// restore the committed before-snapshot exactly, including the sample bytes a plane edit
    /// overwrote.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation, _expected) = fixture_for(kind, ctx)?;
            let mut current = base.clone();
            let outcome = apply_semio_image_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", outcome.messages()));
            }
            for step in &inverse_semio_image_mutation(&mutation, &base) {
                let step_outcome = apply_semio_image_mutation(&mut current, step);
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

    /// 🔁️ The completeness law: the subset's own full-replace `set-snapshot` diff must carry an
    /// empty snapshot all the way to the committed document, with no slot of the typed model
    /// silently dropped on the way through.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = decode_snapshot(&ctx.fixture_json(&before_uri(NULLARY_KIND))?);
        let mut rebuilt = SemioImageSnapshot::default();
        let outcome = apply_semio_image_mutation(&mut rebuilt, &SemioImageMutation::SetSnapshot { snapshot: committed.clone() });
        if !outcome.messages().is_empty() {
            return Err(format!("identity-round-trip: full-replace rejected: {:?}", outcome.messages()));
        }
        if rebuilt != committed {
            return Err(disagreement("identity-round-trip: rebuilding the committed image from an empty snapshot did not land on it — a slot of the typed model was dropped on the way through the full-replace diff", &rebuilt, &committed));
        }
        Ok(outcome_of(&rebuilt))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so every kind is registered in a loop over `KINDS`.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
