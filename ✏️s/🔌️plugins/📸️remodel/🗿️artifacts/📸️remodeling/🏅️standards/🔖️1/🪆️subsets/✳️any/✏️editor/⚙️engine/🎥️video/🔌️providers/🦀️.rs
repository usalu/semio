//! 🔌️ Container capabilities linked by the remodeling artifact owner.
use super::VideoContainerProviderV1;

/// 🎥️ Present ISO-BMFF capability.
#[path = "🎥️mp4/🦀️.rs"]
pub mod mp4;

/// 📼️ Present AVI capability.
#[path = "📼️avi/🦀️.rs"]
pub mod avi;

/// 🔌️ Supplies only capabilities selected in the owner's prepared manifest.
pub fn inventory_v1() -> Vec<VideoContainerProviderV1<'static>> {
    vec![
        mp4::provider_v1(),
        avi::provider_v1(),
    ]
}
