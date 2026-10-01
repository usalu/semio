//! 🔌️ Container capabilities linked by the remodeling artifact owner.
use super::VideoContainerProviderV1;

/// 🎥️ Present ISO-BMFF capability.
#[cfg(feature = "video-mp4")]
#[path = "🎥️mp4/🦀️.rs"]
pub mod mp4;

/// 📼️ Present AVI capability.
#[cfg(feature = "video-avi")]
#[path = "📼️avi/🦀️.rs"]
pub mod avi;

/// 🔌️ Supplies only capabilities selected in the owner's prepared manifest.
pub fn inventory_v1() -> Vec<VideoContainerProviderV1<'static>> {
    vec![
        #[cfg(feature = "video-mp4")]
        mp4::provider_v1(),
        #[cfg(feature = "video-avi")]
        avi::provider_v1(),
    ]
}
