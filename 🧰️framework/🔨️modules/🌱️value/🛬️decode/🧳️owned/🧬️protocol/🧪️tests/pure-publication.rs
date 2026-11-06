use semio_framework_value_native_receiver_protocol::{native_output_demand,native_output_take,RetainedCloneProgress,RetainedCloneStep,RetainedWorkPoll};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

#[test]
fn denied_publication_keeps_original_then_larger_turn_moves_it(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️publication.json")).unwrap();
 let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));
 let mut source=Some(law["source"].as_str().unwrap().to_owned());let original=source.as_ref().unwrap().as_ptr();let mut output=None;
 let denied=bank.loan(1,0);let(poll,marker)=bank.run(denied,1,|effect|native_output_take(&mut source,&mut output,0,effect));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("unstarted denial retained a live action")};assert_eq!(turn.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert!(matches!(turn.result.unwrap(),RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()));assert_eq!(law["denied"],"completedNoProgress");assert_eq!(marker,None);assert!(output.is_none());assert_eq!(source.as_ref().unwrap().as_ptr(),original);assert_eq!(bank.take_admitted(),None);bank.post(RetainedCloneProgress::default(),denied,(0,0));
 let demand=native_output_demand::<String>(0).unwrap();let grant=bank.loan(1,demand.copy_bytes);let(poll,marker)=bank.run(grant,1,|effect|native_output_take(&mut source,&mut output,0,effect));let RetainedWorkPoll::Complete(turn)=poll else{panic!("funded publication remained pending")};assert_eq!(turn.work,u128::from(law["publishedWork"].as_u64().unwrap()));let RetainedCloneStep::Complete(progress)=turn.result.unwrap() else{panic!("original output was not published")};assert_eq!(law["published"],"complete");assert_eq!(marker,None);assert!(source.is_none());assert_eq!(output.as_ref().unwrap().as_ptr(),original);assert_eq!(bank.take_admitted(),Some(true));bank.post(progress,grant,(0,0));
}
