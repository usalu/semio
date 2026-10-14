//! 🚪️ IO s.draw (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<DrawingSnapshot>`/`Deserializer<DrawingSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`,
//! deleted outright) — all io now goes exclusively through the `io_mechanism` registry (design.md
//! rule 3). `import_stdio_kinds`/`export_stdio_kinds` (old, zero callers even before this pass)
//! deleted alongside them.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

//#region 🔖️SemioBridge

use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::FillStyle;
/// 🌉️ Relocated verbatim from the `⚙️engine` directory (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 5: sniff/codec dispatch and
/// cross-format bridge functions live in `🚪️io/`).
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas as SemioDrawCanvas, DrawLayer as SemioDrawLayer, DrawNode as SemioDrawNode, DrawStyle as SemioDrawStyle, PathSegment as SemioPathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};


//#endregion 🔖️SemioBridge

//#region 🔖️IoDeclaration
//#endregion 🔖️IoDeclaration

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path="🖼️image/🦀️.rs"]
pub mod image;

#[path="🧵️write/🦀️.rs"]
pub mod byte_writer;

