//! 🧫️ Child identity receiving retains the actual original source through every interrupted frontier.
use super::*;
use semio_framework_value::DslValue;

#[path="../../../../../../../🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;

fn input(law:&serde_json::Value)->DslValue {
    let dialect=&law["target"]["dialect"];
    DslValue::Object(vec![("childId".into(),DslValue::String(law["childId"].as_str().unwrap().into())),("target".into(),DslValue::Object(vec![("artifactId".into(),DslValue::String(law["target"]["artifactId"].as_str().unwrap().into())),("dialect".into(),DslValue::Object(["artifactKind","standard","subset"].map(|key|(key.into(),DslValue::String(dialect[key].as_str().unwrap().into()))).into()))]))])
}
#[test]
fn original_child_receive_same_caller_all_partial_prefixes_and_literal_target() {
    let laws:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
    for law in laws["cases"].as_array().unwrap() {
        let mut completed=false;
        for cut in (0..2048u64).map(Some).chain(std::iter::once(None)) {
            if completed&&cut.is_some() { continue }
            let mut bank=driver::bank();
            let source=driver::admit(&mut bank,input(law));
            let mut inspected=false;
            let received=driver::lifecycle(<ArtifactChild<()> as NativeReceive<DslValue>>::receiver(),source,&mut bank,4096,cut,|output:&ArtifactChild<()>|{
                inspected=true;
                assert_eq!(output.child_id,law["childId"].as_str().unwrap());
                assert_eq!(output.target.artifact_id,law["target"]["artifactId"].as_str().unwrap());
                assert_eq!(output.target.dialect.subset,law["target"]["dialect"]["subset"].as_str().unwrap());
                assert!(output.local_owner.is_none());
            });
            assert_eq!(received,Ok(inspected));
            if cut.is_none() { assert!(inspected,"an uncut receive publishes its child"); completed=true; } else if inspected { completed=true; }
        }
        assert!(completed);
    }
}
