//! 🌈️ PDF function domains, samples, nested colour spaces and exact optional ranges.
use super::*;

pub(super) fn write_reals(out: &mut Projection<'_, '_>, table: &str, owner: i64, role: &str, values: &[f64]) -> Result<(), String> {
    for (ordinal, value) in values.iter().enumerate() { out.insert(table, &[C::Integer(owner), C::Text(role), C::Integer(ordinal as i64), C::Real(*value)])?; }
    Ok(())
}

pub(super) fn read_reals(reader: &mut Reader<'_, '_, '_>, table: &'static str, owner: i64, role: &'static str) -> Result<Vec<f64>, String> {
    let mut values = Vec::new();
    for row in reader.children_with_role(table, 1, 3, owner, Some((2, role)))? { let row = reader.take(table, row.rowid, 5)?; values.push(row.real(4)?); }
    Ok(values)
}

pub(super) fn fixed<const N: usize>(values: Vec<f64>) -> Result<[f64; N], String> { values.try_into().map_err(|_| format!("PDF property requires exactly {N} components")) }

fn write_function_shallow(out: &mut Projection<'_, '_>, function: &PdfFunction) -> Result<i64, String> {
    let mut fields = [C::Null; 8];
    let kind = match function {
        PdfFunction::Sampled { encode, decode, bits_per_sample, order, samples, .. } => { fields[1] = C::Integer(i64::from(encode.is_some())); fields[2] = C::Integer(i64::from(decode.is_some())); fields[3] = C::Integer(i64::from(*bits_per_sample)); fields[4] = order.map_or(C::Null, |value| C::Integer(i64::from(value))); fields[6] = C::Blob(samples); "sampled" },
        PdfFunction::Exponential { range, n, .. } => { fields[0] = C::Integer(i64::from(range.is_some())); fields[5] = C::Real(*n); "exponential" },
        PdfFunction::Stitching { range, .. } => { fields[0] = C::Integer(i64::from(range.is_some())); "stitching" },
        PdfFunction::PostScript { code, .. } => { fields[7] = C::Text(code); "postScript" }, PdfFunction::Array { .. } => "array",
    };
    let mut cells = vec![C::Text(kind)]; cells.extend(fields); let key = out.insert("pdf_function", &cells)?;
    match function {
        PdfFunction::Sampled { domain, range, size, encode, decode, .. } => { write_reals(out, "pdf_function_real", key, "domain", domain)?; write_reals(out, "pdf_function_real", key, "range", range)?; for (ordinal, value) in size.iter().enumerate() { out.insert("pdf_function_size", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(*value))])?; } if let Some(values) = encode { write_reals(out, "pdf_function_real", key, "encode", values)?; } if let Some(values) = decode { write_reals(out, "pdf_function_real", key, "decode", values)?; } },
        PdfFunction::Exponential { domain, range, c0, c1, .. } => { write_reals(out, "pdf_function_real", key, "domain", domain)?; if let Some(values) = range { write_reals(out, "pdf_function_real", key, "range", values)?; } write_reals(out, "pdf_function_real", key, "c0", c0)?; write_reals(out, "pdf_function_real", key, "c1", c1)?; },
        PdfFunction::Stitching { domain, range, bounds, encode, .. } => { write_reals(out, "pdf_function_real", key, "domain", domain)?; if let Some(values) = range { write_reals(out, "pdf_function_real", key, "range", values)?; } write_reals(out, "pdf_function_real", key, "bounds", bounds)?; write_reals(out, "pdf_function_real", key, "encode", encode)?;  },
        PdfFunction::PostScript { domain, range, .. } => { write_reals(out, "pdf_function_real", key, "domain", domain)?; write_reals(out, "pdf_function_real", key, "range", range)?; },
        PdfFunction::Array { .. } => {},
    }
    Ok(key)
}

