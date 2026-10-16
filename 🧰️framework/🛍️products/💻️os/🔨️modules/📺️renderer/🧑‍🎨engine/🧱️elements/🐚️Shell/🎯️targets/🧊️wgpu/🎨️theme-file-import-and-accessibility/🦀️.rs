//! 🎨️ Media export, file import, theme documents and chrome accessibility.
//!
//! Media export, file open/import, host IO tracing, media frame requests, the theme document model and editor,
//! theme doors, merge policy and the chrome accessible names and nodes.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region 📤️FileOpenImport

//#endregion 📤️FileOpenImport

//#region RequestMediaFrames

//#endregion RequestMediaFrames






#[cfg(test)]
#[path = "../../../🧪️tests/🔀️wgpu-document-relay/🦀️.rs"]
mod document_relay_tests;

//#region 🎨️ThemeDocumentModel

//#endregion 🎨️ThemeDocumentModel


//#region 🎨️ThemeDocumentDoors

//#endregion 🎨️ThemeDocumentDoors


//#region ♿️ChromeAccessibleNames

//#endregion ♿️ChromeAccessibleNames
#[cfg(test)]
#[path = "../../../🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs"]
mod theme_editor_and_accessibility_tests;



#[cfg(test)]
#[path = "../../../🧪️tests/🎯️presented-input-authority/🦀️.rs"]
mod presented_input_authority_tests;
