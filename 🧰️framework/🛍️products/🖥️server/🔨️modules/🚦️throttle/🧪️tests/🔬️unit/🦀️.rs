use super::*;
use governor::clock::{Clock, FakeRelativeClock};
use governor::{Quota, RateLimiter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::num::NonZeroU32;

/// 🧫️ The shared schedule document, read from the product root rather than copied into this module.
const FIXTURE: &str = include_str!("../../../../🧫️fixtures/🚦️throttle/🔣️.json");

fn v4(last: u8) -> ClientKey {
    ClientKey::of(IpAddr::V4(Ipv4Addr::new(203, 0, 113, last)))
}

fn limits(rate: Rate) -> Limits {
    Limits { writes: rate, reads: rate, upgrades: rate, ..Limits::default() }
}

fn at(start: Instant, millis: u64) -> Instant {
    start + Duration::from_millis(millis)
}

//#region 🔖️Bucket
#[test]
fn the_bucket_answers_every_schedule_of_the_fixture_as_an_independent_limiter_does() {
    let document: serde_json::Value = serde_json::from_str(FIXTURE).expect("the throttle fixture is valid JSON");
    assert_eq!(document["schema"], "semio.framework.server.throttle/v2");
    let cases = document["cases"].as_array().expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("a named case");
        let rate = Rate { tokens: case["rate"]["tokens"].as_u64().expect("tokens") as u32, per: Duration::from_millis(case["rate"]["perMs"].as_u64().expect("perMs")), burst: case["rate"]["burst"].as_u64().expect("burst") as u32 };
        let start = Instant::now();
        let throttle = Throttle::new(limits(rate), start);
        let clock = FakeRelativeClock::default();
        let oracle = RateLimiter::direct_with_clock(Quota::with_period(rate.per / rate.tokens).expect("a period").allow_burst(NonZeroU32::new(rate.burst).expect("a burst")), clock.clone());
        let mut elapsed = 0;
        for (index, request) in case["requests"].as_array().expect("requests").iter().enumerate() {
            let arrives = request["atMs"].as_u64().expect("atMs");
            clock.advance(Duration::from_millis(arrives - elapsed));
            elapsed = arrives;
            let expected = match request["admitted"].as_bool().expect("admitted") {
                true => Ok(()),
                false => Err(Duration::from_millis(request["retryAfterMs"].as_u64().expect("retryAfterMs"))),
            };
            let ours = throttle.admit(v4(1), RequestClass::Write, at(start, arrives)).map_err(|refusal| match refusal {
                Refusal::Throttled { retry_after } => retry_after,
                Refusal::Overloaded { .. } => panic!("{name} #{index}: a rate never overloads"),
            });
            let theirs = oracle.check().map_err(|not_until| not_until.wait_time_from(clock.now()));
            assert_eq!(ours, expected, "{name} #{index}");
            assert_eq!(theirs, expected, "{name} #{index}: the generic cell rate algorithm disagrees with the fixture");
        }
    }
}

