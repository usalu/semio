#[test]
fn demo_metadata_is_language_neutral(){
 assert_eq!(super::ID,"demo");
 assert_eq!(super::ICON,"file");
}

#[test]
fn inference_default_law(){
 use crate::standards::v1::subsets::any::schema::inferences::JackInference;
 use protocol::Inference;
 let projection=<crate::test_support::FixtureComposition as crate::JackComposition>::initial_snapshot();
 assert_eq!(JackInference::infer(&projection).expect("retained fixture"),JackInference::default());
 crate::retire_owned_to_terminal(projection).expect("fixture close");
}

#[test]
fn inference_determinism_law(){
 use crate::standards::v1::subsets::any::schema::inferences::JackInference;
 use protocol::Inference;
 let projection=<crate::test_support::FixtureComposition as crate::JackComposition>::initial_snapshot();
 assert_eq!(JackInference::infer(&projection).expect("retained fixture"),JackInference::infer(&projection).expect("retained fixture"));
 crate::retire_owned_to_terminal(projection).expect("fixture close");
}
