//! 🧵️ `shell-doc-open` staging of React's `🧵️TaskManager`: the live actor table (one honest row per resident actor), the running-task model with suspend/resume/cancel, and the label tables, all pinned by the language-agnostic fixtures `🧵️actor-table.json` and `🏃️running-tasks.json`.

use super::*;
use semio_renderer_wgpu_shell_vocab::shell_chrome_format;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashMap;

pub const TASK_MANAGER_CONTROLLER_ID: &str = "os.task-manager";
pub const TASK_MANAGER_UNOBSERVED_METRIC: &str = "—";

//#region Vocabulary
/// 🛣️ Mirrors `semio_framework_actor::Lane`'s four camelCase tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagerLane {
    Interactive,
    UserVisible,
    Background,
    Maintenance,
}

impl TaskManagerLane {
    pub fn label(self, is_de: bool) -> &'static str {
        shell_chrome_string(
            match self {
                Self::Interactive => "taskManager.lanes.interactive",
                Self::UserVisible => "taskManager.lanes.userVisible",
                Self::Background => "taskManager.lanes.background",
                Self::Maintenance => "taskManager.lanes.maintenance",
            },
            is_de,
        )
    }
}

/// 🗂️ Mirrors `semio_framework_actor::ActorStatus`'s tags.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagerStatus {
    Cold,
    Activating,
    Active,
    Suspended,
    Draining,
    Trapped,
    Quarantined,
    Disabled,
}

impl TaskManagerStatus {
    pub fn label(self, is_de: bool) -> &'static str {
        shell_chrome_string(
            match self {
                Self::Cold => "taskManager.statuses.cold",
                Self::Activating => "taskManager.statuses.activating",
                Self::Active => "taskManager.statuses.active",
                Self::Suspended => "taskManager.statuses.suspended",
                Self::Draining => "taskManager.statuses.draining",
                Self::Trapped => "taskManager.statuses.trapped",
                Self::Quarantined => "taskManager.statuses.quarantined",
                Self::Disabled => "taskManager.statuses.disabled",
            },
            is_de,
        )
    }
}

/// 🚑️ Mirrors `semio_framework_actor::FailureStage`'s tags; React prints the tag itself, untranslated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagerStage {
    Healthy,
    Warned,
    Throttled,
    Trapped,
    Quarantined,
    Disabled,
    Cancelled,
}

impl TaskManagerStage {
    pub fn tag(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warned => "warned",
            Self::Throttled => "throttled",
            Self::Trapped => "trapped",
            Self::Quarantined => "quarantined",
            Self::Disabled => "disabled",
            Self::Cancelled => "cancelled",
        }
    }
}

/// 🎬️ The three row actions, in React's column order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskManagerActionKind {
    Suspend,
    Resume,
    Cancel,
}

impl TaskManagerActionKind {
    pub const ALL: [Self; 3] = [Self::Suspend, Self::Resume, Self::Cancel];

    pub fn tag(self) -> &'static str {
        match self {
            Self::Suspend => "suspend",
            Self::Resume => "resume",
            Self::Cancel => "cancel",
        }
    }

    pub fn from_tag(tag: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.tag() == tag)
    }

    pub fn icon(self) -> IconName {
        match self {
            Self::Suspend => IconName::Pause,
            Self::Resume => IconName::Play,
            Self::Cancel => IconName::Square,
        }
    }

    pub fn icon_id(self) -> &'static str {
        match self {
            Self::Suspend => "pause",
            Self::Resume => "play",
            Self::Cancel => "square",
        }
    }

    pub fn label(self, is_de: bool) -> &'static str {
        shell_chrome_string(
            match self {
                Self::Suspend => "taskManager.actions.suspend",
                Self::Resume => "taskManager.actions.resume",
                Self::Cancel => "taskManager.actions.cancel",
            },
            is_de,
        )
    }

    fn task_label(self, is_de: bool, task: &str) -> String {
        shell_chrome_format(
            match self {
                Self::Suspend => "taskManager.tasks.suspendTask",
                Self::Resume => "taskManager.tasks.resumeTask",
                Self::Cancel => "taskManager.tasks.cancelTask",
            },
            is_de,
            &[("task", task)],
        )
    }
}

