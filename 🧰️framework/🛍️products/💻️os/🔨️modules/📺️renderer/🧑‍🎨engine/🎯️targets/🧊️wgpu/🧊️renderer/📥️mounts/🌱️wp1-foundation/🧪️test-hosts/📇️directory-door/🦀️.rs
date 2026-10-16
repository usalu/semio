//! 📇️ Test host of the renderer hub. The directory door law reads the shell source text, so it cannot live in `doors`.
//! The host re-exports the module under test, so the law keeps its `use super::*;` and its module path while the law's file stays with its element.

pub(crate) use crate::directory_door::*;

#[path = "../../../../../../../🧪️tests/📇️wgpu-directory-door/🦀️.rs"]
mod tests;
