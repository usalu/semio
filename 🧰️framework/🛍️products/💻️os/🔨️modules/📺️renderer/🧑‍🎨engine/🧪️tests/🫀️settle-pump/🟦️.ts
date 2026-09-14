/**
 * 🫀️ TypeScript twin of `🧪️tests/🫀️settle-pump/🦀️.rs`.
 *
 * Re-derives the runtime-owned settle lane from the SAME oracle (`🧫️fixtures/🫀️settle-pump/🔣️.json`)
 * without looking at the Rust: when a frame owes a settle step, when a live producer is funded and
 * when its frozen witness is driven to a terminal state, and the order one frame's deferred owner
 * hands its work to the shell. The published `World3dScene.statusJson` documents are read through the
 * SHIPPED `world3dComputeStatusV1` parser React itself reads — the same contract the wgpu shell's
 * Rust twin parses — so the two shells cannot drift into two ideas of what "still working" means.
 *
 * ⚖️ An independent derivation is the point: React's own refresh lane
 * (`🛠️ShellHelpers/🟦️.tsx`'s `createUiRefreshCoalescerV1`) has always DECLARED a follow-up pass and let
 * its loop run it, never converging a chain inside the call that started it. This oracle is the wgpu
 * half of that same door, with the loop owned by the frame runtime.
 *
 * 🐛️ Three live defects on 6118 (ticket 26/09/09/PROCEDURAL-3D-END-TO-END): the boot example
 * converged inside `boot_shell` with nothing painted (`📓️wgpu-progress-visibility-2026-09-14.md`
 * §8.1); the preview wedged at `meshingFaces 36/56` and went deaf to `setActiveExample` (§8.2); and
 * narrowing the refresh on `UiDirtyScope` froze 14 of 16 examples mid-solve, because the render it
 * withdrew was the guest crossing funding the solve (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4).
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { world3dComputeStatusV1 } from "../../../../../../../🔨️modules/🖱️ui/🎬️scene/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🫀️settle-pump/🔣️.json");
const gestureSourcePaths = {
  shell: resolve(here, "../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"),
  renderer: resolve(here, "../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"),
} as const;

type ScopeName = "none" | "full" | "partial";
type GestureRow = { readonly id: string; readonly source: keyof typeof gestureSourcePaths; readonly owns: string; readonly forbids: string; readonly declares: string | null };
type OwesRow = { readonly id: string; readonly settling: boolean; readonly session: boolean; readonly armedWork: boolean; readonly scope: ScopeName; readonly armed: boolean; readonly computing: boolean; readonly expect: boolean };
type StatusEntry = { readonly json: string; readonly repeat?: number };
type WatchRow = { readonly id: string; readonly statuses: readonly StatusEntry[]; readonly expect: { readonly funds: number; readonly terminals: number; readonly stands: number; readonly firstTerminalAtStep: number | null } };
type FrameRow = { readonly id: string; readonly actions: number; readonly pumpSync: boolean; readonly flushTutorial: boolean; readonly shellMaintenance: boolean; readonly settle: boolean; readonly expect: { readonly order: readonly string[]; readonly terminalIsEmpty: boolean } };
type Fixture = {
  readonly stallSteps: number;
  readonly terminalDrives: number;
  readonly owesRows: readonly OwesRow[];
  readonly watchRows: readonly WatchRow[];
  readonly frameRows: readonly FrameRow[];
  readonly gestureRows: readonly GestureRow[];
  readonly laws: readonly string[];
};

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 🫀️ The predicate a frame asks, re-derived from the oracle's own five terms. */
function settlePumpOwes(row: OwesRow): boolean {
  if (row.settling || !row.session) return false;
  return row.armedWork || row.scope !== "none" || row.armed || row.computing;
}

/** ⏳️ The progress witness the watchdog compares — every counter a producer moves while it advances. */
function progressWitness(json: string): { readonly signature: string; readonly cancellable: boolean } {
  const status = world3dComputeStatusV1(json);
  return {
    signature: [status.phase, status.unitsDone, status.unitsTotal, status.facesDone, status.facesTotal, status.inFlight].join("|"),
    cancellable: status.cancellable,
  };
}

/** 🧯 One producer's watch, re-derived: patience per freeze, a bounded number of terminal drives, and
 * never a drive at all for a producer that published no way to end its own run. */
class Watch {
  private signature = "";
  private stalled = 0;
  private drives = 0;

  constructor(
    private readonly stallSteps: number,
    private readonly terminalDrives: number,
  ) {}

  verdict(witness: { readonly signature: string; readonly cancellable: boolean }): "fund" | "terminal" | "stand" {
    if (this.signature !== witness.signature) {
      this.signature = witness.signature;
      this.stalled = 0;
      this.drives = 0;
      return "fund";
    }
    this.stalled += 1;
    if (this.stalled < this.stallSteps) return "fund";
    if (this.drives >= this.terminalDrives || !witness.cancellable) {
      this.stalled = this.stallSteps;
      return "stand";
    }
    this.stalled = 0;
    this.drives += 1;
    return "terminal";
  }
}

