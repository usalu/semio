import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020";
import { applyPatch } from "fast-json-patch";
import schema from "../../🧬️schema/🚪️provisioning-lifecycle/🔣️.json";
import fixture from "../../🧫️fixtures/🚪️provisioning-lifecycle/🔣️.json";

type WindowKey = { windowId: string; windowKindId: string };
type DocumentState = { revision: number; camera: string };
type OwnerKind = { id: string; schema: string; policy: "framework-default" | "document-camera"; defaultValue: string };
type Pack = WindowKey & { innerPartitionId: string; schema: string; component: "Pack" | "Dsl"; version: number; value: string };
type CapturePath = "generation" | "render" | "panel-render" | "engagements" | "measures" | "tool-measures" | "context-menu" | "immediate-dispatch" | "retained-command";
type Event =
  | { kind: "instance-opened" }
  | { kind: "instance-closed" }
  | { kind: "restore-pack"; pack: Pack }
  | { kind: "host-save-pack"; window: WindowKey }
  | { kind: "host-restore-pack"; window: WindowKey }
  | { kind: "restore-complete" }
  | { kind: "window-opened"; window: WindowKey }
  | { kind: "window-closed"; window: WindowKey }
  | { kind: "surface-visible"; surfaceId: string; window: WindowKey }
  | { kind: "surface-hidden"; surfaceId: string }
  | { kind: "capture"; path: CapturePath; window: WindowKey }
  | { kind: "advance"; window: WindowKey }
  | { kind: "document-advanced"; document: DocumentState };
type Candidate = WindowKey & {
  phase: "waiting-restore" | "reserved" | "preparing" | "publishing" | "cancelling" | "closing";
  seed: string | null;
  documentRevision: number | null;
  waiters: Set<string>;
  pendingRestore: string | null;
};
type SummaryCandidate = Omit<Candidate, "waiters"> & { waiters: string[] };
type Partition = WindowKey & { value: string; source: "restored" | "framework-default" | "document-camera" };
type Capture = WindowKey & { path: CapturePath; outcome: "absent" | "pending" | "existing" | "wrong-kind" };
type Metrics = { captures: number; readMaterializations: 0; defaultSeeds: number; documentSeeds: number; hostSaves: number; restores: number; publications: number; cancellations: number; staleCandidates: number; duplicateOpens: number };
type Summary = {
  instanceOpen: boolean;
  restorationComplete: boolean;
  document: DocumentState;
  openWindows: WindowKey[];
  partitions: Partition[];
  savedHostPacks: Pack[];
  candidates: SummaryCandidate[];
  captures: Capture[];
  renderedSurfaces: string[];
  rejectedCodes: string[];
  metrics: Metrics;
};
type Scenario = { id: string; events: Event[]; expected: Summary; oraclePatch: { op: "replace"; path: string; value: unknown }[] };
type Fixture = { kinds: OwnerKind[]; initialDocument: DocumentState; scenarios: Scenario[] };

const lifecycle = fixture as Fixture;
const kinds = new Map(lifecycle.kinds.map((kind) => [kind.id, kind]));
const exactKey = ({ windowId, windowKindId }: WindowKey): string => `${windowKindId}\u0000${windowId}`;
const exactPartitionId = ({ windowId, windowKindId }: WindowKey): string => `window-config:${windowKindId}:${windowId}`;
const compareWindow = (left: WindowKey, right: WindowKey): number => exactKey(left).localeCompare(exactKey(right));

export class Oracle {
  instanceOpen = false;
  restorationComplete = false;
  document = structuredClone(lifecycle.initialDocument);
  openWindows = new Map<string, WindowKey>();
  partitions = new Map<string, Partition>();
  savedHostPacks = new Map<string, Pack>();
  candidates = new Map<string, Candidate>();
  surfaces = new Map<string, WindowKey>();
  captures: Capture[] = [];
  renderedSurfaces = new Set<string>();
  rejectedCodes: string[] = [];
  metrics: Metrics = { captures: 0, readMaterializations: 0, defaultSeeds: 0, documentSeeds: 0, hostSaves: 0, restores: 0, publications: 0, cancellations: 0, staleCandidates: 0, duplicateOpens: 0 };

  reject(code: string): void {
    this.rejectedCodes.push(code);
  }

  validateWindow(window: WindowKey): OwnerKind | undefined {
    const kind = kinds.get(window.windowKindId);
    if (!kind) this.reject("window-kind-unregistered");
    return kind;
  }

