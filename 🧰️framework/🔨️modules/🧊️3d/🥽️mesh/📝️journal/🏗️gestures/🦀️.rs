//! 🏗️ The mesh gestures as row emitters over a [`MeshOverlay`](super::MeshOverlay). Each gesture READS the overlay as a polygon soup
//! (positions and wound face loops, taken before it writes anything), decides which faces it replaces, removes or appends and which
//! vertices it moves or appends — the algorithm names those edits itself — and states them through the face-level editor. The
//! journal is the program-order record of exactly those writes; no assembled mesh is built and nothing is compared with the base.
//!
//! 🧱️ A gesture that renumbers corners refuses a mesh carrying authored channels or uv seams (they address corners and need a declared
//! source remap), exactly as the kernel's reconstruction refuses them without one.

use semio_framework_3d_mesh_document::{triangulate_polygon, EdgeId, FaceId, HalfedgeMesh, MeshKernelError, MeshResult, MirrorAxis, Vec3, VertexId, WeldMode};
use super::editor::{loop_normal, Normal, Soup};
use super::{MeshOverlay, Orphans};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[path = "⬆️inflate/🦀️.rs"]
mod inflate;
#[path = "🔳️subdivide/🦀️.rs"]
mod subdivide;
#[path = "🧮️in-place/🦀️.rs"]
mod in_place;
#[path = "🧲️weld/🦀️.rs"]
mod weld;
#[path = "🪚️bevel/🦀️.rs"]
mod bevel;
#[path = "🪶️decimate/🦀️.rs"]
mod decimate;

/// 🔷️ The selected faces in input order: every id must name a face of the soup and appear once.
fn select_faces(soup: &Soup, faces: &[FaceId], duplicate: &str) -> MeshResult<Vec<u32>> {
    if faces.is_empty() {
        return Err(MeshKernelError::EmptySelection);
    }
    let mut seen = BTreeSet::new();
    let mut selected = Vec::with_capacity(faces.len());
    for face in faces {
        if soup.faces.get(face.0 as usize).is_none_or(|corners| corners.is_empty()) {
            return Err(MeshKernelError::InvalidHandle);
        }
        if !seen.insert(face.0) {
            return Err(MeshKernelError::InvalidInput(duplicate.into()));
        }
        selected.push(face.0);
    }
    Ok(selected)
}

impl MeshOverlay<'_> {
    /// 🔁️ Replaces `face` by the first of `pieces` and appends the others, all sharing `face`'s shading.
    fn replace_with_pieces(&mut self, face: u32, pieces: Vec<Vec<u32>>, smooth: bool) -> MeshResult<()> {
        let mut pieces = pieces.into_iter();
        if let Some(first) = pieces.next() {
            self.replace_face(face, &first)?;
        }
        for piece in pieces {
            self.add_face(&piece, smooth)?;
        }
        Ok(())
    }
}
