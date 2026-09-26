// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🧵️TaskManager/component.test.tsx
/** @emoji 🧪️ `🧵️TaskManager` tests: the pure scene-JSON builders (`taskManagerColumns`/
 * `taskManagerRows`/`buildTaskManagerTableScene`/`taskManagerRowAction`) plus a render +
 * action-dispatch test for the standalone `TaskManagerPanel`. Run directly the same way
 * `🤖️AgentApprovals/🧪️component.test.tsx` documents (see `📓️terra-T1-report.md`) — not (yet) picked
 * up by `@semio-tech/framework-renderer-react:test`'s semantic Vitest owner `test.include`, same
 * pre-existing gap that packet's own report already flagged.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildTaskManagerTableScene, createTaskManagerDispatcher, installTasksV1, runtimeMetricsRowsV1, spawnedJobTasksV1, taskManagerColumns, taskManagerElapsedSecondsV1, taskManagerMetricCell, taskManagerRowAction, taskManagerRows, TaskManagerPanel, TaskManagerTasksPanel, TaskManagerWindow, toolCallTasksV1, toolRunTasksV1, TASK_MANAGER_UNOBSERVED_METRIC, type TaskManagerLabels, type TaskManagerRow, type TaskManagerSourcesV1, type TaskManagerTableCell, type TaskManagerTaskV1 } from "../../🟦️.tsx";
import { type SpawnedJobRowV1 } from "../../../🔌️PluginRuntime/💼️job-ledger/🟦️.ts";
import { type AgentConversationEntry } from "../../../🔗️AgentBridge/🟦️.tsx";
import runningTasks from "../../🧫️fixtures/🏃️running-tasks.json";
import { ActivationRegistry } from "../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { ShardClient, type ShardBudget, type ShardWorkerLike } from "../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { OwnedResidentLedger } from "../../../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
const LABELS: TaskManagerLabels = {
  columns: { actorId: "Actor", packageId: "Package", lane: "Lane", status: "Status", stage: "Stage", shard: "Shard", wallUsP95: "p95 wall (µs)", mailboxLen: "Mailbox", turns: "Turns", traps: "Traps", restarts: "Restarts", actions: "Actions" },
  lanes: { interactive: "Interactive", userVisible: "User-visible", background: "Background", maintenance: "Maintenance" },
  statuses: { cold: "Cold", activating: "Activating", active: "Active", suspended: "Suspended", draining: "Draining", trapped: "Trapped", quarantined: "Quarantined", disabled: "Disabled" },
  actions: { suspend: "Suspend", resume: "Resume", cancel: "Cancel" },
  empty: "No actors are running.",
  noRuntime: "No actor runtime is attached to this shell.",
};

const ROW: TaskManagerRow = { actorId: "actor-1", packageId: "s.cad", lane: "interactive", status: "active", stage: "healthy", shard: 2, wallUsP95: 1500, mailboxLen: 3, turns: 42, traps: 0, restarts: 0 };
//#endregion 🔖️Fixtures

//#region 🔖️PureBuilders
describe("taskManagerColumns", () => {
  it("includes every required column id in display order, ending with actions", () => {
    const ids = taskManagerColumns(LABELS).map((column) => column.id);
    expect(ids).toEqual(["actorId", "packageId", "lane", "status", "stage", "shard", "wallUsP95", "mailboxLen", "turns", "traps", "restarts", "actions"]);
  });
});

describe("taskManagerRowAction", () => {
  it("mints an ActionDescriptor addressed at os.task-manager with the actor id in args", () => {
    expect(taskManagerRowAction("suspend", "actor-1")).toEqual({ controllerId: "os.task-manager", action: "suspend", args: { actorId: "actor-1" } });
    expect(taskManagerRowAction("resume", "actor-2")).toEqual({ controllerId: "os.task-manager", action: "resume", args: { actorId: "actor-2" } });
    expect(taskManagerRowAction("cancel", "actor-3")).toEqual({ controllerId: "os.task-manager", action: "cancel", args: { actorId: "actor-3" } });
  });
});

describe("taskManagerRows", () => {
  it("localizes lane/status text and carries every metric field through as a number cell", () => {
    const [row] = taskManagerRows([ROW], LABELS);
    expect(row!.id).toBe("actor-1");
    expect(row!.lane).toEqual({ kind: "text", value: "Interactive" });
    expect(row!.status).toEqual({ kind: "text", value: "Active" });
    expect(row!.shard).toEqual({ kind: "number", value: 2 });
    expect(row!.wallUsP95).toEqual({ kind: "number", value: 1500 });
    expect(row!.turns).toEqual({ kind: "number", value: 42 });
  });

  it("emits all three row actions (suspend/resume/cancel), each addressed at this row's actor id", () => {
    const [row] = taskManagerRows([ROW], LABELS);
    // 🔖️ `TaskManagerTableRow`'s index signature admits the row's own `id` string beside its cells,
    // so a cell read narrows explicitly rather than assuming every key is a cell.
    const actionsCell = row!.actions as TaskManagerTableCell;
    if (typeof actionsCell === "string" || actionsCell.kind !== "buttons") throw new Error("expected a buttons cell");
    expect(actionsCell.buttons.map((button) => button.action.action)).toEqual(["suspend", "resume", "cancel"]);
    for (const button of actionsCell.buttons) expect(button.action.args).toEqual({ actorId: "actor-1" });
  });
});

describe("buildTaskManagerTableScene", () => {
  it("produces columnsJson/rowsJson that round-trip through JSON.parse into the shape Table/component.tsx expects", () => {
    const scene = buildTaskManagerTableScene([ROW], LABELS);
    const columns = JSON.parse(scene.columnsJson) as Array<{ readonly id: string; readonly label: string }>;
    const rows = JSON.parse(scene.rowsJson) as Array<Record<string, unknown>>;
    expect(columns.find((column) => column.id === "actorId")?.label).toBe("Actor");
    expect(rows).toHaveLength(1);
    expect((rows[0] as { readonly id: string }).id).toBe("actor-1");
  });

  it("stays valid JSON with a full row set (multiple lanes/statuses)", () => {
    const rows: TaskManagerRow[] = [ROW, { ...ROW, actorId: "actor-2", lane: "background", status: "trapped", stage: "trapped", traps: 1, restarts: 1 }];
    const scene = buildTaskManagerTableScene(rows, LABELS);
    expect(() => JSON.parse(scene.columnsJson)).not.toThrow();
    expect(() => JSON.parse(scene.rowsJson)).not.toThrow();
    expect((JSON.parse(scene.rowsJson) as unknown[]).length).toBe(2);
  });
});
//#endregion 🔖️PureBuilders

//#region 🔖️Render
afterEach(cleanup);

describe("TaskManagerPanel", () => {
  it("renders one row per actor with its id and package visible", () => {
    render(<TaskManagerPanel rows={[ROW]} onAction={vi.fn()} />);
    expect(screen.getByText("actor-1")).toBeTruthy();
    expect(screen.getByText("s.cad")).toBeTruthy();
  });

  it("dispatches the right action for the right actor when a row's suspend/resume/cancel button is clicked", () => {
    const onAction = vi.fn();
    render(<TaskManagerPanel rows={[ROW]} onAction={onAction} />);
    fireEvent.click(screen.getByRole("button", { name: /Suspend: actor-1/ }));
    fireEvent.click(screen.getByRole("button", { name: /Resume: actor-1/ }));
    fireEvent.click(screen.getByRole("button", { name: /Cancel: actor-1/ }));
    expect(onAction).toHaveBeenNthCalledWith(1, "suspend", "actor-1");
    expect(onAction).toHaveBeenNthCalledWith(2, "resume", "actor-1");
    expect(onAction).toHaveBeenNthCalledWith(3, "cancel", "actor-1");
  });

  it("gives every action button an accessible name naming both the action and the actor (keyboard/screen-reader reachable)", () => {
    render(<TaskManagerPanel rows={[ROW, { ...ROW, actorId: "actor-2" }]} onAction={vi.fn()} />);
    expect(screen.getAllByRole("button", { name: /Suspend:/ })).toHaveLength(2);
    expect(screen.getByRole("button", { name: "Suspend: actor-2" })).toBeTruthy();
  });
});
//#endregion 🔖️Render

//#region 🔖️LiveDispatch
/** 🧪️ `createTaskManagerDispatcher` against a REAL `ActivationRegistry` + `ShardClient` (not a
 * mock of either) — an auto-replying fake `Worker` stands in for the browser `Worker`/`MessagePort`
 * (the one seam `ShardWorkerLike` exists to let a test inject), so every other layer in the chain —
 * `TaskManagerPanel`'s button click → `onAction` → `ActivationRegistry.suspend`/`resume`/`cancel` →
 * `ShardClient` → a postMessage round trip — is the exact production code path. */
