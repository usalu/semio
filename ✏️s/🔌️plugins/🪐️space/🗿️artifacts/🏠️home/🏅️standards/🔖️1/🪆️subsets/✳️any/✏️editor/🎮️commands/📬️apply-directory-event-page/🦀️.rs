//! 📄️ Accepts one authenticated, receipt-sealed directory page as one local config replacement.

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{DirectoryProjectionReceiptV1, HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{AppEvent, ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-directory-event-page")]
pub struct ApplyDirectoryEventPage {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON returned by the authenticated hub.
    pub page_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handle
pub fn handle(payload: &ApplyDirectoryEventPage, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let page = store::os_directory::DirectoryEventPageV1::parse_canonical_json(&payload.page_json).map_err(|_| Fault::from("s.home.directory-event-page-invalid"))?;
    let next = cfg.snapshot.apply_directory_event_page(&page)?;
    let receipt = next.directory_projection_receipt().ok_or_else(|| Fault::from("s.home.directory-projection-receipt-invalid"))?;
    let event = AppEvent { kind: DirectoryProjectionReceiptV1::SCHEMA.into(), payload: protocol::ToValue::to_value(&receipt) };
    if next == *cfg.snapshot {
        return Ok(Emit { events: vec![event], ..Default::default() });
    }
    Ok(Emit {
        config_mutations: vec![HomeConfigMutation::ReplaceDirectoryProjection {
            directory_json: next.directory_json,
            session_binding_sha256: next.directory_session_binding_sha256,
            authorization_generation: next.directory_authorization_generation,
            receipt_sha256: next.directory_receipt_sha256,
        }],
        events: vec![event],
        ..Default::default()
    })
}
//#endregion 🔖️Handle

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation as _;

    fn seal(mut page: store::os_directory::DirectoryEventPageV1) -> store::os_directory::DirectoryEventPageV1 {
        page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
        page
    }

    fn dispatch(config: &HomeConfig, page: &store::os_directory::DirectoryEventPageV1) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
        let history = semio_framework_plugin::HistoryView::empty();
        let document = SHomeSnapshot::default();
        let view = ArtifactView::new(&document, &history);
        handle(&ApplyDirectoryEventPage { page_json: pack::to_json_string(page) }, &view, &ConfigView { snapshot: config })
    }

    #[semio_framework_async_macros::async_test]
    async fn sealed_page_replaces_projection_once_and_rejects_races() {
        let binding = "a".repeat(64);
        let first = seal(store::os_directory::DirectoryEventPageV1 {
            schema: "semio.directory.event-page.v1".into(),
            session_binding_sha256: binding.clone(),
            authorization_generation: 7,
            after_seq_exclusive: 0,
            through_seq_inclusive: 5,
            has_more: true,
            events: Vec::new(),
            receipt_sha256: String::new(),
        });
        let initial = HomeConfig::default();
        let emitted = dispatch(&initial, &first).expect("first sealed page");
        assert_eq!(emitted.config_mutations.len(), 1);
        assert!(emitted.artifact_mutations.is_empty());
        let current = emitted.config_mutations[0].diff(&initial).diff().clone();
        assert_eq!(current.directory().expect("projection").cursor, 5, "invisible raw holes advance the resume frontier");
        assert_eq!(current.directory_session_binding_sha256, binding);
        assert_eq!(current.directory_authorization_generation, 7);
        assert_eq!(current.directory_receipt_sha256, first.receipt_sha256);
        let duplicate = dispatch(&current, &first).expect("idempotent replay");
        assert!(duplicate.config_mutations.is_empty());
        assert_eq!(duplicate.events.len(), 1, "an already-published frontier returns the same terminal receipt without another edit");
        let duplicate_receipt: DirectoryProjectionReceiptV1 = protocol::FromValue::from_value(duplicate.events[0].payload.clone()).expect("typed duplicate receipt");
        assert_eq!(duplicate_receipt, current.directory_projection_receipt().expect("current receipt"));

        let stale = seal(store::os_directory::DirectoryEventPageV1 { after_seq_exclusive: 0, through_seq_inclusive: 6, receipt_sha256: String::new(), ..first.clone() });
        assert!(dispatch(&current, &stale).is_err(), "same-authority stale base cannot replace the projection");

        let event = store::os_directory::DirectoryEvent {
            seq: 7,
            id: "event-7".into(),
            hlc: store::os_directory::Hlc { physical_ms: 7, logical: 0 },
            actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "user:u1#s1".into() },
            space_id: Some("space-1".into()),
            user_id: None,
            body: store::os_directory::DirectoryEventBody::SpaceCreated {
                space_id: "space-1".into(),
                name: "Werkstatt".into(),
                space_kind: store::os_directory::DirectorySpaceKind::Studio,
                visibility: store::os_directory::DirectorySpaceVisibility::Private,
                owner_user_id: "u1".into(),
            },
            recorded_at_ms: 7,
        };
        let second = seal(store::os_directory::DirectoryEventPageV1 {
            schema: "semio.directory.event-page.v1".into(),
            session_binding_sha256: binding,
            authorization_generation: 7,
            after_seq_exclusive: 5,
            through_seq_inclusive: 7,
            has_more: false,
            events: vec![event],
            receipt_sha256: String::new(),
        });
        let emitted = dispatch(&current, &second).expect("ordered successor page");
        assert_eq!(emitted.events.len(), 1);
        let receipt: DirectoryProjectionReceiptV1 = protocol::FromValue::from_value(emitted.events[0].payload.clone()).expect("typed projection receipt");
        assert_eq!(receipt.schema, DirectoryProjectionReceiptV1::SCHEMA);
        assert_eq!(receipt.through_seq_inclusive, 7);
        assert_eq!(receipt.receipt_sha256, second.receipt_sha256);
        let advanced = emitted.config_mutations[0].diff(&current).diff().clone();
        let projection = advanced.directory().expect("advanced projection");
        assert_eq!(projection.cursor, 7);
        assert!(projection.spaces.contains_key("space-1"));

        let replacement = seal(store::os_directory::DirectoryEventPageV1 {
            schema: "semio.directory.event-page.v1".into(),
            session_binding_sha256: "b".repeat(64),
            authorization_generation: 8,
            after_seq_exclusive: 0,
            through_seq_inclusive: 0,
            has_more: false,
            events: Vec::new(),
            receipt_sha256: String::new(),
        });
        let emitted = dispatch(&advanced, &replacement).expect("new authority rebootstrap");
        let replaced = emitted.config_mutations[0].diff(&advanced).diff().clone();
        assert_eq!(replaced.directory().expect("replaced projection").cursor, 0);
        assert!(replaced.directory().expect("replaced projection").spaces.is_empty());

        let mut forged = second;
        forged.receipt_sha256 = "c".repeat(64);
        assert!(dispatch(&current, &forged).is_err(), "forged receipt is terminally rejected");
    }
}
//#endregion 🧪️Tests
