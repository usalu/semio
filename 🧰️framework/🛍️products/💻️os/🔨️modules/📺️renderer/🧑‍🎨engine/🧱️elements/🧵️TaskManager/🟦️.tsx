// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🧵️TaskManager/component.tsx
/** @emoji 🧵️ `🧵️TaskManager` — the design of record's `🧵️task-manager` pane: one row per LIVE actor
 * (id, package, lane, status/failure stage, p95 wall time, mailbox length, shard, turns, traps,
 * restarts) with suspend/resume/cancel actions. Ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
 * RUNTIME` packet T1.
 *
 * Dual-rendering (React + wgpu) is achieved by NOT hand-writing either renderer's draw code: this
 * module mints the generic `TableScene` JSON shape (`columnsJson`/`rowsJson`, "buttons" cells) that
 * `Table/🟦️.tsx`'s `TableHost` already parses on the React side, and that the wgpu
 * `🟦️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` already dispatches generically via `ui_wgpu::wgpu::SurfaceKind::Table`
 * (one of the 11 "generic fallback" surface kinds — see that file's own
 * `scene_command_reaches_every_generic_fallback_surface_kind_without_panicking` test). Studied both
 * `Table/🟦️.tsx` and `🟦️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` before choosing this: reusing an already
 * dual-rendered surface kind is "follow the exact structure," not "invent a new pattern" — a
 * bespoke `SurfaceKind::TaskManager` would need a new WIT/Rust/TS variant registered in `ui_wgpu` and
 * dispatched from `🟦️Interpreter`, both outside this packet's `path_scope` (a NEW module directory
 * only). `TableColumnRecord`/`TableRowRecord`/`TableCellButton` are private to `Table/🟦️.tsx`
 * (not exported), so this file re-states their JSON shape by hand rather than importing it — see
 * `📓️terra-T1-report.md` for the byte-shape this was checked against.
 *
 * ✅️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit
 * `📓️g5-ux-completeness-audit.md` ranked item 1) closed the mount half of that punch-list: the pane is
 * registered as the `os.task-manager` chrome panel (`📌️ChromePanels/🟦️.tsx`'s
 * `createFrameworkTaskManagerPanelTab`), docked by `🏛️ShellHost`, and reachable from the command
 * palette through the `os.openTaskManager` os command. `useRuntimeMetricsRows` below is the live feed:
 * it subscribes to `ActivationRegistry.metricsBus`'s `os.runtime.metrics` topic — the publisher that
 * until now had no consumer anywhere in the codebase.
 *
 * ✅️ Slice U5 (session 11) made the window say what is RUNNING, not only which actors exist: the React shell's
 * plugin runtime does own an `ActivationRegistry` (`pluginRuntimeActivationRegistryV1`), the window itself starts
 * that registry's metrics publisher for as long as it is mounted (the deliberate consumer-side choice
 * `autoStartMetricsPublisher`'s doc asks for), and `TaskManagerTasksPanel` lists every live spawned plugin job,
 * plugin installation and agent tool call with its progress and a cancel control (`TaskManagerSourcesV1`).
 *
 * `TaskManagerPanel`'s three row actions are REAL on web: `createTaskManagerDispatcher` (region
 * `🔖️LiveDispatch` below) routes them through `ActivationRegistry.suspend`/`resume`/`cancel`, which
 * call straight through to a real `ShardClient` — not stubbed. K1 (sibling packet) landed the native
 * counterpart (`ShardLoop::pump` now really dispatches `Payload::Suspend`/`Resume`/`Cancel`); that
 * side stays unreachable for the same reason the metrics publisher does (no live `Kernel` thread on
 * native yet) — see `🔖️LiveDispatch`'s own doc comment and the report's `## honest gaps`.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useState, useSyncExternalStore, type ReactElement } from "react";
import { Button, Table, registerUiTranslationBundles, useLabel, type IconName, type TableColumn } from "@semio-tech/ui-react";
import { type ActionDescriptor } from "@semio-tech/framework";
import { type ActivationRegistry, type RuntimeMetricsSnapshot } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { type SpawnedJobRowV1 } from "../🔌️PluginRuntime/💼️job-ledger/🟦️.ts";
import { type AgentConversationEntry } from "../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Types
/** 🛣️ Mirrors `semio_framework_actor::Lane`'s four camelCase tags. */
export type TaskManagerLane = "interactive" | "userVisible" | "background" | "maintenance";

/** 🗂️ Mirrors `semio_framework_actor::ActorStatus`'s tags (the `kind` discriminant). */
export type TaskManagerStatus = "cold" | "activating" | "active" | "suspended" | "draining" | "trapped" | "quarantined" | "disabled";

/** 🚑️ Mirrors `semio_framework_actor::FailureStage`'s tags. */
export type TaskManagerStage = "healthy" | "warned" | "throttled" | "trapped" | "quarantined" | "disabled" | "cancelled";

/** 🧵️ One live-actor row — field-compatible with `semio_framework_actor::ActorMetricsSample` joined
 * with its own `ActorMetrics` (`wallUsP95` is that type's `wall_us_p95()`).
 *
 * 🚧️ Every counter is `number | null` because a web `ActivationRegistry` genuinely cannot observe
 * some of them: it delegates straight to `ShardClient` and never holds a `Kernel`, so `turns`/`traps`/
 * `wallUsP95`/`mailboxLen`/`restarts` have no source on that side of the boundary
 * (`🎠️kernel/🟦️.ts`'s `runtimeMetricsActorRows` says the same). `null` renders as an em dash — an
 * honest "not observable here", never a silent zero that reads as "nothing has happened". */
export interface TaskManagerRow {
  readonly actorId: string;
  readonly packageId: string;
  readonly lane: TaskManagerLane;
  readonly status: TaskManagerStatus;
  readonly stage: TaskManagerStage;
  readonly shard: number | null;
  readonly wallUsP95: number | null;
  readonly mailboxLen: number | null;
  readonly turns: number | null;
  readonly traps: number | null;
  readonly restarts: number | null;
}

