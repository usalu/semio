use super::{HomeTransient, HomeTransientRetirementFactory};
use super::mutations::{ApplyDirectoryPage, HomeTransientDiff, HomeTransientMutation};
use crate::editor::home::transient::component::io::text::mutations::{home_transient_mutation_report_json};
use protocol::Mutation as _;
use std::sync::Arc;

//#region 🧫️Pages
const BINDING: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn event(seq: u64, body: store::os_directory::DirectoryEventBody) -> store::os_directory::DirectoryEvent {
    let space_id = match &body {
        store::os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } | store::os_directory::DirectoryEventBody::SpaceRenamed { space_id, .. } | store::os_directory::DirectoryEventBody::MemberUpserted { space_id, .. } => Some(space_id.clone()),
        _ => None,
    };
    store::os_directory::DirectoryEvent {
        seq,
        id: format!("event-{seq}"),
        hlc: store::os_directory::Hlc { physical_ms: seq as i64, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "user:u1#s1".into() },
        space_id,
        user_id: None,
        body,
        recorded_at_ms: seq as i64,
    }
}

fn created(seq: u64, space_id: &str, name: &str) -> store::os_directory::DirectoryEvent {
    event(seq, store::os_directory::DirectoryEventBody::SpaceCreated { space_id: space_id.into(), name: name.into(), space_kind: store::os_directory::DirectorySpaceKind::Studio, visibility: store::os_directory::DirectorySpaceVisibility::Private, owner_user_id: "u1".into() })
}

fn member(seq: u64, space_id: &str) -> store::os_directory::DirectoryEvent {
    event(seq, store::os_directory::DirectoryEventBody::MemberUpserted { space_id: space_id.into(), user_id: "u1".into(), role: store::os_directory::DirectorySpaceRole::Author })
}

fn sealed(after_seq_exclusive: u64, through_seq_inclusive: u64, has_more: bool, events: Vec<store::os_directory::DirectoryEvent>) -> store::os_directory::DirectoryEventPageV1 {
    let mut page = store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: BINDING.into(),
        authorization_generation: 3,
        after_seq_exclusive,
        through_seq_inclusive,
        has_more,
        events,
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    page
}

/// 📚️ A directory of `spaces` spaces the way the hub pages it: `space.created` + `member.upserted` per space, at most
/// `DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS` events per sealed page, each page continuing the previous frontier.
fn directory_pages(spaces: usize) -> Vec<String> {
    let events: Vec<_> = (0..spaces).flat_map(|index| {
        let id = format!("space-{index:05}");
        let seq = 2 * index as u64;
        [created(seq + 1, &id, &format!("Werkstatt 雪 {index}")), member(seq + 2, &id)]
    }).collect();
    let chunks: Vec<_> = events.chunks(store::os_directory::DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).collect();
    let mut after = 0;
    chunks.iter().enumerate().map(|(index, chunk)| {
        let through = chunk.last().expect("non-empty page").seq;
        let page = sealed(after, through, index + 1 < chunks.len(), chunk.to_vec());
        after = through;
        semio_framework_pack_json::to_json_string(&page)
    }).collect()
}

fn item(page_json: &str) -> HomeTransientMutation {
    ApplyDirectoryPage { page_json: page_json.to_owned() }.into()
}

/// 📬️ Publishes one page item through the PRODUCTION transient preparation (the bounded one-item factory the Home surfaces
/// install), exactly as the transient store does: preflight, begin over the captured root, advance with the pump's page grant.
fn publish(root: &Arc<HomeTransient>, mutation: HomeTransientMutation) -> (Arc<HomeTransient>, store::ArtifactStoreOneItemFootprint) {
    let factory = semio_framework_plugin::bounded_transient_preparation_factory::<HomeTransient, HomeTransientMutation>();
    let footprint = factory.preflight(&mutation).expect("one page is one admissible item");
    assert!(footprint.is_admissible());
    let request = store::ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1),
        generation: semio_framework_job::Generation(0),
        base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(root.clone())),
        mutation,
    };
    let Ok(mut preparation) = factory.begin(request) else { panic!("the bounded factory begins every admitted page") };
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: semio_framework_plugin::app::TYPED_OPERATION_RESULT_PAGE_BYTES };
    assert!(matches!(preparation.advance(grant).expect("the page folds"), store::ArtifactStoreOneItemPreparationStep::Prepared(_)));
    let prepared = preparation.take_prepared().expect("prepared root");
    (prepared.next_root, footprint)
}
//#endregion 🧫️Pages

