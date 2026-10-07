//! 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCoedge, BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex, SemioBrepSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<BrepEdge>,
    #[state(artifact)]
    #[value(default)]
    pub loops: Vec<BrepLoop>,
    #[state(artifact)]
    #[value(default)]
    pub faces: Vec<BrepFace>,
    #[state(artifact)]
    #[value(default)]
    pub shells: Vec<BrepShell>,
    #[state(artifact)]
    #[value(default)]
    pub solids: Vec<BrepSolid>,
    #[state(artifact)]
    #[value(default)]
    pub coedges: Vec<BrepCoedge>,
    #[state(artifact)]
    #[value(default)]
    pub next_label: u64,
}

impl Default for SemioBrepArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioBrepSnapshot::default())
    }
}

impl SemioBrepArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioBrepSnapshot {
        SemioBrepSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            edges: self.edges.clone(),
            loops: self.loops.clone(),
            faces: self.faces.clone(),
            shells: self.shells.clone(),
            solids: self.solids.clone(),
            coedges: self.coedges.clone(),
            next_label: self.next_label,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioBrepSnapshot) -> Self {
        Self { schema: snapshot.schema, vertices: snapshot.vertices, edges: snapshot.edges, loops: snapshot.loops, faces: snapshot.faces, shells: snapshot.shells, solids: snapshot.solids, coedges: snapshot.coedges, next_label: snapshot.next_label }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioBrepSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_brep_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.brep",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

use std::collections::HashSet;

/// 🔗️ Real cross-collection referential-invariant check — dangling ids are reported as errors, not
/// silently ignored (nothing here is decode-only anymore).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn check_brep_referential_integrity(snapshot: &SemioBrepSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
    let vertex_ids: HashSet<&str> = snapshot.vertices.iter().map(|v| v.id.as_str()).collect();
    let edge_ids: HashSet<&str> = snapshot.edges.iter().map(|e| e.id.as_str()).collect();
    let loop_ids: HashSet<&str> = snapshot.loops.iter().map(|l| l.id.as_str()).collect();
    let face_ids: HashSet<&str> = snapshot.faces.iter().map(|f| f.id.as_str()).collect();
    let shell_ids: HashSet<&str> = snapshot.shells.iter().map(|s| s.id.as_str()).collect();

    let mut diagnostics = Vec::new();
    let mut dangling = |code: &'static str, message: String| {
        diagnostics.push(semio_framework_diagnostic::Diagnostic::error(code, semio_framework_diagnostic::TextSpan::at(1, 1), message));
    };

    for e in &snapshot.edges {
        if !vertex_ids.contains(e.start_vertex.as_str()) {
            dangling("stdio.semio_brep.dangling-edge-start-vertex", format!("edge {:?} references unknown start vertex {:?}", e.id, e.start_vertex));
        }
        if !vertex_ids.contains(e.end_vertex.as_str()) {
            dangling("stdio.semio_brep.dangling-edge-end-vertex", format!("edge {:?} references unknown end vertex {:?}", e.id, e.end_vertex));
        }
    }
    for l in &snapshot.loops {
        for le in &l.edges {
            if !edge_ids.contains(le.edge.as_str()) {
                dangling("stdio.semio_brep.dangling-loop-edge", format!("loop {:?} references unknown edge {:?}", l.id, le.edge));
            }
        }
    }
    for f in &snapshot.faces {
        if !loop_ids.contains(f.outer_loop.as_str()) {
            dangling("stdio.semio_brep.dangling-face-outer-loop", format!("face {:?} references unknown outer loop {:?}", f.id, f.outer_loop));
        }
        for inner in &f.inner_loops {
            if !loop_ids.contains(inner.as_str()) {
                dangling("stdio.semio_brep.dangling-face-inner-loop", format!("face {:?} references unknown inner loop {:?}", f.id, inner));
            }
        }
    }
    for s in &snapshot.shells {
        for sf in &s.faces {
            if !face_ids.contains(sf.face.as_str()) {
                dangling("stdio.semio_brep.dangling-shell-face", format!("shell {:?} references unknown face {:?}", s.id, sf.face));
            }
        }
    }
    for so in &snapshot.solids {
        for ss in &so.shells {
            if !shell_ids.contains(ss.shell.as_str()) {
                dangling("stdio.semio_brep.dangling-solid-shell", format!("solid {:?} references unknown shell {:?}", so.id, ss.shell));
            }
        }
    }
    diagnostics
}

