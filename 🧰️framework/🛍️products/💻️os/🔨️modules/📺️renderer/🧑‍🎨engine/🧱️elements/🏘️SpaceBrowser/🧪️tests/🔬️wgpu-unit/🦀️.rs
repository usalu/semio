//! 🧪️ Laws of the wgpu spaces surface (ticket 26/09/18, slice WG6 / G8 item WG-6).
//!
//! 🤝️ Every table is read from the SAME fixture the React twin's vitest suite reads
//! (`📇️directory/🏘️spaces/🔣️.json`). The canonical command field order is checked against the
//! DECLARATION order of `DirectoryCommand` in `📇️directory/🧬️schema/🦀️.rs` — read off disk, the way
//! U2's tablet-breakpoint gate reads its constants — because the `#[value(tag = "kind",
//! rename_all_fields = "camelCase")]` derive serializes fields in declaration order, so that order
//! IS the canonical order the sealer re-serializes and byte-compares.

use super::*;
use semio_framework_os_kernel::os_directory::{MemberSpaceViewV1, PublicSpaceViewV1};

const FIXTURE: &str = include_str!("../../../../../../📇️directory/🏘️spaces/🔣️.json");
const DIRECTORY_SCHEMA_SOURCE: &str = include_str!("../../../../../../📇️directory/🧬️schema/🦀️.rs");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("the shared spaces fixture is valid JSON")
}

fn member_space(id: &str, name: &str, role: DirectorySpaceRole, updated_at_ms: i64) -> MemberSpaceViewV1 {
    MemberSpaceViewV1 {
        id: id.to_string(),
        name: name.to_string(),
        kind: DirectorySpaceKind::Studio,
        visibility: DirectorySpaceVisibility::Private,
        owner_user_id: "owner".to_string(),
        role,
        member_count: 2,
        document_count: 3,
        active_connections: 1,
        created_at_ms: 1,
        updated_at_ms,
    }
}

fn public_space(id: &str, name: &str, updated_at_ms: i64) -> PublicSpaceViewV1 {
    PublicSpaceViewV1 {
        id: id.to_string(),
        name: name.to_string(),
        kind: DirectorySpaceKind::Archive,
        visibility: DirectorySpaceVisibility::Public,
        member_count: 9,
        document_count: 4,
        created_at_ms: 1,
        updated_at_ms,
    }
}

fn member(user_id: &str, display_name: &str, email: &str, role: DirectorySpaceRole, owner: bool) -> DirectorySpaceAdministrationMemberRowV1 {
    DirectorySpaceAdministrationMemberRowV1 { user_id: user_id.to_string(), email: email.to_string(), display_name: display_name.to_string(), role, owner }
}

#[test]
fn the_fixture_names_the_routes_and_bounds_this_module_pins() {
    let fixture = fixture();
    assert_eq!(fixture["commandsPath"], DIRECTORY_COMMANDS_PATH_V1);
    assert_eq!(fixture["spacesPath"], DIRECTORY_SPACES_PATH_V1);
    assert_eq!(fixture["inviteLinkFragment"], INVITE_LINK_FRAGMENT_V1);
    let ttls: Vec<u64> = fixture["inviteTtlChoicesSecs"].as_array().expect("ttls").iter().map(|value| value.as_u64().expect("ttl")).collect();
    assert_eq!(ttls, INVITE_TTL_CHOICES_SECS_V1.to_vec());
}

#[test]
fn the_access_order_is_the_shared_ones_and_it_is_the_enums_own_ordering() {
    let order: Vec<String> = fixture()["accessOrder"].as_array().expect("accessOrder").iter().map(|value| value.as_str().expect("access").to_string()).collect();
    let mut declared = vec![SpaceAccess::Public, SpaceAccess::Author, SpaceAccess::Member];
    declared.sort();
    assert_eq!(declared.iter().map(|access| access.as_str().to_string()).collect::<Vec<String>>(), order);
}

#[test]
fn every_command_builder_emits_the_shared_canonical_field_order() {
    let orders = fixture()["commandFieldOrder"].clone();
    for (variant, command) in [("create-space", "CreateSpace"), ("create-invite", "CreateInvite"), ("archive-space", "ArchiveSpace")] {
        let expected: Vec<String> = orders[variant].as_array().expect("field order").iter().map(|value| value.as_str().expect("field").to_string()).collect();
        let needle = format!("    {command} {{ ");
        let line = DIRECTORY_SCHEMA_SOURCE.lines().find(|line| line.starts_with(&needle)).unwrap_or_else(|| panic!("{command} is no longer declared in the directory schema"));
        let fields: Vec<String> = line
            .trim_start()
            .trim_start_matches(command)
            .trim()
            .trim_start_matches('{')
            .trim_end_matches("},")
            .split(',')
            .filter_map(|part| part.split(':').next())
            .map(|field| camel_case(field.trim()))
            .filter(|field| !field.is_empty())
            .collect();
        assert_eq!(std::iter::once("kind".to_string()).chain(fields).collect::<Vec<String>>(), expected, "{variant}");
    }
}

