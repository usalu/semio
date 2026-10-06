//! 🦴️ Subject adapter of the rig-solving case: the rig of the `pets` crate answers every committed vector.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🦴️rig/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{compose, invert, rest_pose, solve_rig, transform, Affine, Point, Pose, Species};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🦴️rig-solving/🔣️.json";

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

    /// 🔢️ A number field of a vector.
    fn number(vector: &Value, name: &str) -> Result<f64, String> {
        field(vector, name)?.as_f64().ok_or_else(|| format!("{name} of {vector} is no number"))
    }

    /// 🔳️ A matrix field of a vector.
    fn affine(vector: &Value, name: &str) -> Result<Affine, String> {
        serde_json::from_value(field(vector, name)?.clone()).map_err(|error| format!("{name} of {vector}: {error}"))
    }

    /// 🤸️ The pose of a vector.
    fn pose(vector: &Value) -> Result<Pose, String> {
        serde_json::from_value(field(vector, "pose")?.clone()).map_err(|error| format!("pose of {vector}: {error}"))
    }

    /// 🧬️ The committed species a vector names.
    fn species_of(document: &Value, vector: &Value) -> Result<Species, String> {
        let id = vector.get("species").and_then(Value::as_str).ok_or_else(|| format!("vector {vector} names no species"))?;
        let species = group(document, "species")?.iter().find(|candidate| candidate["id"] == id).ok_or_else(|| format!("the vectors name the unknown species {id}"))?;
        serde_json::from_value(species.clone()).map_err(|error| format!("species {id}: {error}"))
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

    /// 🧾️ The bit patterns of a list of doubles.
    fn patterns(values: &[f64]) -> Value {
        json!(values.iter().map(|value| bits(*value)).collect::<Vec<_>>())
    }

    /// ✖️ `compose` of a committed pair.
    fn product(vector: &Value) -> Result<Affine, String> {
        Ok(compose(affine(vector, "parent")?, affine(vector, "local")?))
    }

    /// 📌️ `transform` of a committed point.
    fn carried(vector: &Value) -> Result<Point, String> {
        Ok(transform(affine(vector, "matrix")?, number(vector, "x")?, number(vector, "y")?))
    }

    /// 🩻️ `solve_rig` of a committed species in a committed pose.
    fn skeleton(document: &Value, vector: &Value) -> Result<Vec<f64>, String> {
        Ok(solve_rig(&species_of(document, vector)?, &pose(vector)?))
    }

    /// 🪟️ The product of every committed pair of matrices.
    pub fn products(ctx: &Context) -> Result<Outcome, String> {
        projected(&keyed(group(&vectors(ctx)?, "products")?, |vector| Ok(json!(product(vector)?)))?)
    }

    /// ↩️ The inverse of every committed matrix.
    pub fn inverses(ctx: &Context) -> Result<Outcome, String> {
        projected(&keyed(group(&vectors(ctx)?, "inverses")?, |vector| Ok(json!(invert(affine(vector, "matrix")?))))?)
    }

    /// 📍️ Every committed point carried by its matrix.
    pub fn points(ctx: &Context) -> Result<Outcome, String> {
        projected(&keyed(group(&vectors(ctx)?, "points")?, |vector| Ok(json!(carried(vector)?)))?)
    }

    /// 🛌️ The rest pose of every committed species and the skeleton it solves to.
    pub fn rest_poses(ctx: &Context) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&keyed(group(&document, "restPoses")?, |vector| {
            let species = species_of(&document, vector)?;
            let pose = rest_pose(&species);
            Ok(json!({"pose": pose, "bones": solve_rig(&species, &pose)}))
        })?)
    }

    /// 🦿️ The skeleton of every committed species in every committed pose.
    pub fn skeletons(ctx: &Context) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&keyed(group(&document, "skeletons")?, |vector| Ok(json!(skeleton(&document, vector)?)))?)
    }

    /// 🧮️ The bit patterns of every product, inverse, carried point and solved skeleton.
    pub fn bit_patterns(ctx: &Context) -> Result<Outcome, String> {
        let document = vectors(ctx)?;
        projected(&json!({
            "products": keyed(group(&document, "products")?, |vector| Ok(patterns(&product(vector)?)))?,
            "inverses": keyed(group(&document, "inverses")?, |vector| Ok(patterns(&invert(affine(vector, "matrix")?))))?,
            "points": keyed(group(&document, "points")?, |vector| {
                let point = carried(vector)?;
                Ok(json!({"x": bits(point.x), "y": bits(point.y)}))
            })?,
            "skeletons": keyed(group(&document, "skeletons")?, |vector| Ok(patterns(&skeleton(&document, vector)?)))?,
        }))
    }
}

/// 🧭️ Subject role only — the oracle is numpy in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("products", subject::products)
        .subject("inverses", subject::inverses)
        .subject("points", subject::points)
        .subject("rest-poses", subject::rest_poses)
        .subject("skeletons", subject::skeletons)
        .subject("bit-patterns", subject::bit_patterns);
    built
}