const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };

function autoReplyingShardClient(): ShardClient {
  return new ShardClient({
    residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }),
    shardCount: 1,
    createWorker: () => {
      const worker: ShardWorkerLike = {
        postMessage: (message) => {
          const requestId = (message as { readonly requestId?: string }).requestId;
          if (requestId) queueMicrotask(() => worker.onmessage?.({ data: { kind: "result", requestId, ok: true, value: undefined } }));
        },
        terminate: () => {},
        onmessage: null,
        onerror: null,
      };
      return worker;
    },
  });
}

describe("createTaskManagerDispatcher wired to a real ActivationRegistry/ShardClient", () => {
  it("suspend really checkpoints + disposes the worker-side instance, through a genuine button click", async () => {
    const shardClient = autoReplyingShardClient();
    const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET, fetchAssets: async () => [] });
    registry.registerManifest({ pluginId: "s.cad", moduleUrl: "https://x/cad.js", caps: [] });
    await registry.activate("s.cad", "actor-1", "manual");
    expect(registry.isResident("actor-1")).toBe(true);

    render(<TaskManagerPanel rows={[ROW]} onAction={createTaskManagerDispatcher(registry)} />);
    fireEvent.click(screen.getByRole("button", { name: /Suspend: actor-1/ }));

    await waitFor(() => expect(registry.isResident("actor-1")).toBe(false));
    expect(shardClient.shardIndexFor("actor-1")).toBeUndefined();
  });

  it("cancel really disposes and forgets the actor — resume() afterward rejects unknown actor", async () => {
    const shardClient = autoReplyingShardClient();
    const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET, fetchAssets: async () => [] });
    registry.registerManifest({ pluginId: "s.cad", moduleUrl: "https://x/cad.js", caps: [] });
    await registry.activate("s.cad", "actor-1", "manual");

    const dispatch = createTaskManagerDispatcher(registry);
    dispatch("cancel", "actor-1");

    await expect(registry.resume("actor-1")).rejects.toThrow(/unknown actor/);
  });

  it("resume really re-activates a suspended actor, through a genuine button click", async () => {
    const shardClient = autoReplyingShardClient();
    const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET, fetchAssets: async () => [] });
    registry.registerManifest({ pluginId: "s.cad", moduleUrl: "https://x/cad.js", caps: [] });
    await registry.activate("s.cad", "actor-1", "manual");
    await registry.suspend("actor-1");
    expect(registry.isResident("actor-1")).toBe(false);

    render(<TaskManagerPanel rows={[ROW]} onAction={createTaskManagerDispatcher(registry)} />);
    fireEvent.click(screen.getByRole("button", { name: /Resume: actor-1/ }));

    await waitFor(() => expect(registry.isResident("actor-1")).toBe(true));
  });
});
//#endregion 🔖️LiveDispatch

