//! 🥽️ Public mesh document and retained modeling API.

pub use semio_framework_3d_mesh_document::*;
use semio_framework_3d_mesh_clipping as clip;
use semio_framework_3d_mesh_reconstruction as recon;

pub use recon::PolygonCombine;
pub type MeshAuthoredIntersection=recon::MeshAuthoredIntersection<clip::MeshConvexIntersection>;
pub type MeshAuthoredDifference=recon::MeshAuthoredIntersection<clip::MeshConvexDifference>;
pub type MeshAuthoredUnion=recon::MeshAuthoredIntersection<clip::MeshConvexUnion>;
#[path = "🛠️modeling/🦀️.rs"]
mod modeling;
pub use modeling::{MeshModelingOps,MeshModelingJob,MeshTranslationSource,MeshTranslationInput,MeshPlaneProduction,MeshTranslationProduction};
#[cfg(feature="mesh-test-observation")]
pub use modeling::inspection::*;
