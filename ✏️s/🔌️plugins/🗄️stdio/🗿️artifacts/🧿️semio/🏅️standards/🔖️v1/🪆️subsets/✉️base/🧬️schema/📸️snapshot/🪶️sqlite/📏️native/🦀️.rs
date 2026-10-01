//! 📏️ Resource primitives for Semio's explicitly authored native field scans.
use semio_framework_os_kernel::sqlite_snapshot::{artifact::NativeEncodingBound, SqliteSnapshotControl};

/// 🧮️ Accounts for scalar spelling, native buffers and explicit entity counts before encoding.
pub struct Bound<'c, 'p> { bound: NativeEncodingBound<'c, 'p>, rows: usize }
impl<'c, 'p> Bound<'c, 'p> {
    pub fn new(schema: &str, control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self, String> { let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?; bound.repeated(schema.len(), 16)?; Ok(Self { bound, rows: 1 }) }
    pub fn entities(&mut self, count: usize) -> Result<(), String> { self.rows = self.rows.checked_add(count).ok_or("Semio native entity count overflow")?; self.bound.check_rows(self.rows)?; self.bound.repeated(count, 128) }
    pub fn text(&mut self, value: &str) -> Result<(), String> { self.bound.repeated(value.len(), 16) }
    pub fn optional_text(&mut self, value: Option<&str>) -> Result<(), String> { if let Some(value) = value { self.text(value)?; } Ok(()) }
    pub fn bytes(&mut self, value: &[u8]) -> Result<(), String> { self.bound.repeated(value.len(), 8) }
    pub fn scalars(&mut self, count: usize) -> Result<(), String> { self.bound.repeated(count, 2048) }
    pub fn checkpoint(&mut self) -> Result<(), String> { self.bound.checkpoint() }
    pub fn finish(self) -> Result<(), String> { self.bound.finish() }
}
