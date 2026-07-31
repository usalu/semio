//! 💾️ Versioned, human-editable serialization schemas. Compiled runtime state (`CompiledModel`'s
//! bitset tables, a live `Checkpoint`) is never trusted directly from an external source:
//! [`SourceModelDoc`] always recompiles through [`crate::model::ModelBuilder`] (the exact same
//! validation path a freshly authored model goes through) and [`CheckpointDoc`] always
//! structurally revalidates against a live model/topology before becoming a usable
//! [`crate::trail::Checkpoint`]. This is one of the few places outside `🦀️ids.rs` this crate derives
//! `serde::Serialize`/`Deserialize` directly on a public type — deliberately, since these types'
//! entire purpose is to cross a serialization boundary; JSON convenience is just `serde_json`
//! applied directly to them (see this module's tests), no wrapper needed.

use crate::bitset::PatternSet;
use crate::error::{ModelError, SolveError};
use crate::ids::{PatternId, RelationId};
use crate::model::{CompiledModel, ModelBuilder};
use crate::trail::Checkpoint;
use serde::{Deserialize, Serialize};

// #region 🔖️SourceModel
/// 💾️ Current [`SourceModelDoc`] schema version. Bump on any breaking field change; old versions
/// are rejected outright by [`SourceModelDoc::compile`], never migrated.
pub const SOURCE_MODEL_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatternDoc {
    pub weight: f64,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationDoc {
    pub name: String,
    /// 💾️ Index into the document's own `relations` list this relation is the inverse of;
    /// `None` means self-inverse.
    #[serde(default)]
    pub inverse: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairDoc {
    pub relation: u32,
    pub src: u32,
    pub dst: u32,
}

/// 💾️ A versioned, human-editable model schema — the input shape [`crate::model::ModelBuilder`]
/// consumes, not [`CompiledModel`]'s compiled bitset tables. Deliberately does not capture
/// [`crate::tiled::TiledModelBuilder`]'s higher-level socket/symmetry authoring (deferred; a tiled
/// model already compiles down to this exact pattern/relation/allow shape, so round-tripping
/// through here is compile-equivalent, just not re-editable at the socket level). `deny` pairs are
/// not reconstructed from a compiled model either — by compile time `deny` has already been folded
/// into `allow`'s absence, so [`SourceModelDoc::from_model`] only ever emits `allow`; a hand-authored
/// document may still use both.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceModelDoc {
    pub version: u32,
    pub patterns: Vec<PatternDoc>,
    pub relations: Vec<RelationDoc>,
    #[serde(default)]
    pub allow: Vec<PairDoc>,
    #[serde(default)]
    pub deny: Vec<PairDoc>,
}

impl SourceModelDoc {
    /// 💾️ Captures `model`'s pattern/relation/tag/allow shape as a serializable document.
    pub fn from_model(model: &CompiledModel) -> Self {
        let patterns = (0..model.pattern_count())
            .map(|i| {
                let info = model.pattern_info(PatternId::from_index(i));
                let tags = info.tags.iter().filter_map(|&t| model.tag_name(t)).map(str::to_string).collect();
                PatternDoc { weight: info.weight, tags }
            })
            .collect();

        let relations = (0..model.relation_count())
            .map(|i| {
                let info = model.relation_info(RelationId::from_index(i));
                let inv = info.inverse.index();
                RelationDoc { name: info.name.clone(), inverse: if inv == i { None } else { Some(inv as u32) } }
            })
            .collect();

        let mut allow = Vec::new();
        for ri in 0..model.relation_count() {
            let r = RelationId::from_index(ri);
            for src in 0..model.pattern_count() {
                let src_id = PatternId::from_index(src);
                for dst in model.allowed(r, src_id).iter_ones() {
                    allow.push(PairDoc { relation: ri as u32, src: src as u32, dst: dst.get() });
                }
            }
        }

        Self { version: SOURCE_MODEL_VERSION, patterns, relations, allow, deny: Vec::new() }
    }

    /// 💾️ Recompiles into a validated [`CompiledModel`] via the same `ModelBuilder::compile` +
    /// `validate()` path any hand-written builder code goes through — an untrusted document never
    /// takes a shortcut around inverse-consistency checking.
    pub fn compile(&self) -> Result<CompiledModel, ModelError> {
        if self.version != SOURCE_MODEL_VERSION {
            return Err(ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: self.version });
        }
        let mut b = ModelBuilder::new();
        for p in &self.patterns {
            let id = b.add_pattern(p.weight);
            for tag in &p.tags {
                b.add_tag(id, tag);
            }
        }
        for r in &self.relations {
            b.add_relation(&r.name);
        }
        for (i, r) in self.relations.iter().enumerate() {
            if let Some(inv) = r.inverse {
                b.set_relation_inverse(RelationId::from_index(i), RelationId::from_index(inv as usize));
            }
        }
        for pair in &self.allow {
            b.allow(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        for pair in &self.deny {
            b.deny(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        let compiled = b.compile()?;
        compiled.validate()?;
        Ok(compiled)
    }
}
// #endregion 🔖️SourceModel

// #region 🔖️Checkpoint
/// 💾️ Current [`CheckpointDoc`] schema version.
pub const CHECKPOINT_VERSION: u32 = 1;

/// 💾️ A versioned, serializable [`Checkpoint`]. Structurally revalidated against a live model and
/// node count on load ([`CheckpointDoc::into_checkpoint`]) — bitset lengths, per-domain word-count/
/// padding-bit well-formedness, domain count, and model fingerprint are all checked, so a
/// hand-tampered file fails with [`SolveError`] rather than panicking or silently corrupting a
/// resumed solve.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointDoc {
    pub version: u32,
    pub domains: Vec<PatternSet>,
    pub model_fingerprint: u64,
    pub seed: u64,
}

impl CheckpointDoc {
    pub fn from_checkpoint(checkpoint: &Checkpoint) -> Self {
        Self { version: CHECKPOINT_VERSION, domains: checkpoint.domains.clone(), model_fingerprint: checkpoint.model_fingerprint, seed: checkpoint.seed }
    }

    /// 💾️ Revalidates every structural invariant a deserialized checkpoint might violate, then
    /// converts into a usable [`Checkpoint`]. `node_count` and `model` should come from the live
    /// topology/model this checkpoint is about to resume against.
    pub fn into_checkpoint(self, model: &CompiledModel, node_count: usize) -> Result<Checkpoint, SolveError> {
        if self.version != CHECKPOINT_VERSION {
            return Err(SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: self.version });
        }
        if self.model_fingerprint != model.fingerprint() {
            return Err(SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
        }
        if self.domains.len() != node_count {
            return Err(SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
        }
        for d in &self.domains {
            if !d.is_well_formed() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
            }
            if d.len() != model.pattern_count() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
            }
        }
        Ok(Checkpoint::new(self.domains, self.model_fingerprint, self.seed))
    }
}
// #endregion 🔖️Checkpoint

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard() -> (CompiledModel, crate::topology::GraphTopology) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(2.0);
        b.add_tag(black, "dark");
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(3);
        tb.arc(crate::ids::NodeId(0), crate::ids::NodeId(1), adj);
        tb.arc(crate::ids::NodeId(1), crate::ids::NodeId(0), adj);
        tb.arc(crate::ids::NodeId(1), crate::ids::NodeId(2), adj);
        tb.arc(crate::ids::NodeId(2), crate::ids::NodeId(1), adj);
        (model, tb.build().unwrap())
    }

    #[test]
    fn from_model_compile_round_trip_preserves_fingerprint() {
        let (model, _topo) = checkerboard();
        let doc = SourceModelDoc::from_model(&model);
        let recompiled = doc.compile().unwrap();
        assert_eq!(recompiled.fingerprint(), model.fingerprint());
        assert_eq!(recompiled.pattern_count(), model.pattern_count());
    }

    #[test]
    fn source_model_doc_json_round_trips() {
        let (model, _topo) = checkerboard();
        let doc = SourceModelDoc::from_model(&model);
        let json = serde_json::to_string(&doc).unwrap();
        let back: SourceModelDoc = serde_json::from_str(&json).unwrap();
        assert_eq!(back, doc);
        assert_eq!(back.compile().unwrap().fingerprint(), model.fingerprint());
    }

    #[test]
    fn compile_rejects_unknown_schema_version() {
        let (model, _topo) = checkerboard();
        let mut doc = SourceModelDoc::from_model(&model);
        doc.version = 999;
        assert_eq!(doc.compile().unwrap_err(), ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: 999 });
    }

    #[test]
    fn hand_authored_asymmetric_allow_fails_validate_on_compile() {
        // A hand-edited document declares `adj` self-inverse but only allows black->white, never
        // the reverse: `compile()` must still run `validate()` and reject it, not just build
        // silently-broken bitset tables.
        let doc = SourceModelDoc {
            version: SOURCE_MODEL_VERSION,
            patterns: vec![PatternDoc { weight: 1.0, tags: vec![] }, PatternDoc { weight: 1.0, tags: vec![] }],
            relations: vec![RelationDoc { name: "adj".to_string(), inverse: None }],
            allow: vec![PairDoc { relation: 0, src: 0, dst: 1 }],
            deny: vec![],
        };
        assert!(matches!(doc.compile().unwrap_err(), ModelError::AsymmetricInverse { .. }));
    }

    #[test]
    fn checkpoint_doc_round_trips_and_resumes() {
        let (model, topo) = checkerboard();
        let fingerprint = model.fingerprint();
        let mut domains = vec![model.full_domain(); topo.node_count()];
        let mut pinned = PatternSet::new_empty(model.pattern_count());
        pinned.set(PatternId(0), true);
        domains[0] = pinned;
        let checkpoint = Checkpoint::new(domains, fingerprint, 5);

        let doc = CheckpointDoc::from_checkpoint(&checkpoint);
        let json = serde_json::to_string(&doc).unwrap();
        let back: CheckpointDoc = serde_json::from_str(&json).unwrap();
        let restored = back.into_checkpoint(&model, topo.node_count()).unwrap();
        assert_eq!(restored.model_fingerprint, fingerprint);
        assert_eq!(restored.seed, 5);
        assert!(restored.domains[0].get(PatternId(0)));
    }

    #[test]
    fn checkpoint_doc_rejects_version_mismatch() {
        let (model, topo) = checkerboard();
        let mut doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: model.fingerprint(), seed: 0 };
        doc.version = 7;
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: 7 });
    }

    #[test]
    fn checkpoint_doc_rejects_fingerprint_mismatch() {
        let (model, topo) = checkerboard();
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count()], model_fingerprint: 0xDEAD_BEEF, seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
    }

    #[test]
    fn checkpoint_doc_rejects_wrong_domain_count() {
        let (model, topo) = checkerboard();
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains: vec![model.full_domain(); topo.node_count() - 1], model_fingerprint: model.fingerprint(), seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
    }

    #[test]
    fn checkpoint_doc_rejects_wrong_bitset_length() {
        let (model, topo) = checkerboard();
        let mut domains = vec![model.full_domain(); topo.node_count()];
        domains[0] = PatternSet::new_full(model.pattern_count() + 1);
        let doc = CheckpointDoc { version: CHECKPOINT_VERSION, domains, model_fingerprint: model.fingerprint(), seed: 0 };
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
    }

    #[test]
    fn checkpoint_doc_rejects_tampered_bitset_from_raw_json() {
        // Simulates a hand-edited file: valid JSON shape, but a bitset with a stray bit set past
        // its declared `len` in the `words` array — must be caught by `is_well_formed`, not panic.
        let (model, topo) = checkerboard();
        let json = format!(
            r#"{{"version":1,"domains":[{{"words":[999999],"len":2}}{}],"model_fingerprint":{},"seed":0}}"#,
            ",{\"words\":[3],\"len\":2}".repeat(topo.node_count() - 1),
            model.fingerprint()
        );
        let doc: CheckpointDoc = serde_json::from_str(&json).unwrap();
        assert_eq!(doc.into_checkpoint(&model, topo.node_count()).unwrap_err(), SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
    }
}
// #endregion 🔖️Tests
