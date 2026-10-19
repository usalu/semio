//! 🛂️ Complete history equality borrows both sources and retains only original paid relational indexes.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::{SpaceHistorySnapshot,S_SPACE_HISTORY_SCHEMA,ArtifactSqliteSnapshotHost,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
use semio_framework_io_sqlite_snapshot::{SqliteTable, SqliteRow, SqliteValue};
use semio_framework_io_sqlite_snapshot::artifact::receiving::{Port, ReceivingWallet, Direction, integer, ordinal, sort, identity_position, parent_range, validate_schema_sql, check_database};
use semio_framework_io_schema::{IoResult, IoOutcome, IoError};
use super::settlement::TypedFileSettlement;
type Result<T>=std::result::Result<T,ValueError>;
pub(super) const TABLES:[(&str,usize);7]=[("space_history_document",2),("space_history_checkpoint",6),("space_history_timestamp",7),("space_history_author",6),("space_history_member_pin",6),("space_history_alternative",5),("space_history_alternative_checkpoint",4)];
const ORDERED:[usize;5]=[1,3,4,5,6];
pub(super) fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"space history semantic state differs")}
pub(super) fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"space history validation row census overflow"))}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Frame{pub(super) ids:[Vec<usize>;7],pub(super) ordered:[Vec<usize>;5],result:Option<IoResult<()>>}
impl Frame{pub(super) fn new()->Self{Self{ids:std::array::from_fn(|_|Vec::new()),ordered:std::array::from_fn(|_|Vec::new()),result:None}}}
pub(super) fn table<'a>(database:&'a SqliteDatabase,index:usize)->Result<&'a SqliteTable>{database.tables.iter().find(|table|table.name.eq_ignore_ascii_case(TABLES[index].0)).ok_or_else(invalid)}
pub(super) fn text(row:&SqliteRow,column:usize)->Result<&str>{match row.values.get(column){Some(SqliteValue::Text(text))=>Ok(text),_=>Err(invalid())}}
fn equal<W:ReceivingWallet>(source:&str,actual:&str,port:&mut Port<'_,'_,'_,W>)->Result<()>{
 port.work(3,0)?;if source.len()!=actual.len(){return Err(invalid())}let total=source.len();port.control.checkpoint(port.phase(),0,total)?;
 for(index,(source,actual))in source.as_bytes().chunks(256).zip(actual.as_bytes().chunks(256)).enumerate(){port.work(3,0)?;if source!=actual{return Err(invalid())}port.control.checkpoint(port.phase(),(index*256+source.len()).min(total),total)?;}Ok(())
}
fn optional<W:ReceivingWallet>(source:Option<&str>,actual:&SqliteValue,port:&mut Port<'_,'_,'_,W>)->Result<()>{port.work(3,0)?;match(source,actual){(None,SqliteValue::Null)=>Ok(()),(Some(source),SqliteValue::Text(actual))=>equal(source,actual,port),_=>Err(invalid())}}
fn coordinate<W:ReceivingWallet>(dialect:&semio_framework_artifact_reference::ArtifactDialect,port:&mut Port<'_,'_,'_,W>)->Result<()>{port.work(1,0)?;if dialect.artifact_kind!=S_SPACE_HISTORY_SCHEMA||dialect.standard!="1"||dialect.subset!="*"{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"space history does not own this SQLite coordinate"))}Ok(())}
fn shape<W:ReceivingWallet>(table:&SqliteTable,index:usize,port:&mut Port<'_,'_,'_,W>)->Result<()>{
 for row in &table.rows{port.work(2,0)?;if row.values.len()!=TABLES[index].1||integer(row,0)?!=row.rowid{return Err(invalid())}for(column,value)in row.values.iter().enumerate(){port.work(3,0)?;let integer_cell=column==0||index==2||index!=0&&column<=2;let optional_cell=matches!((index,column),(0,1)|(1,4)|(3,5));if integer_cell{let SqliteValue::Integer(value)=value else{return Err(invalid())};if index==2&&column>0&&! (0..=i64::from(u32::MAX)).contains(value){return Err(invalid())}if index!=0&&index!=2&&column==2&&*value<0{return Err(invalid())}}else if optional_cell{if !matches!(value,SqliteValue::Null|SqliteValue::Text(_)){return Err(invalid())}}else if !matches!(value,SqliteValue::Text(_)){return Err(invalid())}}}Ok(())
}
fn clock<W:ReceivingWallet>(source:u64,row:&SqliteRow,column:usize,port:&mut Port<'_,'_,'_,W>)->Result<()>{port.work(3,0)?;if integer(row,column)?!=(source>>32)as i64||integer(row,column+1)?!=(source&u64::from(u32::MAX))as i64{return Err(invalid())}Ok(())}
pub(super) fn indexed<'a,W:ReceivingWallet>(database:&'a SqliteDatabase,frame:&mut Frame,port:&mut Port<'_,'_,'_,W>)->Result<[&'a SqliteTable;7]>{
 validate_schema_sql(database,SpaceHistorySnapshot::SQLITE_SCHEMA,port)?;check_database(database,port)?;
 let tables=[table(database,0)?,table(database,1)?,table(database,2)?,table(database,3)?,table(database,4)?,table(database,5)?,table(database,6)?];if tables[0].rows.len()!=1||tables[2].rows.len()!=tables[1].rows.len(){return Err(invalid())}
 for(index,table)in tables.iter().enumerate(){shape(table,index,port)?;port.vector(table.rows.len(),&mut frame.ids[index],2)?;for row in 0..table.rows.len(){port.work(2,size_of::<usize>())?;frame.ids[index].push(row)}sort(&mut frame.ids[index],port,|a,b|Ok(table.rows[a].rowid.cmp(&table.rows[b].rowid)))?;for pair in frame.ids[index].windows(2){port.work(2,0)?;if table.rows[pair[0]].rowid==table.rows[pair[1]].rowid{return Err(invalid())}}}
 for(index,table_index)in ORDERED.iter().copied().enumerate(){let table=tables[table_index];port.vector(table.rows.len(),&mut frame.ordered[index],2)?;for row in 0..table.rows.len(){port.work(2,size_of::<usize>())?;frame.ordered[index].push(row)}sort(&mut frame.ordered[index],port,|a,b|{let a=&table.rows[a];let b=&table.rows[b];Ok((integer(a,1)?,integer(a,2)?).cmp(&(integer(b,1)?,integer(b,2)?)))})?;for pair in frame.ordered[index].windows(2){port.work(2,0)?;let a=&table.rows[pair[0]];let b=&table.rows[pair[1]];if integer(a,1)?==integer(b,1)?&&integer(a,2)?==integer(b,2)?{return Err(invalid())}}let parent=match table_index{1|5=>0,3|4=>1,6=>5,_=>unreachable!()};for row in &table.rows{if identity_position(&tables[parent].rows,&frame.ids[parent],integer(row,1)?,port)?.is_none(){return Err(invalid())}}}
 for row in &tables[2].rows{if identity_position(&tables[1].rows,&frame.ids[1],row.rowid,port)?.is_none(){return Err(invalid())}}
 Ok(tables)
}
fn check<W:ReceivingWallet>(source:&SpaceHistorySnapshot,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,frame:&mut Frame,port:&mut Port<'_,'_,'_,W>)->Result<()>{
 coordinate(dialect,port)?;
 let tables:[&SqliteTable;7]=[table(database,0)?,table(database,1)?,table(database,2)?,table(database,3)?,table(database,4)?,table(database,5)?,table(database,6)?];
 let mut authors=0;let mut members=0;let mut pins=0;for checkpoint in &source.checkpoints{port.work(1,0)?;authors=add(authors,checkpoint.authors.len())?;members=add(members,checkpoint.members.len())?;}for alternative in &source.alternatives{port.work(1,0)?;pins=add(pins,alternative.checkpoint_ids.len())?;}let counts=[1,source.checkpoints.len(),source.checkpoints.len(),authors,members,source.alternatives.len(),pins];
 for(index,table)in tables.iter().enumerate(){if table.rows.len()!=counts[index]{return Err(invalid())}}let tables=indexed(database,frame,port)?;
 let document=&tables[0].rows[0];optional(source.active_alternative_id.as_deref(),&document.values[1],port)?;let checkpoints=parent_range(&tables[1].rows,&frame.ordered[0],document.rowid,1,port)?;if checkpoints.len()!=source.checkpoints.len(){return Err(invalid())}
 for(index,checkpoint)in source.checkpoints.iter().enumerate(){port.work(2,0)?;let row=&tables[1].rows[frame.ordered[0][checkpoints.start+index]];if integer(row,2)?!=ordinal(index)?{return Err(invalid())}equal(&checkpoint.id,text(row,3)?,port)?;optional(checkpoint.parent_id.as_deref(),&row.values[4],port)?;equal(&checkpoint.message,text(row,5)?,port)?;let clock_index=identity_position(&tables[2].rows,&frame.ids[2],row.rowid,port)?.ok_or_else(invalid)?;let timestamp=&tables[2].rows[clock_index];clock(checkpoint.timestamp.actor,timestamp,1,port)?;clock(checkpoint.timestamp.physical_ms,timestamp,3,port)?;clock(checkpoint.timestamp.logical,timestamp,5,port)?;
  let author_rows=parent_range(&tables[3].rows,&frame.ordered[1],row.rowid,1,port)?;if author_rows.len()!=checkpoint.authors.len(){return Err(invalid())}for(index,author)in checkpoint.authors.iter().enumerate(){let actual=&tables[3].rows[frame.ordered[1][author_rows.start+index]];if integer(actual,2)?!=ordinal(index)?{return Err(invalid())}equal(&author.id,text(actual,3)?,port)?;equal(&author.name,text(actual,4)?,port)?;optional(author.avatar.as_deref(),&actual.values[5],port)?;}
  let member_rows=parent_range(&tables[4].rows,&frame.ordered[2],row.rowid,1,port)?;if member_rows.len()!=checkpoint.members.len(){return Err(invalid())}for(index,member)in checkpoint.members.iter().enumerate(){let actual=&tables[4].rows[frame.ordered[2][member_rows.start+index]];if integer(actual,2)?!=ordinal(index)?{return Err(invalid())}equal(&member.document_id,text(actual,3)?,port)?;equal(&member.checkpoint_id,text(actual,4)?,port)?;equal(&member.alternative_id,text(actual,5)?,port)?;}
 }
 let alternatives=parent_range(&tables[5].rows,&frame.ordered[3],document.rowid,1,port)?;if alternatives.len()!=source.alternatives.len(){return Err(invalid())}for(index,alternative)in source.alternatives.iter().enumerate(){let row=&tables[5].rows[frame.ordered[3][alternatives.start+index]];if integer(row,2)?!=ordinal(index)?{return Err(invalid())}equal(&alternative.id,text(row,3)?,port)?;equal(&alternative.name,text(row,4)?,port)?;let checkpoint_ids=parent_range(&tables[6].rows,&frame.ordered[4],row.rowid,1,port)?;if checkpoint_ids.len()!=alternative.checkpoint_ids.len(){return Err(invalid())}for(index,checkpoint_id)in alternative.checkpoint_ids.iter().enumerate(){let actual=&tables[6].rows[frame.ordered[4][checkpoint_ids.start+index]];if integer(actual,2)?!=ordinal(index)?{return Err(invalid())}equal(checkpoint_id,text(actual,3)?,port)?;}}
 Ok(())
}
macro_rules! receive{
 ($name:ident,$owner:ident,$progress:path,$direction:ident,$phase:ident)=>{
 pub(super) fn $name(source:&SpaceHistorySnapshot,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::$owner<'_,'_>)->IoResult<()>{
  let result=(||->IoResult<()>{let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||IoError::from_value_error(ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history validation original SQL ceiling overflow")))?;let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::$phase);let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
   let received=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Frame,IoResult<()>>(|slot,native,body|{
    let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;let mut port=Port{control,native:Direction::$direction(native),body};port.work(1,size_of::<Frame>())?;*slot=Some(Frame::new());let frame=slot.as_mut().unwrap();port.work(1,size_of::<IoResult<()>>())?;frame.result=Some(check(source,dialect,database,frame,&mut port).map(|()|IoOutcome::clean(())).map_err(IoError::from_value_error));let owned=port.owned();drop(port);drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
   }));match received{Ok(result)=>result,Err(error)=>Err(IoError::from_value_error(settlement.refused(owner.native().owned_bytes(),error)))}
  })();result.map_err(|mut error|{error.cause=error.cause.with_retained_progress(owner.progress());error})
 }
 };
}
receive!(decoding,NativeSnapshotDecodeOwner,semio_framework_value_native_owned::native_decoding::NativeDecodeProgress,Decode,ProjectSnapshot);
receive!(encoding,NativeSnapshotEncodeOwner,semio_framework_value::native_encoding::NativeEncodeProgress,Encode,ReconstructSnapshot);
