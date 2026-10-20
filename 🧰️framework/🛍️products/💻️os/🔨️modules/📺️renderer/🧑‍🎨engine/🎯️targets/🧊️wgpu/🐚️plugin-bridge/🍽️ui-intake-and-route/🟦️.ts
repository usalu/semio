import type { BuiltNode, Component, Effect } from "@semio-tech/framework";
import { decodePackWire, packWireNatural } from "@semio-tech/framework-os";
import { OwnedUiPatchIntake, RETAINED_UI_INTAKE_SLICE_STEPS, retainedUiIntakeStepCeiling } from "@semio-tech/framework-os/modules/renderer/engine/elements/UiDocumentStore/intake";
import { DEFAULT_UI_DOCUMENT_LIMITS } from "@semio-tech/ui-contract/limits";
import { WORKER_STEP_BUDGET_MS } from "../../⏱️turn-budget/🟦️.ts";
import type { RetainedUiNodeRecord } from "@semio-tech/ui-contract/retained/wire/typed";
import type { ShardInstanceLifecycleLease } from "@semio-tech/framework-actor/shard-client/activation";
import { OwnedUiInstance, type OwnedUiInstanceRetirement, type OwnedUiInstanceSurface, type OwnedUiPatchAcknowledgement } from "@semio-tech/ui-contract/retained/instance";
import { coerceTurnResult, type WireTurnResult, type WireUiPatch } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";
import { applyUiPatchToRetained, decodeWirePatchOps, type RetainedSurface } from "../../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🩹️retained-ui-patch/🟦️.ts";
import { OwnedUiPatchAcknowledgementEntry } from "@semio-tech/framework-actor/shard-client/ui-patch";
import { DEFAULT_SHARD_BUDGET } from "@semio-tech/framework-actor/shard-runtime";
import { stashLeftoverEffects } from "../🎛️contributions-and-effects/🟦️.ts";

export const RETAINED_DOCUMENT_OPPORTUNITIES = 256;

const WGPU_UI_CONTINUATION_BATCH_SIZE = 8;

const WGPU_UI_GRANT = Object.freeze({ maxItems: 1, maxBytes: 4_096 });

/** 📏️ The whole-document liveness backstop, shared with the React target and declared
 * language-agnostically in `🧵️retained/🧫️fixtures/📥️intake/🔣️.json` — NOT a per-drive cap. The fixed
 * `4_096` that stood here priced every intake as if it were a few KiB: the intake advances ONE phase
 * per `advance(grant)` (a LEB128 byte, a text body, an attach), so the grant never buys more phases,
 * and generation3d's first document (flow window scene + preview + catalogue pages + measures) blew
 * through it at `shell-boot` as `wgpu-ui.intake-budget-exhausted`. */
export const WGPU_UI_INTAKE_STEP_CEILING = retainedUiIntakeStepCeiling(DEFAULT_UI_DOCUMENT_LIMITS);

export type WgpuActorExecutor = <T>(work: () => Promise<T>) => Promise<T>;

/** 📄️ What the owned-UI route itself can answer: the surface as the guest published it.
 * `effects` is NOT part of it — a turn's leftover host effects belong to the caller that drained
 * them, which is why `renderSurfaceSerialized` is the only place the two are joined. */
type WgpuOwnedUiDocument = Readonly<{ node: BuiltNode; document: Readonly<{ surface: string; revision: number; root: number; nodes: readonly object[]; layoutEpoch: number }> }>;

export type WgpuOwnedUiProjection = WgpuOwnedUiDocument & Readonly<{ effects: readonly Effect[] }>;

/** ⏳️ How long this bridge may hold the isolate between yields — the SAME ceiling
 * `../../../⏱️turn-budget/🟦️.ts` prices a frame-Worker step against, so an intake never blocks a frame
 * longer than a frame step is allowed to. */
const WGPU_UI_YIELD_BUDGET_MS = WORKER_STEP_BUDGET_MS;

