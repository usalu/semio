//! 🦀️ Semio ANIMATION exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file registers the SUBJECT role only. The reference answer comes from `🐍️component.py`
//! beside it — an independent Python implementation of the same carrier and the same
//! thirteen verbs, written from the committed grammars, registered as the oracle
//! `semio-animation-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🔮️oracle/🔣️.json`. Registering oracle
//! handlers here as well would put this repository's own answer on both sides of the comparison,
//! which is the one failure the platform exists to prevent, so the registrations this file used to
//! carry are gone rather than merely unused.
//!
//! Every scenario drives this repository's own production entry points — `parse_semio_animation_dsl`/`print_semio_animation_dsl` for
//! the carrier and `apply_semio_animation_mutation`/`inverse_semio_animation_mutation` for the vocabulary — over the real committed walk
//! artifact `../../🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio`, and projects the resulting snapshot as structural JSON for
//! `ordered-json-v1` to compare against the Python side's.
//!
//! The mutation parameters and the specification-vector paths live in `component.feature`, so both
//! implementations read one physical copy of each and cannot drift apart. The laws are asserted
//! here IN ROLE as well as compared: `inverse-` requires the walk back, `spec-vector-` requires
//! the committed after-snapshot, and `identity-round-trip` requires byte-exact re-emission through
//! `law::carrier_is_exact`.
//!
//! The subject half is gated behind the generated host's `sut` feature so a non-subject build never
//! compiles the local implementation.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioAnimationMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the registration loop runs in
/// builds where the subject crate is not linked. The contract's mutation-coverage gate keeps this
/// list honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file
/// keeps it honest against the enum.
#[cfg_attr(not(feature = "sut"), allow(dead_code))]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-timeline", "remove-timeline", "set-timeline-name", "insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation", "insert-keyframe", "remove-keyframe", "set-keyframe-time", "set-keyframe-value"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{
        apply_semio_animation_mutation, insert_channel, insert_keyframe, insert_timeline, inverse_semio_animation_mutation, remove_channel, remove_keyframe, remove_timeline, set_channel_interpolation, set_channel_target, set_keyframe_time,
        set_keyframe_value, set_snapshot, set_timeline_name, SemioAnimationMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{parse_semio_animation_dsl, print_semio_animation_dsl, AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};

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
    /// 🏷️ `null` is a value here, not an absence: a timeline's display name is genuinely optional.
    fn optional_text(value: &Json, key: &str) -> Result<Option<String>, String> {
        match value.get(key) {
            Some(Json::String(found)) => Ok(Some(found.clone())),
            Some(Json::Null) | None => Ok(None),
            _ => Err(format!("vector member {key:?} must be a string or null")),
        }
    }
    fn numbers(value: &Json, key: &str) -> Result<Vec<f64>, String> {
        list(value, key)?
            .iter()
            .map(|item| match item {
                Json::Number(found) => Ok(*found),
                other => Err(format!("member {key:?} must hold numbers, got {other:?}")),
            })
            .collect()
    }
    //#endregion 🔖️JsonReaders

    //#region 🔖️FixtureDecoding
    fn property_of(value: &Json) -> Result<AnimTargetProperty, String> {
        match text(value, "kind")?.as_str() {
            "translation" => Ok(AnimTargetProperty::Translation),
            "rotation" => Ok(AnimTargetProperty::Rotation),
            "scale" => Ok(AnimTargetProperty::Scale),
            "weights" => Ok(AnimTargetProperty::Weights),
            "custom" => Ok(AnimTargetProperty::Custom { name: text(value, "name")? }),
            other => Err(format!("unknown target property {other:?}")),
        }
    }
    fn target_of(value: &Json) -> Result<AnimTarget, String> {
        Ok(AnimTarget { node: text(value, "node")?, property: property_of(&object(value, "property")?)? })
    }
    fn interpolation_of(name: &str) -> Result<AnimInterpolation, String> {
        match name {
            "linear" => Ok(AnimInterpolation::Linear),
            "step" => Ok(AnimInterpolation::Step),
            "cubicSpline" => Ok(AnimInterpolation::CubicSpline),
            other => Err(format!("unknown interpolation {other:?}")),
        }
    }
    fn value_of(value: &Json) -> Result<AnimValue, String> {
        match text(value, "kind")?.as_str() {
            "scalar" => Ok(AnimValue::Scalar { value: number(value, "value")? }),
            "vec3" => {
                let point = object(value, "value")?;
                Ok(AnimValue::Vec3 { value: SemioPoint3 { x: number(&point, "x")?, y: number(&point, "y")?, z: number(&point, "z")? } })
            }
            "quat" => {
                let rotation = object(value, "value")?;
                Ok(AnimValue::Quat { value: SemioQuaternion { x: number(&rotation, "x")?, y: number(&rotation, "y")?, z: number(&rotation, "z")?, w: number(&rotation, "w")? } })
            }
            "weights" => Ok(AnimValue::Weights { values: numbers(value, "values")? }),
            other => Err(format!("unknown keyframe value kind {other:?}")),
        }
    }
    fn keyframe_of(value: &Json) -> Result<AnimKeyframe, String> {
        Ok(AnimKeyframe { t: number(value, "t")?, value: value_of(&object(value, "value")?)? })
    }
    fn channel_of(value: &Json) -> Result<AnimChannel, String> {
        Ok(AnimChannel {
            target: target_of(&object(value, "target")?)?,
            interpolation: interpolation_of(&text(value, "interpolation")?)?,
            keyframes: list(value, "keyframes")?.iter().map(keyframe_of).collect::<Result<Vec<_>, String>>()?,
        })
    }
    fn timeline_of(value: &Json) -> Result<AnimTimeline, String> {
        Ok(AnimTimeline { name: optional_text(value, "name")?, channels: list(value, "channels")?.iter().map(channel_of).collect::<Result<Vec<_>, String>>()? })
    }
    fn snapshot_of(value: &Json) -> Result<SemioAnimationSnapshot, String> {
        Ok(SemioAnimationSnapshot { schema: text(value, "schema")?, timelines: list(value, "timelines")?.iter().map(timeline_of).collect::<Result<Vec<_>, String>>()? })
    }

    /// 🦠️ The committed vector's `{kind, params}` pair, decoded into the real typed mutation.
    fn mutation_of(vector: &Json) -> Result<SemioAnimationMutation, String> {
        let kind = text(vector, "kind")?;
        let params = object(vector, "params")?;
        let index = |key: &str| -> Result<usize, String> { Ok(number(&params, key)? as usize) };
        match kind.as_str() {
            "no-mutation" => Ok(SemioAnimationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_of(&object(vector, "before")?)? })),
            "set-snapshot" => Ok(SemioAnimationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot_of(&object(&params, "snapshot")?)? })),
            "insert-timeline" => Ok(SemioAnimationMutation::InsertTimeline(insert_timeline::InsertTimeline { index: index("index")?, timeline: timeline_of(&object(&params, "timeline")?)? })),
            "remove-timeline" => Ok(SemioAnimationMutation::RemoveTimeline(remove_timeline::RemoveTimeline { index: index("index")? })),
            "set-timeline-name" => Ok(SemioAnimationMutation::SetTimelineName(set_timeline_name::SetTimelineName { index: index("index")?, name: optional_text(&params, "name")? })),
            "insert-channel" => Ok(SemioAnimationMutation::InsertChannel(insert_channel::InsertChannel { timeline_index: index("timelineIndex")?, index: index("index")?, channel: channel_of(&object(&params, "channel")?)? })),
            "remove-channel" => Ok(SemioAnimationMutation::RemoveChannel(remove_channel::RemoveChannel { timeline_index: index("timelineIndex")?, index: index("index")? })),
            "set-channel-target" => Ok(SemioAnimationMutation::SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: index("timelineIndex")?, index: index("index")?, target: target_of(&object(&params, "target")?)? })),
            "set-channel-interpolation" => {
                Ok(SemioAnimationMutation::SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: index("timelineIndex")?, index: index("index")?, interpolation: interpolation_of(&text(&params, "interpolation")?)? }))
            }
            "insert-keyframe" => Ok(SemioAnimationMutation::InsertKeyframe(insert_keyframe::InsertKeyframe {
                timeline_index: index("timelineIndex")?,
                channel_index: index("channelIndex")?,
                index: index("index")?,
                keyframe: keyframe_of(&object(&params, "keyframe")?)?,
            })),
            "remove-keyframe" => Ok(SemioAnimationMutation::RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: index("timelineIndex")?, channel_index: index("channelIndex")?, index: index("index")? })),
            "set-keyframe-time" => Ok(SemioAnimationMutation::SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: index("timelineIndex")?, channel_index: index("channelIndex")?, index: index("index")?, t: number(&params, "t")? })),
            "set-keyframe-value" => Ok(SemioAnimationMutation::SetKeyframeValue(set_keyframe_value::SetKeyframeValue {
                timeline_index: index("timelineIndex")?,
                channel_index: index("channelIndex")?,
                index: index("index")?,
                value: value_of(&object(&params, "value")?)?,
            })),
            other => Err(format!("mutate-semio-animation: no decoder for kind {other:?}")),
        }
    }
    //#endregion 🔖️FixtureDecoding

    //#region 🔖️Projection
    fn property_json(property: &AnimTargetProperty) -> Json {
        match property {
            AnimTargetProperty::Translation => Json::Object(vec![("kind".to_string(), Json::String("translation".to_string()))]),
            AnimTargetProperty::Rotation => Json::Object(vec![("kind".to_string(), Json::String("rotation".to_string()))]),
            AnimTargetProperty::Scale => Json::Object(vec![("kind".to_string(), Json::String("scale".to_string()))]),
            AnimTargetProperty::Weights => Json::Object(vec![("kind".to_string(), Json::String("weights".to_string()))]),
            AnimTargetProperty::Custom { name } => Json::Object(vec![("kind".to_string(), Json::String("custom".to_string())), ("name".to_string(), Json::String(name.clone()))]),
        }
    }
    fn target_json(target: &AnimTarget) -> Json {
        Json::Object(vec![("node".to_string(), Json::String(target.node.clone())), ("property".to_string(), property_json(&target.property))])
    }
    fn interpolation_name(interpolation: AnimInterpolation) -> &'static str {
        match interpolation {
            AnimInterpolation::Linear => "linear",
            AnimInterpolation::Step => "step",
            AnimInterpolation::CubicSpline => "cubicSpline",
        }
    }
    fn value_json(value: &AnimValue) -> Json {
        match value {
            AnimValue::Scalar { value } => Json::Object(vec![("kind".to_string(), Json::String("scalar".to_string())), ("value".to_string(), Json::Number(*value))]),
            AnimValue::Vec3 { value } => Json::Object(vec![
                ("kind".to_string(), Json::String("vec3".to_string())),
                ("value".to_string(), Json::Object(vec![("x".to_string(), Json::Number(value.x)), ("y".to_string(), Json::Number(value.y)), ("z".to_string(), Json::Number(value.z))])),
            ]),
            AnimValue::Quat { value } => Json::Object(vec![
                ("kind".to_string(), Json::String("quat".to_string())),
                (
                    "value".to_string(),
                    Json::Object(vec![("x".to_string(), Json::Number(value.x)), ("y".to_string(), Json::Number(value.y)), ("z".to_string(), Json::Number(value.z)), ("w".to_string(), Json::Number(value.w))]),
                ),
            ]),
            AnimValue::Weights { values } => {
                Json::Object(vec![("kind".to_string(), Json::String("weights".to_string())), ("values".to_string(), Json::Array(values.iter().map(|weight| Json::Number(*weight)).collect()))])
            }
        }
    }
    fn keyframe_json(keyframe: &AnimKeyframe) -> Json {
        Json::Object(vec![("t".to_string(), Json::Number(keyframe.t)), ("value".to_string(), value_json(&keyframe.value))])
    }
    fn channel_json(channel: &AnimChannel) -> Json {
        Json::Object(vec![
            ("target".to_string(), target_json(&channel.target)),
            ("interpolation".to_string(), Json::String(interpolation_name(channel.interpolation).to_string())),
            ("keyframes".to_string(), Json::Array(channel.keyframes.iter().map(keyframe_json).collect())),
        ])
    }
    fn timeline_json(timeline: &AnimTimeline) -> Json {
        Json::Object(vec![
            ("name".to_string(), timeline.name.as_ref().map(|name| Json::String(name.clone())).unwrap_or(Json::Null)),
            ("channels".to_string(), Json::Array(timeline.channels.iter().map(channel_json).collect())),
        ])
    }
    /// 🎯️ The projection every scenario compares under `ordered-json-v1` — field for field the
    /// shape the committed vectors are written in.
    fn snapshot_json(snapshot: &SemioAnimationSnapshot) -> Json {
        Json::Object(vec![
            ("schema".to_string(), Json::String(snapshot.schema.clone())),
            ("timelines".to_string(), Json::Array(snapshot.timelines.iter().map(timeline_json).collect())),
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
    fn vector_of(ctx: &Context, kind: &str) -> Result<(SemioAnimationSnapshot, SemioAnimationMutation, SemioAnimationSnapshot), String> {
        let vector = vector(ctx, kind)?;
        let before = vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?;
        let after = vector.get("after").ok_or_else(|| "specification vector is missing its \"after\" member".to_string())?;
        Ok((snapshot_of(before)?, mutation_of(&vector)?, snapshot_of(after)?))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same structural JSON the committed
    /// vectors are written in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioAnimationSnapshot, expected: &SemioAnimationSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", snapshot_json(got).to_string(), snapshot_json(expected).to_string())
    }
    //#endregion 🔖️Projection

    //#region 🔖️Inputs
    const WALK_DSL: &str = "asset://📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio";

    /// 🚶️ The real committed walk artifact, parsed by this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<SemioAnimationSnapshot, String> {
        let bytes = ctx.fixture_bytes(WALK_DSL)?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the committed walk artifact must be UTF-8: {error}"))?;
        parse_semio_animation_dsl(&source)
    }

    /// 🦠️ The verb the scenario declares, read from the feature's own doc string.
    fn declared(ctx: &Context) -> Result<SemioAnimationMutation, String> {
        mutation_of(&ctx.doc_json()?)
    }

    fn run(current: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation, what: &str) -> Result<(), String> {
        let applied = apply_semio_animation_mutation(current, mutation);
        let refusals = semio_mutation_refusals(&applied);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: mutation rejected: {refusals:?}"))
    }
    //#endregion 🔖️Inputs

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed walk through the production entry point.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = artifact(ctx)?;
        run(&mut current, &declared(ctx)?, ctx.scenario.id.as_str())?;
        Ok(outcome(snapshot_json(&current)))
    }

    /// ↩️ The metamorphic inverse law on the real committed walk: applying the verb and then its
    /// OWN computed inverse must restore the artifact exactly. The MUTATED snapshot travels in the
    /// projection alongside the restored one, so the rows cannot all project the same restored
    /// value and compare vacuously.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = artifact(ctx)?;
        let mutation = declared(ctx)?;
        let mut current = base.clone();
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let mutated = snapshot_json(&current);
        for step in &inverse_semio_animation_mutation(&mutation, &base) {
            run(&mut current, step, ctx.scenario.id.as_str())?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the real committed walk", ctx.scenario.id), &current, &base));
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
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_bytes(WALK_DSL)?;
        let source = String::from_utf8(committed.clone()).map_err(|error| format!("the committed walk artifact must be UTF-8: {error}"))?;
        let once = parse_semio_animation_dsl(&source)?;
        let printed = print_semio_animation_dsl(&once);
        carrier_is_exact(printed.as_bytes(), &committed)?;
        let twice = parse_semio_animation_dsl(&printed)?;
        if twice != once {
            return Err(disagreement("identity-round-trip: re-parsing the printed DSL did not reproduce the parsed snapshot", &twice, &once));
        }
        let (declared, _mutation, _after) = vector_of(ctx, "no-mutation")?;
        if once != declared {
            return Err(disagreement("identity-round-trip: the real committed walk artifact does not decode to the before-snapshot every specification vector starts from", &once, &declared));
        }
        Ok(outcome(Json::Object(vec![
            ("document".to_string(), snapshot_json(&twice)),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("dslLength".to_string(), Json::Number(printed.len() as f64)),
        ])))
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