//#region 🔖️LiveFeed
/** 🧪️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit ranked item 1): the pane is
 * now a mounted window with a real row source. These drive `runtimeMetricsRowsV1`/`TaskManagerWindow`
 * against a REAL `ActivationRegistry` (same auto-replying `ShardClient` the dispatch tests use), so
 * the `os.runtime.metrics` publisher that had no consumer anywhere in the codebase has one. */
describe("runtimeMetricsRowsV1", () => {
  it("maps residency onto status and leaves every counter a web registry cannot observe as null, never a silent zero", async () => {
    const shardClient = autoReplyingShardClient();
    const registry = new ActivationRegistry({ shardClient, defaultBudget: BUDGET, fetchAssets: async () => [] });
    registry.registerManifest({ pluginId: "s.cad", moduleUrl: "https://x/cad.js", caps: [] });
    await registry.activate("s.cad", "actor-1", "manual");

    const [row] = runtimeMetricsRowsV1(registry.runtimeMetricsSnapshot());
    expect(row!.actorId).toBe("actor-1");
    expect(row!.packageId).toBe("s.cad");
    expect(row!.status).toBe("active");
    expect(row!.turns).toBeNull();
    expect(row!.traps).toBeNull();
    expect(row!.wallUsP95).toBeNull();

    await registry.suspend("actor-1");
    expect(runtimeMetricsRowsV1(registry.runtimeMetricsSnapshot())[0]!.status).toBe("suspended");
  });
});