/** 🚧️ What an unobservable counter reads as, in every locale — a typographic dash, not a word. */
export const TASK_MANAGER_UNOBSERVED_METRIC = "—";

/** 🎬️ The three actions routed through `Kernel::suspend`/`Kernel::resume`/the shard-loop's
 * `Payload::Cancel` — see this file's header doc for what is (and isn't) wired yet. */
export type TaskManagerActionKind = "suspend" | "resume" | "cancel";

/** 🌐️ Every user-facing string this module needs, pre-resolved — kept out of the pure builder
 * functions below so they stay React-free and directly unit-testable (`useTaskManagerLabels` is the
 * only place `useLabel` is called). */
export interface TaskManagerLabels {
  readonly columns: {
    readonly actorId: string;
    readonly packageId: string;
    readonly lane: string;
    readonly status: string;
    readonly stage: string;
    readonly shard: string;
    readonly wallUsP95: string;
    readonly mailboxLen: string;
    readonly turns: string;
    readonly traps: string;
    readonly restarts: string;
    readonly actions: string;
  };
  readonly lanes: Record<TaskManagerLane, string>;
  readonly statuses: Record<TaskManagerStatus, string>;
  readonly actions: Record<TaskManagerActionKind, string>;
  /** 🈳️ Shown instead of a table when the registry reports no actor at all. */
  readonly empty: string;
  /** 🈳️ Shown when no `ActivationRegistry` is attached to this shell — a different fact from "no
   * actors", and the human deserves to be told which one it is. */
  readonly noRuntime: string;
}
//#endregion 🔖️Types