  reserve(window: WindowKey, kind: OwnerKind): Candidate {
    const documentPolicy = kind.policy === "document-camera";
    const candidate: Candidate = {
      ...window,
      phase: "reserved",
      seed: documentPolicy ? this.document.camera : kind.defaultValue,
      documentRevision: documentPolicy ? this.document.revision : null,
      waiters: new Set(),
      pendingRestore: null,
    };
    this.candidates.set(exactKey(window), candidate);
    if (documentPolicy) this.metrics.documentSeeds += 1;
    else this.metrics.defaultSeeds += 1;
    return candidate;
  }

  renderWaiters(candidate: Candidate): void {
    for (const surfaceId of candidate.waiters) {
      const binding = this.surfaces.get(surfaceId);
      const open = this.openWindows.get(candidate.windowId);
      if (binding && exactKey(binding) === exactKey(candidate) && open?.windowKindId === candidate.windowKindId) this.renderedSurfaces.add(surfaceId);
    }
  }

  restore(pack: Pack): void {
    const kind = kinds.get(pack.windowKindId);
    if (!kind) return this.reject("window-kind-unregistered");
    if (pack.innerPartitionId !== exactPartitionId(pack)) return this.reject("window-config.partition-id");
    if (pack.schema !== kind.schema) return this.reject("window-config.schema");
    if (pack.component !== "Pack") return this.reject("window-config.component");
    if (pack.version !== 1) return this.reject("window-config.version");
    const key = exactKey(pack);
    const candidate = this.candidates.get(key);
    if (candidate) {
      if (candidate.phase !== "cancelling" && candidate.phase !== "closing") this.metrics.cancellations += 1;
      candidate.phase = "cancelling";
      candidate.pendingRestore = pack.value;
      return;
    }
    this.partitions.set(key, { windowId: pack.windowId, windowKindId: pack.windowKindId, value: pack.value, source: "restored" });
    this.metrics.restores += 1;
  }

  save(window: WindowKey): void {
    const partition = this.partitions.get(exactKey(window));
    const kind = kinds.get(window.windowKindId);
    if (!partition || !kind) return this.reject("window-config.not-ready");
    this.savedHostPacks.set(exactKey(window), {
      ...window,
      innerPartitionId: exactPartitionId(window),
      schema: kind.schema,
      component: "Pack",
      version: 1,
      value: partition.value,
    });
    this.metrics.hostSaves += 1;
  }

  open(window: WindowKey): void {
    const kind = this.validateWindow(window);
    if (!kind) return;
    const prior = this.openWindows.get(window.windowId);
    if (prior) {
      if (prior.windowKindId !== window.windowKindId) return this.reject("window-kind-mismatch");
      this.metrics.duplicateOpens += 1;
      return;
    }
    this.openWindows.set(window.windowId, structuredClone(window));
    const key = exactKey(window);
    if (this.partitions.has(key) || this.candidates.has(key)) return;
    if (!this.restorationComplete) {
      this.candidates.set(key, { ...window, phase: "waiting-restore", seed: null, documentRevision: null, waiters: new Set(), pendingRestore: null });
      return;
    }
    this.reserve(window, kind);
  }

  close(window: WindowKey): void {
    const prior = this.openWindows.get(window.windowId);
    if (!prior) return this.reject("window-not-open");
    if (prior.windowKindId !== window.windowKindId) return this.reject("window-kind-mismatch");
    this.openWindows.delete(window.windowId);
    for (const [surfaceId, binding] of this.surfaces) if (exactKey(binding) === exactKey(window)) this.surfaces.delete(surfaceId);
    const candidate = this.candidates.get(exactKey(window));
    if (candidate && candidate.phase !== "cancelling" && candidate.phase !== "closing") {
      candidate.phase = "cancelling";
      this.metrics.cancellations += 1;
    }
  }

  visible(surfaceId: string, window: WindowKey): void {
    const open = this.openWindows.get(window.windowId);
    if (!open) return this.reject("window-not-open");
    if (open.windowKindId !== window.windowKindId) return this.reject("window-kind-mismatch");
    const prior = this.surfaces.get(surfaceId);
    if (prior) this.candidates.get(exactKey(prior))?.waiters.delete(surfaceId);
    this.surfaces.set(surfaceId, structuredClone(window));
    const key = exactKey(window);
    if (this.partitions.has(key)) this.renderedSurfaces.add(surfaceId);
    else this.candidates.get(key)?.waiters.add(surfaceId);
  }

