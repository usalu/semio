//! 🦀️ Semio ANIMATION exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-animation-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️animation/🧪️oracle/🔣️component.json`): `s.stdio.semio.animation` is a
//! semio-NATIVE format with no third-party reader or writer in any ecosystem, so `oracle` here
//! reads the committed per-kind specification vectors in `🧫️fixtures/` literally — no
//! recomputation, no reimplementation of mutation semantics. `subject` decodes the SAME committed
//! bytes into real `SemioAnimationSnapshot`/`SemioAnimationMutation` values and drives this
//! repository's own `apply_semio_animation_mutation`/`inverse_semio_animation_mutation` over the
//! full 13-kind vocabulary. Both sides project to structural JSON and `ordered-json-v1` compares
//! them.
//!
//! Every vector's BEFORE snapshot is the decoded content of this standard's own committed real
//! artifact `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio`
//! — one named timeline whose four channels between them cover every `AnimTargetProperty`, every
//! `AnimInterpolation` and every `AnimValue` shape — and the identity round trip parses that very
//! file through the subset's own DSL codec, so the real artifact is in the loop rather than
//! described.
//!
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation; the Rust SUBJECT phase is blocked this wave by a concurrent
//! os-kernel refactor (see 📓️w7-fleet-brief.md), so it is written and gated but not run.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioAnimationMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️animation/
/// 🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate. The contract's mutation-coverage gate keeps this list honest
/// against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it
/// honest against the enum.
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-timeline",
    "remove-timeline",
    "set-timeline-name",
    "insert-channel",
    "remove-channel",
    "set-channel-target",
    "set-channel-interpolation",
    "insert-keyframe",
    "remove-keyframe",
    "set-keyframe-time",
    "set-keyframe-value",
];
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

/// 🔮️ The identity reference answer: what the real committed walk artifact decodes to, which is the
/// BEFORE snapshot every other vector starts from.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    Ok(outcome(member(&vector(ctx, "no-mutation")?, "before")?))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{apply_semio_animation_mutation, inverse_semio_animation_mutation, SemioAnimationMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{parse_semio_animation_dsl, print_semio_animation_dsl, AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};

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
            "no-mutation" => Ok(SemioAnimationMutation::NoMutation),
            "set-snapshot" => Ok(SemioAnimationMutation::SetSnapshot { snapshot: snapshot_of(&object(&params, "snapshot")?)? }),
            "insert-timeline" => Ok(SemioAnimationMutation::InsertTimeline { index: index("index")?, timeline: timeline_of(&object(&params, "timeline")?)? }),
            "remove-timeline" => Ok(SemioAnimationMutation::RemoveTimeline { index: index("index")? }),
            "set-timeline-name" => Ok(SemioAnimationMutation::SetTimelineName { index: index("index")?, name: optional_text(&params, "name")? }),
            "insert-channel" => Ok(SemioAnimationMutation::InsertChannel { timeline_index: index("timelineIndex")?, index: index("index")?, channel: channel_of(&object(&params, "channel")?)? }),
            "remove-channel" => Ok(SemioAnimationMutation::RemoveChannel { timeline_index: index("timelineIndex")?, index: index("index")? }),
            "set-channel-target" => Ok(SemioAnimationMutation::SetChannelTarget { timeline_index: index("timelineIndex")?, index: index("index")?, target: target_of(&object(&params, "target")?)? }),
            "set-channel-interpolation" => {
                Ok(SemioAnimationMutation::SetChannelInterpolation { timeline_index: index("timelineIndex")?, index: index("index")?, interpolation: interpolation_of(&text(&params, "interpolation")?)? })
            }
            "insert-keyframe" => Ok(SemioAnimationMutation::InsertKeyframe {
                timeline_index: index("timelineIndex")?,
                channel_index: index("channelIndex")?,
                index: index("index")?,
                keyframe: keyframe_of(&object(&params, "keyframe")?)?,
            }),
            "remove-keyframe" => Ok(SemioAnimationMutation::RemoveKeyframe { timeline_index: index("timelineIndex")?, channel_index: index("channelIndex")?, index: index("index")? }),
            "set-keyframe-time" => Ok(SemioAnimationMutation::SetKeyframeTime { timeline_index: index("timelineIndex")?, channel_index: index("channelIndex")?, index: index("index")?, t: number(&params, "t")? }),
            "set-keyframe-value" => Ok(SemioAnimationMutation::SetKeyframeValue {
                timeline_index: index("timelineIndex")?,
                channel_index: index("channelIndex")?,
                index: index("index")?,
                value: value_of(&object(&params, "value")?)?,
            }),
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
    fn before_and_mutation(ctx: &Context, kind: &str) -> Result<(SemioAnimationSnapshot, SemioAnimationMutation), String> {
        let vector = vector(ctx, kind)?;
        let before = vector.get("before").ok_or_else(|| "specification vector is missing its \"before\" member".to_string())?;
        Ok((snapshot_of(before)?, mutation_of(&vector)?))
    }
    //#endregion 🔖️Projection

    //#region 🔖️Handlers
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (mut current, mutation) = before_and_mutation(ctx, kind)?;
            let applied = apply_semio_animation_mutation(&mut current, &mutation);
            if !applied.messages().is_empty() {
                return Err(format!("mutate-{kind}: mutation rejected: {:?}", applied.messages()));
            }
            Ok(outcome(snapshot_json(&current)))
        }
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation) = before_and_mutation(ctx, kind)?;
            let mut current = base.clone();
            let applied = apply_semio_animation_mutation(&mut current, &mutation);
            if !applied.messages().is_empty() {
                return Err(format!("inverse-{kind}: forward mutation rejected: {:?}", applied.messages()));
            }
            for step in &inverse_semio_animation_mutation(&mutation, &base) {
                let undone = apply_semio_animation_mutation(&mut current, step);
                if !undone.messages().is_empty() {
                    return Err(format!("inverse-{kind}: inverse step rejected: {:?}", undone.messages()));
                }
            }
            Ok(outcome(snapshot_json(&current)))
        }
    }

    /// 🔁️ The real committed artifact, parsed into the typed snapshot, printed back to DSL text and
    /// parsed again — the only channel from input to output is the model, so nothing of the source
    /// bytes can be smuggled into the projection.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let bytes = ctx.fixture_bytes("asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio")?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the committed walk artifact must be UTF-8: {error}"))?;
        let once = parse_semio_animation_dsl(&source)?;
        let printed = print_semio_animation_dsl(&once);
        let twice = parse_semio_animation_dsl(&printed)?;
        if twice != once {
            return Err("re-parsing the printed DSL did not reproduce the parsed snapshot".to_string());
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