//#region 🌐️Labels
export const taskManagerUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      os: {
        taskManager: {
          columns: {
            actorId: { label: { normal: "Actor", beginner: "Actor" } },
            packageId: { label: { normal: "Package", beginner: "Package" } },
            lane: { label: { normal: "Lane", beginner: "Priority" } },
            status: { label: { normal: "Status", beginner: "Status" } },
            stage: { label: { normal: "Stage", beginner: "Health" } },
            shard: { label: { normal: "Shard", beginner: "Shard" } },
            wallUsP95: { label: { normal: "p95 wall (µs)", beginner: "Typical time" } },
            mailboxLen: { label: { normal: "Mailbox", beginner: "Queued work" } },
            turns: { label: { normal: "Turns", beginner: "Turns" } },
            traps: { label: { normal: "Traps", beginner: "Errors" } },
            restarts: { label: { normal: "Restarts", beginner: "Restarts" } },
            actions: { label: { normal: "Actions", beginner: "Actions" } },
          },
          lanes: {
            interactive: { label: { normal: "Interactive", beginner: "Interactive" } },
            userVisible: { label: { normal: "User-visible", beginner: "Visible" } },
            background: { label: { normal: "Background", beginner: "Background" } },
            maintenance: { label: { normal: "Maintenance", beginner: "Maintenance" } },
          },
          statuses: {
            cold: { label: { normal: "Cold", beginner: "Not started" } },
            activating: { label: { normal: "Activating", beginner: "Starting…" } },
            active: { label: { normal: "Active", beginner: "Running" } },
            suspended: { label: { normal: "Suspended", beginner: "Paused" } },
            draining: { label: { normal: "Draining", beginner: "Stopping…" } },
            trapped: { label: { normal: "Trapped", beginner: "Crashed" } },
            quarantined: { label: { normal: "Quarantined", beginner: "Blocked" } },
            disabled: { label: { normal: "Disabled", beginner: "Disabled" } },
          },
          actions: {
            suspend: { label: { normal: "Suspend", beginner: "Pause" } },
            resume: { label: { normal: "Resume", beginner: "Resume" } },
            cancel: { label: { normal: "Cancel", beginner: "Stop" } },
          },
          empty: { label: { normal: "No actors are running.", beginner: "Nothing is running right now." } },
          noRuntime: { label: { normal: "No actor runtime is attached to this shell.", beginner: "This window has nothing to watch yet." } },
          actorsTitle: { label: { normal: "Actors", beginner: "Running programs" } },
          tasks: {
            title: { label: { normal: "Running tasks", beginner: "Work in progress" } },
            empty: { label: { normal: "No task is running.", beginner: "Nothing is being worked on right now." } },
            lanes: {
              job: { label: { normal: "Plugin job", beginner: "Background work" } },
              activation: { label: { normal: "Plugin installation", beginner: "Installing" } },
              toolCall: { label: { normal: "Agent tool call", beginner: "Assistant action" } },
              toolRun: { label: { normal: "Tool run", beginner: "Tool at work" } },
            },
            running: { label: { normal: "Running", beginner: "Working" } },
            suspended: { label: { normal: "Suspended", beginner: "Paused" } },
            cancelling: { label: { normal: "Cancelling…", beginner: "Stopping…" } },
            suspend: { label: { normal: "Suspend", beginner: "Pause" } },
            suspendTask: { label: { normal: "Suspend {{task}}", beginner: "Pause {{task}}" } },
            resume: { label: { normal: "Resume", beginner: "Continue" } },
            resumeTask: { label: { normal: "Resume {{task}}", beginner: "Continue {{task}}" } },
            cancel: { label: { normal: "Cancel", beginner: "Stop" } },
            cancelTask: { label: { normal: "Cancel {{task}}", beginner: "Stop {{task}}" } },
            progressSteps: { label: { normal: "{{steps}} steps · {{seconds}} s", beginner: "{{steps}} steps done · {{seconds}} s" } },
            progressElapsed: { label: { normal: "{{seconds}} s", beginner: "{{seconds}} s so far" } },
            progressLabel: { label: { normal: "Progress of {{task}}", beginner: "How far {{task}} is" } },
          },
        },
      },
    },
  },
  de: {
    translation: {
      os: {
        taskManager: {
          columns: {
            actorId: { label: { normal: "Akteur", beginner: "Akteur" } },
            packageId: { label: { normal: "Paket", beginner: "Paket" } },
            lane: { label: { normal: "Spur", beginner: "Priorität" } },
            status: { label: { normal: "Status", beginner: "Status" } },
            stage: { label: { normal: "Zustand", beginner: "Gesundheit" } },
            shard: { label: { normal: "Shard", beginner: "Shard" } },
            wallUsP95: { label: { normal: "p95 Zeit (µs)", beginner: "Typische Zeit" } },
            mailboxLen: { label: { normal: "Postfach", beginner: "Warteschlange" } },
            turns: { label: { normal: "Züge", beginner: "Züge" } },
            traps: { label: { normal: "Abstürze", beginner: "Fehler" } },
            restarts: { label: { normal: "Neustarts", beginner: "Neustarts" } },
            actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
          },
          lanes: {
            interactive: { label: { normal: "Interaktiv", beginner: "Interaktiv" } },
            userVisible: { label: { normal: "Sichtbar", beginner: "Sichtbar" } },
            background: { label: { normal: "Hintergrund", beginner: "Hintergrund" } },
            maintenance: { label: { normal: "Wartung", beginner: "Wartung" } },
          },
          statuses: {
            cold: { label: { normal: "Kalt", beginner: "Nicht gestartet" } },
            activating: { label: { normal: "Aktiviert…", beginner: "Startet…" } },
            active: { label: { normal: "Aktiv", beginner: "Läuft" } },
            suspended: { label: { normal: "Angehalten", beginner: "Pausiert" } },
            draining: { label: { normal: "Wird beendet…", beginner: "Stoppt…" } },
            trapped: { label: { normal: "Abgestürzt", beginner: "Abgestürzt" } },
            quarantined: { label: { normal: "Isoliert", beginner: "Blockiert" } },
            disabled: { label: { normal: "Deaktiviert", beginner: "Deaktiviert" } },
          },
          actions: {
            suspend: { label: { normal: "Anhalten", beginner: "Pause" } },
            resume: { label: { normal: "Fortsetzen", beginner: "Fortsetzen" } },
            cancel: { label: { normal: "Abbrechen", beginner: "Stopp" } },
          },
          empty: { label: { normal: "Es laufen keine Akteure.", beginner: "Im Moment läuft nichts." } },
          noRuntime: { label: { normal: "Mit dieser Shell ist keine Akteur-Laufzeit verbunden.", beginner: "Dieses Fenster hat noch nichts zu beobachten." } },
          actorsTitle: { label: { normal: "Akteure", beginner: "Laufende Programme" } },
          tasks: {
            title: { label: { normal: "Laufende Aufgaben", beginner: "Arbeit in Bearbeitung" } },
            empty: { label: { normal: "Es läuft keine Aufgabe.", beginner: "Im Moment wird an nichts gearbeitet." } },
            lanes: {
              job: { label: { normal: "Plugin-Auftrag", beginner: "Hintergrundarbeit" } },
              activation: { label: { normal: "Plugin-Installation", beginner: "Wird installiert" } },
              toolCall: { label: { normal: "Werkzeugaufruf des Agenten", beginner: "Aktion des Assistenten" } },
              toolRun: { label: { normal: "Werkzeuglauf", beginner: "Werkzeug arbeitet" } },
            },
            running: { label: { normal: "Läuft", beginner: "In Arbeit" } },
            suspended: { label: { normal: "Angehalten", beginner: "Pausiert" } },
            cancelling: { label: { normal: "Wird abgebrochen…", beginner: "Wird gestoppt…" } },
            suspend: { label: { normal: "Anhalten", beginner: "Pause" } },
            suspendTask: { label: { normal: "{{task}} anhalten", beginner: "{{task}} pausieren" } },
            resume: { label: { normal: "Fortsetzen", beginner: "Weiter" } },
            resumeTask: { label: { normal: "{{task}} fortsetzen", beginner: "{{task}} weitermachen" } },
            cancel: { label: { normal: "Abbrechen", beginner: "Stoppen" } },
            cancelTask: { label: { normal: "{{task}} abbrechen", beginner: "{{task}} stoppen" } },
            progressSteps: { label: { normal: "{{steps}} Schritte · {{seconds}} s", beginner: "{{steps}} Schritte erledigt · {{seconds}} s" } },
            progressElapsed: { label: { normal: "{{seconds}} s", beginner: "bisher {{seconds}} s" } },
            progressLabel: { label: { normal: "Fortschritt von {{task}}", beginner: "Wie weit {{task}} ist" } },
          },
        },
      },
    },
  },
});

/** 🌐️ Resolves every `TaskManagerLabels` string via `useLabel` — the only place in this module that
 * needs a React render context; every builder below takes the resolved bundle as a plain argument. */