/// 🛣️ Where a running task comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagerTaskLane {
    Job,
    Activation,
    ToolCall,
    ToolRun,
    DocumentTransfer,
    Export,
}

impl TaskManagerTaskLane {
    pub fn label(self, is_de: bool) -> &'static str {
        shell_chrome_string(
            match self {
                Self::Job => "taskManager.tasks.lanes.job",
                Self::Activation => "taskManager.tasks.lanes.activation",
                Self::ToolCall => "taskManager.tasks.lanes.toolCall",
                Self::ToolRun => "taskManager.tasks.lanes.toolRun",
                Self::DocumentTransfer => "taskManager.tasks.lanes.documentTransfer",
                Self::Export => "taskManager.tasks.lanes.export",
            },
            is_de,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagerTaskState {
    Running,
    Suspended,
    Cancelling,
}

impl TaskManagerTaskState {
    pub fn label(self, is_de: bool) -> &'static str {
        shell_chrome_string(
            match self {
                Self::Running => "taskManager.tasks.running",
                Self::Suspended => "taskManager.tasks.suspended",
                Self::Cancelling => "taskManager.tasks.cancelling",
            },
            is_de,
        )
    }
}
//#endregion Vocabulary

//#region ActorTable
/// 🧵️ One live-actor row; `None` renders as the em dash, never a silent zero.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagerRow {
    pub actor_id: String,
    pub package_id: String,
    pub lane: TaskManagerLane,
    pub status: TaskManagerStatus,
    pub stage: TaskManagerStage,
    pub shard: Option<u64>,
    pub wall_us_p95: Option<u64>,
    pub mailbox_len: Option<u64>,
    pub turns: Option<u64>,
    pub traps: Option<u64>,
    pub restarts: Option<u64>,
}

/// 📈️ One actor of an `os.runtime.metrics` snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeActorSample {
    pub actor_id: String,
    pub plugin_id: String,
    pub resident: bool,
    pub shard: u64,
}

/// 📈️ React's `runtimeMetricsRowsV1`: residency is the only observable status, `maintenance` the lane of an unlabelled row, every counter unobserved.
pub fn runtime_metrics_rows(actors: &[RuntimeActorSample]) -> Vec<TaskManagerRow> {
    actors
        .iter()
        .map(|actor| TaskManagerRow {
            actor_id: actor.actor_id.clone(),
            package_id: actor.plugin_id.clone(),
            lane: TaskManagerLane::Maintenance,
            status: if actor.resident { TaskManagerStatus::Active } else { TaskManagerStatus::Suspended },
            stage: TaskManagerStage::Healthy,
            shard: Some(actor.shard),
            wall_us_p95: None,
            mailbox_len: None,
            turns: None,
            traps: None,
            restarts: None,
        })
        .collect()
}

const COLUMNS: [(&str, &str); 12] = [
    ("actorId", "taskManager.columns.actorId"),
    ("packageId", "taskManager.columns.packageId"),
    ("lane", "taskManager.columns.lane"),
    ("status", "taskManager.columns.status"),
    ("stage", "taskManager.columns.stage"),
    ("shard", "taskManager.columns.shard"),
    ("wallUsP95", "taskManager.columns.wallUsP95"),
    ("mailboxLen", "taskManager.columns.mailboxLen"),
    ("turns", "taskManager.columns.turns"),
    ("traps", "taskManager.columns.traps"),
    ("restarts", "taskManager.columns.restarts"),
    ("actions", "taskManager.columns.actions"),
];

