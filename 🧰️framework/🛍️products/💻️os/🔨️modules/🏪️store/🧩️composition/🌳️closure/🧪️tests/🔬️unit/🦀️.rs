//! 🧪️ Recursive closure admission from actual typed snapshot projections.
use super::*;
use crate::os_schema_composition::{ArtifactCompositionFields, ChildRefFields, ChildRefVisitor, ChildSlotSpec};
use crate::os_store::ChildRef;
use semio_framework_job::{root_cancel_token, StepBudget};
use std::cell::Cell;

struct FixtureNode { reference: ArtifactRef, owner: Option<OwnerRef>, children: Vec<ChildRef> }
struct FixtureSource { generation: Cell<u64>, root: FixtureNode, members: Vec<FixtureNode>, projections: Cell<usize> }

impl ArtifactCompositionFields for FixtureNode {
    fn child_slots() -> &'static [ChildSlotSpec] {
        &[
            ChildSlotSpec { name: "objects", kind: "s.stdio.semio", many: true },
            ChildSlotSpec { name: "mesh", kind: "s.stdio.semio", many: true },
            ChildSlotSpec { name: "value", kind: "s.stdio.semio", many: true },
            ChildSlotSpec { name: "children", kind: "s.stdio.semio", many: true },
            ChildSlotSpec { name: "other", kind: "s.stdio.semio", many: true },
        ]
    }
    fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        for child in &self.children {
            visitor.step()?;
            let slot = Self::child_slots().iter().find(|slot| slot.name == child.slot).expect("fixture declares every child slot").name;
            visitor.child(slot, ChildRefFields {
                child_id: &child.child_id, artifact_id: &child.target.artifact_id, artifact_kind: &child.target.dialect.artifact_kind,
                standard: &child.target.dialect.standard, subset: &child.target.dialect.subset,
            })?;
        }
        Ok(())
    }
}

impl OwnedDocumentClosureSource for FixtureSource {
    fn generation(&self) -> u64 { self.generation.get() }
    fn root_reference(&self) -> &ArtifactRef { &self.root.reference }
    fn member_count(&self) -> usize { self.members.len() }
    fn member_reference(&self, index: usize) -> Option<&ArtifactRef> { self.members.get(index).map(|member| &member.reference) }
    fn member_owner(&self, index: usize) -> Option<&OwnerRef> { self.members.get(index).and_then(|member| member.owner.as_ref()) }
    fn child_projection(&self, parent: Option<usize>) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        self.projections.set(self.projections.get() + 1);
        ChildRestoreProjection::from_snapshot(parent.map_or(&self.root, |index| &self.members[index]))
    }
}

fn fixture_source(input: &serde_json::Value) -> FixtureSource {
    fn node(row: &serde_json::Value) -> FixtureNode {
        FixtureNode {
            reference: crate::os_pack::json::from_json_str(&row["reference"].to_string()).unwrap(),
            owner: row.get("owner").map(|owner| crate::os_pack::json::from_json_str(&owner.to_string()).unwrap()),
            children: crate::os_pack::json::from_json_str(&row["children"].to_string()).unwrap(),
        }
    }
    FixtureSource { generation: Cell::new(7), root: node(&input["root"]), members: input["members"].as_array().unwrap().iter().map(node).collect(), projections: Cell::new(0) }
}

fn reference(index: usize) -> ArtifactRef {
    ArtifactRef { artifact_id: format!("node-{index}"), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "object".into() } }
}

fn chain(count: usize) -> FixtureSource {
    let children = |index: usize| {
        if index < count { vec![ChildRef { slot: "children".into(), child_id: format!("node-{}", index + 1), target: reference(index + 1) }] } else { vec![] }
    };
    FixtureSource {
        generation: Cell::new(7), projections: Cell::new(0),
        root: FixtureNode { reference: reference(0), owner: None, children: children(0) },
        members: (1..=count).map(|index| FixtureNode {
            reference: reference(index), owner: Some(OwnerRef { parent: reference(index - 1), slot: "children".into(), child_id: format!("node-{index}") }), children: children(index),
        }).collect(),
    }
}

fn validates(source: &FixtureSource, fuel: u64) -> bool {
    let mut cursor = OwnedDocumentClosure::new(OperationId(11), Generation(13), 7, 100);
    let mut sequence = 0;
    let cancel = root_cancel_token();
    let mut zero = StepContext::new(OperationId(11), Generation(13), StepBudget::new(0, 99), cancel.clone(), || Some(1), &mut sequence);
    let zero_step = cursor.step(source, &mut zero);
    assert_eq!(cursor.progress().steps, 0);
    assert_eq!(source.projections.get(), 0);
    if source.member_count() > OWNED_DOCUMENT_MAXIMUM_MEMBERS {
        assert_eq!(zero_step, OwnedDocumentClosureStep::Rejected(OwnedDocumentClosureDiagnostic::MemberLimit));
        return false;
    }
    assert!(matches!(zero_step, OwnedDocumentClosureStep::Pending(_)));
    for _ in 0..20_000 {
        let before = cursor.progress().steps;
        let mut cx = StepContext::new(OperationId(11), Generation(13), StepBudget::new(fuel, 99), cancel.clone(), || Some(1), &mut sequence);
        let step = cursor.step(source, &mut cx);
        assert!(cursor.progress().steps - before <= fuel);
        match step {
            OwnedDocumentClosureStep::Complete { members } => { assert_eq!(members, source.member_count()); return true; }
            OwnedDocumentClosureStep::Rejected(_) => return false,
            OwnedDocumentClosureStep::Pending(_) => {}
        }
    }
    panic!("bounded closure validation did not terminate");
}