export function useTaskManagerLabels(): TaskManagerLabels {
  return {
    columns: {
      actorId: useLabel(taskManagerUiLabel("os.taskManager.columns.actorId")),
      packageId: useLabel(taskManagerUiLabel("os.taskManager.columns.packageId")),
      lane: useLabel(taskManagerUiLabel("os.taskManager.columns.lane")),
      status: useLabel(taskManagerUiLabel("os.taskManager.columns.status")),
      stage: useLabel(taskManagerUiLabel("os.taskManager.columns.stage")),
      shard: useLabel(taskManagerUiLabel("os.taskManager.columns.shard")),
      wallUsP95: useLabel(taskManagerUiLabel("os.taskManager.columns.wallUsP95")),
      mailboxLen: useLabel(taskManagerUiLabel("os.taskManager.columns.mailboxLen")),
      turns: useLabel(taskManagerUiLabel("os.taskManager.columns.turns")),
      traps: useLabel(taskManagerUiLabel("os.taskManager.columns.traps")),
      restarts: useLabel(taskManagerUiLabel("os.taskManager.columns.restarts")),
      actions: useLabel(taskManagerUiLabel("os.taskManager.columns.actions")),
    },
    lanes: {
      interactive: useLabel(taskManagerUiLabel("os.taskManager.lanes.interactive")),
      userVisible: useLabel(taskManagerUiLabel("os.taskManager.lanes.userVisible")),
      background: useLabel(taskManagerUiLabel("os.taskManager.lanes.background")),
      maintenance: useLabel(taskManagerUiLabel("os.taskManager.lanes.maintenance")),
    },
    statuses: {
      cold: useLabel(taskManagerUiLabel("os.taskManager.statuses.cold")),
      activating: useLabel(taskManagerUiLabel("os.taskManager.statuses.activating")),
      active: useLabel(taskManagerUiLabel("os.taskManager.statuses.active")),
      suspended: useLabel(taskManagerUiLabel("os.taskManager.statuses.suspended")),
      draining: useLabel(taskManagerUiLabel("os.taskManager.statuses.draining")),
      trapped: useLabel(taskManagerUiLabel("os.taskManager.statuses.trapped")),
      quarantined: useLabel(taskManagerUiLabel("os.taskManager.statuses.quarantined")),
      disabled: useLabel(taskManagerUiLabel("os.taskManager.statuses.disabled")),
    },
    actions: {
      suspend: useLabel(taskManagerUiLabel("os.taskManager.actions.suspend")),
      resume: useLabel(taskManagerUiLabel("os.taskManager.actions.resume")),
      cancel: useLabel(taskManagerUiLabel("os.taskManager.actions.cancel")),
    },
    empty: useLabel(taskManagerUiLabel("os.taskManager.empty")),
    noRuntime: useLabel(taskManagerUiLabel("os.taskManager.noRuntime")),
  };
}
//#endregion 🌐️Labels

//#region 🔖️TableSceneShape
/** 🔖️ Hand-stated mirror of `Table/🟦️.tsx`'s private `TableColumnRecord` — see this file's
 * header doc for why it isn't imported. */
export interface TaskManagerTableColumn {
  readonly id: string;
  readonly label: string;
  readonly sortable?: boolean;
}

/** 🔖️ Hand-stated mirror of `Table/🟦️.tsx`'s private `TableCellButton`. */
export interface TaskManagerTableButton {
  readonly iconId: IconName;
  readonly label?: string;
  readonly action: ActionDescriptor;
  readonly placement?: "row" | "menu";
}

/** 🔖️ Hand-stated mirror of `Table/🟦️.tsx`'s private `TableCellRecord`. */
export type TaskManagerTableCell = { readonly kind: "text"; readonly value: string } | { readonly kind: "number"; readonly value: number } | { readonly kind: "buttons"; readonly buttons: readonly TaskManagerTableButton[] };

/** 🔖️ Hand-stated mirror of `Table/🟦️.tsx`'s private `TableRowRecord` — `id` is what
 * `TableHost`'s `getRowId` falls back to, and is the ONE key carrying a bare string rather than a
 * cell (an intersection with `Record<string, TaskManagerTableCell>` would make `id` unsatisfiable). */
export type TaskManagerTableRow = { readonly id: string; readonly [column: string]: TaskManagerTableCell | string };

const TASK_MANAGER_CONTROLLER_ID = "os.task-manager";

/** 🎬️ One `ActionDescriptor` per row action — `args.actorId` is the row this action targets. Matches
 * `dispatchCellAction`'s own merge convention in `Table/🟦️.tsx` (base descriptor + a patch
 * merged into `args`), so a shared `onAction` dispatcher (this file's header doc: `Kernel::suspend`/
 * `resume`/the shard-loop's `Payload::Cancel`) sees the SAME shape every other table action does. */
export function taskManagerRowAction(kind: TaskManagerActionKind, actorId: string): ActionDescriptor {
  return { controllerId: TASK_MANAGER_CONTROLLER_ID, action: kind, args: { actorId } };
}

/** 🔖️ Column definitions, in display order — `id` matches the row's own cell keys below. */
export function taskManagerColumns(labels: TaskManagerLabels): readonly TaskManagerTableColumn[] {
  return [
    { id: "actorId", label: labels.columns.actorId, sortable: true },
    { id: "packageId", label: labels.columns.packageId, sortable: true },
    { id: "lane", label: labels.columns.lane, sortable: true },
    { id: "status", label: labels.columns.status, sortable: true },
    { id: "stage", label: labels.columns.stage, sortable: true },
    { id: "shard", label: labels.columns.shard, sortable: true },
    { id: "wallUsP95", label: labels.columns.wallUsP95, sortable: true },
    { id: "mailboxLen", label: labels.columns.mailboxLen, sortable: true },
    { id: "turns", label: labels.columns.turns, sortable: true },
    { id: "traps", label: labels.columns.traps, sortable: true },
    { id: "restarts", label: labels.columns.restarts, sortable: true },
    { id: "actions", label: labels.columns.actions },
  ];
}

/** 🔖️ One `TaskManagerTableRow` per live actor — `actions` is a `"buttons"` cell carrying all three
 * `taskManagerRowAction`s, each with its own accessible `label` (rendered as the button's `title` by
 * `Table/🟦️.tsx`'s `renderTableCell`, the same convention every other button cell in that
 * file already relies on for its accessible name). */
/** 🚧️ A counter cell that tells the truth about an unobservable metric — see {@link TaskManagerRow}. */
export function taskManagerMetricCell(value: number | null): TaskManagerTableCell {
  return value === null ? { kind: "text", value: TASK_MANAGER_UNOBSERVED_METRIC } : { kind: "number", value };
}

