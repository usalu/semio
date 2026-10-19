//! 🧪️ Presence colour leases, room readers, pages, the per-address budget and the state frame, on the presence crate's private rooms. See [`super`].

use super::*;
use semio_framework_server_contract::{OpaqueJson, Scope};
use std::collections::BTreeSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn loopback() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4242))
}

fn caller() -> ClientKey {
    ClientKey::of(loopback().ip())
}

fn address(last: u8) -> ClientKey {
    ClientKey::of(IpAddr::V4(Ipv4Addr::new(203, 0, 113, last)))
}

//#region 🔖️Presence
#[test]
fn a_colour_lease_takes_the_lowest_free_slot() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
    assert_eq!(presence.acquire_colour("space-2", "dave"), 0);
}

#[test]
fn a_colour_is_freed_only_on_the_last_disconnect() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), Some(0));
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), None);
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
}

#[test]
fn joining_and_leaving_maintains_the_roster() {
    let presence = Presence::new();
    presence.join("space-1", "bob", "canvas");
    presence.join("space-1", "alice", "canvas");
    presence.publish_peer("space-1", "alice", vec![7]);
    let roster = presence.roster("space-1");
    assert_eq!(roster.iter().map(|session| session.actor.as_str()).collect::<Vec<_>>(), vec!["alice", "bob"]);
    assert_eq!(roster[0].peer, Some(vec![7]));
    presence.leave("space-1", "alice");
    assert_eq!(presence.roster("space-1").len(), 1);
    assert_eq!(presence.colour_of("space-1", "alice"), None);
}
//#endregion 🔖️Presence

//#region 🔖️PresenceRoom
const ROOM: &str = "presence:s";

pub(super) fn entry(session: &str, state: OpaqueJson) -> PresenceEntry {
    PresenceEntry { session: session.into(), colour: 0, surface: "home".into(), state }
}

pub(super) fn rooms(departures_kept: usize) -> Arc<PresenceRooms> {
    Arc::new(PresenceRooms::new(&PresenceSettings { departures_kept, max_bytes_per_second: None, ..PresenceSettings::default() }, Instant::now()))
}

/// 🧾️ What a page carries, read back the way a client reads its frame: the sessions with their
/// states in order, the sessions that left, and whether it replaces.
fn carried(page: &Page) -> (Vec<(String, OpaqueJson)>, Vec<String>, bool) {
    let PresenceFrame::Batch { entries, left } = serde_json::from_str(&page.batch()).expect("a page is a frame") else { panic!("a batch") };
    (entries.into_iter().map(|entry| (entry.session, entry.state)).collect(), left, page.replace)
}

pub(super) fn sessions(page: &Page) -> Vec<String> {
    carried(page).0.into_iter().map(|(session, _)| session).collect()
}

#[test]
fn a_reader_is_sent_the_newest_state_of_each_session_that_changed_and_who_left() {
    let rooms = rooms(8);
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    rooms.join(ROOM, &entry("b", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, Some("b"), caller());
    assert!(reader.fresh() && reader.has_unsent());
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("b".into(), OpaqueJson::Null), ("a".into(), OpaqueJson::Null)], vec![], true), "the first page replaces and is led by the reader's own session");
    assert!(!reader.fresh() && !reader.has_unsent());
    assert!(reader.page(4096, Instant::now()).is_empty(), "a room that did not change has nothing to send");

    for n in 0..5 {
        rooms.update(ROOM, &entry("a", serde_json::json!({ "n": n })));
    }
    rooms.update(ROOM, &entry("nobody", serde_json::json!(1)));
    assert!(reader.has_unsent());
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("a".into(), serde_json::json!({ "n": 4 }))], vec![], false), "five states are one entry: the newest");
    assert!(reader.page(4096, Instant::now()).is_empty());

    rooms.update(ROOM, &entry("b", serde_json::json!({ "x": 1 })));
    rooms.leave(ROOM, "b");
    rooms.leave(ROOM, "b");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![], vec!["b".into()], false), "a session that left is not sent, its departure is");
    assert_eq!(rooms.roster(ROOM), [entry("a", serde_json::json!({ "n": 4 }))]);

    let mut late = rooms.read(ROOM, None, caller());
    assert_eq!(carried(&late.page(4096, Instant::now())), (vec![("a".into(), serde_json::json!({ "n": 4 }))], vec![], true), "a later reader starts with the roster and no departure");
}

