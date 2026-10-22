//! 🗄️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use semio_framework_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, document_load_fault};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::children_components::ChildComposition;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🪆️ Every live owned child as its archive member (persisted envelope, identity, owner), sorted by owner and identity
    /// with dense ordinals — the member roster of the document archive, which is also the one `artifact:out` carrier.
    pub async fn archive_member_entries(&self) -> Result<Vec<protocol::OwnedDocumentMemberPackEntry>, Fault> {
        let mut members = Vec::new();
        members.try_reserve_exact(self.children.len()).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member roster could not reserve its exact live extent"))?;
        for entry in self.children.entries() {
            let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
            members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: protocol::DocumentArchiveArtifactRef {
                    artifact_id: entry.reference.artifact_id.clone(),
                    artifact_kind: entry.reference.dialect.artifact_kind.clone(),
                    standard: entry.reference.dialect.standard.clone(),
                    subset: entry.reference.dialect.subset.clone(),
                },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef {
                        artifact_id: entry.owner.parent.artifact_id.clone(),
                        artifact_kind: entry.owner.parent.dialect.artifact_kind.clone(),
                        standard: entry.owner.parent.dialect.standard.clone(),
                        subset: entry.owner.parent.dialect.subset.clone(),
                    },
                    slot: entry.owner.slot.clone(),
                    child_id: entry.owner.child_id.clone(),
                },
                envelope_pack,
            });
        }
        members.sort_by(|left, right| {
            (&left.owner.parent.artifact_id, &left.owner.slot, &left.owner.child_id, &left.reference.artifact_kind, &left.reference.standard, &left.reference.subset).cmp(&(
                &right.owner.parent.artifact_id,
                &right.owner.slot,
                &right.owner.child_id,
                &right.reference.artifact_kind,
                &right.reference.standard,
                &right.reference.subset,
            ))
        });
        for (ordinal, entry) in members.iter_mut().enumerate() {
            entry.ordinal = u32::try_from(ordinal).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member ordinal exceeds u32"))?;
        }
        Ok(members)
    }
}

/// 🪪️ The persisted schema a derivable child of `dialect` is minted under — read from the member
/// roster's own open declarations, so an app deriving a child of a dialect its `M` never declares
/// is a fault rather than a member nothing could ever open.
#[doc(hidden)]
pub fn genesis_member_schema<M: MemberFactory>(dialect: &ArtifactDialect) -> Result<&'static str, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    M::OPEN_DECLARATIONS
        .iter()
        .find(|declaration| declaration.kind == dialect.artifact_kind && declaration.standard == dialect.standard && declaration.subset == dialect.subset)
        .map(|declaration| declaration.schema)
        .ok_or_else(|| plugin_sdk_fault(format!("derived child dialect '{}' is not declared by this app's member roster", dialect.to_coordinate())))
}
