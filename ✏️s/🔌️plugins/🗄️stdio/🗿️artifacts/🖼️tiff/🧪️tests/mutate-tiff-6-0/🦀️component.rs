//! 🦀️ TIFF 6.0 mutation case — Rust adapter. Every scenario copies the real, committed, genuinely
//! two-page fixture into the case work directory first; the committed document is never written to.
//! `oracle` drives this subset's own independent hand-rolled IFD-chain codec
//! (`../../🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), `subject` drives this
//! repository's own `decode_tiff`/`apply_tiff_mutation`/`encode_tiff`. Both results are read back by
//! the SAME independent `project_tiff` reader before the `semantic-raster-v1` profile compares them.
//! The subject half is gated behind the generated host's `sut` feature, so the oracle-only run never
//! compiles the local implementation.
//!
//! `KINDS` is duplicated locally rather than imported from `semio_s_plugin_stdio` (which currently
//! fails to compile repo-wide — an unrelated os-kernel refactor's `semio_framework::` cycle, see the
//! wave brief) so oracle-only registration never needs the broken production crate. Keep this list in
//! sync with `../../🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own
//! `KINDS` and `../../🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`'s
//! `mutationCatalogs[0].kinds` — the `kinds_manifest_law` test there is what keeps those two honest;
//! this third copy is test-harness wiring, not vocabulary.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::any::{oracle_apply_mutation, project_tiff};

