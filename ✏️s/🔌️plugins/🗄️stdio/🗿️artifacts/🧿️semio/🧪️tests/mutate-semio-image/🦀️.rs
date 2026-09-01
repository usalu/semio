//! 🦀️ Semio IMAGE exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-image-mutate` is the
//! registered oracle `semio-image-python-pillow-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️image/🧪️oracle/🔣️.json`) — Pillow for the raster payload it genuinely speaks, plus an
//! independent Python implementation of the semio-native carrier and the thirteen verbs, living
//! beside this file as `🐍️component.py`. The runner dispatches the oracle role there and the subject
//! role here, and compares the two projections under `@comparison-ordered-json-v1`. Registering
//! oracle handlers here as well would put this repository's own answer on both sides of that
//! comparison, which is the precise failure the platform exists to prevent.
//!
//! **Where Pillow bites.** Every scenario that produces frames projects a `raster` report beside the
//! document: per frame, whether the plane length matches `width * height * 4`, and — when it does —
//! the reconstructed image's mode, size, per-band extrema and distinct-colour count. The oracle side
//! obtains those four facts by handing the planes to Pillow; this side computes them here, by hand,
//! from the same planes. A projection therefore matches only when a raster library that has never
//! seen this repository and this repository's own codec agree about the actual samples.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! artifact, `spec-vector-<kind>` requires the applied snapshot to be the committed after-snapshot,
//! and `identity-round-trip` requires both committed encodings to be reproduced byte for byte
//! through `law::carrier_is_exact`.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`), so neither
//! `protocol::Mutation` nor a `serde` derive is nameable from here. The small forward-only decoder
//! below therefore reads JSON STRUCTURE field by field, mirroring each payload's own declared serde
//! shape; it never invents or reimplements any mutation SEMANTICS, which still run through the real
//! entry points. Every input is read from a fixture the FEATURE declares, so neither adapter holds a
//! transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioImageMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-dimensions", "set-colorspace", "set-bit-depth", "set-icc", "insert-frame", "remove-frame", "move-frame", "set-frame-delay", "set-frame-pixels", "set-metadata-entry", "remove-metadata-entry"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::mutations::{
        apply_semio_image_mutation, insert_frame, inverse_semio_image_mutation, move_frame, remove_frame, remove_metadata_entry, set_bit_depth, set_colorspace, set_dimensions, set_frame_delay, set_frame_pixels, set_icc, set_metadata_entry, set_snapshot, SemioImageMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{
        decode_semio_image_pack, encode_semio_image_pack, parse_semio_image_dsl, print_semio_image_dsl, SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot,
    };
    use std::collections::BTreeSet;

    //#region 🔖️Input
    /// 🎞️ The real derived animation — the first three native-resolution frames of the committed
    /// `🖼️color-animated-text.gif`, decoded by Pillow once and committed here with its provenance.
    const ARTIFACT_DSL: &str = "local://🗣️artifact.dsl.semio";
    /// 🎒️ The same animation in its binary envelope, written by a separate codec from the DSL text.
    const ARTIFACT_PACK: &str = "local://🎒️artifact.pack.semio";

    /// 🧫️ Every fixture URI of one scheme the scenario's steps name, in step order. The feature is
    /// the single place those paths are written down; both adapters read them from there.
    fn step_uris(ctx: &Context, scheme: &str) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            for token in text.split_whitespace() {
                if token.starts_with(scheme) {
                    found.push(token.to_string());
                }
            }
        }
        found
    }

    fn only_uri(ctx: &Context, scheme: &str, what: &str) -> Result<String, String> {
        step_uris(ctx, scheme).into_iter().next().ok_or_else(|| format!("{}: the scenario names no {what}", ctx.scenario.id))
    }
    //#endregion 🔖️Input

    //#region 🔖️Decode
    fn u32_field(json: &Json, key: &str) -> Result<u32, String> {
        match json.get(key) {
            Some(Json::Number(value)) => Ok(*value as u32),
            other => Err(format!("expected a numeric field {key:?}, found {other:?}")),
        }
    }
    fn usize_field(json: &Json, key: &str) -> Result<usize, String> {
        u32_field(json, key).map(|value| value as usize)
    }
    fn bytes_field(json: &Json, key: &str) -> Result<Vec<u8>, String> {
        json.array(key)
            .iter()
            .map(|entry| match entry {
                Json::Number(value) => Ok(*value as u8),
                other => Err(format!("expected a byte number in {key:?}, found {other:?}")),
            })
            .collect()
    }
    fn optional_bytes_field(json: &Json, key: &str) -> Result<Option<Vec<u8>>, String> {
        match json.get(key) {
            Some(Json::Array(_)) => bytes_field(json, key).map(Some),
            _ => Ok(None),
        }
    }
    fn decode_colorspace(tag: &str) -> Result<SemioColorspace, String> {
        match tag {
            "rgb" => Ok(SemioColorspace::Rgb),
            "rgba" => Ok(SemioColorspace::Rgba),
            "grayscale" => Ok(SemioColorspace::Grayscale),
            "grayscaleAlpha" => Ok(SemioColorspace::GrayscaleAlpha),
            "indexed" => Ok(SemioColorspace::Indexed),
            other => Err(format!("unknown colorspace tag {other:?}")),
        }
    }
    fn decode_frame(json: &Json) -> Result<SemioImageFrame, String> {
        Ok(SemioImageFrame { delay_ms: u32_field(json, "delayMs")?, rgba8: bytes_field(json, "rgba8")? })
    }
    fn decode_snapshot(json: &Json) -> Result<SemioImageSnapshot, String> {
        Ok(SemioImageSnapshot {
            schema: json.str("schema"),
            width: u32_field(json, "width")?,
            height: u32_field(json, "height")?,
            colorspace: decode_colorspace(&json.str("colorspace"))?,
            bit_depth: u32_field(json, "bitDepth")? as u8,
            frames: json.array("frames").iter().map(decode_frame).collect::<Result<Vec<_>, _>>()?,
            icc: optional_bytes_field(json, "icc")?,
            metadata: json.array("metadata").iter().map(|entry| SemioImageMetadataEntry { key: entry.str("key"), value: entry.str("value") }).collect(),
        })
    }
    /// 🧫️ The committed mutation payloads are serde's internally-tagged shape (`{"mutation": "…"}`)
    /// with camelCase VARIANT names, exactly `SemioImageMutation`'s own `#[serde(tag = "mutation",
    /// rename_all = "camelCase")]` declaration. That attribute renames variants and NOT their
    /// fields, so a struct-variant's own keys stay snake_case — `bit_depth`, `delay_ms` — while a
    /// NESTED payload keeps its own camelCase (`bitDepth`, `delayMs`).
    /// 🧭️ `"noMutation"` is the dropped `NoMutation` verb's committed spelling (`no` is not an
    /// APPROVED_VERB, so the leaf migration could not keep it as a variant) — it maps to the
    /// identity mutation `SetSnapshot(base.clone())` rather than failing, so the committed
    /// `no-mutation` scenario keeps exercising the "nothing changes" law instead of being deleted.
    fn decode_mutation(json: &Json, base: &SemioImageSnapshot) -> Result<SemioImageMutation, String> {
        match json.str("mutation").as_str() {
            "noMutation" => Ok(SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })),
            "setSnapshot" => Ok(SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode_snapshot(json.get("snapshot").ok_or("setSnapshot payload carries no snapshot")?)? })),
            "setDimensions" => Ok(SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: u32_field(json, "width")?, height: u32_field(json, "height")? })),
            "setColorspace" => Ok(SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: decode_colorspace(&json.str("colorspace"))? })),
            "setBitDepth" => Ok(SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: u32_field(json, "bit_depth")? as u8 })),
            "setIcc" => Ok(SemioImageMutation::SetIcc(set_icc::SetIcc { icc: optional_bytes_field(json, "icc")? })),
            "insertFrame" => Ok(SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: usize_field(json, "index")?, frame: decode_frame(json.get("frame").ok_or("insertFrame payload carries no frame")?)? })),
            "removeFrame" => Ok(SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: usize_field(json, "index")? })),
            "moveFrame" => Ok(SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: usize_field(json, "from")?, to: usize_field(json, "to")? })),
            "setFrameDelay" => Ok(SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: usize_field(json, "index")?, delay_ms: u32_field(json, "delay_ms")? })),
            "setFramePixels" => Ok(SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: usize_field(json, "index")?, rgba8: bytes_field(json, "rgba8")? })),
            "setMetadataEntry" => Ok(SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: json.str("key"), value: json.str("value") })),
            "removeMetadataEntry" => Ok(SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: json.str("key") })),
            other => Err(format!("no decoder for mutation variant {other:?}")),
        }
    }

    /// 🎞️ The real derived animation, parsed through this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<SemioImageSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(ARTIFACT_DSL)?).map_err(|error| format!("the derived artifact is not UTF-8: {error}"))?;
        parse_semio_image_dsl(&text)
    }

    /// 📜️ The scenario's own committed mutation payload — the feature owns the vector. `base` is
    /// only consulted for the `no-mutation` scenario's identity mapping.
    fn payload(ctx: &Context, base: &SemioImageSnapshot) -> Result<SemioImageMutation, String> {
        let uri = only_uri(ctx, "local://🦠️", "mutation payload")?;
        decode_mutation(&ctx.fixture_json(&uri)?, base).map_err(|error| format!("{}: {error}", ctx.scenario.id))
    }
    //#endregion 🔖️Decode

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
    fn number(value: usize) -> Json {
        Json::Number(value as f64)
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1`: the snapshot's own
    /// structural JSON shape, with every RGBA8 sample present rather than summarised.
    fn snapshot_json(snapshot: &SemioImageSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("width".to_string(), Json::Number(snapshot.width as f64)),
            ("height".to_string(), Json::Number(snapshot.height as f64)),
            ("colorspace".to_string(), Json::String(colorspace_tag(snapshot.colorspace).to_string())),
            ("bitDepth".to_string(), Json::Number(snapshot.bit_depth as f64)),
            ("frames".to_string(), Json::Array(snapshot.frames.iter().map(|frame| Json::Object(vec![("delayMs".to_string(), Json::Number(frame.delay_ms as f64)), ("rgba8".to_string(), bytes_json(&frame.rgba8))])).collect())),
            ("icc".to_string(), snapshot.icc.as_ref().map(|bytes| bytes_json(bytes)).unwrap_or(Json::Null)),
            ("metadata".to_string(), Json::Array(snapshot.metadata.iter().map(|entry| Json::Object(vec![("key".to_string(), Json::String(entry.key.clone())), ("value".to_string(), Json::String(entry.value.clone()))])).collect())),
        ])
    }

    /// 🖼️ The four facts the oracle obtains from Pillow, computed here by hand from the same planes:
    /// whether an `RGBA` image of the declared geometry can be reconstructed from the plane at all,
    /// and — when it can — its mode, its size, its per-band extrema and its distinct-colour count.
    /// The band order is the interleave order the model declares, red first and alpha last.
    fn raster_json(snapshot: &SemioImageSnapshot) -> Json {
        let declared = snapshot.width as usize * snapshot.height as usize * 4;
        let frames = snapshot
            .frames
            .iter()
            .enumerate()
            .map(|(index, frame)| {
                let mut fields = vec![("index".to_string(), number(index)), ("planeBytes".to_string(), number(frame.rgba8.len())), ("declaredBytes".to_string(), number(declared))];
                if declared == 0 || frame.rgba8.len() != declared {
                    fields.push(("reconstructable".to_string(), Json::Bool(false)));
                    return Json::Object(fields);
                }
                fields.push(("reconstructable".to_string(), Json::Bool(true)));
                fields.push(("mode".to_string(), Json::String("RGBA".to_string())));
                fields.push(("size".to_string(), Json::Array(vec![Json::Number(snapshot.width as f64), Json::Number(snapshot.height as f64)])));
                let extrema = (0..4)
                    .map(|band| {
                        let samples = frame.rgba8.iter().skip(band).step_by(4);
                        let (low, high) = samples.fold((u8::MAX, u8::MIN), |(low, high), sample| (low.min(*sample), high.max(*sample)));
                        Json::Array(vec![Json::Number(low as f64), Json::Number(high as f64)])
                    })
                    .collect();
                fields.push(("extrema".to_string(), Json::Array(extrema)));
                let colours: BTreeSet<[u8; 4]> = frame.rgba8.chunks_exact(4).map(|pixel| [pixel[0], pixel[1], pixel[2], pixel[3]]).collect();
                fields.push(("colours".to_string(), number(colours.len())));
                Json::Object(fields)
            })
            .collect();
        Json::Object(vec![("library".to_string(), Json::String("pillow".to_string())), ("frames".to_string(), Json::Array(frames))])
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON both sides
    /// project — trimmed, because a real animation's planes are hundreds of kilobytes.
    fn disagreement(what: &str, got: &SemioImageSnapshot, expected: &SemioImageSnapshot) -> String {
        let short = |snapshot: &SemioImageSnapshot| {
            let frames = snapshot.frames.iter().map(|frame| format!("{{delayMs:{},planeBytes:{},planeDigest:{}}}", frame.delay_ms, frame.rgba8.len(), digest(&frame.rgba8))).collect::<Vec<_>>().join(",");
            let metadata = snapshot.metadata.iter().map(|entry| format!("{}={}", entry.key, entry.value)).collect::<Vec<_>>().join(",");
            format!("{}x{} {} bitDepth={} icc={} frames=[{}] metadata=[{}]", snapshot.width, snapshot.height, colorspace_tag(snapshot.colorspace), snapshot.bit_depth, snapshot.icc.as_ref().map(|bytes| bytes.len().to_string()).unwrap_or_else(|| "none".to_string()), frames, metadata)
        };
        format!("{what}\n     got: {}\nexpected: {}", short(got), short(expected))
    }

    fn apply(current: &mut SemioImageSnapshot, step: &SemioImageMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_image_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real derived animation by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = artifact(ctx)?;
        let step = payload(ctx, &current)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        Ok(Outcome::projection(Json::Object(vec![("document".to_string(), snapshot_json(&current)), ("raster".to_string(), raster_json(&current))])))
    }

    /// ↩️ The metamorphic inverse law on the real animation: applying the verb and then its OWN
    /// computed inverse must restore it exactly — frame order, every sample and the ICC blob
    /// included.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = artifact(ctx)?;
        let step = payload(ctx, &base)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = snapshot_json(&current);
        for undo in inverse_semio_image_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the animation", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    /// statement of what the verb means, independent of both implementations. The nullary kind owns
    /// no leaf, so its payload comes from the scenario's doc string and its expected answer is the
    /// before-snapshot itself.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let uris = step_uris(ctx, "asset://");
        let before = decode_snapshot(&ctx.fixture_json(uris.first().ok_or("the scenario names no before-snapshot")?)?)?;
        let (step, expected) = match uris.len() {
            3 => (decode_mutation(&ctx.fixture_json(&uris[1])?, &before)?, decode_snapshot(&ctx.fixture_json(&uris[2])?)?),
            _ => (decode_mutation(&ctx.doc_json()?, &before)?, before.clone()),
        };
        let mut current = before;
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(snapshot_json(&current)))
    }

    /// 🔁️ Both committed encodings of the real derived animation, each re-emitted from the parsed
    /// document.
    ///
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed artifacts this scenario reads were produced by the INDEPENDENT Python
    /// implementation from the same grammar, so reproducing them BYTE FOR BYTE is the correct answer
    /// here and `law::reparsed_not_copied` would be exactly backwards — the same reading
    /// `mutate-dag-1` records for `.dag.dsl.semio`. Nor is it a self-comparison: the bytes this side
    /// must match were written by the other implementation, and the digests of what each side
    /// emitted are what the runner compares.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let dsl_bytes = ctx.fixture_bytes(ARTIFACT_DSL)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: the derived artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_image_dsl(&text)?;
        let printed = print_semio_image_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_image_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(ARTIFACT_PACK)?;
        let unpacked = decode_semio_image_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different animation than the committed text artifact", &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_image_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_image_pack(&repacked_bytes)?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        Ok(Outcome::projection(Json::Object(vec![
            ("document".to_string(), snapshot_json(&parsed)),
            ("raster".to_string(), raster_json(&parsed)),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(digest(&repacked_bytes))),
            ("dslLength".to_string(), number(printed.len())),
            ("packLength".to_string(), number(repacked_bytes.len())),
        ])))
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
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