#[test]
fn a_page_is_bounded_and_a_round_reaches_every_session_before_any_twice() {
    let rooms = rooms(8);
    let names: Vec<String> = (0..20).map(|n| format!("s{n:02}")).collect();
    for name in &names {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let one = serde_json::to_string(&entry("s00", serde_json::json!({ "round": 0 }))).expect("an entry").len();
    let budget = one * 5 + 4;
    let mut reader = rooms.read(ROOM, Some("s07"), caller());
    let mut roster = Vec::new();
    while reader.has_unsent() {
        assert!(reader.arriving(), "the roster is still on its way");
        let page = reader.page(budget, Instant::now());
        assert!(page.bytes() <= budget, "{} bytes in a page of {budget}", page.bytes());
        roster.extend(sessions(&page));
    }
    assert_eq!(roster.first().map(String::as_str), Some("s07"));
    roster.sort();
    assert_eq!(roster, names, "the roster arrives whole, every session once");

    let mut sent = Vec::new();
    for round in 1..=4 {
        for name in &names {
            rooms.update(ROOM, &entry(name, serde_json::json!({ "round": round })));
        }
        let page = reader.page(budget, Instant::now());
        assert!(page.bytes() <= budget && !page.replace);
        sent.extend(sessions(&page));
    }
    assert_eq!(sent.len(), 20, "five entries fit a page");
    sent.sort();
    assert_eq!(sent, names, "with every session changing before every page, four pages still reach all twenty once");

    rooms.update(ROOM, &entry("s03", serde_json::json!("x".repeat(budget * 2))));
    let mut fresh = rooms.read(ROOM, None, caller());
    let mut large = 0;
    while fresh.has_unsent() {
        let page = fresh.page(budget, Instant::now());
        if page.bytes() > budget {
            assert_eq!(sessions(&page), ["s03"], "an entry larger than a page travels alone");
            large += 1;
        }
    }
    assert_eq!(large, 1);
}

#[test]
fn a_vacated_seat_is_taken_again_and_readers_keep_their_place() {
    let rooms = rooms(4096);
    let names: Vec<String> = (0..600).map(|n| format!("s{n:03}")).collect();
    for name in &names {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let mut readers: Vec<RoomReader> = (0..8).map(|_| rooms.read(ROOM, None, caller())).collect();
    for reader in &mut readers {
        let mut roster = 0;
        while reader.has_unsent() {
            roster += sessions(&reader.page(4096, Instant::now())).len();
        }
        assert_eq!(roster, names.len());
    }
    for name in names.iter().skip(1).step_by(2) {
        rooms.leave(ROOM, name);
    }
    for name in ["late-a", "late-b"] {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    for reader in &mut readers {
        let (entries, left, replace) = carried(&reader.page(1 << 20, Instant::now()));
        assert_eq!((entries.iter().map(|(session, _)| session.as_str()).collect::<BTreeSet<_>>(), left.len(), replace), (BTreeSet::from(["late-a", "late-b"]), 300, false));
    }
    for round in 0..50 {
        let changing = &names[(round * 7 % 300) * 2];
        rooms.update(ROOM, &entry(changing, serde_json::json!(round)));
        for reader in &mut readers {
            assert_eq!(carried(&reader.page(8192, Instant::now())), (vec![(changing.clone(), serde_json::json!(round))], vec![], false));
        }
    }
    assert_eq!(rooms.roster(ROOM).len(), 302);
}

#[test]
fn the_tournament_finds_the_next_changed_seat_like_a_walk_of_the_seats_does() {
    let mut room = Room::default();
    let mut random = 0x9E37_79B9_7F4A_7C15_u64;
    let mut draw = move || {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        random
    };
    for step in 0..3000 {
        let session = format!("s{}", draw() % 97);
        if draw() % 4 == 0 {
            room.vacate(&session, 8);
        } else {
            room.seat(Member { session, wire: "{}".into() });
        }
        if step % 40 == 0 {
            let seats = room.seats.len();
            for clock in [0, room.clock / 2, room.clock.saturating_sub(3), room.clock] {
                for from in 0..=seats {
                    assert_eq!(room.next_changed(from, clock), (from..seats).find(|seat| room.changed[seats + seat] > clock), "from seat {from} after clock {clock} at step {step}");
                }
            }
        }
    }
    let seats = room.seats.len();
    assert!(seats.is_power_of_two() && seats <= 128, "97 sessions never need more than 128 seats, the room has {seats}");
    assert!(room.seats.iter().enumerate().all(|(seat, member)| member.is_some() == (room.changed[seats + seat] > 0)), "exactly the taken seats hold a clock");
    assert_eq!(room.seated.len(), room.seats.iter().flatten().count());
    assert!(room.seated.iter().all(|(session, seat)| room.seats[*seat].as_ref().is_some_and(|member| &member.session == session)));
}

#[test]
fn all_rooms_together_send_their_budget_and_readers_wait_their_turn() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
        rooms.join("presence:other", &entry(name, state.clone()));
    }
    let one = serde_json::to_string(&entry("a", state)).expect("an entry").len();
    let mut readers: Vec<RoomReader> = (0..4).map(|reader| rooms.read(if reader % 2 == 0 { ROOM } else { "presence:other" }, None, caller())).collect();
    let mut sent = 0;
    let mut round = |at: Duration, sent: &mut usize| {
        let before = *sent;
        for reader in &mut readers {
            let page = reader.page(4096, started + at);
            assert!(page.is_empty() || reader.has_unsent() || page.bytes() >= 6 * one, "a page is whole or its reader knows it is behind");
            *sent += page.bytes();
        }
        *sent - before
    };
    let at_once = round(Duration::ZERO, &mut sent);
    assert!(at_once >= 1000 && at_once < 1000 + 6 * one + 5, "a quarter of a second's worth goes at once and at most one page more: {at_once} bytes");
    assert_eq!(round(Duration::ZERO, &mut sent), 0, "the budget is spent: every reader waits, in every room");
    assert_eq!(round(Duration::from_millis(100), &mut sent), 0, "and waits until what was sent beyond the budget has flowed back");
    let mut at = Duration::from_millis(100);
    while sent < 4 * (6 * one + 5) {
        at += Duration::from_millis(100);
        round(at, &mut sent);
        let allowed = 1000 + 4000 * at.as_millis() as usize / 1000 + 6 * one + 5;
        assert!(sent <= allowed, "{sent} bytes after {at:?}, the budget allows {allowed}");
        assert!(at < Duration::from_secs(10), "every reader is sent its roster in the end");
    }
    assert!(at >= Duration::from_millis(1500), "four rosters of {} bytes take their time at 4000 bytes a second, not {at:?}", 6 * one + 5);

}

#[test]
fn a_joining_member_is_welcomed_at_once_and_its_welcome_counts_against_the_budget() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
    }
    let roster = 6 * serde_json::to_string(&entry("a", state)).expect("an entry").len() + 5;
    let mut watcher = rooms.read(ROOM, None, caller());
    assert_eq!(watcher.page(4096, started).bytes(), roster, "the quarter second the rooms may send at once is spent, and more");

    let mut member = rooms.read(ROOM, Some("c"), caller());
    let welcome = member.page(4096, started);
    assert_eq!((sessions(&welcome).first().map(String::as_str), welcome.bytes(), welcome.replace), (Some("c"), roster, true), "a joining member is told at once that it joined, whatever the budget");
    rooms.update(ROOM, &entry("a", OpaqueJson::Null));
    assert!(member.page(4096, started).is_empty() && member.has_unsent(), "what follows its welcome waits like everything else");

    let owed_millis = u64::try_from((2 * roster - 1000) * 1000 / 4000).expect("milliseconds");
    assert!(watcher.page(4096, started + Duration::from_millis(owed_millis - 20)).is_empty(), "the welcome was counted: both rosters have to flow back first");
    assert_eq!(sessions(&watcher.page(4096, started + Duration::from_millis(owed_millis + 20))), ["a"]);
}

