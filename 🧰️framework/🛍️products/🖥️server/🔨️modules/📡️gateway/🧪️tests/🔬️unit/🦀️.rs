
use super::*;
use crate::contract::{CommandId, PolicyGrant, QueryConsistency, QueryId, TraceContext};
use crate::test_instance::{CountingModule, SilentSaga, TestInstance, TestSagas, CREATION};
use crate::throttle::{Allowance, Rate};
use futures::channel::mpsc::unbounded;
use std::time::Duration;

async fn state() -> ServerState<TestInstance> {
    server().await.state().clone()
}

async fn server() -> Server<TestInstance> {
    Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).build().await.expect("the test instance opens its stores")
}

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: vec![seq as u8] }
}

fn grant(state: &ServerState<TestInstance>, point: PolicyPoint, action: &str) {
    let mut engine = state.policy.write().unwrap();
    engine.register_template(PolicyTemplate { name: format!("{point:?}-{action}"), auto_apply: false, grants: vec![PolicyGrant { point, resource: "*".into(), action: action.to_string() }] });
    engine.assign("anonymous".to_string(), format!("{point:?}-{action}"));
}

fn scratch(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("semio-gateway-{name}-{unique}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn loopback() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4242))
}

fn caller() -> ClientKey {
    ClientKey::of(loopback().ip())
}

fn address(last: u8) -> ClientKey {
    ClientKey::of(IpAddr::V4(Ipv4Addr::new(203, 0, 113, last)))
}

#[path = "👥️presence-fanout-and-kick/🦀️.rs"]
mod presence_fanout_and_kick;

#[path = "🗂️static-cors-credential-and-blob/🦀️.rs"]
mod static_cors_credential_and_blob;

#[path = "🔁️event-stream-and-server/🦀️.rs"]
mod event_stream_and_server;
use event_stream_and_server::*;

#[path = "🛰️presence-rooms-and-budget/🦀️.rs"]
mod presence_rooms_and_budget;
use presence_rooms_and_budget::*;

#[path = "🧵️presence-sessions/🦀️.rs"]
mod presence_sessions;

#[path = "🛂️edge-and-throttle/🦀️.rs"]
mod edge_and_throttle;

//#region 🔖️Edge
use tower::ServiceExt;

//#endregion 🔖️Edge
