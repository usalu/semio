//! 🎞️ `set-image-pixels` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-image-pixels")]
pub struct SetImagePixels {
    pub(crate) index: usize,
    #[dsl(base64)]
    pub(crate) indices: Vec<u8>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetImagePixels {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "image-pixels", kind: "set-image-pixels", record: "SetImagePixels" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::SetImagePixels(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&GifMutation::SetImagePixels(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set image pixels", "Bildpixel setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
