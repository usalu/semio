//! 🔒️ Gateway laws that drive sibling-private functions: the session relay framing and the admission request of a command. See [`super`].

use super::*;
use crate::contract::{CommandId, TraceContext};
use crate::test_instance::{CountingModule, TestInstance};

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
}

fn envelope() -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId("cmd-1".into()),
        kind: "counter.increment".into(),
        version: 1,
        target: actor("c1"),
        scope: Scope("space-1".into()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: vec![1],
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: None,
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

//#region 🔖️Relay
#[test]
fn a_relayed_frame_names_the_session_that_produced_it() {
    let wrapped = wrap_relay("session-7", b"frame");
    assert_eq!(unwrap_relay(&wrapped), Some(("session-7", b"frame".as_slice())));
    assert_eq!(unwrap_relay(b"\x40\x00short"), None);
    assert_eq!(unwrap_relay(&[]), None);
}
//#endregion 🔖️Relay


//#region 🔖️Server
#[tokio::test]
async fn build_collects_every_module_contribution() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() })
        .identity("hub", "0.1.0")
        .module(CountingModule)
        .app("admin", "/srv/admin")
        .admin_token(Some("secret".to_string()))
        .build()
        .await
        .expect("the test instance opens its stores");

    assert_eq!(server.definition().modules.len(), 1);
    assert_eq!(server.definition().id, "hub");
    assert!(server.state().admin.is_configured());
    assert_eq!(server.state().apps.names(), vec!["admin".to_string()]);
    assert!(server.state().documents.is_none());

    let request = admission_request(&envelope());
    assert!(server.state().authorize(&request).is_err());
    server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    assert!(server.state().authorize(&request).is_ok());
    let _ = server.router();
}
//#endregion 🔖️Server
