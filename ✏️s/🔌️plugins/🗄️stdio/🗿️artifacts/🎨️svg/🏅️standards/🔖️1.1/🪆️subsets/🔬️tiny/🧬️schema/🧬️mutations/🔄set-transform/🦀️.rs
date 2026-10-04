//! 🔄️ `set-transform` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTransform {
    pub(crate) path: NodePath,
    pub(crate) transform: Option<Vec<TransformOp>>,
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for SetTransform {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "transform", kind: "set-transform", record: "SetTransform" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<<SvgTinyMutation as Mutation<SvgSnapshot>>::Diff> {
        agg_diff(&SvgTinyMutation::SetTransform(self.clone()), base)
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&SvgTinyMutation::SetTransform(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set transform", "Transformation setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
