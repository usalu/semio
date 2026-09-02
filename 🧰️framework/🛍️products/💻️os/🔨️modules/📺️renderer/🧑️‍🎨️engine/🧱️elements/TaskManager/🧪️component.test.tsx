// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/TaskManager/component.test.tsx
/** @emoji 🧪️ `TaskManager` tests: the pure scene-JSON builders (`taskManagerColumns`/
 * `taskManagerRows`/`buildTaskManagerTableScene`/`taskManagerRowAction`) plus a render +
 * action-dispatch test for the standalone `TaskManagerPanel`. Run directly the same way
 * `AgentApprovals/🧪️component.test.tsx` documents (see `📓️terra-T1-report.md`) — not (yet) picked
 * up by `@semio-tech/framework-renderer-react:test`'s `vitest.config.ts` `test.include`, same
 * pre-existing gap that packet's own report already flagged.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildTaskManagerTableScene, createTaskManagerDispatcher, taskManagerColumns, taskManagerRowAction, taskManagerRows, TaskManagerPanel, type TaskManagerLabels, type TaskManagerRow } from "./🟦️.tsx";
import { ActivationRegistry, type ShardBudget } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { ShardClient, type ShardWorkerLike } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️";
import { OwnedResidentLedger } from "../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
const LABELS: TaskManagerLabels = {
  columns: { actorId: "Actor", packageId: "Package", lane: "Lane", status: "Status", stage: "Stage", shard: "Shard", wallUsP95: "p95 wall (µs)", mailboxLen: "Mailbox", turns: "Turns", traps: "Traps", restarts: "Restarts", actions: "Actions" },
  lanes: { interactive: "Interactive", userVisible: "User-visible", background: "Background", maintenance: "Maintenance" },
  statuses: { cold: "Cold", activating: "Activating", active: "Active", suspended: "Suspended", draining: "Draining", trapped: "Trapped", quarantined: "Quarantined", disabled: "Disabled" },
  actions: { suspend: "Suspend", resume: "Resume", cancel: "Cancel" },
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
    const actionsCell = row!.actions;
    if (actionsCell.kind !== "buttons") throw new Error("expected a buttons cell");
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
