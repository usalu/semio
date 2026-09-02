// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TaskManager/component.tsx
/** @emoji 🧵️ `TaskManager` — the design of record's `🧵️task-manager` pane: one row per LIVE actor
 * (id, package, lane, status/failure stage, p95 wall time, mailbox length, shard, turns, traps,
 * restarts) with suspend/resume/cancel actions. Ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
 * RUNTIME` packet T1.
 *
 * Dual-rendering (React + wgpu) is achieved by NOT hand-writing either renderer's draw code: this
 * module mints the generic `TableScene` JSON shape (`columnsJson`/`rowsJson`, "buttons" cells) that
 * `Table/🟦️.tsx`'s `TableHost` already parses on the React side, and that the wgpu
 * `Interpreter/🧊️component.rs` already dispatches generically via `ui_wgpu::wgpu::SurfaceKind::Table`
 * (one of the 11 "generic fallback" surface kinds — see that file's own
 * `scene_command_reaches_every_generic_fallback_surface_kind_without_panicking` test). Studied both
 * `Table/🟦️.tsx` and `Interpreter/🧊️component.rs` before choosing this: reusing an already
 * dual-rendered surface kind is "follow the exact structure," not "invent a new pattern" — a
 * bespoke `SurfaceKind::TaskManager` would need a new WIT/Rust/TS variant registered in `ui_wgpu` and
 * dispatched from `Interpreter`, both outside this packet's `path_scope` (a NEW module directory
 * only). `TableColumnRecord`/`TableRowRecord`/`TableCellButton` are private to `Table/🟦️.tsx`
 * (not exported), so this file re-states their JSON shape by hand rather than importing it — see
 * `📓️terra-T1-report.md` for the byte-shape this was checked against.
 *
 * Mounting this as a real window (`host.open_window("os.task-manager", …)`, committing a scene from
 * `Kernel::runtime_metrics_snapshot` on every publish) needs a window-kind registration and a
 * `ShellHost` mount — both registrar-only; see the report's lease-request.
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
import { type ReactElement } from "react";
import { Button, Icon, Table, registerUiTranslationBundles, useLabel, type IconName, type TableColumn } from "@semio-tech/ui-react";
import { type ActionDescriptor } from "@semio-tech/framework";
import { type ActivationRegistry } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Types
/** 🛣️ Mirrors `semio_framework_actor::Lane`'s four camelCase tags. */
export type TaskManagerLane = "interactive" | "userVisible" | "background" | "maintenance";

/** 🗂️ Mirrors `semio_framework_actor::ActorStatus`'s tags (the `kind` discriminant). */
export type TaskManagerStatus = "cold" | "activating" | "active" | "suspended" | "draining" | "trapped" | "quarantined" | "disabled";

/** 🚑️ Mirrors `semio_framework_actor::FailureStage`'s tags. */
export type TaskManagerStage = "healthy" | "warned" | "throttled" | "trapped" | "quarantined" | "disabled" | "cancelled";

/** 🧵️ One live-actor row — field-compatible with `semio_framework_actor::ActorMetricsSample` joined
 * with its own `ActorMetrics` (`wallUsP95` is that type's `wall_us_p95()`). */
export interface TaskManagerRow {
  readonly actorId: string;
  readonly packageId: string;
  readonly lane: TaskManagerLane;
  readonly status: TaskManagerStatus;
  readonly stage: TaskManagerStage;
  readonly shard: number;
  readonly wallUsP95: number;
  readonly mailboxLen: number;
  readonly turns: number;
  readonly traps: number;
  readonly restarts: number;
}

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
 * `TableHost`'s `getRowId` falls back to. */
export type TaskManagerTableRow = Record<string, TaskManagerTableCell> & { readonly id: string };

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
export function taskManagerRows(rows: readonly TaskManagerRow[], labels: TaskManagerLabels): readonly TaskManagerTableRow[] {
  return rows.map((row) => ({
    id: row.actorId,
    actorId: { kind: "text", value: row.actorId },
    packageId: { kind: "text", value: row.packageId },
    lane: { kind: "text", value: labels.lanes[row.lane] },
    status: { kind: "text", value: labels.statuses[row.status] },
    stage: { kind: "text", value: row.stage },
    shard: { kind: "number", value: row.shard },
    wallUsP95: { kind: "number", value: row.wallUsP95 },
    mailboxLen: { kind: "number", value: row.mailboxLen },
    turns: { kind: "number", value: row.turns },
    traps: { kind: "number", value: row.traps },
    restarts: { kind: "number", value: row.restarts },
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
}

/** @emoji 🧵️ Directly-mountable React view of `TaskManagerRow[]` — for a standalone dialog/pane
 * context (same "typed props in, typed callback out" shape as `AgentApprovals`), independent of the
 * scene-commit path above. Renders through the SAME `@semio-tech/ui-react` `Table` primitive
 * `Table/🟦️.tsx`'s `TableHost` uses, so it inherits that component's table semantics/
 * keyboard navigation rather than a bespoke one. */
export function TaskManagerPanel({ rows, onAction }: TaskManagerPanelProps): ReactElement {
  const labels = useTaskManagerLabels();
  const columns: TableColumn<TaskManagerRow>[] = [
    { id: "actorId", header: labels.columns.actorId, accessor: (row) => row.actorId, sortable: true },
    { id: "packageId", header: labels.columns.packageId, accessor: (row) => row.packageId, sortable: true },
    { id: "lane", header: labels.columns.lane, accessor: (row) => labels.lanes[row.lane], sortable: true },
    { id: "status", header: labels.columns.status, accessor: (row) => labels.statuses[row.status], sortable: true },
    { id: "stage", header: labels.columns.stage, accessor: (row) => row.stage, sortable: true },
    { id: "shard", header: labels.columns.shard, accessor: (row) => String(row.shard), sortable: true },
    { id: "wallUsP95", header: labels.columns.wallUsP95, accessor: (row) => String(row.wallUsP95), sortable: true },
    { id: "mailboxLen", header: labels.columns.mailboxLen, accessor: (row) => String(row.mailboxLen), sortable: true },
    { id: "turns", header: labels.columns.turns, accessor: (row) => String(row.turns), sortable: true },
    { id: "traps", header: labels.columns.traps, accessor: (row) => String(row.traps), sortable: true },
    { id: "restarts", header: labels.columns.restarts, accessor: (row) => String(row.restarts), sortable: true },
    {
      id: "actions",
      header: labels.columns.actions,
      accessor: (row) => (
        <div className="flex items-center gap-1">
          <Button type="button" variant="outline" aria-label={`${labels.actions.suspend}: ${row.actorId}`} onClick={() => onAction("suspend", row.actorId)}>
            <Icon icon="pause" size="small" />
          </Button>
          <Button type="button" variant="outline" aria-label={`${labels.actions.resume}: ${row.actorId}`} onClick={() => onAction("resume", row.actorId)}>
            <Icon icon="play" size="small" />
          </Button>
          <Button type="button" variant="outline" aria-label={`${labels.actions.cancel}: ${row.actorId}`} onClick={() => onAction("cancel", row.actorId)}>
            <Icon icon="square" size="small" />
          </Button>
        </div>
      ),
    },
  ];
  return <Table columns={columns} data={[...rows]} getRowId={(row) => row.actorId} />;
}
//#endregion 🔖️TaskManagerPanel

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
