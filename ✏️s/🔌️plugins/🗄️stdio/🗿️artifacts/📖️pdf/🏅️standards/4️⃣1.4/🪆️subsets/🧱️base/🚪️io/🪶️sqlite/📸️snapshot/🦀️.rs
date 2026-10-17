//! 📄️ PDF 1.4 resolved pages in their declared reading order.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C, Projection,FloatColumn,FloatRow,insert_ieee754}};

const PAGE_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];

impl store::os_store::serialization::ArtifactSqliteSnapshotHost for PdfSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:sqlite_snapshot::SnapshotEncoding,control:&mut Control<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{(||->Result<(),ValueError>{control.checkpoint(Phase::EncodeNative,0,0)?;control.check_rows(self.pages.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF page count overflow"))?)?;Ok(())})()?;store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut Control<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    
    fn preflight_sqlite_snapshot_encoding(&self, _: sqlite_snapshot::SnapshotEncoding, control: &mut Control<'_>) -> Result<(),ValueError> {(||->Result<(),ValueError>{
        let mut bound = sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(4096)?;
        bound.repeated(self.schema.len(), 16)?;
        for page in &self.pages { bound.add(8192)?; bound.repeated(page.text.len(), 16)?; }
        bound.finish()
    })()}
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&Db,control:&mut Control<'_>)->store::io_schema::IoResult<()>{
        if dialect.artifact_kind!="s.stdio.pdf"||dialect.standard!="1.4"{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"PDF1.4 snapshot does not own this dialect")));}
        control.checkpoint(Phase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;
        store::io::validate_registered_snapshot_subset(dialect,self,database,control)
    }
    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for PdfSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut Control<'_>) -> Result<Db,ValueError> {(||->Result<Db,ValueError>{
        let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;
        projection.check_rows(self.pages.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF page count overflow"))?)?;
        projection.insert("pdf14_document",&[C::Text(&self.schema)])?;
        for(ordinal,page)in self.pages.iter().enumerate(){insert_ieee754(&mut projection,"pdf14_page",&[C::Integer(1),C::Integer(ordinal as i64),C::Real(page.width),C::Real(page.height),C::Text(&page.text)],PAGE_FLOATS)?;}
        projection.finish()
    })()}
fn from_sqlite_database(db:&Db,control:&mut Control<'_>)->Result<Self,ValueError>{
        use semio_framework_value::DecodedValue;
        use sqlite_snapshot::transfer;
        control.checkpoint(Phase::ReconstructSnapshot,0,0)?;
        transfer::validate_database_controlled(db,Self::SQLITE_SCHEMA,Phase::ReconstructSnapshot,control)?;
        control.check_database(db,Phase::ReconstructSnapshot)?;
        let document=db.table("pdf14_document")?.single_row()?;
        if document.rowid!=1||document.values.len()!=2||document.integer(0)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF document identity"));}
        let rows=&db.table("pdf14_page")?.rows;
        let mut sorted=transfer::reserve(rows.len(),control)?;
        for(index,row)in rows.iter().enumerate(){
            control.checkpoint(Phase::ReconstructSnapshot,index,rows.len())?;
            if row.values.len()!=10||row.rowid<=0||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF page identity or document relationship"));}
            sorted.push((row.integer(2)?,FloatRow::new(row,PAGE_FLOATS)?));
        }
        transfer::heap_sort(&mut sorted,Phase::ReconstructSnapshot,control,|left,right,_|Ok(left.1.rowid.cmp(&right.1.rowid)))?;
        for index in 1..sorted.len(){
            control.checkpoint(Phase::ReconstructSnapshot,index,sorted.len())?;
            if sorted[index-1].1.rowid==sorted[index].1.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF page identity or document relationship"));}
        }
        transfer::heap_sort(&mut sorted,Phase::ReconstructSnapshot,control,|left,right,_|Ok(left.0.cmp(&right.0)))?;
        let mut pages=DecodedValue::new(transfer::reserve(rows.len(),control)?,|pages:Vec<PageDoc>|drop(pages));
        for(index,(ordinal,row))in sorted.into_iter().enumerate(){
            let expected=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"PDF page ordinal exceeds INTEGER width"))?;
            if ordinal!=expected{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF page order must be contiguous and unique"));}
            control.checkpoint(Phase::ReconstructSnapshot,index,rows.len())?;
            pages.get_mut().push(PageDoc{width:row.real(3)?,height:row.real(4)?,text:sqlite_snapshot::artifact::Reconstruction::new(control)?.text(row.text(5)?)?});
        }
        let schema=sqlite_snapshot::artifact::Reconstruction::new(control)?.text(document.text(1)?)?;
        control.checkpoint(Phase::ReconstructSnapshot,rows.len(),rows.len())?;
        Ok(Self{schema,pages:pages.take()})
    }
}


