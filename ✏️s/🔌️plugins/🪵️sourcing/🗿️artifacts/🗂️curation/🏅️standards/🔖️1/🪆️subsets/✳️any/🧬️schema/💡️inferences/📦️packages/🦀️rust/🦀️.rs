//! 💡️ inferences of `semio-s-artifact-sourcing-curation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::CurationSnapshot;
pub use crate::schema::mutations::SourcingMutation;
pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }

        }
pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
pub use semio_s_artifact_sourcing_curation_examples::{CuratedItem, SortDirection, TableSort};
pub use semio_s_artifact_sourcing_curation_domain::{ObjectKind, SOURCING_CURATION_SCHEMA, curation_child_restore_projection, curation_snapshot_from_stock, genesis_catalog_pack, stock_of};
pub use crate::viewer::sourcing::artifact_items_2::{SOURCING_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
