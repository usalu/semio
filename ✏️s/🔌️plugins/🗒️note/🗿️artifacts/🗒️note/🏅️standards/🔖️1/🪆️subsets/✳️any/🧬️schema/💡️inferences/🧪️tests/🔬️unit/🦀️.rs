use crate::NoteSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::any::schema::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = NoteSnapshot::default();
    assert_eq!(NoteInference::infer(&snapshot).expect("valid materialized inference fixture"), NoteInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(NoteInference::infer(&NoteSnapshot::default()).expect("valid materialized inference fixture"), NoteInference::default());
}
