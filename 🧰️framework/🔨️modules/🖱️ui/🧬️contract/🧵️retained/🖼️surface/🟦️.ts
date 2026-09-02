//#region 🧬️SurfaceContract
import { NumericIndex, type NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { UiDocumentLimits } from "../../../../🛂️manifest/🟦️.ts";
import { RetainedUiNumericTable } from "../🟦️.ts";
import { OwnedUiNodeIndex, type OwnedUiNodeIndexReader, type OwnedUiNodeIndexRetirement } from "../🗂️nodes/🟦️.ts";
import { OwnedUiOperation, OwnedUiOperationCursor } from "../🩹️operations/🟦️.ts";
import { OwnedUiValidationCursor } from "../🛡️validation/🟦️.ts";
import { OwnedUiSnapshotHashCursor } from "../🔢️hash/🟦️.ts";
import { OwnedUiSceneBindingCursor, type OwnedUiSceneBinding, type OwnedUiSceneBindingRetirement } from "../🎬️scene/🔗️binding/🟦️.ts";
import { OwnedUiSceneBindingIndex, type OwnedUiSceneBindingIndexReader, type OwnedUiSceneBindingIndexEdit, type OwnedUiSceneBindingIndexRetirement } from "../🎬️scene/🔗️binding/🗂️index/🟦️.ts";
import type { OwnedUiSceneHostProfile } from "../🎬️scene/🧾️typed/🟦️.ts";
import { OwnedUiNodeReadLease, OwnedUiReadPublication, type OwnedUiReadCommit, type OwnedUiNodeReadSnapshot, type OwnedUiSceneReadSource, type OwnedUiReadSubscription, type OwnedUiIssuedSceneReader, type OwnedUiIssuedSceneRetirement, type OwnedUiSceneRecordView, type OwnedUiSceneTextView } from "../📖️read-lease/🟦️.ts";
import type { OwnedUiNode, UiNodeRetirement } from "../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️.ts";

export type OwnedUiSurfaceIdentity = { readonly actor: string; readonly instance: number; readonly surface: string };
export type OwnedUiSurfaceView = { readonly revision: number; readonly root: number | null; readonly hash: string | null };
export type OwnedUiSurfaceAcknowledgement = OwnedUiSurfaceIdentity & { readonly revision: number; readonly hash: string };
type SurfaceState = { readonly nodes: OwnedUiNodeIndex; readonly bindings: OwnedUiSceneBindingIndex; readonly view: OwnedUiSurfaceView };
type Program = Generator<number, void, void>;
type Staged = { cell: Cell | null; next: Staged | null };
type Cell = {
  owner: OwnedUiSurface | null; readonly id: number | null; readonly ordinal: number; notify: (() => void) | null;
  active: boolean; initialized: boolean; initialNotify: boolean; queued: boolean; previous: Cell | null; next: Cell | null; queueNext: Cell | null;
  lease: OwnedUiNodeReadLease | null; reader: OwnedUiSceneBindingIndexReader | null; readerComplete: boolean; retirement: OwnedUiSceneBindingIndexRetirement | null;
  node: OwnedUiSceneBinding | null; nodeRetirement: OwnedUiSceneBindingRetirement | null; lookupVersion: number;
  subscription: OwnedUiReadSubscription | null; failed: boolean; failureQueued: boolean; failurePrevious: Cell | null; failureNext: Cell | null;
  readonly sceneClose: [OwnedUiIssuedSceneRetirement | null, OwnedUiIssuedSceneRetirement | null, OwnedUiIssuedSceneRetirement | null, OwnedUiIssuedSceneRetirement | null];
  readonly sceneActive: [OwnedUiIssuedSceneReader | null, OwnedUiIssuedSceneReader | null, OwnedUiIssuedSceneReader | null, OwnedUiIssuedSceneReader | null];
};
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const MINT = Object.freeze({});
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
const validWork = (current: { readonly items: number; readonly bytes: number }, grant: NumericIndexGrant): boolean => Number.isSafeInteger(current.items) && current.items >= 0 && current.items <= 1 && Number.isSafeInteger(current.bytes) && current.bytes >= 0 && current.bytes <= grant.maxBytes;
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep { if (!validWork(current, grant)) return { ...current, kind: "rejected" }; return current.kind === "complete" ? { ...current, kind: "pending" } : current; }
function natural(value: number): number { if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("Invalid owned UI identity"); return value === 0 ? 0 : value; }
function limitsOf(value: UiDocumentLimits): UiDocumentLimits { return Object.freeze({ maxNodes: natural(value.maxNodes), maxDepth: natural(value.maxDepth), maxChildren: natural(value.maxChildren), maxTextBytes: natural(value.maxTextBytes), maxPatchOps: natural(value.maxPatchOps), maxPatchBytes: natural(value.maxPatchBytes) }); }
let cellOf: (subscription: OwnedUiReadSubscription) => Cell | null;
let subscriptionOf: (cell: Cell) => OwnedUiReadSubscription;
let sourceOf: (owner: OwnedUiSurface) => SurfaceState;
let firstCell: (owner: OwnedUiSurface) => Cell | null;
let lastOrdinal: (owner: OwnedUiSurface) => number;
let epochOf: (owner: OwnedUiSurface) => OwnedUiReadPublication;
let profileOf: (owner: OwnedUiSurface) => OwnedUiSceneHostProfile;
let enqueueCell: (owner: OwnedUiSurface, cell: Cell) => void;
let notifyCell: (owner: OwnedUiSurface, cell: Cell) => void;
let publish: (owner: OwnedUiSurface, patch: OwnedUiSurfacePatch, source: SurfaceState, nodes: OwnedUiNodeIndex, bindings: OwnedUiSceneBindingIndex, root: number | null, revision: number, hash: string, epoch: OwnedUiReadCommit) => SurfaceState;
let releasePatch: (owner: OwnedUiSurface, patch: OwnedUiSurfacePatch) => void;
let createPatch: (owner: OwnedUiSurface, source: SurfaceState, revision: number, limits: UiDocumentLimits) => OwnedUiSurfacePatch;
let appended: (patch: OwnedUiSurfacePatch, cell: Cell) => void;
let detached: (patch: OwnedUiSurfacePatch, cell: Cell) => void;
//#endregion 🧬️SurfaceContract

//#region 📖️OpaqueSubscription
class Subscription implements OwnedUiReadSubscription {
  readonly #cell: Cell;
  private constructor(mint: object, cell: Cell) { if (mint !== MINT) throw new Error("Surface subscription requires exact mint authority"); this.#cell = cell; Object.freeze(this); }
  static { subscriptionOf = cell => new Subscription(MINT, cell); cellOf = value => value instanceof Subscription ? value.#cell : null; }
  get snapshot(): OwnedUiNodeReadSnapshot | null { if (!this.#cell.owner) throw new Error("Owned UI subscription is retired"); return this.#cell.lease?.snapshot ?? null; }
}
//#endregion 📖️OpaqueSubscription

//#region 🖼️SurfaceOwner
/** 🖼️ Owns one flat published surface and every issued consumer root through explicit terminal close. */
export class OwnedUiSurface implements OwnedUiSceneReadSource {
  readonly identity: OwnedUiSurfaceIdentity;
  readonly #limits: UiDocumentLimits;
  readonly #profile: OwnedUiSceneHostProfile;
  #state: SurfaceState | null;
  readonly #epoch = new OwnedUiReadPublication(0);
  #patch: OwnedUiSurfacePatch | null = null;
  #head: Cell | null = null;
  #tail: Cell | null = null;
  #queue: Cell | null = null;
  #queueTail: Cell | null = null;
  #queueWorked = false;
  #queueComplete = false;
  #maintenanceFailure: string | null = null;
  #ordinal = 0;
  #retirement: OwnedUiNodeIndexRetirement | null = null;
  #bindingRetirement: OwnedUiSceneBindingIndexRetirement | null = null;
  #closing = false;
  #closed = false;
  #notificationFailures = 0;
  #failureHead: Cell | null = null;
  #failureTail: Cell | null = null;

  constructor(identity: OwnedUiSurfaceIdentity, limits: UiDocumentLimits, profile: OwnedUiSceneHostProfile) {
    if (typeof identity.actor !== "string" || typeof identity.surface !== "string" || natural(identity.instance) > 0xffff_ffff) throw new Error("Invalid owned UI surface identity");
    const bits = profile.usizeBits; if (bits !== 32 && bits !== 64) throw new Error("Owned surface requires an owning host width"); this.#profile = Object.freeze({ usizeBits: bits });
    this.identity = Object.freeze({ actor: identity.actor, instance: identity.instance === 0 ? 0 : identity.instance, surface: identity.surface }); this.#limits = limitsOf(limits);
    this.#state = { nodes: OwnedUiNodeIndex.empty(), bindings: OwnedUiSceneBindingIndex.empty(), view: Object.freeze({ revision: 0, root: null, hash: null }) }; Object.freeze(this);
  }
  static {
    sourceOf = owner => owner.#live(); firstCell = owner => owner.#head; lastOrdinal = owner => owner.#tail?.ordinal ?? 0; epochOf = owner => owner.#epoch; profileOf = owner => owner.#profile;
    enqueueCell = (owner, cell) => owner.#enqueue(cell); notifyCell = (owner, cell) => owner.#notify(cell);
    releasePatch = (owner, patch) => { if (owner.#patch !== patch) throw new Error("Foreign owned UI patch release"); owner.#patch = null; };
    publish = (owner, patch, source, nodes, bindings, root, revision, hash, epoch) => {
      if (owner.#closing || owner.#patch !== patch || owner.#state !== source) throw new Error("Stale owned UI publication authority");
      const next = { nodes, bindings, view: Object.freeze({ revision, root, hash }) };
      if (!owner.#epoch.publish(epoch)) throw new Error("Stale owned UI read publication");
      owner.#state = next; return source;
    };
  }
  #live(): SurfaceState { if (!this.#state || this.#closed) throw new Error("Owned UI surface is closed"); return this.#state; }
  get view(): OwnedUiSurfaceView { return this.#live().view; }
  get maintenancePending(): boolean { return this.#queue !== null; }
  get maintenanceFailure(): string | null { return this.#maintenanceFailure; }
  get notificationFailures(): number { return this.#notificationFailures; }

  beginPatch(baseRevision: number, revision: number): OwnedUiSurfacePatch {
    const source = this.#live(); const base = natural(baseRevision); const next = natural(revision);
    if (this.#closing || this.#patch || base !== source.view.revision || next <= base) throw new Error("Owned UI patch admission rejected");
    const patch = createPatch(this, source, next, this.#limits); this.#patch = patch; return patch;
  }
  subscribeNode(id: number, notify: () => void): OwnedUiReadSubscription { return this.#subscribe(natural(id), notify); }
  subscribeView(notify: () => void): OwnedUiReadSubscription { return this.#subscribe(null, notify); }
  #subscribe(id: number | null, notify: () => void): OwnedUiReadSubscription {
    this.#live(); if (this.#closing || this.#ordinal === Number.MAX_SAFE_INTEGER || typeof notify !== "function") throw new Error("Owned UI subscription admission rejected");
    const cell: Cell = { owner: this, id, ordinal: ++this.#ordinal, notify, active: true, initialized: false, initialNotify: true, queued: false, previous: this.#tail, next: null, queueNext: null, lease: null, reader: null, readerComplete: false, retirement: null, node: null, nodeRetirement: null, lookupVersion: 0, subscription: null, failed: false, failureQueued: false, failurePrevious: null, failureNext: null, sceneClose: [null, null, null, null], sceneActive: [null, null, null, null] };
    cell.subscription = subscriptionOf(cell);
    if (this.#tail) this.#tail.next = cell; else this.#head = cell; this.#tail = cell; this.#enqueue(cell);
    if (this.#patch) appended(this.#patch, cell); return cell.subscription;
  }
  acknowledgeRead(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot): void {
    const cell = cellOf(subscription); if (!cell || cell.owner !== this || !cell.active || !cell.lease) return;
    if (cell.lease.acknowledge(snapshot) && cell.lease.retirementPending) this.#enqueue(cell);
  }
  unsubscribeNode(subscription: OwnedUiReadSubscription): void { const cell = cellOf(subscription); if (cell?.owner === this && cell.active) this.#detach(cell); }
  retireSceneRead(subscription: OwnedUiReadSubscription, reader: OwnedUiIssuedSceneReader): boolean {
    const cell = cellOf(subscription); if (!cell || cell.owner !== this || !cell.lease) return false;
    for (let index = 0; index < 4; index++) if (cell.sceneClose[index] === null) {
      const retirement = cell.lease.takeSceneRetirement(reader); if (!retirement) return false;
      cell.sceneClose[index] = retirement; for (let active = 0; active < 4; active++) if (cell.sceneActive[active] === reader) cell.sceneActive[active] = null;
      this.#enqueue(cell); return true;
    }
    return false;
  }
  #sceneCell(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot): Cell | null {
    const cell = cellOf(subscription); return cell?.owner === this && cell.active && cell.lease?.canReadSnapshot(snapshot) ? cell : null;
  }
  openSceneRecord(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source = 0): OwnedUiSceneRecordView | null {
    const cell = this.#sceneCell(subscription, snapshot); if (!cell) return null;
    for (let index = 0; index < 4; index++) if (!cell.sceneActive[index]) {
      const reader = snapshot.beginSceneRecord(source); if (!reader) return null; cell.sceneActive[index] = reader;
      return Object.freeze({ advance: (grant: NumericIndexGrant) => reader.advance(grant), close: () => this.retireSceneRead(subscription, reader) });
    }
    return null;
  }
  openSceneText(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source: number): OwnedUiSceneTextView | null {
    const cell = this.#sceneCell(subscription, snapshot); if (!cell) return null;
    for (let index = 0; index < 4; index++) if (!cell.sceneActive[index]) {
      const reader = snapshot.beginSceneText(source); if (!reader) return null; cell.sceneActive[index] = reader;
      return Object.freeze({ advance: (grant: NumericIndexGrant) => reader.advance(grant), close: () => this.retireSceneRead(subscription, reader) });
    }
    return null;
  }
  #detach(cell: Cell): void {
    let free = 0; let active = 0; for (let index = 0; index < 4; index++) { if (cell.sceneClose[index] === null) free++; if (cell.sceneActive[index] !== null) active++; }
    if (free < active) throw new Error("Owned UI scene retirement reservation violated");
    for (let index = 0; index < 4; index++) { const reader = cell.sceneActive[index]; if (reader && !this.retireSceneRead(cell.subscription!, reader)) throw new Error("Owned UI scene retirement authority violated"); }
    this.#removeFailure(cell);
    if (this.#patch) detached(this.#patch, cell);
    if (cell.previous) cell.previous.next = cell.next; else this.#head = cell.next;
    if (cell.next) cell.next.previous = cell.previous; else this.#tail = cell.previous;
    cell.previous = null; cell.next = null; cell.active = false; cell.notify = null; this.#enqueue(cell);
  }
  #enqueue(cell: Cell): void { if (cell.queued) { if (this.#queue === cell) this.#queueComplete = false; return; } cell.queued = true; if (this.#queueTail) this.#queueTail.queueNext = cell; else this.#queue = cell; this.#queueTail = cell; }
  #dequeue(): Cell { const cell = this.#queue!; this.#queue = cell.queueNext; if (!this.#queue) this.#queueTail = null; cell.queueNext = null; cell.queued = false; return cell; }
  #removeFailure(cell: Cell): void {
    if (!cell.failureQueued) return;
    if (cell.failurePrevious) cell.failurePrevious.failureNext = cell.failureNext; else this.#failureHead = cell.failureNext;
    if (cell.failureNext) cell.failureNext.failurePrevious = cell.failurePrevious; else this.#failureTail = cell.failurePrevious;
    cell.failurePrevious = null; cell.failureNext = null; cell.failureQueued = false;
  }
  #notify(cell: Cell): void {
    cell.initialNotify = false;
    if (cell.active && cell.notify && !cell.failed) {
      try { cell.notify(); }
      catch {
        if (this.#notificationFailures < Number.MAX_SAFE_INTEGER) this.#notificationFailures++;
        if (cell.active) { cell.failed = true; cell.failureQueued = true; cell.failurePrevious = this.#failureTail; if (this.#failureTail) this.#failureTail.failureNext = cell; else this.#failureHead = cell; this.#failureTail = cell; }
      }
    }
  }
  takeNotificationFailure(): { readonly subscription: OwnedUiReadSubscription; readonly reason: "callback-threw" } | null {
    const cell = this.#failureHead; if (!cell) return null; this.#removeFailure(cell); return { subscription: cell.subscription!, reason: "callback-threw" };
  }
  retryNotification(subscription: OwnedUiReadSubscription): boolean {
    const cell = cellOf(subscription); if (!cell || cell.owner !== this || !cell.active || !cell.failed || this.#closing) return false;
    this.#removeFailure(cell); cell.failed = false; cell.initialNotify = true; this.#enqueue(cell); return true;
  }

  #maintain(cell: Cell, grant: NumericIndexGrant): RetainedUiWireStep {
    for (let index = 0; index < 4; index++) {
      const child = cell.sceneClose[index]; if (!child) continue;
      if (child.terminalIsEmpty()) { cell.sceneClose[index] = null; return state("pending", "subscription-scene-release", 64); } return childStep(child.advance(grant), grant);
    }
    if (cell.nodeRetirement) { if (cell.nodeRetirement.terminalIsEmpty()) { cell.nodeRetirement = null; return state("pending", "subscription-node-release", 64); } return childStep(cell.nodeRetirement.advance(grant), grant); }
    if (cell.node && !cell.reader) { cell.nodeRetirement = cell.node.beginClose(); cell.node = null; return state("pending", "subscription-node-close", 64); }
    if (cell.retirement) { if (cell.retirement.terminalIsEmpty()) { cell.retirement = null; return state("pending", "subscription-reader-release", 64); } return childStep(cell.retirement.advance(grant), grant); }
    if (!cell.active) {
      if (cell.reader) { cell.retirement = cell.reader.beginClose(); cell.reader = null; cell.readerComplete = false; return state("pending", "subscription-reader-close", 64); }
      if (cell.lease) { if (cell.lease.terminalIsEmpty()) { cell.lease = null; return state("pending", "subscription-lease-release", 64); } cell.lease.beginClose(); return childStep(cell.lease.closeStep(grant), grant); }
      cell.owner = null; cell.subscription = null; cell.failed = false; return state("complete", "subscription-close", 64);
    }
    if (!cell.initialized) {
      if (cell.id === null) { cell.initialized = true; return state("pending", "view-subscription", 32); }
      if (!cell.reader) { const source = this.#live(); cell.lookupVersion = source.view.revision; cell.reader = source.bindings.beginLookup(cell.id); return state("pending", "subscription-lookup", 64); }
      if (cell.readerComplete) {
        if (cell.lookupVersion === this.view.revision) { cell.lease = new OwnedUiNodeReadLease(cell.id, cell.lookupVersion, cell.node, this.#epoch); cell.initialized = true; }
        cell.retirement = cell.reader.beginClose(); cell.reader = null; cell.readerComplete = false; return state("pending", "subscription-read-captured", 512);
      }
      const result = cell.reader.advance(grant);
      if (result.kind === "value") cell.node = result.value;
      if (!validWork(result, grant)) return { kind: "rejected", phase: "subscription-read-grant", items: result.items, bytes: result.bytes };
      if (result.kind === "value") return { kind: "pending", phase: "subscription-read", items: result.items, bytes: result.bytes };
      if (result.kind === "complete") { cell.readerComplete = true; return { ...result, kind: "pending" }; }
      return result;
    }
    if (cell.initialNotify) { this.#notify(cell); return state("pending", "subscription-initial-notify", 64); }
    if (cell.lease?.retirementPending) return cell.lease.advanceRetirement(grant);
    return state("complete", "subscription-idle");
  }
  advanceMaintenance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "surface-maintenance"); if (!this.#queue) return state("complete", "surface-maintenance");
    if (this.#queueWorked) { const cell = this.#dequeue(); const complete = this.#queueComplete; this.#queueWorked = false; this.#queueComplete = false; if (!complete) this.#enqueue(cell); return state("pending", "surface-maintenance-queue", 64); }
    try {
      const result = this.#maintain(this.#queue, grant); if (!validWork(result, grant)) { this.#maintenanceFailure = "Surface maintenance child exceeded its grant"; return { ...result, kind: "rejected" }; }
      if (result.kind === "blocked" || result.kind === "rejected") { if (result.kind === "rejected") this.#maintenanceFailure = result.phase; return result; }
      this.#maintenanceFailure = null; this.#queueWorked = true; this.#queueComplete = result.kind === "complete"; return { ...result, kind: "pending" };
    } catch (error) { this.#maintenanceFailure = error instanceof Error ? error.message : "Surface maintenance failed"; return state("rejected", "surface-maintenance-failed"); }
  }
  beginClose(): void { if (this.#closing || this.#closed) return; this.#closing = true; this.#patch?.beginClose(); }
  takePendingAcknowledgement(): OwnedUiSurfaceAcknowledgement | null { return this.#patch?.takeAcknowledgement() ?? null; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#closed) return state("complete", "surface-close"); if (!this.#closing) throw new Error("Owned UI surface close has not begun"); if (!admitted(grant)) return state("blocked", "surface-close");
    if (this.#patch) return childStep(this.#patch.closeStep(grant), grant);
    if (this.#head) return state("blocked", "surface-readers");
    if (this.#queue) return this.advanceMaintenance(grant);
    if (this.#retirement) { if (this.#retirement.terminalIsEmpty()) { this.#retirement = null; return state("pending", "surface-nodes-release", 64); } return childStep(this.#retirement.advance(grant), grant); }
    if (this.#bindingRetirement) { if (this.#bindingRetirement.terminalIsEmpty()) { this.#bindingRetirement = null; return state("pending", "surface-bindings-release", 64); } return childStep(this.#bindingRetirement.advance(grant), grant); }
    if (this.#state) { this.#retirement = this.#state.nodes.beginClose(); this.#bindingRetirement = this.#state.bindings.beginClose(); this.#state = null; return state("pending", "surface-root-close", 128); }
    this.#closed = true; return state("complete", "surface-close");
  }
  terminalIsEmpty(): boolean { return this.#closed && !this.#state && !this.#patch && !this.#head && !this.#tail && !this.#queue && !this.#queueTail && !this.#queueWorked && !this.#queueComplete && !this.#retirement && !this.#bindingRetirement && !this.#failureHead && !this.#failureTail; }
}
//#endregion 🖼️SurfaceOwner

//#region 🩹️PrivatePublication
/** 🩹️ Only this owner-created cursor can bind computed graph/hash results to the captured surface root. */
export class OwnedUiSurfacePatch {
  #owner: OwnedUiSurface | null;
  #source: SurfaceState | null;
  #nodes: OwnedUiNodeIndex | null;
  #bindings: OwnedUiSceneBindingIndex | null = null;
  #binding: OwnedUiSceneBinding | null = null;
  #bindingClose: OwnedUiSceneBindingRetirement | null = null;
  #bindingRetirement: OwnedUiSceneBindingIndexRetirement | null = null;
  #bindingReader: OwnedUiSceneBindingIndexReader | null = null;
  #bindingEdit: OwnedUiSceneBindingIndexEdit | null = null;
  #bindingPreparation: OwnedUiSceneBindingCursor | null = null;
  #root: number | null;
  readonly #revision: number;
  readonly #limits: UiDocumentLimits;
  #operation: OwnedUiOperationCursor | null = null;
  #operationTouched: RetainedUiNumericTable<true> | null = null;
  #touched: RetainedUiNumericTable<true>;
  #validation: OwnedUiValidationCursor | null = null;
  #violations: RetainedUiNumericTable<unknown> | null = null;
  #hash: OwnedUiSnapshotHashCursor | null = null;
  #retirement: OwnedUiNodeIndexRetirement | null = null;
  #reader: OwnedUiNodeIndexReader | null = null;
  #node: OwnedUiNode | null = null;
  #nodeRetirement: UiNodeRetirement | null = null;
  #program: Program | null = null;
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #status: "input" | "operation" | "preparing" | "ready" | "rejected" | "closing" | "closed" = "input";
  #phase = "input";
  #finished = false;
  #count = 0;
  #estimatedBytes = 0;
  #failure: string | null = null;
  #epoch: OwnedUiReadCommit | null = null;
  #scan: Cell | null = null;
  #cell: Cell | null = null;
  #notify: Cell | null = null;
  #notifyLimit = 0;
  #ack: OwnedUiSurfaceAcknowledgement | null = null;
  #published = false;
  #maintenanceTurn = true;
  #running = false;
  #staged: Staged | null = null;
  #closeRequested = false;

  private constructor(mint: object, owner: OwnedUiSurface, source: SurfaceState, revision: number, limits: UiDocumentLimits) {
    if (mint !== MINT) throw new Error("Surface patch requires exact mint authority");
    this.#owner = owner; this.#source = source; this.#nodes = source.nodes.capture(); this.#root = source.view.root; this.#revision = revision; this.#limits = limits;
    this.#touched = new RetainedUiNumericTable(NumericIndex.empty<true>(), () => this.#grant); Object.freeze(this);
  }
  static {
    createPatch = (owner, source, revision, limits) => new OwnedUiSurfacePatch(MINT, owner, source, revision, limits);
    appended = (patch, cell) => { if (patch.#phase === "staging" && !patch.#scan) patch.#scan = cell; };
    detached = (patch, cell) => { if (patch.#scan === cell) patch.#scan = cell.next; if (patch.#notify === cell) patch.#notify = cell.next; };
  }
  get failure(): string | null { return this.#failure; }
  get phase(): string { return this.#phase; }
  pushOperation(operation: OwnedUiOperation): void {
    if (this.#status !== "input" || this.#finished || this.#operation || this.#count >= this.#limits.maxPatchOps) throw new Error("Owned UI operation admission rejected");
    this.#operation = new OwnedUiOperationCursor(this.#nodes!, this.#root, operation, this.#limits); this.#count++; this.#status = "operation"; this.#program = this.#apply();
  }
  finishInput(): void { if (this.#status !== "input" || this.#finished) throw new Error("Owned UI input is not available"); this.#finished = true; }
  *#drainIndex(): Program { while (this.#retirement) { const result = this.#retirement.advance(this.#grant); if (result.kind === "complete") this.#retirement = null; yield result.bytes; } }
  *#releaseNode(): Program { if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; yield 64; } while (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(this.#grant); if (result.kind === "complete") this.#nodeRetirement = null; yield result.bytes; } }
  *#apply(): Program {
    for (;;) { const result = this.#operation!.advance(this.#grant); yield result.bytes; if (result.kind === "rejected") throw new Error(this.#operation!.failure ?? "Owned operation failed"); if (result.kind === "ready") break; }
    const result = this.#operation!.takeResult()!; this.#retirement = this.#nodes!.beginClose(); this.#nodes = result.nodes; this.#root = result.root; this.#operationTouched = new RetainedUiNumericTable(result.touched, () => this.#grant); this.#estimatedBytes += result.estimatedBytes; yield 192;
    if (this.#estimatedBytes > this.#limits.maxPatchBytes) throw new Error("Owned UI patch byte quota exceeded");
    yield* this.#drainIndex(); this.#operation!.beginClose(); yield 32;
    while (this.#operation) { const step = this.#operation.closeStep(this.#grant); if (step.kind === "complete") this.#operation = null; yield step.bytes; }
    for (const entry of this.#operationTouched.entries()) { if (typeof entry === "number") yield entry; else yield* this.#touched.set(entry[0], true); }
    while (this.#operationTouched) { const step = this.#operationTouched.closeStep(this.#grant); if (step.complete) this.#operationTouched = null; yield step.bytes; }
  }
  *#lookup(id: number): Program {
    this.#reader = this.#nodes!.beginLookup(id); yield 64;
    for (;;) { const step = this.#reader.advance(this.#grant); if (step.kind === "value") this.#node = step.value; yield step.bytes; if (step.kind === "complete") break; }
    this.#retirement = this.#reader.beginClose(); this.#reader = null; yield 64; yield* this.#drainIndex();
  }
  *#drainBindings(): Program { while (this.#bindingRetirement) { const current = this.#bindingRetirement.advance(this.#grant); if (current.kind === "complete") this.#bindingRetirement = null; yield current.bytes; } }
  *#releaseBinding(): Program { if (this.#binding) { this.#bindingClose = this.#binding.beginClose(); this.#binding = null; yield 64; } while (this.#bindingClose) { const current = this.#bindingClose.advance(this.#grant); if (current.kind === "complete") this.#bindingClose = null; yield current.bytes; } }
  *#lookupBinding(index: OwnedUiSceneBindingIndex, id: number): Program {
    this.#bindingReader = index.beginLookup(id); yield 64;
    for (;;) { const current = this.#bindingReader.advance(this.#grant); if (current.kind === "value") this.#binding = current.value; yield current.bytes; if (current.kind === "complete") break; }
    this.#bindingRetirement = this.#bindingReader.beginClose(); this.#bindingReader = null; yield 64; yield* this.#drainBindings();
  }
  *#prepareBindings(): Program {
    this.#phase = "scenes";
    for (const entry of this.#touched.entries()) {
      if (typeof entry === "number") { yield entry; continue; }
      const id = entry[0]; yield* this.#lookup(id);
      if (this.#node) {
        yield* this.#lookupBinding(this.#source!.bindings, id);
        this.#bindingPreparation = new OwnedUiSceneBindingCursor(this.#node, profileOf(this.#owner!)); yield 64;
        if (this.#binding) this.#bindingPreparation.considerPrevious(this.#binding); yield 64; yield* this.#releaseBinding();
        for (;;) { const current = this.#bindingPreparation.advance(this.#grant); yield current.bytes; if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(this.#bindingPreparation.failure ?? "Scene ownership failed"); }
        this.#binding = this.#bindingPreparation.takeResult()!; this.#bindingPreparation.beginClose(); yield 64;
        while (this.#bindingPreparation) { const current = this.#bindingPreparation.closeStep(this.#grant); if (current.kind === "complete") this.#bindingPreparation = null; yield current.bytes; }
        this.#bindingEdit = this.#bindings!.beginSet(this.#binding); yield 64; yield* this.#releaseBinding();
      } else { this.#bindingEdit = this.#bindings!.beginRemove(id); yield 64; }
      yield* this.#releaseNode();
      for (;;) { const current = this.#bindingEdit!.advance(this.#grant); yield current.bytes; if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(this.#bindingEdit!.failure ?? "Scene index failed"); }
      const next = this.#bindingEdit!.takeResult()!; this.#bindingRetirement = this.#bindings!.beginClose(); this.#bindings = next; yield 128; yield* this.#drainBindings();
      this.#bindingRetirement = this.#bindingEdit!.beginClose(); this.#bindingEdit = null; yield 64; yield* this.#drainBindings();
    }
  }
  *#prepare(): Program {
    this.#bindings = this.#source!.bindings.capture(); yield 64;
    this.#phase = "validation"; this.#validation = new OwnedUiValidationCursor(this.#nodes!, this.#root, this.#limits); yield 256;
    for (;;) { const step = this.#validation.advance(this.#grant); yield step.bytes; if (step.kind === "rejected") throw new Error(this.#validation.failure ?? "Owned UI validation failed"); if (step.kind === "ready") break; }
    const violations = this.#validation.takeResult()!; const valid = violations.size === 0; this.#violations = new RetainedUiNumericTable<unknown>(violations, () => this.#grant); this.#validation.beginClose(); yield 64;
    if (!valid) throw new Error("Owned UI graph invariants violated");
    while (this.#validation) { const step = this.#validation.closeStep(this.#grant); if (step.kind === "complete") this.#validation = null; yield step.bytes; }
    while (this.#violations) { const step = this.#violations.closeStep(this.#grant); if (step.complete) this.#violations = null; yield step.bytes; }
    yield* this.#prepareBindings();
    this.#phase = "hash"; this.#hash = new OwnedUiSnapshotHashCursor(this.#nodes!, { surface: this.#owner!.identity.surface, revision: this.#revision, root: this.#root }); yield 128;
    for (;;) { const step = this.#hash.advance(this.#grant); yield step.bytes; if (step.kind === "rejected") throw new Error(this.#hash.failure ?? "Owned UI hash failed"); if (step.kind === "ready") break; }
    const digest = this.#hash.takeResult()!; this.#hash.beginClose(); yield 64;
    while (this.#hash) { const step = this.#hash.closeStep(this.#grant); if (step.kind === "complete") this.#hash = null; yield step.bytes; }
    this.#phase = "staging"; this.#epoch = epochOf(this.#owner!).begin(this.#revision); this.#scan = firstCell(this.#owner!); yield 96;
    while (this.#scan) {
      this.#cell = this.#scan; this.#scan = this.#cell.next; yield 32;
      while (this.#cell.active && !this.#cell.initialized) yield 16;
      if (this.#cell.active && this.#cell.id !== null && (yield* this.#touched.lookup(this.#cell.id))) {
        while (this.#cell.active && !this.#cell.lease!.hasCapacity) yield 16;
        if (this.#cell.active) {
          yield* this.#lookupBinding(this.#bindings!, this.#cell.id);
          if (this.#cell.active) { if (!this.#cell.lease!.stage(this.#epoch, this.#binding)) throw new Error("Owned UI staging reservation changed"); this.#staged = { cell: this.#cell, next: this.#staged }; }
          yield 544; yield* this.#releaseBinding();
        }
      }
      this.#cell = null; yield 16;
    }
    this.#phase = "publication";
    const old = publish(this.#owner!, this, this.#source!, this.#nodes!, this.#bindings!, this.#root, this.#revision, digest.hash, this.#epoch); this.#nodes = null; this.#bindings = null; this.#published = true; this.#retirement = old.nodes.beginClose(); this.#bindingRetirement = old.bindings.beginClose();
    this.#notify = firstCell(this.#owner!); this.#notifyLimit = lastOrdinal(this.#owner!); this.#phase = "notifications"; yield 256;
    while (this.#notify && this.#notify.ordinal <= this.#notifyLimit) { const cell = this.#notify; this.#notify = cell.next; if (cell.active && (cell.id === null || cell.lease?.snapshot.version === this.#revision)) notifyCell(this.#owner!, cell); yield 64; }
    this.#notify = null; yield* this.#drainIndex(); yield* this.#drainBindings();
    this.#ack = Object.freeze({ ...this.#owner!.identity, revision: this.#revision, hash: digest.hash }); this.#phase = "accepted"; yield 128;
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#running) throw new Error("Reentrant owned UI patch drive");
    if (this.#status === "closing") throw new Error("Owned UI patch is closing");
    if (this.#status === "closed" || this.#status === "ready" || this.#status === "rejected") return state(this.#status === "closed" ? "complete" : this.#status, this.#phase);
    if (!admitted(grant)) return state("blocked", this.#phase);
    this.#grant = grant; this.#running = true;
    try {
      if (this.#maintenanceTurn && this.#owner!.maintenancePending) { this.#maintenanceTurn = false; return this.#owner!.advanceMaintenance(grant); } this.#maintenanceTurn = true;
      if (!this.#program) { if (!this.#finished) return state("ready", "input"); this.#status = "preparing"; this.#program = this.#prepare(); }
      const result = this.#program.next(); if (result.done) { this.#program = null; this.#status = this.#status === "operation" ? "input" : "ready"; return state("ready", this.#phase, 32); }
      if (result.value > grant.maxBytes) throw new Error("Owned UI surface exceeded its byte grant"); return state("pending", this.#phase, result.value);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Owned UI surface failed"; this.#status = "rejected"; this.#program = null; return state("rejected", this.#phase, 64); }
    finally { this.#running = false; }
  }
  takeAcknowledgement(): OwnedUiSurfaceAcknowledgement | null { if (this.#status !== "ready" || !this.#published) return null; const result = this.#ack; this.#ack = null; return result; }
  beginClose(): void {
    if (this.#status === "closed" || this.#status === "closing") return; this.#closeRequested = true;
    if (this.#published && (this.#status !== "ready" || this.#ack)) return;
    this.#startClose();
  }
  #startClose(): void {
    this.#status = "closing"; this.#program = null; this.#scan = null; this.#cell = null; this.#notify = null;
    if (this.#epoch && !this.#published) epochOf(this.#owner!).cancel(this.#epoch);
    this.#operation?.beginClose(); this.#validation?.beginClose(); this.#hash?.beginClose(); this.#bindingPreparation?.beginClose();
  }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#status === "closed") return state("complete", "surface-patch-close"); if (!this.#closeRequested) throw new Error("Owned UI patch close has not begun"); if (!admitted(grant)) return state("blocked", "surface-patch-close");
    if (this.#status !== "closing") {
      if (this.#status === "rejected") return state("rejected", "committed-publication-fault");
      if (this.#status !== "ready") { const result = this.advance(grant); return { ...result, kind: result.kind === "rejected" ? "rejected" : "pending" }; }
      if (this.#ack) return state("blocked", "surface-acknowledgement");
      this.#startClose(); return state("pending", "committed-publication-close", 64);
    }
    if (this.#staged) {
      const entry = this.#staged; const cell = entry.cell!;
      if (!this.#published && cell.lease) {
        if (!cell.active) { enqueueCell(this.#owner!, cell); return this.#owner!.advanceMaintenance(grant); }
        if (cell.lease.retirementPending) return { ...cell.lease.advanceRetirement(grant), kind: "pending" };
      }
      this.#staged = entry.next; entry.next = null; entry.cell = null; return state("pending", "staged-read-release", 48);
    }
    if (this.#bindingClose) { const current = this.#bindingClose.advance(grant); if (current.kind === "complete") this.#bindingClose = null; return { ...current, kind: "pending" }; }
    if (this.#binding) { this.#bindingClose = this.#binding.beginClose(); this.#binding = null; return state("pending", "scene-read-close", 64); }
    if (this.#bindingRetirement) { const current = this.#bindingRetirement.advance(grant); if (current.kind === "complete") this.#bindingRetirement = null; return { ...current, kind: "pending" }; }
    if (this.#bindingReader) { this.#bindingRetirement = this.#bindingReader.beginClose(); this.#bindingReader = null; return state("pending", "scene-reader-close", 64); }
    if (this.#bindingEdit) { this.#bindingRetirement = this.#bindingEdit.beginClose(); this.#bindingEdit = null; return state("pending", "scene-edit-close", 64); }
    if (this.#bindingPreparation) { const current = this.#bindingPreparation.closeStep(grant); if (current.kind === "complete") this.#bindingPreparation = null; return { ...current, kind: "pending" }; }
    if (this.#bindings) { this.#bindingRetirement = this.#bindings.beginClose(); this.#bindings = null; return state("pending", "scene-index-close", 64); }
    if (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(grant); if (result.kind === "complete") this.#nodeRetirement = null; return { ...result, kind: "pending" }; }
    if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; return state("pending", "staging-node-close", 64); }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return state("pending", "staging-reader-close", 64); }
    if (this.#operation) { const result = this.#operation.closeStep(grant); if (result.kind === "complete") this.#operation = null; return { ...result, kind: "pending" }; }
    if (this.#operationTouched) { const result = this.#operationTouched.closeStep(grant); if (result.complete) this.#operationTouched = null; return state("pending", "operation-touched-close", result.bytes); }
    if (this.#validation) { const result = this.#validation.closeStep(grant); if (result.kind === "complete") this.#validation = null; return { ...result, kind: "pending" }; }
    if (this.#violations) { const result = this.#violations.closeStep(grant); if (result.complete) this.#violations = null; return state("pending", "violations-close", result.bytes); }
    if (this.#hash) { const result = this.#hash.closeStep(grant); if (result.kind === "complete") this.#hash = null; return { ...result, kind: "pending" }; }
    if (this.#nodes) { this.#retirement = this.#nodes.beginClose(); this.#nodes = null; return state("pending", "candidate-close", 64); }
    const touched = this.#touched.closeStep(grant); if (!touched.complete) return state("pending", "touched-close", touched.bytes);
    releasePatch(this.#owner!, this); this.#owner = null; this.#source = null; this.#epoch = null; this.#status = "closed"; return state("complete", "surface-patch-close");
  }
  terminalIsEmpty(): boolean { return this.#status === "closed" && !this.#owner && !this.#source && !this.#nodes && !this.#bindings && !this.#binding && !this.#bindingClose && !this.#bindingRetirement && !this.#bindingReader && !this.#bindingEdit && !this.#bindingPreparation && !this.#operation && !this.#operationTouched && !this.#validation && !this.#violations && !this.#hash && !this.#retirement && !this.#reader && !this.#node && !this.#nodeRetirement && !this.#program && !this.#epoch && !this.#scan && !this.#cell && !this.#notify && !this.#ack && !this.#staged; }
}
//#endregion 🩹️PrivatePublication
