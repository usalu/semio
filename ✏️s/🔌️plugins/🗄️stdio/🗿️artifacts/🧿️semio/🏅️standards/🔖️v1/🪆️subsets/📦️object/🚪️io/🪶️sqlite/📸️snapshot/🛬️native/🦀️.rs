//! 📦️ Controlled spatial placement and literal persisted child identities.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use crate::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use {semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity, semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint3, SemioQuaternion}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<native::Output<SemioObjectSnapshot>>)->native::Outcome{
 if let Err(error)=crate::io::sqlite::snapshot::admit_layout(control.limits()){return native::refused(error)}
 native::decode_receiving(payload,STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA,control,native_owner,output,empty,|body,output,decoder,limits|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)})
}


/// 🧩️ Fills the retained object root from borrowed binary octets, selecting each child before copying its identity.
pub fn binary_into(output:&mut SemioObjectSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::record_binary_into(output,body,control,limits)}

/// 🧸️ Fills one optional persisted child in its original handle from the borrowed document field.
fn text_child_into<S>(output:&mut Option<store::os_store::artifact_child::ArtifactChild<S>>,value:&str,subset:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 if value=="[]"{return Ok(())}native::entities(entities,2,limits)?;let[id,target]=native::record(value,control)?;let[artifact_id,kind,standard,declared_subset]=native::record(target,control)?;
 let child=output.insert(native::empty_child());native::hex_text_into(&mut child.child_id,id,control)?;native::hex_text_into(&mut child.target.artifact_id,artifact_id,control)?;native::hex_text_into(&mut child.target.dialect.artifact_kind,kind,control)?;native::hex_text_into(&mut child.target.dialect.standard,standard,control)?;native::hex_text_into(&mut child.target.dialect.subset,declared_subset,control)?;validate_semio_child_identity(&child.child_id,&child.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))
}
/// 📝️ Fills the retained object root from the borrowed document body, selecting each child before copying its identity.
pub fn document_into(output:&mut SemioObjectSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?; let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","transform","brep","mesh","properties"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Object native schema missing"))?,control)?;
 let[tx,ty,tz,rx,ry,rz,rw,sx,sy,sz]=native::record(fields[1].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Object native transform missing"))?,control)?;output.transform=SemioTransform{translation:SemioPoint3{x:native::float(tx,control)?,y:native::float(ty,control)?,z:native::float(tz,control)?},rotation:SemioQuaternion{x:native::float(rx,control)?,y:native::float(ry,control)?,z:native::float(rz,control)?,w:native::float(rw,control)?},scale:SemioPoint3{x:native::float(sx,control)?,y:native::float(sy,control)?,z:native::float(sz,control)?}};
 text_child_into(&mut output.brep,fields[2].unwrap_or("[]"),"brep",control,limits,&mut entities)?;text_child_into(&mut output.mesh,fields[3].unwrap_or("[]"),"mesh",control,limits,&mut entities)?;text_child_into(&mut output.properties,fields[4].unwrap_or("[]"),"value",control,limits,&mut entities)?;output.validate().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioObjectSnapshot{SemioObjectSnapshot{schema:String::new(),transform:SemioTransform::identity(),brep:None,mesh:None,properties:None}}
