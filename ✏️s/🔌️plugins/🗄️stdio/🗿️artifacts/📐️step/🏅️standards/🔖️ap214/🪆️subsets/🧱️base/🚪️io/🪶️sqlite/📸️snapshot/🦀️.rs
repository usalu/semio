//! 📐️ STEP exchange headers, typed argument ownership and entity references.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};
use crate::standards::v_ap214::subsets::base::schema::snapshot::{StepSnapshot,StepHeader,StepFileDescription,StepFileName,StepFileSchema,StepEntity,StepComplexType,StepValue};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,FloatColumn,read_binary64,ieee754_is_null,validate_ieee754_row},*}};
#[path = "📏️encoding/💰️backing/🦀️.rs"]
mod native_backing;
#[path = "🛬️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
const REAL:&[FloatColumn]=&[FloatColumn::Binary64(3)];
struct UnsignedWord{bytes:[u8;20],start:usize}
impl UnsignedWord{
    fn new(mut value:u64)->Self{let mut out=Self{bytes:[0;20],start:20};loop{out.start-=1;out.bytes[out.start]=b'0'+(value%10)as u8;value/=10;if value==0{return out;}}}
    fn text(&self)->&str{std::str::from_utf8(&self.bytes[self.start..]).expect("decimal digits are UTF-8")}
}
fn sort_paid<T>(values:&mut[T],mut compare:impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{
    fn sift<T>(values:&mut[T],mut root:usize,end:usize,compare:&mut impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{loop{let Some(child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let mut next=child;if child+1<end&&compare(&values[child],&values[child+1])?.is_lt(){next=child+1;}if !compare(&values[root],&values[next])?.is_lt(){return Ok(())}values.swap(root,next);root=next;}}
    for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut compare)?;}Ok(())
}
/// 🪜️ Borrowed AP214 facts shared by the six owned conformance classes.
pub struct ConformanceFacts<'a>{pub file_schema:bool,pub product_chain:bool,pub violations:Vec<(u64,&'a str,u8)>}
/// 🔎️ Scans retained STEP type names without materializing a Part-21 graph.
pub fn conformance_facts<'a>(snapshot:&'a StepSnapshot,max_rung:u8,control:&mut SqliteSnapshotControl<'_>)->Result<ConformanceFacts<'a>,ValueError>{
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
    let mut total=snapshot.header.file_schema.schemas.len();
    for(index,entity)in snapshot.entities.iter().enumerate(){total=total.checked_add(entity.complex.len()).and_then(|n|n.checked_add(1)).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "STEP conformance count overflow"))?;control.check_rows(total)?;if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,snapshot.entities.len())?;}}
    control.check_rows(total)?;
    let mut facts=ConformanceFacts{file_schema:false,product_chain:false,violations:Vec::new()};let mut seen=[false;3];let mut count=0usize;let mut diagnostic_bytes=512usize;
    for name in &snapshot.header.file_schema.schemas{facts.file_schema|=name.eq_ignore_ascii_case("AUTOMOTIVE_DESIGN");count+=1;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,total)?;}}
    for entity in &snapshot.entities{for name in std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part|part.name.as_str())){
        seen[0]|=name.eq_ignore_ascii_case("PRODUCT");seen[1]|=name.eq_ignore_ascii_case("PRODUCT_DEFINITION_FORMATION")||name.eq_ignore_ascii_case("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE");seen[2]|=name.eq_ignore_ascii_case("PRODUCT_DEFINITION")||name.eq_ignore_ascii_case("PRODUCT_DEFINITION_WITH_ASSOCIATED_DOCUMENTS");
        let rung=if name.get(name.len().saturating_sub(20)..).is_some_and(|suffix|suffix.eq_ignore_ascii_case("SHAPE_REPRESENTATION")){Some(if name.eq_ignore_ascii_case("GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION"){3}else if name.eq_ignore_ascii_case("MANIFOLD_SURFACE_SHAPE_REPRESENTATION"){4}else if name.eq_ignore_ascii_case("FACETED_BREP_SHAPE_REPRESENTATION"){5}else if name.eq_ignore_ascii_case("ADVANCED_BREP_SHAPE_REPRESENTATION"){6}else{2})}else{None};
        if let Some(rung)=rung.filter(|rung|*rung>max_rung){diagnostic_bytes=diagnostic_bytes.checked_add(name.len()).and_then(|n|n.checked_add(200)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "STEP diagnostic size overflow"))?;control.check_value_bytes(diagnostic_bytes)?;if name.len()>65536{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,total)?;}facts.violations.push((entity.id,name,rung));}
        count+=1;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,total)?;}
    }}facts.product_chain=seen.into_iter().all(|value|value);control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,total)?;Ok(facts)
}
fn number(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error| ValueError::new(ValueRefusalKind::WorkLimit, error.to_string()))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid STEP entity identity or columns"))}else{Ok(())}}
fn unsigned(text:&str)->Result<u64,ValueError>{let value=text.parse::<u64>().map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error.to_string()))?;if UnsignedWord::new(value).text()!=text{Err(ValueError::new(ValueRefusalKind::InvalidValue, "STEP unsigned field must be canonical decimal text"))}else{Ok(value)}}
fn null(row:&SqliteRow,index:usize)->Result<(),ValueError>{if row.values.get(index)==Some(&SqliteValue::Null){Ok(())}else{Err(ValueError::new(ValueRefusalKind::InvalidValue, "unexpected STEP value variant field"))}}
enum Owner<'a>{Argument(Option<i64>,Option<i64>,usize),Aggregate(i64,usize),Typed(i64,&'a str)}
fn project_value<'a>(value:&'a StepValue,owner:Owner<'a>,entities:&[(u64,i64)],projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
    let mut pending=projection.allocate_frontier(1)?;pending.push((value,owner));
    while let Some((value,owner))=pending.pop(){projection.checkpoint()?;let mut cells=[Cell::Null;7];let reference;
        cells[0]=Cell::Text(match value{StepValue::Unset=>"unset",StepValue::Derived=>"derived",StepValue::Integer(_)=>"integer",StepValue::Real(_)=>"real",StepValue::String(_)=>"string",StepValue::Enum(_)=>"enum",StepValue::Reference(_)=>"reference",StepValue::Aggregate(_)=>"aggregate",StepValue::TypedValue{..}=>"typedValue"});
        match value{StepValue::Integer(v)=>cells[1]=Cell::Integer(*v),StepValue::Real(v)=>cells[2]=Cell::Real(*v),StepValue::String(v)=>cells[3]=Cell::Text(v),StepValue::Enum(v)=>cells[4]=Cell::Text(v),StepValue::Reference(v)=>{reference=UnsignedWord::new(*v);cells[5]=Cell::Text(reference.text());cells[6]=Cell::Integer(entities.binary_search_by_key(v,|entry|entry.0).map(|index|entities[index].1).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"dangling STEP entity reference"))?);},_=>{}}
        let id=projection.insert_float("step_value",&cells,REAL)?;
        match owner{Owner::Argument(entity,complex,ordinal)=>{projection.insert("step_argument",&[entity.map_or(Cell::Null,Cell::Integer),complex.map_or(Cell::Null,Cell::Integer),Cell::Integer(number(ordinal)?),Cell::Integer(id)])?;},Owner::Aggregate(parent,ordinal)=>{projection.insert("step_aggregate_element",&[Cell::Integer(parent),Cell::Integer(number(ordinal)?),Cell::Integer(id)])?;},Owner::Typed(parent,name)=>{projection.insert("step_typed_value",&[Cell::Integer(parent),Cell::Text(name),Cell::Integer(id)])?;}}
        match value{StepValue::Aggregate(values)=>{projection.check_rows(pending.len().checked_add(values.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "STEP pending value count overflow"))?)?;for(ordinal,value)in values.iter().enumerate().rev(){projection.push_frontier(&mut pending,(value,Owner::Aggregate(id,ordinal)))?;if ordinal%256==0{projection.checkpoint()?;}}},StepValue::TypedValue{type_name,value}=>projection.push_frontier(&mut pending,(value,Owner::Typed(id,type_name)))?,_=>{}}
    }Ok(())
}
fn visit_rows(snapshot:&StepSnapshot,projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
        let header=&snapshot.header;let mut predicted=2usize;for length in [snapshot.entities.len(),header.file_description.description.len(),header.file_name.author.len(),header.file_name.organization.len(),header.file_schema.schemas.len()]{predicted=predicted.checked_add(length).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "STEP projected row count overflow"))?;}projection.check_rows(predicted)?;projection.checkpoint()?;
        
        let mut identities=projection.allocate_frontier(snapshot.entities.len())?;
        for(ordinal,entity)in snapshot.entities.iter().enumerate(){identities.push((entity.id,number(ordinal+1)?));projection.checkpoint()?;}
        sort_paid(&mut identities,|a,b|{projection.checkpoint()?;Ok(a.0.cmp(&b.0))})?;
        for pair in identities.windows(2){if pair[0].0==pair[1].0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate STEP instance identifier"));}projection.checkpoint()?;}
        projection.insert_key("step_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(1)])?;let name=&header.file_name;projection.insert_key("step_header",1,&[Cell::Text(&header.file_description.implementation_level),Cell::Text(&name.name),Cell::Text(&name.timestamp),Cell::Text(&name.preprocessor_version),Cell::Text(&name.originating_system),Cell::Text(&name.authorization)])?;
        for(table,values)in [("step_description",&header.file_description.description),("step_author",&name.author),("step_organization",&name.organization),("step_schema_identifier",&header.file_schema.schemas)]{for(ordinal,value)in values.iter().enumerate(){projection.insert(table,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(value)])?;}}
        for(ordinal,entity)in snapshot.entities.iter().enumerate(){let key=identities[identities.binary_search_by_key(&entity.id,|entry|entry.0).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"missing STEP entity identity"))?].1;let word=UnsignedWord::new(entity.id);projection.insert_key("step_entity",key,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(word.text()),Cell::Text(&entity.name)])?;projection.check_rows(entity.args.len())?;for(ordinal,value)in entity.args.iter().enumerate(){project_value(value,Owner::Argument(Some(key),None,ordinal),&identities,projection)?;}projection.check_rows(entity.complex.len())?;for(ordinal,complex)in entity.complex.iter().enumerate(){let complex_id=projection.insert("step_complex_type",&[Cell::Integer(key),Cell::Integer(number(ordinal)?),Cell::Text(&complex.name)])?;projection.check_rows(complex.args.len())?;for(ordinal,value)in complex.args.iter().enumerate(){project_value(value,Owner::Argument(None,Some(complex_id),ordinal),&identities,projection)?;}}}Ok(())
    
    }
