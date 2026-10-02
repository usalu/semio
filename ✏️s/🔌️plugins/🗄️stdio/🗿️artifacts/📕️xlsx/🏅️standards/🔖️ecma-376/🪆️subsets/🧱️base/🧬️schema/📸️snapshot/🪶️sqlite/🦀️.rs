//! 📕️ Complete XLSX OPC entities and typed multi-document XML graph.
use super::*;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{project_text,reconstruct_text,ordered_row_refs}}};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlSnapshot,sqlite::{XmlSqliteTables,project_xml_documents,measure_xml_documents,reconstruct_xml_documents}};
use semio_s_artifact_stdio_zip::opc::sqlite::{OpcSqliteTables,measure_opc_package,project_opc_package,reconstruct_opc_package};
use std::collections::BTreeSet;
const OPC:OpcSqliteTables=OpcSqliteTables{package:"xlsx_package",part:"xlsx_binary_part",default_type:"xlsx_default_content_type",override_type:"xlsx_override_content_type",relationship_owner:"xlsx_relationship_owner",relationship:"xlsx_relationship"};
const XML:XmlSqliteTables=XmlSqliteTables{document:"xlsx_xml_document",node:"xlsx_xml_node",element:"xlsx_xml_element",text:"xlsx_xml_text",cdata:"xlsx_xml_cdata",comment:"xlsx_xml_comment",processing_instruction:"xlsx_xml_processing_instruction",attribute:"xlsx_xml_attribute",child:"xlsx_xml_child",document_misc:"xlsx_xml_document_misc",declaration:"xlsx_xml_declaration",doctype:"xlsx_xml_doctype",entity:"xlsx_xml_entity"};
fn add(total:&mut usize,value:usize)->Result<(),String>{*total=total.checked_add(value).ok_or("XLSX relational size overflow")?;Ok(())}
struct Documents(Vec<XmlSnapshot>);
impl Drop for Documents{fn drop(&mut self){for snapshot in self.0.drain(..){snapshot.retire_sqlite_snapshot();}}}
struct Parts(Vec<XlsxXmlPart>);
impl Drop for Parts{fn drop(&mut self){for part in self.0.drain(..){XmlSnapshot{schema:part.path,doc:part.document}.retire_sqlite_snapshot();}}}
impl ArtifactSqliteSnapshot for XlsxSnapshot{
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,_database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.xlsx"||dialect.standard!="ecma-376"{return Err("XLSX owned subset has no declared semantic validator".to_string().into())}match dialect.subset.as_str(){"*"=>Ok(store::io_schema::IoOutcome::clean(())),"strict"|"transitional"=>super::subset::validate(self,&dialect.subset,control),_=>Err("XLSX owned subset has no declared semantic validator".to_string().into())}}
 fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{super::native::encode(self,encoding,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{super::native::decode(payload,control)}
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn retire_sqlite_snapshot(self){drop(Parts(self.xml_parts));}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;let(mut rows,mut bytes)=measure_opc_package(&self.opc,control)?;add(&mut rows,1)?;add(&mut bytes,16)?;add(&mut bytes,self.schema.len())?;add(&mut rows,self.xml_parts.len())?;control.check_rows(rows.checked_add(self.xml_parts.len()).ok_or("XLSX document count overflow")?)?;
  let mut documents=Vec::new();documents.try_reserve_exact(self.xml_parts.len()).map_err(|_|"XLSX document reference allocation")?;
  for(position,part)in self.xml_parts.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,position,self.xml_parts.len())?;}add(&mut bytes,32)?;add(&mut bytes,part.content_type.len())?;control.check_value_bytes(bytes)?;documents.push((part.path.as_str(),&part.document));}
  let(xml_rows,xml_bytes)=measure_xml_documents(&documents,control)?;add(&mut rows,xml_rows)?;add(&mut bytes,xml_bytes)?;control.check_rows(rows)?;control.check_value_bytes(bytes)?;
  let mut database=project_xml_documents(&documents,Self::SQLITE_SCHEMA,XML,control)?;
  let package_id=project_opc_package(&self.opc,&mut database,OPC,control)?;
  database.table_mut("xlsx_document")?.rows.push(SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Text(project_text(control,&self.schema)?),SqliteValue::Integer(package_id)]});
  for(position,part)in self.xml_parts.iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,position,self.xml_parts.len())?;}let id=i64::try_from(position+1).map_err(|_|"XLSX part identity overflow")?;database.table_mut("xlsx_xml_part")?.rows.push(SqliteRow{rowid:id,values:vec![SqliteValue::Integer(id),SqliteValue::Integer(1),SqliteValue::Integer(id-1),SqliteValue::Text(project_text(control,&part.content_type)?),SqliteValue::Integer(id)]});}
  control.check_database(&database,SqliteSnapshotPhase::ProjectSnapshot)?;Ok(database)
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|error|error.to_string())?;
  let row=database.table("xlsx_document")?.single_row()?;if row.rowid!=1||row.values.len()!=3||row.integer(0)?!=1||row.integer(2)?!=1{return Err("XLSX document or package ownership is invalid".into());}let schema=reconstruct_text(control,row.text(1)?)?;let opc=reconstruct_opc_package(database,OPC,control)?;
  let rows=ordered_row_refs(database.table("xlsx_xml_part")?,2,control)?;let mut identities=BTreeSet::new();let mut documents=Documents(reconstruct_xml_documents(database,Self::SQLITE_SCHEMA,XML,control)?);if documents.0.len()!=rows.len(){return Err("XLSX XML part/document cardinality disagrees".into());}
  let mut parts=Parts(Vec::new());for(position,row)in rows.into_iter().enumerate(){if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,documents.0.len())?;}if row.values.len()!=5||row.rowid<=0||row.integer(0)?!=row.rowid||row.integer(1)?!=1||!identities.insert(row.rowid){return Err("XLSX part identity or document owner is invalid".into());}let target=usize::try_from(row.integer(4)?).map_err(|_|"XLSX XML document reference")?;if target!=position+1{return Err("XLSX XML part must own its ordered document identity".into());}let content_type=reconstruct_text(control,row.text(3)?)?;let snapshot=&mut documents.0[position];parts.0.push(XlsxXmlPart{path:std::mem::take(&mut snapshot.schema),content_type,document:std::mem::take(&mut snapshot.doc)});}
  Ok(Self{schema,opc,xml_parts:std::mem::take(&mut parts.0)})
 }
}