#[test]
fn owned_document_closure_matches_neutral_graphs_and_independent_limits() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["continuationCases"].as_array().unwrap() {
        let mut source = chain(row["beforeMembers"].as_u64().unwrap() as usize);
        let mut cursor = OwnedDocumentClosure::new(OperationId(11), Generation(13), 7, 100);
        let mut sequence = 0;
        let cancel = root_cancel_token();
        let mut first = StepContext::new(OperationId(11), Generation(13), StepBudget::new(1, 99), cancel.clone(), || Some(1), &mut sequence);
        assert!(matches!(cursor.step(&source, &mut first), OwnedDocumentClosureStep::Pending(_)));
        let progress = cursor.progress();
        source.members = chain(row["afterMembers"].as_u64().unwrap() as usize).members;
        let mut next = StepContext::new(OperationId(11), Generation(13), StepBudget::new(1, 99), cancel, || Some(1), &mut sequence);
        assert_eq!(cursor.step(&source, &mut next), OwnedDocumentClosureStep::Rejected(OwnedDocumentClosureDiagnostic::Stale));
        assert_eq!(cursor.progress(), progress);
    }
    for row in fixture["ownerCases"]["valid"].as_array().unwrap() {
        let owner: OwnerRef = crate::os_pack::json::from_json_str(&row.to_string()).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&crate::os_pack::json::to_json_string(&owner)).unwrap(), *row);
    }
    for row in fixture["ownerCases"]["invalid"].as_array().unwrap() {
        assert!(crate::os_pack::json::from_json_str::<OwnerRef>(&row.to_string()).is_err(), "invalid owner stamp accepted: {row}");
    }
    for row in fixture["cases"].as_array().unwrap() {
        for fuel in [1, 7, 64] {
            assert_eq!(validates(&fixture_source(&row["input"]), fuel), row["accepted"].as_bool().unwrap(), "{} fuel={fuel}", row["id"]);
        }
    }
    for row in fixture["chainCases"].as_array().unwrap() {
        let source = chain(row["members"].as_u64().unwrap() as usize);
        assert_eq!(validates(&source, 1), row["accepted"].as_bool().unwrap(), "chain {}", row["members"]);
    }
    for row in fixture["breadthCases"].as_array().unwrap() {
        let mut source = chain(row["children"].as_u64().unwrap() as usize);
        source.root.children = source.members.iter().map(|member| ChildRef { slot: "children".into(), child_id: member.reference.artifact_id.clone(), target: member.reference.clone() }).collect();
        for member in &mut source.members { member.owner.as_mut().unwrap().parent = source.root.reference.clone(); member.children.clear(); }
        assert_eq!(validates(&source, 1), row["accepted"].as_bool().unwrap(), "breadth {}", row["children"]);
    }
    eprintln!("[DEBUG] Recursive closure: 16 neutral graphs x3 grants, 1024-node chain accepted, 65 references on one parent refused");
}

#[test]
fn owned_document_closure_cancellation_generation_and_deadline_retain_source() {
    assert!(!std::mem::needs_drop::<OwnedDocumentClosure>());
    for (operation, generation, source_generation, cancelled, now, reason) in [
        (12, 13, 7, false, 1, OwnedDocumentClosureDiagnostic::Stale),
        (11, 14, 7, false, 1, OwnedDocumentClosureDiagnostic::Stale),
        (11, 13, 8, false, 1, OwnedDocumentClosureDiagnostic::Stale),
        (11, 13, 7, true, 1, OwnedDocumentClosureDiagnostic::Cancelled),
        (11, 13, 7, false, 100, OwnedDocumentClosureDiagnostic::Expired),
    ] {
        let source = chain(3);
        let original = source.members.as_ptr();
        let mut cursor = OwnedDocumentClosure::new(OperationId(11), Generation(13), 7, 100);
        let mut sequence = 0;
        let cancel = root_cancel_token();
        let mut first = StepContext::new(OperationId(11), Generation(13), StepBudget::new(2, 99), cancel.clone(), || Some(1), &mut sequence);
        assert!(matches!(cursor.step(&source, &mut first), OwnedDocumentClosureStep::Pending(_)));
        let progress = cursor.progress();
        source.generation.set(source_generation);
        if cancelled { cancel.cancel_now(); }
        let clock: fn() -> Option<u64> = if now == 100 { || Some(100) } else { || Some(1) };
        let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(64, 999), cancel, clock, &mut sequence);
        assert_eq!(cursor.step(&source, &mut cx), OwnedDocumentClosureStep::Rejected(reason));
        assert_eq!(cursor.progress(), progress);
        assert_eq!(source.members.as_ptr(), original);
        assert_eq!(source.members.len(), 3);
    }
    eprintln!("[DEBUG] Recursive closure resumes with scalar metadata only; five sticky authority denials retain exact candidate source allocation");
}
