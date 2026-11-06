//! 🎟️ Raster's native declaration installs its actual presence, image member family and Composite transient owner.
use crate::RasterPlayApp;
use semio_plugin::{ArtifactEditor,EditorApp,WindowConfigOwnerRegistry,WindowTransientOwnerRegistry,OriginalWindowTransientOwnerRegistration};
use semio_plugin::original_instance_registration::{OriginalNativeRegistration,OriginalInstanceDeclarations,OriginalWindowSequence,OriginalNoWindows};
use crate::modes::edit::windows::composite::transient::RasterCompositeWindowTransientOwner;
/// 🧩️ Camera and viewport persist in RasterConfig; Composite gesture state belongs only to its concrete window.
pub struct RasterInstanceDeclarations;
impl OriginalInstanceDeclarations<EditorApp<RasterPlayApp>> for RasterInstanceDeclarations{
 type Ephemeral=crate::app::original_ephemeral::RasterEphemeralDeclaration;
 type ConfigWindows=OriginalNoWindows<WindowConfigOwnerRegistry>;
 type TransientWindows=OriginalWindowSequence<OriginalWindowTransientOwnerRegistration<RasterCompositeWindowTransientOwner>,OriginalNoWindows<WindowTransientOwnerRegistry>>;
}
/// 🌱️ The native builder selects this original registrar with its unchanged mounted owner policy.
pub type RasterNativeRegistration=OriginalNativeRegistration<EditorApp<RasterPlayApp>,<RasterPlayApp as ArtifactEditor>::Members,RasterInstanceDeclarations>;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
