//#region 🧪️DependencyContributionLaws
//! 🧪️ Builder contribution fixture laws, codecs, and source provenance.
use super::*;
use protocol::{Mutation, MutationDiff, MutationLeaf, OpBinary, OpText};

fn cases() -> serde_json::Value { serde_json::from_str(include_str!("../🔣️.json")).expect("neutral builder fixture cases") }
fn operation(delta: i32) -> DependencyTestOp { DependencyTestOp::AddValue(AddValue { delta }) }

pub(super) fn assert_add_value_contract(descriptor: &str) {
    assert_eq!(serde_json::to_value(AddValue::DESCRIPTOR).expect("descriptor JSON"), serde_json::from_str::<serde_json::Value>(descriptor).expect("owned descriptor JSON"));
    assert!(AddValue::DESCRIPTOR.validate().is_ok());
    assert_eq!(operation(5).descriptor(), &AddValue::DESCRIPTOR);
    assert_eq!(<DependencyTestOp as Mutation<DependencyTestSnapshot>>::DESCRIPTORS.len(), 1);
    assert_eq!(operation(5).descriptor().binary_tag, Some(0));
    let base = DependencyTestSnapshot { value: 7 };
    assert_eq!(operation(5).diff(&base).diff().apply(&base).expect("direct add"), DependencyTestSnapshot { value: 12 });
}

#[test]
fn actual_descriptor_provenance() {
    let provenance = AddValue::PROVENANCE;
    assert_eq!(provenance.owner, AddValue::DESCRIPTOR.owner);
    assert_eq!(provenance.source_path, format!("{}/🦀️.rs", provenance.owner));
    assert_eq!(provenance.descriptor_path, format!("{}/🔣️.json", provenance.owner));
    assert!(provenance.owner.ends_with("/🧬️mutations/➕️add-value"));
    let scope = protocol::MutationLeafSourceScope { workspace_token: provenance.workspace_token, mutation_root: provenance.mutation_root, taxonomy_path: provenance.taxonomy_path, source_filename: "🦀️.rs", descriptor_filename: "🔣️.json" };
    assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR, &provenance, &scope).is_ok());
    let mut invalid = provenance;
    invalid.source_path = "elsewhere/🦀️.rs";
    assert!(protocol::validate_mutation_leaf_source(&AddValue::DESCRIPTOR, &invalid, &scope).is_err());
}

#[test]
fn exact_i32_inverse_and_boundary_laws() {
    for row in cases()["cases"].as_array().expect("cases") {
        let base = DependencyTestSnapshot { value: i32::try_from(row["base"].as_i64().expect("base")).expect("i32") };
        let mut current = base.clone();
        let mut inverse = Vec::new();
        let mut rejected = false;
        for value in row["deltas"].as_array().expect("deltas") {
            let mutation = operation(i32::try_from(value.as_i64().expect("delta")).expect("i32"));
            inverse.extend(mutation.inverse(&current));
            match mutation.diff(&current).diff().apply(&current) {
                Ok(next) => current = next,
                Err(error) => { assert_eq!(error.code, "mutation.apply.overflow"); rejected = true; break; }
            }
        }
        if row["error"].as_bool() == Some(true) { assert!(rejected, "{row}"); continue; }
        assert!(!rejected);
        assert_eq!(i64::from(current.value), row["result"].as_i64().expect("result"));
        let stored: Vec<_> = inverse.iter().map(|mutation| { let DependencyTestOp::AddValue(leaf) = mutation; leaf.delta }).collect();
        assert_eq!(serde_json::to_value(stored).expect("stored inverse"), row["inverse"]);
        for inverse in inverse.iter().rev() { current = inverse.diff(&current).diff().apply(&current).expect("Store reverse inverse"); }
        assert_eq!(current, base);
    }
}

