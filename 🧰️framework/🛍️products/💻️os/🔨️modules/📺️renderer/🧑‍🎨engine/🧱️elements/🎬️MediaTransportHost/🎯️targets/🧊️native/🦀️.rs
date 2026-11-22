//! 🎬️ `🎬️MediaTransportHost` for hosts with no media element: the native decoders and transport. The React twin is the browser overlay
//! (`🎬️media/🌐️browser/🎛️host/🟦️.ts`), which hands the bytes to an `HTMLMediaElement`; this host decodes WAV audio and the MP4 the
//! framework's own AVC encoder writes in-repo, drives the same transport rules, and refuses everything else with a typed reason.
//! [`NATIVE_MEDIA_BROWSER_ONLY`] lists exactly what stays browser-only. Playback reaches a window through [`presenter::MediaSlotPresenter`] and the
//! operating system's speakers through [`audio_output::AudioOutputPort`].

#[path = "🔊️wav/🦀️.rs"]
pub mod wav;
#[path = "⏯️transport/🦀️.rs"]
pub mod transport;
#[path = "🎞️player/🦀️.rs"]
pub mod player;
#[path = "🔈️audio-output/🦀️.rs"]
pub mod audio_output;
#[path = "🖼️presenter/🦀️.rs"]
pub mod presenter;

pub use audio_output::*;
pub use player::*;
pub use presenter::*;
pub use transport::*;
pub use wav::*;
