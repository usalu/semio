//! 🎨 Colour (ISO 32000-1 §8.6), functions (§7.10), shadings and patterns (§8.7) and extended
//! graphics states (§8.4.5): lifting each COS shape into its typed twin and lowering it back.
//! Lowering allocates indirect objects through an [`ObjectSink`] wherever the spec requires a
//! stream (sampled/PostScript functions, ICC profiles, mesh shadings, tiling patterns).

use super::lexer::{dict_f64, dict_get, dict_i64, dict_name};
use super::xref::{ObjectSink, ObjectSource};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfDictEntry, PdfExtGState, PdfFunction, PdfLineCap, PdfLineJoin, PdfMatrix, PdfObject, PdfRect, PdfShading, PdfShadingKind, PdfSoftMask};

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn numbers_of(value: Option<&PdfObject>) -> Vec<f64> {
    value.and_then(PdfObject::as_array).map(|items| items.iter().filter_map(PdfObject::as_f64).collect()).unwrap_or_default()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn array_n<const N: usize>(value: Option<&PdfObject>) -> Option<[f64; N]> {
    let values = numbers_of(value);
    (values.len() >= N).then(|| {
        let mut out = [0.0; N];
        out.copy_from_slice(&values[..N]);
        out
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn rect_of(value: Option<&PdfObject>) -> Option<PdfRect> {
    array_n::<4>(value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn matrix_of(value: Option<&PdfObject>) -> Option<PdfMatrix> {
    array_n::<6>(value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn entry(key: &str, value: PdfObject) -> PdfDictEntry {
    PdfDictEntry::new(key, value)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn push_opt(entries: &mut Vec<PdfDictEntry>, key: &str, value: Option<PdfObject>) {
    if let Some(value) = value {
        entries.push(entry(key, value));
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stream(dict: Vec<PdfDictEntry>, data: Vec<u8>) -> PdfObject {
    PdfObject::Stream { dict, data, filters: vec![crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter::Flate { predictor: None }] }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn raw_stream(dict: Vec<PdfDictEntry>, data: Vec<u8>) -> PdfObject {
    PdfObject::Stream { dict, data, filters: Vec::new() }
}
/// 🧹 Entries of `dict` not in `known` — the lossless `extra` of a typed dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extra_entries(dict: &[PdfDictEntry], known: &[&str]) -> Vec<PdfDictEntry> {
    dict.iter().filter(|entry| !known.contains(&entry.key.as_str())).cloned().collect()
}
//#endregion 🔖️Helpers

//#region 🔖️Functions
/// ⬇️ Lifts a function object (dictionary, stream, reference or array of functions).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_function(value: &PdfObject, source: &mut dyn ObjectSource) -> Option<PdfFunction> {
    let resolved = source.deref(value);
    if let PdfObject::Array(items) = &resolved {
        let functions: Vec<PdfFunction> = items.iter().filter_map(|item| lift_function(item, source)).collect();
        return Some(PdfFunction::Array { functions });
    }
    let dict = resolved.as_dict()?;
    let domain = numbers_of(dict_get(dict, "Domain"));
    let range = numbers_of(dict_get(dict, "Range"));
    match dict_i64(dict, "FunctionType")? {
        0 => {
            let PdfObject::Stream { data, .. } = &resolved else { return None };
            Some(PdfFunction::Sampled {
                domain,
                range,
                size: numbers_of(dict_get(dict, "Size")).iter().map(|v| *v as u32).collect(),
                bits_per_sample: dict_i64(dict, "BitsPerSample").unwrap_or(8) as u32,
                order: dict_i64(dict, "Order").map(|v| v as u32),
                encode: dict_get(dict, "Encode").map(|v| numbers_of(Some(v))),
                decode: dict_get(dict, "Decode").map(|v| numbers_of(Some(v))),
                samples: data.clone(),
            })
        }
        2 => Some(PdfFunction::Exponential { domain, range: dict_get(dict, "Range").map(|v| numbers_of(Some(v))), c0: dict_get(dict, "C0").map(|v| numbers_of(Some(v))).unwrap_or_else(|| vec![0.0]), c1: dict_get(dict, "C1").map(|v| numbers_of(Some(v))).unwrap_or_else(|| vec![1.0]), n: dict_f64(dict, "N").unwrap_or(1.0) }),
        3 => {
            let functions = dict_get(dict, "Functions").map(|v| source.deref(v)).and_then(|v| v.as_array().map(|items| items.iter().filter_map(|item| lift_function(item, source)).collect())).unwrap_or_default();
            Some(PdfFunction::Stitching { domain, range: dict_get(dict, "Range").map(|v| numbers_of(Some(v))), functions, bounds: numbers_of(dict_get(dict, "Bounds")), encode: numbers_of(dict_get(dict, "Encode")) })
        }
        4 => {
            let PdfObject::Stream { data, .. } = &resolved else { return None };
            Some(PdfFunction::PostScript { domain, range, code: String::from_utf8_lossy(data).into_owned() })
        }
        _ => None,
    }
}

/// ⬆️ Lowers a function to the object a `/Function` entry holds (stream-based ones become
/// indirect).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_function(function: &PdfFunction, sink: &mut dyn ObjectSink) -> PdfObject {
    match function {
        PdfFunction::Sampled { domain, range, size, bits_per_sample, order, encode, decode, samples } => {
            let mut dict = vec![entry("FunctionType", PdfObject::Int(0)), entry("Domain", PdfObject::numbers(domain)), entry("Range", PdfObject::numbers(range)), entry("Size", PdfObject::Array(size.iter().map(|v| PdfObject::Int(*v as i64)).collect())), entry("BitsPerSample", PdfObject::Int(*bits_per_sample as i64))];
            push_opt(&mut dict, "Order", order.map(|v| PdfObject::Int(v as i64)));
            push_opt(&mut dict, "Encode", encode.as_ref().map(|v| PdfObject::numbers(v)));
            push_opt(&mut dict, "Decode", decode.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Ref(sink.add(stream(dict, samples.clone())))
        }
        PdfFunction::Exponential { domain, range, c0, c1, n } => {
            let mut dict = vec![entry("FunctionType", PdfObject::Int(2)), entry("Domain", PdfObject::numbers(domain)), entry("C0", PdfObject::numbers(c0)), entry("C1", PdfObject::numbers(c1)), entry("N", PdfObject::number(*n))];
            push_opt(&mut dict, "Range", range.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Dict(dict)
        }
        PdfFunction::Stitching { domain, range, functions, bounds, encode } => {
            let mut dict = vec![entry("FunctionType", PdfObject::Int(3)), entry("Domain", PdfObject::numbers(domain)), entry("Functions", PdfObject::Array(functions.iter().map(|f| lower_function(f, sink)).collect())), entry("Bounds", PdfObject::numbers(bounds)), entry("Encode", PdfObject::numbers(encode))];
            push_opt(&mut dict, "Range", range.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Dict(dict)
        }
        PdfFunction::PostScript { domain, range, code } => {
            let dict = vec![entry("FunctionType", PdfObject::Int(4)), entry("Domain", PdfObject::numbers(domain)), entry("Range", PdfObject::numbers(range))];
            PdfObject::Ref(sink.add(stream(dict, code.as_bytes().to_vec())))
        }
        PdfFunction::Array { functions } => PdfObject::Array(functions.iter().map(|f| lower_function(f, sink)).collect()),
    }
}
//#endregion 🔖️Functions

//#region 🔖️ColourSpaces
/// ⬇️ Lifts a colour-space object: a family name, or the `[/Family …]` array (§8.6.3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_colour_space(value: &PdfObject, source: &mut dyn ObjectSource) -> PdfColorSpace {
    let resolved = source.deref(value);
    match &resolved {
        PdfObject::Name(name) => match name.as_str() {
            "DeviceGray" | "G" | "CalGray" => PdfColorSpace::DeviceGray,
            "DeviceRGB" | "RGB" | "CalRGB" => PdfColorSpace::DeviceRgb,
            "DeviceCMYK" | "CMYK" => PdfColorSpace::DeviceCmyk,
            "Pattern" => PdfColorSpace::Pattern { base: None },
            other => PdfColorSpace::Named { name: other.to_string() },
        },
        PdfObject::Array(items) => {
            let family = items.first().and_then(PdfObject::as_name).unwrap_or("");
            let param = items.get(1).map(|v| source.deref(v));
            let dict = param.as_ref().and_then(PdfObject::as_dict);
            match family {
                "DeviceGray" | "G" => PdfColorSpace::DeviceGray,
                "DeviceRGB" | "RGB" => PdfColorSpace::DeviceRgb,
                "DeviceCMYK" | "CMYK" => PdfColorSpace::DeviceCmyk,
                "CalGray" => PdfColorSpace::CalGray { white_point: dict.and_then(|d| array_n::<3>(dict_get(d, "WhitePoint"))).unwrap_or([0.9505, 1.0, 1.089]), black_point: dict.and_then(|d| array_n::<3>(dict_get(d, "BlackPoint"))), gamma: dict.and_then(|d| dict_f64(d, "Gamma")) },
                "CalRGB" => PdfColorSpace::CalRgb { white_point: dict.and_then(|d| array_n::<3>(dict_get(d, "WhitePoint"))).unwrap_or([0.9505, 1.0, 1.089]), black_point: dict.and_then(|d| array_n::<3>(dict_get(d, "BlackPoint"))), gamma: dict.and_then(|d| array_n::<3>(dict_get(d, "Gamma"))), matrix: dict.and_then(|d| array_n::<9>(dict_get(d, "Matrix"))) },
                "Lab" => PdfColorSpace::Lab { white_point: dict.and_then(|d| array_n::<3>(dict_get(d, "WhitePoint"))).unwrap_or([0.9505, 1.0, 1.089]), black_point: dict.and_then(|d| array_n::<3>(dict_get(d, "BlackPoint"))), range: dict.and_then(|d| array_n::<4>(dict_get(d, "Range"))) },
                "ICCBased" => {
                    let (profile, dict_entries) = match &param {
                        Some(PdfObject::Stream { dict, data, .. }) => (data.clone(), dict.clone()),
                        _ => (Vec::new(), Vec::new()),
                    };
                    PdfColorSpace::IccBased { components: dict_i64(&dict_entries, "N").unwrap_or(3) as u32, profile, alternate: dict_get(&dict_entries, "Alternate").map(|alt| Box::new(lift_colour_space(alt, source))), range: dict_get(&dict_entries, "Range").map(|v| numbers_of(Some(v))) }
                }
                "Indexed" | "I" => {
                    let base = items.get(1).map(|b| lift_colour_space(b, source)).unwrap_or(PdfColorSpace::DeviceRgb);
                    let hival = items.get(2).and_then(PdfObject::as_i64).unwrap_or(0).max(0) as u32;
                    let lookup = match items.get(3).map(|v| source.deref(v)) {
                        Some(PdfObject::Str(bytes)) => bytes,
                        Some(PdfObject::Stream { data, .. }) => data,
                        _ => Vec::new(),
                    };
                    PdfColorSpace::Indexed { base: Box::new(base), hival, lookup }
                }
                "Separation" => PdfColorSpace::Separation { name: items.get(1).and_then(PdfObject::as_name).unwrap_or("All").to_string(), alternate: Box::new(items.get(2).map(|a| lift_colour_space(a, source)).unwrap_or(PdfColorSpace::DeviceGray)), tint_transform: items.get(3).and_then(|f| lift_function(f, source)).unwrap_or(PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![1.0], c1: vec![0.0], n: 1.0 }) },
                "DeviceN" => {
                    let names = items.get(1).map(|v| source.deref(v)).and_then(|v| v.as_array().map(|items| items.iter().filter_map(PdfObject::as_name).map(str::to_string).collect())).unwrap_or_default();
                    PdfColorSpace::DeviceN { names, alternate: Box::new(items.get(2).map(|a| lift_colour_space(a, source)).unwrap_or(PdfColorSpace::DeviceGray)), tint_transform: items.get(3).and_then(|f| lift_function(f, source)).unwrap_or(PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![1.0], c1: vec![0.0], n: 1.0 }), attributes: items.get(4).map(|v| source.deref(v)).and_then(|v| v.as_dict().map(<[PdfDictEntry]>::to_vec)) }
                }
                "Pattern" => PdfColorSpace::Pattern { base: items.get(1).map(|b| Box::new(lift_colour_space(b, source))) },
                other => PdfColorSpace::Named { name: other.to_string() },
            }
        }
        _ => PdfColorSpace::DeviceGray,
    }
}

/// ⬇️ Lifts an inline-image colour space (only names, and `[/Indexed …]` arrays, are legal).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_colour_space_inline(value: &PdfObject) -> PdfColorSpace {
    struct NoSource;
    impl ObjectSource for NoSource {
        fn get(&mut self, _reference: crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef) -> Option<PdfObject> {
            None
        }
    }
    lift_colour_space(value, &mut NoSource)
}

/// ⬆️ Lowers a colour space to its COS form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_colour_space(space: &PdfColorSpace, sink: &mut dyn ObjectSink) -> PdfObject {
    let cal = |white: &[f64; 3], black: &Option<[f64; 3]>| -> Vec<PdfDictEntry> {
        let mut dict = vec![entry("WhitePoint", PdfObject::numbers(white))];
        push_opt(&mut dict, "BlackPoint", black.as_ref().map(|v| PdfObject::numbers(v)));
        dict
    };
    match space {
        PdfColorSpace::DeviceGray => PdfObject::name("DeviceGray"),
        PdfColorSpace::DeviceRgb => PdfObject::name("DeviceRGB"),
        PdfColorSpace::DeviceCmyk => PdfObject::name("DeviceCMYK"),
        PdfColorSpace::CalGray { white_point, black_point, gamma } => {
            let mut dict = cal(white_point, black_point);
            push_opt(&mut dict, "Gamma", gamma.map(PdfObject::number));
            PdfObject::Array(vec![PdfObject::name("CalGray"), PdfObject::Dict(dict)])
        }
        PdfColorSpace::CalRgb { white_point, black_point, gamma, matrix } => {
            let mut dict = cal(white_point, black_point);
            push_opt(&mut dict, "Gamma", gamma.as_ref().map(|v| PdfObject::numbers(v)));
            push_opt(&mut dict, "Matrix", matrix.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Array(vec![PdfObject::name("CalRGB"), PdfObject::Dict(dict)])
        }
        PdfColorSpace::Lab { white_point, black_point, range } => {
            let mut dict = cal(white_point, black_point);
            push_opt(&mut dict, "Range", range.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Array(vec![PdfObject::name("Lab"), PdfObject::Dict(dict)])
        }
        PdfColorSpace::IccBased { components, profile, alternate, range } => {
            let mut dict = vec![entry("N", PdfObject::Int(*components as i64))];
            push_opt(&mut dict, "Alternate", alternate.as_ref().map(|alt| lower_colour_space(alt, sink)));
            push_opt(&mut dict, "Range", range.as_ref().map(|v| PdfObject::numbers(v)));
            PdfObject::Array(vec![PdfObject::name("ICCBased"), PdfObject::Ref(sink.add(stream(dict, profile.clone())))])
        }
        PdfColorSpace::Indexed { base, hival, lookup } => PdfObject::Array(vec![PdfObject::name("Indexed"), lower_colour_space(base, sink), PdfObject::Int(*hival as i64), PdfObject::Str(lookup.clone())]),
        PdfColorSpace::Separation { name, alternate, tint_transform } => PdfObject::Array(vec![PdfObject::name("Separation"), PdfObject::name(name), lower_colour_space(alternate, sink), lower_function(tint_transform, sink)]),
        PdfColorSpace::DeviceN { names, alternate, tint_transform, attributes } => {
            let mut items = vec![PdfObject::name("DeviceN"), PdfObject::Array(names.iter().map(PdfObject::name).collect()), lower_colour_space(alternate, sink), lower_function(tint_transform, sink)];
            if let Some(attributes) = attributes {
                items.push(PdfObject::Dict(attributes.clone()));
            }
            PdfObject::Array(items)
        }
        PdfColorSpace::Pattern { base: None } => PdfObject::name("Pattern"),
        PdfColorSpace::Pattern { base: Some(base) } => PdfObject::Array(vec![PdfObject::name("Pattern"), lower_colour_space(base, sink)]),
        PdfColorSpace::Named { name } => PdfObject::name(name),
    }
}

/// ⬆️ Lowers an inline-image colour space (no indirect objects possible).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_colour_space_inline(space: &PdfColorSpace) -> PdfObject {
    struct Inline;
    impl ObjectSink for Inline {
        fn add(&mut self, _value: PdfObject) -> crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef {
            crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef { num: 0, gen: 0 }
        }
    }
    match space {
        PdfColorSpace::DeviceGray => PdfObject::name("G"),
        PdfColorSpace::DeviceRgb => PdfObject::name("RGB"),
        PdfColorSpace::DeviceCmyk => PdfObject::name("CMYK"),
        PdfColorSpace::Indexed { base, hival, lookup } => PdfObject::Array(vec![PdfObject::name("I"), lower_colour_space_inline(base), PdfObject::Int(*hival as i64), PdfObject::Str(lookup.clone())]),
        other => lower_colour_space(other, &mut Inline),
    }
}
//#endregion 🔖️ColourSpaces

//#region 🔖️Shadings
/// ⬇️ Lifts a shading dictionary or stream (§8.7.4.3).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_shading(id: &str, value: &PdfObject, source: &mut dyn ObjectSource) -> Option<PdfShading> {
    let resolved = source.deref(value);
    let dict = resolved.as_dict()?.to_vec();
    let shading_type = dict_i64(&dict, "ShadingType")? as u32;
    let color_space = dict_get(&dict, "ColorSpace").map(|cs| lift_colour_space(cs, source)).unwrap_or(PdfColorSpace::DeviceRgb);
    let function = dict_get(&dict, "Function").and_then(|f| lift_function(f, source));
    let extend = dict_get(&dict, "Extend").and_then(PdfObject::as_array).map(|items| [items.first().and_then(PdfObject::as_bool).unwrap_or(false), items.get(1).and_then(PdfObject::as_bool).unwrap_or(false)]).unwrap_or([false, false]);
    let kind = match shading_type {
        1 => PdfShadingKind::FunctionBased { domain: array_n::<4>(dict_get(&dict, "Domain")), matrix: matrix_of(dict_get(&dict, "Matrix")), function: function? },
        2 => PdfShadingKind::Axial { coords: array_n::<4>(dict_get(&dict, "Coords"))?, domain: array_n::<2>(dict_get(&dict, "Domain")), function: function?, extend },
        3 => PdfShadingKind::Radial { coords: array_n::<6>(dict_get(&dict, "Coords"))?, domain: array_n::<2>(dict_get(&dict, "Domain")), function: function?, extend },
        4..=7 => {
            let PdfObject::Stream { data, .. } = &resolved else { return None };
            PdfShadingKind::Mesh { shading_type, bits_per_coordinate: dict_i64(&dict, "BitsPerCoordinate").unwrap_or(16) as u32, bits_per_component: dict_i64(&dict, "BitsPerComponent").unwrap_or(16) as u32, bits_per_flag: dict_i64(&dict, "BitsPerFlag").map(|v| v as u32), vertices_per_row: dict_i64(&dict, "VerticesPerRow").map(|v| v as u32), decode: numbers_of(dict_get(&dict, "Decode")), function, data: data.clone() }
        }
        _ => return None,
    };
    Some(PdfShading { id: id.to_string(), color_space, kind, background: dict_get(&dict, "Background").map(|v| numbers_of(Some(v))), bbox: rect_of(dict_get(&dict, "BBox")), anti_alias: dict_get(&dict, "AntiAlias").and_then(PdfObject::as_bool).unwrap_or(false), extra: extra_entries(&dict, &["ShadingType", "ColorSpace", "Function", "Extend", "Domain", "Matrix", "Coords", "BitsPerCoordinate", "BitsPerComponent", "BitsPerFlag", "VerticesPerRow", "Decode", "Background", "BBox", "AntiAlias", "Length"]) })
}

/// ⬆️ Lowers a shading to an indirect object and returns its reference.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_shading(shading: &PdfShading, sink: &mut dyn ObjectSink) -> PdfObject {
    let mut dict = vec![entry("ColorSpace", lower_colour_space(&shading.color_space, sink))];
    let extend_of = |extend: &[bool; 2]| PdfObject::Array(vec![PdfObject::Bool(extend[0]), PdfObject::Bool(extend[1])]);
    let object = match &shading.kind {
        PdfShadingKind::FunctionBased { domain, matrix, function } => {
            dict.insert(0, entry("ShadingType", PdfObject::Int(1)));
            push_opt(&mut dict, "Domain", domain.as_ref().map(|v| PdfObject::numbers(v)));
            push_opt(&mut dict, "Matrix", matrix.as_ref().map(|v| PdfObject::numbers(v)));
            dict.push(entry("Function", lower_function(function, sink)));
            None
        }
        PdfShadingKind::Axial { coords, domain, function, extend } => {
            dict.insert(0, entry("ShadingType", PdfObject::Int(2)));
            dict.push(entry("Coords", PdfObject::numbers(coords)));
            push_opt(&mut dict, "Domain", domain.as_ref().map(|v| PdfObject::numbers(v)));
            dict.push(entry("Function", lower_function(function, sink)));
            if *extend != [false, false] {
                dict.push(entry("Extend", extend_of(extend)));
            }
            None
        }
        PdfShadingKind::Radial { coords, domain, function, extend } => {
            dict.insert(0, entry("ShadingType", PdfObject::Int(3)));
            dict.push(entry("Coords", PdfObject::numbers(coords)));
            push_opt(&mut dict, "Domain", domain.as_ref().map(|v| PdfObject::numbers(v)));
            dict.push(entry("Function", lower_function(function, sink)));
            if *extend != [false, false] {
                dict.push(entry("Extend", extend_of(extend)));
            }
            None
        }
        PdfShadingKind::Mesh { shading_type, bits_per_coordinate, bits_per_component, bits_per_flag, vertices_per_row, decode, function, data } => {
            dict.insert(0, entry("ShadingType", PdfObject::Int(*shading_type as i64)));
            dict.push(entry("BitsPerCoordinate", PdfObject::Int(*bits_per_coordinate as i64)));
            dict.push(entry("BitsPerComponent", PdfObject::Int(*bits_per_component as i64)));
            push_opt(&mut dict, "BitsPerFlag", bits_per_flag.map(|v| PdfObject::Int(v as i64)));
            push_opt(&mut dict, "VerticesPerRow", vertices_per_row.map(|v| PdfObject::Int(v as i64)));
            dict.push(entry("Decode", PdfObject::numbers(decode)));
            push_opt(&mut dict, "Function", function.as_ref().map(|f| lower_function(f, sink)));
            Some(data.clone())
        }
    };
    push_opt(&mut dict, "Background", shading.background.as_ref().map(|v| PdfObject::numbers(v)));
    push_opt(&mut dict, "BBox", shading.bbox.as_ref().map(|v| PdfObject::numbers(v)));
    if shading.anti_alias {
        dict.push(entry("AntiAlias", PdfObject::Bool(true)));
    }
    dict.extend(shading.extra.iter().cloned());
    match object {
        Some(data) => PdfObject::Ref(sink.add(stream(dict, data))),
        None => PdfObject::Ref(sink.add(PdfObject::Dict(dict))),
    }
}
//#endregion 🔖️Shadings

//#region 🔖️ExtGState
/// ⬇️ Lifts an extended graphics state dictionary (§8.4.5). Soft-mask groups are reported by
/// the object reference of their form XObject so the caller can bind them to a form id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_ext_g_state(id: &str, dict: &[PdfDictEntry], source: &mut dyn ObjectSource, form_id_of: &mut dyn FnMut(&PdfObject, &mut dyn ObjectSource) -> Option<String>, font_id_of: &mut dyn FnMut(&PdfObject, &mut dyn ObjectSource) -> Option<String>) -> PdfExtGState {
    let mut state = PdfExtGState { id: id.to_string(), ..PdfExtGState::default() };
    state.line_width = dict_f64(dict, "LW");
    state.line_cap = dict_i64(dict, "LC").map(|v| match v {
        1 => PdfLineCap::Round,
        2 => PdfLineCap::Square,
        _ => PdfLineCap::Butt,
    });
    state.line_join = dict_i64(dict, "LJ").map(|v| match v {
        1 => PdfLineJoin::Round,
        2 => PdfLineJoin::Bevel,
        _ => PdfLineJoin::Miter,
    });
    state.miter_limit = dict_f64(dict, "ML");
    state.dash = dict_get(dict, "D").and_then(PdfObject::as_array).map(|items| (numbers_of(items.first()), items.get(1).and_then(PdfObject::as_f64).unwrap_or(0.0)));
    state.rendering_intent = dict_name(dict, "RI").map(str::to_string);
    state.overprint_stroke = dict_get(dict, "OP").and_then(PdfObject::as_bool);
    state.overprint_fill = dict_get(dict, "op").and_then(PdfObject::as_bool).or(state.overprint_stroke);
    state.overprint_mode = dict_i64(dict, "OPM").map(|v| v as u32);
    state.font = dict_get(dict, "Font").and_then(PdfObject::as_array).and_then(|items| Some((font_id_of(items.first()?, source)?, items.get(1).and_then(PdfObject::as_f64).unwrap_or(0.0))));
    state.blend_mode = dict_get(dict, "BM").map(|v| match v {
        PdfObject::Name(name) => vec![name.clone()],
        PdfObject::Array(items) => items.iter().filter_map(PdfObject::as_name).map(str::to_string).collect(),
        _ => Vec::new(),
    });
    state.soft_mask = dict_get(dict, "SMask").map(|value| {
        let resolved = source.deref(value);
        match resolved.as_dict() {
            Some(mask) => {
                let group = dict_get(mask, "G").and_then(|g| form_id_of(g, source)).unwrap_or_default();
                let transfer = dict_get(mask, "TR").filter(|tr| !matches!(tr, PdfObject::Name(_))).and_then(|tr| lift_function(tr, source));
                if dict_name(mask, "S") == Some("Luminosity") {
                    PdfSoftMask::Luminosity { group, backdrop: dict_get(mask, "BC").map(|v| numbers_of(Some(v))), transfer }
                } else {
                    PdfSoftMask::Alpha { group, transfer }
                }
            }
            None => PdfSoftMask::None,
        }
    });
    state.stroke_alpha = dict_f64(dict, "CA");
    state.fill_alpha = dict_f64(dict, "ca");
    state.alpha_is_shape = dict_get(dict, "AIS").and_then(PdfObject::as_bool);
    state.stroke_adjust = dict_get(dict, "SA").and_then(PdfObject::as_bool);
    state.flatness = dict_f64(dict, "FL");
    state.smoothness = dict_f64(dict, "SM");
    state.text_knockout = dict_get(dict, "TK").and_then(PdfObject::as_bool);
    state.extra = extra_entries(dict, &["Type", "LW", "LC", "LJ", "ML", "D", "RI", "OP", "op", "OPM", "Font", "BM", "SMask", "CA", "ca", "AIS", "SA", "FL", "SM", "TK"]);
    state
}

/// ⬆️ Lowers an extended graphics state to its dictionary; `form_ref_of`/`font_ref_of` resolve
/// the ids the typed state names.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lower_ext_g_state(state: &PdfExtGState, sink: &mut dyn ObjectSink, form_ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>, font_ref_of: &mut dyn FnMut(&str) -> Option<PdfObject>) -> PdfObject {
    let mut dict = vec![entry("Type", PdfObject::name("ExtGState"))];
    push_opt(&mut dict, "LW", state.line_width.map(PdfObject::number));
    push_opt(&mut dict, "LC", state.line_cap.map(|v| PdfObject::Int(v as u8 as i64)));
    push_opt(&mut dict, "LJ", state.line_join.map(|v| PdfObject::Int(v as u8 as i64)));
    push_opt(&mut dict, "ML", state.miter_limit.map(PdfObject::number));
    push_opt(&mut dict, "D", state.dash.as_ref().map(|(array, phase)| PdfObject::Array(vec![PdfObject::numbers(array), PdfObject::number(*phase)])));
    push_opt(&mut dict, "RI", state.rendering_intent.as_ref().map(PdfObject::name));
    push_opt(&mut dict, "OP", state.overprint_stroke.map(PdfObject::Bool));
    push_opt(&mut dict, "op", state.overprint_fill.map(PdfObject::Bool));
    push_opt(&mut dict, "OPM", state.overprint_mode.map(|v| PdfObject::Int(v as i64)));
    if let Some((font, size)) = &state.font {
        if let Some(reference) = font_ref_of(font) {
            dict.push(entry("Font", PdfObject::Array(vec![reference, PdfObject::number(*size)])));
        }
    }
    push_opt(&mut dict, "BM", state.blend_mode.as_ref().map(|modes| if modes.len() == 1 { PdfObject::name(&modes[0]) } else { PdfObject::Array(modes.iter().map(PdfObject::name).collect()) }));
    if let Some(mask) = &state.soft_mask {
        let value = match mask {
            PdfSoftMask::None => PdfObject::name("None"),
            PdfSoftMask::Alpha { group, transfer } => {
                let mut mask_dict = vec![entry("Type", PdfObject::name("Mask")), entry("S", PdfObject::name("Alpha"))];
                push_opt(&mut mask_dict, "G", form_ref_of(group));
                push_opt(&mut mask_dict, "TR", transfer.as_ref().map(|f| lower_function(f, sink)));
                PdfObject::Dict(mask_dict)
            }
            PdfSoftMask::Luminosity { group, backdrop, transfer } => {
                let mut mask_dict = vec![entry("Type", PdfObject::name("Mask")), entry("S", PdfObject::name("Luminosity"))];
                push_opt(&mut mask_dict, "G", form_ref_of(group));
                push_opt(&mut mask_dict, "BC", backdrop.as_ref().map(|v| PdfObject::numbers(v)));
                push_opt(&mut mask_dict, "TR", transfer.as_ref().map(|f| lower_function(f, sink)));
                PdfObject::Dict(mask_dict)
            }
        };
        dict.push(entry("SMask", value));
    }
    push_opt(&mut dict, "CA", state.stroke_alpha.map(PdfObject::number));
    push_opt(&mut dict, "ca", state.fill_alpha.map(PdfObject::number));
    push_opt(&mut dict, "AIS", state.alpha_is_shape.map(PdfObject::Bool));
    push_opt(&mut dict, "SA", state.stroke_adjust.map(PdfObject::Bool));
    push_opt(&mut dict, "FL", state.flatness.map(PdfObject::number));
    push_opt(&mut dict, "SM", state.smoothness.map(PdfObject::number));
    push_opt(&mut dict, "TK", state.text_knockout.map(PdfObject::Bool));
    dict.extend(state.extra.iter().cloned());
    PdfObject::Dict(dict)
}
//#endregion 🔖️ExtGState

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