/** 🚏️ An UNTHROTTLED macrotask.
 *
 * 🩸️ What this replaces: `setTimeout(resolve, 0)`. Every browser clamps a `setTimeout` nested more than
 * five deep to **4 ms**, and a page whose tab/pane is hidden clamps its timers — the Worker's included —
 * to **≥1000 ms**. The intake and read paths below yield per step, and one generation3d document costs
 * tens of thousands of steps (`RETAINED_UI_INTAKE_SLICE_STEPS`'s own doc measures a first publication at
 * 163 284 consecutive steps), so the boot paid one clamped timer per handful of steps: the same work that
 * finishes in 3.9 s under Node — which clamps nothing — cannot finish inside `shell-boot` at all in a
 * hidden pane. A `MessagePort` task is a task, not a timer: no nesting clamp, no background throttle. */
let yieldPort: MessagePort | undefined;

const yieldWaiters: (() => void)[] = [];

function unthrottledMacrotask(): Promise<void> {
  if (!yieldPort) {
    const channel = new MessageChannel();
    channel.port1.onmessage = () => yieldWaiters.shift()?.();
    channel.port1.start();
    channel.port2.start();
    yieldPort = channel.port2;
  }
  return new Promise<void>((resolve) => {
    yieldWaiters.push(resolve);
    yieldPort!.postMessage(0);
  });
}

/** ⏳️ Hands the isolate back only once this drive has actually held it for
 * {@link WGPU_UI_YIELD_BUDGET_MS}. A fixed every-Nth-step cadence cannot know what a step cost: it
 * yielded ~20 000 times for one document's worth of sub-microsecond steps, which is pure clamp on the
 * browser and pure scheduling on Node. Deadline-driven, a document costs one yield per 8 ms of real work
 * and the isolate is still never held longer than one frame step. */
let yieldDeadlineMs = 0;

/** ⏳️ The same decision, reported as NOTHING TO AWAIT while the drive is still inside its hold
 * budget — the shape a per-phase caller needs.
 *
 * 🩸️ What this replaces: `await yieldWgpuUi()` on every step. An `async` function allocates a promise
 * and `await` costs a microtask tick even when the body returns immediately, and the retained intake
 * advances ONE decoder phase per call — so generation3d's flow window paid 525 k promise allocations
 * and 1.05 M microtask ticks to re-publish one document, 265 ms of which ~0.20 ms is the decoding
 * itself. A caller that awaits only what this returns pays for a yield exactly when the budget really
 * is spent (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-edit-convergence-perf-2026-09-14.md`). */
function yieldWgpuUiIfDue(): Promise<void> | undefined {
  const now = performance.now();
  if (yieldDeadlineMs === 0 || now < yieldDeadlineMs) {
    if (yieldDeadlineMs === 0) yieldDeadlineMs = now + WGPU_UI_YIELD_BUDGET_MS;
    return undefined;
  }
  yieldDeadlineMs = now + WGPU_UI_YIELD_BUDGET_MS;
  return unthrottledMacrotask();
}

export async function yieldWgpuUi(): Promise<void> {
  const pending = yieldWgpuUiIfDue();
  if (pending) await pending;
}

/** 🚏️ Hands the ISOLATE back at a slice boundary and restarts the hold budget — one task, and
 * the intake resumes on the very next one.
 *
 * 🩸️ What this replaces: an `await requestAnimationFrame(…)`. Nothing in the frame Worker is
 * animation-frame driven — the shell paints inside `tick()`, which arrives as a `postMessage` TASK from
 * the UI isolate — so awaiting a frame let nothing run that a task does not, and cost a full display
 * interval. Measured inside the live `semio-frame-worker` isolate on 6118: an animation frame is
 * **15.46 ms**, an {@link unthrottledMacrotask} is **0.0115 ms** — 1 345×. At one frame per
 * {@link RETAINED_UI_INTAKE_SLICE_STEPS}, generation3d's 525 k-step surface publication crossed 128
 * slices and therefore SLEPT 2.13 s per published surface: seven such blocks (2 881 / 2 376 / 2 148 /
 * 2 144 / 2 142 / 2 136 / 2 134 ms) made 16.0 s of the 21.6 s `shell-boot:flush-deferred` in edit mode,
 * with the worker's own CPU profile 87.6 % IDLE
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md`). */
async function handBackWgpuIsolate(): Promise<void> {
  yieldDeadlineMs = performance.now() + WGPU_UI_YIELD_BUDGET_MS;
  await unthrottledMacrotask();
}

