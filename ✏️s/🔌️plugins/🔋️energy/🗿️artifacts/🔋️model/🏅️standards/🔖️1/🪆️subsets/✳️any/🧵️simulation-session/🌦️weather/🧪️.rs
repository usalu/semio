//! 🧪️ Actual registry replies, canonical EPW packs and independent CSV values drive original weather capture.
use super::*;
use semio_framework_os_kernel::pack::snapshot::ArtifactPack;
use semio_framework_plugin_actor_host::requests::RequestRegistry;
use semio_framework::kernel::{Effect,RequestId};
use semio_s_plugin_energy_engine_sim::testing::OriginalEnergyCaller;
use semio_framework_job::{OperationId,Generation};

static ANNUAL:std::sync::LazyLock<&'static str>=std::sync::LazyLock::new(||semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/🌦️denver-tmy/🌦️.epw"));
static EPW:std::sync::LazyLock<&'static str>=std::sync::LazyLock::new(||semio_framework_test_fixtures::fixture_text!("../../../../../../../../../../🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🖼️assets/🎬️demo/🧪️example/🌦️.epw"));

#[test]
fn linked_weather_uses_original_pack_and_matches_independent_csv_for_two_sources(){ capture_sources(*EPW,&[0.0,20.0],24); }

#[test]
fn annual_linked_weather_decodes_under_original_one_mib_birth_grants(){ capture_sources(*ANNUAL,&[0.0],8760); }

fn capture_sources(text:&str,offsets:&[f64],records:usize){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let expected:Vec<f64>=csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(text.as_bytes()).records().skip(8).map(|row|row.unwrap()[6].parse().unwrap()).collect();
 let mut digests=Vec::new();
 for &offset in offsets {
  let mut epw=semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::io::decode_epw(text).unwrap();
  for row in &mut epw.records { row.dry_bulb_temp=(row.dry_bulb_temp.parse::<f64>().unwrap()+offset).to_string(); }
  let bytes=epw.encode_pack();
  let pointer=bytes.as_ptr();
  let digest=semio_framework_hash::Sha256::digest(&bytes);
  let registry=RequestRegistry::new();
  let port=ToolRunJobPort::default();
  let receiver=registry.for_instance(71).request_retained(port.request_waker(),|req|Effect::LinkResolve{req,link:"captured-weather".into()}).unwrap();
  assert!(matches!(&registry.drain()[0],Effect::LinkResolve{req,link}if req.0==1&&link=="captured-weather"));
  let mut source=LinkedWeather::new(receiver,port);
  let mut original=OriginalEnergyCaller::new(OperationId(710),Generation(1));
  let grant=crate::sim::ENERGY_BATCH_GRANT;
  let mut output=None;
  original.turn(grant,1,u64::MAX,|cx|source.step(cx,&mut output)).unwrap();
  assert!(output.is_none());
  let mut reply=Some(Ok(bytes));
  assert!(registry.resolve_original(RequestId(1),&mut reply));
  assert!(reply.is_none());
  registry.release_completed(RequestId(1));
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){
   if original.turn(grant,32,u64::MAX,|cx|{
    for _ in 0..32 {if cx.should_yield(){break;}if source.step(cx,&mut output)?{return Ok(true);}}
    Ok(false)
   }).unwrap(){break;}
  }
  let result=output.as_ref().expect("the actual linked weather source must complete");
  assert_eq!(result.records.len(),records);
  assert_eq!(source.owned.wire.as_ref().unwrap().as_ptr(),pointer);
  assert_eq!(source.digest,Some(digest));
  assert_eq!(result.records.iter().map(|row|row.dry_bulb_c).collect::<Vec<_>>(),expected.iter().map(|value|value+offset).collect::<Vec<_>>());
  assert!(!result.location.is_empty());
  digests.push(source.digest.unwrap());
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break;}original.turn(grant,32,u64::MAX,|cx|cx.with_original_retirement(|cx|source.close_step(cx))).unwrap();}
  assert!(source.terminal_is_empty());
  drop(source);
  let mut result=ControlledRetirement::new(output.take()).ok().unwrap();
  for _ in 0..100000{if result.terminal_is_empty(){break;}original.turn(grant,32,u64::MAX,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,result.selected_demand(4096)?,|grant|result.step(grant)))).unwrap();}
  assert!(result.terminal_is_empty());
  drop(result);
  original.finish();
 }
 if digests.len()>1 { assert_ne!(digests[0],digests[1]); }
}
