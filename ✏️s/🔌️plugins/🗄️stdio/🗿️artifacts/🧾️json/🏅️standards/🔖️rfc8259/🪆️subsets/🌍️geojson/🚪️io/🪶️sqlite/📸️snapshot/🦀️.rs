//! 🪶️ SQLite work authority adapts to neutral GeoJSON semantic conformance.
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
use crate::standards::v_rfc8259::subsets::geojson::schema::{GeoJsonConformanceControl,check_geojson_conformance_with_control};
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,ValueError};
struct SqliteGeoJsonConformance<'a,'b>{control:&'a mut SqliteSnapshotControl<'b>}
impl GeoJsonConformanceControl for SqliteGeoJsonConformance<'_,'_>{
fn checkpoint(&mut self,visited:usize,total:usize)->Result<(),ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,visited,total)}
fn maximum_rows(&self)->usize{self.control.limits().max_rows}
fn number_valid(&mut self,lexeme:&str,visited:usize)->Result<bool,ValueError>{Ok(crate::schema::snapshot::number::meaning(lexeme,self.control,SqliteSnapshotPhase::ProjectSnapshot,visited,0)?.valid)}
}
/// 🛡️ Validates GeoJSON under the SQLite operation's cancellation and row budget.
pub fn check_geojson_conformance_controlled(snapshot:&JsonSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<semio_framework_diagnostic::Diagnostic>,ValueError>{check_geojson_conformance_with_control(snapshot,&mut SqliteGeoJsonConformance{control})}