/// 🔖️ React's `taskManagerColumns`: the twelve columns in display order, every one sortable but `actions`.
pub fn task_manager_columns(is_de: bool) -> Vec<Value> {
    COLUMNS.iter().map(|(id, key)| if *id == "actions" { json!({ "id": id, "label": shell_chrome_string(key, is_de) }) } else { json!({ "id": id, "label": shell_chrome_string(key, is_de), "sortable": true }) }).collect()
}

/// 🎬️ React's `taskManagerRowAction`: `args.actorId` is the row the action targets.
pub fn task_manager_row_action(kind: TaskManagerActionKind, actor_id: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: TASK_MANAGER_CONTROLLER_ID.into(), action: kind.tag().into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "actorId": actor_id }) }
}

fn metric_cell(value: Option<u64>) -> Value {
    value.map_or_else(|| json!({ "kind": "text", "value": TASK_MANAGER_UNOBSERVED_METRIC }), |value| json!({ "kind": "number", "value": value }))
}

fn text_cell(value: &str) -> Value {
    json!({ "kind": "text", "value": value })
}

/// 🔖️ React's `taskManagerRows`: the `TableScene` row records, one per actor with its three action buttons.
pub fn task_manager_rows(rows: &[TaskManagerRow], is_de: bool) -> Vec<Value> {
    rows.iter()
        .map(|row| {
            let buttons: Vec<Value> = TaskManagerActionKind::ALL.iter().map(|kind| json!({ "iconId": kind.icon_id(), "label": kind.label(is_de), "action": { "controllerId": TASK_MANAGER_CONTROLLER_ID, "action": kind.tag(), "args": { "actorId": row.actor_id } } })).collect();
            json!({
                "id": row.actor_id,
                "actorId": text_cell(&row.actor_id),
                "packageId": text_cell(&row.package_id),
                "lane": text_cell(row.lane.label(is_de)),
                "status": text_cell(row.status.label(is_de)),
                "stage": text_cell(row.stage.tag()),
                "shard": metric_cell(row.shard),
                "wallUsP95": metric_cell(row.wall_us_p95),
                "mailboxLen": metric_cell(row.mailbox_len),
                "turns": metric_cell(row.turns),
                "traps": metric_cell(row.traps),
                "restarts": metric_cell(row.restarts),
                "actions": { "kind": "buttons", "buttons": buttons },
            })
        })
        .collect()
}

/// 🖼️ React's `buildTaskManagerTableScene`: the `{columnsJson, rowsJson}` pair of a `TableScene`.
pub fn build_task_manager_table_scene(rows: &[TaskManagerRow], is_de: bool) -> (String, String) {
    (Value::Array(task_manager_columns(is_de)).to_string(), Value::Array(task_manager_rows(rows, is_de)).to_string())
}

fn metric_text(value: Option<u64>) -> String {
    value.map_or_else(|| TASK_MANAGER_UNOBSERVED_METRIC.to_string(), |value| value.to_string())
}

/// 🧵️ The cells of one actor row as text, in column order up to (not including) `actions`.
pub fn task_manager_row_cells(row: &TaskManagerRow, is_de: bool) -> [String; 11] {
    [
        row.actor_id.clone(),
        row.package_id.clone(),
        row.lane.label(is_de).into(),
        row.status.label(is_de).into(),
        row.stage.tag().into(),
        metric_text(row.shard),
        metric_text(row.wall_us_p95),
        metric_text(row.mailbox_len),
        metric_text(row.turns),
        metric_text(row.traps),
        metric_text(row.restarts),
    ]
}
//#endregion ActorTable

//#region RunningTasks
/// 📈️ Progress a task states itself: `completed` of `total` (`None` = open-ended) in its own words.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagerTaskProgress {
    pub completed: u64,
    pub total: Option<u64>,
    pub text: String,
}