describe("taskManagerMetricCell", () => {
  it("renders an unobservable counter as an em dash rather than a number cell", () => {
    expect(taskManagerMetricCell(null)).toEqual({ kind: "text", value: TASK_MANAGER_UNOBSERVED_METRIC });
    expect(taskManagerMetricCell(0)).toEqual({ kind: "number", value: 0 });
  });
});

/** 🧪️ A window reads everything through `sources`; these hand it a fixed registry and task list. */
function sourcesFor(registry: ActivationRegistry | null, tasks: readonly TaskManagerTaskV1[] = [], cancel: (task: TaskManagerTaskV1) => void = () => undefined): TaskManagerSourcesV1 {
  return { registry: () => registry, tasks: () => tasks, subscribe: () => () => undefined, cancel, suspend: () => undefined, resume: () => undefined };
}

const NO_CONTROLS = { onCancel: () => undefined, onSuspend: () => undefined, onResume: () => undefined };

describe("TaskManagerWindow", () => {
  it("distinguishes 'no runtime attached' from 'the runtime reports no actors'", async () => {
    const { container } = render(<TaskManagerWindow sources={sourcesFor(null)} />);
    expect(container.querySelector("[data-semio-task-manager-empty]")?.getAttribute("data-semio-task-manager-empty")).toBe("no-runtime");
    cleanup();

    const registry = new ActivationRegistry({ shardClient: autoReplyingShardClient(), defaultBudget: BUDGET, fetchAssets: async () => [] });
    const second = render(<TaskManagerWindow sources={sourcesFor(registry)} />);
    await waitFor(() => expect(second.container.querySelector("[data-semio-task-manager-empty]")?.getAttribute("data-semio-task-manager-empty")).toBe("no-actors"));
  });

  it("renders the live actor the registry publishes, and follows an os.runtime.metrics event", async () => {
    const registry = new ActivationRegistry({ shardClient: autoReplyingShardClient(), defaultBudget: BUDGET, fetchAssets: async () => [] });
    registry.registerManifest({ pluginId: "s.cad", moduleUrl: "https://x/cad.js", caps: [] });
    await registry.activate("s.cad", "actor-1", "manual");

    render(<TaskManagerWindow sources={sourcesFor(registry)} />);
    await waitFor(() => expect(screen.getByText("actor-1")).toBeTruthy());

    await registry.activate("s.cad", "actor-2", "manual");
    registry.metricsBus.dispatchEvent(new CustomEvent("os.runtime.metrics", { detail: registry.runtimeMetricsSnapshot() }));
    await waitFor(() => expect(screen.getByText("actor-2")).toBeTruthy());
  });

  it("starts the registry's metrics publisher while mounted and stops it when closed", () => {
    const registry = new ActivationRegistry({ shardClient: autoReplyingShardClient(), defaultBudget: BUDGET, fetchAssets: async () => [] });
    const stop = vi.fn();
    const start = vi.spyOn(registry, "startRuntimeMetricsPublisher").mockReturnValue(stop);
    const { unmount } = render(<TaskManagerWindow sources={sourcesFor(registry)} />);
    expect(start).toHaveBeenCalledTimes(1);
    unmount();
    expect(stop).toHaveBeenCalledTimes(1);
  });
});
//#endregion 🔖️LiveFeed