#[test]
fn every_class_spends_from_its_own_bucket() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { writes: Rate::per_second(1, 1), reads: Rate::per_second(1, 2), upgrades: Rate::per_second(1, 1), ..Limits::default() }, start);
    assert_eq!(throttle.admit(v4(1), RequestClass::Write, start), Ok(()));
    assert_eq!(throttle.admit(v4(1), RequestClass::Write, start), Err(Refusal::Throttled { retry_after: Duration::from_secs(1) }));
    assert_eq!(throttle.admit(v4(1), RequestClass::Read, start), Ok(()));
    assert_eq!(throttle.admit(v4(1), RequestClass::Read, start), Ok(()));
    assert!(throttle.admit(v4(1), RequestClass::Read, start).is_err());
    assert_eq!(throttle.admit(v4(1), RequestClass::Upgrade, start), Ok(()));
}
#[test]
fn a_named_allowance_is_a_bucket_of_its_own_and_takes_back_what_changed_nothing() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { allowances: vec![Allowance { name: "sign-up", rate: Rate::per_hour(60, 2) }], ..Limits::default() }, start);
    assert_eq!(throttle.spend(v4(1), "sign-up", start), Ok(()));
    assert_eq!(throttle.spend(v4(1), "sign-up", start), Ok(()));
    assert_eq!(throttle.spend(v4(1), "sign-up", start), Err(Refusal::Throttled { retry_after: Duration::from_secs(60) }), "the wait is the time one token takes at sixty per hour");
    assert_eq!(throttle.admit(v4(1), RequestClass::Write, start), Ok(()), "the request classes are other buckets");
    assert_eq!(throttle.spend(v4(2), "sign-up", start), Ok(()), "another address has its own allowance");
    throttle.refund(v4(1), "sign-up", start);
    assert_eq!(throttle.spend(v4(1), "sign-up", start), Ok(()), "a token handed back is spent again");
    assert!(throttle.spend(v4(1), "sign-up", start).is_err());
    for _ in 0..5 {
        throttle.refund(v4(1), "sign-up", start);
    }
    assert_eq!((throttle.spend(v4(1), "sign-up", start), throttle.spend(v4(1), "sign-up", start)), (Ok(()), Ok(())));
    assert!(throttle.spend(v4(1), "sign-up", start).is_err(), "handing back never fills a bucket past its burst");
    assert_eq!(throttle.spend(v4(1), "sign-up", at(start, 60_000)), Ok(()), "a minute later one token has flowed in");
    for _ in 0..1000 {
        assert_eq!(throttle.spend(v4(1), "upload", start), Ok(()), "an allowance the instance did not size refuses nothing");
    }
    throttle.refund(v4(1), "upload", start);
    assert_eq!(Rate::per_hour(60, 2).refill(), Duration::from_secs(120));
    assert_eq!(Rate::per_second(300, 900).refill(), Duration::from_secs(3));
}
//#endregion 🔖️Bucket

//#region 🔖️Client
#[test]
fn an_exhausted_address_does_not_slow_another() {
    let start = Instant::now();
    let throttle = Throttle::new(limits(Rate::per_second(1, 2)), start);
    for _ in 0..2 {
        assert_eq!(throttle.admit(v4(1), RequestClass::Write, start), Ok(()));
    }
    for _ in 0..1000 {
        assert!(throttle.admit(v4(1), RequestClass::Write, start).is_err());
    }
    assert_eq!(throttle.admit(v4(2), RequestClass::Write, start), Ok(()));
    assert_eq!(throttle.clients(), 2);
}

#[test]
fn an_ipv6_client_is_the_whole_block_a_subscriber_is_routed() {
    let site = |network: u16, host: u16| ClientKey::of(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0x1234, network, 0, 0, 0, host)));
    assert_eq!(site(0x5678, 1), site(0x5678, 0xffff), "one network is one client however many addresses it mints");
    assert_eq!(site(0, 1), site(0xffff, 1), "one /48 is one client however many of its 65536 networks it mints");
    assert_ne!(site(0, 1), ClientKey::of(IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0x1235, 0, 0, 0, 0, 1))));
    assert_eq!(ClientKey::of(IpAddr::V6(Ipv4Addr::new(203, 0, 113, 7).to_ipv6_mapped())), v4(7), "an IPv4 client reached over a dual-stack socket is the same client");
    assert_ne!(ClientKey::of(IpAddr::V4(Ipv4Addr::LOCALHOST)), ClientKey::of(IpAddr::V6(Ipv6Addr::LOCALHOST)));
}

#[test]
fn the_forwarded_address_is_the_last_entry_the_proxy_wrote() {
    let address = |text: &str| text.parse::<IpAddr>().expect("an address");
    assert_eq!(forwarded_address("203.0.113.7"), Some(address("203.0.113.7")));
    assert_eq!(forwarded_address("198.51.100.1, 203.0.113.7"), Some(address("203.0.113.7")), "what a client claimed in front of the proxy's entry is ignored");
    assert_eq!(forwarded_address(" 2001:db8::1 "), Some(address("2001:db8::1")));
    assert_eq!(forwarded_address("[2001:db8::1]"), Some(address("2001:db8::1")));
    assert_eq!(forwarded_address("[2001:db8::1]:4711"), Some(address("2001:db8::1")));
    assert_eq!(forwarded_address("203.0.113.7:4711"), Some(address("203.0.113.7")));
    for junk in ["", "unknown", "203.0.113.7, ", "203.0.113.7; DROP", "_hidden"] {
        assert_eq!(forwarded_address(junk), None, "{junk:?}");
    }
}
//#endregion 🔖️Client

