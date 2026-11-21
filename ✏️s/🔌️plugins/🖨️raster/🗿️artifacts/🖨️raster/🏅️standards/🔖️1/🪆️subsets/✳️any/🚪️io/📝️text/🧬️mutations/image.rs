//! 🖼️ Compact image octets belong to Raster mutations, independently of snapshot and diff codecs.

use semio_framework_dsl_record::{BorrowedDslField,BorrowedShape,DslField,FieldValue,NativeSchemaControl,RecordBytes,Shape,native_encoding::FieldProjectionView};
use semio_framework_dsl_record_derive::{DslOwnedReceiving,DslRecord};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,RetireOwned,ValueError};
use semio_s_artifact_raster_raster_diff::io::text::{RasterImageColorspaceDsl,RasterImageMetadataDsl};
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioImageFrame,SemioImageMetadataEntry,SemioImageSnapshot};

#[derive(Clone,Debug,PartialEq,RetireOwned)]
pub struct RasterMutationOctets(RecordBytes);

impl RasterMutationOctets {
    fn from_slice(bytes:&[u8])->Self {
        Self(RecordBytes::try_from_slice(bytes).expect("addressable Raster image octets"))
    }

    /// 📏️ Reports the retained original octet count.
    pub(crate) fn len(&self)->usize { self.0.len() }

    /// 🏷️ Quotes the native lookup of one original page.
    pub(crate) fn lookup_work(&self,offset:usize)->Result<u64,ValueError> {
        self.0.retained_bytes().get_source_work_demand(offset).map_err(ValueError::from)
    }

    /// 📄️ Borrows the original contiguous bytes starting at the selected offset.
    pub(crate) fn page(&self,offset:usize)->Option<&[u8]> { self.0.retained_bytes().contiguous_at(offset) }
}

impl BorrowedDslField for RasterMutationOctets { const SHAPE:BorrowedShape=BorrowedShape::Bytes64; }

impl DslField for RasterMutationOctets {
    fn shape()->Shape { Shape::Bytes64 }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError> {RecordBytes::shape_controlled(control)}
    fn to_value(&self)->FieldValue {self.0.to_value()}
    fn from_value(value:&FieldValue)->Result<Self,String> {RecordBytes::from_value(value).map(Self)}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{self.0.to_value_controlled(control)}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{RecordBytes::from_value_controlled(value,control).map(Self)}
    fn projection_view(&self,path:&[usize])->Result<FieldProjectionView<'_>,ValueError>{self.0.projection_view(path)}
}

semio_framework_dsl_record::dsl_owned_scalar_receiving!(move RasterMutationOctets, |source| {
    use semio_framework_dsl_record::receiving::owned::invalid;
    if !matches!(source,Some(FieldValue::Bytes64(_))){return Err(invalid());}
    let Some(FieldValue::Bytes64(bytes))=source.take() else{unreachable!()};
    Ok(RasterMutationOctets(bytes))
});

#[derive(Clone,Debug,PartialEq,DslRecord,DslOwnedReceiving,RetireOwned)]
pub struct RasterMutationImageFrameDsl {
    pub delay_ms:u32,
    pub rgba8:RasterMutationOctets,
}

#[derive(Clone,Debug,PartialEq,DslRecord,DslOwnedReceiving,RetireOwned)]
pub struct RasterMutationImageDsl {
    pub schema:String,
    pub width:u32,
    pub height:u32,
    pub colorspace:RasterImageColorspaceDsl,
    pub bit_depth:u8,
    #[dsl(table)]
    pub frames:Vec<RasterMutationImageFrameDsl>,
    pub icc:Option<RasterMutationOctets>,
    #[dsl(table)]
    pub metadata:Vec<RasterImageMetadataDsl>,
}

impl From<&SemioImageSnapshot> for RasterMutationImageDsl {
    fn from(image:&SemioImageSnapshot)->Self {
        Self {
            schema:image.schema.clone(),width:image.width,height:image.height,
            colorspace:image.colorspace.into(),bit_depth:image.bit_depth,
            frames:image.frames.iter().map(|frame|RasterMutationImageFrameDsl {
                delay_ms:frame.delay_ms,rgba8:RasterMutationOctets::from_slice(&frame.rgba8),
            }).collect(),
            icc:image.icc.as_deref().map(RasterMutationOctets::from_slice),
            metadata:image.metadata.iter().map(RasterImageMetadataDsl::from).collect(),
        }
    }
}

impl RasterMutationImageDsl {
    /// 📸️ Moves the image header into its already prepared native content.
    pub(crate) fn take_snapshot(
        &mut self,frames:Vec<SemioImageFrame>,metadata:Vec<SemioImageMetadataEntry>,icc:Option<Vec<u8>>,
    )->SemioImageSnapshot {
        SemioImageSnapshot {
            schema:std::mem::take(&mut self.schema),width:self.width,height:self.height,
            colorspace:self.colorspace.into(),bit_depth:self.bit_depth,frames,icc,metadata,
        }
    }
}