/// 🏃️ One running task as the window lists it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagerTask {
    pub id: String,
    pub lane: TaskManagerTaskLane,
    pub title: String,
    pub owner: String,
    pub started_at_ms: u64,
    pub steps: Option<u64>,
    pub progress: Option<TaskManagerTaskProgress>,
    pub suspendable: bool,
    pub state: TaskManagerTaskState,
}

/// 💼️ A spawned plugin job of the job ledger.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedJobRow {
    pub key: String,
    pub plugin_id: String,
    pub kind: String,
    pub steps: u64,
    pub started_at_ms: u64,
    pub cancelling: bool,
}

/// 🤖️ One row of the agent conversation; only a tool call still running or cancelling is a task.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentConversationRow {
    pub kind: String,
    pub id: String,
    pub tool_name: Option<String>,
    pub state: Option<String>,
    pub at_ms: u64,
}

/// ⏯️ One live tool run of a program, as its ToolRun panel states it.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagerToolRun {
    pub run: String,
    pub label: String,
    pub completed: u64,
    pub total: Option<u64>,
    pub value_text: String,
    pub paused: bool,
    #[serde(skip)]
    pub controls: TaskManagerToolRunControls,
}

/// ⏯️ The pause, resume and abort actions a run's panel group offers — React's `toolRunControlsRef` entry; `None` while the panel does not offer that control.
#[derive(Clone, Debug, Default)]
pub struct TaskManagerToolRunControls {
    pub pause: Option<ActionDescriptor>,
    pub resume: Option<ActionDescriptor>,
    pub abort: Option<ActionDescriptor>,
}

fn plain_task(id: String, lane: TaskManagerTaskLane, title: String, owner: String, started_at_ms: u64, state: TaskManagerTaskState) -> TaskManagerTask {
    TaskManagerTask { id, lane, title, owner, started_at_ms, steps: None, progress: None, suspendable: false, state }
}

/// 💼️ React's `spawnedJobTasksV1`.
pub fn spawned_job_tasks(rows: &[SpawnedJobRow]) -> Vec<TaskManagerTask> {
    rows.iter()
        .map(|row| TaskManagerTask {
            steps: Some(row.steps),
            ..plain_task(format!("job:{}", row.key), TaskManagerTaskLane::Job, row.kind.clone(), row.plugin_id.clone(), row.started_at_ms, if row.cancelling { TaskManagerTaskState::Cancelling } else { TaskManagerTaskState::Running })
        })
        .collect()
}

/// 🧩️ React's `installTasksV1`: timed from when the shell first saw the plugin installing, else `now_ms`.
pub fn install_tasks(plugin_ids: &[String], started_at_ms: &HashMap<String, u64>, now_ms: u64) -> Vec<TaskManagerTask> {
    plugin_ids.iter().map(|plugin_id| plain_task(format!("install:{plugin_id}"), TaskManagerTaskLane::Activation, plugin_id.clone(), String::new(), started_at_ms.get(plugin_id).copied().unwrap_or(now_ms), TaskManagerTaskState::Running)).collect()
}

/// 🤖️ React's `toolCallTasksV1`.
pub fn tool_call_tasks(conversation: &[AgentConversationRow]) -> Vec<TaskManagerTask> {
    conversation
        .iter()
        .filter(|entry| entry.kind == "toolCall" && matches!(entry.state.as_deref(), Some("running" | "cancelling")))
        .map(|entry| plain_task(format!("tool:{}", entry.id), TaskManagerTaskLane::ToolCall, entry.tool_name.clone().unwrap_or_default(), String::new(), entry.at_ms, if entry.state.as_deref() == Some("cancelling") { TaskManagerTaskState::Cancelling } else { TaskManagerTaskState::Running }))
        .collect()
}

/// ⏯️ React's `taskManagerToolRunIdV1`.
pub fn task_manager_tool_run_id(spawned_id: &str, run: &str) -> String {
    format!("toolRun:{spawned_id}#{run}")
}

