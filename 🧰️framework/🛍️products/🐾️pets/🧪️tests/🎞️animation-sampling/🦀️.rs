//! 🎞️ Subject adapter of the animation-sampling case: the animation module of the `pets` crate answers every committed vector.
//!
//! The crate links its JSON codec with `float_roundtrip`, which rounds every decimal correctly, so the subject is
//! given the very doubles the TypeScript adapter is given and both project the same bits.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🎞️animation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{blend_pose, clip_ticks, ease_bezier, lid_at, sample_clip, sample_track, Clip, Ease, Pose, Species, Ticks, Track, BLINK_TICKS};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🎞️animation-sampling/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context<'_>) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 📚️ A list the document or one of its vectors carries.
    fn list<'a>(holder: &'a Value, name: &str) -> Result<&'a [Value], String> {
        holder.get(name).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("{VECTORS} carries no list {name}"))
    }

    /// 🔢️ A number.
    fn number(value: &Value) -> Result<f64, String> {
        value.as_f64().ok_or_else(|| format!("{value} is not a number"))
    }

    /// 🔡️ A text member of a vector.
    fn text<'a>(vector: &'a Value, name: &str) -> Result<&'a str, String> {
        vector.get(name).and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no text {name}"))
    }

    /// 📤️ A projection as the host carries it.
    fn projected(projection: &Value) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&projection.to_string())?))
    }

    /// 🗝️ A group of vectors answered one by one, keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Value, String> {
        let mut projection = Map::new();
        for vector in vectors {
            projection.insert(text(vector, "id")?.to_string(), answer(vector)?);
        }
        Ok(Value::Object(projection))
    }

    /// 🪜️ What a function answers along a list of numbers of a vector.
    fn along(vector: &Value, name: &str, answer: impl Fn(f64) -> Value) -> Result<Value, String> {
        Ok(Value::Array(list(vector, name)?.iter().map(|entry| number(entry).map(&answer)).collect::<Result<_, _>>()?))
    }

    /// 🎢️ `ease_bezier` of every committed easing at every committed amount.
    pub fn bezier_easings(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "easings")?, |vector| {
            let ease: Ease = serde_json::from_value(vector["ease"].clone()).map_err(|error| error.to_string())?;
            along(vector, "amounts", |amount| json!(ease_bezier(ease, amount)))
        })?)
    }

    /// 🛤️ `sample_track` of every committed track at every committed phase.
    pub fn track_samples(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "tracks")?, |vector| {
            let track: Track = serde_json::from_value(vector["track"].clone()).map_err(|error| error.to_string())?;
            along(vector, "phases", |phase| json!(sample_track(&track, phase)))
        })?)
    }

    /// ⏱️ `clip_ticks` of a clip of every committed length.
    pub fn clip_lengths(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "lengths")?, |vector| Ok(json!(clip_ticks(&Clip { id: text(vector, "id")?.to_string(), seconds: number(&vector["seconds"])?, looping: false, tracks: Vec::new() }))))?)
    }

    /// 🎬️ `sample_clip` of every committed clip of the committed species at every committed tick.
    pub fn clip_poses(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        let species: Species = serde_json::from_value(document["species"].clone()).map_err(|error| error.to_string())?;
        projected(&keyed(list(&document, "poses")?, |vector| {
            let played = text(vector, "clip")?;
            let clip = species.clips.iter().find(|candidate| candidate.id == played).ok_or_else(|| format!("the committed species has no clip {played}"))?;
            along(vector, "ticks", |ticks| json!(sample_clip(&species, clip, ticks as Ticks)))
        })?)
    }

    /// 🌗️ `blend_pose` of every committed pair of poses at every committed amount.
    pub fn pose_blends(ctx: &Context<'_>) -> Result<Outcome, String> {
        projected(&keyed(list(&vectors(ctx)?, "blends")?, |vector| {
            let from: Pose = serde_json::from_value(vector["from"].clone()).map_err(|error| error.to_string())?;
            let to: Pose = serde_json::from_value(vector["to"].clone()).map_err(|error| error.to_string())?;
            along(vector, "amounts", |amount| json!(blend_pose(&from, &to, amount)))
        })?)
    }

    /// 😉️ `BLINK_TICKS` and `lid_at` of every committed tick.
    pub fn blink_lids(ctx: &Context<'_>) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&json!({ "blinkTicks": BLINK_TICKS, "closures": along(&document["lids"], "ticks", |ticks| json!(lid_at(ticks as Ticks)))? }))
    }
}

/// 🧭️ Subject role only — the oracle is scipy and numpy in `🐍️.py`, and the oracle-only build links nothing of the crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("bezier-easings", subject::bezier_easings)
        .subject("track-samples", subject::track_samples)
        .subject("clip-lengths", subject::clip_lengths)
        .subject("clip-poses", subject::clip_poses)
        .subject("pose-blends", subject::pose_blends)
        .subject("blink-lids", subject::blink_lids);
    built
}
