//! 🦀️ Semio AUDIO exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-audio-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️audio/🧪️oracle/🔣️component.json`): `s.stdio.semio.audio` is a semio-NATIVE
//! format with no third-party reader or writer in any ecosystem, so `oracle` here reads the
//! committed per-kind specification vectors in `🧫️fixtures/` literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` decodes the SAME committed bytes into real
//! `SemioAudioSnapshot`/`SemioAudioMutation` values and drives this repository's own
//! `apply_semio_audio_mutation`/`inverse_semio_audio_mutation` over the full 10-kind vocabulary.
//! Both sides project to structural JSON and `ordered-json-v1` compares them.
//!
//! Every vector's BEFORE snapshot is the decoded content of this standard's own committed real
//! artifact `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio`
//! — 44.1 kHz stereo `f32`, one `title` tag — and the identity round trip parses that very file
//! through the subset's own DSL codec, so the real artifact is in the loop rather than described.
//! Because both roles read the fixture through `Context::fixture_json` rather than transcribing it
//! into Rust literals, there is exactly one physical copy of each vector and no way for the two
//! halves to drift apart.
//!
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation; the Rust SUBJECT phase is blocked this wave by a concurrent
//! os-kernel refactor (see 📓️w7-fleet-brief.md), so it is written and gated but not run.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never receives two sides to compare and the `oracle` handlers below are
//! the written statement of the reference answer rather than a second running party. Every law this
//! case claims is therefore asserted INSIDE the subject handler, which fails with both documents
//! printed. A handler that merely ran the mutation and returned would report a pass having checked
//! nothing.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioAudioMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value"];
//#endregion 🔖️Kinds

//#region 🔖️Vectors
/// 🧫️ The committed `(before, mutation, after)` specification vector for one kind, read through the
/// plan so the URI the feature declares is the only way in.
fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
    ctx.fixture_json(&format!("local://🦠️{kind}.json"))
}

/// 🔎️ One required member of a vector — an absent member is an error, never a silent default.
fn member(vector: &Json, key: &str) -> Result<Json, String> {
    vector.get(key).cloned().ok_or_else(|| format!("specification vector is missing its {key:?} member"))
}

/// 🎯️ Both roles emit the same shape: the projection, plus its canonical bytes.
fn outcome(projection: Json) -> Outcome {
    Outcome::with_raw(projection.to_string().into_bytes(), projection)
}
//#endregion 🔖️Vectors

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| Ok(outcome(member(&vector(ctx, kind)?, "after")?))
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| Ok(outcome(member(&vector(ctx, kind)?, "before")?))
}

/// 🔮️ The identity reference answer: what the real committed tone artifact decodes to, which is the
/// BEFORE snapshot every other vector starts from.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    Ok(outcome(member(&vector(ctx, "no-mutation")?, "before")?))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::audio::schema::mutations::{apply_semio_audio_mutation, inverse_semio_audio_mutation, SemioAudioMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{parse_semio_audio_dsl, print_semio_audio_dsl, SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};

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

    //#region 🔖️FixtureDecoding
    /// 🎚️ The DSL spelling the committed artifact itself uses (`format=f32`), so a vector reads the
    /// same way the real file does.
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

    /// 🦠️ The committed vector's `{kind, params}` pair, decoded into the real typed mutation.
    fn mutation_of(vector: &Json) -> Result<SemioAudioMutation, String> {
        let kind = text(vector, "kind")?;
        let params = object(vector, "params")?;
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
    //#endregion 🔖️FixtureDecoding

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
    /// shape the committed vectors are written in.
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
    fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
        ctx.fixture_json(&format!("local://🦠️{kind}.json"))
    }
    /// 🧫️ The committed specification vector, decoded into typed values: the before-snapshot the
    /// kind is applied to, the mutation payload itself, and the after-snapshot the applied result
    /// has to equal. All three come out of the SAME file the oracle role reads literally.
    fn vector_of(ctx: &Context, kind: &str) -> Result<(SemioAudioSnapshot, SemioAudioMutation, SemioAudioSnapshot), String> {
        let vector = vector(ctx, kind)?;
        let before = vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?;
        let after = vector.get("after").ok_or_else(|| "specification vector is missing its \"after\" member".to_string())?;
        Ok((snapshot_of(before)?, mutation_of(&vector)?, snapshot_of(after)?))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioAudioSnapshot, expected: &SemioAudioSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — sample rate, channel layout and the per-track sample payload together, so an edit
    /// that reached the right track at the wrong rate still fails. The assertion lives here rather than in the
    /// comparison because a recorded no-oracle case runs no oracle role: a handler that merely
    /// returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut current, mutation, expected) = vector_of(ctx, kind)?;
            let applied = apply_semio_audio_mutation(&mut current, &mutation);
            if !applied.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", applied.messages()));
            }
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the vector's after-snapshot"), &current, &expected));
            }
            Ok(outcome(snapshot_json(&current)))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly — the sample payload a track edit overwrote included, not merely the track's presence.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation, _expected) = vector_of(ctx, kind)?;
            let mut current = base.clone();
            let applied = apply_semio_audio_mutation(&mut current, &mutation);
            if !applied.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", applied.messages()));
            }
            for step in &inverse_semio_audio_mutation(&mutation, &base) {
                let undone = apply_semio_audio_mutation(&mut current, step);
                if !undone.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", undone.messages()));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the vector's before-snapshot"), &current, &base));
            }
            Ok(outcome(snapshot_json(&current)))
        }
    }

    /// 🔁️ The real committed artifact, parsed into the typed snapshot, printed back to DSL text and
    /// parsed again — the only channel from input to output is the model, so nothing of the source
    /// bytes can be smuggled into the projection.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.fixture_bytes("asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio")?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the committed tone artifact must be UTF-8: {error}"))?;
        let once = parse_semio_audio_dsl(&source)?;
        let printed = print_semio_audio_dsl(&once);
        let twice = parse_semio_audio_dsl(&printed)?;
        if twice != once {
            return Err(disagreement("identity-round-trip: re-parsing the printed DSL did not reproduce the parsed snapshot", &twice, &once));
        }
        let (declared, _mutation, _after) = vector_of(ctx, "no-mutation")?;
        if once != declared {
            return Err(disagreement("identity-round-trip: the real committed tone artifact does not decode to the before-snapshot every specification vector starts from — the vectors describe a document this codec does not produce", &once, &declared));
        }
        Ok(outcome(snapshot_json(&twice)))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so both roles are registered in one loop over the declared vocabulary.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
