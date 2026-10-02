use super::*;

fn admission(seed: &str) -> semio_framework_plugin::AppOperationContext {
    semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "remodel".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: seed.into() }
}

/// 🆔️ A minted id is a pure function of the admission's seed and the prefix: two admissions mint two ids, a replayed
/// admission mints the same one, and the id is the specified `content_id` (SHA-256 over `seed U+001F prefix`).
#[semio_framework_async_macros::async_test]
async fn a_minted_id_is_content_addressed_over_the_admission_seed() {
    let (first, second) = (admission("seed-a"), admission("seed-b"));
    let id = mint_remodeling_id(Some(&first), "stream");
    assert!(id.starts_with("stream-") && id.len() == "stream-".len() + 16, "{id}");
    assert_eq!(mint_remodeling_id(Some(&first), "stream"), id);
    assert_ne!(mint_remodeling_id(Some(&second), "stream"), id);
    assert_ne!(mint_remodeling_id(Some(&first), "gcp").trim_start_matches("gcp-"), id.trim_start_matches("stream-"));
    assert_eq!(id, store::content_id("stream", "seed-a\u{1f}stream".as_bytes()));
}


#[semio_framework_async_macros::async_test]
async fn video_codec_from_label_recognizes_common_aliases() {
    assert_eq!(video_codec_from_label("h264"), VideoCodec::Avc);
    assert_eq!(video_codec_from_label("h.265"), VideoCodec::Hevc);
    assert_eq!(video_codec_from_label("mjpg"), VideoCodec::Mjpeg);
    assert_eq!(video_codec_from_label("weird"), VideoCodec::Unknown);
}
