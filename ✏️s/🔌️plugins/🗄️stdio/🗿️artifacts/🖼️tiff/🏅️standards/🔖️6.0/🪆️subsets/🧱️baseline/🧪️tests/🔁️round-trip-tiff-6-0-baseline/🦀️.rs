//! 🔁️ Canonical owned TIFF export reopens through the independent TIFF reader.
use semio_repo_test_host::{Adapter,Context,Outcome};
const SCAN:&str="shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff";
fn reference(ctx:&Context)->Result<Outcome,String>{
 use semio_s_artifact_stdio_tiff_test_oracle::standards::v6_0::subsets::{document::{oracle_identity_round_trip,project_tiff},baseline::{read_axes,verdict}};
 let input=ctx.input_bytes(SCAN)?;let bytes=oracle_identity_round_trip(&input)?;
 let codes=verdict(&read_axes(&bytes)?);if !codes.is_empty(){return Err(format!("independent native TIFF baseline codes {codes:?}"))}
 let result=project_tiff(&bytes)?;
 semio_repo_test_host::law::round_trip_preserves(&result,&project_tiff(&input)?)?;
 eprintln!("[DEBUG] independent TIFF baseline export {}",result.to_string());
 Ok(Outcome::with_raw(bytes,result))
}
#[cfg(feature="sut")]
mod subject{
 use super::*;
 use semio_s_artifact_stdio_tiff::standards::v6_0::subsets::{document::io::{decode_tiff,encode_tiff,controlled_decoding::inspect_tiff_native},baseline::io::native_conformance::classify_tiff_native_baseline};
 use semio_s_artifact_stdio_tiff_test_oracle::standards::v6_0::subsets::document::project_tiff;
 pub fn round_trip(ctx:&Context)->Result<Outcome,String>{
  let input=ctx.input_bytes(SCAN)?;let base=decode_tiff(&input)?;
  let bytes=encode_tiff(&base)?;let reparsed=decode_tiff(&bytes)?;
  if reparsed!=base{return Err("TIFF native round trip changed canonical owned samples or metadata".into())}
  let codes=classify_tiff_native_baseline(&inspect_tiff_native(&bytes)?);
  if !codes.is_empty(){return Err(format!("exported TIFF native baseline codes {codes:?}"))}
  let result=project_tiff(&bytes)?;
  semio_repo_test_host::law::round_trip_preserves(&result,&project_tiff(&input)?)?;
  let mut changed=base.clone();
  let first=changed.ifds.first_mut().and_then(|p|p.blocks.first_mut()).and_then(|b|b.samples.first_mut()).ok_or("no exact sample to perturb")?;
  first.lo^=1;let altered=encode_tiff(&changed)?;
  if altered==bytes||project_tiff(&altered)?.get("samplesDigest")==result.get("samplesDigest"){return Err("an exact sample edit did not reach independently decoded native bytes".into())}
  let again=encode_tiff(&reparsed)?;
  semio_repo_test_host::law::round_trip_preserves(&project_tiff(&again)?,&result)?;
  eprintln!("[DEBUG] TIFF baseline owned round trip independently reopened {}",result.to_string());
  Ok(Outcome::with_raw(bytes,result))
 }
}
/// 🧭️ Registers both independent and first-party native round-trip roles.
pub fn adapter()->Adapter{
 #[allow(unused_mut)]
 let mut built=Adapter::new("rust").oracle("identity-round-trip",reference);
 #[cfg(feature="sut")]
 {built=built.subject("identity-round-trip",subject::round_trip);}
 built
}