  capture(path: CapturePath, window: WindowKey): void {
    let outcome: Capture["outcome"] = "absent";
    const open = this.openWindows.get(window.windowId);
    if ((open && open.windowKindId !== window.windowKindId) || !kinds.has(window.windowKindId)) outcome = "wrong-kind";
    else if (this.partitions.has(exactKey(window))) outcome = "existing";
    else if (this.candidates.has(exactKey(window))) outcome = "pending";
    this.captures.push({ ...window, path, outcome });
    this.metrics.captures += 1;
  }

  advance(window: WindowKey): void {
    const key = exactKey(window);
    const candidate = this.candidates.get(key);
    if (!candidate) return this.reject("candidate-absent");
    if (candidate.phase === "waiting-restore") return;
    if (candidate.phase === "reserved") candidate.phase = "preparing";
    else if (candidate.phase === "preparing") candidate.phase = "publishing";
    else if (candidate.phase === "publishing") {
      const kind = kinds.get(candidate.windowKindId)!;
      if (kind.policy === "document-camera" && candidate.documentRevision !== this.document.revision) {
        candidate.phase = "cancelling";
        this.metrics.cancellations += 1;
        this.metrics.staleCandidates += 1;
      } else {
        this.partitions.set(key, { windowId: candidate.windowId, windowKindId: candidate.windowKindId, value: candidate.seed!, source: kind.policy });
        this.metrics.publications += 1;
        this.renderWaiters(candidate);
        this.candidates.delete(key);
      }
    } else if (candidate.phase === "cancelling") candidate.phase = "closing";
    else {
      this.candidates.delete(key);
      if (candidate.pendingRestore !== null && this.instanceOpen) {
        this.partitions.set(key, { windowId: candidate.windowId, windowKindId: candidate.windowKindId, value: candidate.pendingRestore, source: "restored" });
        this.metrics.restores += 1;
        this.renderWaiters(candidate);
      } else {
        const open = this.openWindows.get(candidate.windowId);
        const kind = kinds.get(candidate.windowKindId);
        if (this.instanceOpen && open?.windowKindId === candidate.windowKindId && kind) {
          const replacement = this.reserve(candidate, kind);
          for (const waiter of candidate.waiters) replacement.waiters.add(waiter);
        }
      }
    }
  }

  apply(event: Event): void {
    if (event.kind === "instance-opened") {
      if (this.instanceOpen) return this.reject("instance-already-open");
      this.instanceOpen = true;
      return;
    }
    if (event.kind === "instance-closed") {
      if (!this.instanceOpen) return this.reject("instance-not-open");
      this.instanceOpen = false;
      this.restorationComplete = false;
      this.openWindows.clear();
      this.surfaces.clear();
      this.partitions.clear();
      for (const candidate of this.candidates.values()) {
        if (candidate.phase !== "cancelling" && candidate.phase !== "closing") {
          candidate.phase = "cancelling";
          this.metrics.cancellations += 1;
        }
      }
      return;
    }
    if (event.kind === "advance" && this.candidates.has(exactKey(event.window))) return this.advance(event.window);
    if (!this.instanceOpen) return this.reject("instance-not-open");
    if (event.kind === "restore-pack") return this.restore(event.pack);
    if (event.kind === "host-save-pack") return this.save(event.window);
    if (event.kind === "host-restore-pack") {
      const pack = this.savedHostPacks.get(exactKey(event.window));
      if (!pack) return this.reject("host-pack-absent");
      return this.restore(pack);
    }
    if (event.kind === "restore-complete") {
      if (this.restorationComplete) return;
      this.restorationComplete = true;
      for (const [key, candidate] of [...this.candidates]) {
        if (candidate.phase !== "waiting-restore") continue;
        if (this.partitions.has(key)) this.candidates.delete(key);
        else this.reserve(candidate, kinds.get(candidate.windowKindId)!);
      }
      return;
    }
    if (event.kind === "window-opened") return this.open(event.window);
    if (event.kind === "window-closed") return this.close(event.window);
    if (event.kind === "surface-visible") return this.visible(event.surfaceId, event.window);
    if (event.kind === "surface-hidden") {
      const binding = this.surfaces.get(event.surfaceId);
      if (binding) this.candidates.get(exactKey(binding))?.waiters.delete(event.surfaceId);
      this.surfaces.delete(event.surfaceId);
      return;
    }
    if (event.kind === "capture") return this.capture(event.path, event.window);
    if (event.kind === "advance") return this.advance(event.window);
    this.document = structuredClone(event.document);
  }

