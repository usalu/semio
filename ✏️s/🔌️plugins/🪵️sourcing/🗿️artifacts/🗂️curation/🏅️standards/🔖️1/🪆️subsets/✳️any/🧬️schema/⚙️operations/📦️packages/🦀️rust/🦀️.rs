//! ⚙️ operations of `semio-s-artifact-sourcing-curation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::CurationSnapshot;
pub use crate::schema::diff::CurationDiff;
pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }

        }
pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }

        }
pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
pub use semio_s_artifact_sourcing_curation_examples::{CuratedItem, Filters, SortDirection, TableSort};
pub use semio_s_artifact_sourcing_curation_domain::{GeometryRecipe, ObjectKind, ObjectKindExtra, SOURCING_CURATION_SCHEMA, curation_snapshot_from_stock, stock_of};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
