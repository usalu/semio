//! 🦀️ Semio VIDEO exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file registers the SUBJECT role only. The reference answer comes from `🐍️component.py`
//! beside it — an independent Python implementation of the same carrier and the same
//! nine verbs, written from the committed grammars, registered as the oracle
//! `semio-video-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️video/🧪️oracle/🔣️.json`. Registering oracle
//! handlers here as well would put this repository's own answer on both sides of the comparison,
//! which is the one failure the platform exists to prevent, so the registrations this file used to
//! carry are gone rather than merely unused.
//!
//! Every scenario drives this repository's own production entry points — `parse_semio_video_dsl`/`print_semio_video_dsl` for
//! the carrier and `apply_semio_video_mutation`/`inverse_semio_video_mutation` for the vocabulary — over the real committed clip
//! artifact `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio`, and projects the resulting snapshot as structural JSON for
//! `ordered-json-v1` to compare against the Python side's.
//!
//! The mutation parameters and the specification-vector paths live in `component.feature`, so both
//! implementations read one physical copy of each and cannot drift apart. The laws are asserted
//! here IN ROLE as well as compared: `inverse-` requires the clip back, `spec-vector-` requires
//! the committed after-snapshot, and `identity-round-trip` requires byte-exact re-emission through
//! `law::carrier_is_exact`.
//!
//! The subject half is gated behind the generated host's `sut` feature so a non-subject build never
//! compiles the local implementation.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioVideoMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the registration loop runs in
/// builds where the subject crate is not linked. The contract's mutation-coverage gate keeps this
/// list honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file
/// keeps it honest against the enum.
#[cfg_attr(not(feature = "sut"), allow(dead_code))]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::video::schema::mutations::{
        apply_semio_video_mutation, insert_sample, insert_stream, inverse_semio_video_mutation, remove_sample, remove_stream, set_sample_data, set_sample_flags, set_snapshot, set_stream_meta, SemioVideoMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{parse_semio_video_dsl, print_semio_video_dsl, SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind};

    //#region 🔖️JsonReaders
    fn text(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(found)) => Ok(found.clone()),
            _ => Err(format!("vector member {key:?} must be a string")),
        }
    }
    fn number(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(found)) => Ok(*found),
            _ => Err(format!("vector member {key:?} must be a number")),
        }
    }
    fn flag(value: &Json, key: &str) -> Result<bool, String> {
        match value.get(key) {
            Some(Json::Bool(found)) => Ok(*found),
            _ => Err(format!("vector member {key:?} must be a boolean")),
        }
    }
    fn list(value: &Json, key: &str) -> Result<Vec<Json>, String> {
        match value.get(key) {
            Some(Json::Array(found)) => Ok(found.clone()),
            None => Ok(Vec::new()),
            _ => Err(format!("vector member {key:?} must be an array")),
        }
    }
    fn object(value: &Json, key: &str) -> Result<Json, String> {
        match value.get(key) {
            Some(found @ Json::Object(_)) => Ok(found.clone()),
            _ => Err(format!("vector member {key:?} must be an object")),
        }
    }
    //#endregion 🔖️JsonReaders

    //#region 🔖️FixtureDecoding
    fn hex_of(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("a sample payload must be an even number of hex digits, got {text:?}"));
        }
        (0..text.len()).step_by(2).map(|at| u8::from_str_radix(&text[at..at + 2], 16).map_err(|error| error.to_string())).collect()
    }
    fn hex_string(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    /// 🎬️ The DSL spelling the committed artifact itself uses (`V`/`A`/`S`), so a vector reads the
    /// same way the real file does.
    fn kind_of(name: &str) -> Result<SemioVideoStreamKind, String> {
        match name {
            "V" => Ok(SemioVideoStreamKind::Video),
            "A" => Ok(SemioVideoStreamKind::Audio),
            "S" => Ok(SemioVideoStreamKind::Subtitle),
            other => Err(format!("unknown stream kind {other:?}")),
        }
    }

    fn rational_of(value: &Json) -> Result<SemioRational, String> {
        Ok(SemioRational { num: number(value, "num")? as i64, den: number(value, "den")? as i64 })
    }

    fn sample_of(value: &Json) -> Result<SemioVideoSample, String> {
        Ok(SemioVideoSample { pts: number(value, "pts")? as u64, key: flag(value, "key")?, data: hex_of(&text(value, "data")?)? })
    }

    fn stream_of(value: &Json) -> Result<SemioVideoStream, String> {
        Ok(SemioVideoStream {
            kind: kind_of(&text(value, "kind")?)?,
            codec: text(value, "codec")?,
            width: number(value, "width")? as u32,
            height: number(value, "height")? as u32,
            rate: rational_of(&object(value, "rate")?)?,
            samples: list(value, "samples")?.iter().map(sample_of).collect::<Result<Vec<_>, String>>()?,
        })
    }

    fn snapshot_of(value: &Json) -> Result<SemioVideoSnapshot, String> {
        Ok(SemioVideoSnapshot { schema: text(value, "schema")?, streams: list(value, "streams")?.iter().map(stream_of).collect::<Result<Vec<_>, String>>()? })
    }

    /// 🦠️ The committed vector's `{kind, params}` pair, decoded into the real typed mutation.
    ///
    /// 🧭️ `"no-mutation"` is the dropped `NoMutation` verb's committed spelling (`no` is not an
    /// APPROVED_VERB, so the leaf migration could not keep it as a variant) — it maps to the
    /// identity mutation `SetSnapshot(base.clone())` rather than failing, so the committed
    /// `no-mutation` scenario keeps exercising the "nothing changes" law instead of being deleted.
    fn mutation_of(vector: &Json, base: &SemioVideoSnapshot) -> Result<SemioVideoMutation, String> {
        let kind = text(vector, "kind")?;
        let params = object(vector, "params")?;
        match kind.as_str() {
            "no-mutation" => Ok(SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })),
            "set-snapshot" => Ok(SemioVideoMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_of(&object(&params, "snapshot")?)? })),
            "insert-stream" => Ok(SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: number(&params, "index")? as usize, stream: stream_of(&object(&params, "stream")?)? })),
            "remove-stream" => Ok(SemioVideoMutation::RemoveStream(remove_stream::RemoveStream { index: number(&params, "index")? as usize })),
            "set-stream-meta" => Ok(SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta {
                index: number(&params, "index")? as usize,
                kind: kind_of(&text(&params, "kind")?)?,
                codec: text(&params, "codec")?,
                width: number(&params, "width")? as u32,
                height: number(&params, "height")? as u32,
                rate: rational_of(&object(&params, "rate")?)?,
            })),
            "insert-sample" => Ok(SemioVideoMutation::InsertSample(insert_sample::InsertSample { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize, sample: sample_of(&object(&params, "sample")?)? })),
            "remove-sample" => Ok(SemioVideoMutation::RemoveSample(remove_sample::RemoveSample { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize })),
            "set-sample-data" => Ok(SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize, data: hex_of(&text(&params, "data")?)? })),
            "set-sample-flags" => Ok(SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags {
                stream_index: number(&params, "streamIndex")? as usize,
                index: number(&params, "index")? as usize,
                pts: number(&params, "pts")? as u64,
                key: flag(&params, "key")?,
            })),
            other => Err(format!("mutate-semio-video: no decoder for kind {other:?}")),
        }
    }
    //#endregion 🔖️FixtureDecoding

    //#region 🔖️Projection
    fn kind_name(kind: SemioVideoStreamKind) -> &'static str {
        match kind {
            SemioVideoStreamKind::Video => "V",
            SemioVideoStreamKind::Audio => "A",
            SemioVideoStreamKind::Subtitle => "S",
        }
    }
    fn sample_json(sample: &SemioVideoSample) -> Json {
        Json::Object(vec![
            ("pts".to_string(), Json::Number(sample.pts as f64)),
            ("key".to_string(), Json::Bool(sample.key)),
            ("data".to_string(), Json::String(hex_string(&sample.data))),
        ])
    }
    fn stream_json(stream: &SemioVideoStream) -> Json {
        Json::Object(vec![
            ("kind".to_string(), Json::String(kind_name(stream.kind).to_string())),
            ("codec".to_string(), Json::String(stream.codec.clone())),
            ("width".to_string(), Json::Number(f64::from(stream.width))),
            ("height".to_string(), Json::Number(f64::from(stream.height))),
            ("rate".to_string(), Json::Object(vec![("num".to_string(), Json::Number(stream.rate.num as f64)), ("den".to_string(), Json::Number(stream.rate.den as f64))])),
            ("samples".to_string(), Json::Array(stream.samples.iter().map(sample_json).collect())),
        ])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1` — field for field the
    /// shape the committed vectors are written in.
    fn snapshot_json(snapshot: &SemioVideoSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("streams".to_string(), Json::Array(snapshot.streams.iter().map(stream_json).collect())),
        ])
    }
    fn outcome(projection: Json) -> Outcome {
        Outcome::with_raw(projection.to_string().into_bytes(), projection)
    }
    fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
        ctx.fixture_json(&format!("local://🦠️{kind}.json"))
    }
    /// 🧫️ The committed specification vector, decoded into typed values: the before-snapshot the
    /// kind is applied to, the mutation payload itself, and the after-snapshot the applied result
    /// has to equal. All three come out of the SAME file the oracle role reads literally.
    fn vector_of(ctx: &Context, kind: &str) -> Result<(SemioVideoSnapshot, SemioVideoMutation, SemioVideoSnapshot), String> {
        let vector = vector(ctx, kind)?;
        let before = vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?;
        let after = vector.get("after").ok_or_else(|| "specification vector is missing its \"after\" member".to_string())?;
        let before_snapshot = snapshot_of(before)?;
        let mutation = mutation_of(&vector, &before_snapshot)?;
        Ok((before_snapshot, mutation, snapshot_of(after)?))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioVideoSnapshot, expected: &SemioVideoSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Inputs
    /// 🎬️ The document every mutation row runs on: two real streams of the real "Bauen mit Bestand"
    /// recording — eight real MJPEG frames of the committed AVI and twenty-four real MPEG-1 Layer III
    /// frames of the committed mp3 — derived ONCE by `🐍️derive-video-fixture.py` in the ticket folder.
    const RECORDING_DSL: &str = "local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio";
    const CLIP_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio";

    /// 🎬️ The real recording, parsed by this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<SemioVideoSnapshot, String> {
        let bytes = ctx.fixture_bytes(RECORDING_DSL)?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the recording artifact must be UTF-8: {error}"))?;
        parse_semio_video_dsl(&source)
    }

    /// 🦠️ The verb the scenario declares, read from the feature's own doc string. `base` is only
    /// consulted for the `no-mutation` scenario's identity mapping.
    fn declared(ctx: &Context, base: &SemioVideoSnapshot) -> Result<SemioVideoMutation, String> {
        mutation_of(&ctx.doc_json()?, base)
    }

    fn run(current: &mut SemioVideoSnapshot, mutation: &SemioVideoMutation, what: &str) -> Result<(), String> {
        let applied = apply_semio_video_mutation(current, mutation);
        let refusals = semio_mutation_refusals(&applied);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: mutation rejected: {refusals:?}"))
    }
    //#endregion 🔖️Inputs

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed clip through the production entry point.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = artifact(ctx)?;
        let mutation = declared(ctx, &current)?;
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        Ok(outcome(snapshot_json(&current)))
    }

    /// ↩️ The metamorphic inverse law on the real committed clip: applying the verb and then its
    /// OWN computed inverse must restore the artifact exactly. The MUTATED snapshot travels in the
    /// projection alongside the restored one, so the rows cannot all project the same restored
    /// value and compare vacuously.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = artifact(ctx)?;
        let mutation = declared(ctx, &base)?;
        let mut current = base.clone();
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let mutated = snapshot_json(&current);
        for step in &inverse_semio_video_mutation(&mutation, &base) {
            run(&mut current, step, ctx.scenario.id.as_str())?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the real committed clip", ctx.scenario.id), &current, &base));
        }
        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    /// what the verb means, independent of both implementations, kept from before this oracle
    /// existed rather than replaced by it.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut current, mutation, expected) = vector_of(ctx, kind)?;
            run(&mut current, &mutation, ctx.scenario.id.as_str())?;
            if current != expected {
                return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
            }
            Ok(outcome(snapshot_json(&current)))
        }
    }

    /// 🔁️ The real committed artifact, parsed into the typed snapshot, printed back to DSL text and
    /// parsed again — the only channel from input to output is the model, so nothing of the source
    /// bytes can be smuggled into the projection.
    /// 🔒️ **The byte half of the identity law — asserted as `carrier_is_exact`.** `.dsl.semio` is a
    /// fixed-layout record grammar and the committed artifact was produced by this very printer, so
    /// reproducing it BYTE FOR BYTE is the correct answer here and `law::reparsed_not_copied` would
    /// be exactly backwards. Nor is it a self-comparison: the Python oracle re-emits the same file
    /// from the grammar alone and the two digests are compared. This subset's committed
    /// `.pack.semio` twin is NOT read here — no pack bridge is exported for it — so the byte claim
    /// is about the text carrier alone.
    /// 🔁️ One document, re-emitted from the parsed snapshot and required back byte for byte.
    fn carrier_once(ctx: &Context, uri: &str, what: &str) -> Result<(SemioVideoSnapshot, Json), String> {
        let committed = ctx.fixture_bytes(uri)?;
        let source = String::from_utf8(committed.clone()).map_err(|error| format!("{what} must be UTF-8: {error}"))?;
        let once = parse_semio_video_dsl(&source)?;
        let printed = print_semio_video_dsl(&once);
        carrier_is_exact(printed.as_bytes(), &committed)?;
        let twice = parse_semio_video_dsl(&printed)?;
        if twice != once {
            return Err(disagreement(&format!("identity-round-trip: re-parsing the printed DSL of {what} did not reproduce the parsed snapshot"), &twice, &once));
        }
        let report = Json::Object(vec![
            ("document".to_string(), snapshot_json(&twice)),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("dslLength".to_string(), Json::Number(printed.len() as f64)),
        ]);
        Ok((once, report))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let (clip, clip_report) = carrier_once(ctx, CLIP_DSL, "the committed clip")?;
        let (declared, _mutation, _after) = vector_of(ctx, "no-mutation")?;
        if clip != declared {
            return Err(disagreement("identity-round-trip: the real committed clip artifact does not decode to the before-snapshot every specification vector starts from", &clip, &declared));
        }
        let (recording, recording_report) = carrier_once(ctx, RECORDING_DSL, "the recording")?;
        if recording.streams.len() != 2 || recording.streams[0].samples.len() != 8 || recording.streams[1].samples.len() != 24 {
            return Err(format!(
                "identity-round-trip: the recording is the two-stream 8+24-sample document this case describes, but decoded as {} stream(s)",
                recording.streams.len()
            ));
        }
        Ok(outcome(Json::Object(vec![("clip".to_string(), clip_report), ("recording".to_string(), recording_report)])))
    }

    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the whole vocabulary is registered in one loop. Subject only — the oracle role belongs to
/// `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