export function taskManagerRows(rows: readonly TaskManagerRow[], labels: TaskManagerLabels): readonly TaskManagerTableRow[] {
  return rows.map((row) => ({
    id: row.actorId,
    actorId: { kind: "text", value: row.actorId },
    packageId: { kind: "text", value: row.packageId },
    lane: { kind: "text", value: labels.lanes[row.lane] },
    status: { kind: "text", value: labels.statuses[row.status] },
    stage: { kind: "text", value: row.stage },
    shard: taskManagerMetricCell(row.shard),
    wallUsP95: taskManagerMetricCell(row.wallUsP95),
    mailboxLen: taskManagerMetricCell(row.mailboxLen),
    turns: taskManagerMetricCell(row.turns),
    traps: taskManagerMetricCell(row.traps),
    restarts: taskManagerMetricCell(row.restarts),
    actions: {
      kind: "buttons",
      buttons: [
        { iconId: "pause", label: labels.actions.suspend, action: taskManagerRowAction("suspend", row.actorId) },
        { iconId: "play", label: labels.actions.resume, action: taskManagerRowAction("resume", row.actorId) },
        { iconId: "square", label: labels.actions.cancel, action: taskManagerRowAction("cancel", row.actorId) },
      ],
    },
  }));
}

/** 🖼️ The exact `{columnsJson, rowsJson}` pair a `TableScene` (`Table/🟦️.tsx`'s `scene.table`,
 * `ui_wgpu::wgpu::SurfaceKind::Table` on the wgpu side) needs — both renderers already parse this
 * shape generically, so committing it as a window's scene patch (`Kernel::apply_scene_patch`, host-
 * side, not yet wired — see this file's header doc) is the only remaining step for this pane to
 * render live in both backends. */
export function buildTaskManagerTableScene(rows: readonly TaskManagerRow[], labels: TaskManagerLabels): { readonly columnsJson: string; readonly rowsJson: string } {
  return { columnsJson: JSON.stringify(taskManagerColumns(labels)), rowsJson: JSON.stringify(taskManagerRows(rows, labels)) };
}
//#endregion 🔖️TableSceneShape

//#region 🔖️TaskManagerPanel
export interface TaskManagerPanelProps {
  readonly rows: readonly TaskManagerRow[];
  readonly onAction: (action: TaskManagerActionKind, actorId: string) => void | Promise<void>;
  /** 🈳️ `false` distinguishes "no runtime is attached to this shell" from "the runtime reports no
   * actors" — two very different facts that an empty table alone would conflate. Defaults to `true`. */
  readonly runtimeAttached?: boolean;
}

/** 🚧️ A counter's cell text — the em dash for an unobservable metric, so the React table and the
 * wgpu `TableScene` say the identical thing. */
function metricText(value: number | null): string {
  return value === null ? TASK_MANAGER_UNOBSERVED_METRIC : String(value);
}

/** @emoji 🧵️ Directly-mountable React view of `TaskManagerRow[]` — for a standalone dialog/pane
 * context (same "typed props in, typed callback out" shape as `🤖️AgentApprovals`), independent of the
 * scene-commit path above. Renders through the SAME `@semio-tech/ui-react` `Table` primitive
 * `Table/🟦️.tsx`'s `TableHost` uses, so it inherits that component's table semantics/
 * keyboard navigation rather than a bespoke one. */
export function TaskManagerPanel({ rows, onAction, runtimeAttached = true }: TaskManagerPanelProps): ReactElement {
  const labels = useTaskManagerLabels();
  const actorsTitle = useLabel(taskManagerUiLabel("os.taskManager.actorsTitle"));
  const columns: TableColumn<TaskManagerRow>[] = [
    { id: "actorId", header: labels.columns.actorId, accessor: (row) => row.actorId, sortable: true },
    { id: "packageId", header: labels.columns.packageId, accessor: (row) => row.packageId, sortable: true },
    { id: "lane", header: labels.columns.lane, accessor: (row) => labels.lanes[row.lane], sortable: true },
    { id: "status", header: labels.columns.status, accessor: (row) => labels.statuses[row.status], sortable: true },
    { id: "stage", header: labels.columns.stage, accessor: (row) => row.stage, sortable: true },
    { id: "shard", header: labels.columns.shard, accessor: (row) => metricText(row.shard), sortable: true },
    { id: "wallUsP95", header: labels.columns.wallUsP95, accessor: (row) => metricText(row.wallUsP95), sortable: true },
    { id: "mailboxLen", header: labels.columns.mailboxLen, accessor: (row) => metricText(row.mailboxLen), sortable: true },
    { id: "turns", header: labels.columns.turns, accessor: (row) => metricText(row.turns), sortable: true },
    { id: "traps", header: labels.columns.traps, accessor: (row) => metricText(row.traps), sortable: true },
    { id: "restarts", header: labels.columns.restarts, accessor: (row) => metricText(row.restarts), sortable: true },
    {
      id: "actions",
      header: labels.columns.actions,
      accessor: (row) => (
        <div className="flex items-center gap-1">
          <Button type="button" variant="outline" icon="pause" aria-label={`${labels.actions.suspend}: ${row.actorId}`} onClick={() => onAction("suspend", row.actorId)} />
          <Button type="button" variant="outline" icon="play" aria-label={`${labels.actions.resume}: ${row.actorId}`} onClick={() => onAction("resume", row.actorId)} />
          <Button type="button" variant="outline" icon="square" aria-label={`${labels.actions.cancel}: ${row.actorId}`} onClick={() => onAction("cancel", row.actorId)} />
        </div>
      ),
    },
  ];
  if (rows.length === 0) {
    return (
      <p role="status" data-semio-task-manager-empty={runtimeAttached ? "no-actors" : "no-runtime"} className="p-single text-xs text-muted-foreground">
        {runtimeAttached ? labels.empty : labels.noRuntime}
      </p>
    );
  }
  return (
    <div data-semio-task-manager="" role="region" aria-label={actorsTitle} tabIndex={0} className="min-h-0 min-w-0 overflow-x-auto">
      <Table columns={columns} data={[...rows]} getRowId={(row) => row.actorId} />
    </div>
  );
}
//#endregion 🔖️TaskManagerPanel

//#region 🔖️LiveFeed
/** 📈️ Maps one `os.runtime.metrics` snapshot onto the pane's rows. Residency is the only status a
 * web `ActivationRegistry` can state (`active` vs `suspended`); `lane`/`stage` and every counter it
 * cannot observe stay honest — `maintenance` is NOT guessed, it is the lane an unlabelled row is
 * scheduled on, and the counters are `null`. Pure, so a test drives it without a registry. */