//#region 🔖️Bootstrap
/// 📏️ The P1 law (14c): a directory of 10 000 spaces bootstraps from the origin page by page, every page ONE admissible item
/// whose bytes are the page's own — never the projection's — and the projection lists every space in id order. The config
/// history is not involved at all: the only thing a page produces is its transient item.
#[test]
fn a_ten_thousand_space_directory_bootstraps_page_by_page_under_the_one_item_bound() {
    const SPACES: usize = 10_000;
    let pages = directory_pages(SPACES);
    assert!(pages.len() >= SPACES * 2 / store::os_directory::DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS);
    let mut root = Arc::new(HomeTransient::default());
    let mut largest_item = 0;
    for page_json in &pages {
        assert!(page_json.len() <= store::os_directory::DIRECTORY_EVENT_PAGE_MAX_BYTES, "the hub pages the directory within its page bound");
        let (next, footprint) = publish(&root, item(page_json));
        assert!(footprint.retained_bytes <= page_json.len() + 64, "an item costs its page, never the projection it folds into");
        largest_item = largest_item.max(footprint.retained_bytes);
        root = next;
    }
    assert!(largest_item <= store::os_directory::DIRECTORY_EVENT_PAGE_MAX_BYTES + 64);
    assert!(largest_item < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES / 8, "a page stays far inside the one-item bound whatever the directory size");
    let directory = root.directory();
    assert_eq!(directory.space_count(), SPACES);
    assert_eq!(directory.cursor(), 2 * SPACES as u64);
    let ids: Vec<&str> = directory.spaces().map(|space| space.view.id.as_str()).collect();
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]), "rows list in id order");
    assert!(directory.spaces().all(|space| space.members.len() == 1 && space.view.member_count == 1), "every member join folded onto its row");
    assert_eq!(directory.receipt().expect("frontier receipt").through_seq_inclusive, 2 * SPACES as u64);
}

