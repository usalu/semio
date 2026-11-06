//! 🥡️ Typed default placement denies missing copy and empty arrays remain genuine values.
use semio_framework_value_native_default::{NativeDefault,NativeDefaultValue,NativeDefaultCursor,NativeArrayDefaultCursor};
use semio_framework_value_testing_admission::{self as fixture,Wallet,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value_native_decoding::native_decoding::NativeDecodeControl;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[test]
fn empty_string_default_requires_its_real_option_placement_and_never_allocates(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️work/🥡️placement/🔣️.json")).unwrap();let mut bank=Wallet::from_profile(&law["stock"]);let mut output=None;let mut actual=None;let mut grant=bank.loan(1,4096);grant.maximum_copy_bytes=0;
 let(turn,heap)=observe(||<String as NativeDefaultValue>::admit_default(&mut output,grant,Some(&mut actual)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(output,None);assert_eq!(actual,None);assert_eq!(heap,(0,0));
 let demand=<String as NativeDefaultValue>::default_demand();assert_eq!(demand.copy_bytes,std::mem::size_of::<Option<String>>());assert!(demand.copy_bytes>0);let grant=bank.loan(1,demand.copy_bytes);assert!(<String as NativeDefaultValue>::default_work_demand().unwrap()<=bank.work_remaining());
 let(turn,heap)=observe(||<String as NativeDefaultValue>::admit_default(&mut output,grant,Some(&mut actual)));let p=turn.result.unwrap().progress();bank.post(p,turn.work,grant,heap);assert_eq!(turn.work,1);assert_eq!(heap,(0,0));assert_eq!(actual,None);assert_eq!(serde_json::to_value(output.as_ref().unwrap()).unwrap(),law["string"]);fixture::close_owned(output.take().unwrap(),&mut bank,4096,10000);
}
#[test]
fn empty_array_denies_zero_work_then_publishes_the_real_zero_length_value(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🎟️work/🥡️placement/🔣️.json")).unwrap();let mut bank=Wallet::from_profile(&law["stock"]);let(mut cursor,heap)=observe(NativeArrayDefaultCursor::<u64,0>::new);assert_eq!(heap,(0,0));let mut checkpoint=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut checkpoint);let grant=bank.loan(1,4096);
 let(turn,heap)=observe(||fixture::receive(&mut control,grant,bank.work_loan(0),|native|cursor.advance(native,4096,0,grant)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert!(!cursor.output_ready());
 for _ in 0..10000{if cursor.output_ready(){break}let maximum=bank.work_remaining();let q=cursor.advance_work_demand(4096,maximum).unwrap();assert!(q>0&&q<=maximum);let grant=bank.loan(1,4096);let(turn,heap)=observe(||fixture::receive(&mut control,grant,bank.work_loan(maximum),|native|cursor.advance(native,4096,maximum,grant)));assert!(turn.work<=q);let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|error|error.retained_progress());bank.post(p,turn.work,grant,heap);turn.result.unwrap();}
 assert!(cursor.output_ready());let grant=RetainedCloneGrant{maximum_copy_bytes:0,..bank.loan(1,4096)};let(turn,heap)=observe(||cursor.take(grant));let(value,p)=turn.result.unwrap().unwrap();bank.post(p,turn.work,grant,heap);assert_eq!(serde_json::to_value(value).unwrap(),law["emptyArray"]);cursor.begin_close().unwrap();let mut actual=None;
 for _ in 0..10000{if cursor.terminal_is_empty()&&actual.is_none(){break}let grant=bank.loan(1,4096);let(turn,heap)=observe(||fixture::close_step(&mut actual,grant,bank.work_remaining(),cursor.close_demands(4096),cursor.close_work_demand(4096),|g,a|cursor.close_step(g,a)));let p=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());bank.post(p,turn.work,grant,heap);turn.result.unwrap();}
 assert!(cursor.terminal_is_empty());assert!(actual.is_none());assert_eq!(observe(||drop(cursor)).1,(0,0));
}
