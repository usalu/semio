use super::*;
use std::collections::BTreeMap;
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{Cell, Projection}, validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};

fn ordinal(index: usize) -> Result<i64, String> { i64::try_from(index).map_err(|error| error.to_string()) }
fn byte(row: &SqliteRow, index: usize) -> Result<u8, String> { u8::try_from(row.integer(index)?).map_err(|error| error.to_string()) }
fn word(row: &SqliteRow, index: usize) -> Result<u16, String> { u16::try_from(row.integer(index)?).map_err(|error| error.to_string()) }
fn unsigned(row: &SqliteRow, index: usize) -> Result<u32, String> { u32::try_from(row.integer(index)?).map_err(|error| error.to_string()) }
fn boolean(row: &SqliteRow, index: usize) -> Result<bool, String> { match row.integer(index)? { 0 => Ok(false), 1 => Ok(true), _ => Err("PNG boolean must be zero or one".into()) } }
fn nulls(row: &SqliteRow, indices: &[usize]) -> Result<(), String> { if indices.iter().all(|index| matches!(row.values.get(*index), Some(SqliteValue::Null))) { Ok(()) } else { Err("PNG inactive variant columns must be NULL".into()) } }
fn singleton<'a>(database: &'a SqliteDatabase, name: &str) -> Result<Option<&'a SqliteRow>, String> { let rows = &database.table(name)?.rows; if rows.is_empty() { return Ok(None); } let row = database.table(name)?.single_row()?; if row.rowid != 1 || row.integer(0)? != 1 || row.integer(1)? != 1 { return Err(format!("{name} must own document 1 with identity 1")); } Ok(Some(row)) }
fn entities<'a>(database: &'a SqliteDatabase, name: &str) -> Result<Vec<&'a SqliteRow>, String> { let rows = database.table(name)?.ordered_rows(2)?; for row in &rows { if row.integer(1)? != 1 || row.integer(0)? != row.rowid || row.rowid <= 0 { return Err(format!("{name} has invalid ownership or identity")); } } Ok(rows) }
fn pixel_count(width: u32, height: u32) -> Result<usize, String> { usize::try_from(u64::from(width) * u64::from(height)).map_err(|error| error.to_string()) }

