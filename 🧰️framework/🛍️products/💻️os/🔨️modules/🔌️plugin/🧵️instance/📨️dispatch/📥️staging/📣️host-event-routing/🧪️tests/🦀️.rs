//! 🧪️ The command scope is a held lease that grants its waiting lanes in arrival order; the language-agnostic law is `🔣️.json`.
use super::*;
use std::convert::Infallible;

fn law() -> serde_json::Value { serde_json::from_str(include_str!("🔣️.json")).unwrap() }
fn lane(key: &serde_json::Value) -> char { key.as_str().unwrap().chars().next().unwrap() }
/// 🎟️ One claim turn of a lane; `affordable` is whether its grant pays the claim.
fn claim(lease: &mut Lease<char>, key: char, pending: bool, affordable: bool) -> bool { lease.arbitrate::<Infallible>(key, pending, || Ok(affordable)).unwrap() }

#[test]
fn every_scripted_lease_sequence_follows_the_law() {
    let law = law();
    for (name, script) in law["scripts"].as_object().unwrap() {
        let mut lease = Lease::<char>::default();
        for (step, op) in script.as_array().unwrap().iter().enumerate() {
            let op = op.as_array().unwrap();
            let at = format!("{name} step {step}: {op:?}");
            match op[0].as_str().unwrap() {
                "claim" => assert_eq!(claim(&mut lease, lane(&op[1]), op[2].as_bool().unwrap(), true), op[3].as_bool().unwrap(), "{at}"),
                "poor" => assert_eq!(claim(&mut lease, lane(&op[1]), op[2].as_bool().unwrap(), false), op[3].as_bool().unwrap(), "{at}"),
                "holds" => assert_eq!(lease.holds(lane(&op[1])), op[2].as_bool().unwrap(), "{at}"),
                "queued" => assert_eq!(lease.queued(lane(&op[1])), op[2].as_bool().unwrap(), "{at}"),
                "leave" => assert_eq!(lease.leave(lane(&op[1])), op[2].as_bool().unwrap(), "{at}"),
                "idlePublication" => assert_eq!(lease.leave_if_idle(lane(&op[1]), op[2].as_bool().unwrap()), op[3].as_bool().unwrap(), "{at}"),
                "idle" => assert_eq!(lease.is_idle(), op[1].as_bool().unwrap(), "{at}"),
                other => panic!("unknown lease op {other}"),
            }
        }
    }
}

#[test]
fn two_idle_callers_in_reactor_order_each_complete_within_the_fair_bound() {
    let law = law();
    let fairness = &law["fairness"];
    let (rounds, bound) = (fairness["rounds"].as_u64().unwrap(), fairness["eachCompletesWithin"].as_u64().unwrap());
    let order: Vec<char> = fairness["order"].as_array().unwrap().iter().map(lane).collect();
    let mut lease = Lease::<char>::default();
    let mut first = vec![None; order.len()];
    let mut completions = vec![0u64; order.len()];
    for round in 1..=rounds {
        for (at, key) in order.iter().enumerate() {
            if claim(&mut lease, *key, false, true) {
                assert!(lease.leave(*key));
                first[at].get_or_insert(round);
                completions[at] += 1;
            }
        }
    }
    for (at, key) in order.iter().enumerate() {
        assert!(first[at].is_some_and(|round| round <= bound), "lane {key} first completed in round {:?}, bound {bound}", first[at]);
        assert!(completions[at] >= 2, "lane {key} completed {} times in {rounds} rounds", completions[at]);
    }
}

#[test]
fn a_held_lease_is_never_overwritten_in_any_round() {
    let mut lease = Lease::<char>::default();
    assert!(!claim(&mut lease, 'a', false, true));
    for _ in 0..32 {
        assert!(!claim(&mut lease, 'b', false, true));
        assert!(lease.holds('a'));
    }
    assert!(claim(&mut lease, 'a', false, true));
}

#[test]
fn a_full_queue_refuses_further_lanes_without_disturbing_the_holder() {
    let mut lease = Lease::<u8>::default();
    assert!(!lease.arbitrate::<Infallible>(0, false, || Ok(true)).unwrap());
    for key in 1..=COMMAND_LEASE_WAITERS as u8 + 3 {
        assert!(!lease.arbitrate::<Infallible>(key, false, || Ok(true)).unwrap());
    }
    assert!(lease.holds(0));
    assert!(lease.queued(1) && lease.queued(COMMAND_LEASE_WAITERS as u8));
    assert!(!lease.queued(COMMAND_LEASE_WAITERS as u8 + 1));
    assert!(lease.leave(0));
    assert!(!lease.arbitrate::<Infallible>(1, false, || Ok(true)).unwrap());
    assert!(lease.holds(1));
}