#[test]
fn a_reader_is_not_held_back_by_what_another_address_took() {
    let started = Instant::now();
    let rooms = Arc::new(PresenceRooms::new(&PresenceSettings { max_bytes_per_second: Some(4000), ..PresenceSettings::default() }, started));
    let state = serde_json::json!("x".repeat(400));
    for name in ["a", "b", "c", "d", "e", "f"] {
        rooms.join(ROOM, &entry(name, state.clone()));
    }
    let roster = 6 * serde_json::to_string(&entry("a", state)).expect("an entry").len() + 5;
    let mut flood: Vec<RoomReader> = (0..4).map(|_| rooms.read(ROOM, None, address(66))).collect();
    let mut hall = rooms.read(ROOM, None, address(1));
    assert_eq!(flood[0].page(4096, started).bytes(), roster, "more than the quarter second the rooms may send at once");
    for reader in &mut flood[1..] {
        assert!(reader.page(4096, started).is_empty() && reader.has_unsent(), "the address that took it waits, through every socket it reads with");
    }
    assert_eq!(hall.page(4096, started).bytes(), roster, "another address reads at once: its even split of the budget is its own");
    let mut sent = 2 * roster;
    let mut at = Duration::ZERO;
    while flood.iter().any(RoomReader::has_unsent) {
        at += Duration::from_millis(100);
        sent += flood.iter_mut().map(|reader| reader.page(4096, started + at).bytes()).sum::<usize>();
        let allowed = 2000 + 4000 * at.as_millis() as usize / 1000 + 2 * roster;
        assert!(sent <= allowed, "{sent} bytes after {at:?}, the budget, half a second's worth and a page per address allow {allowed}");
        assert!(at < Duration::from_secs(10), "every reader is sent its roster in the end");
    }
    assert_eq!(sent, 5 * roster);
    assert!(at >= Duration::from_secs(2), "four rosters of {roster} bytes take one address its time at 4000 bytes a second, not {at:?}");
}

