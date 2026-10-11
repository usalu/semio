//! 📏️ `scale-object` — sets the object's `transform.scale`, keeping translation/rotation.

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct ScaleObject {
    pub scale: SemioPoint3,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for ScaleObject {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "object", kind: "scale-object", record: "ScaledObject" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale object to ({}, {}, {})", self.scale.x, self.scale.y, self.scale.z), &format!("Objekt auf ({}, {}, {}) skalieren", self.scale.x, self.scale.y, self.scale.z))
    }
    fn target(&self) -> Vec<String> {
        vec!["transform".to_string()]
    }
}
//#endregion 🔖️Payload
