//! 🦀️ LAS 1.0 mutation case — Rust adapter.
//!
//! Every scenario copies the real, derived-once 8,448-point fixture into the case work directory
//! first; the committed fixture is never written to. `oracle` drives the registered `las` 0.11
//! reference implementation (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`),
//! `subject` drives this repository's own decode/apply/encode round trip, and both results are read
//! back by the SAME independent `project_las` (built on `las::raw::{Header, Vlr, Point}`) before the
//! `semantic-las-v1` profile compares them. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::las::standards::v1_0::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_las};

//#region 🔖️Kinds
/// 🧾️ Mirrors `LasMutation::KINDS`
/// (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — kept in sync by
/// the contract phase's `mutation-kind-uncovered`/`mutation-kind-undeclared` gates, which fail loudly
/// if this list and the catalog ever drift apart.
const KINDS: [&str; 15] = ["no-mutation", "set-snapshot", "set-version", "set-system-identifier", "set-software-info", "set-creation-date", "set-scale-and-offset", "set-bounds", "set-points-by-return", "insert-vlr", "remove-vlr", "set-vlr-data", "insert-point", "remove-point", "set-point"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧊️pattern-sphere.las";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.las"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_las(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — both against the SAME
/// untouched `input`, matching `LasMutation::inverse()`'s own base-relative semantics.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = oracle_inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_las(&restored)?;
    Ok(Outcome::with_raw(restored, projection))
}

fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: oracle output is bit-identical to the input".to_string());
    }
    let projection = project_las(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::las::standards::v1_0::subsets::any::io::{decode_las, encode_las};
    use semio_s_plugin_stdio::artifacts::las::standards::v1_0::subsets::any::schema::mutations::{apply_las_mutation, LasMutation};
    use semio_s_plugin_stdio::artifacts::las::standards::v1_0::subsets::any::schema::snapshot::{LasHeader, LasPoint, LasSnapshot, LasVlr};
    use semio_s_plugin_stdio_test_oracle::artifacts::las::standards::v1_0::subsets::any::project_las;

    //#region 🔖️SpecReaders
    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Null)
    }
    fn number(value: &Json, key: &str) -> Option<f64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number),
            _ => None,
        }
    }
    fn string(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }
    fn bool_of(value: &Json, key: &str) -> Option<bool> {
        match value.get(key) {
            Some(Json::Bool(flag)) => Some(*flag),
            _ => None,
        }
    }
    fn f64x3(value: &Json, key: &str) -> Option<(f64, f64, f64)> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 3 => {
                let at = |index: usize| match &items[index] {
                    Json::Number(number) => Some(*number),
                    _ => None,
                };
                Some((at(0)?, at(1)?, at(2)?))
            }
            _ => None,
        }
    }
    fn u32x5(value: &Json, key: &str) -> Option<[u32; 5]> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 5 => {
                let mut out = [0u32; 5];
                for (slot, item) in out.iter_mut().zip(items.iter()) {
                    *slot = match item {
                        Json::Number(number) => *number as u32,
                        _ => return None,
                    };
                }
                Some(out)
            }
            _ => None,
        }
    }
    fn vlr_of(value: &Json) -> Option<LasVlr> {
        Some(LasVlr { user_id: string(value, "userId")?, record_id: number(value, "recordId")? as u16, description: string(value, "description")?, data: string(value, "data")?.into_bytes() })
    }
    fn point_of(value: &Json) -> Option<LasPoint> {
        let gps_time = match value.get("gpsTime") {
            Some(Json::Number(time)) => Some(*time),
            _ => None,
        };
        let rgb = match value.get("rgb") {
            Some(Json::Array(items)) if items.len() == 3 => {
                let at = |index: usize| match &items[index] {
                    Json::Number(channel) => Some(*channel as u16),
                    _ => None,
                };
                Some((at(0)?, at(1)?, at(2)?))
            }
            _ => None,
        };
        Some(LasPoint {
            x: number(value, "x")?,
            y: number(value, "y")?,
            z: number(value, "z")?,
            intensity: number(value, "intensity")? as u16,
            return_number: number(value, "returnNumber")? as u8,
            number_of_returns: number(value, "numberOfReturns")? as u8,
            scan_direction_flag: bool_of(value, "scanDirectionFlag")?,
            edge_of_flight_line: bool_of(value, "edgeOfFlightLine")?,
            classification: number(value, "classification")? as u8,
            scan_angle_rank: number(value, "scanAngleRank")? as i8,
            user_data: number(value, "userData")? as u8,
            point_source_id: number(value, "pointSourceId")? as u16,
            gps_time,
            rgb,
        })
    }
    fn header_of(value: &Json) -> Option<LasHeader> {
        let scale = f64x3(value, "scale")?;
        let offset = f64x3(value, "offset")?;
        let max = f64x3(value, "max")?;
        let min = f64x3(value, "min")?;
        Some(LasHeader {
            version_major: number(value, "versionMajor")? as u8,
            version_minor: number(value, "versionMinor")? as u8,
            system_identifier: string(value, "systemIdentifier")?,
            generating_software: string(value, "generatingSoftware")?,
            creation_day_of_year: number(value, "dayOfYear")? as u16,
            creation_year: number(value, "year")? as u16,
            points_by_return: u32x5(value, "counts")?,
            x_scale: scale.0,
            y_scale: scale.1,
            z_scale: scale.2,
            x_offset: offset.0,
            y_offset: offset.1,
            z_offset: offset.2,
            max_x: max.0,
            max_y: max.1,
            max_z: max.2,
            min_x: min.0,
            min_y: min.1,
            min_z: min.2,
            ..LasHeader::default()
        })
    }
    fn snapshot_of(value: &Json) -> Option<LasSnapshot> {
        let header = header_of(value.get("header")?)?;
        let vlrs: Vec<LasVlr> = match value.get("vlrs")? {
            Json::Array(items) => items.iter().map(vlr_of).collect::<Option<Vec<_>>>()?,
            _ => return None,
        };
        let points: Vec<LasPoint> = match value.get("points")? {
            Json::Array(items) => items.iter().map(point_of).collect::<Option<Vec<_>>>()?,
            _ => return None,
        };
        Some(LasSnapshot { schema: "stdio.las".to_string(), header, vlrs, points })
    }
    //#endregion 🔖️SpecReaders

    //#region 🔖️Mutation
    /// 🧭️ Builds the real `LasMutation` a spec describes.
    fn mutation_of(spec: &Json, _base: &LasSnapshot) -> Result<LasMutation, String> {
        let params = params_of(spec);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => LasMutation::NoMutation,
            "set-snapshot" => LasMutation::SetSnapshot { snapshot: snapshot_of(&params).ok_or("set-snapshot: malformed snapshot")? },
            "set-version" => LasMutation::SetVersion { major: number(&params, "major").ok_or("set-version: missing `major`")? as u8, minor: number(&params, "minor").ok_or("set-version: missing `minor`")? as u8 },
            "set-system-identifier" => LasMutation::SetSystemIdentifier { system_identifier: string(&params, "systemIdentifier").ok_or("set-system-identifier: missing `systemIdentifier`")? },
            "set-software-info" => LasMutation::SetSoftwareInfo { generating_software: string(&params, "generatingSoftware").ok_or("set-software-info: missing `generatingSoftware`")? },
            "set-creation-date" => LasMutation::SetCreationDate { day_of_year: number(&params, "dayOfYear").ok_or("set-creation-date: missing `dayOfYear`")? as u16, year: number(&params, "year").ok_or("set-creation-date: missing `year`")? as u16 },
            "set-scale-and-offset" => LasMutation::SetScaleAndOffset { scale: f64x3(&params, "scale").ok_or("set-scale-and-offset: missing `scale`")?, offset: f64x3(&params, "offset").ok_or("set-scale-and-offset: missing `offset`")? },
            "set-bounds" => LasMutation::SetBounds { max: f64x3(&params, "max").ok_or("set-bounds: missing `max`")?, min: f64x3(&params, "min").ok_or("set-bounds: missing `min`")? },
            "set-points-by-return" => LasMutation::SetPointsByReturn { counts: u32x5(&params, "counts").ok_or("set-points-by-return: missing `counts`")? },
            "insert-vlr" => LasMutation::InsertVlr { index: number(&params, "index").ok_or("insert-vlr: missing `index`")? as usize, vlr: vlr_of(params.get("vlr").ok_or("insert-vlr: missing `vlr`")?).ok_or("insert-vlr: malformed `vlr`")? },
            "remove-vlr" => LasMutation::RemoveVlr { index: number(&params, "index").ok_or("remove-vlr: missing `index`")? as usize },
            "set-vlr-data" => LasMutation::SetVlrData { index: number(&params, "index").ok_or("set-vlr-data: missing `index`")? as usize, data: string(&params, "data").ok_or("set-vlr-data: missing `data`")?.into_bytes() },
            "insert-point" => LasMutation::InsertPoint { index: number(&params, "index").ok_or("insert-point: missing `index`")? as usize, point: point_of(params.get("point").ok_or("insert-point: missing `point`")?).ok_or("insert-point: malformed `point`")? },
            "remove-point" => LasMutation::RemovePoint { index: number(&params, "index").ok_or("remove-point: missing `index`")? as usize },
            "set-point" => LasMutation::SetPoint { index: number(&params, "index").ok_or("set-point: missing `index`")? as usize, point: point_of(params.get("point").ok_or("set-point: missing `point`")?).ok_or("set-point: malformed `point`")? },
            kind => return Err(format!("mutation kind {kind:?} is not implemented by the subject")),
        })
    }

    /// ↩️ Mirrors `LasMutation::inverse()`
    /// (`../../../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`)
    /// independently: the generated oracle-role host never links `protocol`, so the trait method
    /// itself is unreachable here, and this reconstructs the same index-aware inverse by hand
    /// against the pre-mutation `base` — an out-of-range index inverts to `NoMutation`, exactly as
    /// that method does.
    fn inverse_of(spec: &Json, base: &LasSnapshot) -> Result<LasMutation, String> {
        let params = params_of(spec);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => LasMutation::NoMutation,
            "set-snapshot" => LasMutation::SetSnapshot { snapshot: base.clone() },
            "set-version" => LasMutation::SetVersion { major: base.header.version_major, minor: base.header.version_minor },
            "set-system-identifier" => LasMutation::SetSystemIdentifier { system_identifier: base.header.system_identifier.clone() },
            "set-software-info" => LasMutation::SetSoftwareInfo { generating_software: base.header.generating_software.clone() },
            "set-creation-date" => LasMutation::SetCreationDate { day_of_year: base.header.creation_day_of_year, year: base.header.creation_year },
            "set-scale-and-offset" => LasMutation::SetScaleAndOffset { scale: (base.header.x_scale, base.header.y_scale, base.header.z_scale), offset: (base.header.x_offset, base.header.y_offset, base.header.z_offset) },
            "set-bounds" => LasMutation::SetBounds { max: (base.header.max_x, base.header.max_y, base.header.max_z), min: (base.header.min_x, base.header.min_y, base.header.min_z) },
            "set-points-by-return" => LasMutation::SetPointsByReturn { counts: base.header.points_by_return },
            "insert-vlr" => LasMutation::RemoveVlr { index: (number(&params, "index").ok_or("insert-vlr: missing `index`")? as usize).min(base.vlrs.len()) },
            "remove-vlr" => {
                let index = number(&params, "index").ok_or("remove-vlr: missing `index`")? as usize;
                match base.vlrs.get(index) {
                    Some(vlr) => LasMutation::InsertVlr { index, vlr: vlr.clone() },
                    None => LasMutation::NoMutation,
                }
            }
            "set-vlr-data" => {
                let index = number(&params, "index").ok_or("set-vlr-data: missing `index`")? as usize;
                match base.vlrs.get(index) {
                    Some(vlr) => LasMutation::SetVlrData { index, data: vlr.data.clone() },
                    None => LasMutation::NoMutation,
                }
            }
            "insert-point" => LasMutation::RemovePoint { index: (number(&params, "index").ok_or("insert-point: missing `index`")? as usize).min(base.points.len()) },
            "remove-point" => {
                let index = number(&params, "index").ok_or("remove-point: missing `index`")? as usize;
                match base.points.get(index) {
                    Some(point) => LasMutation::InsertPoint { index, point: point.clone() },
                    None => LasMutation::NoMutation,
                }
            }
            "set-point" => {
                let index = number(&params, "index").ok_or("set-point: missing `index`")? as usize;
                match base.points.get(index) {
                    Some(point) => LasMutation::SetPoint { index, point: point.clone() },
                    None => LasMutation::NoMutation,
                }
            }
            kind => return Err(format!("mutation kind {kind:?} is not implemented by the subject")),
        })
    }
    //#endregion 🔖️Mutation

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let mut snapshot = decode_las(&bytes).map_err(|error| format!("decode_las failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_of(&spec, &snapshot)?;
        apply_las_mutation(&mut snapshot, &mutation);
        let output = encode_las(&snapshot).map_err(|error| format!("encode_las failed: {error}"))?;
        let projection = project_las(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let bytes = mutable_input(ctx)?;
        let base = decode_las(&bytes).map_err(|error| format!("decode_las failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let forward = mutation_of(&spec, &base)?;
        let backward = inverse_of(&spec, &base)?;
        let mut snapshot = base;
        apply_las_mutation(&mut snapshot, &forward);
        apply_las_mutation(&mut snapshot, &backward);
        let output = encode_las(&snapshot).map_err(|error| format!("encode_las failed: {error}"))?;
        let projection = project_las(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_las(&input).map_err(|error| format!("decode_las failed: {error}"))?;
        let output = encode_las(&snapshot).map_err(|error| format!("encode_las failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: subject output is bit-identical to the input".to_string());
        }
        let projection = project_las(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