  summary(): Summary {
    return {
      instanceOpen: this.instanceOpen,
      restorationComplete: this.restorationComplete,
      document: structuredClone(this.document),
      openWindows: [...this.openWindows.values()].sort(compareWindow),
      partitions: [...this.partitions.values()].sort(compareWindow),
      savedHostPacks: [...this.savedHostPacks.values()].sort(compareWindow),
      candidates: [...this.candidates.values()].sort(compareWindow).map((candidate) => ({ ...candidate, waiters: [...candidate.waiters].sort() })),
      captures: structuredClone(this.captures),
      renderedSurfaces: [...this.renderedSurfaces].sort(),
      rejectedCodes: [...this.rejectedCodes],
      metrics: structuredClone(this.metrics),
    };
  }

  assertInvariants(label: string): void {
    assert.equal(this.metrics.readMaterializations, 0, `${label}: reads never materialize`);
    for (const [key, partition] of this.partitions) {
      assert.equal(key, exactKey(partition), `${label}: exact partition key`);
      assert.equal(this.candidates.has(key), false, `${label}: partition and candidate are exclusive`);
    }
    for (const [key, candidate] of this.candidates) {
      assert.equal(key, exactKey(candidate), `${label}: exact candidate key`);
      if (candidate.phase === "waiting-restore") assert.equal(candidate.seed, null, `${label}: no seed before restore barrier`);
      if (candidate.phase === "reserved" || candidate.phase === "preparing" || candidate.phase === "publishing") {
        assert.notEqual(candidate.seed, null, `${label}: active candidate has an explicit seed`);
      }
    }
    for (const [key, pack] of this.savedHostPacks) {
      assert.equal(key, exactKey(pack), `${label}: exact saved host key`);
      assert.equal(pack.innerPartitionId, exactPartitionId(pack), `${label}: exact saved inner partition id`);
    }
  }
}

const blankSummary = (): Summary => ({
  instanceOpen: false,
  restorationComplete: false,
  document: structuredClone(lifecycle.initialDocument),
  openWindows: [],
  partitions: [],
  savedHostPacks: [],
  candidates: [],
  captures: [],
  renderedSurfaces: [],
  rejectedCodes: [],
  metrics: { captures: 0, readMaterializations: 0, defaultSeeds: 0, documentSeeds: 0, hostSaves: 0, restores: 0, publications: 0, cancellations: 0, staleCandidates: 0, duplicateOpens: 0 },
});

/** 🚪️ Exact owners provision only after the restore barrier and window-open lifecycle. */
export function testWindowConfigProvisioningLifecycleOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert.equal(validate(lifecycle), true, JSON.stringify(validate.errors));
  assert.deepEqual(new Set(lifecycle.kinds.map((kind) => kind.policy)), new Set(["framework-default", "document-camera"]));
  const hostileEvent = structuredClone(lifecycle) as any;
  hostileEvent.scenarios[0].events[0].unknown = true;
  assert.equal(validate(hostileEvent), false, "events reject unknown fields");
  const hostileState = structuredClone(lifecycle) as any;
  hostileState.scenarios[0].expected.unknown = true;
  assert.equal(validate(hostileState), false, "summaries reject unknown fields");
  for (const scenario of lifecycle.scenarios) {
    assert.equal(new Set(scenario.oraclePatch.map((operation) => operation.path)).size, 11, `${scenario.id}: every summary field has one independent patch`);
    const oracle = new Oracle();
    scenario.events.forEach((event, index) => {
      oracle.apply(event);
      oracle.assertInvariants(`${scenario.id}[${index}]`);
    });
    const actual = oracle.summary();
    assert.deepEqual(actual, scenario.expected, `${scenario.id}: retained lifecycle reducer`);
    const patched = applyPatch(blankSummary(), scenario.oraclePatch, true, false).newDocument;
    assert.deepEqual(patched, scenario.expected, `${scenario.id}: independent JSON Patch oracle`);
    assert.equal(actual.metrics.readMaterializations, 0, `${scenario.id}: reads must not create owners`);
  }
  console.log(`[DEBUG] window-config-provisioning-lifecycle scenarios=${lifecycle.scenarios.length} capturePaths=9 policies=${lifecycle.kinds.length} readMaterializations=0`);
}

if (import.meta.main) testWindowConfigProvisioningLifecycleOracle();
