#[test]
fn accepted_final_page_completes_without_a_follow_up_turn() {
    let cursor = semio_framework::kernel::CommandPageCursor { owner: 7, generation: 11, command_index: 0, command_count: 1, instance: 13, seq: 17, kind: 19, page_index: 0, page_count: 1, item_count: 0, metadata: 0 };

    assert!(matches!(
        super::terminal_command_ingress(cursor, None),
        semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal)
            if terminal.owner == 7 && terminal.page_index == 1
    ));
}

#[test]
fn accepted_final_page_preserves_a_terminal_fault() {
    let cursor = semio_framework::kernel::CommandPageCursor { owner: 7, generation: 11, command_index: 0, command_count: 1, instance: 13, seq: 17, kind: 19, page_index: 0, page_count: 1, item_count: 0, metadata: 0 };

    assert!(matches!(
        super::terminal_command_ingress(cursor, Some(b"rejected".to_vec())),
        semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault }
            if terminal.page_index == 1 && fault == b"rejected"
    ));
}

#[test]
fn async_actor_poll_awaits_exchange_and_render_work() {
    let source = include_str!("../../🔄️turn/🦀️.rs");
    let start = source.find("pub async fn poll_kernel<").expect("native kernel poll");
    let end = source[start..].find("fn route_exchange_output").map(|offset| start + offset).expect("poll implementation boundary");
    let poll = &source[start..end];
    assert!(poll.contains("pub async fn poll_kernel<"));
    assert!(poll.contains("plugin_exchange_boxed(runtime, cursor.instance, None).await"));
    assert!(poll.contains("plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, command))).await"));
    assert!(poll.contains("crate::plugin_runtime::plugin_render_surface(runtime, instance, &surface_key).await"));
    assert!(!poll.contains("resolve_ready(crate::plugin_runtime::plugin_exchange"));
    assert!(!poll.contains("resolve_ready(crate::plugin_runtime::plugin_render"));
}

/// 🧫️ The shared drain contract both twins read. The TS half lives in
/// `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧫️fixtures/command-ingress-drain.json`.
fn drain_contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧫️fixtures/command-ingress-drain.json")).expect("drain contract parses")
}

fn cursor(page_index: u32, page_count: u32) -> semio_framework::kernel::CommandPageCursor {
    semio_framework::kernel::CommandPageCursor { owner: 7, generation: 11, command_index: 0, command_count: 1, instance: 13, seq: 17, kind: 19, page_index, page_count, item_count: 0, metadata: 0 }
}

#[test]
fn a_turn_that_still_owns_an_ingress_never_answers_idle() {
    let contract = drain_contract();
    assert_eq!(contract["reactorStatuses"]["ownedTurnNeverIdle"], serde_json::Value::Bool(true));

    let owned = crate::reactor::turn::command_ingress_while_owned(semio_framework::kernel::CommandIngressStatus::Idle, Some(cursor(2, 5)));
    assert!(
        matches!(&owned, semio_framework::kernel::CommandIngressStatus::CommandPending(pending) if pending.page_index == 2 && pending.seq == 17),
        "an owner that advanced nothing this turn is still an owner: {owned:?}"
    );

    let unowned = crate::reactor::turn::command_ingress_while_owned(semio_framework::kernel::CommandIngressStatus::Idle, None);
    assert!(matches!(unowned, semio_framework::kernel::CommandIngressStatus::Idle), "and with no owner, idle stays the honest answer");

    for already in [
        semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor(0, 2)),
        semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor(1, 1)),
        semio_framework::kernel::CommandIngressStatus::Backpressure(cursor(0, 1)),
        semio_framework::kernel::CommandIngressStatus::Fault { cursor: cursor(0, 1), fault: b"named".to_vec() },
    ] {
        let kept = crate::reactor::turn::command_ingress_while_owned(already.clone(), Some(cursor(0, 1)));
        assert_eq!(format!("{kept:?}"), format!("{already:?}"), "a turn that already said something keeps saying it");
    }
    eprintln!("[DEBUG] owned ingress turn answers command-pending, unowned answers idle, every stated status is preserved");
}

#[test]
fn every_admitted_command_page_leaves_the_turn_with_a_named_status() {
    let contract = drain_contract();
    let named = contract["reactorStatuses"]["unownedPageFault"].as_str().expect("fault code declared").to_string();
    let source = include_str!("../../🔄️turn/🦀️.rs");
    let start = source.find("if let Some((cursor, page)) = command_page {").expect("the page admission chain");
    let end = source[start..].find("command_ingress = command_ingress_while_owned").map(|offset| start + offset).expect("the chain ends at the ownership rule");
    let chain = &source[start..end];

    assert!(chain.contains(&named), "the chain names the drop it used to perform silently");
    assert!(
        chain.contains("} else {\n            // 📥️ The terminal arm this chain never had"),
        "the admission chain has a terminal else, so no page can fall out of it unremarked"
    );
    assert!(
        !chain.contains("} else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, mut pages }) = retained.take() {"),
        "the assembly arm no longer takes-and-drops every other owner shape"
    );
    eprintln!("[DEBUG] command page admission: terminal else present, fault named {named}, assembly take is shape-guarded");
}