/// ⏯️ React's `toolRunTasksV1`: suspendable, suspended while the run is paused.
pub fn tool_run_tasks(runs: &[TaskManagerToolRun], plugin_id: &str, spawned_id: &str, started_at_ms: &HashMap<String, u64>, now_ms: u64) -> Vec<TaskManagerTask> {
    runs.iter()
        .map(|run| {
            let id = task_manager_tool_run_id(spawned_id, &run.run);
            let started = started_at_ms.get(&id).copied().unwrap_or(now_ms);
            TaskManagerTask {
                progress: Some(TaskManagerTaskProgress { completed: run.completed, total: run.total, text: run.value_text.clone() }),
                suspendable: true,
                ..plain_task(id, TaskManagerTaskLane::ToolRun, run.label.clone(), plugin_id.into(), started, if run.paused { TaskManagerTaskState::Suspended } else { TaskManagerTaskState::Running })
            }
        })
        .collect()
}

/// ⏱️ Whole seconds a task has run — never negative, whatever the clocks say.
pub fn task_manager_elapsed_seconds(started_at_ms: u64, now_ms: u64) -> u64 {
    now_ms.saturating_sub(started_at_ms) / 1000
}

/// 📈️ The task's progress line: its own text plus elapsed time, steps plus elapsed time, or elapsed time alone.
pub fn task_manager_progress_text(task: &TaskManagerTask, now_ms: u64, is_de: bool) -> String {
    let seconds = task_manager_elapsed_seconds(task.started_at_ms, now_ms).to_string();
    let elapsed = shell_chrome_format("taskManager.tasks.progressElapsed", is_de, &[("seconds", &seconds)]);
    match (&task.progress, task.steps) {
        (Some(progress), _) => format!("{} · {elapsed}", progress.text),
        (None, None) => elapsed,
        (None, Some(steps)) => shell_chrome_format("taskManager.tasks.progressSteps", is_de, &[("steps", &steps.to_string()), ("seconds", &seconds)]),
    }
}

/// 📊️ The determinate percent of a task that states a positive total; `None` is an indeterminate bar.
pub fn task_manager_determinate_percent(task: &TaskManagerTask) -> Option<u64> {
    let progress = task.progress.as_ref()?;
    let total = progress.total.filter(|total| *total > 0)?;
    Some((((progress.completed as f64 / total as f64) * 100.0).round() as u64).min(100))
}

/// 🎛️ What a task or actor control asks of the host, queued until the runtime bridge drains it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskManagerRequest {
    pub kind: TaskManagerActionKind,
    pub actor_id: Option<String>,
    pub task_id: Option<String>,
}
//#endregion RunningTasks

//#region Panel
fn stack(direction: &str, id: &str, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: direction.into(), gap: None, padding: None, id: Some(id.into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
}

/// 📊️ React's actor `Table` as the retained node: eleven text columns plus the trailing actions column, one row per actor
/// (row id = actor id) whose Suspend/Resume/Cancel are row actions on one `{actorId}` target.
pub fn task_manager_table_node(rows: &[TaskManagerRow], is_de: bool) -> UiNode {
    let columns = COLUMNS.iter().filter(|(id, _)| *id != "actions").map(|(_, key)| Label::data(shell_chrome_string(key, is_de))).collect();
    let rows = rows
        .iter()
        .map(|row| semio_ui_declaration::ui::UiTableRowNode {
            id: row.actor_id.clone(),
            cells: task_manager_row_cells(row, is_de).into(),
            action: None,
            actions: TaskManagerActionKind::ALL
                .iter()
                .map(|kind| semio_ui_declaration::ui::UiTreeItemAction { icon_id: kind.icon(), label: Some(Label::data(kind.label(is_de))), action: task_manager_row_action(*kind, &row.actor_id), placement: None, disabled: false, reason: None })
                .collect(),
            presence: UiPresence::default(),
        })
        .collect();
    UiNode::Table(semio_ui_declaration::ui::UiTableNode {
        id: Some("os.task-manager.actors.table".into()),
        label: Label::data(shell_chrome_string("taskManager.actorsTitle", is_de)),
        columns,
        actions_label: Some(Label::data(shell_chrome_string("taskManager.columns.actions", is_de))),
        rows,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn action_button(id: String, kind: TaskManagerActionKind, label: String, action: ActionDescriptor) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id), icon_id: kind.icon(), label: Label::data(label), action, style: None, presence: UiPresence::default(), menu: None })
}

