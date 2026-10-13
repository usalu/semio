//! 📏️ Resource primitives for Semio's explicitly authored native field scans.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_os_kernel::sqlite_snapshot::{artifact::NativeEncodingBound, SqliteSnapshotControl};

/// 🧮️ Accounts for scalar spelling, native buffers and explicit entity counts before encoding.
pub struct Bound<'c, 'p> { bound: NativeEncodingBound<'c, 'p>, rows: usize }
impl<'c, 'p> Bound<'c, 'p> {
    pub fn allocate_frontier<T>(&mut self,count:usize)->Result<Vec<T>,ValueError>{self.bound.allocate_frontier(count)}
    pub fn push_frontier<T>(&mut self,frontier:&mut Vec<T>,value:T)->Result<(),ValueError>{self.bound.push_frontier(frontier,value)}
    pub fn new(schema: &str, control: &'c mut SqliteSnapshotControl<'p>) -> Result<Self, ValueError> { let mut bound = NativeEncodingBound::new(control)?; bound.add(1024)?; bound.repeated(schema.len(), 16)?; Ok(Self { bound, rows: 1 }) }
    /// 📁️ Forecasts native file bytes after the owner separately admitted complete SQL cells.
    pub fn file_only(schema:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{let mut bound=NativeEncodingBound::file_only(control)?;bound.add(1024)?;bound.repeated(schema.len(),16)?;Ok(Self{bound,rows:1})}
    pub fn entities(&mut self, count: usize) -> Result<(), ValueError> { self.rows = self.rows.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio native entity count overflow"))?; self.bound.check_rows(self.rows)?; self.bound.repeated(count, 128) }
    pub fn text(&mut self, value: &str) -> Result<(), ValueError> { self.bound.repeated(value.len(), 16) }
    pub fn optional_text(&mut self, value: Option<&str>) -> Result<(), ValueError> { if let Some(value) = value { self.text(value)?; } Ok(()) }
    pub fn bytes(&mut self, value: &[u8]) -> Result<(), ValueError> { self.bound.repeated(value.len(), 8) }
    pub fn scalars(&mut self, count: usize) -> Result<(), ValueError> { self.bound.repeated(count, 2048) }
    pub fn checkpoint(&mut self) -> Result<(), ValueError> { self.bound.checkpoint() }
    pub fn finish(self) -> Result<(), ValueError> { self.bound.finish() }
}

/// 🧭️ Bounds the actual typed ArtifactRef fields in an owned Semio relationship.
pub fn native_reference(reference:&semio_framework_artifact_reference::ArtifactRef,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.text(&reference.artifact_id)?;b.text(&reference.dialect.artifact_kind)?;b.text(&reference.dialect.standard)?;b.text(&reference.dialect.subset)?;Ok(())}
/// 🪆️ Bounds a typed durable ArtifactChild identity and target without materializing its cache.
pub fn native_child<S>(child:&store::os_store::artifact_child::ArtifactChild<S>,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(1)?;b.text(&child.child_id)?;native_reference(&child.target,b)}
