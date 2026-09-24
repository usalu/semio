use super::*;

#[test]
fn a_neutral_slug_resolves_without_a_client() {
    let (event, parent) = resolve_hook_event("agent.started", "", "", None).expect("neutral slug resolves");
    assert_eq!(event, HookEvent::AgentStarted);
    assert_eq!(parent, "");
}

#[test]
fn a_composite_command_is_blocked_by_its_worst_segment() {
    assert_eq!(is_tool_blocked("Bash", "ls -la && git stash"), Some("blocked: git stash; other developers and agents may be editing the same files concurrently".to_string()));
}

#[test]
fn inline_python_hiding_git_is_blocked() {
    assert_eq!(contains_blocked_git_in_code("-c import os; os.system('git  CHECKOUT main')"), Some("blocked: git  checkout".to_string()));
}

#[test]
fn a_kill_by_lsof_port_is_blocked() {
    assert!(is_command_segment_blocked("kill -9 $(lsof -t -i:3000)").is_some());
}

#[test]
fn a_dropped_plan_step_is_abandoned() {
    let existing = merge_plan_steps(&[], &[HookPlanStep { name: "one".into(), status: "not-started".into() }], "2026-09-06T00:00:00Z");
    let merged = merge_plan_steps(&existing, &[HookPlanStep { name: "two".into(), status: "in-progress".into() }], "2026-09-06T00:01:00Z");
    assert_eq!(merged.len(), 2);
    assert_eq!(merged[1].abandoned, "2026-09-06T00:01:00Z");
}

#[test]
fn a_session_log_is_silent_when_logging_is_off() {
    let mut store = MemorySessionStore::new();
    let context = HookContext::new(HookEvent::AgentStarted, "claude-code");
    let logging = LoggingConfig { session: false, operations: true, plan: true, detail: "standard".into() };
    let written = record_session_hook(&mut store, &context, &HookResult::new(true, ""), "s1", &logging, &InertEnvironment).expect("no failure");
    assert!(written.is_none());
    assert!(store.sessions().is_empty());
}
