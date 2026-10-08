use super::*;

/// 🔬️ `content_addressed_checkpoint_id_core`'s committed-`Change` branch now hashes
/// `crate::os_pack::json::to_json_string(change)` instead of `serde_json::to_vec(change)` —
/// direct proof the two are byte-identical for `Change`, both with and without `description`
/// (its one `Option` field, `skip_serializing_if`-omitted when `None`).
#[test]
fn change_to_json_string_matches_serde_json_byte_for_byte() {
    for description in [Some("a change".to_string()), None] {
        let change = Change { id: "change-x".into(), edit_ids: vec!["edit-1".into(), "edit-2".into()], description, saved_at: "2026-09-01T00:00:00Z".into() };
        let mine = semio_framework_pack_json::to_json_string(&change);
        let theirs = serde_json::to_string(&change).unwrap();
        assert_eq!(mine, theirs, "Change's ToValue/pack::json bridge diverged from serde_json for description={:?}", change.description);
    }
}

/// 🔬️ `pending_change_ref_json`'s hand-built wire shape, byte-for-byte against an
/// independent `serde_json` oracle (a local `#[derive(Serialize)]` twin reproducing
/// `PendingChangeRef`'s pre-conversion shape) — the direct proof this ticket's own
/// `float-format-parity.md` calls for, that converting `content_addressed_checkpoint_id_core`
/// off `serde_json` changed zero bytes. Both branches (`description` present and absent, since
/// unlike `Change` this type has no `skip_serializing_if`) are checked.
#[test]
fn pending_change_ref_json_matches_serde_json_oracle() {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Oracle<'a> {
        id: &'a str,
        edit_ids: &'a [String],
        description: Option<&'a str>,
        saved_at: &'a str,
    }
    let edit_ids = vec!["edit-1".to_string(), "edit-2".to_string()];
    for description in [Some("a pending change"), None] {
        let pending = PendingChangeRef { id: "change-x", edit_ids: &edit_ids, description, saved_at: "2026-09-01T00:00:00Z" };
        let mine = pending_change_ref_json(&pending);
        let oracle = Oracle { id: "change-x", edit_ids: &edit_ids, description, saved_at: "2026-09-01T00:00:00Z" };
        let theirs = serde_json::to_string(&oracle).unwrap();
        assert_eq!(mine, theirs, "pending_change_ref_json diverged from the serde_json oracle for description={description:?}");
    }
}