export function runtimeMetricsRowsV1(snapshot: RuntimeMetricsSnapshot): readonly TaskManagerRow[] {
  return snapshot.actors.map((actor) => ({
    actorId: actor.actorId,
    packageId: actor.pluginId,
    lane: "maintenance",
    status: actor.resident ? "active" : "suspended",
    stage: "healthy",
    shard: actor.shard,
    wallUsP95: null,
    mailboxLen: null,
    turns: null,
    traps: null,
    restarts: null,
  }));
}

/** 📈️ The live feed: seeds from `runtimeMetricsSnapshot()` on mount and then follows
 * `ActivationRegistry.metricsBus`'s `os.runtime.metrics` events — the consumer that publisher never
 * had (`🎠️kernel/🟦️.ts`'s `startRuntimeMetricsPublisher` doc called its absence an honest gap).
 * A `null`/absent registry yields no rows, which {@link TaskManagerPanel} reports as "no runtime
 * attached" rather than as an empty runtime. */
export function useRuntimeMetricsRows(registry: ActivationRegistry | null | undefined): readonly TaskManagerRow[] {
  const [rows, setRows] = useState<readonly TaskManagerRow[]>([]);
  useEffect(() => {
    if (!registry) {
      setRows([]);
      return;
    }
    setRows(runtimeMetricsRowsV1(registry.runtimeMetricsSnapshot()));
    const listener = (event: Event) => {
      const snapshot = (event as CustomEvent<RuntimeMetricsSnapshot>).detail;
      if (snapshot) setRows(runtimeMetricsRowsV1(snapshot));
    };
    registry.metricsBus.addEventListener("os.runtime.metrics", listener);
    const stopPublisher = registry.startRuntimeMetricsPublisher((topic, snapshot) => registry.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot })));
    return () => {
      stopPublisher();
      registry.metricsBus.removeEventListener("os.runtime.metrics", listener);
    };
  }, [registry]);
  return rows;
}
//#endregion 🔖️LiveFeed

//#region 🔖️RunningTasks
/** 🛣️ Where a running task comes from: a guest's spawned job, a plugin installation, a tool call the connected agent
 * is executing, or a program's tool run (Fill, Generate, Reconstruct…). */
export type TaskManagerTaskLaneV1 = "job" | "activation" | "toolCall" | "toolRun";

/** 📈️ Progress a task states itself: `completed` of `total` (`null` = open-ended) in its own words. */
export interface TaskManagerTaskProgressV1 {
  readonly completed: number;
  readonly total: number | null;
  readonly text: string;
}

/** 🏃️ One running task as the window lists it. `steps` is the progress measure every spawned job has (admitted step
 * slices); `progress` is the progress a task reports itself (a tool run's stage and units); with neither, elapsed time
 * alone. `suspendable` tasks can be held and continued, not only stopped. */
export interface TaskManagerTaskV1 {
  readonly id: string;
  readonly lane: TaskManagerTaskLaneV1;
  readonly title: string;
  readonly owner: string;
  readonly startedAtMs: number;
  readonly steps: number | null;
  readonly progress: TaskManagerTaskProgressV1 | null;
  readonly suspendable: boolean;
  readonly state: "running" | "suspended" | "cancelling";
}

/** 🔌️ Where the window reads its live content from. Every member is a stable function the host hands in once,
 * so the window re-reads on `subscribe` notifications instead of being re-created by its dock tab. */
export interface TaskManagerSourcesV1 {
  readonly registry: () => ActivationRegistry | null;
  readonly tasks: () => readonly TaskManagerTaskV1[];
  readonly subscribe: (listener: () => void) => () => void;
  readonly cancel: (task: TaskManagerTaskV1) => void;
  readonly suspend: (task: TaskManagerTaskV1) => void;
  readonly resume: (task: TaskManagerTaskV1) => void;
}

/** 💼️ A spawned plugin job as a task: its kind is what it is doing, its plugin who asked for it. */
export function spawnedJobTasksV1(rows: readonly SpawnedJobRowV1[]): readonly TaskManagerTaskV1[] {
  return rows.map((row) => ({ id: `job:${row.key}`, lane: "job", title: row.kind, owner: row.pluginId, startedAtMs: row.startedAtMs, steps: row.steps, progress: null, suspendable: false, state: row.cancelling ? "cancelling" : "running" }));
}

/** 🧩️ Every plugin installation in flight as a task, timed from when the shell first saw it installing. */
export function installTasksV1(pluginIds: readonly string[], startedAtMs: ReadonlyMap<string, number>): readonly TaskManagerTaskV1[] {
  return pluginIds.map((pluginId) => ({ id: `install:${pluginId}`, lane: "activation", title: pluginId, owner: "", startedAtMs: startedAtMs.get(pluginId) ?? Date.now(), steps: null, progress: null, suspendable: false, state: "running" }));
}

/** 🤖️ Every agent tool call still running (or asked to stop) as a task. */
export function toolCallTasksV1(conversation: readonly AgentConversationEntry[]): readonly TaskManagerTaskV1[] {
  return conversation.flatMap((entry): TaskManagerTaskV1[] =>
    entry.kind === "toolCall" && (entry.state === "running" || entry.state === "cancelling") ? [{ id: `tool:${entry.id}`, lane: "toolCall", title: entry.toolName, owner: "", startedAtMs: entry.atMs, steps: null, progress: null, suspendable: false, state: entry.state }] : [],
  );
}

/** ⏯️ One live tool run of a program, as its ToolRun panel states it (`toolRunPanelTasksV1` in `🛠️ShellHelpers`). */
export interface TaskManagerToolRunV1 {
  readonly run: bigint;
  readonly label: string;
  readonly completed: number;
  readonly total: number | null;
  readonly valueText: string;
  readonly paused: boolean;
}