//#region 🔖️Caps
#[test]
fn sockets_are_capped_per_address_and_in_total_and_freed_when_closed() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { sockets_per_client: 2, sockets: 3, ..Limits::default() }, start);
    let first = throttle.open_socket(v4(1), start).expect("first");
    let _second = throttle.open_socket(v4(1), start).expect("second");
    assert_eq!(throttle.open_socket(v4(1), start).err(), Some(Refusal::Throttled { retry_after: OVERLOAD_RETRY }), "the address is at its own cap");
    let _third = throttle.open_socket(v4(2), start).expect("another address still opens one");
    assert_eq!(throttle.open_socket(v4(2), start).err(), Some(Refusal::Overloaded { retry_after: OVERLOAD_RETRY }), "the instance is at its cap");
    assert_eq!(throttle.sockets(), 3);
    drop(first);
    assert_eq!(throttle.sockets(), 2);
    let _again = throttle.open_socket(v4(1), start).expect("a closed socket frees its place");
}

#[test]
fn requests_in_flight_are_capped_and_freed_when_answered() {
    let throttle = Throttle::new(Limits { in_flight: 2, ..Limits::default() }, Instant::now());
    let first = throttle.enter().expect("first");
    let _second = throttle.enter().expect("second");
    assert_eq!(throttle.enter().err(), Some(Refusal::Overloaded { retry_after: OVERLOAD_RETRY }));
    assert_eq!(throttle.in_flight(), 2);
    drop(first);
    assert!(throttle.enter().is_ok());
}

#[test]
fn open_limits_refuse_nothing_and_track_nobody() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits::open(), start);
    for index in 0..10_000u32 {
        let client = ClientKey::of(IpAddr::V4(Ipv4Addr::from(index)));
        assert_eq!(throttle.admit(client, RequestClass::Write, start), Ok(()));
    }
    let held: Vec<_> = (0..5000).map(|_| (throttle.enter().expect("in flight"), throttle.open_socket(v4(1), start).expect("socket"))).collect();
    assert_eq!((throttle.in_flight(), throttle.sockets(), throttle.clients()), (5000, 5000, 0));
    drop(held);
    assert_eq!((throttle.in_flight(), throttle.sockets()), (0, 0));
}
//#endregion 🔖️Caps

//#region 🔖️Table
#[test]
fn an_idle_address_is_forgotten_and_one_holding_a_socket_is_not() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { idle: Duration::from_secs(60), ..limits(Rate::per_second(1, 1)) }, start);
    for last in 1..=50 {
        assert_eq!(throttle.admit(v4(last), RequestClass::Read, start), Ok(()));
    }
    let held = throttle.open_socket(v4(200), start).expect("a socket");
    assert_eq!(throttle.clients(), 51);
    assert_eq!(throttle.admit(v4(1), RequestClass::Read, at(start, 61_000)), Ok(()), "the returning address starts with a full bucket");
    assert_eq!(throttle.clients(), 2, "the fifty idle addresses are gone, the one holding a socket and the one returning stay");
    drop(held);
    assert_eq!(throttle.admit(v4(1), RequestClass::Read, at(start, 200_000)), Ok(()));
    assert_eq!(throttle.clients(), 1);
}

#[test]
fn an_address_is_remembered_until_every_bucket_of_it_is_full_again() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { idle: Duration::from_secs(60), allowances: vec![Allowance { name: "sign-up", rate: Rate::per_hour(60, 2) }], ..Limits::default() }, start);
    assert_eq!((throttle.spend(v4(1), "sign-up", start), throttle.spend(v4(1), "sign-up", start)), (Ok(()), Ok(())));
    assert_eq!(throttle.admit(v4(2), RequestClass::Read, start), Ok(()));
    assert_eq!(throttle.admit(v4(3), RequestClass::Read, at(start, 61_000)), Ok(()));
    assert_eq!(throttle.clients(), 2, "the address that only read is forgotten; the one whose allowance is still refilling is not, however idle");
    assert_eq!(throttle.spend(v4(1), "sign-up", at(start, 61_000)), Ok(()));
    assert!(throttle.spend(v4(1), "sign-up", at(start, 61_000)).is_err(), "waiting out the idle span bought no second allowance");
    assert_eq!(throttle.admit(v4(3), RequestClass::Read, at(start, 100_000)), Ok(()));
    assert_eq!(throttle.clients(), 2);
    assert_eq!(throttle.admit(v4(3), RequestClass::Read, at(start, 400_000)), Ok(()));
    assert_eq!(throttle.clients(), 1, "full again and idle, the address is what a fresh one would be and is forgotten");
    assert_eq!((throttle.spend(v4(1), "sign-up", at(start, 400_000)), throttle.spend(v4(1), "sign-up", at(start, 400_000))), (Ok(()), Ok(())));
}

