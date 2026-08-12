//! 🗂️ Raster play app commands — document-scoped commands. The pre-migration whole-document
//! setters (`setSnapshot`, `setActiveExample`) are gone: whole-document replace is not expressible
//! as an in-history `RasterMutation` anymore (the retired whole-document-replace variant has no
//! replacement — see `🧬️mutations/🦀️component.rs`'s module docstring). File-open/import/load-example
//! now go through `store::ArtifactStore::reset` (non-history, clears undo/redo) via the app manifest's own
//! `.example(...)` registration (`create_raster_app` in `🎛️apps/🖨️raster/🦀️component.rs`), entirely
//! outside `RasterCommand`/`RasterMutation` dispatch — this module has no commands left to declare.