fn read_function_shallow(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfFunction, String> {
    let row = reader.take("pdf_function", key, 10)?;
    let kind = row.text(1)?; let present: &[usize] = match kind { "sampled" => &[3, 4, 5, 6, 8], "exponential" => &[2, 7], "stitching" => &[2], "postScript" => &[9], "array" => &[], _ => return Err("unknown PDF function kind".into()) };
    null_except(row, 2..10, present)?;
    Ok(match kind {
        "sampled" => { let mut size = Vec::new(); for value in reader.children("pdf_function_size", 1, 2, key)? { let value = reader.take("pdf_function_size", value.rowid, 4)?; size.push(integer(value, 3)?); } PdfFunction::Sampled { domain: read_reals(reader, "pdf_function_real", key, "domain")?, range: read_reals(reader, "pdf_function_real", key, "range")?, size, bits_per_sample: integer(row, 5)?, order: optional_integer(row, 6)?.map(|value| u32::try_from(value).map_err(|_| "PDF function order exceeds u32".to_string())).transpose()?, encode: if boolean(row, 3)? { Some(read_reals(reader, "pdf_function_real", key, "encode")?) } else { None }, decode: if boolean(row, 4)? { Some(read_reals(reader, "pdf_function_real", key, "decode")?) } else { None }, samples: reader.blob(row,8)? } },
        "exponential" => PdfFunction::Exponential { domain: read_reals(reader, "pdf_function_real", key, "domain")?, range: if boolean(row, 2)? { Some(read_reals(reader, "pdf_function_real", key, "range")?) } else { None }, c0: read_reals(reader, "pdf_function_real", key, "c0")?, c1: read_reals(reader, "pdf_function_real", key, "c1")?, n: row.real(7)? },
        "stitching" => PdfFunction::Stitching { domain: read_reals(reader, "pdf_function_real", key, "domain")?, range: if boolean(row, 2)? { Some(read_reals(reader, "pdf_function_real", key, "range")?) } else { None }, functions: Vec::new(), bounds: read_reals(reader, "pdf_function_real", key, "bounds")?, encode: read_reals(reader, "pdf_function_real", key, "encode")? },
        "postScript" => PdfFunction::PostScript { domain: read_reals(reader, "pdf_function_real", key, "domain")?, range: read_reals(reader, "pdf_function_real", key, "range")?, code: reader.text(row,9)? },
        "array" => PdfFunction::Array { functions: Vec::new() }, _ => unreachable!(),
    })
}

fn write_color_shallow(out: &mut Projection<'_, '_>, color: &PdfColorSpace, alternate:Option<i64>) -> Result<i64, String> {
    let mut fields = [C::Null; 12];
    let kind = match color {
        PdfColorSpace::DeviceGray => "deviceGray", PdfColorSpace::DeviceRgb => "deviceRgb", PdfColorSpace::DeviceCmyk => "deviceCmyk",
        PdfColorSpace::CalGray { black_point, gamma, .. } => { fields[9] = C::Integer(i64::from(black_point.is_some())); fields[10] = C::Integer(i64::from(gamma.is_some())); "calGray" },
        PdfColorSpace::CalRgb { black_point, gamma, matrix, .. } => { fields[9] = C::Integer(i64::from(black_point.is_some())); fields[10] = C::Integer(i64::from(gamma.is_some())); fields[11] = C::Integer(i64::from(matrix.is_some())); "calRgb" },
        PdfColorSpace::Lab { black_point, range, .. } => { fields[9] = C::Integer(i64::from(black_point.is_some())); fields[8] = C::Integer(i64::from(range.is_some())); "lab" },
        PdfColorSpace::IccBased { components, profile, range, .. } => { fields[0] = C::Integer(i64::from(*components)); fields[1] = C::Blob(profile); fields[5] = alternate.map_or(C::Null,C::Integer); fields[8] = C::Integer(i64::from(range.is_some())); "iccBased" },
        PdfColorSpace::Indexed { hival, lookup, .. } => { fields[5] = C::Integer(alternate.ok_or("PDF indexed colour has no base")?); fields[2] = C::Integer(i64::from(*hival)); fields[3] = C::Blob(lookup); "indexed" },
        PdfColorSpace::Separation { name, tint_transform, .. } => { fields[4] = C::Text(name); fields[5] = C::Integer(alternate.ok_or("PDF colour has no alternate")?); fields[6] = C::Integer(write_function(out, tint_transform)?); "separation" },
        PdfColorSpace::DeviceN { tint_transform, attributes, .. } => { fields[5] = C::Integer(alternate.ok_or("PDF colour has no alternate")?); fields[6] = C::Integer(write_function(out, tint_transform)?); if let Some(attributes) = attributes { fields[7] = C::Integer(cos::write_dictionary(out, attributes)?); } "deviceN" },
        PdfColorSpace::Pattern { .. } => { fields[5] = alternate.map_or(C::Null,C::Integer); "pattern" }, PdfColorSpace::Named { name } => { fields[4] = C::Text(name); "named" },
    };
    let mut cells = vec![C::Text(kind)]; cells.extend(fields); let key = out.insert("pdf_color_space", &cells)?;
    match color {
        PdfColorSpace::CalGray { white_point, black_point, gamma } => { write_reals(out, "pdf_color_real", key, "whitePoint", white_point)?; if let Some(values) = black_point { write_reals(out, "pdf_color_real", key, "blackPoint", values)?; } if let Some(value) = gamma { write_reals(out, "pdf_color_real", key, "gamma", &[*value])?; } },
        PdfColorSpace::CalRgb { white_point, black_point, gamma, matrix } => { write_reals(out, "pdf_color_real", key, "whitePoint", white_point)?; if let Some(values) = black_point { write_reals(out, "pdf_color_real", key, "blackPoint", values)?; } if let Some(values) = gamma { write_reals(out, "pdf_color_real", key, "gamma", values)?; } if let Some(values) = matrix { write_reals(out, "pdf_color_real", key, "matrix", values)?; } },
        PdfColorSpace::Lab { white_point, black_point, range } => { write_reals(out, "pdf_color_real", key, "whitePoint", white_point)?; if let Some(values) = black_point { write_reals(out, "pdf_color_real", key, "blackPoint", values)?; } if let Some(values) = range { write_reals(out, "pdf_color_real", key, "range", values)?; } },
        PdfColorSpace::IccBased { range, .. } => { if let Some(values) = range { write_reals(out, "pdf_color_real", key, "range", values)?; } },
        PdfColorSpace::DeviceN { names, .. } => { for (ordinal, name) in names.iter().enumerate() { out.insert("pdf_color_name", &[C::Integer(key), C::Integer(ordinal as i64), C::Text(name)])?; } }, _ => {},
    }
    Ok(key)
}

fn read_color_shallow(reader: &mut Reader<'_, '_, '_>, row:Row<'_>, alternate:Option<PdfColorSpace>) -> Result<PdfColorSpace, String> {
    let key=row.rowid; let kind = row.text(1)?;
    let present: &[usize] = match kind { "deviceGray" | "deviceRgb" | "deviceCmyk" => &[], "calGray" => &[11, 12], "calRgb" => &[11, 12, 13], "lab" => &[10, 11], "iccBased" => &[2, 3, 7, 10], "indexed" => &[4, 5, 7], "separation" => &[6, 7, 8], "deviceN" => &[7, 8, 9], "pattern" => &[7], "named" => &[6], _ => return Err("unknown PDF colour space kind".into()) };
    null_except(row, 2..14, present)?;
    Ok(match kind {
        "deviceGray" => PdfColorSpace::DeviceGray, "deviceRgb" => PdfColorSpace::DeviceRgb, "deviceCmyk" => PdfColorSpace::DeviceCmyk,
        "calGray" => PdfColorSpace::CalGray { white_point: fixed(read_reals(reader, "pdf_color_real", key, "whitePoint")?)?, black_point: if boolean(row, 11)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "blackPoint")?)?) } else { None }, gamma: if boolean(row, 12)? { Some(fixed::<1>(read_reals(reader, "pdf_color_real", key, "gamma")?)?[0]) } else { None } },
        "calRgb" => PdfColorSpace::CalRgb { white_point: fixed(read_reals(reader, "pdf_color_real", key, "whitePoint")?)?, black_point: if boolean(row, 11)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "blackPoint")?)?) } else { None }, gamma: if boolean(row, 12)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "gamma")?)?) } else { None }, matrix: if boolean(row, 13)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "matrix")?)?) } else { None } },
        "lab" => PdfColorSpace::Lab { white_point: fixed(read_reals(reader, "pdf_color_real", key, "whitePoint")?)?, black_point: if boolean(row, 11)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "blackPoint")?)?) } else { None }, range: if boolean(row, 10)? { Some(fixed(read_reals(reader, "pdf_color_real", key, "range")?)?) } else { None } },
        "iccBased" => PdfColorSpace::IccBased { components: integer(row, 2)?, profile: reader.blob(row,3)?, alternate: alternate.map(Box::new), range: if boolean(row, 10)? { Some(read_reals(reader, "pdf_color_real", key, "range")?) } else { None } },
        "indexed" => PdfColorSpace::Indexed { base: Box::new(alternate.ok_or("PDF colour has no declared base or alternate")?), hival: integer(row, 4)?, lookup: reader.blob(row,5)? },
        "separation" => PdfColorSpace::Separation { name: reader.text(row,6)?, alternate: Box::new(alternate.ok_or("PDF colour has no declared base or alternate")?), tint_transform: read_function(reader, row.integer(8)?)? },
        "deviceN" => { let mut names = Vec::new(); for name in reader.children("pdf_color_name", 1, 2, key)? { let name = reader.take("pdf_color_name", name.rowid, 4)?; names.push(reader.text(name,3)?); } PdfColorSpace::DeviceN { names, alternate: Box::new(alternate.ok_or("PDF colour has no declared base or alternate")?), tint_transform: read_function(reader, row.integer(8)?)?, attributes: optional_integer(row, 9)?.map(|value| cos::read_dictionary(reader, value)).transpose()? } },
        "pattern" => PdfColorSpace::Pattern { base: alternate.map(Box::new) }, "named" => PdfColorSpace::Named { name: reader.text(row,6)? }, _ => unreachable!(),
    })
}