fn task_action(kind: TaskManagerActionKind, task_id: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: TASK_MANAGER_CONTROLLER_ID.into(), action: kind.tag().into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "task": task_id }) }
}

impl OpeningState {
    /// 📈️ Replaces the actor feed: `None` is React's absent registry ("no runtime is attached"), `Some` the latest `os.runtime.metrics` snapshot.
    pub fn set_runtime_actors(&mut self, actors: Option<Vec<RuntimeActorSample>>) {
        self.runtime_actors = actors;
    }

    /// 🏃️ Replaces the runtime-owned running tasks (jobs, installations, tool calls, tool runs).
    pub fn set_runtime_tasks(&mut self, tasks: Vec<TaskManagerTask>) {
        self.runtime_tasks = tasks;
    }

    /// 🧩️ Refreshes the runtime tasks from the shell's live sources — today the plugin install in flight (`installing`, its plugin id) — keeping the start time of every task already listed so elapsed seconds survive a rebuild.
    pub fn sync_runtime_install_tasks(&mut self, installing: Option<&str>, now_ms: u64) {
        let ids: Vec<String> = installing.map(str::to_string).into_iter().collect();
        let started: HashMap<String, u64> = self.runtime_tasks.iter().map(|task| (task.id.clone(), task.started_at_ms)).collect();
        #[cfg(not(target_arch = "wasm32"))]
        let jobs: Vec<SpawnedJobRow> = {
            let plugin_of: HashMap<String, String> = semio_renderer_wgpu_host_base::observed_actors().unwrap_or_default().into_iter().map(|actor| (actor.actor_id, actor.plugin_id)).collect();
            semio_renderer_wgpu_host_base::spawned_jobs_snapshot().into_iter().map(|row| SpawnedJobRow { plugin_id: plugin_of.get(&row.job.actor_id).cloned().unwrap_or(row.job.plugin_id), key: row.job.key, kind: row.job.kind, steps: row.steps, started_at_ms: row.job.started_at_ms, cancelling: row.cancelling }).collect()
        };
        #[cfg(target_arch = "wasm32")]
        let jobs: Vec<SpawnedJobRow> = Vec::new();
        self.runtime_tasks = install_tasks(&ids, &started, now_ms).into_iter().chain(spawned_job_tasks(&jobs)).chain(tool_run_tasks(&self.tool_runs, &self.tool_run_owner, "", &started, now_ms)).collect();
    }

    /// ⏯️ Replaces the focused program's live tool runs, read from its ToolRun panel (React's `toolRunPanelTasksV1` feed). `true` when the listing changed.
    pub fn set_tool_runs(&mut self, owner: &str, runs: Vec<TaskManagerToolRun>) -> bool {
        let changed = self.tool_run_owner != owner || self.tool_runs.len() != runs.len() || self.tool_runs.iter().zip(&runs).any(|(a, b)| (&a.run, &a.label, a.completed, a.total, &a.value_text, a.paused) != (&b.run, &b.label, b.completed, b.total, &b.value_text, b.paused));
        self.tool_run_owner = owner.to_string();
        self.tool_runs = runs;
        changed
    }

