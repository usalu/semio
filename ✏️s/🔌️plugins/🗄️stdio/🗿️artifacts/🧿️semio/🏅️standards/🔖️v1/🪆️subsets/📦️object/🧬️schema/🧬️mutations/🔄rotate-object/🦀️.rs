//! 🔄️ `rotate-object` — sets the object's `transform.rotation`, keeping translation/scale.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use crate::schema::mutations::SemioObjectMutation;
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct RotateObject {
    pub rotation: SemioQuaternion,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for RotateObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "object", kind: "rotate-object", record: "RotatedObject" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rotate object", "Objekt drehen")
    }
    fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
