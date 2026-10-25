//! 🧵️ Collection triples of the DXF diff codec: the name-keyed and index-keyed removed, modified and added triple codecs and the collection diff pair codecs. See [`super`].

use super::*;

/// 🧮 Binary twins of [`enc_name_triple`]/[`dec_name_triple`] and [`enc_index_triple`]/
/// [`dec_index_triple`] above — varint-counted lists instead of `;`-separated bracket sections,
/// same removed/modified/added shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_name_triple_bin<T, D>(removed: &[String], modified: &[(String, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for name in removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (name, d) in modified {
        write_str_lp(out, name);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_name_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<NamedDiffParts<D, T>, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(read_str_lp(reader)?);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let name = read_str_lp(reader)?;
        let d = dec_diff(reader)?;
        modified.push((name, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_index_triple_bin<T, D>(removed: &[usize], modified: &[(usize, D)], added: &[(usize, T)], out: &mut Vec<u8>, enc_diff: impl Fn(&D, &mut Vec<u8>), enc_item: impl Fn(&T, &mut Vec<u8>)) {
    store::pack_rt::write_varint_u64(out, removed.len() as u64);
    for idx in removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, modified.len() as u64);
    for (idx, d) in modified {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_diff(d, out);
    }
    store::pack_rt::write_varint_u64(out, added.len() as u64);
    for (idx, item) in added {
        store::pack_rt::write_varint_u64(out, *idx as u64);
        enc_item(item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_index_triple_bin<T, D>(reader: &mut store::ByteReader<'_>, dec_diff: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>, dec_item: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<IndexedDiffParts<D, T>, String> {
    let rc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(rc as usize);
    for _ in 0..rc {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let mc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(mc as usize);
    for _ in 0..mc {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let d = dec_diff(reader)?;
        modified.push((idx, d));
    }
    let ac = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(ac as usize);
    for _ in 0..ac {
        let idx = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_item(reader)?;
        added.push((idx, item));
    }
    Ok((removed, modified, added))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_header_vars_diff_bin(d: &DxfHeaderVarsDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfHeaderVarDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfHeaderVar)> = d.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_header_var_diff_bin, enc_header_var_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_header_vars_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfHeaderVarsDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_header_var_diff_bin, dec_header_var_bin)?;
    Ok(DxfHeaderVarsDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_layers_diff_bin(d: &DxfLayersDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLayerDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLayer)> = d.added.iter().map(|a| (a.index, a.layer.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_layer_diff_bin, enc_layer_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_layers_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLayersDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_layer_diff_bin, dec_layer_bin)?;
    Ok(DxfLayersDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_styles_diff_bin(d: &DxfStylesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfStyleDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfStyle)> = d.added.iter().map(|a| (a.index, a.style.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_style_diff_bin, enc_style_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_styles_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfStylesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_style_diff_bin, dec_style_bin)?;
    Ok(DxfStylesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_linetypes_diff_bin(d: &DxfLinetypesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(String, DxfLinetypeDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
    let added: Vec<(usize, DxfLinetype)> = d.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
    enc_name_triple_bin(&d.removed, &modified, &added, out, enc_linetype_diff_bin, enc_linetype_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_linetypes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfLinetypesDiff, String> {
    let (removed, modified, added) = dec_name_triple_bin(reader, dec_linetype_diff_bin, dec_linetype_bin)?;
    Ok(DxfLinetypesDiff { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_entities_diff_bin(d: &DxfEntitiesDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfEntityDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfEntity)> = d.added.iter().map(|a| (a.index, a.entity.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_entity_diff_bin, enc_dxf_entity_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfEntitiesDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_entity_diff_bin, dec_dxf_entity_bin)?;
    Ok(DxfEntitiesDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_blocks_diff_bin(d: &DxfBlocksDiff, out: &mut Vec<u8>) {
    let modified: Vec<(usize, DxfBlockDiff)> = d.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let added: Vec<(usize, DxfBlock)> = d.added.iter().map(|a| (a.index, a.block.clone())).collect();
    enc_index_triple_bin(&d.removed, &modified, &added, out, enc_block_diff_bin, enc_block_bin);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<DxfBlocksDiff, String> {
    let (removed, modified, added) = dec_index_triple_bin(reader, dec_block_diff_bin, dec_block_bin)?;
    Ok(DxfBlocksDiff { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() })
}