/// 📏️ The largest page the Home route admits (the retained wire budget) is one admissible transient item: its encoded
/// size is its page's, independent of any projection.
#[test]
fn a_page_item_never_exceeds_the_one_item_bound() {
    let factory = semio_framework_plugin::bounded_transient_preparation_factory::<HomeTransient, HomeTransientMutation>();
    let largest = item(&"x".repeat(crate::editor::home::config::HOME_DIRECTORY_PAGE_BYTES));
    let footprint = factory.preflight(&largest).expect("the largest admitted page is one item");
    assert!(footprint.is_admissible());
    assert!(footprint.retained_bytes <= crate::editor::home::config::HOME_DIRECTORY_PAGE_BYTES + 64);
    assert!(footprint.retained_bytes <= store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
}
//#endregion 🔖️Bootstrap

//#region 🔖️Snapshots
/// 🧵️ Coordinator check (1): a retained rename/share/remove job reads the root it captured at dispatch. A page folded
/// meanwhile publishes a NEW root — the job's root and the one row it holds never change, and only the rows the page
/// touches are copied (every other row stays shared between the two roots).
#[test]
fn a_captured_root_never_sees_a_later_page_and_untouched_rows_stay_shared() {
    let first = sealed(0, 4, true, vec![created(1, "space-a", "One"), created(2, "space-b", "Beta"), member(3, "space-a"), member(4, "space-b")]);
    let captured = Arc::new(HomeTransient::with_directory(HomeTransient::default().directory().fold_page(&first)));
    let held_row = captured.directory().space_row("space-a").expect("the job holds one row");
    let renamed = sealed(4, 5, false, vec![event(5, store::os_directory::DirectoryEventBody::SpaceRenamed { space_id: "space-a".into(), name: "Two".into() })]);
    let (published, _) = publish(&captured, item(&semio_framework_pack_json::to_json_string(&renamed)));
    assert_eq!(captured.directory().space("space-a").expect("captured row").view.name, "One", "the captured root is immutable");
    assert_eq!(held_row.view.name, "One", "the held row belongs to the captured root");
    assert_eq!(published.directory().space("space-a").expect("published row").view.name, "Two");
    let untouched = |root: &HomeTransient| root.directory().space_row("space-b").expect("untouched row");
    assert!(Arc::ptr_eq(&untouched(&captured), &untouched(&published)), "a page copies only the rows it touches");
}

/// 🔁️ Coordinator check (2): dropping the transient (a window close or a reload) and re-bootstrapping from the origin
/// (`after = 0`) rebuilds exactly the same projection page by page, one item per page and nothing else; an origin replay
/// over a live root rebuilds it too, so nothing folded before the origin survives.
#[test]
fn a_dropped_transient_rebootstraps_from_the_origin_without_history() {
    let pages = directory_pages(300);
    let bootstrap = || pages.iter().fold(Arc::new(HomeTransient::default()), |root, page_json| publish(&root, item(page_json)).0);
    let first = bootstrap();
    let after_reload = bootstrap();
    assert_eq!(first, after_reload, "a fresh transient rebuilt from the origin equals the one it replaces");
    let origin_replay = publish(&first, item(&pages[0])).0;
    let from_nothing = publish(&Arc::new(HomeTransient::default()), item(&pages[0])).0;
    assert_eq!(origin_replay, from_nothing, "an origin page over a live root keeps nothing folded before it");
    let answer = crate::editor::home::commands::apply_directory_event_page::directory_page_answer(pages.last().expect("last page"), first.directory()).expect("replaying the held frontier");
    assert!(answer.item.is_none(), "an already-held frontier produces no item at all");
}
//#endregion 🔖️Snapshots

//#region 🔖️Vectors
/// 🧫️ Every committed vector of the lane, read through the production dispatch bridge the language-neutral case drives:
/// the applied snapshot is the committed after-snapshot, the produced delta is the committed 🔺️diff, the diagnostics are the
/// declared outcome's, an `applied` vector moves the snapshot and a `no-op`/`rejected` vector does not.
#[test]
fn the_committed_vectors_hold_through_the_production_bridge() {
    macro_rules! vector {
        ($name:literal) => {
            (
                $name,
                include_str!(concat!("../../🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/⬅️before/🔣️.json")),
                include_str!(concat!("../../🧫️fixtures/📬️apply-directory/", $name, "/🦠️mutation/🔣️.json")),
                include_str!(concat!("../../🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/➡️after/🔣️.json")),
                include_str!(concat!("../../🧫️fixtures/📬️apply-directory/", $name, "/🔺️diff/🔣️.json")),
                include_str!(concat!("../../🧫️fixtures/📬️apply-directory/", $name, "/🎯️outcome/🔣️.json")),
            )
        };
    }
    for (name, before, mutation, after, diff, outcome) in [vector!("✅️apply"), vector!("🟰️apply"), vector!("🚫️apply")] {
        let report = semio_framework_pack_json::parse(&home_transient_mutation_report_json(before, mutation, after).expect("bridge report"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("report JSON");
        let outcome = semio_framework_pack_json::parse(outcome, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome JSON");
        assert_eq!(report["snapshot"], report["expectedSnapshot"], "{name}: the applied snapshot is the committed after-snapshot");
        assert_eq!(report["diff"], semio_framework_pack_json::parse(diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("diff JSON"), "{name}: the delta is the committed diff");
        let codes = |messages: &semio_framework_pack_json::Value| messages.as_array().expect("messages").iter().map(|message| message["code"].as_str().expect("code").to_owned()).collect::<Vec<_>>();
        assert_eq!(codes(&report["messages"]), codes(&outcome["messages"]), "{name}: the diagnostics are the declared outcome's");
        assert_eq!(report["snapshot"] != report["base"], outcome["status"] == "applied", "{name}: only an applied vector moves the snapshot");
        assert!(report["inverseSteps"].as_array().expect("inverse steps").is_empty(), "{name}: a derived page is never undone");
    }
}
//#endregion 🔖️Vectors

//#region 🔖️Codec
/// 📇️ The transient's JSON wire is the framework's `DirectoryReadModel` under the resume authority: it round-trips every
/// document and indexed document of the language-neutral fixture, and corruption is explicit rather than defaulted.
#[test]
fn the_projection_wire_round_trips_documents_and_rejects_corruption() {
    let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/📇️projection-wire-v1/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral projection fixture");
    let wire = semio_framework_pack_json::json!({ "sessionBindingSha256": "", "authorizationGeneration": 0, "receiptSha256": "", "directory": fixture["wire"].clone() });
    let transient: HomeTransient = semio_framework_pack_json::from_json_str(&wire.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture projection");
    let document_ids: Vec<&str> = transient.directory().spaces().flat_map(|space| space.documents.iter().map(|document| document.document_id.as_str())).collect();
    assert_eq!(document_ids, vec!["document-雪"]);
    let encoded: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&semio_framework_pack_json::to_json_string(&transient), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("encoded projection JSON");
    assert_eq!(encoded, wire);
    for malformed in fixture["malformed"].as_array().expect("malformed cases") {
        assert!(semio_framework_pack_json::parse(malformed.as_str().expect("malformed text"), semio_framework_pack_json::JsonMemberPolicy::Reject).ok().is_none_or(|directory| semio_framework_pack_json::from_json_str::<HomeTransient>(&semio_framework_pack_json::json!({ "sessionBindingSha256": "", "authorizationGeneration": 0, "receiptSha256": "", "directory": directory }).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err()));
    }
    let half_bound = semio_framework_pack_json::json!({ "sessionBindingSha256": BINDING, "authorizationGeneration": 0, "receiptSha256": "", "directory": fixture["wire"].clone() });
    assert!(semio_framework_pack_json::from_json_str::<HomeTransient>(&half_bound.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "a partial resume authority is refused");
    store::os_store::test_support::assert_dsl_round_trip(&transient);
    let bytes = store::ArtifactPack::encode_pack(&transient);
    assert_eq!(<HomeTransient as store::ArtifactPack>::decode_pack(&bytes).expect("pack round trip"), transient);
}

/// 🔤️ The one verb round-trips its text line and its binary op.
#[test]
fn the_page_verb_round_trips_text_and_binary() {
    let mutation = item(&semio_framework_pack_json::to_json_string(&sealed(0, 1, false, vec![created(1, "space-a", "One")])));
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
    let bytes = protocol::OpBinary::encode_op(&mutation).expect("binary op");
    assert_eq!(<HomeTransientMutation as protocol::OpBinary>::decode_op(&bytes).expect("binary round trip"), mutation);
    assert!(mutation.inverse(&HomeTransient::default()).expect("valid retained mutation inverse fixture").is_empty(), "a derived projection page is never undone");
    let refused = item("{").diff(&HomeTransient::default());
    assert!(refused.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == semio_framework_diagnostic::Severity::Fatal));
    assert_eq!(refused.diff(), &HomeTransientDiff::default());
}

/// 🧹️ A displaced 10 000-space root retires in page grants measured by the projection itself and never encodes it.
#[test]
fn a_displaced_root_retires_in_page_grants() {
    let root = directory_pages(10_000).iter().fold(Arc::new(HomeTransient::default()), |root, page_json| publish(&root, item(page_json)).0);
    let estimate = root.directory().retained_bytes();
    let mut retirement = store::SnapshotRetirementFactory::retire(&HomeTransientRetirementFactory, root);
    let grant = semio_framework_plugin::app::TYPED_OPERATION_RESULT_PAGE_BYTES;
    let mut released = 0;
    loop {
        match retirement.close_step(1, grant).expect("retirement step") {
            store::SnapshotRetirementStep::Pending { released_bytes, .. } => {
                assert!(released_bytes <= grant);
                released += released_bytes;
            }
            store::SnapshotRetirementStep::Complete => break,
            store::SnapshotRetirementStep::Blocked => panic!("a transient root never blocks its own retirement"),
        }
    }
    assert!(retirement.terminal_is_empty());
    assert_eq!(released, estimate);
}
//#endregion 🔖️Codec
