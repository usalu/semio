#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_default_law() {
    use crate::artifacts::rewrite::standards::v1::subsets::any::schema::inferences::RewriteInference;
    use protocol::Inference;
    assert_eq!(
        RewriteInference::infer(&crate::artifacts::rewrite::RewriteSnapshot::default()),
        RewriteInference::default()
    );
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::rewrite::standards::v1::subsets::any::schema::inferences::RewriteInference;
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let projection = crate::artifacts::rewrite::dsl::parse_dsl(text).expect("example dsl parses");
    assert_eq!(RewriteInference::infer(&projection), RewriteInference::infer(&projection));
}