#[test]
fn ordered_diff_preserves_rejection() {
    let mut diff = DependencyTestDiff { deltas: vec![i32::MAX] };
    diff.absorb(DependencyTestDiff { deltas: vec![-i32::MAX] });
    assert_eq!(diff.deltas, [i32::MAX, -i32::MAX]);
    assert!(diff.apply(&DependencyTestSnapshot { value: 1 }).is_err());
    assert_eq!(diff.apply(&DependencyTestSnapshot { value: 0 }).expect("valid sequence").value, 0);
    assert_eq!(DependencyTestDiff::default().apply(&DependencyTestSnapshot { value: 9 }).expect("identity").value, 9);
}

#[test]
fn contribution_plan_matches_direct_leaf() {
    let leaf = AddValue { delta: 5 };
    let base = DependencyTestSnapshot { value: 7 };
    let plan = protocol::plan_of::<DependencyTestSnapshot, DependencyTestOp, AddValue>(&leaf, &base).expect("contribution plan");
    assert_eq!(plan.len(), 1);
    assert!(matches!(&plan[0], protocol::PlanStep::Local(DependencyTestOp::AddValue(AddValue { delta: 5 }))));
    let direct = operation(5).diff(&base).diff().apply(&base).expect("direct result");
    let folded = protocol::fold_plan_diff(&leaf, &base).diff().apply(&base).expect("contribution result");
    assert_eq!(direct, folded);
    assert_eq!(<AddValue as protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp>>::SEMANTICS.kind, "add-value");
    assert_eq!(<AddValue as protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp>>::label(&leaf), "Add 5 to value");
    let minimum = AddValue { delta: i32::MIN };
    let zero = DependencyTestSnapshot { value: 0 };
    assert_eq!(protocol::fold_plan_inverse(&minimum, &zero), vec![operation(1), operation(i32::MAX)]);
    assert!(protocol::plan_of::<DependencyTestSnapshot, DependencyTestOp, AddValue>(&AddValue { delta: 1 }, &DependencyTestSnapshot { value: i32::MAX }).is_err());
}

#[test]
fn strict_payload_and_all_codecs() {
    for delta in [i32::MIN, -5, 0, 5, i32::MAX] {
        let mutation = operation(delta);
        let json = serde_json::to_value(&mutation).expect("operation JSON");
        assert_eq!(json, serde_json::json!({"operation":"addValue","delta":delta}));
        assert_eq!(serde_json::from_value::<DependencyTestOp>(json).expect("JSON decode"), mutation);
        let text = mutation.print_op();
        assert!(text.starts_with("add-value"));
        assert_eq!(DependencyTestOp::parse_op(&text).expect("text decode"), mutation);
        let bytes = mutation.encode_op().expect("binary encode");
        assert_eq!(&bytes[..2], &[1,0]);
        assert_eq!(DependencyTestOp::decode_op(&bytes).expect("binary decode"), mutation);
    }
    for value in cases()["invalid"].as_array().expect("invalid cases") {
        assert!(serde_json::from_value::<AddValue>(value.clone()).is_err());
        let mut envelope = value.clone();
        envelope["operation"] = serde_json::json!("addValue");
        assert!(serde_json::from_value::<DependencyTestOp>(envelope).is_err());
    }
    assert!(DependencyTestOp::decode_op(&[1,1]).is_err());
}

#[test]
fn keyword_owned_record_codec_is_forwarded_once() {
    let spec = AddValue::__dsl_spec();
    assert_eq!(spec.keyword.as_deref(), Some("add-value"));
    let variants = <DependencyTestOp as dsl::DslVariants>::variants();
    assert_eq!((variants[0].1)().keyword, spec.keyword);
    let rows: serde_json::Value = serde_json::from_str(include_str!("🔤️keywords/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let delta = i32::try_from(row["delta"].as_i64().unwrap()).unwrap();
        let expected = row["text"].as_str().unwrap();
        let mutation = operation(delta);
        assert_eq!(mutation.print_op(), expected);
        assert_eq!(DependencyTestOp::parse_op(expected).unwrap(), mutation);
        assert!(DependencyTestOp::parse_op(expected.strip_prefix("add-value ").unwrap()).is_err());
        assert!(DependencyTestOp::parse_op(&format!("add-value {expected}")).is_err());
    }
}
//#endregion 🧪️DependencyContributionLaws
