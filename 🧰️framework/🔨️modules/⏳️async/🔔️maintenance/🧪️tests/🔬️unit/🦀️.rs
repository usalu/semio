
use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).unwrap()
}
fn idle(_: [u64; 2]) -> WorkerMaintenanceStep {
    WorkerMaintenanceStep::Idle
}

#[test]
fn worker_maintenance_matches_neutral_retention_and_aba_lifecycle() {
    let registry = WorkerMaintenanceRegistry::new();
    let mut owners = std::collections::BTreeMap::new();
    let mut running = None;
    for step in fixture()["lifecycle"].as_array().unwrap() {
        let owner = step["owner"].as_str().unwrap();
        let actual = match step["action"].as_str().unwrap() {
            "install" => {
                owners.insert(owner.to_string(), registry.install(Lane::Io, idle, [0; 2]).unwrap());
                "installed"
            }
            "request" => match registry.request(owners[owner]) {
                Ok(WorkerMaintenanceRequest::Requested) => "requested",
                Ok(WorkerMaintenanceRequest::Coalesced) => "coalesced",
                Err(WorkerMaintenanceError::Stale) => "stale",
                Err(WorkerMaintenanceError::Closed) => "closed",
                other => panic!("unexpected request {other:?}"),
            },
            "take" => {
                let Some(PoolWork::Maintenance(invocation)) = registry.select(Lane::Io, &mut VecDeque::new()) else { panic!("requested hook was not selected") };
                assert_eq!(invocation.ticket, owners[owner]);
                running = Some(invocation);
                "running"
            }
            "remove" => {
                if registry.remove(owners[owner]).unwrap() {
                    "removed"
                } else {
                    "pending"
                }
            }
            action => {
                let disposition = match action {
                    "finish-more" => WorkerMaintenanceStep::More,
                    "finish-idle" => WorkerMaintenanceStep::Idle,
                    "finish-fault" => WorkerMaintenanceStep::Fault,
                    _ => unreachable!(),
                };
                registry.finish(&running.take().unwrap(), disposition);
                if registry.has_pending(None) { "requested" } else { "idle" }
            }
        };
        assert_eq!(actual, step["expected"].as_str().unwrap(), "{step}");
    }
    assert!(registry.state.lock().unwrap().entries.iter().all(Option::is_none));
    eprintln!("[DEBUG] fixed maintenance slots matched all 20 neutral coalescing, running-close, concurrent-wake, fault, and ABA transitions");
}

#[test]
fn worker_maintenance_capacity_and_pool_identity_are_exact() {
    let registry = WorkerMaintenanceRegistry::new();
    let other = WorkerMaintenanceRegistry::new();
    let mut tickets = Vec::new();
    assert_eq!(WORKER_MAINTENANCE_CAPACITY, fixture()["capacity"].as_u64().unwrap() as usize);
    for _ in 0..WORKER_MAINTENANCE_CAPACITY {
        tickets.push(registry.install(Lane::Io, idle, [0; 2]).unwrap());
    }
    assert_eq!(registry.install(Lane::Io, idle, [0; 2]), Err(WorkerMaintenanceError::Capacity));
    assert_eq!(other.request(tickets[0]), Err(WorkerMaintenanceError::Stale));
    let old = tickets.remove(0);
    assert!(registry.remove(old).unwrap());
    let fresh = registry.install(Lane::Io, idle, [0; 2]).unwrap();
    assert_eq!(old.slot, fresh.slot);
    assert_ne!(old.generation, fresh.generation);
    assert_eq!(registry.request(old), Err(WorkerMaintenanceError::Stale));
    tickets.push(fresh);
    for ticket in tickets {
        assert!(registry.remove(ticket).unwrap());
    }
    registry.state.lock().unwrap().next_generation = u64::MAX;
    assert_eq!(registry.install(Lane::Io, idle, [0; 2]), Err(WorkerMaintenanceError::GenerationExhausted));
    eprintln!("[DEBUG] maintenance admission fenced foreign pools, capacity+1, retired generations, and exhausted generation without wrapping");
}

#[test]
fn worker_maintenance_running_callback_retires_requested_generation_exactly_once() {
    let registry = WorkerMaintenanceRegistry::new();
    let ticket = registry.install(Lane::Io, idle, [0; 2]).unwrap();
    assert_eq!(registry.request(ticket), Ok(WorkerMaintenanceRequest::Requested));
    let Some(PoolWork::Maintenance(invocation)) = registry.select(Lane::Io, &mut VecDeque::new()) else {
        panic!("requested self-retiring hook was not selected");
    };
    assert_eq!(registry.request(ticket), Ok(WorkerMaintenanceRequest::Requested));
    registry.finish(&invocation, WorkerMaintenanceStep::Retire);
    assert_eq!(registry.request(ticket), Err(WorkerMaintenanceError::Stale));
    let replacement = registry.install(Lane::Io, idle, [0; 2]).unwrap();
    assert_ne!(replacement.generation, ticket.generation);
    assert!(registry.remove(replacement).unwrap());
}
