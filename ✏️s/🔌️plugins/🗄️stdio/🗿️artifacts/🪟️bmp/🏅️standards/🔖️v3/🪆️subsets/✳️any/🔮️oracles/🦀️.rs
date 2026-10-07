//! 🔮️ Independent BMP v3 owned sample, metadata, mutation, and revision oracle.

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
mod oracles {
    use image::ImageDecoder;
    use semio_repo_test_host::{digest, Json};

    pub enum Content {
        Indexed { indices: Vec<u8>, palette: Vec<[u8; 4]> },
        Direct { samples: Vec<[u32; 5]> },
    }

    pub struct OracleDoc {
        pub width: u32,
        pub height: u32,
        pub top_down: bool,
        pub profile: String,
        pub masks: [u32; 4],
        pub x_pixels_per_meter: i32,
        pub y_pixels_per_meter: i32,
        pub colors_used: u32,
        pub colors_important: u32,
        pub reserved_1: u16,
        pub reserved_2: u16,
        pub opaque_gap: Vec<u8>,
        pub opaque_trailer: Vec<u8>,
        pub content: Content,
    }

    fn object(fields: Vec<(&str, Json)>) -> Json { Json::Object(fields.into_iter().map(|(name, value)| (name.into(), value)).collect()) }
    fn number(value: impl Into<f64>) -> Json { Json::Number(value.into()) }
    fn text(value: &str) -> Json { Json::String(value.into()) }
    fn array(values: impl IntoIterator<Item = Json>) -> Json { Json::Array(values.into_iter().collect()) }
    fn integers<T: Copy + Into<f64>>(values: &[T]) -> Json { array(values.iter().map(|value| number(*value))) }
    fn member<'a>(value: &'a Json, name: &str) -> Result<&'a Json, String> { value.get(name).ok_or_else(|| format!("missing owned member {name}")) }
    fn numeric(value: &Json, min: f64, max: f64) -> Result<f64, String> {
        match value { Json::Number(number) if number.is_finite() && number.fract() == 0.0 && *number >= min && *number <= max => Ok(*number), _ => Err("invalid owned integer".into()) }
    }
    fn uint(value: &Json) -> Result<u32, String> { numeric(value, 0.0, u32::MAX as f64).map(|n| n as u32) }
    fn scalar(value: &Json, name: &str) -> Result<u32, String> { uint(member(value, name)?) }
    fn sequence(value: &Json) -> Result<&[Json], String> { match value { Json::Array(values) => Ok(values), _ => Err("expected owned array".into()) } }
    fn bytes(value: &Json) -> Result<Vec<u8>, String> { sequence(value)?.iter().map(|n| numeric(n, 0.0, 255.0).map(|n| n as u8)).collect() }
    fn exact(value: &Json, names: &[&str]) -> Result<(), String> {
        match value { Json::Object(fields) if fields.len() == names.len() && names.iter().all(|name| fields.iter().filter(|(field, _)| field == name).count() == 1) => Ok(()), _ => Err("owned record has missing, duplicate, or foreign members".into()) }
    }
    fn depth(profile: &str) -> Result<usize, String> {
        match profile { "indexedRgb1" => Ok(1), "indexedRgb4" => Ok(4), "indexedRgb8" => Ok(8), "directRgb16" | "directBitfields16" => Ok(16), "directRgb24" => Ok(24), "directRgb32" | "directBitfields32" => Ok(32), _ => Err("unknown BMP profile".into()) }
    }
    fn maximum(mask: u32) -> u32 { if mask == 0 { 0 } else { mask >> mask.trailing_zeros() } }
    fn read16(input: &[u8], at: usize) -> Result<u16, String> { input.get(at..at + 2).map(|v| u16::from_le_bytes(v.try_into().unwrap())).ok_or("truncated BMP word".into()) }
    fn read32(input: &[u8], at: usize) -> Result<u32, String> { input.get(at..at + 4).map(|v| u32::from_le_bytes(v.try_into().unwrap())).ok_or("truncated BMP scalar".into()) }

    impl OracleDoc {
        fn validate(&self) -> Result<(), String> {
            let bits = depth(&self.profile)?;
            if self.width == 0 || self.height == 0 || self.width > i32::MAX as u32 || self.height > i32::MAX as u32 { return Err("invalid native dimensions".into()); }
            let count = (self.width as usize).checked_mul(self.height as usize).ok_or("owned extent overflow")?;
            match &self.content {
                Content::Indexed { indices, palette } => {
                    let declared = if self.colors_used == 0 { 1usize << bits } else { self.colors_used as usize };
                    if bits > 8 || self.masks != [0; 4] || palette.is_empty() || palette.len() > 1usize << bits || palette.len() != declared || indices.len() != count || indices.iter().any(|index| *index as usize >= palette.len()) { return Err("invalid indexed ownership".into()); }
                }
                Content::Direct { samples } => {
                    if bits <= 8 || samples.len() != count || self.masks[3] != 0 { return Err("invalid direct ownership".into()); }
                    let fixed = match self.profile.as_str() { "directRgb16" => Some([0x7c00, 0x3e0, 0x1f, 0]), "directRgb24" | "directRgb32" => Some([0xff0000, 0xff00, 0xff, 0]), _ => None };
                    if fixed.is_some_and(|expected| self.masks != expected) { return Err("RGB profile masks differ".into()); }
                    let allowed = if bits == 32 { u32::MAX } else { (1u32 << bits) - 1 };
                    let mut assigned = 0;
                    for (lane, mask) in self.masks.iter().copied().enumerate() {
                        let limit = maximum(mask);
                        if lane < 3 && mask == 0 || mask & assigned != 0 || mask & !allowed != 0 || limit != 0 && limit & limit.wrapping_add(1) != 0 { return Err("invalid native channel mask".into()); }
                        assigned |= mask;
                    }
                    if samples.iter().any(|sample| sample[..4].iter().zip(self.masks).any(|(value, mask)| *value > maximum(mask)) || sample[4] & assigned != 0 || sample[4] & !allowed != 0) { return Err("invalid native component precision".into()); }
                }
            }
            Ok(())
        }

        pub fn snapshot(&self) -> Json {
            let palette = match &self.content { Content::Indexed { palette, .. } => array(palette.iter().map(|entry| object(vec![("b", number(entry[2])), ("g", number(entry[1])), ("r", number(entry[0])), ("reserved", number(entry[3]))]))), _ => array([]) };
            let pixels = match &self.content {
                Content::Indexed { indices, .. } => object(vec![("storage", text("indexed")), ("indices", integers(indices))]),
                Content::Direct { samples } => object(vec![("storage", text("direct")), ("samples", array(samples.iter().map(|sample| object(["red", "green", "blue", "alpha", "reserved"].into_iter().zip(*sample).map(|(key, value)| (key, number(value))).collect()))))]),
            };
            object(vec![("schema", text("stdio.bmp")), ("image", object(vec![
                ("width", number(self.width)), ("height", number(self.height)), ("rowOrder", text(if self.top_down { "topDown" } else { "bottomUp" })), ("profile", text(&self.profile)),
                ("masks", integers(&self.masks)), ("palette", palette), ("pixels", pixels), ("xPixelsPerMeter", number(self.x_pixels_per_meter)), ("yPixelsPerMeter", number(self.y_pixels_per_meter)),
                ("colorsUsed", number(self.colors_used)), ("colorsImportant", number(self.colors_important)), ("reserved1", number(self.reserved_1)), ("reserved2", number(self.reserved_2)),
                ("opaqueGap", integers(&self.opaque_gap)), ("opaqueTrailer", integers(&self.opaque_trailer))
            ]))])
        }

        pub fn from_snapshot(snapshot: &Json) -> Result<Self, String> {
            exact(snapshot, &["schema", "image"])?;
            if snapshot.str("schema") != "stdio.bmp" { return Err("invalid owned schema".into()); }
            let image = member(snapshot, "image")?;
            exact(image, &["width", "height", "rowOrder", "profile", "masks", "palette", "pixels", "xPixelsPerMeter", "yPixelsPerMeter", "colorsUsed", "colorsImportant", "reserved1", "reserved2", "opaqueGap", "opaqueTrailer"])?;
            let masks: [u32; 4] = sequence(member(image, "masks")?)?.iter().map(uint).collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| "four masks required")?;
            let palette = sequence(member(image, "palette")?)?.iter().map(|entry| {
                exact(entry, &["b", "g", "r", "reserved"])?;
                ["r", "g", "b", "reserved"].map(|key| numeric(member(entry, key)?, 0.0, 255.0).map(|n| n as u8)).into_iter().collect::<Result<Vec<_>, String>>()?.try_into().map_err(|_| "invalid palette entry".into())
            }).collect::<Result<Vec<[u8; 4]>, String>>()?;
            let pixels = member(image, "pixels")?;
            let content = match pixels.str("storage").as_str() {
                "indexed" => { exact(pixels, &["storage", "indices"])?; Content::Indexed { indices: bytes(member(pixels, "indices")?)?, palette } }
                "direct" => {
                    exact(pixels, &["storage", "samples"])?;
                    if !palette.is_empty() { return Err("direct pixels cannot own palette".into()); }
                    let samples = sequence(member(pixels, "samples")?)?.iter().map(|sample| {
                        exact(sample, &["red", "green", "blue", "alpha", "reserved"])?;
                        ["red", "green", "blue", "alpha", "reserved"].map(|key| scalar(sample, key)).into_iter().collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| "invalid sample".into())
                    }).collect::<Result<Vec<[u32; 5]>, String>>()?;
                    Content::Direct { samples }
                }
                _ => return Err("unknown pixel storage".into())
            };
            let doc = Self {
                width: scalar(image, "width")?, height: scalar(image, "height")?, top_down: match image.str("rowOrder").as_str() { "topDown" => true, "bottomUp" => false, _ => return Err("invalid row order".into()) }, profile: image.str("profile"), masks, content,
                x_pixels_per_meter: numeric(member(image, "xPixelsPerMeter")?, i32::MIN as f64, i32::MAX as f64)? as i32,
                y_pixels_per_meter: numeric(member(image, "yPixelsPerMeter")?, i32::MIN as f64, i32::MAX as f64)? as i32,
                colors_used: scalar(image, "colorsUsed")?, colors_important: scalar(image, "colorsImportant")?,
                reserved_1: numeric(member(image, "reserved1")?, 0.0, 65535.0)? as u16, reserved_2: numeric(member(image, "reserved2")?, 0.0, 65535.0)? as u16,
                opaque_gap: bytes(member(image, "opaqueGap")?)?, opaque_trailer: bytes(member(image, "opaqueTrailer")?)?,
            };
            doc.validate()?;
            Ok(doc)
        }
    }

    pub fn decode(input: &[u8]) -> Result<OracleDoc, String> {
        let mut decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(input)).map_err(|error| error.to_string())?;
        let (width, height) = decoder.dimensions();
        if input.get(..2) != Some(b"BM") || read32(input, 14)? != 40 || read16(input, 26)? != 1 { return Err("not BMP v3".into()); }
        let bits = read16(input, 28)? as usize;
        let compression = read32(input, 30)?;
        let profile = match (bits, compression) { (1, 0) => "indexedRgb1", (4, 0) => "indexedRgb4", (8, 0) => "indexedRgb8", (16, 0) => "directRgb16", (24, 0) => "directRgb24", (32, 0) => "directRgb32", (16, 3) => "directBitfields16", (32, 3) => "directBitfields32", _ => return Err("unsupported native profile".into()) };
        let masks = match compression { 3 => [read32(input, 54)?, read32(input, 58)?, read32(input, 62)?, 0], _ if bits <= 8 => [0; 4], _ if bits == 16 => [0x7c00, 0x3e0, 0x1f, 0], _ => [0xff0000, 0xff00, 0xff, 0] };
        let offset = read32(input, 10)? as usize;
        let colors_used = read32(input, 46)?;
        let palette_count = if bits <= 8 { if colors_used == 0 { 1usize << bits } else { colors_used as usize } } else { 0 };
        let metadata_end = 54 + usize::from(compression == 3) * 12 + palette_count * 4;
        let stride = (width as usize * bits).div_ceil(32) * 4;
        let raster_end = offset.checked_add(stride.checked_mul(height as usize).ok_or("raster overflow")?).ok_or("raster overflow")?;
        if metadata_end > offset || raster_end > input.len() { return Err("truncated native raster".into()); }
        let top_down = (read32(input, 22)? as i32) < 0;
        let content = if bits <= 8 {
            let palette: Vec<[u8; 4]> = input[54..metadata_end].chunks_exact(4).map(|entry| [entry[2], entry[1], entry[0], entry[3]]).collect();
            let mut indices = vec![0; width as usize * height as usize];
            if bits == 8 {
                decoder.set_indexed_color(true);
                decoder.read_image(&mut indices).map_err(|error| error.to_string())?;
            } else {
                for y in 0..height as usize {
                    let native_y = if top_down { y } else { height as usize - 1 - y };
                    for x in 0..width as usize {
                        let bit = x * bits;
                        indices[y * width as usize + x] = (input[offset + native_y * stride + bit / 8] >> (8 - bits - bit % 8)) & ((1 << bits) - 1);
                    }
                }
                let actual = image::load_from_memory(input).map_err(|error| error.to_string())?.to_rgb8();
                for (index, pixel) in indices.iter().zip(actual.pixels()) {
                    let entry = palette.get(*index as usize).ok_or("indexed palette range")?;
                    if pixel.0 != entry[..3] { return Err("independent visual palette witness differs".into()); }
                }
            }
            Content::Indexed { indices, palette }
        } else {
            let assigned = masks.iter().fold(0, |union, mask| union | mask);
            let mut samples = Vec::with_capacity(width as usize * height as usize);
            for y in 0..height as usize {
                let native_y = if top_down { y } else { height as usize - 1 - y };
                for x in 0..width as usize {
                    let at = offset + native_y * stride + x * (bits / 8);
                    let word = match bits { 16 => read16(input, at)? as u32, 24 => u32::from_le_bytes([input[at], input[at + 1], input[at + 2], 0]), _ => read32(input, at)? };
                    samples.push([if masks[0] == 0 { 0 } else { (word & masks[0]) >> masks[0].trailing_zeros() }, if masks[1] == 0 { 0 } else { (word & masks[1]) >> masks[1].trailing_zeros() }, if masks[2] == 0 { 0 } else { (word & masks[2]) >> masks[2].trailing_zeros() }, 0, word & !assigned]);
                }
            }
            image::load_from_memory_with_format(input, image::ImageFormat::Bmp).map_err(|error| error.to_string())?;
            Content::Direct { samples }
        };
        let doc = OracleDoc { width, height, top_down, profile: profile.into(), masks, content, x_pixels_per_meter: read32(input, 38)? as i32, y_pixels_per_meter: read32(input, 42)? as i32, colors_used, colors_important: read32(input, 50)?, reserved_1: read16(input, 6)?, reserved_2: read16(input, 8)?, opaque_gap: input[metadata_end..offset].to_vec(), opaque_trailer: input[raster_end..].to_vec() };
        doc.validate()?;
        Ok(doc)
    }

    pub fn encode(doc: &OracleDoc) -> Result<Vec<u8>, String> {
        doc.validate()?;
        let bits = depth(&doc.profile)?;
        let bitfields = doc.profile.starts_with("directBitfields");
        let palette = match &doc.content { Content::Indexed { palette, .. } => palette.as_slice(), _ => &[] };
        let stride = (doc.width as usize * bits).div_ceil(32) * 4;
        let offset = 54 + usize::from(bitfields) * 12 + palette.len() * 4 + doc.opaque_gap.len();
        let raster_size = stride * doc.height as usize;
        let size = offset + raster_size + doc.opaque_trailer.len();
        let mut out = vec![0; size];
        out[..2].copy_from_slice(b"BM");
        out[6..8].copy_from_slice(&doc.reserved_1.to_le_bytes()); out[8..10].copy_from_slice(&doc.reserved_2.to_le_bytes());
        for (at, scalar) in [(2, u32::try_from(size).map_err(|_| "native file overflow")?), (10, offset as u32), (14, 40), (18, doc.width), (22, if doc.top_down { (-(doc.height as i32)) as u32 } else { doc.height }), (30, if bitfields { 3 } else { 0 }), (34, raster_size as u32), (38, doc.x_pixels_per_meter as u32), (42, doc.y_pixels_per_meter as u32), (46, doc.colors_used), (50, doc.colors_important)] { out[at..at + 4].copy_from_slice(&scalar.to_le_bytes()); }
        out[26..28].copy_from_slice(&1u16.to_le_bytes()); out[28..30].copy_from_slice(&(bits as u16).to_le_bytes());
        let mut cursor = 54;
        if bitfields { for mask in &doc.masks[..3] { out[cursor..cursor + 4].copy_from_slice(&mask.to_le_bytes()); cursor += 4; } }
        for entry in palette { out[cursor..cursor + 4].copy_from_slice(&[entry[2], entry[1], entry[0], entry[3]]); cursor += 4; }
        out[cursor..offset].copy_from_slice(&doc.opaque_gap);
        for y in 0..doc.height as usize {
            let stored_y = if doc.top_down { y } else { doc.height as usize - 1 - y };
            for x in 0..doc.width as usize {
                let pixel = y * doc.width as usize + x;
                let at = offset + stored_y * stride + x * bits / 8;
                match &doc.content {
                    Content::Indexed { indices, .. } => { let shift = 8 - bits - x * bits % 8; out[at] |= indices[pixel] << shift; }
                    Content::Direct { samples } => {
                        let sample = samples[pixel];
                        let word = sample[..4].iter().zip(doc.masks).fold(sample[4], |word, (component, mask)| word | if mask == 0 { 0 } else { component << mask.trailing_zeros() });
                        out[at..at + bits / 8].copy_from_slice(&word.to_le_bytes()[..bits / 8]);
                    }
                }
            }
        }
        out[offset + raster_size..].copy_from_slice(&doc.opaque_trailer);
        image::load_from_memory_with_format(&out, image::ImageFormat::Bmp).map_err(|error| error.to_string())?;
        Ok(out)
    }

    pub fn revision(doc: &OracleDoc) -> String {
        let mut ordered = Vec::new();
        fn counted(out: &mut Vec<u8>, values: &[u8]) { out.extend_from_slice(&(values.len() as u64).to_le_bytes()); out.extend_from_slice(values); }
        counted(&mut ordered, b"stdio.bmp"); counted(&mut ordered, doc.profile.as_bytes()); ordered.push(u8::from(doc.top_down));
        for n in [doc.width, doc.height, doc.colors_used, doc.colors_important, doc.reserved_1.into(), doc.reserved_2.into()] { ordered.extend_from_slice(&n.to_le_bytes()); }
        ordered.extend_from_slice(&doc.x_pixels_per_meter.to_le_bytes()); ordered.extend_from_slice(&doc.y_pixels_per_meter.to_le_bytes());
        for mask in doc.masks { ordered.extend_from_slice(&mask.to_le_bytes()); }
        let palette = match &doc.content { Content::Indexed { palette, .. } => palette.as_slice(), _ => &[] };
        ordered.extend_from_slice(&(palette.len() as u64).to_le_bytes()); for entry in palette { ordered.extend_from_slice(entry); }
        match &doc.content {
            Content::Indexed { indices, .. } => { ordered.push(0); counted(&mut ordered, indices); }
            Content::Direct { samples } => { ordered.push(1); ordered.extend_from_slice(&(samples.len() as u64).to_le_bytes()); for sample in samples { for component in sample { ordered.extend_from_slice(&component.to_le_bytes()); } } }
        }
        counted(&mut ordered, &doc.opaque_gap); counted(&mut ordered, &doc.opaque_trailer);
        let hash = ordered.iter().fold(0xcbf29ce484222325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
        format!("{hash:016x}")
    }

    pub fn paint(doc: &mut OracleDoc, kind: &str, params: &Json) -> Result<(), String> {
        let [x, y, width, height]: [u32; 4] = ["x", "y", "width", "height"].map(|key| scalar(params, key)).into_iter().collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| "invalid region")?;
        if x.checked_add(width).is_none_or(|right| right > doc.width) || y.checked_add(height).is_none_or(|bottom| bottom > doc.height) { return Err("region exceeds owned dimensions".into()); }
        match (kind, &mut doc.content) {
            ("paint-indexed-region", Content::Indexed { indices, palette }) => {
                let index = scalar(params, "paletteIndex")?;
                if index as usize >= palette.len() { return Err("palette index exceeds owned palette".into()); }
                for row in y..y + height { let at = (row * doc.width + x) as usize; indices[at..at + width as usize].fill(index as u8); }
            }
            ("paint-direct-region", Content::Direct { samples }) => {
                let color = ["red", "green", "blue", "alpha"].map(|key| numeric(member(params, key)?, 0.0, 255.0).map(|n| n as u8)).into_iter().collect::<Result<Vec<_>, String>>()?;
                for row in y..y + height { for column in x..x + width { let sample = &mut samples[(row * doc.width + column) as usize]; for lane in 0..4 { sample[lane] = ((u64::from(color[lane]) * u64::from(maximum(doc.masks[lane])) + 127) / 255) as u32; } } }
            }
            _ => return Err("paint storage differs from owned profile".into())
        }
        Ok(())
    }

    pub fn apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        let params = member(spec, "params")?;
        let mut doc = decode(input)?;
        match kind.as_str() {
            "set-snapshot" => doc = OracleDoc::from_snapshot(member(params, "snapshot")?)?,
            "patch-snapshot" => { let patched = semio_repo_test_host::law::patched_snapshot(&doc.snapshot(), member(params, "patch")?)?; doc = OracleDoc::from_snapshot(&patched)?; }
            "paint-indexed-region" | "paint-direct-region" => { if params.str("revision") != revision(&doc) { return Err("stale owned revision".into()); } paint(&mut doc, &kind, params)?; }
            _ => return Err(format!("unknown BMP mutation {kind}"))
        }
        encode(&doc)
    }

    pub fn undo_mutation(original: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        decode(mutated)?;
        match spec.str("kind").as_str() { "set-snapshot" | "patch-snapshot" | "paint-indexed-region" | "paint-direct-region" => encode(&decode(original)?), _ => Err("unknown inverse".into()) }
    }

    pub fn project(input: &[u8]) -> Result<Json, String> {
        let doc = decode(input)?;
        let mut members = vec![
            ("format", text("bmp")), ("width", number(doc.width)), ("height", number(doc.height)), ("rowOrder", text(if doc.top_down { "topDown" } else { "bottomUp" })),
            ("profile", text(&doc.profile)), ("masks", integers(&doc.masks)), ("xPixelsPerMeter", number(doc.x_pixels_per_meter)), ("yPixelsPerMeter", number(doc.y_pixels_per_meter)),
            ("colorsUsed", number(doc.colors_used)), ("colorsImportant", number(doc.colors_important)), ("reserved1", number(doc.reserved_1)), ("reserved2", number(doc.reserved_2)),
            ("gapDigest", text(&digest(&doc.opaque_gap))), ("trailerDigest", text(&digest(&doc.opaque_trailer))), ("revision", text(&revision(&doc))),
        ];
        match &doc.content {
            Content::Indexed { indices, palette } => { let table: Vec<u8> = palette.iter().flatten().copied().collect(); members.extend([("storage", text("indexed")), ("paletteEntries", number(palette.len() as f64)), ("paletteDigest", text(&digest(&table))), ("indicesDigest", text(&digest(indices)))]); }
            Content::Direct { samples } => { let words: Vec<u8> = samples.iter().flatten().flat_map(|value| value.to_le_bytes()).collect(); members.extend([("storage", text("direct")), ("nativeSamplesDigest", text(&digest(&words)))]); }
        }
        let visual = image::load_from_memory_with_format(input, image::ImageFormat::Bmp).map_err(|error| error.to_string())?.to_rgba8().into_raw();
        members.push(("pixelsDigest", text(&digest(&visual))));
        Ok(object(members))
    }
}

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::apply_mutation(input, spec)
}