/** 🎞️ One frame's deferred owner, re-derived: maintenance, sync, the frame's own actions, the tutorial
 * flush, and the settle step LAST. */
function frameOrder(row: FrameRow): readonly string[] {
  const order: string[] = [];
  if (row.shellMaintenance) order.push("shellMaintenance");
  if (row.pumpSync) order.push("pumpSync");
  for (let index = 0; index < row.actions; index += 1) order.push("action");
  if (row.flushTutorial) order.push("flushTutorial");
  if (row.settle) order.push("settle");
  return order;
}

describe("the wgpu host's runtime-owned settle pump", () => {
  it("owes a settle step exactly while a chain is live", () => {
    expect(fixture.owesRows.length).toBeGreaterThanOrEqual(7);
    for (const row of fixture.owesRows) expect(settlePumpOwes(row), row.id).toBe(row.expect);
  });

  it("funds a producer while it advances and drives a frozen witness to a terminal state", () => {
    for (const row of fixture.watchRows) {
      const watch = new Watch(fixture.stallSteps, fixture.terminalDrives);
      let funds = 0;
      let terminals = 0;
      let stands = 0;
      let firstTerminal: number | null = null;
      let step = 0;
      for (const entry of row.statuses) {
        const witness = progressWitness(entry.json);
        for (let repeat = 0; repeat < (entry.repeat ?? 1); repeat += 1) {
          step += 1;
          const verdict = watch.verdict(witness);
          if (verdict === "terminal") {
            terminals += 1;
            firstTerminal ??= step;
          } else if (verdict === "stand") {
            stands += 1;
          } else {
            funds += 1;
          }
        }
      }
      expect(funds, `${row.id}: crossings spent`).toBe(row.expect.funds);
      expect(terminals, `${row.id}: terminal drives`).toBe(row.expect.terminals);
      expect(stands, `${row.id}: steps the pump stood down for`).toBe(row.expect.stands);
      expect(firstTerminal, `${row.id}: first terminal drive`).toBe(row.expect.firstTerminalAtStep);
    }
  });

  it("drains a frame's input actions before it takes a settle step", () => {
    for (const row of fixture.frameRows) expect(frameOrder(row), row.id).toEqual(row.expect.order);
  });

  it("reads every watched status through the shipped world3d status contract", () => {
    for (const row of fixture.watchRows) {
      for (const entry of row.statuses) expect(world3dComputeStatusV1(entry.json).computing, row.id).toBe(true);
    }
  });

  it("fails a shell that never stands down — the non-vacuity guard", () => {
    const settled = fixture.owesRows.find((row) => row.expect === false && !row.settling && row.session);
    expect(settled, "the oracle declares at least one settled shell").toBeDefined();
    expect(fixture.owesRows.some((row) => row.expect === true)).toBe(true);
  });

  /** 🖱️ The gesture half of the same authority — the one law no predicate can express, so both
   * languages read the SOURCE and the oracle, not either implementation, names the entry points.
   *
   * 🩸️ Measured on 6118 (`📓️wgpu-generate-add-port-fit-2026-09-14.md`): a stray port drag rewired the
   * node graph, the pointer move behind it held the renderer's single `dispatch-event` interaction
   * checkout for 8 515 ms across 6 whole-shell refresh passes, and in that window the host dispatched
   * no input at all — the `F` pressed 1.1 s in reached the node-graph fit 7 632 ms after its key-down. */
  it("lets an input gesture declare the chain and never converge it inside its own dispatch", () => {
    expect(fixture.gestureRows.length).toBeGreaterThanOrEqual(4);
    const sources = new Map(Object.entries(gestureSourcePaths).map(([name, path]) => [name, readFileSync(path, "utf8")]));
    for (const row of fixture.gestureRows) {
      const source = sources.get(row.source);
      expect(source, `${row.id}: the oracle names a source this suite reads`).toBeDefined();
      const at = (source ?? "").indexOf(row.owns);
      expect(at, `${row.id}: ${row.owns} is declared`).toBeGreaterThanOrEqual(0);
      const after = (source ?? "").slice(at + row.owns.length);
      const end = after.indexOf("\n    }\n");
      const body = end < 0 ? after : after.slice(0, end);
      expect(body.includes(row.forbids), `${row.id}: ${row.owns} must not reach ${row.forbids} — a gesture that converges its own chain starves every later input`).toBe(false);
      if (row.declares !== null) expect(body.includes(row.declares), `${row.id}: ${row.owns} declares the chain with ${row.declares}`).toBe(true);
    }
    expect(sources.get("shell")?.includes("pub fn owe_settle(&mut self)"), "the shell publishes the one way a producer declares a live chain").toBe(true);
  });

  it("declares every law this suite answers", () => {
    expect(fixture.laws.length).toBeGreaterThanOrEqual(10);
    for (const law of fixture.laws) expect(law).toMatch(/^[a-z0-9-]+$/);
  });
});