impl StepSnapshot{pub(crate)fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;visit_rows(self,&mut writer)?;writer.finish_borrowed()}pub(crate)fn admit_sqlite_record(value:&semio_framework_dsl_record::RecordValue,limits:SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{semantic::admit_record(value,limits,native)}}
impl ArtifactSqliteSnapshot for StepSnapshot{
    fn retire_sqlite_snapshot(self){crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native::close(self)}
    fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native::decode(payload,control)}
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{crate::standards::v_ap214::subsets::base::io::sqlite::snapshot::native::encode(self,encoding,control)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native_backing::preflight(self,control)}
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
        (|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.step"||dialect.standard!="ap214"{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP dialect differs from its semantic standard"));}let row=database.table("step_document")?.single_row()?;if row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "owned STEP document identity differs from semantic projection"));}let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"cc1"=>crate::standards::v_ap214::subsets::cc1::io::check_cc1_conformance_controlled(self,control)?,"cc2"=>crate::standards::v_ap214::subsets::cc2::io::check_cc2_conformance_controlled(self,control)?,"cc3"=>crate::standards::v_ap214::subsets::cc3::io::check_cc3_conformance_controlled(self,control)?,"cc4"=>crate::standards::v_ap214::subsets::cc4::io::check_cc4_conformance_controlled(self,control)?,"cc5"=>crate::standards::v_ap214::subsets::cc5::io::check_cc5_conformance_controlled(self,control)?,"cc6"=>crate::standards::v_ap214::subsets::cc6::io::check_cc6_conformance_controlled(self,control)?,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown owned STEP subset"))};Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    
        })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)
    }
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::extent(control.limits())?;let mut projection=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut projection)?;projection.finish()}
    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
        validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;reconstruction::read(database,control)
    
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