/// 🧮️ Play `addresses` against one budget for `span`: every address asks for `demand` bytes per
/// second through `readers` readers that each try one page of `page` bytes every ten milliseconds;
/// the bytes each address drew in every half second.
fn played(outflow: &mut Outflow, started: Instant, from: Duration, span: Duration, page: usize, addresses: &[(ClientKey, u64, usize)]) -> Vec<Vec<usize>> {
    let mut drawn = vec![vec![0; (span.as_millis() / 500) as usize]; addresses.len()];
    let mut owed = vec![0u64; addresses.len()];
    for step in 0..span.as_millis() as u64 / 10 {
        let now = started + from + Duration::from_millis(step * 10);
        for (index, (client, demand, readers)) in addresses.iter().enumerate() {
            owed[index] = owed[index].saturating_add(demand / 100);
            for _ in 0..*readers {
                if owed[index] >= page as u64 && outflow.open(*client, now) {
                    outflow.draw(*client, page);
                    owed[index] -= page as u64;
                    drawn[index][(step / 50) as usize] += page;
                }
            }
        }
    }
    drawn
}

#[test]
fn the_budget_is_dealt_fairly_among_addresses_however_many_readers_each_has() {
    let started = Instant::now();
    let (hall, flood) = (address(1), address(66));
    let mut outflow = Outflow::new(16_000, started);
    for _ in 0..3 {
        outflow.attend(hall);
    }
    let alone = played(&mut outflow, started, Duration::ZERO, Duration::from_secs(2), 100, &[(hall, 12_000, 3)]);
    assert!(alone[0].iter().all(|half| (5_900..=6_100).contains(half)), "alone, an address is sent what it asks for: {alone:?}");

    for _ in 0..200 {
        outflow.attend(flood);
    }
    let beside = played(&mut outflow, started, Duration::from_secs(2), Duration::from_secs(4), 100, &[(hall, 12_000, 3), (flood, u64::MAX / 2, 200)]);
    assert!(beside[0].iter().all(|half| *half >= 3_900), "from the first moment of a flood the other address keeps an even split of the budget: {beside:?}");
    assert!(beside[0][2..].iter().all(|half| (3_900..=4_300).contains(half)) && beside[1][2..].iter().all(|half| (3_900..=4_300).contains(half)), "two addresses that both ask for more than half are each sent half: {beside:?}");
    assert!((0..8).all(|half| beside[0][half] + beside[1][half] <= 8_000 + 8_000), "the rooms never send more than their budget and half a second's worth: {beside:?}");

    let modest = played(&mut outflow, started, Duration::from_secs(6), Duration::from_secs(4), 100, &[(hall, 5_000, 3), (flood, u64::MAX / 2, 200)]);
    assert!(modest[0].iter().all(|half| (2_400..=2_600).contains(half)), "an address that asks for less than an even split is sent all of it: {modest:?}");
    assert!(modest[1][2..].iter().all(|half| (5_200..=5_700).contains(half)), "and what it leaves goes to the one that asks for more: {modest:?}");

    for _ in 0..200 {
        outflow.depart(flood);
    }
    let after = played(&mut outflow, started, Duration::from_secs(10), Duration::from_secs(2), 100, &[(hall, 12_000, 3)]);
    assert!(after[0][1..].iter().all(|half| (5_900..=6_100).contains(half)), "the flood gone, the address is sent what it asks for again: {after:?}");
    assert_eq!(outflow.shares.len(), 1, "an address without a reader holds no share");
}

