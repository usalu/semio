//! 🔮️ Mutation oracle for the TIFF 6.0 🧱️baseline conformance-class vocabulary.
//!
//! Reference: `tiff-tiff-6-0-baseline-mutate-reader` (`tiff` 0.11, MIT OR Apache-2.0). [`read_axes`]
//! takes the five Baseline axes of the real document out of IFD 0 with the `tiff` crate's own tag
//! reader — `Compression` (259), `PhotometricInterpretation` (262), `BitsPerSample` (258),
//! `TileWidth`/`TileLength` (322/323) and `StripOffsets` (273) — and counts its IFDs by walking the
//! chain the way that reader does. Each kind is then applied to those axes as Adobe TIFF 6.0 Part 1
//! defines them, and [`verdict`] re-reads the class from the specification's own tables. This
//! repository's decoder, snapshot and checker are never consulted.
//!
//! The comparison is on axes, not bytes: this repository's encoder regenerates every strip tag from
//! the raster it writes, so four of the kinds are not byte-observable at all — the vocabulary's own
//! module says so, and the case measures where the axes live.
//!
//! @see https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf — Section 7, Baseline field tables
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary this module is measured against.

use semio_repo_test_host::Json;

//#region 🔖️Axes
/// 🧭️ IFD 0's Baseline axes as the third-party reader sees them. `None` is an absent tag.
#[derive(Clone, Debug, PartialEq)]
pub struct Axes {
    pub ifd_count: usize,
    pub raster: bool,
    pub compression: Option<Vec<u32>>,
    pub photometric: Option<Vec<u32>>,
    pub bits_per_sample: Option<Vec<u32>>,
    pub tile_width: Option<Vec<u32>>,
    pub tile_length: Option<Vec<u32>>,
    pub strip_offsets: Option<Vec<u32>>,
}

/// 📖️ Reads the axes of `input` with the `tiff` crate: every tag through `Decoder::find_tag`, the raster
/// through `read_image` (a document whose pixels cannot be decoded is degenerate), the IFD count by
/// walking `more_images`/`next_image`.
#[cfg(feature = "oracles")]
pub fn read_axes(input: &[u8]) -> Result<Axes, String> {
    use tiff::tags::Tag;
    let mut decoder = tiff::decoder::Decoder::new(std::io::Cursor::new(input)).map_err(|error| format!("tiff reader refused the document: {error}"))?;
    let mut tag = |tag: Tag| -> Result<Option<Vec<u32>>, String> { decoder.find_tag(tag).map_err(|error| format!("tiff reader: {error}"))?.map(|value| value.into_u32_vec().map_err(|error| format!("tiff reader: {error}"))).transpose() };
    let compression = tag(Tag::Compression)?;
    let photometric = tag(Tag::PhotometricInterpretation)?;
    let bits_per_sample = tag(Tag::BitsPerSample)?;
    let tile_width = tag(Tag::TileWidth)?;
    let tile_length = tag(Tag::TileLength)?;
    let strip_offsets = tag(Tag::StripOffsets)?;
    let raster = matches!(decoder.dimensions(), Ok((width, height)) if width > 0 && height > 0) && decoder.read_image().is_ok();
    let mut ifd_count = 1;
    while decoder.more_images() {
        decoder.next_image().map_err(|error| format!("tiff reader: {error}"))?;
        ifd_count += 1;
    }
    Ok(Axes { ifd_count, raster, compression, photometric, bits_per_sample, tile_width, tile_length, strip_offsets })
}

/// 🚫️ Without the `oracles` feature the registered reader is not linked.
#[cfg(not(feature = "oracles"))]
pub fn read_axes(_input: &[u8]) -> Result<Axes, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Axes

//#region 🔖️Semantics
fn numbers(params: &Json, key: &str) -> Vec<u32> {
    params.array(key).into_iter().filter_map(|entry| if let Json::Number(value) = entry { Some(value as u32) } else { None }).collect()
}

fn number(params: &Json, key: &str) -> Result<u32, String> {
    match params.get(key) {
        Some(Json::Number(value)) => Ok(*value as u32),
        _ => Err(format!("params carry no numeric {key:?}")),
    }
}

