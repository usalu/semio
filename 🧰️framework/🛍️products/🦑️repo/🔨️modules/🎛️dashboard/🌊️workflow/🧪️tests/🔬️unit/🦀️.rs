
use super::*;

#[test]
fn scheduler_respects_scope_and_dependencies() {
    let spec = WorkflowSpec {
        id: "demo".into(),
        concurrency: 1,
        waves: vec![WorkflowWave {
            id: "w0".into(),
            tasks: vec![
                WorkflowTask { id: "a".into(), model: "claude".into(), prompt: "one".into(), path_scope: vec!["x".into()], verify: None, retries: 0, depends_on: vec![] },
                WorkflowTask { id: "b".into(), model: "claude".into(), prompt: "two".into(), path_scope: vec!["x".into()], verify: None, retries: 0, depends_on: vec!["a".into()] },
            ],
        }],
    };
    let mut scheduler = Scheduler::new(spec);
    assert_eq!(scheduler.poll_ready(), vec!["a".to_string()]);
    assert!(scheduler.poll_ready().is_empty(), "scope claimed / concurrency full");
    scheduler.complete("a", true, None);
    assert_eq!(scheduler.poll_ready(), vec!["b".to_string()]);
    assert_eq!(scheduler.statuses["b"].state, TaskState::Running);
}

struct UnavailableRunner;

impl AgentRunner for UnavailableRunner {
    fn id(&self) -> &str {
        "unavailable"
    }
    fn available(&self) -> bool {
        false
    }
    fn spawn(&self, _: &str, _: &Path) -> std::io::Result<std::process::Child> {
        unreachable!()
    }
}

#[test]
fn unavailable_runners_are_filtered() {
    assert!(available_runners(vec![UnavailableRunner]).is_empty());
}

#[test]
fn platform_probe_command_matches_host() {
    assert_eq!(probe_command(), if cfg!(windows) { "where" } else { "which" });
}