#[test]
fn what_modest_addresses_leave_is_split_evenly_among_those_that_ask_for_more() {
    let started = Instant::now();
    let (hall, flood) = (address(1), address(66));
    let homes: Vec<ClientKey> = (0..100u32).map(|home| ClientKey::of(IpAddr::V4(Ipv4Addr::from(0x0A00_0000 + home)))).collect();
    let mut outflow = Outflow::new(64_000, started);
    let mut addresses: Vec<(ClientKey, u64, usize)> = homes.iter().map(|home| (*home, 200, 1)).collect();
    addresses.extend([(hall, 36_000, 9), (flood, u64::MAX / 2, 200)]);
    for (client, _, readers) in &addresses {
        for _ in 0..*readers {
            outflow.attend(*client);
        }
    }
    let drawn = played(&mut outflow, started, Duration::ZERO, Duration::from_secs(6), 100, &addresses);
    for home in &drawn[..100] {
        assert_eq!(home.iter().sum::<usize>(), 1_200, "an address that asks for little is sent all of it, flood or not: {home:?}");
    }
    let (hall, flood) = (&drawn[100], &drawn[101]);
    assert!(hall.iter().all(|half| *half >= 300), "the even split of a hundred and two addresses is the least an address is ever sent: {hall:?}");
    assert!(hall[4..].iter().all(|half| (10_000..=11_500).contains(half)) && flood[4..].iter().all(|half| (10_000..=11_500).contains(half)), "what the hundred leave of 64000 bytes a second — 44000 — is halved between the two that ask for more: hall {hall:?}, flood {flood:?}");
}

#[test]
fn a_reader_that_missed_a_forgotten_departure_starts_over() {
    let rooms = rooms(2);
    for name in ["a", "b", "c", "d", "e"] {
        rooms.join(ROOM, &entry(name, OpaqueJson::Null));
    }
    let mut reader = rooms.read(ROOM, Some("a"), caller());
    assert_eq!(sessions(&reader.page(4096, Instant::now())).len(), 5);
    rooms.leave(ROOM, "b");
    rooms.leave(ROOM, "c");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![], vec!["b".into(), "c".into()], false), "two departures are remembered");
    for name in ["d", "e"] {
        rooms.leave(ROOM, name);
    }
    rooms.join(ROOM, &entry("f", OpaqueJson::Null));
    rooms.leave(ROOM, "f");
    assert_eq!(carried(&reader.page(4096, Instant::now())), (vec![("a".into(), OpaqueJson::Null)], vec![], true), "three departures are one too many: the roster replaces what the reader held");
    assert!(reader.page(4096, Instant::now()).is_empty());
}