fn function_children(value:&PdfFunction)->&[PdfFunction]{match value{PdfFunction::Array{functions}|PdfFunction::Stitching{functions,..}=>functions,_=>&[]}}
pub(super) fn write_function(out:&mut Projection<'_,'_>,function:&PdfFunction)->Result<i64,String>{
    let key=write_function_shallow(out,function)?;let mut pending=vec![(key,function_children(function).iter().enumerate())];
    while let Some((parent,children))=pending.last_mut(){if let Some((ordinal,child))=children.next(){let parent=*parent;let key=write_function_shallow(out,child)?;out.insert("pdf_function_child",&[C::Integer(parent),C::Integer(ordinal as i64),C::Integer(key)])?;pending.push((key,function_children(child).iter().enumerate()));}else{pending.pop();}}Ok(key)
}
pub(super) fn read_function(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfFunction,String>{
    let function=read_function_shallow(reader,key)?;let children=reader.children("pdf_function_child",1,2,key)?.into_iter();let mut pending=vec![(function,children)];
    loop{let child=pending.last_mut().and_then(|(_,children)|children.next());if let Some(child)=child{let child=reader.take("pdf_function_child",child.rowid,4)?;let key=child.integer(3)?;let function=read_function_shallow(reader,key)?;let children=reader.children("pdf_function_child",1,2,key)?.into_iter();pending.push((function,children));}else{let(function,_)=pending.pop().ok_or("PDF function stack is empty")?;if let Some((parent,_))=pending.last_mut(){match parent{PdfFunction::Array{functions}|PdfFunction::Stitching{functions,..}=>functions.push(function),_=>return Err("PDF scalar function cannot own a child function".into())}}else{return Ok(function);}}}
}
fn color_alternate(value:&PdfColorSpace)->Option<&PdfColorSpace>{match value{PdfColorSpace::IccBased{alternate,..}=>alternate.as_deref(),PdfColorSpace::Indexed{base,..}=>Some(base),PdfColorSpace::Separation{alternate,..}|PdfColorSpace::DeviceN{alternate,..}=>Some(alternate),PdfColorSpace::Pattern{base}=>base.as_deref(),_=>None}}
pub(super) fn write_color(out:&mut Projection<'_,'_>,color:&PdfColorSpace)->Result<i64,String>{
    let mut pending=Vec::new();let mut node=color;loop{out.check_rows(pending.len().checked_add(1).ok_or("PDF colour depth row count overflow")?)?;out.checkpoint()?;pending.push(node);if let Some(child)=color_alternate(node){node=child;}else{break;}}
    let mut alternate=None;while let Some(node)=pending.pop(){alternate=Some(write_color_shallow(out,node,alternate)?);}alternate.ok_or_else(||"PDF colour projection stack is empty".into())
}
pub(super) fn read_color(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfColorSpace,String>{
    let mut pending=Vec::new();let mut key=key;loop{let row=reader.take("pdf_color_space",key,14)?;let child=optional_integer(row,7)?;pending.push(row);if let Some(child)=child{key=child;}else{break;}}
    let mut alternate=None;while let Some(row)=pending.pop(){alternate=Some(read_color_shallow(reader,row,alternate)?);}alternate.ok_or_else(||"PDF colour reconstruction stack is empty".into())
}
