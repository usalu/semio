//! 👁️ Native TIFF6 Baseline conformance profiles; physical policy stays at IO.
use semio_repo_test_host::{Adapter,Context,Json,Outcome};
use semio_s_artifact_stdio_tiff_test_oracle::standards::v6_0::subsets::baseline::{profile,project,read_axes,Axes};
const SCAN:&str="shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff";
fn prepared_axes(ctx:&Context,row:&Json)->Result<Axes,String>{
 let axes=read_axes(&ctx.input_bytes(SCAN)?)?;
 let setup=row.get("setup").cloned().unwrap_or(Json::Object(Vec::new()));
 if setup.str("kind").is_empty(){Ok(axes)}else{profile(&axes,&setup.str("kind"),&setup.get("params").cloned().unwrap_or(Json::Null))}
}
fn reference(ctx:&Context)->Result<Outcome,String>{
 let row=ctx.doc_json()?;let base=prepared_axes(ctx,&row)?;
 let next=profile(&base,&row.str("kind"),&row.get("params").cloned().unwrap_or(Json::Null))?;
 let result=project(&next);
 if result==project(&base){return Err("native profile did not move its observation".into())}
 eprintln!("[DEBUG] independent TIFF native profile {} {}",row.str("kind"),result.to_string());
 Ok(Outcome::with_raw(result.to_string().into_bytes(),result))
}
#[cfg(feature="sut")]
mod subject{
 use super::*;
 use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::{document::io::{controlled_decoding::inspect_tiff_native,native_observations::TiffNativeObservations},baseline::io::native_conformance::classify_tiff_native_baseline};
 fn words(params:&Json,key:&str)->Result<Vec<u32>,String>{
  params.array(key).iter().map(|v|match v{Json::Number(v)if v.is_finite()&&v.fract()==0.0&&(0.0..=u32::MAX as f64).contains(v)=>Ok(*v as u32),_=>Err("native profile word is not a UInt32".into())}).collect()
 }
 fn one(params:&Json,key:&str)->Result<Vec<u32>,String>{
  let value=params.get(key).ok_or("native profile scalar missing")?;
  words(&Json::Object(vec![(key.into(),Json::Array(vec![value.clone()]))]),key)
 }
 fn select(base:&TiffNativeObservations,kind:&str,params:&Json)->Result<TiffNativeObservations,String>{
  let mut next=base.clone();
  match kind{
   "set-compression"=>next.compression=Some(one(params,"compression")?),
   "set-photometric-interpretation"=>next.photometric=Some(one(params,"photometric")?),
   "set-bits-per-sample"=>next.bits_per_sample=Some(words(params,"bits")?),
   "insert-tile-tags"=>{next.tile_width=Some(one(params,"tileWidth")?);next.tile_length=Some(one(params,"tileLength")?);}
   "remove-tile-tags"=>{next.tile_width=None;next.tile_length=None;}
   "set-strip-offsets"=>next.strip_offsets=Some(words(params,"offsets")?),
   "remove-strip-offsets"=>next.strip_offsets=None,
   _=>return Err("unknown native conformance profile".into()),
  }Ok(next)
 }
 fn projection(o:&TiffNativeObservations)->Json{
  let list=|v:&Option<Vec<u32>>|Json::String(v.as_ref().map(|v|v.iter().map(u32::to_string).collect::<Vec<_>>().join(" ")).unwrap_or_else(||"absent".into()));
  Json::Object(vec![("format".into(),Json::String("tiff-baseline".into())),("ifdCount".into(),Json::Number(o.ifd_count as f64)),("compression".into(),list(&o.compression)),("photometric".into(),list(&o.photometric)),("bitsPerSample".into(),list(&o.bits_per_sample)),("tileWidth".into(),list(&o.tile_width)),("tileLength".into(),list(&o.tile_length)),("stripOffsets".into(),list(&o.strip_offsets)),("conformance".into(),Json::Array(classify_tiff_native_baseline(o).into_iter().map(|c|Json::String(c.into())).collect()))])
 }
 pub fn classify(ctx:&Context)->Result<Outcome,String>{
  let row=ctx.doc_json()?;let mut base=inspect_tiff_native(&ctx.input_bytes(SCAN)?)?;
  let setup=row.get("setup").cloned().unwrap_or(Json::Object(Vec::new()));
  if !setup.str("kind").is_empty(){base=select(&base,&setup.str("kind"),&setup.get("params").cloned().unwrap_or(Json::Null))?;}
  let next=select(&base,&row.str("kind"),&row.get("params").cloned().unwrap_or(Json::Null))?;
  let codes=classify_tiff_native_baseline(&next);let expected=row.str("code");
  let expected=if expected.is_empty(){Vec::new()}else{vec![expected.as_str()]};
  if codes!=expected{return Err(format!("native conformance profile exact codes {codes:?}, expected {expected:?}"))}
  let result=projection(&next);
  semio_repo_test_host::law::mutation_is_observable(&row.str("kind"),&result,&projection(&base),&[])?;
  eprintln!("[DEBUG] TIFF native profile {} {}",row.str("kind"),result.to_string());
  Ok(Outcome::with_raw(result.to_string().into_bytes(),result))
 }
}
/// 🧭️ Registers IO observations independently of authored mutation dispatch.
pub fn adapter()->Adapter{
 #[allow(unused_mut)]
 let mut built=Adapter::new("rust").oracle("classify",reference);
 #[cfg(feature="sut")]
 {built=built.subject("classify",subject::classify);}
 built
}
