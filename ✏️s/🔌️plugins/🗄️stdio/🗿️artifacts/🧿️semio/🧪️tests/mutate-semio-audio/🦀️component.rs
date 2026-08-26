//! 🦀️ Semio AUDIO exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file registers the SUBJECT role only. The reference answer comes from `🐍️component.py`
//! beside it — an independent Python implementation of the same carrier and the same ten verbs,
//! written from the committed grammars, registered as the oracle `semio-audio-python-independent`
//! in `../../🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧪️oracle/🔣️component.json`. Registering oracle
//! handlers here as well would put this repository's own answer on both sides of the comparison,
//! which is the one failure the platform exists to prevent, so the registrations this file used to
//! carry are gone rather than merely unused.
//!
//! Every scenario drives this repository's own production entry points —
//! `parse_semio_audio_dsl`/`print_semio_audio_dsl` for the carrier and
//! `apply_semio_audio_mutation`/`inverse_semio_audio_mutation` for the vocabulary — over the real
//! committed tone artifact `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/
//! 🗣️example.dsl.semio`, and projects the resulting snapshot as structural JSON for
//! `ordered-json-v1` to compare against the Python side's.
//!
//! The mutation parameters and the specification-vector paths live in `component.feature`, so both
//! implementations read one physical copy of each and cannot drift apart. The laws are asserted
//! here IN ROLE as well as compared: `mutate-` checks nothing beyond the projection, `inverse-`
//! requires the tone back, `spec-vector-` requires the committed after-snapshot, and
//! `identity-round-trip` requires byte-exact re-emission through `law::carrier_is_exact`.
//!
//! The subject half is gated behind the generated host's `sut` feature so a non-subject build never
//! compiles the local implementation.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioAudioMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the registration loop runs in
/// builds where the subject crate is not linked. The contract's mutation-coverage gate keeps this
/// list honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file
/// keeps it honest against the enum.
#[cfg_attr(not(feature = "sut"), allow(dead_code))]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, inverse_semio_audio_mutation, SemioAudioMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{parse_semio_audio_dsl, print_semio_audio_dsl, SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};

    /// 🎤️ The document every mutation row runs on: the first real second of the real committed
    /// "Bauen mit Bestand" recording — 8 000 real 16-bit PCM samples at the file's own 8 000 Hz —
    /// carrying the real ID3v2.3 tags of the same recording's committed mp3, derived ONCE by
    /// `🐍️derive-audio-fixture.py` in the ticket folder.
    const RECORDING_DSL: &str = "local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio";
    const TONE_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio";

    //#region 🔖️JsonReaders
    fn text(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(found)) => Ok(found.clone()),
            _ => Err(format!("member {key:?} must be a string")),
        }
    }
    fn number(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(found)) => Ok(*found),
            _ => Err(format!("member {key:?} must be a number")),
        }
    }
    fn list(value: &Json, key: &str) -> Result<Vec<Json>, String> {
        match value.get(key) {
            Some(Json::Array(found)) => Ok(found.clone()),
            None => Ok(Vec::new()),
            _ => Err(format!("member {key:?} must be an array")),
        }
    }
    fn object(value: &Json, key: &str) -> Result<Json, String> {
        match value.get(key) {
            Some(found @ Json::Object(_)) => Ok(found.clone()),
            _ => Err(format!("member {key:?} must be an object")),
        }
    }
    fn samples(value: &Json) -> Result<Vec<f32>, String> {
        list(value, "samples")?
            .iter()
            .map(|item| match item {
                Json::Number(found) => Ok(*found as f32),
                other => Err(format!("a sample must be a number, got {other:?}")),
            })
            .collect()
    }
    //#endregion 🔖️JsonReaders

    //#region 🔖️Decoding
    /// 🎚️ The DSL spelling the committed artifact itself uses (`format=f32`), so a mutation payload
    /// reads the same way the real file does.
    fn format_of(name: &str) -> Result<SemioAudioFormat, String> {
        match name {
            "pcm8" => Ok(SemioAudioFormat::Pcm8),
            "pcm16" => Ok(SemioAudioFormat::Pcm16),
            "pcm24" => Ok(SemioAudioFormat::Pcm24),
            "pcm32" => Ok(SemioAudioFormat::Pcm32),
            "f32" => Ok(SemioAudioFormat::Float32),
            "f64" => Ok(SemioAudioFormat::Float64),
            other => Err(format!("unknown sample format {other:?}")),
        }
    }

    fn channel_of(value: &Json) -> Result<SemioAudioChannel, String> {
        Ok(SemioAudioChannel { samples: samples(value)? })
    }

    fn tag_of(value: &Json) -> Result<SemioAudioTag, String> {
        Ok(SemioAudioTag { key: text(value, "key")?, value: text(value, "value")? })
    }

    fn snapshot_of(value: &Json) -> Result<SemioAudioSnapshot, String> {
        Ok(SemioAudioSnapshot {
            schema: text(value, "schema")?,
            sample_rate: number(value, "sampleRate")? as u32,
            format: format_of(&text(value, "format")?)?,
            channels: list(value, "channels")?.iter().map(channel_of).collect::<Result<Vec<_>, String>>()?,
            tags: list(value, "tags")?.iter().map(tag_of).collect::<Result<Vec<_>, String>>()?,
        })
    }

    /// 🦠️ A `{kind, params}` payload — the shape the feature writes and the shape the committed
    /// specification vectors carry — decoded into the real typed mutation.
    fn mutation_of(payload: &Json) -> Result<SemioAudioMutation, String> {
        let kind = text(payload, "kind")?;
        let params = object(payload, "params")?;
        match kind.as_str() {
            "no-mutation" => Ok(SemioAudioMutation::NoMutation),
            "set-snapshot" => Ok(SemioAudioMutation::SetSnapshot { snapshot: snapshot_of(&object(&params, "snapshot")?)? }),
            "set-sample-rate" => Ok(SemioAudioMutation::SetSampleRate { sample_rate: number(&params, "sampleRate")? as u32 }),
            "set-format" => Ok(SemioAudioMutation::SetFormat { format: format_of(&text(&params, "format")?)? }),
            "insert-channel" => Ok(SemioAudioMutation::InsertChannel { index: number(&params, "index")? as usize, channel: channel_of(&object(&params, "channel")?)? }),
            "remove-channel" => Ok(SemioAudioMutation::RemoveChannel { index: number(&params, "index")? as usize }),
            "set-channel-samples" => Ok(SemioAudioMutation::SetChannelSamples { index: number(&params, "index")? as usize, samples: samples(&params)? }),
            "insert-tag" => Ok(SemioAudioMutation::InsertTag { index: number(&params, "index")? as usize, tag: tag_of(&object(&params, "tag")?)? }),
            "remove-tag" => Ok(SemioAudioMutation::RemoveTag { index: number(&params, "index")? as usize }),
            "set-tag-value" => Ok(SemioAudioMutation::SetTagValue { index: number(&params, "index")? as usize, value: text(&params, "value")? }),
            other => Err(format!("mutate-semio-audio: no decoder for kind {other:?}")),
        }
    }
    //#endregion 🔖️Decoding

    //#region 🔖️Projection
    fn format_name(format: SemioAudioFormat) -> &'static str {
        match format {
            SemioAudioFormat::Pcm8 => "pcm8",
            SemioAudioFormat::Pcm16 => "pcm16",
            SemioAudioFormat::Pcm24 => "pcm24",
            SemioAudioFormat::Pcm32 => "pcm32",
            SemioAudioFormat::Float32 => "f32",
            SemioAudioFormat::Float64 => "f64",
        }
    }

    /// 🎯️ The projection every scenario compares under `ordered-json-v1` — field for field the
    /// shape the Python implementation emits and the committed vectors are written in.
    fn snapshot_json(snapshot: &SemioAudioSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("sampleRate".to_string(), Json::Number(f64::from(snapshot.sample_rate))),
            ("format".to_string(), Json::String(format_name(snapshot.format).to_string())),
            (
                "channels".to_string(),
                Json::Array(snapshot.channels.iter().map(|channel| Json::Object(vec![("samples".to_string(), Json::Array(channel.samples.iter().map(|sample| Json::Number(f64::from(*sample))).collect()))])).collect()),
            ),
            (
                "tags".to_string(),
                Json::Array(snapshot.tags.iter().map(|tag| Json::Object(vec![("key".to_string(), Json::String(tag.key.clone())), ("value".to_string(), Json::String(tag.value.clone()))])).collect()),
            ),
        ])
    }

    fn outcome(projection: Json) -> Outcome {
        Outcome::with_raw(projection.to_string().into_bytes(), projection)
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON both sides
    /// project, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioAudioSnapshot, expected: &SemioAudioSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Inputs
    /// 🎤️ The real recording, parsed by this repository's own DSL codec.
    fn tone(ctx: &Context) -> Result<SemioAudioSnapshot, String> {
        let bytes = ctx.fixture_bytes(RECORDING_DSL)?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the recording artifact must be UTF-8: {error}"))?;
        parse_semio_audio_dsl(&source)
    }

    /// 🦠️ The verb the scenario declares, read from the feature's own doc string.
    fn declared(ctx: &Context) -> Result<SemioAudioMutation, String> {
        mutation_of(&ctx.doc_json()?)
    }

    fn apply(current: &mut SemioAudioSnapshot, mutation: &SemioAudioMutation, what: &str) -> Result<(), String> {
        let applied = apply_semio_audio_mutation(current, mutation);
        let refusals = semio_mutation_refusals(&applied);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: mutation rejected: {refusals:?}"))
    }
    //#endregion 🔖️Inputs

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed tone through the production entry point.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = tone(ctx)?;
        apply(&mut current, &declared(ctx)?, ctx.scenario.id.as_str())?;
        Ok(outcome(snapshot_json(&current)))
    }

    /// ↩️ The metamorphic inverse law on the real committed tone: applying the verb and then its OWN
    /// computed inverse must restore the artifact exactly. The MUTATED snapshot travels in the
    /// projection alongside the restored one, so the ten rows cannot all project the same restored
    /// value and compare vacuously.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = tone(ctx)?;
        let mutation = declared(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let mutated = snapshot_json(&current);
        for step in &inverse_semio_audio_mutation(&mutation, &base) {
            apply(&mut current, step, ctx.scenario.id.as_str())?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the real committed tone", ctx.scenario.id), &current, &base));
        }
        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    /// what the verb means, independent of both implementations, kept from before this oracle
    /// existed rather than replaced by it.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let vector = ctx.fixture_json(&format!("local://🦠️{kind}.json"))?;
            let expected = snapshot_of(vector.get("after").ok_or_else(|| "specification vector is missing its \"after\" member".to_string())?)?;
            let mut current = snapshot_of(vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?)?;
            apply(&mut current, &mutation_of(&vector)?, ctx.scenario.id.as_str())?;
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
    /// `.pack.semio` twin is NOT read here — no `encode`/`decode_semio_audio_pack` bridge is
    /// exported for it — so the byte claim is about the text carrier alone.
    /// 🔁️ One document, re-emitted from the parsed snapshot and required back byte for byte.
    fn carrier_once(ctx: &Context, uri: &str, what: &str) -> Result<(SemioAudioSnapshot, Json), String> {
        let committed = ctx.fixture_bytes(uri)?;
        let source = String::from_utf8(committed.clone()).map_err(|error| format!("{what} must be UTF-8: {error}"))?;
        let once = parse_semio_audio_dsl(&source)?;
        let printed = print_semio_audio_dsl(&once);
        carrier_is_exact(printed.as_bytes(), &committed)?;
        let twice = parse_semio_audio_dsl(&printed)?;
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
        let (tone, tone_report) = carrier_once(ctx, TONE_DSL, "the committed tone")?;
        let vector = ctx.fixture_json("local://🦠️no-mutation.json")?;
        let declared = snapshot_of(vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?)?;
        if tone != declared {
            return Err(disagreement("identity-round-trip: the real committed tone artifact does not decode to the before-snapshot every specification vector starts from", &tone, &declared));
        }
        let (recording, recording_report) = carrier_once(ctx, RECORDING_DSL, "the recording")?;
        let samples: usize = recording.channels.iter().map(|channel| channel.samples.len()).sum();
        if recording.sample_rate != 8000 || recording.channels.len() != 1 || samples != 8000 || recording.tags.len() != 4 {
            return Err(format!(
                "identity-round-trip: the recording is the 8 000 Hz one-channel 8 000-sample four-tag document this case describes, but decoded as {} Hz, {} channel(s), {samples} sample(s) and {} tag(s)",
                recording.sample_rate,
                recording.channels.len(),
                recording.tags.len()
            ));
        }
        Ok(outcome(Json::Object(vec![("tone".to_string(), tone_report), ("recording".to_string(), recording_report)])))
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
