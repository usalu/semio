//! 🧱️ Vector-tile facade: the neutral MVT decoder crate under the `vector_tiles` path.
//!
//! `vector_tiles::decode_mvt`, the property helpers and the tile-edge geometry helpers resolve here; vendor cartography (paint roles, class tables, label rules) lives in the host's `MapPolicy`.

// #region 🔖️VectorTiles
pub use tiled_map_mvt::{
    decode_mvt, feature_label, feature_label_anchor, mvt_point_on_tile_bbox_edge, mvt_polyline_is_tile_bbox_artifact, mvt_ring_is_tile_bbox_cover, mvt_segment_is_tile_seam, mvt_segment_touches_tile_bbox, property_flag, property_str, property_u64,
    GeomType, MvtDecodeError, VectorFeature, VectorLayer, VectorTile, DEFAULT_MVT_EXTENT,
};
// #endregion 🔖️VectorTiles
