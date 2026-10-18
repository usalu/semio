//! ✏️ Editor and member subjects of the time-travel session: the editor, the composed member subject and metadata and the review children. See [`super`].

use super::*;

//#region 🔖️Ledger
/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the current draft payload and the draft's own outcome against the state before it. The operation
/// whose kind the draft rebuilds lives with the typed owners of the store it belongs to ([`TimeTravelStoreState`]).
/// A mutation without editable inputs — opened by a history row's Withdraw (design §22.1) — has no `schema` and no
/// inputs: its editor admits no draft but the withdrawal. Whether the draft is withdrawn is the session's pending draft.
#[derive(semio_framework_value::RetireOwned)]
pub struct TimeTravelEditor {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    pub schema: Option<&'static str>,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    pub validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    pub value: DslValue,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}

impl TimeTravelEditor {
    /// 🧯️ Stands in for a payload schema that compiles no validator, for the fail-closed law.
    #[cfg(test)]
    pub fn refuse_validator(&mut self, reason: &str) {
        self.validator = Err(reason.to_string());
    }
}

/// 🧩️ The composed member store a session edits (design §12): its owner edge, owner path and dialect, the typed owners of the
/// edit on that store (created by the first visit, erased here), and the children view every render seam reads while
/// the member's preview or replayed head is shown.
#[doc(hidden)]
pub struct TimeTravelMemberSubject {
    pub key: MemberKey,
    pub path: MemberPath,
    pub dialect: ArtifactDialect,
    pub owners: Option<Box<dyn TimeTravelOwners>>,
    pub children: Option<ChildContentView>,
    pub review_children: Option<TimeTravelReviewChildren>,
    pub metadata_close: Option<ControlledRetirement<TimeTravelMemberMetadata>>,
    pub metadata_retired: bool,
}

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct TimeTravelMemberMetadata {pub key:MemberKey,pub path:MemberPath,pub dialect:ArtifactDialect}

#[doc(hidden)]
pub struct TimeTravelReviewChildren {
    pub view: ChildContentView,
    pub base: TimeTravelBase,
    pub generation: u32,
}

impl TimeTravelReviewChildren {
    pub fn capture(view:&ChildContentView,base:TimeTravelBase,generation:u32,grant:RetainedCloneGrant)->Option<(Self,RetainedCloneProgress)>{
        let copied_bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return None}
        Some((Self{view:view.clone(),base,generation},RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))
    }
}

impl TimeTravelMemberSubject {
    /// 🧩️ The member store id the history wire names it by: the text of its owner path (`<slot>/<childId>` for a member of the document).
    pub fn store(&self) -> String {
        self.path.to_string()
    }
}
//#endregion 🔖️Ledger