#[test]
fn a_room_lives_while_it_has_a_member_or_a_reader() {
    let rooms = rooms(8);
    assert_eq!(rooms.rooms(), 0);
    let watching = rooms.read(ROOM, None, caller());
    assert_eq!(rooms.rooms(), 1, "a watched room exists before anybody joins it");
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    drop(watching);
    assert_eq!(rooms.rooms(), 1);
    let reading = rooms.read(ROOM, Some("a"), caller());
    rooms.leave(ROOM, "a");
    assert_eq!(rooms.rooms(), 1, "its departure is still to be read");
    drop(reading);
    assert_eq!(rooms.rooms(), 0);
    assert!(rooms.roster(ROOM).is_empty());
}

#[test]
fn a_page_is_written_as_the_frame_serde_writes() {
    let rooms = rooms(8);
    let first = entry("a\"1", serde_json::json!({ "cursor": { "x": 0.5 }, "text": "é\n" }));
    rooms.join(ROOM, &first);
    rooms.join(ROOM, &entry("b", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, Some("a\"1"), caller());
    let page = reader.page(4096, Instant::now());
    let roster = vec![first.clone(), entry("b", OpaqueJson::Null)];
    assert_eq!(page.welcome("a\"1", 7), serde_json::to_string(&PresenceFrame::Welcome { session: "a\"1".into(), colour: 7, roster: roster.clone() }).expect("serde"));
    assert_eq!(page.watched("room \"x\""), serde_json::to_string(&PresenceFrame::Watched { scope: Scope("room \"x\"".into()), entries: roster, left: vec![], snapshot: true }).expect("serde"));
    rooms.leave(ROOM, "b");
    rooms.update(ROOM, &first);
    let page = reader.page(4096, Instant::now());
    assert_eq!(page.batch(), serde_json::to_string(&PresenceFrame::Batch { entries: vec![first.clone()], left: vec!["b".into()] }).expect("serde"));
    assert_eq!(page.watched("r"), serde_json::to_string(&PresenceFrame::Watched { scope: Scope("r".into()), entries: vec![first], left: vec!["b".into()], snapshot: false }).expect("serde"));
}

#[tokio::test]
async fn a_reader_waits_for_its_room_to_change() {
    let rooms = rooms(8);
    rooms.join(ROOM, &entry("a", OpaqueJson::Null));
    let mut reader = rooms.read(ROOM, None, caller());
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("a fresh reader has its roster to read");
    reader.page(4096, Instant::now());
    assert!(tokio::time::timeout(Duration::from_millis(60), reader.unsent()).await.is_err(), "nothing changed");
    let writer = Arc::clone(&rooms);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(30)).await;
        writer.update(ROOM, &entry("a", serde_json::json!(1)));
    });
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("the change wakes the reader");
    tokio::time::timeout(Duration::from_secs(5), reader.unsent()).await.expect("and it stays due until it is read");
    assert_eq!(sessions(&reader.page(4096, Instant::now())), ["a"]);
}

#[test]
fn a_state_frame_is_bounded_by_size_rate_and_shape() {
    let settings = PresenceSettings::default();
    let now = Instant::now();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"state","state":{"x":1}}"#, &settings, &mut budget, now), Ok(Some(PresenceFrame::State { state: serde_json::json!({ "x": 1 }) })));
    let large = format!(r#"{{"type":"state","state":"{}"}}"#, "x".repeat(settings.max_state_bytes));
    assert_eq!(admit_frame(&large, &settings, &mut budget, now), Err(REFUSED_TOO_LARGE.to_string()));
    assert_eq!(admit_frame(r#"{"type":"batch","entries":[],"left":[]}"#, &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    assert_eq!(admit_frame("not json", &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    let mut spent = StateBudget::new(settings.max_states_per_second);
    let start = Instant::now();
    let admitted = (0..40).filter(|_| admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start) == Ok(Some(PresenceFrame::State { state: serde_json::json!(1) }))).count();
    assert_eq!(admitted, settings.max_states_per_second as usize);
    assert_eq!(admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start + Duration::from_secs(1)), Ok(Some(PresenceFrame::State { state: serde_json::json!(1) })));
}

#[test]
fn presence_session_ids_are_unique_hex() {
    let first = presence_session_id();
    let second = presence_session_id();
    assert_ne!(first, second);
    assert!(first.len() == 32 && first.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
}
//#endregion 🔖️PresenceRoom
