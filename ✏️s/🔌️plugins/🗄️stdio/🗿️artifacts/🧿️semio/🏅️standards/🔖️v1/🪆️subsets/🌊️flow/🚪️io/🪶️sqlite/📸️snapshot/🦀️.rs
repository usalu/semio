//! 🌊️ Named flow nodes, ordered parameters and port-addressed edge entities.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot,FlowNode,FlowEdge,FlowParam,PortRef};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🫙️projection/🦀️.rs"]
pub(crate)mod projection;
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🫳️ Visits the exact Flow entities through the caller's owned or borrowed row writer.
pub(crate)fn visit_rows(snapshot:&SemioFlowSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
let phase=out.phase();let mut nodes=out.allocate_frontier(snapshot.nodes.len())?;
for(ordinal,node)in snapshot.nodes.iter().enumerate(){out.checkpoint()?;nodes.push((node.id.as_str(),number(ordinal+1)?));}
out.sort_frontier(&mut nodes,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,c))?;
for pair in nodes.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow node identifier"))}}
let mut ids=out.allocate_frontier(snapshot.edges.len())?;for edge in &snapshot.edges{out.checkpoint()?;ids.push(edge.id.as_str());}
out.sort_frontier(&mut ids,|a,b,c|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a,b,phase,c))?;
for pair in ids.windows(2){if out.compare_text(pair[0],pair[1])?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio flow edge identifier"))}}
out.insert_key("semio_flow_document",1,&[Cell::Text(&snapshot.schema)])?;
for(ordinal,node)in snapshot.nodes.iter().enumerate(){
let id=out.insert_float("semio_flow_node",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&node.id),Cell::Text(&node.kind),Cell::Text(&node.label),Cell::Real(node.position.x),Cell::Real(node.position.y)],float_columns("semio_flow_node"))?;
for(ordinal,param)in node.params.iter().enumerate(){out.insert("semio_flow_parameter",&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&param.key),Cell::Text(&param.value)])?;}
}
for(ordinal,edge)in snapshot.edges.iter().enumerate(){
let source=node_reference(&nodes,&edge.from.node,out)?;let target=node_reference(&nodes,&edge.to.node,out)?;
out.insert("semio_flow_edge",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&edge.id),Cell::Integer(source),Cell::Text(&edge.from.port),Cell::Integer(target),Cell::Text(&edge.to.port),Cell::Text(&edge.kind)])?;
}Ok(())
}
/// 🔎️ Resolves the actual sorted borrowed node frontier with bounded literal comparisons.
fn node_reference(nodes:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
let mut lo=0;let mut hi=nodes.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(nodes[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(nodes[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling Semio flow edge node"))
}
/// 🎟️ Admits complete typed cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioFlowSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;projection::admit(snapshot,phase,control)}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts actual binary primitive cells before allocating typed fields.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts actual document primitive cells before allocating typed fields.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio flow row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioFlowSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::flow::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="flow"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_flow_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioFlowSnapshot {
/// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_flow_node"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7)],_=>&[]}}


fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioFlowSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.nodes.len())?;for node in &self.nodes{b.text(&node.id)?;b.text(&node.kind)?;b.text(&node.label)?;b.scalars(2)?;b.entities(node.params.len())?;for param in &node.params{b.text(&param.key)?;b.text(&param.value)?;}}b.entities(self.edges.len())?;for edge in &self.edges{b.text(&edge.id)?;b.text(&edge.from.node)?;b.text(&edge.from.port)?;b.text(&edge.to.node)?;b.text(&edge.to.port)?;b.text(&edge.kind)?;}Ok(())}
}

impl SemioFlowSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;projection::project(self,Self::SQLITE_SCHEMA,control)
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
                                                                  