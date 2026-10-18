//! 🏠️ Artifact document-store and publication authorities.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::{RasterAssetChild, RasterLayerNode, RasterOwnedMap, RasterOwnedMapInsert};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use protocol::{Mutation, OpBinary};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

#[path = "🔓️owned-field-decode/🦀️.rs"]
mod owned_field_decode;
pub use owned_field_decode::*;


#[path = "🚀️store-initialization/🦀️.rs"]
mod store_initialization;
pub use store_initialization::*;

#[path = "🧬️snapshot-clone/🦀️.rs"]
mod snapshot_clone;
use snapshot_clone::*;


#[path = "🧩️mutation-candidate/🦀️.rs"]
mod mutation_candidate;
pub use mutation_candidate::*;





#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

pub use semio_s_artifact_raster_raster_host_retirement_bounds::retirement::*;
use semio_s_artifact_raster_raster_host_retirement_bounds::bounds_census::*;
use semio_s_artifact_raster_raster_host_retirement_bounds::{RASTER_MAXIMUM_NESTED_BYTES, RASTER_MAXIMUM_NESTED_DEPTH, RASTER_MAXIMUM_NESTED_ITEMS, RASTER_COMBINED_DEPTH_CAPACITY};
