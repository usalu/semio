//! 🪜️ A multi-unit grammar turn funded for its first unit's frontier depth ends its turn when a later unit needs more, and the next turn quotes it; it never refuses.
use semio_framework_pack_json::{to_dsl_value,to_json_string,JsonMemberPolicy,JsonParseCursor};
use semio_framework_value::{retained_clone::AuthoringWallet,RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

#[test]
fn a_turn_funded_for_its_first_unit_depth_resumes_deeper_on_the_next_turn(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let units=fixture["turnUnits"].as_u64().unwrap() as usize;
    for text in fixture["documents"].as_array().unwrap(){
        let text=text.as_str().unwrap();let oracle:serde_json::Value=serde_json::from_str(text).unwrap();
        let mut cursor=JsonParseCursor::new(JsonMemberPolicy::Reject);let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1<<26,&mut accepted);
        let mut wallet=AuthoringWallet::new(RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<28,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096},1<<24);
        let mut turns=0;
        let value=loop{
            turns+=1;assert!(turns<10000,"{text}: parse never settled");
            let depth=cursor.normal_step_demands(text).unwrap().depth;let grant=RetainedCloneGrant{maximum_items:units,maximum_depth:depth,..wallet.remaining()};
            let done=wallet.run(grant,units as u64,|effect|cursor.step(text,units,&mut control,effect)).unwrap().expect("quote admitted");wallet.post(cursor.normal_step_progress()).unwrap();
            if let Some(value)=done.output.unwrap_or_else(|error|panic!("{text}: turn {turns} refused as {error:?}")){break value}
        };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&to_dsl_value(&value))).unwrap(),oracle,"{text}");
    }
}