/** 🎞️ One RESUMABLE drive cursor for a single retained intake. Exhausting a slice
 * ({@link RETAINED_UI_INTAKE_SLICE_STEPS}) is a yield, not a fault: the cursor is retained across the
 * boundary and the very same intake continues where it stopped, so a document larger than one slice
 * publishes across several turns of the isolate. Only {@link WGPU_UI_INTAKE_STEP_CEILING} — the
 * whole-document backstop — is terminal, and a genuine stall is caught earlier and more precisely by
 * the intake's own 32-consecutive-zero-progress rejection. Both yields are TASKS
 * ({@link handBackWgpuIsolate}, {@link yieldWgpuUi}): the slice is a resumption point, never a sleep. */
export class WgpuUiIntakeCursor {
  #steps = 0;
  readonly #ceiling: number;
  constructor(ceiling: number = WGPU_UI_INTAKE_STEP_CEILING) { this.#ceiling = ceiling; }
  get steps(): number { return this.#steps; }
  /** ⏳️ Returns what the caller must await, and `undefined` when this step owes the isolate nothing —
   * `await cursor.next(…)` on every phase is itself the cost, see {@link yieldWgpuUiIfDue}. */
  next(phase: string): Promise<void> | undefined {
    this.#steps += 1;
    if (this.#steps > this.#ceiling) throw new Error(`wgpu-ui.intake-budget-exhausted:${phase}:${this.#steps}`);
    return this.#steps % RETAINED_UI_INTAKE_SLICE_STEPS === 0 ? handBackWgpuIsolate() : yieldWgpuUiIfDue();
  }
}

function ownedUiComponentToBuilt(component: RetainedUiNodeRecord["component"]): Component {
  if (component.type !== "surface") return component;
  return { ...component, doc: { bytes: Array.from({ length: component.doc.bytes.length }, (_, index) => component.doc.bytes.byteAt(index)) } };
}

/** 🏠️ Exact WGPU host owner for one captured guest lifetime. Patch intake, render reads and
 * terminal retirement all traverse this same owner; no parallel retained tree can outlive its witness. */
export class WgpuOwnedUiInstanceRoute {
  readonly lifecycle: ShardInstanceLifecycleLease;
  readonly owner: OwnedUiInstance;
  readonly #surfaces = new Map<string, OwnedUiInstanceSurface>();
  readonly #intakes = new Set<OwnedUiPatchIntake>();
  readonly #reads = new Set<Promise<unknown>>();
  #retirement: OwnedUiInstanceRetirement | null = null;
  #closing = false;
  #intakeSteps = 0;
  get intakeSteps(): number { return this.#intakeSteps; }

  constructor(lifecycle: ShardInstanceLifecycleLease) {
    const lifetime = lifecycle.lifetime;
    if (!lifetime) throw new Error("wgpu-ui.native-lifetime-required");
    this.lifecycle = lifecycle;
    this.owner = new OwnedUiInstance(lifecycle.activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 32 });
    lifecycle.bindHostRetirement(this.owner);
  }

  get hasPendingReads(): boolean { return this.#reads.size > 0; }
  get terminalIsEmpty(): boolean { return this.owner.terminalIsEmpty() && this.#surfaces.size === 0 && this.#intakes.size === 0 && this.#reads.size === 0; }
  hasSurface(name: string): boolean { return this.#surfaces.has(name); }

  async #advanceMaintenance(phase: string, budget: { steps: number }): Promise<void> {
    while (this.owner.maintenancePending) {
      if (++budget.steps > DEFAULT_UI_DOCUMENT_LIMITS.maxNodes * 64) throw new Error(`wgpu-ui.${phase}-budget-exhausted`);
      const current = this.owner.advanceMaintenance(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.${phase}-${current.kind}:${current.phase}`);
      const pending = yieldWgpuUiIfDue();
      if (pending) await pending;
    }
  }

  async #closeIntake(intake: OwnedUiPatchIntake): Promise<void> {
    intake.beginClose();
    const cursor = new WgpuUiIntakeCursor();
    while (!intake.terminalIsEmpty()) {
      const current = intake.closeStep(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.intake-close-${current.kind}:${current.phase}`);
      { const pending = cursor.next("intake-close"); if (pending) await pending; }
    }
    this.#intakes.delete(intake);
  }

  async accept(turn: WireTurnResult, execute: WgpuActorExecutor): Promise<readonly WireTurnResult[]> {
    if (this.#closing) throw new Error("wgpu-ui.owner-closing");
    if (turn.uiPatches.length === 0) {
      if (turn.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.receipt-without-patch");
      return [];
    }
    if (!turn.original || !turn.uiPatchReceipt) throw new Error("wgpu-ui.native-owner-required");
    const admitted: { readonly surfaceId: string; readonly intake: OwnedUiPatchIntake; readonly cursor: WgpuUiIntakeCursor; readonly entry: OwnedUiPatchAcknowledgementEntry }[] = [];
    for (const [index, patch] of turn.uiPatches.entries()) {
      const surfaceId = patch.surface?.surface;
      if (!surfaceId) throw new Error("wgpu-ui.projection-surface-required");
      const source = this.lifecycle.captureUiPatchAuthority(turn.original, index);
      const intake = new OwnedUiPatchIntake(this.owner, source);
      this.#intakes.add(intake);
      const cursor = new WgpuUiIntakeCursor();
      let token: OwnedUiPatchAcknowledgement | null = null;
      while (token === null) {
        const current = intake.advance(WGPU_UI_GRANT);
        token = intake.peekAcknowledgement();
        if (current.kind === "rejected") throw new Error(`wgpu-ui.intake-rejected:${current.phase}:${intake.failure ?? "unknown"}`);
        if (current.kind === "blocked" && token === null) throw new Error(`wgpu-ui.intake-blocked:${current.phase}`);
        { const pending = cursor.next("intake"); if (pending) await pending; }
      }
      admitted.push({ surfaceId, intake, cursor, entry: { source, token } });
    }
    const acknowledged = await execute(() => this.lifecycle.submitUiAcknowledgements(admitted.map(({ entry }) => entry), DEFAULT_SHARD_BUDGET));
    if (acknowledged.receipts.length !== admitted.length) throw new Error("wgpu-ui.acknowledgement-refused");
    for (const [index, { surfaceId, intake, cursor }] of admitted.entries()) {
      if (!intake.acceptAcknowledgement(acknowledged.receipts[index]!)) throw new Error("wgpu-ui.acknowledgement-refused");
      for (;;) {
        const current = intake.advance(WGPU_UI_GRANT);
        if (current.kind === "ready") break;
        if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.intake-${current.kind}:${current.phase}`);
        { const pending = cursor.next("publication-close"); if (pending) await pending; }
      }
      const surface = intake.takeSurface();
      if (!surface) throw new Error("wgpu-ui.surface-missing");
      this.#surfaces.set(surfaceId, surface);
      this.#intakeSteps += cursor.steps;
      await this.#closeIntake(intake);
    }
    const next = coerceTurnResult(acknowledged.result);
    return [next, ...await this.accept(next, execute)];
  }

  async project(surfaceId: string): Promise<WgpuOwnedUiDocument | null> {
    if (this.#closing) throw new Error("wgpu-ui.owner-closing");
    const surface = this.#surfaces.get(surfaceId);
    if (!surface) return null;
    const read = this.#project(surfaceId, surface);
    this.#reads.add(read);
    try { return await read; } finally { this.#reads.delete(read); }
  }

  async #project(surfaceId: string, surface: OwnedUiInstanceSurface): Promise<WgpuOwnedUiDocument | null> {
    const view = surface.view;
    if (view.root === null) return null;
    if (!view.hash) throw new Error("wgpu-ui.surface-hash-required");
    const visited = new Set<number>();
    const records: object[] = [];
    const budget = { steps: 0 };
    const build = async (id: number, depth: number): Promise<BuiltNode> => {
      if (this.#closing || this.#surfaces.get(surfaceId) !== surface) throw new Error("wgpu-ui.read-stale");
      if (depth > DEFAULT_UI_DOCUMENT_LIMITS.maxDepth || visited.size >= DEFAULT_UI_DOCUMENT_LIMITS.maxNodes || visited.has(id)) throw new Error("wgpu-ui.read-graph-invalid");
      visited.add(id);
      const subscription = surface.subscribeNode(id, () => {});
      let record: RetainedUiNodeRecord | null = null;
      try {
        await this.#advanceMaintenance("read", budget);
        const snapshot = subscription.snapshot;
        if (!snapshot || snapshot.version !== view.revision || !snapshot.record) throw new Error("wgpu-ui.read-snapshot-missing");
        record = snapshot.record;
        surface.acknowledgeRead(subscription, snapshot);
      } finally {
        surface.unsubscribeNode(subscription);
        await this.#advanceMaintenance("read-retirement", budget);
      }
      if (!record) throw new Error("wgpu-ui.read-snapshot-missing");
      const component = ownedUiComponentToBuilt(record.component);
      records.push({ ...record, component });
      const children: BuiltNode[] = [];
      for (const child of record.children) children.push(await build(child, depth + 1));
      return { key: record.key, component, layout: record.layout, style: record.style, activity: record.activity, disabled: record.disabled, accessibility: record.accessibility, bindings: record.bindings, menu: record.menu, children };
    };
    const node = await build(view.root, 1);
    if (this.#closing || this.#surfaces.get(surfaceId) !== surface || surface.view !== view) throw new Error("wgpu-ui.read-stale");
    return { node, document: { surface: surfaceId, revision: view.revision, root: view.root, nodes: records, layoutEpoch: 0 } };
  }

  async retire(): Promise<OwnedUiInstanceRetirement> {
    if (this.#retirement) return this.#retirement;
    this.#closing = true;
    await Promise.allSettled([...this.#reads]);
    for (const intake of [...this.#intakes]) await this.#closeIntake(intake);
    this.#surfaces.clear();
    this.owner.beginClose();
    const cursor = new WgpuUiIntakeCursor();
    while (!this.owner.terminalIsEmpty()) {
      const current = this.owner.closeStep(WGPU_UI_GRANT);
      if (current.kind === "blocked" || current.kind === "rejected") throw new Error(`wgpu-ui.owner-close-${current.kind}:${current.phase}`);
      { const pending = cursor.next("owner-close"); if (pending) await pending; }
    }
    const witness = this.owner.takeRetirementWitness();
    if (!witness) throw new Error("wgpu-ui.retirement-witness-missing");
    this.#retirement = witness;
    return witness;
  }
}

/** 🚪️ Opens one captured lifecycle only after binding its exact host UI retirement owner. */
export async function settleInstanceLifecycle(lifecycle: ShardInstanceLifecycleLease, route: WgpuOwnedUiInstanceRoute, initial: WireTurnResult, execute: WgpuActorExecutor, instanceId: number): Promise<void> {
  let current = initial;
  const turns: WireTurnResult[] = [];
  const settle = (): void => {
    stashLeftoverEffects(instanceId, turns);
  };
  for (let opportunity = 0; opportunity < RETAINED_DOCUMENT_OPPORTUNITIES; opportunity += 1) {
    turns.push(current, ...await route.accept(current, execute));
    const receipt = lifecycle.pendingReceipt;
    if (receipt !== null) {
      if (receipt.kind !== "captured") throw new Error("wgpu-ui.open-receipt-kind");
      current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET)));
      continue;
    }
    const phase = lifecycle.progress().kind;
    if (phase === "open") {
      settle();
      return;
    }
    if (phase !== "opening" && phase !== "captured") throw new Error(`wgpu-ui.open-${phase}`);
    current = coerceTurnResult(await execute(() => lifecycle.poll(DEFAULT_SHARD_BUDGET)));
  }
  throw new Error(`wgpu-ui.open-budget-exhausted:${RETAINED_DOCUMENT_OPPORTUNITIES}`);
}

/** 🧹 Retires one captured WGPU lifetime without asking `beginClose` to cross an unacknowledged
 * Captured receipt. Cancellation at that exact stage first admits the original lifetime, then closes
 * the same bound UI owner through Accepted and Retired. */
export async function retireWgpuOwnedUiInstanceLifecycle(lifecycle: ShardInstanceLifecycleLease, route: WgpuOwnedUiInstanceRoute, execute: WgpuActorExecutor): Promise<void> {
  const captured = lifecycle.pendingReceipt;
  if (captured?.kind === "captured") {
    const acknowledged = coerceTurnResult(await execute(() => lifecycle.acknowledge(captured, DEFAULT_SHARD_BUDGET)));
    if (acknowledged.uiPatches.length > 0 || acknowledged.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.patch-during-cancelled-open");
  }
  lifecycle.beginClose();
  const cursor = new WgpuUiIntakeCursor();
  while (lifecycle.progress().kind !== "complete") {
    const receipt = lifecycle.pendingReceipt;
    let current: WireTurnResult;
    if (receipt?.kind === "accepted") current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET)));
    else if (receipt?.kind === "retired") {
      const retirement = await route.retire();
      current = coerceTurnResult(await execute(() => lifecycle.acknowledge(receipt, DEFAULT_SHARD_BUDGET, retirement)));
    } else {
      const progress = lifecycle.progress();
      if (progress.kind === "blocked") throw new Error(`wgpu-ui.lifecycle-close-blocked:${progress.failure ?? "unknown"}`);
      current = coerceTurnResult(await execute(() => progress.kind === "closing" ? lifecycle.close(DEFAULT_SHARD_BUDGET) : lifecycle.poll(DEFAULT_SHARD_BUDGET)));
    }
    if (current.uiPatches.length > 0 || current.uiPatchReceipt !== undefined) throw new Error("wgpu-ui.patch-after-lifecycle-close");
    { const pending = cursor.next("lifecycle-close"); if (pending) await pending; }
  }
  lifecycle.dispose();
}

/** 🖼️ One wire `UiPatch` reconciled onto `previous`: every `pack`-typed op payload is projected through
 * {@link decodePackWire} and the two WIT `u64` revisions narrowed through {@link packWireNatural}, so a
 * lossless integer carrier never reaches the retained tree as a `{kind, value}` object and a `bigint`
 * revision never fails the `baseRevision` identity check as a permanent desync. */
export function reconcileRetainedWindowPatch(previous: RetainedSurface | null, patch: WireUiPatch): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  const ops = decodeWirePatchOps(patch.ops ?? [], decodePackWire);
  return applyUiPatchToRetained(previous, { revision: packWireNatural(patch.revision, "uiPatch.revision"), baseRevision: packWireNatural(patch.baseRevision, "uiPatch.baseRevision"), ops });
}