//#region 🔖️Kinds
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-byte-order", "insert-ifd", "remove-ifd", "set-tag", "remove-tag", "set-pixels"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.tiff"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🧩️ `set-pixels`' payload is full-resolution real RGBA8 (this fixture decodes to 23M+ bytes), far
/// too large for an inline feature-file hex literal — its row instead carries
/// `{"pixelsFixture": "local://…"}`, resolved here into the literal `pixels` hex key
/// `oracle_apply_mutation` (and the subject's own parser below) expect. Every other kind's spec
/// passes through untouched.
fn resolve_spec(ctx: &Context, spec: Json) -> Result<Json, String> {
    if spec.str("kind") != "set-pixels" {
        return Ok(spec);
    }
    let fixture_uri = spec
        .get("params")
        .and_then(|params| params.get("pixelsFixture"))
        .and_then(|value| match value {
            Json::String(s) => Some(s.clone()),
            _ => None,
        })
        .ok_or("set-pixels spec needs params.pixelsFixture")?;
    let bytes = ctx.fixture_bytes(&fixture_uri)?;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(Json::Object(vec![("kind".to_string(), Json::String("set-pixels".to_string())), ("params".to_string(), Json::Object(vec![("pixels".to_string(), Json::String(hex))]))]))
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = resolve_spec(ctx, ctx.doc_json()?)?;
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_tiff(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ "Undoing `<id>` restores the document" is a PROPERTY, not a differential comparison: the
/// oracle's target is simply the pristine, untouched original's own projection — the subject's own
/// handler is what actually applies the mutation and then its inverse and must land back here.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let projection = project_tiff(&input)?;
    Ok(Outcome::with_raw(input, projection))
}

/// ↩️ Same property shape as [`inverse_oracle`]: the target is the untouched original's projection.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let projection = project_tiff(&input)?;
    Ok(Outcome::with_raw(input, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, resolve_spec, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::any::io::{decode_tiff, encode_tiff};
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::any::schema::mutations::apply_tiff_mutation;
    use semio_s_plugin_stdio::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
    use semio_s_plugin_stdio::artifacts::tiff::{TiffMutation, TiffSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::tiff::standards::v6_0::subsets::any::project_tiff;

    //#region 🔖️SpecParsing
    /// 🧩️ Same JSON param shapes the oracle's own `oracle_apply_mutation` parses (this file's own
    /// independent construction of the REAL `TiffMutation` — never routed through the oracle's
    /// parser, only through the same wire shape the feature file's `params` cells declare).
    fn j_num(v: &Json, key: &str) -> Option<f64> {
        match v.get(key) {
            Some(Json::Number(n)) => Some(*n),
            _ => None,
        }
    }
    fn j_str<'a>(v: &'a Json, key: &str) -> Option<&'a str> {
        match v.get(key) {
            Some(Json::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }
    fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| format!("hex: {e}"))).collect()
    }
    fn values_from_json(type_code: u16, values: &Json) -> Result<TiffValues, String> {
        let kind = TiffFieldType::from_u16(type_code)?;
        let items = match values {
            Json::Array(items) => items,
            _ => return Err("tag values must be a JSON array".to_string()),
        };
        let nums = || -> Result<Vec<f64>, String> {
            items
                .iter()
                .map(|v| match v {
                    Json::Number(n) => Ok(*n),
                    _ => Err("expected a number in tag values".to_string()),
                })
                .collect()
        };
        Ok(match kind {
            TiffFieldType::Byte => TiffValues::Byte(nums()?.into_iter().map(|n| n as u8).collect()),
            TiffFieldType::Ascii => TiffValues::Ascii(match items.first() {
                Some(Json::String(s)) => s.clone(),
                _ => return Err("ascii tag values must be [\"text\"]".to_string()),
            }),
            TiffFieldType::Short => TiffValues::Short(nums()?.into_iter().map(|n| n as u16).collect()),
            TiffFieldType::Long => TiffValues::Long(nums()?.into_iter().map(|n| n as u32).collect()),
            TiffFieldType::Rational => TiffValues::Rational(
                items
                    .iter()
                    .map(|pair| match pair {
                        Json::Array(p) if p.len() == 2 => Ok((json_num(&p[0]) as u32, json_num(&p[1]) as u32)),
                        _ => Err("rational value must be [num,den]".to_string()),
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
            TiffFieldType::SByte => TiffValues::SByte(nums()?.into_iter().map(|n| n as i8).collect()),
            TiffFieldType::Undefined => TiffValues::Undefined(nums()?.into_iter().map(|n| n as u8).collect()),
            TiffFieldType::SShort => TiffValues::SShort(nums()?.into_iter().map(|n| n as i16).collect()),
            TiffFieldType::SLong => TiffValues::SLong(nums()?.into_iter().map(|n| n as i32).collect()),
            TiffFieldType::SRational => TiffValues::SRational(
                items
                    .iter()
                    .map(|pair| match pair {
                        Json::Array(p) if p.len() == 2 => Ok((json_num(&p[0]) as i32, json_num(&p[1]) as i32)),
                        _ => Err("srational value must be [num,den]".to_string()),
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ),
            TiffFieldType::Float => TiffValues::Float(nums()?.into_iter().map(|n| n as f32).collect()),
            TiffFieldType::Double => TiffValues::Double(nums()?),
        })
    }
    fn json_num(v: &Json) -> f64 {
        match v {
            Json::Number(n) => *n,
            _ => 0.0,
        }
    }
    fn ifd_from_json(v: &Json) -> Result<(TiffIfd, Option<Vec<u8>>), String> {
        let entries_json = match v.get("entries") {
            Some(Json::Array(items)) => items,
            _ => return Err("ifd needs an `entries` array".to_string()),
        };
        let mut entries = Vec::new();
        for entry in entries_json {
            let tag = j_num(entry, "tag").ok_or("entry needs `tag`")? as u16;
            if tag == 273 || tag == 279 {
                continue; // StripOffsets/StripByteCounts are layout-computed by `encode_tiff`, never caller-supplied.
            }
            let type_code = j_num(entry, "type").ok_or("entry needs `type`")? as u16;
            let values = entry.get("values").ok_or("entry needs `values`")?;
            entries.push(TiffTag { tag, kind: TiffFieldType::from_u16(type_code)?, values: values_from_json(type_code, values)? });
        }
        entries.sort_by_key(|t| t.tag);
        let pixels = match j_str(v, "pixels") {
            Some(hex) => Some(hex_decode(hex)?),
            None => None,
        };
        Ok((TiffIfd { entries }, pixels))
    }

    fn spec_to_mutation(spec: &Json) -> Result<TiffMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params");
        let p_num = |key: &str| -> Option<f64> { params.and_then(|p| j_num(p, key)) };
        let p_str = |key: &str| -> Option<&str> { params.and_then(|p| j_str(p, key)) };
        Ok(match kind.as_str() {
            "no-mutation" => TiffMutation::NoMutation,
            "set-byte-order" => TiffMutation::SetByteOrder { byte_order: if p_str("byteOrder") == Some("big-endian") { TiffByteOrder::BigEndian } else { TiffByteOrder::LittleEndian } },
            "insert-ifd" => {
                let index = p_num("index").ok_or("insert-ifd needs `index`")? as usize;
                let ifd_json = params.and_then(|p| p.get("ifd")).ok_or("insert-ifd needs `ifd`")?;
                let (ifd, _pixels) = ifd_from_json(ifd_json)?;
                TiffMutation::InsertIfd { index, ifd }
            }
            "remove-ifd" => TiffMutation::RemoveIfd { index: p_num("index").ok_or("remove-ifd needs `index`")? as usize },
            "set-tag" => {
                let ifd_index = p_num("ifdIndex").ok_or("set-tag needs `ifdIndex`")? as usize;
                let tag = p_num("tag").ok_or("set-tag needs `tag`")? as u16;
                let type_code = p_num("type").ok_or("set-tag needs `type`")? as u16;
                let values = params.and_then(|p| p.get("values")).ok_or("set-tag needs `values`")?;
                TiffMutation::SetTag { ifd_index, tag, kind: TiffFieldType::from_u16(type_code)?, values: values_from_json(type_code, values)? }
            }
            "remove-tag" => TiffMutation::RemoveTag { ifd_index: p_num("ifdIndex").ok_or("remove-tag needs `ifdIndex`")? as usize, tag: p_num("tag").ok_or("remove-tag needs `tag`")? as u16 },
            "set-pixels" => TiffMutation::SetPixels { pixels: hex_decode(p_str("pixels").ok_or("set-pixels needs `pixels`")?)? },
            "set-snapshot" => {
                let ifds_json = match params.and_then(|p| p.get("ifds")) {
                    Some(Json::Array(items)) => items,
                    _ => return Err("set-snapshot needs an `ifds` array".to_string()),
                };
                let mut ifds = Vec::new();
                let mut pixels = Vec::new();
                for ifd_json in ifds_json {
                    let (ifd, strip) = ifd_from_json(ifd_json)?;
                    if let Some(strip) = strip {
                        pixels = strip.chunks_exact(3).flat_map(|px| [px[0], px[1], px[2], 255]).collect();
                    }
                    ifds.push(ifd);
                }
                let byte_order = if p_str("byteOrder") == Some("big-endian") { TiffByteOrder::BigEndian } else { TiffByteOrder::LittleEndian };
                TiffMutation::SetSnapshot { snapshot: TiffSnapshot { schema: "stdio.tiff".to_string(), byte_order, ifds, pixels } }
            }
            other => return Err(format!("subject: unrecognized mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️SpecParsing

    //#region 🔖️Inverse
    /// ↩️ Mirrors `TiffMutation::inverse`'s own documented, base-aware rules
    /// (`../../🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — kept local
    /// rather than calling that trait method directly, since doing so would need the production
    /// `protocol` crate as an extra direct dependency of this generated test crate for no gain: the
    /// mutation this scenario actually puts under test is still `apply_tiff_mutation`, exercised in
    /// BOTH directions below.
    fn inverse_of(mutation: &TiffMutation, base: &TiffSnapshot) -> TiffMutation {
        match mutation {
            TiffMutation::NoMutation => TiffMutation::NoMutation,
            TiffMutation::SetSnapshot { .. } => TiffMutation::SetSnapshot { snapshot: base.clone() },
            TiffMutation::SetByteOrder { .. } => TiffMutation::SetByteOrder { byte_order: base.byte_order },
            TiffMutation::InsertIfd { index, .. } => TiffMutation::RemoveIfd { index: (*index).min(base.ifds.len()) },
            TiffMutation::RemoveIfd { index } => match base.ifds.get(*index) {
                Some(ifd) => TiffMutation::InsertIfd { index: *index, ifd: ifd.clone() },
                None => TiffMutation::NoMutation,
            },
            TiffMutation::SetTag { ifd_index, tag, .. } => match base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|t| t.tag == *tag)) {
                Some(existing) => TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() },
                None => TiffMutation::RemoveTag { ifd_index: *ifd_index, tag: *tag },
            },
            TiffMutation::RemoveTag { ifd_index, tag } => match base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|t| t.tag == *tag)) {
                Some(existing) => TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() },
                None => TiffMutation::NoMutation,
            },
            TiffMutation::SetPixels { .. } => TiffMutation::SetPixels { pixels: base.pixels.clone() },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    /// 🚫️ The tripwire this whole wave exists for: our encoder cannot reproduce another writer's
    /// object layout, so byte-identical output means the input was smuggled through, not parsed.
    fn no_byte_pass_through(output: &[u8], input: &[u8]) -> Result<(), String> {
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(())
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = resolve_spec(ctx, ctx.doc_json()?)?;
        let input = mutable_input(ctx)?;
        let mutation = spec_to_mutation(&spec)?;
        let mut snapshot = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        apply_tiff_mutation(&mut snapshot, &mutation);
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        no_byte_pass_through(&output, &input)?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = resolve_spec(ctx, ctx.doc_json()?)?;
        let input = mutable_input(ctx)?;
        let mutation = spec_to_mutation(&spec)?;
        let base = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        let mut snapshot = base.clone();
        apply_tiff_mutation(&mut snapshot, &mutation);
        let inverse = inverse_of(&mutation, &base);
        apply_tiff_mutation(&mut snapshot, &inverse);
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_tiff(&input).map_err(|error| format!("decode_tiff failed: {error:?}"))?;
        let output = encode_tiff(&snapshot).map_err(|error| format!("encode_tiff failed: {error:?}"))?;
        no_byte_pass_through(&output, &input)?;
        let projection = project_tiff(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    #[allow(dead_code)]
    const _KEEP_KINDS_REACHABLE: &[&str] = KINDS;
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
