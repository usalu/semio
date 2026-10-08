//! 🪪️ Explicit relational persistence for identity-bound semantic stream roles.
use super::*;
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue};

fn invalid() -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, "PDF stream role fields differ from the declared semantic variant") }
fn write_identity(out: &mut Projection<'_, '_>, value: &PdfGraphIdentity) -> Result<i64, ValueError> {
    let key = out.insert("pdf_graph_identity", &[C::Integer(i64::from(value.owner.num)), C::Integer(i64::from(value.owner.gen))])?;
    for (ordinal, part) in value.path.iter().enumerate() {
        let (kind, name, index) = match part { PdfGraphPath::Entry { key } => ("entry", C::Text(key), C::Null), PdfGraphPath::Item { index } => ("item", C::Null, C::Integer(i64::try_from(*index).map_err(|_| invalid())?)) };
        out.insert("pdf_graph_path", &[C::Integer(key), C::Integer(ordinal as i64), C::Text(kind), name, index])?;
    }
    Ok(key)
}
fn read_identity(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfGraphIdentity, ValueError> {
    let row = reader.take("pdf_graph_identity", key, 3)?;
    let owner = ObjRef { num: integer(row, 1)?, gen: integer(row, 2)? };
    let mut path = Vec::new();
    for child in reader.children("pdf_graph_path", 1, 2, key)? {
        let row = reader.take("pdf_graph_path", child.rowid, 6)?;
        path.push(match row.text(3)? { "entry" if row.values[5] == V::Null => PdfGraphPath::Entry { key: reader.text(row, 4)? }, "item" if row.values[4] == V::Null => PdfGraphPath::Item { index: integer(row, 5)? }, _ => return Err(invalid()) });
    }
    Ok(PdfGraphIdentity { owner, path })
}
pub(super) fn write_role(out: &mut Projection<'_, '_>, role: &PdfAdmittedStreamRole) -> Result<i64, ValueError> {
    let identity = write_identity(out, &role.identity)?;
    let mut cells = [C::Null; 14];
    cells[0] = C::Integer(identity);
    cells[1] = C::Text(match &role.value {
        PdfStreamRoleValue::Operators { content } => { cells[2] = C::Integer(content::write_ops(out, content)?); "operators" },
        PdfStreamRoleValue::SampledWords { samples } => { cells[3] = C::Integer(i64::try_from(samples.len()).map_err(|_| invalid())?); "sampledWords" },
        PdfStreamRoleValue::CalculatorProgram { code } => { cells[4] = C::Text(code); "calculatorProgram" },
        PdfStreamRoleValue::UnicodeMap { mapping } => { cells[5] = C::Integer(font::write_unicode(out, mapping)?); "unicodeMap" },
        PdfStreamRoleValue::CharacterMap { cmap } => { cells[6] = C::Integer(font::write_cmap(out, &PdfCMap::Embedded { cmap: cmap.clone() })?); "characterMap" },
        PdfStreamRoleValue::FontProgram { program } => { cells[7] = C::Integer(font::write_program(out, program)?); "fontProgram" },
        PdfStreamRoleValue::Image { image } => { cells[8] = C::Integer(resource::write_image(out, image)?); "image" },
        PdfStreamRoleValue::MetadataText { text } => { cells[9] = C::Text(text); "metadataText" },
        PdfStreamRoleValue::AttachmentBytes { bytes } => { cells[10] = C::Blob(bytes); "attachmentBytes" },
        PdfStreamRoleValue::PaletteComponents { components } => { cells[11] = C::Blob(components); "paletteComponents" },
        PdfStreamRoleValue::GlyphIds { glyphs } => { cells[12] = C::Integer(glyphs.len() as i64); "glyphIds" },
        PdfStreamRoleValue::ReferenceBody { reference } => { cells[13] = C::Integer(artifact_reference::write(out,reference)?); "referenceBody" },
    });
    let key = out.insert("pdf_stream_role", &cells)?;
    if let PdfStreamRoleValue::SampledWords { samples } = &role.value { for (ordinal, word) in samples.iter().enumerate() { out.insert("pdf_stream_role_sample", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(*word))])?; } }
    if let PdfStreamRoleValue::GlyphIds { glyphs } = &role.value { for (ordinal, glyph) in glyphs.iter().enumerate() { out.insert("pdf_stream_role_glyph", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(*glyph))])?; } }
    for (ordinal, dependency) in role.dependencies.iter().enumerate() { let identity = write_identity(out, dependency)?; out.insert("pdf_stream_role_dependency", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(identity)])?; }
    Ok(key)
}
pub(super) fn read_role(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfAdmittedStreamRole, ValueError> {
    let row = reader.take("pdf_stream_role", key, 15)?;
    let kind = row.text(2)?;
    let column = match kind { "operators" => 3, "sampledWords" => 4, "calculatorProgram" => 5, "unicodeMap" => 6, "characterMap" => 7, "fontProgram" => 8, "image" => 9, "metadataText" => 10, "attachmentBytes" => 11, "paletteComponents" => 12, "glyphIds" => 13, "referenceBody" => 14, _ => return Err(invalid()) };
    if row.values[column] == V::Null || (3..15).any(|other| other != column && row.values[other] != V::Null) { return Err(invalid()); }
    let value = match kind {
        "operators" => PdfStreamRoleValue::Operators { content: content::read_ops(reader, row.integer(3)?)? },
        "sampledWords" => {
            let children = reader.children("pdf_stream_role_sample", 1, 2, key)?;
            if children.len() != integer::<usize>(row, 4)? { return Err(invalid()); }
            let mut samples = Vec::new();
            for child in children { let row = reader.take("pdf_stream_role_sample", child.rowid, 4)?; samples.push(integer(row, 3)?); }
            PdfStreamRoleValue::SampledWords { samples }
        },
        "calculatorProgram" => PdfStreamRoleValue::CalculatorProgram { code: reader.text(row, 5)? },
        "unicodeMap" => PdfStreamRoleValue::UnicodeMap { mapping: font::read_unicode(reader, row.integer(6)?)? },
        "characterMap" => match font::read_cmap(reader, row.integer(7)?)? { PdfCMap::Embedded { cmap } => PdfStreamRoleValue::CharacterMap { cmap }, _ => return Err(invalid()) },
        "fontProgram" => PdfStreamRoleValue::FontProgram { program: font::read_program(reader, row.integer(8)?)? },
        "image" => PdfStreamRoleValue::Image { image: resource::read_image(reader, row.integer(9)?)? },
        "metadataText" => PdfStreamRoleValue::MetadataText { text: reader.text(row, 10)? },
        "attachmentBytes" => PdfStreamRoleValue::AttachmentBytes { bytes: reader.blob(row, 11)? },
        "paletteComponents" => PdfStreamRoleValue::PaletteComponents { components: reader.blob(row,12)? },
        "referenceBody" => PdfStreamRoleValue::ReferenceBody { reference: artifact_reference::read(reader,row.integer(14)?)? },
        "glyphIds" => { let children=reader.children("pdf_stream_role_glyph",1,2,key)?;if children.len()!=integer::<usize>(row,13)? {return Err(invalid());}let mut glyphs=Vec::new();for child in children{let row=reader.take("pdf_stream_role_glyph",child.rowid,4)?;glyphs.push(integer(row,3)?);}PdfStreamRoleValue::GlyphIds {glyphs}},
        _ => return Err(invalid()),
    };
    let identity = read_identity(reader, row.integer(1)?)?;
    let mut dependencies = Vec::new();
    for child in reader.children("pdf_stream_role_dependency", 1, 2, key)? { let row = reader.take("pdf_stream_role_dependency", child.rowid, 4)?; dependencies.push(read_identity(reader, row.integer(3)?)?); }
    Ok(PdfAdmittedStreamRole { identity, dependencies, value })
}
