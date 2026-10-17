//! 🧪️ Real source-owned miniature document for aggregate and registry integration laws.

use semio_framework_test_fixtures::fixture_path;

//#region 🧬️Document
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct MiniDoc {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct MiniDiff {
    pub name: Option<String>,
}

impl protocol::DiffAlgebra<MiniDoc> for MiniDiff {
    fn inverse(&self, base: &MiniDoc) -> Self { Self { name: self.name.as_ref().map(|_| base.name.clone()) } }
    fn is_empty(&self) -> bool { self.name.is_none() }
}

impl protocol::MutationDiff<MiniDoc> for MiniDiff {
    fn apply(&self, base: &MiniDoc, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<MiniDoc> {
        Ok(MiniDoc { name: self.name.clone().unwrap_or_else(|| base.name.clone()) })
    }
    fn absorb(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
    }
}
//#endregion 🧬️Document

//#region 🧬️Mutations
#[path = "../../🧪️testing/📔️registry/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;
//#endregion 🧬️Mutations

#[test]
fn mini_diff_rename_matches_neutral_names() {
    use protocol::DiffAlgebra;
    let fixture: serde_json::Value = serde_json::from_str(std::fs::read_to_string(fixture_path!("../../../../../🧪️testing/🧬️mutation-laws/🔣️.json")).expect("fixture").as_str()).unwrap();
    for row in fixture["rename"].as_array().unwrap() {
        let base: MiniDoc = serde_json::from_value(serde_json::json!({"name": row["base"].as_str().unwrap()})).unwrap();
        let other: MiniDoc = serde_json::from_value(serde_json::json!({"name": row["other"].as_str().unwrap()})).unwrap();
        let diff = MiniDiff { name: (base != other).then(|| other.name.clone()) };
        assert_eq!(protocol::apply_diff(&diff, &base), Ok(other.clone()));
        assert_eq!(protocol::apply_diff(&diff.inverse(&base), &other), Ok(base.clone()));
        assert_eq!(diff.is_empty(), base == other);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&diff)).unwrap(), serde_json::to_value(&diff).unwrap());
    }
}
