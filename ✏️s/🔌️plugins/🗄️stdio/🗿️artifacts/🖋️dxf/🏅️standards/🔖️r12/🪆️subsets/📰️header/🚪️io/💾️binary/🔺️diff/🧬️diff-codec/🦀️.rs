//! 🧬️ Binary codec of the DXF diff: the varint-framed primitives, the item pair codecs, the collection triple codecs and the binary diff implementation of the root diff, with the shared imports of the former inline module. See [`super`].

use super::*;
use crate::standards::v_r12::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSolid, DxfText, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

#[path = "📦️primitives-and-values/🦀️.rs"]
mod primitives_and_values;
pub use primitives_and_values::*;

#[path = "📐️group-codes-and-entities/🦀️.rs"]
mod group_codes_and_entities;
pub use group_codes_and_entities::*;

#[path = "🗂️tables-and-blocks/🦀️.rs"]
mod tables_and_blocks;
pub use tables_and_blocks::*;

#[path = "🏷️item-diffs/🦀️.rs"]
mod item_diffs;
pub use item_diffs::*;

#[path = "🧵️triples-and-collection-diffs/🦀️.rs"]
mod triples_and_collection_diffs;
pub use triples_and_collection_diffs::*;

#[path = "🚦️diff-binary-impl/🦀️.rs"]
mod diff_binary_impl;