/// 🦠️ Applies one kind to the axes as TIFF 6.0 defines the field it names: a set writes the field, a
/// removal deletes it, and `set-snapshot` stamps the three value axes at once.
pub fn apply(axes: &Axes, kind: &str, params: &Json) -> Result<Axes, String> {
    let mut next = axes.clone();
    match kind {
        "no-mutation" => {}
        "set-snapshot" => {
            next.compression = Some(vec![number(params, "compression")?]);
            next.photometric = Some(vec![number(params, "photometric")?]);
            next.bits_per_sample = Some(numbers(params, "bits"));
        }
        "set-compression" => next.compression = Some(vec![number(params, "compression")?]),
        "set-photometric-interpretation" => next.photometric = Some(vec![number(params, "photometric")?]),
        "set-bits-per-sample" => next.bits_per_sample = Some(numbers(params, "bits")),
        "insert-tile-tags" => {
            next.tile_width = Some(vec![number(params, "tileWidth")?]);
            next.tile_length = Some(vec![number(params, "tileLength")?]);
        }
        "remove-tile-tags" => {
            next.tile_width = None;
            next.tile_length = None;
        }
        "set-strip-offsets" => next.strip_offsets = Some(numbers(params, "offsets")),
        "remove-strip-offsets" => next.strip_offsets = None,
        other => return Err(format!("no TIFF 6.0 Baseline semantics for kind {other:?}")),
    }
    Ok(next)
}

/// 🛡️ The Baseline class read off TIFF 6.0 Part 1's own tables, in the order the fields are defined:
/// a decodable raster, `Compression` in {1, 2, 32773}, `PhotometricInterpretation` in 0..=3, every
/// `BitsPerSample` in {1, 4, 8}, and strip rather than tile organization.
pub fn verdict(axes: &Axes) -> Vec<&'static str> {
    let mut codes = Vec::new();
    if !axes.raster {
        codes.push("stdio.tiff.baseline.degenerate-raster");
    }
    if axes.compression.as_ref().and_then(|values| values.first()).is_some_and(|value| ![1, 2, 32773].contains(value)) {
        codes.push("stdio.tiff.baseline.unsupported-compression");
    }
    if axes.photometric.as_ref().and_then(|values| values.first()).is_some_and(|value| *value > 3) {
        codes.push("stdio.tiff.baseline.unsupported-photometric");
    }
    if axes.bits_per_sample.as_ref().is_some_and(|values| values.iter().any(|value| ![1, 4, 8].contains(value))) {
        codes.push("stdio.tiff.baseline.unsupported-bits-per-sample");
    }
    if axes.tile_width.is_some() || axes.tile_length.is_some() {
        codes.push("stdio.tiff.baseline.tiled-not-baseline");
    } else if axes.strip_offsets.is_none() {
        codes.push("stdio.tiff.baseline.missing-strip-offsets");
    }
    codes
}
//#endregion 🔖️Semantics

//#region 🔖️Projection
/// 🎯️ The conformance projection the case compares: IFD count, the five axes (values space-joined,
/// `absent` for a missing tag) and the class verdict.
pub fn project(axes: &Axes) -> Json {
    let list = |values: &Option<Vec<u32>>| Json::String(values.as_ref().map(|values| values.iter().map(u32::to_string).collect::<Vec<_>>().join(" ")).unwrap_or_else(|| "absent".to_string()));
    Json::Object(vec![
        ("format".to_string(), Json::String("tiff-baseline".to_string())),
        ("ifdCount".to_string(), Json::Number(axes.ifd_count as f64)),
        ("compression".to_string(), list(&axes.compression)),
        ("photometric".to_string(), list(&axes.photometric)),
        ("bitsPerSample".to_string(), list(&axes.bits_per_sample)),
        ("tileWidth".to_string(), list(&axes.tile_width)),
        ("tileLength".to_string(), list(&axes.tile_length)),
        ("stripOffsets".to_string(), list(&axes.strip_offsets)),
        ("conformance".to_string(), Json::Array(verdict(axes).into_iter().map(|code| Json::String(code.to_string())).collect())),
    ])
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(test)]
#[cfg(feature = "oracles")]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
