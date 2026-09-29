#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🔁️ The committed example is this codec's own output — byte for byte, its document query included — the fixpoint
/// `🔌️mutate-jack-1`'s round trip and the query editor's first frame both read.
#[semio_framework_async_macros::async_test]
async fn primary_asset_is_the_codec_s_own_output() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let parsed = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(parsed.query, crate::TRINITY_JACK_DEFAULT_QUERY, "the example opens on the default query");
    assert_eq!(crate::standards::v1::subsets::any::schema::snapshot::text::print_dsl(&parsed), text, "the committed example is the codec's own output");
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    assert_eq!(JackInference::infer(&crate::JackSnapshot::default()), JackInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::standards::v1::subsets::any::schema::inferences::JackInference;
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let projection = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(JackInference::infer(&projection), JackInference::infer(&projection));
}
