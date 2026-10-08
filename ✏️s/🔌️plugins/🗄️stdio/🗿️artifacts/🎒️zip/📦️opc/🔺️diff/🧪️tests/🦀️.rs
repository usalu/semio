//! 🧪️ The OPC package diff through the central applier: positions are exact, the rewind restores the base, and a diff absorbed with its rewind is the identity.

use super::*;
use semio_s_artifact_stdio_contract::kernel::command::DiffAlgebra;
use semio_s_artifact_stdio_contract::kernel::{apply_diff, MutationApplyResult, MutationDiff};

struct PackageDiff(OpcDiff);

impl MutationDiff<OpcPackage> for PackageDiff {
    fn apply(&self, base: &OpcPackage, capability: ApplyCapability) -> MutationApplyResult<OpcPackage> {
        let mut next = base.clone();
        self.0.commit_into(&mut next, capability)?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.0.absorb(other.0);
    }
}

impl DiffAlgebra<OpcPackage> for PackageDiff {
    fn inverse(&self, base: &OpcPackage) -> Self {
        Self(self.0.rewind(base))
    }

    fn is_empty(&self) -> bool {
        self.0 == OpcDiff::default()
    }
}

fn relationship(id: &str, target: &str) -> OpcRelationship {
    OpcRelationship { id: id.into(), rel_type: "http://example.invalid/relationships/demo".into(), target: target.into(), target_mode: OpcTargetMode::Internal }
}

fn package() -> OpcPackage {
    let mut package = OpcPackage::default();
    package.content_types.defaults = vec![("rels".into(), "application/rels".into()), ("xml".into(), "application/xml".into())];
    package.content_types.overrides = vec![("/a.xml".into(), "application/a".into()), ("/b.xml".into(), "application/b".into())];
    package.parts = vec![
        OpcPart { path: "a.xml".into(), content_type: "application/a".into(), bytes: vec![1] },
        OpcPart { path: "b.xml".into(), content_type: "application/b".into(), bytes: vec![2] },
        OpcPart { path: "c.bin".into(), content_type: "application/octet-stream".into(), bytes: vec![3] },
    ];
    package.relationships.replace_owner(String::new(), vec![relationship("rId1", "a.xml"), relationship("rId2", "b.xml")]);
    package.relationships.replace_owner("a.xml".into(), vec![relationship("rId1", "c.bin")]);
    package
}

fn diff() -> PackageDiff {
    PackageDiff(OpcDiff {
        comment: Some("edited".into()),
        content_types: Some(OpcContentTypesDiff {
            defaults: Some(OpcContentTypeEntriesDelta { inserted: vec![OpcContentTypeInsertion { index: 0, row: OpcContentTypeRow { name: "png".into(), content_type: "image/png".into() } }], ..Default::default() }),
            overrides: Some(OpcContentTypeEntriesDelta { removed: vec![OpcContentTypeRemoval { id: "/a.xml".into(), index: 0 }], ..Default::default() }),
        }),
        parts: Some(OpcPartsDelta {
            moved: vec![OpcPartRelocation { id: "c.bin".into(), from: 2, to: 0 }],
            modified: vec![OpcPartModification { id: "b.xml".into(), patch: OpcPartPatch { bytes: Some(vec![9, 9]), ..Default::default() } }],
            ..Default::default()
        }),
        relationships: Some(OpcOwnersDelta {
            removed: vec![OpcOwnerRemoval { id: "a.xml".into(), index: 1 }],
            modified: vec![OpcOwnerModification {
                id: String::new(),
                patch: OpcOwnerPatch { relationships: OpcRelationshipsDelta { inserted: vec![OpcRelationshipInsertion { index: 0, row: relationship("rId3", "c.bin") }], ..Default::default() } },
            }],
            ..Default::default()
        }),
    })
}

#[test]
fn every_row_lands_at_the_position_its_coordinate_names() {
    let after = apply_diff(&diff(), &package()).expect("the diff applies");
    assert_eq!(after.comment, "edited");
    assert_eq!(after.content_types.defaults.first().map(|(name, _)| name.as_str()), Some("png"));
    assert_eq!(after.content_types.overrides, vec![("/b.xml".to_string(), "application/b".to_string())]);
    assert_eq!(after.parts.iter().map(|part| part.path.as_str()).collect::<Vec<_>>(), vec!["c.bin", "a.xml", "b.xml"]);
    assert_eq!(after.parts[2].bytes, vec![9, 9]);
    assert_eq!(after.relationships.relationships("").map(|list| list.iter().map(|rel| rel.id.clone()).collect::<Vec<_>>()), Some(vec!["rId3".to_string(), "rId1".to_string(), "rId2".to_string()]));
    assert!(after.relationships.relationships("a.xml").is_none(), "the owner left with its relationships");
}

#[test]
fn the_rewind_restores_the_base_and_a_diff_absorbed_with_its_rewind_is_the_identity() {
    let base = package();
    let forward = diff();
    let after = apply_diff(&forward, &base).expect("the diff applies");
    let backward = forward.inverse(&base);
    assert_eq!(apply_diff(&backward, &after).expect("the rewind applies"), base);
    let mut summed = diff();
    summed.absorb(backward);
    assert_eq!(apply_diff(&summed, &base).expect("the sum applies"), base);
}

#[test]
fn an_owner_delta_that_breaks_the_owner_order_is_refused() {
    let broken = PackageDiff(OpcDiff { relationships: Some(OpcOwnersDelta { inserted: vec![OpcOwnerInsertion { index: 0, row: OpcOwnerRow { owner: "z.xml".into(), relationships: vec![relationship("rId1", "c.bin")] } }], ..Default::default() }), ..Default::default() });
    assert_eq!(apply_diff(&broken, &package()).unwrap_err().code, "mutation.apply.invalid-order");
}

#[test]
fn a_removal_must_name_the_row_at_its_base_index() {
    let wrong = PackageDiff(OpcDiff { parts: Some(OpcPartsDelta { removed: vec![OpcPartRemoval { id: "a.xml".into(), index: 1 }], ..Default::default() }), ..Default::default() });
    assert_eq!(apply_diff(&wrong, &package()).unwrap_err().code, "mutation.apply.missing-target");
}
