use crate::BmpSnapshot;

#[test]
fn owned_fixture_logical_carriers_roundtrip_and_demo_assets_match() {
 use store::{ArtifactDsl,ArtifactPack};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {let snapshot=semio_framework_pack_json::from_json_str::<BmpSnapshot>(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let text=snapshot.print_dsl();let binary=snapshot.encode_pack_with(&Default::default()).unwrap();assert_eq!(BmpSnapshot::parse_dsl(&text).unwrap(),snapshot);assert_eq!(BmpSnapshot::decode_pack_with(&binary,&Default::default()).unwrap(),snapshot);}
 let demo=crate::schema::demo_bmp_snapshot();assert_eq!(demo.print_dsl(),include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"));assert_eq!(demo.encode_pack_with(&Default::default()).unwrap(),include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio"));
}

#[test]
fn owned_native_controls_keep_precision_and_refusal_identity() {
 use crate::standards::v_v3::subsets::any::io::{decode_bmp_controlled,encode_bmp_controlled};
 use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap() {
  let snapshot=semio_framework_pack_json::from_json_str::<BmpSnapshot>(&row["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let mut encode_steps=0;let mut progress=|p:semio_framework_value::native_encoding::NativeEncodeProgress|{encode_steps+=usize::from(p.completed>0);true};let mut control=NativeEncodeControl::new(1024*1024,&mut progress);let native=encode_bmp_controlled(&snapshot,&mut control).unwrap();assert!(encode_steps>0);
  let mut decode_steps=0;let mut progress=|p:semio_framework_value::native_decoding::NativeDecodeProgress|{decode_steps+=usize::from(p.completed>0);true};let mut control=NativeDecodeControl::new(1024*1024,&mut progress);assert_eq!(decode_bmp_controlled(&native,&mut control).unwrap(),snapshot);assert!(decode_steps>0);
  let mut progress=|p:semio_framework_value::native_encoding::NativeEncodeProgress|p.completed==0;assert_eq!(encode_bmp_controlled(&snapshot,&mut NativeEncodeControl::new(1024*1024,&mut progress)).unwrap_err().kind,ValueRefusalKind::Canceled);
  let mut progress=|p:semio_framework_value::native_decoding::NativeDecodeProgress|p.completed==0;assert_eq!(decode_bmp_controlled(&native,&mut NativeDecodeControl::new(1024*1024,&mut progress)).unwrap_err().kind,ValueRefusalKind::Canceled);
  let mut progress=|_|true;assert_eq!(encode_bmp_controlled(&snapshot,&mut NativeEncodeControl::new(1,&mut progress)).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
  let mut progress=|_|true;assert_eq!(decode_bmp_controlled(&native,&mut NativeDecodeControl::new(1,&mut progress)).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
 }
}