impl ArtifactSqliteSnapshot for PngSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        let count = pixel_count(self.width, self.height)?;
        if self.pixels.len() != count.checked_mul(4).ok_or("PNG pixel size overflow")? { return Err("PNG pixels must contain exactly one RGBA tuple per grid position".into()); }
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        out.insert("png_document", &[Cell::Text(&self.schema), Cell::Integer(i64::from(self.width)), Cell::Integer(i64::from(self.height)), Cell::Integer(i64::from(self.bit_depth)), Cell::Integer(i64::from(self.color_type.to_u8())), Cell::Integer(i64::from(self.interlace))])?;
        if let Some(palette) = &self.plte {
            out.insert("png_palette", &[Cell::Integer(1)])?;
            for (index, entry) in palette.iter().enumerate() { out.insert("png_palette_entry", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(entry.r)), Cell::Integer(i64::from(entry.g)), Cell::Integer(i64::from(entry.b))])?; }
        }
        if let Some(transparency) = &self.trns {
            match transparency {
                PngTransparency::Indexed { alpha } => { out.insert("png_transparency", &[Cell::Integer(1), Cell::Text("indexed"), Cell::Null, Cell::Null, Cell::Null, Cell::Null])?; for (index, value) in alpha.iter().enumerate() { out.insert("png_transparency_alpha", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(*value))])?; } },
                PngTransparency::Grayscale { gray } => { out.insert("png_transparency", &[Cell::Integer(1), Cell::Text("grayscale"), Cell::Integer(i64::from(*gray)), Cell::Null, Cell::Null, Cell::Null])?; },
                PngTransparency::Rgb { r, g, b } => { out.insert("png_transparency", &[Cell::Integer(1), Cell::Text("rgb"), Cell::Null, Cell::Integer(i64::from(*r)), Cell::Integer(i64::from(*g)), Cell::Integer(i64::from(*b))])?; },
            }
        }
        if let Some(value) = self.gama { out.insert("png_gamma", &[Cell::Integer(1), Cell::Integer(i64::from(value))])?; }
        if let Some(value) = &self.chrm { out.insert("png_chromaticity", &[Cell::Integer(1), Cell::Integer(i64::from(value.white_x)), Cell::Integer(i64::from(value.white_y)), Cell::Integer(i64::from(value.red_x)), Cell::Integer(i64::from(value.red_y)), Cell::Integer(i64::from(value.green_x)), Cell::Integer(i64::from(value.green_y)), Cell::Integer(i64::from(value.blue_x)), Cell::Integer(i64::from(value.blue_y))])?; }
        if let Some(value) = self.srgb { out.insert("png_srgb", &[Cell::Integer(1), Cell::Integer(i64::from(value.to_u8()))])?; }
        if let Some(value) = &self.phys { out.insert("png_physical_dimensions", &[Cell::Integer(1), Cell::Integer(i64::from(value.ppu_x)), Cell::Integer(i64::from(value.ppu_y)), Cell::Integer(i64::from(value.unit_is_meter))])?; }
        if let Some(value) = &self.time { out.insert("png_modification_time", &[Cell::Integer(1), Cell::Integer(i64::from(value.year)), Cell::Integer(i64::from(value.month)), Cell::Integer(i64::from(value.day)), Cell::Integer(i64::from(value.hour)), Cell::Integer(i64::from(value.minute)), Cell::Integer(i64::from(value.second))])?; }
        if let Some(background) = &self.bkgd {
            match background {
                PngBackground::Indexed { index } => { out.insert("png_background", &[Cell::Integer(1), Cell::Text("indexed"), Cell::Null, Cell::Null, Cell::Null, Cell::Null, Cell::Integer(i64::from(*index))])?; },
                PngBackground::Grayscale { gray } => { out.insert("png_background", &[Cell::Integer(1), Cell::Text("grayscale"), Cell::Integer(i64::from(*gray)), Cell::Null, Cell::Null, Cell::Null, Cell::Null])?; },
                PngBackground::Rgb { r, g, b } => { out.insert("png_background", &[Cell::Integer(1), Cell::Text("rgb"), Cell::Null, Cell::Integer(i64::from(*r)), Cell::Integer(i64::from(*g)), Cell::Integer(i64::from(*b)), Cell::Null])?; },
            }
        }
        for (index, text) in self.text_chunks.iter().enumerate() { let kind = match text.kind { PngTextKind::Text => "text", PngTextKind::ZText => "ztext", PngTextKind::IText => "itext" }; out.insert("png_text", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Text(&text.keyword), Cell::Text(&text.value), Cell::Integer(i64::from(text.compressed)), Cell::Text(kind), Cell::Text(&text.language_tag), Cell::Text(&text.translated_keyword)])?; }
        for (index, rgba) in self.pixels.chunks_exact(4).enumerate() { let width = usize::try_from(self.width).map_err(|error| error.to_string())?; out.insert("png_pixel", &[Cell::Integer(1), Cell::Integer(ordinal(index % width)?), Cell::Integer(ordinal(index / width)?), Cell::Integer(i64::from(rgba[0])), Cell::Integer(i64::from(rgba[1])), Cell::Integer(i64::from(rgba[2])), Cell::Integer(i64::from(rgba[3]))])?; }
        for (index, chunk) in self.unknown_chunks.iter().enumerate() {
            let id = out.insert("png_unknown_chunk", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(chunk.kind[0])), Cell::Integer(i64::from(chunk.kind[1])), Cell::Integer(i64::from(chunk.kind[2])), Cell::Integer(i64::from(chunk.kind[3]))])?;
            for (index, value) in chunk.data.iter().enumerate() { out.insert("png_unknown_chunk_byte", &[Cell::Integer(id), Cell::Integer(ordinal(index)?), Cell::Integer(i64::from(*value))])?; }
        }
        for (index, marker) in self.chunk_order.iter().enumerate() {
            let (kind, text, unknown) = match marker {
                PngChunkMarker::Ihdr => ("ihdr", Cell::Null, Cell::Null), PngChunkMarker::Plte => ("plte", Cell::Null, Cell::Null), PngChunkMarker::Trns => ("trns", Cell::Null, Cell::Null), PngChunkMarker::Gama => ("gama", Cell::Null, Cell::Null), PngChunkMarker::Chrm => ("chrm", Cell::Null, Cell::Null), PngChunkMarker::Srgb => ("srgb", Cell::Null, Cell::Null), PngChunkMarker::Phys => ("phys", Cell::Null, Cell::Null), PngChunkMarker::Time => ("time", Cell::Null, Cell::Null), PngChunkMarker::Bkgd => ("bkgd", Cell::Null, Cell::Null), PngChunkMarker::Idat => ("idat", Cell::Null, Cell::Null), PngChunkMarker::Iend => ("iend", Cell::Null, Cell::Null),
                PngChunkMarker::Text { index } => { if *index >= self.text_chunks.len() { return Err("PNG chunk sequence references unknown text".into()); } ("text", Cell::Integer(ordinal(*index)?.checked_add(1).ok_or("PNG text identifier overflow")?), Cell::Null) },
                PngChunkMarker::Unknown { index } => { if *index >= self.unknown_chunks.len() { return Err("PNG chunk sequence references unknown chunk".into()); } ("unknown", Cell::Null, Cell::Integer(ordinal(*index)?.checked_add(1).ok_or("PNG chunk identifier overflow")?)) },
            };
            out.insert("png_chunk_sequence", &[Cell::Integer(1), Cell::Integer(ordinal(index)?), Cell::Text(kind), text, unknown])?;
        }
        out.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("png_document")?.single_row()?;
        if document.rowid != 1 || document.integer(0)? != 1 { return Err("PNG document identifier must be 1".into()); }
        let mut result = Self { schema: document.text(1)?.into(), width: unsigned(document, 2)?, height: unsigned(document, 3)?, bit_depth: byte(document, 4)?, color_type: PngColorType::from_u8(byte(document, 5)?)?, interlace: boolean(document, 6)?, chunk_order: Vec::new(), ..Self::default() };
        let mut completed = 0usize;
        let total = database.tables.iter().map(|table| table.rows.len()).sum();
        let mut tick = |control: &mut SqliteSnapshotControl<'_>| -> Result<(), String> { if completed % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, completed, total)?; } completed += 1; Ok(()) };
        let palette = singleton(database, "png_palette")?;
        let palette_rows = entities(database, "png_palette_entry")?;
        if palette.is_none() && !palette_rows.is_empty() { return Err("PNG palette entries require a palette".into()); }
        if palette.is_some() { let mut entries = Vec::new(); for row in palette_rows { tick(control)?; entries.push(PngRgb { r: byte(row, 3)?, g: byte(row, 4)?, b: byte(row, 5)? }); } result.plte = Some(entries); }
        let alpha_rows = entities(database, "png_transparency_alpha")?;
        if let Some(row) = singleton(database, "png_transparency")? {
            result.trns = Some(match row.text(2)? {
                "indexed" => { nulls(row, &[3, 4, 5, 6])?; let mut alpha = Vec::new(); for row in &alpha_rows { tick(control)?; alpha.push(byte(row, 3)?); } PngTransparency::Indexed { alpha } },
                "grayscale" => { nulls(row, &[4, 5, 6])?; if !alpha_rows.is_empty() { return Err("PNG grayscale transparency cannot own alpha entries".into()); } PngTransparency::Grayscale { gray: word(row, 3)? } },
                "rgb" => { nulls(row, &[3])?; if !alpha_rows.is_empty() { return Err("PNG RGB transparency cannot own alpha entries".into()); } PngTransparency::Rgb { r: word(row, 4)?, g: word(row, 5)?, b: word(row, 6)? } },
                _ => return Err("unknown PNG transparency color model".into()),
            });
        } else if !alpha_rows.is_empty() { return Err("PNG alpha entries require indexed transparency".into()); }
        result.gama = singleton(database, "png_gamma")?.map(|row| unsigned(row, 2)).transpose()?;
        result.chrm = singleton(database, "png_chromaticity")?.map(|row| Ok::<_, String>(PngChromaticities { white_x: unsigned(row, 2)?, white_y: unsigned(row, 3)?, red_x: unsigned(row, 4)?, red_y: unsigned(row, 5)?, green_x: unsigned(row, 6)?, green_y: unsigned(row, 7)?, blue_x: unsigned(row, 8)?, blue_y: unsigned(row, 9)? })).transpose()?;
        result.srgb = singleton(database, "png_srgb")?.map(|row| PngSrgbIntent::from_u8(byte(row, 2)?)).transpose()?;
        result.phys = singleton(database, "png_physical_dimensions")?.map(|row| Ok::<_, String>(PngPhysicalDims { ppu_x: unsigned(row, 2)?, ppu_y: unsigned(row, 3)?, unit_is_meter: boolean(row, 4)? })).transpose()?;
        result.time = singleton(database, "png_modification_time")?.map(|row| Ok::<_, String>(PngTimestamp { year: word(row, 2)?, month: byte(row, 3)?, day: byte(row, 4)?, hour: byte(row, 5)?, minute: byte(row, 6)?, second: byte(row, 7)? })).transpose()?;
        if let Some(row) = singleton(database, "png_background")? { result.bkgd = Some(match row.text(2)? { "indexed" => { nulls(row, &[3, 4, 5, 6])?; PngBackground::Indexed { index: byte(row, 7)? } }, "grayscale" => { nulls(row, &[4, 5, 6, 7])?; PngBackground::Grayscale { gray: word(row, 3)? } }, "rgb" => { nulls(row, &[3, 7])?; PngBackground::Rgb { r: word(row, 4)?, g: word(row, 5)?, b: word(row, 6)? } }, _ => return Err("unknown PNG background color model".into()) }); }
        let mut text_ids = BTreeMap::new();
        for row in entities(database, "png_text")? { tick(control)?; if text_ids.insert(row.rowid, result.text_chunks.len()).is_some() { return Err("PNG text identity must be unique".into()); } result.text_chunks.push(PngTextChunk { keyword: row.text(3)?.into(), value: row.text(4)?.into(), compressed: boolean(row, 5)?, kind: match row.text(6)? { "text" => PngTextKind::Text, "ztext" => PngTextKind::ZText, "itext" => PngTextKind::IText, _ => return Err("unknown PNG text chunk kind".into()) }, language_tag: row.text(7)?.into(), translated_keyword: row.text(8)?.into() }); }
        let count = pixel_count(result.width, result.height)?;
        let pixel_rows = &database.table("png_pixel")?.rows;
        if pixel_rows.len() != count { return Err("PNG grid must have exactly one pixel per position".into()); }
        result.pixels = vec![0; count.checked_mul(4).ok_or("PNG pixel size overflow")?];
        let mut seen = vec![false; count];
        for row in pixel_rows { tick(control)?; if row.integer(1)? != 1 || row.integer(0)? != row.rowid || row.rowid <= 0 { return Err("PNG pixel has invalid ownership or identity".into()); } let x = unsigned(row, 2)?; let y = unsigned(row, 3)?; if x >= result.width || y >= result.height { return Err("PNG pixel coordinates exceed the grid".into()); } let index = usize::try_from(u64::from(y) * u64::from(result.width) + u64::from(x)).map_err(|error| error.to_string())?; if std::mem::replace(&mut seen[index], true) { return Err("PNG pixel coordinates must be unique".into()); } result.pixels[index * 4..index * 4 + 4].copy_from_slice(&[byte(row, 4)?, byte(row, 5)?, byte(row, 6)?, byte(row, 7)?]); }
        let mut unknown_ids = BTreeMap::new();
        for row in entities(database, "png_unknown_chunk")? { tick(control)?; if unknown_ids.insert(row.rowid, result.unknown_chunks.len()).is_some() { return Err("PNG unknown chunk identity must be unique".into()); } result.unknown_chunks.push(PngChunk { kind: [byte(row, 3)?, byte(row, 4)?, byte(row, 5)?, byte(row, 6)?], data: Vec::new() }); }
        let mut data_rows = BTreeMap::<i64, Vec<(i64, &SqliteRow)>>::new();
        for row in &database.table("png_unknown_chunk_byte")?.rows { tick(control)?; if row.integer(0)? != row.rowid || row.rowid <= 0 || !unknown_ids.contains_key(&row.integer(1)?) { return Err("PNG unknown chunk byte has invalid identity or parent".into()); } data_rows.entry(row.integer(1)?).or_default().push((row.integer(2)?, row)); }
        for (parent, mut rows) in data_rows { rows.sort_by_key(|(ordinal, _)| *ordinal); let chunk = &mut result.unknown_chunks[unknown_ids[&parent]]; for (index, (position, row)) in rows.iter().enumerate() { tick(control)?; if *position != ordinal(index)? { return Err("PNG unknown chunk bytes require contiguous ordinals".into()); } chunk.data.push(byte(row, 3)?); } }
        for row in entities(database, "png_chunk_sequence")? { tick(control)?; let kind = row.text(3)?; let marker = match kind {
            "text" => { nulls(row, &[5])?; PngChunkMarker::Text { index: *text_ids.get(&row.integer(4)?).ok_or("PNG chunk sequence references unknown text")? } },
            "unknown" => { nulls(row, &[4])?; PngChunkMarker::Unknown { index: *unknown_ids.get(&row.integer(5)?).ok_or("PNG chunk sequence references unknown chunk")? } },
            _ => { nulls(row, &[4, 5])?; match kind { "ihdr" => PngChunkMarker::Ihdr, "plte" => PngChunkMarker::Plte, "trns" => PngChunkMarker::Trns, "gama" => PngChunkMarker::Gama, "chrm" => PngChunkMarker::Chrm, "srgb" => PngChunkMarker::Srgb, "phys" => PngChunkMarker::Phys, "time" => PngChunkMarker::Time, "bkgd" => PngChunkMarker::Bkgd, "idat" => PngChunkMarker::Idat, "iend" => PngChunkMarker::Iend, _ => return Err("unknown PNG chunk marker".into()) } },
        }; result.chunk_order.push(marker); }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(result)
    }
}
