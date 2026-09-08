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
    assert!(poll.contains("plugin_exchange(runtime, cursor.instance, None).await"));
    assert!(poll.contains("plugin_exchange(runtime, cursor.instance, Some((cursor.seq, command))).await"));
    assert!(poll.contains("plugin_render(runtime, instance, &body_key, \"{}\").await"));
    assert!(!poll.contains("resolve_ready(crate::plugin_runtime::plugin_exchange"));
    assert!(!poll.contains("resolve_ready(crate::plugin_runtime::plugin_render"));
}