    /// 💼️ Takes the queued Cancel presses of spawned plugin jobs as their ledger keys; the press reaches the job's own owner through the ledger, which only marks the row cancelling.
    pub fn take_job_cancel_requests(&mut self) -> Vec<String> {
        let (jobs, kept): (Vec<_>, Vec<_>) = core::mem::take(&mut self.runtime_requests).into_iter().partition(|request| request.kind == TaskManagerActionKind::Cancel && request.task_id.as_deref().is_some_and(|id| id.starts_with("job:")));
        self.runtime_requests = kept;
        jobs.into_iter().filter_map(|request| request.task_id?.strip_prefix("job:").map(str::to_string)).collect()
    }

    /// ⏯️ Takes the queued task presses that name a live tool run and answers each with the panel action its run offers — React's `taskManagerSuspendRef` and the run's abort: Suspend presses `pause`, Resume `resume`, Cancel `abort`. A press whose run no longer offers that control is dropped, as React drops a press on an ended run; every other task press stays queued.
    pub fn take_tool_run_control_actions(&mut self) -> Vec<ActionDescriptor> {
        let (runs, kept): (Vec<_>, Vec<_>) = core::mem::take(&mut self.runtime_requests).into_iter().partition(|request| request.task_id.as_deref().is_some_and(|id| id.starts_with("toolRun:")));
        self.runtime_requests = kept;
        runs.into_iter()
            .filter_map(|request| {
                let run = self.tool_runs.iter().find(|run| Some(task_manager_tool_run_id("", &run.run).as_str()) == request.task_id.as_deref())?;
                match request.kind {
                    TaskManagerActionKind::Suspend => run.controls.pause.clone(),
                    TaskManagerActionKind::Resume => run.controls.resume.clone(),
                    TaskManagerActionKind::Cancel => run.controls.abort.clone(),
                }
            })
            .collect()
    }

    /// 🎛️ Takes the queued Cancel presses of plugin installations — the one runtime the shell serves itself — leaving every other press queued for the runtime bridge. The result is how many install cancels were taken.
    pub fn take_install_cancel_requests(&mut self) -> usize {
        let before = self.runtime_requests.len();
        self.runtime_requests.retain(|request| !(request.kind == TaskManagerActionKind::Cancel && request.task_id.as_deref().is_some_and(|id| id.starts_with("install:"))));
        before - self.runtime_requests.len()
    }

    /// 📡️ Refreshes the actor feed from the kernel pool's published snapshot — React's `runtimeMetricsRowsV1` input; `None` while no kernel has published one.
    pub fn sync_runtime_actors(&mut self) {
        self.runtime_actors = semio_renderer_wgpu_host_base::observed_actors().map(|actors| actors.into_iter().map(|actor| RuntimeActorSample { actor_id: actor.actor_id, plugin_id: actor.plugin_id, resident: actor.resident, shard: actor.shard }).collect());
    }

    /// 🎛️ Hands every queued actor press to the kernel pool through the host-base actor observation port; task presses stay queued. The result is how many were forwarded.
    pub fn forward_actor_requests(&mut self) -> usize {
        use semio_renderer_wgpu_host_base::{ActorControl, ActorControlKind, request_actor_control};
        let (actors, kept): (Vec<_>, Vec<_>) = core::mem::take(&mut self.runtime_requests).into_iter().partition(|request| request.actor_id.is_some());
        self.runtime_requests = kept;
        let forwarded = actors.len();
        for request in actors {
            let kind = match request.kind {
                TaskManagerActionKind::Suspend => ActorControlKind::Suspend,
                TaskManagerActionKind::Resume => ActorControlKind::Resume,
                TaskManagerActionKind::Cancel => ActorControlKind::Cancel,
            };
            request_actor_control(ActorControl { kind, actor_id: request.actor_id.unwrap_or_default() });
        }
        forwarded
    }

