//! 🧾️ domain of `semio-s-artifact-sourcing-curation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::CurationSnapshot;
pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
pub use semio_s_artifact_sourcing_curation_examples::{CuratedItem, Filters, SortDirection, TableSort};
pub use crate::editor::sourcing::presence::artifact_items::{SOURCING_CURATION_SCHEMA};
pub use crate::editor::sourcing::presence::artifact_items_3::{GeometryRecipe};
pub use crate::editor::sourcing::presence::artifact_items_4::{ObjectKind};
pub use crate::editor::sourcing::presence::artifact_items_6::{CATALOG_CHILD_SLOT, ObjectKindExtra, catalog_child_handle, catalog_snapshot_from_stock, curation_child_restore_projection, curation_snapshot_from_stock, genesis_catalog_pack, kit_type_from_object_kind, object_kind_extra_from_object_kind, object_kind_from_parts, stock_extra_from_stock, stock_from_catalog_and_extra, stock_of, validate_catalog_payload};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
