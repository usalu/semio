//! 🧱️ Vector-tile facade: the neutral MVT decoder plus the GIS vector style, both re-exported under one public path.
//!
//! Public API is unchanged: `vector_tiles::decode_mvt`, `vector_tiles::layer_draw_rank` and every other former inline item resolve here.

use super::FrameworkSurfaceTiledMapError;

// #region 🔖️VectorTiles
#[path = "📦️mvt-decoder/🦀️.rs"]
mod mvt_decoder;
pub use mvt_decoder::*;
pub use super::gis_policy::vector_style::*;
// #endregion 🔖️VectorTiles
