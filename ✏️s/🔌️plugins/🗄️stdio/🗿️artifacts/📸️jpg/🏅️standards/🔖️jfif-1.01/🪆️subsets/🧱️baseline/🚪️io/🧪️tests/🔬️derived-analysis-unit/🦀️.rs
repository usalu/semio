mod tests {
 use super::*;
 use crate::standards::v_jfif_1_01::subsets::document::io::{inspect_jpg_native_header,inspect_jpg_native_header_controlled,decode_jpg};
 fn native()->Vec<u8>{include_bytes!("../../../🧫️fixtures/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg").to_vec()}
 fn marker(bytes:&[u8],code:u8)->usize{bytes.windows(2).position(|word|word==[0xff,code]).expect("native marker")}
 #[test]
 fn native_observations_and_pure_facts_keep_profile_axes(){
  let bytes=native();let observed=inspect_jpg_native_header(&bytes.as_slice()).unwrap();
  assert!(check_baseline_facts(&observed.baseline_facts()).is_empty());
  let mut progressive=bytes.clone();let at=marker(&progressive,0xc0);progressive[at+1]=0xc2;
  let observations=inspect_jpg_native_header(&progressive.as_slice()).unwrap();
  assert_eq!(observations.sof_marker,0xc2);assert!(decode_jpg(&progressive).is_err());
  assert!(check_baseline_facts(&observations.baseline_facts()).iter().any(|d|d.code.0==CODE_SOF_MARKER));
  let mut precision=bytes.clone();precision[at+4]=12;
  assert_eq!(inspect_jpg_native_header(&precision.as_slice()).unwrap().frame.precision,12);
  let mut sampling=bytes.clone();sampling[at+11]=0x81;
  assert_eq!(inspect_jpg_native_header(&sampling.as_slice()).unwrap().frame.components[0].h_sampling,8);
  let mut dac=bytes.clone();dac.splice(2..2,[0xff,0xcc,0,4,0,0x10]);
  let observations=inspect_jpg_native_header(&dac.as_slice()).unwrap();assert!(observations.arithmetic);
  assert_eq!(observations.arithmetic_conditioning[0].value,0x10);
  assert!(check_baseline_facts(&observations.baseline_facts()).iter().any(|d|d.code.0==CODE_ARITHMETIC));
  eprintln!("[DEBUG] JPEG native SOF, precision, sampling and DAC axes retained independently");
 }
 #[test]
 fn analysis_retains_unsupported_native_profile_without_content_projection(){
  let mut bytes=native();let at=marker(&bytes,0xc0);bytes[at+1]=0xc2;
  let analysis=JpgBaselineAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
  assert!(analysis.parts.snapshot.is_none());assert_eq!(analysis.parts.observations.unwrap().sof_marker,0xc2);
  assert!(analysis.diagnostics.iter().any(|d|d.code.0==CODE_SOF_MARKER));
 }
 #[test]
 fn physical_observation_admission_obeys_budget_and_cancellation(){
  let bytes=native();let mut events=Vec::new();
  let canceled=inspect_jpg_native_header_controlled(&bytes.as_slice(),bytes.len(),&mut |visited|{events.push(visited);visited<256}).unwrap_err();
  assert_eq!(canceled.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(events,[0,256]);
  assert_eq!(inspect_jpg_native_header_controlled(&bytes.as_slice(),bytes.len()-1,&mut |_|true).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
 }
}