    /// 🎛️ Queues the actor or task action a Task Manager control sent; `false` when the action is not one of React's three or names nothing.
    pub fn queue_task_manager_request(&mut self, action: &ActionDescriptor) -> bool {
        let Some(kind) = TaskManagerActionKind::from_tag(&action.action) else { return false };
        let args = action.args.as_ref().map(dsl_value_as_json);
        let field = |name: &str| args.as_ref().and_then(|args| args.get(name)).and_then(Value::as_str).map(str::to_string);
        let (actor_id, task_id) = (field("actorId"), field("task"));
        if actor_id.is_none() && task_id.is_none() {
            return false;
        }
        self.runtime_requests.push(TaskManagerRequest { kind, actor_id, task_id });
        true
    }

    /// 🎛️ Hands the queued requests to the runtime bridge, oldest first.
    pub fn take_task_manager_requests(&mut self) -> Vec<TaskManagerRequest> {
        core::mem::take(&mut self.runtime_requests)
    }

    /// 🏃️ React's `TaskManagerTaskRow` for one runtime task: title and state, lane and owner, progress bar, progress line and its Suspend/Resume/Cancel controls.
    pub fn runtime_task_nodes(&self, task: &TaskManagerTask, now_ms: u64, is_de: bool) -> Vec<UiNode> {
        let text = task_manager_progress_text(task, now_ms, is_de);
        let cancelling = task.state == TaskManagerTaskState::Cancelling;
        let suspended = task.state == TaskManagerTaskState::Suspended;
        let owner = if task.owner.is_empty() { String::new() } else { format!(" · {}", task.owner) };
        let mut children = vec![
            settings_text_row(&format!("{} · {}", task.title, task.state.label(is_de))),
            settings_text_row(&format!("{}{owner}", task.lane.label(is_de))),
            UiNode::Progress(semio_ui_declaration::ui::UiProgressNode {
                id: format!("os.task-manager.progress.{}", task.id),
                completed: task.progress.as_ref().map_or(task.steps.unwrap_or(0), |progress| progress.completed) as f64,
                total: task.progress.as_ref().and_then(|progress| progress.total).filter(|total| *total > 0).map(|total| total as f64),
                value_text: Label::data(text.clone()),
                presence: UiPresence::default(),
                menu: None,
            }),
            settings_text_row(&text),
        ];
        if task.suspendable && !cancelling {
            let kind = if suspended { TaskManagerActionKind::Resume } else { TaskManagerActionKind::Suspend };
            children.push(action_button(format!("os.task-manager.{}.{}", kind.tag(), task.id), kind, kind.task_label(is_de, &task.title), task_action(kind, &task.id)));
        }
        if !cancelling {
            let kind = TaskManagerActionKind::Cancel;
            children.push(action_button(format!("os.task-manager.cancel.{}", task.id), kind, kind.task_label(is_de, &task.title), task_action(kind, &task.id)));
        }
        vec![stack("column", &format!("os.task-manager.task.{}", task.id), children)]
    }

    /// 🧵️ React's actors section: its title above the actor `TableScene` (twelve columns, one row per actor with Suspend/Resume/Cancel) the generic Table surface paints — or the distinct `no-runtime` / `no-actors` notice.
    pub fn task_manager_actor_nodes(&self, is_de: bool) -> UiNode {
        let title = settings_text_row(shell_chrome_string("taskManager.actorsTitle", is_de));
        let Some(actors) = &self.runtime_actors else {
            return stack("column", "os.task-manager.actors", vec![title, stack("column", "os.task-manager.no-runtime", vec![settings_text_row(shell_chrome_string("taskManager.noRuntime", is_de))])]);
        };
        let rows = runtime_metrics_rows(actors);
        if rows.is_empty() {
            return stack("column", "os.task-manager.actors", vec![title, stack("column", "os.task-manager.no-actors", vec![settings_text_row(shell_chrome_string("taskManager.empty", is_de))])]);
        }
        stack("column", "os.task-manager.actors", vec![title, task_manager_table_node(&rows, is_de)])
    }
}
//#endregion Panel
