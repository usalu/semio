//! 🎨️ SVG's explicit relational vocabulary over its native typed SvgDocument fields.

use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
use crate::schema::snapshot::{SvgDocument, SvgNode};
use semio_os_kernel as store;
use semio_os_kernel::{sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl}, os_store::serialization::ArtifactSqliteSnapshotHost};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::sqlite::snapshot::{project_xml_document, reconstruct_xml_document, XmlSqliteTables};

const TABLES: XmlSqliteTables = XmlSqliteTables {
    document: "svg_document", node: "svg_node", element: "svg_element", text: "svg_text", cdata: "svg_cdata", comment: "svg_comment",
    processing_instruction: "svg_processing_instruction", attribute: "svg_attribute", child: "svg_child", document_misc: "svg_document_misc",
    declaration: "svg_declaration", doctype: "svg_doctype", entity: "svg_entity",
};

fn validate_root(doc: &SvgDocument) -> Result<(), ValueError> { doc.validate_attribute_owners().map_err(|detail|ValueError::new(ValueRefusalKind::InvalidValue,detail))?;match &doc.root { Some(SvgNode::Element { name, .. }) if name == "svg" || name.ends_with(":svg") => Ok(()), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"SVG snapshot requires an svg root element")) } }

impl ArtifactSqliteSnapshotHost for SvgSnapshot {
    fn decode_sqlite_snapshot_native(payload:&semio_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome{
        let limits=control.limits();
        store::space_history::io::sqlite::snapshot::settlement::decode_typed(control,native_owner,output,|native|{
            let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native::decode(payload,limits,native)?,|value:Self|crate::schema::snapshot::retire_svg_document(value.doc));
            validate_root(&owner.as_mut().doc)?;
            Ok(owner.take())
        })
    }
    fn encode_sqlite_snapshot_native(&self,encoding:semio_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<semio_os_kernel::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {
        if let Err(error)=validate_root(&self.doc){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}
        let limits=control.limits();
        store::space_history::io::sqlite::snapshot::settlement::encode_typed(control,native_owner,output,|native|native::encode(self,encoding,limits,native))
    }
    fn retire_sqlite_snapshot(self){crate::schema::snapshot::retire_svg_document(self.doc)}
    fn preflight_sqlite_snapshot_encoding(&self, encoding: semio_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(),ValueError> { native::preflight(self,encoding,control) }
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_os_kernel::io_schema::IoResult<()>{
        control.checkpoint(semio_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(semio_os_kernel::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind!="s.stdio.svg"||dialect.standard!="1.1"{return Err(semio_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"SVG owned snapshot dialect differs from SVG 1.1")));}
        validate_root(&self.doc).map_err(semio_os_kernel::io_schema::IoError::from_value_error)?;let row=database.table("svg_document").map_err(semio_os_kernel::io_schema::IoError::from_value_error)?.single_row().map_err(semio_os_kernel::io_schema::IoError::from_value_error)?;if row.rowid!=1||row.text(1).map_err(semio_os_kernel::io_schema::IoError::from_value_error)?!=self.schema{return Err(semio_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"SVG owned document identity differs from semantic projection")));}
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"tiny"=>crate::standards::v1_1::subsets::tiny::schema::conformance::check_svg_tiny_conformance_controlled(self,control).map_err(semio_os_kernel::io_schema::IoError::from_value_error)?,"basic"=>crate::standards::v1_1::subsets::basic::schema::conformance::check_svg_basic_conformance_controlled(self,control).map_err(semio_os_kernel::io_schema::IoError::from_value_error)?,_=>return Err(semio_os_kernel::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner,"SVG named subset has no owned semantic validator")))};Ok(semio_os_kernel::io_schema::IoOutcome{value:(),diagnostics})
    }

    
    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for SvgSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase,ValueError> {
         validate_root(&self.doc)?; let mut native=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(native::project_document(&self.doc,control)?,semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);project_xml_document(&self.schema, native.as_mut(), Self::SQLITE_SCHEMA, TABLES, control) 
        
    }
fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {
         let (schema, doc) = reconstruct_xml_document(database, Self::SQLITE_SCHEMA, TABLES, control)?; let doc=native::admit_document(doc,control)?;let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self{schema,doc},|value:Self|crate::schema::snapshot::retire_svg_document(value.doc));validate_root(&owner.as_mut().doc)?;Ok(owner.take())
        
    }
}

use semio_os_kernel::sqlite_snapshot::{ValueError, ValueRefusalKind};



#[path = "🚦️native/🦀️.rs"]
pub(crate) mod native;
