//! 🧰️ Controlled catalog, design, child and representation native materialization.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use crate::schema::snapshot::{SemioKitSnapshot, SemioKitType, SemioKitPiece, SemioKitConnection, SemioKitDesign, STDIO_SEMIOKIT_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use {semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity, semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint3, SemioQuaternion}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<native::Output<SemioKitSnapshot>>)->native::Outcome{
 if let Err(error)=crate::io::sqlite::snapshot::admit_layout(control.limits()){return native::refused(error)}
 native::decode_receiving(payload,STDIO_SEMIOKIT_DOCUMENT_SCHEMA,control,native_owner,output,empty,|body,output,decoder,limits|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)})
}







/// 🧵️ Creates a zero-heap artifact link whose target, pin and role are filled in place.
fn empty_link()->store::ArtifactLink{store::ArtifactLink{target:native::empty_reference(),pin:store::LinkPin::Head,role:String::new()}}
/// 🧩️ Fills the retained kit root from borrowed binary octets, inserting every catalog entry, child and link before its fields.
pub fn binary_into(output:&mut SemioKitSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::record_binary_into(output,body,control,limits)}



fn text_transform(value:&str,control:&mut NativeDecodeControl<'_>)->Result<SemioTransform,ValueError>{let[tx,ty,tz,rx,ry,rz,rw,sx,sy,sz]=native::record(value,control)?;Ok(SemioTransform{translation:SemioPoint3{x:native::float(tx,control)?,y:native::float(ty,control)?,z:native::float(tz,control)?},rotation:SemioQuaternion{x:native::float(rx,control)?,y:native::float(ry,control)?,z:native::float(rz,control)?,w:native::float(rw,control)?},scale:SemioPoint3{x:native::float(sx,control)?,y:native::float(sy,control)?,z:native::float(sz,control)?}})}





/// ⛓️ Fills one artifact reference in its retained slot from a bracketed hexadecimal record.
fn text_reference_into(output:&mut semio_framework_artifact_reference::ArtifactRef,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[id,kind,standard,subset]=native::record(value,control)?;native::hex_text_into(&mut output.artifact_id,id,control)?;native::hex_text_into(&mut output.dialect.artifact_kind,kind,control)?;native::hex_text_into(&mut output.dialect.standard,standard,control)?;native::hex_text_into(&mut output.dialect.subset,subset,control)}
/// 🧸️ Fills one child handle in its retained slot from a bracketed hexadecimal record.
fn text_child_into<S>(output:&mut store::os_store::artifact_child::ArtifactChild<S>,value:&str,subset:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{native::entities(entities,1,limits)?;let[id,target]=native::record(value,control)?;native::hex_text_into(&mut output.child_id,id,control)?;text_reference_into(&mut output.target,target,control)?;validate_semio_child_identity(&output.child_id,&output.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))}
/// 🧫️ Fills one catalog type in its retained slot from a bracketed hexadecimal record.
fn text_type_into(output:&mut SemioKitType,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[id,name,category]=native::record(value,control)?;native::hex_text_into(&mut output.id,id,control)?;native::hex_text_into(&mut output.name,name,control)?;native::hex_text_into(&mut output.category,category,control)}
/// 🪀️ Fills one design piece in its retained slot from a bracketed hexadecimal record.
fn text_piece_into(output:&mut SemioKitPiece,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[id,type_id,transform]=native::record(value,control)?;native::hex_text_into(&mut output.id,id,control)?;native::hex_text_into(&mut output.type_id,type_id,control)?;output.transform=text_transform(transform,control)?;Ok(())}
/// 🪢️ Fills one design connection in its retained slot from a bracketed hexadecimal record.
fn text_connection_into(output:&mut SemioKitConnection,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[id,a,ap,b,bp]=native::record(value,control)?;native::hex_text_into(&mut output.id,id,control)?;native::hex_text_into(&mut output.connecting_piece_id,a,control)?;native::hex_text_into(&mut output.connecting_port,ap,control)?;native::hex_text_into(&mut output.connected_piece_id,b,control)?;native::hex_text_into(&mut output.connected_port,bp,control)}
/// 📏️ Fills one design in its retained slot, inserting every piece and connection before its fields.
fn text_design_into(output:&mut SemioKitDesign,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{let[id,name,pieces,connections]=native::record(value,control)?;native::hex_text_into(&mut output.id,id,control)?;native::hex_text_into(&mut output.name,name,control)?;native::text_list_into(&mut output.pieces,pieces,control,limits,entities,Default::default,|piece,value,control,_|text_piece_into(piece,value,control))?;native::text_list_into(&mut output.connections,connections,control,limits,entities,Default::default,|connection,value,control,_|text_connection_into(connection,value,control))}
/// 📍️ Selects each text pin variant in its retained slot before copying its fields.
fn text_pin_into(output:&mut store::LinkPin,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{if value=="[h]"{*output=store::LinkPin::Head;return Ok(())}let inner=value.strip_prefix('[').and_then(|value|value.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Kit native pin requires brackets"))?;match inner.split_once(',').map(|(tag,_)|tag){Some("c")=>{let[_,id]=native::record(value,control)?;*output=store::LinkPin::Checkpoint{id:String::new()};let store::LinkPin::Checkpoint{id:slot}=&mut *output else{unreachable!()};native::hex_text_into(slot,id,control)},Some("s")=>{native::entities(entities,1,limits)?;let[_,hash,size,media_type]=native::record(value,control)?;let size=control.scoped_stage(|control|{control.begin_stage(size.len())?;for _ in size.bytes(){control.step()?;}size.parse::<u64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Kit native invalid blob size"))})?;*output=store::LinkPin::Snapshot{blob:store::BlobRef{hash:String::new(),size,media_type:String::new()}};let store::LinkPin::Snapshot{blob}=&mut *output else{unreachable!()};native::hex_text_into(&mut blob.hash,hash,control)?;native::hex_text_into(&mut blob.media_type,media_type,control)},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Kit native invalid pin shape"))}}
/// 🔠️ Fills one representation link in its retained slot from a bracketed hexadecimal record.
fn text_link_into(output:&mut store::ArtifactLink,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{native::entities(entities,1,limits)?;let[target,pin,role]=native::record(value,control)?;text_reference_into(&mut output.target,target,control)?;text_pin_into(&mut output.pin,pin,control,limits,entities)?;native::hex_text_into(&mut output.role,role,control)}
/// 📝️ Fills the retained kit root from the borrowed document body, inserting every catalog entry, child and link before its fields.
pub fn document_into(output:&mut SemioKitSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{crate::io::sqlite::snapshot::admit_document(body,control,limits)?;let fields=native::fields(body,["schema","types","designs","objects","models","properties","representations"],control)?;let mut entities=0;native::entities(&mut entities,1,limits)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Kit native schema missing"))?,control)?;
 native::text_list_into(&mut output.types,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|entry,value,control,_|text_type_into(entry,value,control))?;native::text_list_into(&mut output.designs,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|design,value,control,entities|text_design_into(design,value,control,limits,entities))?;native::text_list_into(&mut output.objects,fields[3].unwrap_or("[]"),control,limits,&mut entities,native::empty_child,|child,value,control,entities|text_child_into(child,value,"object",control,limits,entities))?;native::text_list_into(&mut output.models,fields[4].unwrap_or("[]"),control,limits,&mut entities,native::empty_child,|child,value,control,entities|text_child_into(child,value,"model",control,limits,entities))?;let properties=fields[5].unwrap_or("[]");if properties!="[]"{native::entities(&mut entities,1,limits)?;text_child_into(output.properties.insert(native::empty_child()),properties,"value",control,limits,&mut entities)?}
 native::text_list_into(&mut output.representations,fields[6].unwrap_or("[]"),control,limits,&mut entities,empty_link,|link,value,control,entities|text_link_into(link,value,control,limits,entities))}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioKitSnapshot{SemioKitSnapshot{schema:String::new(),types:Vec::new(),designs:Vec::new(),objects:Vec::new(),models:Vec::new(),properties:None,representations:Vec::new()}}