/** ⏯️ Every live tool run of the program `program` (`pluginId`, and its spawned id or `""` for the session's own app) as a
 * suspendable task — suspended while the run is paused — timed from when the shell first saw it. */
export function toolRunTasksV1(runs: readonly TaskManagerToolRunV1[], program: { readonly pluginId: string; readonly spawnedId: string }, startedAtMs: ReadonlyMap<string, number>): readonly TaskManagerTaskV1[] {
  return runs.map((run) => {
    const id = taskManagerToolRunIdV1(program.spawnedId, run.run);
    return { id, lane: "toolRun", title: run.label, owner: program.pluginId, startedAtMs: startedAtMs.get(id) ?? Date.now(), steps: null, progress: { completed: run.completed, total: run.total, text: run.valueText }, suspendable: true, state: run.paused ? "suspended" : "running" };
  });
}

/** ⏯️ The task id of a program's tool run — the program's spawned id (`""` for the session's own app) and the run id. */
export function taskManagerToolRunIdV1(spawnedId: string, run: bigint): string {
  return `toolRun:${spawnedId}#${run}`;
}

/** ⏱️ Whole seconds a task has run — never negative, whatever the clocks say. */
export function taskManagerElapsedSecondsV1(startedAtMs: number, nowMs: number): number {
  return Math.max(0, Math.floor((nowMs - startedAtMs) / 1000));
}

/** 🎛️ What a task row's controls ask of the host. */
export interface TaskManagerTaskControlsV1 {
  readonly onCancel: (task: TaskManagerTaskV1) => void;
  readonly onSuspend: (task: TaskManagerTaskV1) => void;
  readonly onResume: (task: TaskManagerTaskV1) => void;
}

function TaskManagerTaskRow({ task, nowMs, controls }: { readonly task: TaskManagerTaskV1; readonly nowMs: number; readonly controls: TaskManagerTaskControlsV1 }): ReactElement {
  const laneLabels: Readonly<Record<TaskManagerTaskLaneV1, string>> = {
    job: useLabel(taskManagerUiLabel("os.taskManager.tasks.lanes.job")),
    activation: useLabel(taskManagerUiLabel("os.taskManager.tasks.lanes.activation")),
    toolCall: useLabel(taskManagerUiLabel("os.taskManager.tasks.lanes.toolCall")),
    toolRun: useLabel(taskManagerUiLabel("os.taskManager.tasks.lanes.toolRun")),
  };
  const seconds = String(taskManagerElapsedSecondsV1(task.startedAtMs, nowMs));
  const withSteps = useLabel(taskManagerUiLabel("os.taskManager.tasks.progressSteps"), { steps: String(task.steps ?? 0), seconds });
  const elapsedOnly = useLabel(taskManagerUiLabel("os.taskManager.tasks.progressElapsed"), { seconds });
  const progressLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.progressLabel"), { task: task.title });
  const stateLabels: Readonly<Record<TaskManagerTaskV1["state"], string>> = {
    running: useLabel(taskManagerUiLabel("os.taskManager.tasks.running")),
    suspended: useLabel(taskManagerUiLabel("os.taskManager.tasks.suspended")),
    cancelling: useLabel(taskManagerUiLabel("os.taskManager.tasks.cancelling")),
  };
  const cancelLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.cancel"));
  const cancelTaskLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.cancelTask"), { task: task.title });
  const suspendLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.suspend"));
  const suspendTaskLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.suspendTask"), { task: task.title });
  const resumeLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.resume"));
  const resumeTaskLabel = useLabel(taskManagerUiLabel("os.taskManager.tasks.resumeTask"), { task: task.title });
  const progressText = task.progress !== null ? `${task.progress.text} · ${elapsedOnly}` : task.steps === null ? elapsedOnly : withSteps;
  const cancelling = task.state === "cancelling";
  const suspended = task.state === "suspended";
  const determinate = task.progress !== null && task.progress.total !== null && task.progress.total > 0 ? Math.min(100, Math.round((task.progress.completed / task.progress.total) * 100)) : null;
  return (
    <li data-semio-task-manager-task={task.id} data-semio-task-manager-lane={task.lane} data-semio-task-manager-state={task.state} className="flex min-w-0 flex-col gap-single py-single">
      <div className="flex min-w-0 items-baseline justify-between gap-single">
        <span className="min-w-0 truncate text-xs font-medium" title={task.title}>
          {task.title}
        </span>
        <span className="shrink-0 text-2xs text-muted-foreground">{stateLabels[task.state]}</span>
      </div>
      <div className="flex min-w-0 items-center gap-single text-2xs text-muted-foreground">
        <span className="shrink-0">{laneLabels[task.lane]}</span>
        {task.owner ? <span className="min-w-0 truncate">· {task.owner}</span> : null}
      </div>
      <div
        role="progressbar"
        aria-label={progressLabel}
        aria-valuetext={progressText}
        aria-valuemin={determinate === null ? undefined : 0}
        aria-valuemax={determinate === null ? undefined : 100}
        aria-valuenow={determinate ?? undefined}
        aria-busy={!cancelling && !suspended}
        data-semio-task-manager-progress={task.progress !== null ? task.progress.completed : (task.steps ?? "")}
        className="relative h-1 w-full min-w-0 overflow-hidden rounded-full bg-muted"
      >
        {determinate === null ? (
          <div className={`absolute inset-y-0 w-1/3 rounded-full bg-emphasized ${cancelling || suspended ? "opacity-40" : "animate-pulse"}`} />
        ) : (
          <div className={`absolute inset-y-0 left-0 rounded-full bg-emphasized ${cancelling || suspended ? "opacity-40" : ""}`} style={{ width: `${determinate}%` }} />
        )}
      </div>
      <div data-slot="task-manager-task-footer" className="flex min-w-0 flex-wrap items-center gap-single">
        <span data-slot="task-manager-task-progress-text" className="min-w-0 flex-1 basis-40 break-words text-2xs tabular-nums text-muted-foreground">{progressText}</span>
        <div data-slot="task-manager-task-actions" className="ms-auto flex shrink-0 items-center gap-single">
          {task.suspendable && !suspended ? (
            <Button type="button" variant="ghost" icon="pause" id={`os.task-manager.suspend.${task.id}`} data-semio-task-manager-suspend={task.id} aria-label={suspendTaskLabel} title={suspendTaskLabel} text={suspendLabel} disabled={cancelling} onClick={() => controls.onSuspend(task)} />
          ) : null}
          {task.suspendable && suspended ? (
            <Button type="button" variant="ghost" icon="play" id={`os.task-manager.resume.${task.id}`} data-semio-task-manager-resume={task.id} aria-label={resumeTaskLabel} title={resumeTaskLabel} text={resumeLabel} disabled={cancelling} onClick={() => controls.onResume(task)} />
          ) : null}
          <Button type="button" variant="ghost" icon="square" id={`os.task-manager.cancel.${task.id}`} data-semio-task-manager-cancel={task.id} aria-label={cancelTaskLabel} title={cancelTaskLabel} text={cancelLabel} disabled={cancelling} onClick={() => controls.onCancel(task)} />
        </div>
      </div>
    </li>
  );
}