//#region 🔖️RunningTasks
/** 🧪️ Slice U5: the window lists what is RUNNING — spawned jobs, installations, agent tool calls — each with
 * a progress bar and a cancel control. The row law is replayed from the language-agnostic fixture, and the
 * rendered semantics are read back through Testing Library's role queries (the third-party oracle for what a
 * screen reader is told). */
describe("running tasks", () => {
  const fixtureJobs: readonly SpawnedJobRowV1[] = runningTasks.jobs.map((job) => ({ ...job, job: BigInt(job.job) }));
  const fixtureTasks = [...spawnedJobTasksV1(fixtureJobs), ...installTasksV1(runningTasks.installs.pluginIds, new Map(Object.entries(runningTasks.installs.startedAtMs))), ...toolCallTasksV1(runningTasks.conversation as readonly AgentConversationEntry[])];

  it("maps every live job, installation and running tool call onto exactly one task", () => {
    expect(fixtureTasks).toEqual(runningTasks.expected);
  });

  it("counts elapsed seconds down to whole seconds and never below zero", () => {
    for (const row of runningTasks.elapsed) expect(taskManagerElapsedSecondsV1(row.startedAtMs, row.nowMs)).toBe(row.seconds);
  });

  it("renders a named progress bar and a named cancel control per task, and a cancelling task cannot be cancelled twice", () => {
    const onCancel = vi.fn();
    render(<TaskManagerTasksPanel tasks={fixtureTasks} controls={{ ...NO_CONTROLS, onCancel }} />);
    expect(screen.getAllByRole("progressbar")).toHaveLength(runningTasks.expected.length);
    const fill = screen.getByRole("progressbar", { name: /semio\.puzzle3d\.fill/u });
    expect(fill.getAttribute("aria-valuetext")).toMatch(/^128 /u);
    fireEvent.click(screen.getByRole("button", { name: /semio\.puzzle3d\.fill/u }));
    expect(onCancel).toHaveBeenCalledWith(expect.objectContaining({ id: "job:actor-7#3", lane: "job" }));
    const cancelling = screen.getByRole("button", { name: /inference_run/u }) as HTMLButtonElement;
    expect(cancelling.disabled).toBe(true);
  });

  it("says nothing is running rather than showing an empty list", () => {
    const { container } = render(<TaskManagerTasksPanel tasks={[]} controls={NO_CONTROLS} />);
    expect(container.querySelector("[data-semio-task-manager-tasks-empty]")).not.toBeNull();
    expect(container.querySelectorAll("[role='progressbar']")).toHaveLength(0);
  });

  it("re-reads its sources on every notification", async () => {
    let tasks: readonly TaskManagerTaskV1[] = [];
    const listeners = new Set<() => void>();
    const sources: TaskManagerSourcesV1 = { registry: () => null, tasks: () => tasks, subscribe: (listener) => (listeners.add(listener), () => listeners.delete(listener)), cancel: () => undefined, suspend: () => undefined, resume: () => undefined };
    const { container } = render(<TaskManagerWindow sources={sources} />);
    expect(container.querySelectorAll("[role='progressbar']")).toHaveLength(0);
    tasks = fixtureTasks.slice(0, 1);
    for (const listener of listeners) listener();
    await waitFor(() => expect(screen.getAllByRole("progressbar")).toHaveLength(1));
  });

  const fixtureToolRuns = () => {
    const now = vi.spyOn(Date, "now").mockReturnValue(runningTasks.toolRuns.nowMs);
    const tasks = toolRunTasksV1(runningTasks.toolRuns.runs.map((run) => ({ ...run, run: BigInt(run.run) })), runningTasks.toolRuns.program, new Map(Object.entries(runningTasks.toolRuns.startedAtMs)));
    now.mockRestore();
    return tasks;
  };

  it("maps every live tool run of the focused program onto one suspendable task, suspended while the run is paused", () => {
    expect(fixtureToolRuns()).toEqual(runningTasks.toolRuns.expected);
  });

  it("offers Suspend on a running tool run and Resume on a paused one, each named after the run, beside a determinate progress bar", () => {
    const onSuspend = vi.fn();
    const onResume = vi.fn();
    const onCancel = vi.fn();
    render(<TaskManagerTasksPanel tasks={fixtureToolRuns()} controls={{ onCancel, onSuspend, onResume }} />);
    const fill = screen.getByRole("progressbar", { name: /Fill/u });
    expect([fill.getAttribute("aria-valuenow"), fill.getAttribute("aria-valuemax"), fill.getAttribute("aria-valuetext")?.startsWith("Filling (1/2): 3 of 10 cells (30 %)")]).toEqual(["30", "100", true]);
    expect(screen.getByRole("progressbar", { name: /Reconstruct/u }).getAttribute("aria-valuenow")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Suspend Fill" }));
    fireEvent.click(screen.getByRole("button", { name: "Resume Reconstruct" }));
    fireEvent.click(screen.getByRole("button", { name: "Cancel Reconstruct" }));
    expect(screen.getAllByRole("button").map((button) => button.getAttribute("aria-label")).filter((name) => name === "Resume Fill" || name === "Suspend Reconstruct")).toEqual([]);
    expect([onSuspend.mock.calls[0]?.[0].id, onResume.mock.calls[0]?.[0].id, onCancel.mock.calls[0]?.[0].id]).toEqual(["toolRun:wfc-2#1", "toolRun:wfc-2#2", "toolRun:wfc-2#2"]);
  });

  /** 📐️ A live tool run's progress line ("Matching features (3/10): 227 of 374 decisions (60 %) · 61 s") used to share one
   * unwrapping row with its controls, so in the narrow Tasks window Cancel sat outside the panel — unreachable by pointer
   * (Playwright: "element is outside of the viewport", ticket 26/09/23 U5). The controls now live in their own group
   * after a wrapping progress line, and the progress bar spans the row. */
  it("keeps every task control in one group after a wrapping progress line, so a narrow window never clips Cancel", () => {
    render(<TaskManagerTasksPanel tasks={fixtureToolRuns()} controls={{ onCancel: vi.fn(), onSuspend: vi.fn(), onResume: vi.fn() }} />);
    const rows = [...document.querySelectorAll("[data-semio-task-manager-task]")];
    expect(rows.length).toBeGreaterThan(0);
    for (const row of rows) {
      const footer = row.querySelector('[data-slot="task-manager-task-footer"]');
      const actions = footer?.querySelector('[data-slot="task-manager-task-actions"]');
      const text = footer?.querySelector('[data-slot="task-manager-task-progress-text"]');
      expect(footer?.classList.contains("flex-wrap"), "the progress line and the controls wrap").toBe(true);
      expect(text?.classList.contains("shrink-0"), "the progress text yields width to the controls").toBe(false);
      expect([...row.querySelectorAll("button")].every((button) => actions?.contains(button)), "every control is in the actions group").toBe(true);
      expect(row.querySelector('[role="progressbar"]')?.parentElement, "the bar spans the row").toBe(row);
    }
  });

  it("scrolls the wide actor table inside a named, keyboard-focusable region instead of clipping its columns", () => {
    render(<TaskManagerPanel rows={[ROW]} onAction={vi.fn()} />);
    const region = screen.getByRole("region", { name: "Actors" });
    expect([region.getAttribute("tabindex"), region.classList.contains("overflow-x-auto"), region.querySelector("table") !== null]).toEqual(["0", true, true]);
  });
});
//#endregion 🔖️RunningTasks

