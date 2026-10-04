//! 🪶️ Unmounted authored SQLite mapping adds paid literal-field copies after its owning baseline.
use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::{validate_sqlite_database_schema, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue}, ArtifactSqliteSnapshot};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

fn pixels_count(width: u32, height: u32) -> Result<usize, ValueError> { usize::try_from(u64::from(width).checked_mul(u64::from(height)).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP pixel count overflow"))?).map_err(|error|invalid(error.to_string())) }
fn byte(row: &SqliteRow, column: usize) -> Result<u8, ValueError> { u8::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }
fn word(row: &SqliteRow, column: usize) -> Result<u16, ValueError> { u16::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }
fn unsigned(row: &SqliteRow, column: usize) -> Result<u32, ValueError> { u32::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }
fn signed(row: &SqliteRow, column: usize) -> Result<i32, ValueError> { i32::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string())) }

impl ArtifactSqliteSnapshot for BmpSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../../🪶️sqlite/🗄️.sql");

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        let count = pixels_count(self.width, self.height)?;
        if self.pixels.len() != count.checked_mul(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP pixel size overflow"))? { return Err(invalid("BMP pixels must contain one RGBA tuple per grid position")); }
        let total = count.checked_add(self.palette.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP entity count overflow"))?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, total)?;
        control.check_rows(total.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP entity count overflow"))?)?;
        let row_order = match self.row_order { BmpRowOrder::BottomUp => "bottom_up", BmpRowOrder::TopDown => "top_down" };
        control.check_value_bytes(count.checked_mul(64).and_then(|count| self.palette.len().checked_mul(56).and_then(|palette| count.checked_add(palette))).and_then(|count| count.checked_add(96)).and_then(|count| count.checked_add(self.schema.len())).and_then(|count| count.checked_add(row_order.len())).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP value size overflow"))?)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("bmp_document")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Text(semio_framework_os_kernel::sqlite_snapshot::artifact::project_text(control, &self.schema)?), SqliteValue::Integer(i64::from(self.header_size)), SqliteValue::Integer(i64::from(self.width)), SqliteValue::Integer(i64::from(self.height)), SqliteValue::Text(row_order.into()), SqliteValue::Integer(i64::from(self.planes)), SqliteValue::Integer(i64::from(self.bits_per_pixel)), SqliteValue::Integer(i64::from(self.compression)), SqliteValue::Integer(i64::from(self.image_size)), SqliteValue::Integer(i64::from(self.x_pixels_per_meter)), SqliteValue::Integer(i64::from(self.y_pixels_per_meter)), SqliteValue::Integer(i64::from(self.colors_used)), SqliteValue::Integer(i64::from(self.colors_important))] });
        for (ordinal, entry) in self.palette.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, ordinal, total)?; }
            let ordinal = i64::try_from(ordinal).map_err(|error|invalid(error.to_string()))?;
            let id = ordinal.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP palette identifier overflow"))?;
            database.table_mut("bmp_palette_entry")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Integer(1), SqliteValue::Integer(ordinal), SqliteValue::Integer(i64::from(entry.b)), SqliteValue::Integer(i64::from(entry.g)), SqliteValue::Integer(i64::from(entry.r)), SqliteValue::Integer(i64::from(entry.reserved))] });
        }
        for (ordinal, rgba) in self.pixels.chunks_exact(4).enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.palette.len() + ordinal, total)?; }
            let id = i64::try_from(ordinal).map_err(|error|invalid(error.to_string()))?.checked_add(1).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP pixel identifier overflow"))?;
            let width = usize::try_from(self.width).map_err(|error|invalid(error.to_string()))?;
            database.table_mut("bmp_pixel")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Integer(1), SqliteValue::Integer((ordinal % width) as i64), SqliteValue::Integer((ordinal / width) as i64), SqliteValue::Integer(i64::from(rgba[0])), SqliteValue::Integer(i64::from(rgba[1])), SqliteValue::Integer(i64::from(rgba[2])), SqliteValue::Integer(i64::from(rgba[3]))] });
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, total, total)?;
        Ok(database)
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("bmp_document")?.single_row()?;
        if document.integer(0)? != 1 || document.rowid != 1 { return Err(invalid("BMP document identifier must be 1")); }
        let width = unsigned(document, 3)?;
        let height = unsigned(document, 4)?;
        let count = pixels_count(width, height)?;
        let pixel_rows = &database.table("bmp_pixel")?.rows;
        if count != pixel_rows.len() { return Err(invalid("BMP grid must have exactly one pixel per position")); }
        let total = count.checked_add(database.table("bmp_palette_entry")?.rows.len()).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP entity count overflow"))?;
        let row_order = match document.text(5)? { "bottom_up" => BmpRowOrder::BottomUp, "top_down" => BmpRowOrder::TopDown, _ => return Err(invalid("unknown BMP row order")) };
        let mut palette = Vec::new();
        for row in database.table("bmp_palette_entry")?.ordered_rows(2)? {
            if palette.len() % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, palette.len(), total)?; }
            if row.integer(1)? != 1 { return Err(invalid("BMP palette entry has an unknown document")); }
            palette.push(BmpPaletteEntry { b: byte(row, 3)?, g: byte(row, 4)?, r: byte(row, 5)?, reserved: byte(row, 6)? });
        }
        let mut pixels = vec![0; count.checked_mul(4).ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"BMP pixel size overflow"))?];
        let mut seen = vec![false; count];
        for (ordinal, row) in pixel_rows.iter().enumerate() {
            if ordinal % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, palette.len() + ordinal, total)?; }
            if row.integer(1)? != 1 { return Err(invalid("BMP pixel has an unknown document")); }
            let x = unsigned(row, 2)?;
            let y = unsigned(row, 3)?;
            if x >= width || y >= height { return Err(invalid("BMP pixel coordinates exceed the grid")); }
            let position = usize::try_from(u64::from(y) * u64::from(width) + u64::from(x)).map_err(|error|invalid(error.to_string()))?;
            if std::mem::replace(&mut seen[position], true) { return Err(invalid("BMP pixel coordinates must be unique")); }
            pixels[position * 4..position * 4 + 4].copy_from_slice(&[byte(row, 4)?, byte(row, 5)?, byte(row, 6)?, byte(row, 7)?]);
        }
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, total, total)?;
        Ok(Self { schema: semio_framework_os_kernel::sqlite_snapshot::artifact::reconstruct_text(control, document.text(1)?)?, header_size: unsigned(document, 2)?, width, height, row_order, planes: word(document, 6)?, bits_per_pixel: word(document, 7)?, compression: unsigned(document, 8)?, image_size: unsigned(document, 9)?, x_pixels_per_meter: signed(document, 10)?, y_pixels_per_meter: signed(document, 11)?, colors_used: unsigned(document, 12)?, colors_important: unsigned(document, 13)?, palette, pixels })
    }
}