fn camel_case(snake: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for character in snake.chars() {
        if character == '_' {
            upper = true;
            continue;
        }
        out.push(if upper { character.to_ascii_uppercase() } else { character });
        upper = false;
    }
    out
}

#[test]
fn my_spaces_come_first_then_shared_then_public_each_newest_first_with_an_id_tie_break() {
    let entries = vec![
        DirectorySpaceListEntryV1::Public { space: public_space("p1", "Archive", 900) },
        DirectorySpaceListEntryV1::Member { space: member_space("m1", "Shared", DirectorySpaceRole::Spectator, 500) },
        DirectorySpaceListEntryV1::Author { space: member_space("a2", "Mine two", DirectorySpaceRole::Author, 100) },
        DirectorySpaceListEntryV1::Author { space: member_space("a1", "Mine one", DirectorySpaceRole::Author, 100) },
    ];
    let rows = space_rows(&entries);
    assert_eq!(rows.iter().map(|row| row.id.as_str()).collect::<Vec<&str>>(), vec!["a1", "a2", "m1", "p1"]);
    assert_eq!(rows[0].access, SpaceAccess::Author);
    assert_eq!(rows[3].access, SpaceAccess::Public);
    assert_eq!(rows[3].role, None, "a public space the caller is not a member of carries no role");
    assert_eq!(rows[3].active_connections, 0, "live activity is structurally absent from a public view");
}

#[test]
fn a_member_entry_whose_role_is_author_is_writable_but_never_invitable() {
    let entries = vec![DirectorySpaceListEntryV1::Member { space: member_space("m1", "Shared", DirectorySpaceRole::Author, 1) }];
    let rows = space_rows(&entries);
    assert!(space_row_writable(&rows[0]));
    assert!(!space_row_invitable(&rows[0]), "membership alone is never enough to issue an invitation");
    let public = space_rows(&[DirectorySpaceListEntryV1::Public { space: public_space("p1", "Archive", 1) }]);
    assert!(!space_row_writable(&public[0]));
    assert!(!space_row_invitable(&public[0]));
}

#[test]
fn the_filter_is_case_insensitive_over_name_and_id_and_an_empty_query_keeps_every_row() {
    let rows = space_rows(&[
        DirectorySpaceListEntryV1::Author { space: member_space("atelier-7", "Concrete Forest", DirectorySpaceRole::Author, 2) },
        DirectorySpaceListEntryV1::Author { space: member_space("studio-1", "Bridge", DirectorySpaceRole::Author, 1) },
    ]);
    assert_eq!(filter_space_rows(&rows, "").len(), 2);
    assert_eq!(filter_space_rows(&rows, "   ").len(), 2);
    assert_eq!(filter_space_rows(&rows, "FOREST").len(), 1);
    assert_eq!(filter_space_rows(&rows, "studio").len(), 1, "the id is searchable too");
    assert_eq!(filter_space_rows(&rows, "nothing").len(), 0);
}

#[test]
fn the_roster_puts_owners_first_then_authors_then_spectators_each_ascending_by_user_id() {
    let members = vec![
        member("u3", "", "zoe@example.org", DirectorySpaceRole::Spectator, false),
        member("u2", "Bo", "bo@example.org", DirectorySpaceRole::Author, false),
        member("u1", "Ada", "ada@example.org", DirectorySpaceRole::Author, true),
        member("u0", "Cy", "cy@example.org", DirectorySpaceRole::Author, false),
    ];
    let joined = space_member_presence(&members, &["u2".to_string()]);
    assert_eq!(joined.iter().map(|row| row.user_id.as_str()).collect::<Vec<&str>>(), vec!["u1", "u0", "u2", "u3"]);
    assert!(joined[2].online, "presence decorates the roster, it never reorders it");
    assert!(!joined[0].online);
    assert_eq!(joined[3].display_name, "zoe@example.org", "an empty display name falls back to the email, never to a blank row");
}

#[test]
fn a_presence_tick_for_a_user_the_directory_does_not_know_adds_nobody() {
    let members = vec![member("u1", "Ada", "ada@example.org", DirectorySpaceRole::Author, true)];
    let joined = space_member_presence(&members, &["stranger".to_string(), "u1".to_string()]);
    assert_eq!(joined.len(), 1);
    assert!(joined[0].online);
}

