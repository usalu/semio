//! 🔁️ Native JPEG export preserves logical metadata and admits independent baseline facts.
use semio_repo_test_host::{Adapter,Context,Json,Outcome};

fn independent_round_trip(ctx:&Context)->Result<Outcome,String>{
 let input=ctx.input_bytes("shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg")?;
 let bytes=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::oracle_identity_round_trip(&input)?;
 let path=ctx.artifact("oracle","baseline-reference.jpg")?;std::fs::write(&path,&bytes).map_err(|error|error.to_string())?;
 let axes=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::baseline::read_axes(&path)?;let codes=semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::baseline::verdict(&axes);if !codes.is_empty(){return Err(format!("independent native export baseline codes: {codes:?}"));}
 let projection=Json::Object(vec![("format".into(),Json::String("jpg-baseline".into())),("sofMarker".into(),Json::String(format!("{:02x}",axes.sof_marker))),("precision".into(),Json::Number(f64::from(axes.precision))),("conformance".into(),Json::Array(Vec::new()))]);
 eprintln!("[DEBUG] independent JPEG baseline export reopened through libjpeg");Ok(Outcome::with_raw(bytes,projection))
}
#[cfg(feature="sut")]
mod subject {
 use semio_repo_test_host::{Context,Json,Outcome,law};
 use semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::{document::io::{decode_jpg,encode_jpg,inspect_jpg_native_header,JpgEncodeOptions},baseline::schema::conformance::check_baseline_facts};
 use semio_s_artifact_stdio_jpg_test_oracle::standards::v_jfif_1_01::subsets::document::project_jpg_mutation;
 pub fn round_trip(ctx:&Context)->Result<Outcome,String>{
  let input=ctx.input_bytes("shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg")?;
  let base=decode_jpg(&input).map_err(|error|format!("{error:?}"))?;
  let bytes=encode_jpg(&base,&JpgEncodeOptions::default()).map_err(|error|format!("{error:?}"))?;
  law::reparsed_not_copied(&bytes,&input)?;
  let observations=inspect_jpg_native_header(&bytes).map_err(|error|format!("{error:?}"))?;
  let diagnostics=check_baseline_facts(&observations.baseline_facts());
  if !diagnostics.is_empty(){return Err(format!("native export baseline admission failed: {diagnostics:?}"));}
  let (was,now)=(project_jpg_mutation(&input)?,project_jpg_mutation(&bytes)?);
  for field in ["dimensions","jfifVersion","jfifDensity","jfifThumbnail","otherSegments"] {
   if was.get(field)!=now.get(field){return Err(format!("independent JPEG metadata changed at {field}"));}
  }
  let projection=Json::Object(vec![("format".into(),Json::String("jpg-baseline".into())),("sofMarker".into(),Json::String(format!("{:02x}",observations.sof_marker))),("precision".into(),Json::Number(f64::from(observations.frame.precision))),("conformance".into(),Json::Array(Vec::new()))]);
  eprintln!("[DEBUG] JPEG independent reopen dimensions={} baseline=SOF0",now.str("dimensions"));
  Ok(Outcome::with_raw(bytes,projection))
 }
}
/// 🧭️ Registers the physical round-trip law.
pub fn adapter()->Adapter{
 #[allow(unused_mut)]
 let mut built=Adapter::new("rust").oracle("identity-round-trip",independent_round_trip);
 #[cfg(feature="sut")]
 {built=built.subject("identity-round-trip",subject::round_trip);}
 built
}
