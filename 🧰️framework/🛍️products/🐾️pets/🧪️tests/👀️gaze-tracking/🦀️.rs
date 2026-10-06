//! 👀️ Subject adapter of the gaze-tracking case: the rig of the `pets` crate answers every committed vector.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🦴️rig/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{look_offset, transform, Affine, Point};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://👀️gaze-tracking/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 🗂️ One group of the committed vectors.
    fn group<'a>(document: &'a Value, name: &str) -> Result<&'a Vec<Value>, String> {
        document.get(name).and_then(Value::as_array).ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🧷️ A field of a vector.
    fn field<'a>(vector: &'a Value, name: &str) -> Result<&'a Value, String> {
        vector.get(name).ok_or_else(|| format!("vector {vector} carries no {name}"))
    }

    /// 📍️ A point field of a vector.
    fn point(vector: &Value, name: &str) -> Result<Point, String> {
        serde_json::from_value(field(vector, name)?.clone()).map_err(|error| format!("{name} of {vector}: {error}"))
    }

    /// 🔭️ The reach of a vector.
    fn reach(vector: &Value) -> Result<f64, String> {
        field(vector, "reach")?.as_f64().ok_or_else(|| format!("reach of {vector} is no number"))
    }

    /// 🗝️ The answers keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Value, String> {
        let mut projection = Map::new();
        for vector in vectors {
            let id = vector.get("id").and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no id"))?;
            projection.insert(id.to_string(), answer(vector)?);
        }
        Ok(Value::Object(projection))
    }

    /// 🔣️ A projection as the owned protocol value.
    fn projected(value: &Value) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&value.to_string())?))
    }

    /// 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits.
    fn bits(value: f64) -> String {
        format!("{:016x}", value.to_bits())
    }

    /// 🔬️ The bit patterns of a point.
    fn point_bits(point: Point) -> Value {
        json!({"x": bits(point.x), "y": bits(point.y)})
    }

    /// 🎯️ `look_offset` of a committed eye, target and reach.
    fn offset(vector: &Value) -> Result<Point, String> {
        Ok(look_offset(point(vector, "eye")?, point(vector, "target")?, reach(vector)?))
    }

    /// 👁️ Where an eye on a posed bone sits in the pet's frame, and the offset of its pupil towards the target.
    fn eye_look(vector: &Value) -> Result<(Point, Point), String> {
        let bone: Affine = serde_json::from_value(field(vector, "bone")?.clone()).map_err(|error| format!("bone of {vector}: {error}"))?;
        let resting = point(vector, "eye")?;
        let carried = transform(bone, resting.x, resting.y);
        Ok((carried, look_offset(carried, point(vector, "target")?, reach(vector)?)))
    }

    /// 🧲️ The offset of every committed pair of eye and target.
    pub fn offsets(ctx: &Context) -> Result<Outcome, String> {
        projected(&keyed(group(&vectors(ctx)?, "offsets")?, |vector| Ok(json!(offset(vector)?)))?)
    }

    /// 🧿️ Every committed eye carried by its bone, with the offset of its pupil.
    pub fn eyes(ctx: &Context) -> Result<Outcome, String> {
        projected(&keyed(group(&vectors(ctx)?, "eyes")?, |vector| {
            let (eye, offset) = eye_look(vector)?;
            Ok(json!({"eye": eye, "offset": offset}))
        })?)
    }

    /// 🧬️ The bit patterns of every offset and of every carried eye with its offset.
    pub fn bit_patterns(ctx: &Context) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&json!({
            "offsets": keyed(group(&document, "offsets")?, |vector| Ok(point_bits(offset(vector)?)))?,
            "eyes": keyed(group(&document, "eyes")?, |vector| {
                let (eye, offset) = eye_look(vector)?;
                Ok(json!({"eye": point_bits(eye), "offset": point_bits(offset)}))
            })?,
        }))
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("offsets", subject::offsets).subject("eyes", subject::eyes).subject("bit-patterns", subject::bit_patterns);
    built
}
