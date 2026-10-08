//! 🎨️ Pure dictionary projections over admitted colour/function roles.
use super::*;
pub fn numbers_of(value: Option<&PdfObject>) -> Vec<f64> {
    value.and_then(PdfObject::as_array).map(|items| items.iter().filter_map(PdfObject::as_f64).collect()).unwrap_or_default()
}

pub fn array_n<const N: usize>(value: Option<&PdfObject>) -> Option<[f64; N]> {
    let values = numbers_of(value);
    (values.len() >= N).then(|| {
        let mut out = [0.0; N];
        out.copy_from_slice(&values[..N]);
        out
    })
}

pub fn rect_of(value: Option<&PdfObject>) -> Option<PdfRect> {
    array_n::<4>(value)
}

pub fn matrix_of(value: Option<&PdfObject>) -> Option<PdfMatrix> {
    array_n::<6>(value)
}

pub fn extra_entries(dict: &[PdfDictEntry], known: &[&str]) -> Vec<PdfDictEntry> {
    dict.iter().filter(|entry| !known.contains(&entry.key.as_str())).cloned().collect()
}

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
            let PdfObject::Stream { .. } = &resolved else { return None };
            let size:Vec<u32>=dict_get(dict,"Size")?.as_array()?.iter().map(|value|u32::try_from(value.as_i64()?).ok().filter(|value|*value>0)).collect::<Option<_>>()?;
            let bits_per_sample=u32::try_from(dict_i64(dict,"BitsPerSample")?).ok()?;if range.is_empty()||range.len()%2!=0{return None;}let count=size.iter().try_fold(range.len()/2,|count,size|count.checked_mul(*size as usize))?;
            let samples = match source.semantic_role(value, PdfStreamRoleKind::SampledWords)? { PdfStreamRoleValue::SampledWords { samples } => samples, _ => return None };
            if samples.len() != count { return None; }
            Some(PdfFunction::Sampled {
                domain,
                range,
                size,
                bits_per_sample,
                order: dict_i64(dict, "Order").map(|v| v as u32),
                encode: dict_get(dict, "Encode").map(|v| numbers_of(Some(v))),
                decode: dict_get(dict, "Decode").map(|v| numbers_of(Some(v))),
                samples,
            })
        }
        2 => Some(PdfFunction::Exponential { domain, range: dict_get(dict, "Range").map(|v| numbers_of(Some(v))), c0: dict_get(dict, "C0").map(|v| numbers_of(Some(v))).unwrap_or_else(|| vec![0.0]), c1: dict_get(dict, "C1").map(|v| numbers_of(Some(v))).unwrap_or_else(|| vec![1.0]), n: dict_f64(dict, "N").unwrap_or(1.0) }),
        3 => {
            let functions = dict_get(dict, "Functions").map(|v| source.deref(v)).and_then(|v| v.as_array().map(|items| items.iter().filter_map(|item| lift_function(item, source)).collect())).unwrap_or_default();
            Some(PdfFunction::Stitching { domain, range: dict_get(dict, "Range").map(|v| numbers_of(Some(v))), functions, bounds: numbers_of(dict_get(dict, "Bounds")), encode: numbers_of(dict_get(dict, "Encode")) })
        }
        4 => {
            let PdfObject::Stream { .. } = &resolved else { return None };
            let code = match source.semantic_role(value, PdfStreamRoleKind::CalculatorProgram)? { PdfStreamRoleValue::CalculatorProgram { code } => code, _ => return None };
            Some(PdfFunction::PostScript { domain, range, code })
        }
        _ => None,
    }
}

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
                    let Some(PdfObject::Stream { dict: dict_entries, .. }) = &param else { return PdfColorSpace::DeviceRgb; };
                    let profile = match source.semantic_role(items.get(1).unwrap_or(&PdfObject::Null), PdfStreamRoleKind::ReferenceBody) { Some(PdfStreamRoleValue::ReferenceBody { reference }) => reference, _ => return PdfColorSpace::DeviceRgb };
                    PdfColorSpace::IccBased { components: dict_i64(&dict_entries, "N").unwrap_or(3) as u32, profile, alternate: dict_get(&dict_entries, "Alternate").map(|alt| Box::new(lift_colour_space(alt, source))), range: dict_get(&dict_entries, "Range").map(|v| numbers_of(Some(v))) }
                }
                "Indexed" | "I" => {
                    let base = items.get(1).map(|b| lift_colour_space(b, source)).unwrap_or(PdfColorSpace::DeviceRgb);
                    let hival = items.get(2).and_then(PdfObject::as_i64).unwrap_or(0).max(0) as u32;
                    let palette = match items.get(3).map(|v| source.deref(v)) {
                        Some(PdfObject::Str(_)) | Some(PdfObject::Stream { .. }) => match source.semantic_role(items.get(3).unwrap_or(&PdfObject::Null), PdfStreamRoleKind::PaletteComponents) { Some(PdfStreamRoleValue::PaletteComponents { components }) => components, _ => Vec::new() },
                        _ => Vec::new(),
                    };
                    PdfColorSpace::Indexed { base: Box::new(base), hival, palette }
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
            let PdfObject::Stream { .. } = &resolved else { return None };
            let reference = match source.semantic_role(value, PdfStreamRoleKind::ReferenceBody) { Some(PdfStreamRoleValue::ReferenceBody { reference }) => reference, _ => return None };
            PdfShadingKind::Mesh { shading_type, bits_per_coordinate: dict_i64(&dict, "BitsPerCoordinate").unwrap_or(16) as u32, bits_per_component: dict_i64(&dict, "BitsPerComponent").unwrap_or(16) as u32, bits_per_flag: dict_i64(&dict, "BitsPerFlag").map(|v| v as u32), vertices_per_row: dict_i64(&dict, "VerticesPerRow").map(|v| v as u32), decode: numbers_of(dict_get(&dict, "Decode")), function, reference }
        }
        _ => return None,
    };
    Some(PdfShading { id: id.to_string(), color_space, kind, background: dict_get(&dict, "Background").map(|v| numbers_of(Some(v))), bbox: rect_of(dict_get(&dict, "BBox")), anti_alias: dict_get(&dict, "AntiAlias").and_then(PdfObject::as_bool).unwrap_or(false), extra: extra_entries(&dict, &["ShadingType", "ColorSpace", "Function", "Extend", "Domain", "Matrix", "Coords", "BitsPerCoordinate", "BitsPerComponent", "BitsPerFlag", "VerticesPerRow", "Decode", "Background", "BBox", "AntiAlias", "Length"]) })
}

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