#[test]
fn a_full_table_makes_every_further_address_share_one_bucket() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits { clients: 4, sockets_per_client: 1, ..limits(Rate::per_second(1, 2)) }, start);
    for last in 1..=4 {
        assert_eq!(throttle.admit(v4(last), RequestClass::Write, start), Ok(()));
    }
    assert_eq!(throttle.admit(v4(100), RequestClass::Write, start), Ok(()));
    assert_eq!(throttle.admit(v4(101), RequestClass::Write, start), Ok(()));
    assert!(throttle.admit(v4(102), RequestClass::Write, start).is_err(), "minting addresses buys no further allowance");
    assert_eq!(throttle.clients(), 4, "the table never grows past its bound");
    assert_eq!(throttle.admit(v4(1), RequestClass::Write, start), Ok(()), "a tracked address keeps its own bucket");
    let shared = throttle.open_socket(v4(103), start).expect("the shared seat holds one socket");
    assert!(throttle.open_socket(v4(104), start).is_err());
    drop(shared);
    assert!(throttle.open_socket(v4(104), start).is_ok());
}
//#endregion 🔖️Table

//#region 🔖️Sizing
/// 🏫️ What a class of `learners` behind one address asks for during the second `second` of a
/// lecture: everyone enrols and starts a run within the first seconds, then answers, polls and
/// queries; at `drop` the network drops and every socket of every learner reconnects within three
/// seconds.
fn class_second(learners: u64, second: u64, drop: u64) -> (u64, u64, u64) {
    let enrolling = if second < 3 { learners * 2 / 3 } else { 0 };
    let answering = learners;
    let polling = learners / 10 + learners;
    let sockets = if second < 3 || (drop..drop + 3).contains(&second) { learners } else { 0 };
    (enrolling + answering, polling + enrolling, sockets)
}

#[test]
fn the_default_limits_never_throttle_a_class_of_three_hundred_behind_one_address() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits::default(), start);
    let hall = v4(1);
    let mut sockets = Vec::new();
    for second in 0..1800u64 {
        let (writes, reads, upgrades) = class_second(300, second, 600);
        if second == 600 {
            sockets.clear();
        }
        for (class, count) in [(RequestClass::Write, writes), (RequestClass::Read, reads), (RequestClass::Upgrade, upgrades)] {
            for request in 0..count {
                let now = at(start, second * 1000 + request * 1000 / count.max(1));
                assert_eq!(throttle.admit(hall, class, now), Ok(()), "{class:?} #{request} of second {second}");
                if class == RequestClass::Upgrade {
                    sockets.push(throttle.open_socket(hall, now).expect("a class's sockets fit one address"));
                }
            }
        }
    }
    assert_eq!(throttle.sockets(), 900);
}

#[test]
fn the_default_limits_stop_one_script_at_a_class_worth_of_requests() {
    let start = Instant::now();
    let throttle = Throttle::new(Limits::default(), start);
    let script = v4(66);
    let mut admitted = 0;
    for request in 0..600_000u64 {
        if throttle.admit(script, RequestClass::Write, at(start, request / 10)).is_ok() {
            admitted += 1;
        }
    }
    let limits = Limits::default();
    assert_eq!(admitted, u64::from(limits.writes.burst) + 60 * u64::from(limits.writes.tokens) - 1, "ten thousand commands per second for a minute are cut to the burst plus the rate");
    let held: Vec<_> = (0..limits.sockets_per_client).map(|_| throttle.open_socket(script, start).expect("up to the cap")).collect();
    assert!(throttle.open_socket(script, start).is_err());
    assert!(throttle.open_socket(v4(1), start).is_ok(), "the script cannot take every socket of the instance");
    drop(held);
}
//#endregion 🔖️Sizing