#[cfg(feature = "oracles")]
pub fn oracle_undo_mutation(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    oracles::undo_mutation(original_input, spec, mutated)
}

#[cfg(feature = "oracles")]
pub fn project_bmp_mutation(input: &[u8]) -> Result<Json, String> {
    oracles::project(input)
}

/// 🖼️ Independent image-rs visual meaning behind an owned test interface.
#[cfg(feature = "oracles")]
pub fn oracle_visual_rgba8(input: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let decoded = image::load_from_memory_with_format(input, image::ImageFormat::Bmp).map_err(|error| error.to_string())?.to_rgba8();
    Ok((decoded.width(), decoded.height(), decoded.into_raw()))
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_undo_mutation(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_bmp_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️FixtureDerivation
/// 🧫️ One-off real-world fixture derivation. NOT a test step — `#[ignore]`d, run once by hand, the
/// same convention as the TIFF subset's own `derive_real_world_fixture`. Builds the committed
/// `shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp` out of the real
/// `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png`: the
/// independent `png` decoder recovers that file's genuine 233-entry PLTE and its real index buffer,
/// and the `image` reference encoder writes both back as an 8-bit indexed BITMAPINFOHEADER.
///
/// The colour table is padded from its 233 real, all-referenced colours (index 0 alone covers
/// 5 659 668 of 5 975 040 pixels) to 240 entries, so a paint has table entries no pixel resolves to.
///
/// The spare colours are a deterministic ramp chosen at derivation time from values the real
/// palette does not already contain, and the derivation asserts that no pixel references any of
/// them — a padded table whose padding collided with a real colour would silently reintroduce
/// exactly the problem it exists to avoid.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️fixture-derivation/🦀️.rs"]
mod fixture_derivation;
//#endregion 🔖️FixtureDerivation

//#region RoundTrip
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> { oracles::encode(&oracles::decode(input)?) }
#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> { Err("the oracles feature is disabled".into()) }
//#endregion RoundTrip

#[cfg(all(test, feature = "oracles"))]
mod canonical_byte_authority {
    use image::ImageDecoder;

    fn accepted() -> [(&'static str, &'static [u8]); 9] {
        [
            ("direct-rgb24-padding-gap-trailer", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")),
            ("indexed-rgb1-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp")),
            ("indexed-rgb4-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")),
            ("indexed-rgb8-duplicate-palette", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp")),
            ("direct-rgb16-555", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb16-555.bmp")),
            ("direct-rgb32-reserved-sample", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb32-reserved-sample.bmp")),
            ("direct-bitfields16-565", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields16-565.bmp")),
            ("direct-bitfields32", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp")),
            ("direct-rgb24-top-down", include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-top-down.bmp")),
        ]
    }

    #[test]
    fn image_decoder_reopens_every_accepted_canonical_fixture() {
        for (id, source) in accepted() {
            let decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(source)).unwrap_or_else(|failure| panic!("{id}: {failure}"));
            let (width, height) = decoder.dimensions();
            assert!(width > 0 && height > 0, "{id}: dimensions");
            let decoded = image::load_from_memory(source).unwrap_or_else(|failure| panic!("{id}: {failure}")).to_rgba8();
            assert_eq!(decoded.len(), width as usize * height as usize * 4, "{id}: independent RGBA projection");
        }
    }

    #[test]
    fn image_decoder_reopens_neutral_opaque_rgba8_conversion() {
        let source = include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/rgba8-opaque-direct-rgb24.bmp");
        let decoded = image::load_from_memory(source).unwrap().to_rgba8();
        assert_eq!((decoded.width(), decoded.height()), (1, 1));
        assert_eq!(decoded.get_pixel(0, 0).0, [10, 20, 30, 255]);
    }

    #[test]
    fn image_decoder_preserves_eight_bit_duplicate_index_selection() {
        let source = include_bytes!("../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp");
        let mut decoder = image::codecs::bmp::BmpDecoder::new(std::io::Cursor::new(source)).unwrap();
        decoder.set_indexed_color(true);
        let (width, height) = decoder.dimensions();
        let mut indices = vec![0; width as usize * height as usize];
        decoder.read_image(&mut indices).unwrap();
        assert_eq!(indices, [2, 1, 0]);
    }

    #[test]
    fn independent_owned_native_models_preserve_every_precise_lane() {
        use super::oracles::{decode, encode, project};
        for (name, input) in accepted() {
            let original = decode(input).unwrap();
            let output = encode(&original).unwrap();
            let restored = decode(&output).unwrap();
            assert_eq!(restored.snapshot().to_string(), original.snapshot().to_string(), "{name}: complete owned model");
            assert_eq!(project(&output).unwrap(), project(input).unwrap(), "{name}: independent owned projection");
        }
    }

    #[test]
    fn neutral_owned_samples_and_paints_match_independent_native_reader() {
        use super::oracles::{decode, encode, paint, revision, Content, OracleDoc};
        let fixture = semio_repo_test_host::parse_json(include_str!("../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
        let semio_repo_test_host::Json::Array(cases) = fixture.get("cases").unwrap() else { panic!("cases"); };
        for case in cases {
            let snapshot = case.get("snapshot").unwrap();
            let mut document = OracleDoc::from_snapshot(snapshot).unwrap();
            let native = encode(&document).unwrap();
            let before = revision(&document);
            assert_eq!(before, case.str("expectedRevision"), "{}: exact authored owned revision", case.str("name"));
            assert_eq!(decode(&native).unwrap().snapshot(), document.snapshot(), "{}: owned sample fidelity", case.str("name"));
            let indexed = matches!(&document.content, Content::Indexed { .. });
            paint(&mut document, if indexed { "paint-indexed-region" } else { "paint-direct-region" }, case.get("paint").unwrap()).unwrap();
            assert_ne!(revision(&document), before, "{}: owned revision changes", case.str("name"));
            let painted = encode(&document).unwrap();
            let restored = decode(&painted).unwrap().snapshot();
            assert_eq!(restored, document.snapshot(), "{}: independently encoded paint", case.str("name"));
            let pixels = restored.get("image").unwrap().get("pixels").unwrap();
            let (key, expected_key) = if indexed { ("indices", "expectedIndices") } else { ("samples", "expectedSamples") };
            assert_eq!(pixels.get(key), case.get(expected_key), "{}: authored precise paint", case.str("name"));
        }
    }

    #[test]
    fn encoded_bytes_are_not_an_owned_snapshot_or_revision() {
        use super::oracles::{decode, OracleDoc};
        let source = accepted()[8].1;
        let old = semio_repo_test_host::parse_json(r#"{"schema":"stdio.bmp","bytes":[66,77]}"#).unwrap();
        assert!(OracleDoc::from_snapshot(&old).is_err());
        let doc = decode(source).unwrap();
        let native_hash = source.iter().fold(0xcbf29ce484222325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
        assert_ne!(super::oracles::revision(&doc), format!("{native_hash:016x}"));
    }
}
