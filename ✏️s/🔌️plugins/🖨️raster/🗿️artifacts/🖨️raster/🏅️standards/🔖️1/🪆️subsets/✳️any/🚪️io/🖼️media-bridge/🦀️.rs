//! 🧱️ media-bridge of `semio-s-artifact-raster-raster-io`: the mutually coupled modules `semio_bridge`, `composite_and_media`, `testing`.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_schema as schema;
use semio_s_artifact_raster_raster_model::RasterAssetChild;
use semio_s_artifact_raster_raster_model::image_asset_child_handle;
use semio_s_artifact_raster_raster_model::image_content_child_handle;
use semio_s_artifact_raster_raster_model::RasterOwnedMap;
use semio_s_artifact_raster_raster_model::{RasterImageAsset, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_stdio_png::PngSnapshot;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::geometry::{compose_affine, flatten_segments, semio_transform_affine, transformed_segments};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_svg::SvgSnapshot;
use semio_framework_pixels::compositing::layers::{RasterStackContent, RasterStackInput, RasterStackJob, RasterStackLayer, RasterStackMask, RasterStackTransform};
use semio_framework_pixels::{editing::validate_extent, RasterImage};
use std::{collections::BTreeMap, sync::Arc};
use {semio_framework_os_kernel::io::io_compose_via, semio_framework_os_kernel::io::io_dispatch, semio_framework_artifact_reference::Dialect, semio_framework_os_kernel::io::ErasedComposeSource, semio_framework_os_kernel::io::IoDirection, semio_framework_os_kernel::io::IoKey, semio_framework_os_kernel::io::IoPayload, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};

/// 🌉️ The single funnel-through "add real content" primitive: converts the real bytes into the
/// composed child's own real content (`SemioImageSnapshot`, via the real `🚪️io` png↔semio/image
/// bridge — never a stub), mints the CANONICAL content-addressed handle off that decoded content
/// (`image_content_child_handle`), and attaches immutable content to that exact child owner. An
/// unsupported mime or undecodable payload falls back to a raw-bytes handle without fabricating a
/// materialization. Every call site receives one self-contained child owner; no process-global cache
/// can leak content between snapshots or retain abandoned payloads.
pub fn mint_raster_asset_child(asset_id: &str, asset: &RasterImageAsset) -> RasterAssetChild {
    match crate::semio_image_snapshot_from_raster_asset(asset) {
        Ok(image) => image_content_child_handle(asset_id, &image).with_local_owner(std::sync::Arc::new(image)),
        Err(_) => image_asset_child_handle(asset_id, asset),
    }
}

#[path = "../🌉️semio-bridge/🦀️.rs"]
mod semio_bridge;
pub use semio_bridge::*;
#[path = "../🧩️composite-and-media/🦀️.rs"]
mod composite_and_media;
pub use composite_and_media::*;
#[cfg(any(test, feature = "testkit"))]
#[path = "../🧪️testing/🦀️.rs"]
pub mod testing;