/** @emoji 🏃️ Every task running right now, with its progress, a cancel control per task and suspend/resume for a suspendable one. A tick once a
 * second keeps the elapsed time honest while anything runs, and costs nothing while the list is empty. */
export function TaskManagerTasksPanel({ tasks, controls }: { readonly tasks: readonly TaskManagerTaskV1[]; readonly controls: TaskManagerTaskControlsV1 }): ReactElement {
  const title = useLabel(taskManagerUiLabel("os.taskManager.tasks.title"));
  const empty = useLabel(taskManagerUiLabel("os.taskManager.tasks.empty"));
  const [nowMs, setNowMs] = useState(() => Date.now());
  useEffect(() => {
    if (tasks.length === 0) return;
    setNowMs(Date.now());
    const ticker = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(ticker);
  }, [tasks.length]);
  return (
    <section aria-label={title} data-semio-task-manager-tasks={tasks.length} className="flex min-w-0 flex-col gap-single px-single">
      <h3 className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{title}</h3>
      {tasks.length === 0 ? (
        <p data-semio-task-manager-tasks-empty="" className="text-xs text-muted-foreground">
          {empty}
        </p>
      ) : (
        <ul aria-live="polite" className="flex min-w-0 flex-col divide-y divide-border">
          {tasks.map((task) => (
            <TaskManagerTaskRow key={task.id} task={task} nowMs={nowMs} controls={controls} />
          ))}
        </ul>
      )}
    </section>
  );
}

/** @emoji 🧵️ The mounted `os.task-manager` window: running tasks (spawned jobs, plugin installations, agent tool calls,
 * the focused program's tool runs — progress, cancel, and suspend/resume where the task can be held) above the live actor
 * table (suspend/resume/cancel through {@link createTaskManagerDispatcher}). Everything is read from `sources` on every
 * notification. */
export function TaskManagerWindow({ sources }: { readonly sources: TaskManagerSourcesV1 }): ReactElement {
  useSyncExternalStore(sources.subscribe, () => sources.tasks());
  const registry = sources.registry();
  const rows = useRuntimeMetricsRows(registry);
  const actorsTitle = useLabel(taskManagerUiLabel("os.taskManager.actorsTitle"));
  return (
    <div data-semio-task-manager-window="" className="flex min-w-0 flex-col gap-double py-single">
      <TaskManagerTasksPanel tasks={sources.tasks()} controls={{ onCancel: sources.cancel, onSuspend: sources.suspend, onResume: sources.resume }} />
      <section aria-label={actorsTitle} className="flex min-w-0 flex-col gap-single">
        <h3 className="px-single text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{actorsTitle}</h3>
        <TaskManagerPanel rows={rows} runtimeAttached={Boolean(registry)} onAction={registry ? createTaskManagerDispatcher(registry) : () => undefined} />
      </section>
    </div>
  );
}
//#endregion 🔖️RunningTasks

//#region 🔖️LiveDispatch
/** @emoji 🎬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1 follow-up, K1 landed): the REAL dispatch
 * path for the three row actions — `ActivationRegistry.suspend`/`resume`/`cancel` (the last one new
 * in this follow-up), which call straight through to a real `ShardClient`, the same live round-trip
 * `activate()` already proved out. This is genuinely reachable on web TODAY: build an
 * `ActivationRegistry` (already wired to a real worker pool in a browser), pass it here, and a
 * button click really suspends/resumes/cancels that actor's worker-side instance — nothing about
 * this call chain is a stub.
 *
 * Native parity: `Kernel::suspend`/`Kernel::resume` (pure, pre-existing) and submitting an
 * `Envelope { to: actor, payload: Payload::Cancel(0) }` (`Kernel::submit`) are the exact calls a
 * native dispatcher would make — `Payload::Cancel`'s inner `u64` is ignored by `ShardLoop::pump`
 * (K1: cancels every running job regardless of the value), so `0` is a correct, not a placeholder,
 * argument. Nothing on native currently CALLS that chain, because — same root cause as the metrics
 * publisher this same packet already flagged as unreachable — no code anywhere drives a live
 * `Kernel` on a native thread yet. That gap is not this function's to close (`📓️terra-T1-report.md`
 * `## honest gaps`); `createTaskManagerDispatcher` below is the web half, which needed no such thread
 * because `ActivationRegistry`/`ShardClient` are already live objects. */
export function createTaskManagerDispatcher(registry: ActivationRegistry): (action: TaskManagerActionKind, actorId: string) => void | Promise<void> {
  return (action, actorId) => {
    switch (action) {
      case "suspend":
        return registry.suspend(actorId);
      case "resume":
        return registry.resume(actorId);
      case "cancel":
        return registry.cancel(actorId);
    }
  };
}
//#endregion 🔖️LiveDispatch
