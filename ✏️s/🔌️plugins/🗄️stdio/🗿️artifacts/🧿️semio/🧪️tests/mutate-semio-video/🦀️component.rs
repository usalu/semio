//! 🦀️ Semio VIDEO exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-video-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️video/🧪️oracle/🔣️component.json`): `s.stdio.semio.video` is a semio-NATIVE
//! format with no third-party reader or writer in any ecosystem, so `oracle` here reads the
//! committed per-kind specification vectors in `🧫️fixtures/` literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` decodes the SAME committed bytes into real
//! `SemioVideoSnapshot`/`SemioVideoMutation` values and drives this repository's own
//! `apply_semio_video_mutation`/`inverse_semio_video_mutation` over the full 9-kind vocabulary.
//! Both sides project to structural JSON and `ordered-json-v1` compares them.
//!
//! Every vector's BEFORE snapshot is the decoded content of this standard's own committed real
//! artifact `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio`
//! — an h264 video track of two samples beside an aac audio track — and the identity round trip
//! parses that very file through the subset's own DSL codec, so the real artifact is in the loop
//! rather than described. A sample's opaque payload is carried as the same lowercase hex the DSL
//! itself writes, so a vector and the real file read alike.
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
/// 🏷️ Mirrors `SemioVideoMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-stream", "remove-stream", "set-stream-meta", "insert-sample", "remove-sample", "set-sample-data", "set-sample-flags"];
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

/// 🔮️ The identity reference answer: what the real committed clip artifact decodes to, which is the
/// BEFORE snapshot every other vector starts from.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    Ok(outcome(member(&vector(ctx, "no-mutation")?, "before")?))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::video::schema::mutations::{apply_semio_video_mutation, inverse_semio_video_mutation, SemioVideoMutation};
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
    fn mutation_of(vector: &Json) -> Result<SemioVideoMutation, String> {
        let kind = text(vector, "kind")?;
        let params = object(vector, "params")?;
        match kind.as_str() {
            "no-mutation" => Ok(SemioVideoMutation::NoMutation),
            "set-snapshot" => Ok(SemioVideoMutation::SetSnapshot { snapshot: snapshot_of(&object(&params, "snapshot")?)? }),
            "insert-stream" => Ok(SemioVideoMutation::InsertStream { index: number(&params, "index")? as usize, stream: stream_of(&object(&params, "stream")?)? }),
            "remove-stream" => Ok(SemioVideoMutation::RemoveStream { index: number(&params, "index")? as usize }),
            "set-stream-meta" => Ok(SemioVideoMutation::SetStreamMeta {
                index: number(&params, "index")? as usize,
                kind: kind_of(&text(&params, "kind")?)?,
                codec: text(&params, "codec")?,
                width: number(&params, "width")? as u32,
                height: number(&params, "height")? as u32,
                rate: rational_of(&object(&params, "rate")?)?,
            }),
            "insert-sample" => Ok(SemioVideoMutation::InsertSample { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize, sample: sample_of(&object(&params, "sample")?)? }),
            "remove-sample" => Ok(SemioVideoMutation::RemoveSample { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize }),
            "set-sample-data" => Ok(SemioVideoMutation::SetSampleData { stream_index: number(&params, "streamIndex")? as usize, index: number(&params, "index")? as usize, data: hex_of(&text(&params, "data")?)? }),
            "set-sample-flags" => Ok(SemioVideoMutation::SetSampleFlags {
                stream_index: number(&params, "streamIndex")? as usize,
                index: number(&params, "index")? as usize,
                pts: number(&params, "pts")? as u64,
                key: flag(&params, "key")?,
            }),
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
        Ok((snapshot_of(before)?, mutation_of(&vector)?, snapshot_of(after)?))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioVideoSnapshot, expected: &SemioVideoSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot — frame geometry, timebase and the track/clip arrangement together, so a retimed clip
    /// that landed on the right track at the wrong offset still fails. The assertion lives here rather than in the
    /// comparison because a recorded no-oracle case runs no oracle role: a handler that merely
    /// returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut current, mutation, expected) = vector_of(ctx, kind)?;
            let applied = apply_semio_video_mutation(&mut current, &mutation);
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
    /// restore the committed before-snapshot exactly — the source offset a moved clip carried included, not merely its track membership.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation, _expected) = vector_of(ctx, kind)?;
            let mut current = base.clone();
            let applied = apply_semio_video_mutation(&mut current, &mutation);
            if !applied.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", applied.messages()));
            }
            for step in &inverse_semio_video_mutation(&mutation, &base) {
                let undone = apply_semio_video_mutation(&mut current, step);
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
        let bytes = ctx.fixture_bytes("asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio")?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the committed clip artifact must be UTF-8: {error}"))?;
        let once = parse_semio_video_dsl(&source)?;
        let printed = print_semio_video_dsl(&once);
        let twice = parse_semio_video_dsl(&printed)?;
        if twice != once {
            return Err(disagreement("identity-round-trip: re-parsing the printed DSL did not reproduce the parsed snapshot", &twice, &once));
        }
        let (declared, _mutation, _after) = vector_of(ctx, "no-mutation")?;
        if once != declared {
            return Err(disagreement("identity-round-trip: the real committed clip artifact does not decode to the before-snapshot every specification vector starts from — the vectors describe a document this codec does not produce", &once, &declared));
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