#[test]
fn every_command_builder_refuses_an_out_of_bounds_field_locally() {
    assert!(create_space_command("  Concrete Forest  ", DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).is_some());
    assert_eq!(create_space_command("   ", DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private), None);
    assert_eq!(create_space_command(&"n".repeat(SPACE_NAME_MAX_BYTES + 1), DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private), None);
    assert_eq!(create_space_command("bad\u{0007}name", DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private), None);
    assert_eq!(create_invite_command("space-1", DirectorySpaceRole::Spectator, 0), None, "a zero lifetime is never an invitation");
    assert!(create_invite_command("space-1", DirectorySpaceRole::Spectator, INVITE_TTL_CHOICES_SECS_V1[0]).is_some());
    assert_eq!(archive_space_command(""), None);
    assert!(archive_space_command("space-1").is_some());
}

#[test]
fn the_create_space_command_trims_the_name_it_seals() {
    let Some(DirectoryCommand::CreateSpace { name, space_kind, visibility }) = create_space_command("  Concrete Forest  ", DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Public) else {
        panic!("a valid name builds a create-space command");
    };
    assert_eq!(name, "Concrete Forest");
    assert_eq!(space_kind, DirectorySpaceKind::Atelier);
    assert_eq!(visibility, DirectorySpaceVisibility::Public);
}

#[test]
fn every_invite_token_vector_parses_exactly_as_the_react_twin_parses_it() {
    let table = fixture()["inviteTokens"].as_array().expect("inviteTokens").clone();
    assert_eq!(table.len(), 7);
    for row in &table {
        let typed = row["typed"].as_str().expect("typed");
        let expected = row["token"].as_str().map(str::to_string);
        assert_eq!(parse_invite_token(typed), expected, "typed {typed:?}");
    }
}

#[test]
fn an_invitation_link_round_trips_through_its_own_parser() {
    let token = "abcDEF-012_~.";
    let link = invite_link("https://hub.example.org", token).expect("a valid capability renders a link");
    assert_eq!(link, format!("https://hub.example.org/{INVITE_LINK_FRAGMENT_V1}{token}"));
    assert_eq!(parse_invite_token(&link).as_deref(), Some(token));
    assert_eq!(invite_link("https://hub.example.org", "abc/def"), None);
    assert_eq!(invite_redeem_path(token), Some(format!("/directory/invites/{token}/redeem")));
    assert_eq!(invite_redeem_path(""), None);
}

#[test]
fn every_redemption_status_maps_to_the_shared_closed_class() {
    let table = fixture()["redemptionStatusCodes"].as_array().expect("redemptionStatusCodes").clone();
    assert_eq!(table.len(), 9);
    for row in &table {
        let status = row["status"].as_u64().expect("status") as u16;
        assert_eq!(invite_redemption_error_from_status(status).as_str(), row["code"].as_str().expect("code"), "status {status}");
    }
}

#[test]
fn rows_stay_usable_in_every_phase_except_an_empty_first_load() {
    assert!(!space_browser_rows_usable(SpaceBrowserPhase::Loading, 3));
    assert!(!space_browser_rows_usable(SpaceBrowserPhase::Ready, 0));
    assert!(space_browser_rows_usable(SpaceBrowserPhase::Stale, 3), "a hub that stopped answering must not empty a list the human was reading");
    assert!(space_browser_rows_usable(SpaceBrowserPhase::Failed, 3));
    assert!(space_browser_rows_usable(SpaceBrowserPhase::Submitting, 3));
}

#[test]
fn every_phase_and_every_denial_class_reads_differently_in_the_two_languages() {
    for phase in [SpaceBrowserPhase::Loading, SpaceBrowserPhase::Ready, SpaceBrowserPhase::Stale, SpaceBrowserPhase::Submitting, SpaceBrowserPhase::Failed] {
        assert_ne!(phase.text(Locale::En), phase.text(Locale::De), "{phase:?} was left untranslated");
    }
    for code in [
        InviteRedemptionErrorCode::InvalidInvite,
        InviteRedemptionErrorCode::ExpiredInvite,
        InviteRedemptionErrorCode::AlreadyMember,
        InviteRedemptionErrorCode::Unauthorized,
        InviteRedemptionErrorCode::Unreachable,
        InviteRedemptionErrorCode::HubRefused,
        InviteRedemptionErrorCode::Cancelled,
    ] {
        assert_ne!(code.text(Locale::En), code.text(Locale::De), "{code:?} was left untranslated");
        assert!(!code.as_str().is_empty());
    }
}

#[test]
fn a_row_summary_names_the_access_class_the_member_count_and_who_is_here() {
    let rows = space_rows(&[DirectorySpaceListEntryV1::Author { space: member_space("a1", "Mine", DirectorySpaceRole::Author, 1) }]);
    let summary = space_row_summary(&rows[0], Locale::En);
    assert!(summary.contains('2'), "{summary}");
    assert!(summary.contains('1'), "{summary}");
    assert_ne!(summary, space_row_summary(&rows[0], Locale::De));
}
