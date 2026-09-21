//! 🌐️ The locale × terminology axes and the label carriers built on them.
//!
//! 🏛️ Home moved here from `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu` (ticket
//! 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice U3): `MutationKind::label` returns a
//! [`LocalizedLabel`], and the mutation trait lives in this crate, which the UI crate depends on —
//! the axes had to sit at or below the kernel for that to be expressible. `semio-framework-ui`'s
//! wgpu target re-exports every name below, so `ui_wgpu::wgpu::{Locale, Terminology, Label,
//! LabelText, LocalizedLabel, AppLabels}` still resolves for every `app_labels!` call site.

#[path = "🤖️generated/🦀️.rs"]
mod ui_axes_gen;

pub use ui_axes_gen::{Locale, Terminology};

#[path = "🧾️value/🦀️.rs"]
mod locale_terminology_value;

#[path = "🏷️label/🦀️.rs"]
mod label_impl;

pub use label_impl::{AppLabels, Label, LabelText, LocalizedLabel};
