//! 🖍️ Genuine typed Draw reception is compared with serde over the shared complete authored Drawing corpus.
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::native_decoding::NativeReceive;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[path="../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
const BODY:usize=4096;
#[test]fn complete_genuine_drawing_receives_original_native_fields_and_typed_recursive_children(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../✅️validation/🧪️.json")).unwrap();let input=&corpus["base"];
 let expected:DrawingSnapshot=serde_json::from_value(input.clone()).unwrap();
 for cut in [Some(0),Some(1),Some(20),Some(200),Some(100000),None]{
  let mut bank=driver::bank();let source=driver::admit(&mut bank,DslValue::from(input.clone()));
  let published=driver::lifecycle(<DrawingSnapshot as NativeReceive<DslValue>>::receiver(),source,&mut bank,BODY,cut,|output|assert_eq!(output,&expected)).unwrap();
  assert_eq!(published,cut.is_none());
 }
 let mut bank=driver::bank();driver::retire(expected,&mut bank);
}
