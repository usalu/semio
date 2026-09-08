//! 🧪️ Real source-owned miniature document for aggregate and registry integration laws.

//#region 🧬️Document
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct MiniDoc {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct MiniDiff {
    pub name: Option<String>,
}

impl crate::os_spr::MutationDiff<MiniDoc> for MiniDiff {
    fn apply(&self, base: &MiniDoc) -> crate::os_spr::MutationApplyResult<MiniDoc> {
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
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;
//#endregion 🧬️Mutations
