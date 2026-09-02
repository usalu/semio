//#region 🧬️InstanceContract
import type { ActorInstanceLifetime } from "../../../../🎭️actor/🚪️lifetime/🟦️.ts";
import type { ActorUiPatchReceipt } from "../../../../🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, type ShardActorActivationLease } from "../../../../🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { UiDocumentLimits } from "../../../../🛂️manifest/🟦️.ts";
import { OwnedUiSurface, type OwnedUiSurfaceAcknowledgement, type OwnedUiSurfaceView } from "../🖼️surface/🟦️.ts";
import { OwnedUiWirePatchCursor } from "../🩹️operations/📥️wire/🟦️.ts";
import type { OwnedUiSceneHostProfile } from "../🎬️scene/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️.ts";
import { OwnedUiResidentInstance } from "../💾️resident/🟦️.ts";
import type { OwnedUiSceneReadSource, OwnedUiReadSubscription, OwnedUiNodeReadSnapshot, OwnedUiIssuedSceneReader, OwnedUiSceneRecordView, OwnedUiSceneTextView } from "../📖️read-lease/🟦️.ts";

type Cell = { owner: OwnedUiInstance | null; name: string; surface: OwnedUiSurface | null; facade: OwnedUiInstanceSurface | null; wire: OwnedUiWirePatchCursor | null; patch: OwnedUiInstancePatch | null; source: OwnedNativeUiPatchAuthority | null; ordinal: number; inputActive: boolean; original: unknown; page: OwnedUiPatchInputRetirement | null; ack: OwnedUiPatchAcknowledgement | null; next: Cell | null; workNext: Cell | null; queued: boolean };
const MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function closeChild(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function surfaceName(value: string): string { if (typeof value !== "string" || value.length > 512 || new TextEncoder().encode(value).length > 512) throw new Error("Native surface name exceeds UiText capacity"); return value; }
function generation(value: bigint): bigint { if (typeof value !== "bigint" || value <= 0n || value > 0xffffffffffffffffn) throw new Error("Invalid native instance generation"); return value; }
let cellOf: (facade: OwnedUiInstanceSurface) => Cell;
let createFacade: (cell: Cell) => OwnedUiInstanceSurface;
let createPatch: (cell: Cell) => OwnedUiInstancePatch;
let createLookup: (owner: OwnedUiInstance, name: string, first: Cell | null) => OwnedUiSurfaceLookup;
let closeLookup: (owner: OwnedUiInstance, lookup: OwnedUiSurfaceLookup) => void;
let appendSurface: (owner: OwnedUiInstance, name: string) => Cell;
let operationAuthority: (owner: OwnedUiInstance) => void;
let enqueue: (owner: OwnedUiInstance, cell: Cell) => void;
let createAcknowledgement: (owner: OwnedUiInstance, source: OwnedNativeUiPatchAuthority, lifetime: ActorInstanceLifetime, value: OwnedUiSurfaceAcknowledgement) => OwnedUiPatchAcknowledgement;
let createRetirement: (owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime) => OwnedUiInstanceRetirement;
let createInputRetirement: (source: OwnedNativeUiPatchAuthority, ordinal: number, original: unknown) => OwnedUiPatchInputRetirement;
let createInputAcceptance: (source: OwnedNativeUiPatchAuthority, ordinal: number, original: unknown) => OwnedUiPatchInputAcceptance;
function live(cell: Cell, operation = false): OwnedUiSurface { if (!cell.owner || !cell.surface) throw new Error("Owned UI instance surface is retired"); if (operation) operationAuthority(cell.owner); return cell.surface; }
function prepareInputRetirement(cell: Cell, wire: OwnedUiWirePatchCursor): OwnedUiPatchInputRetirement | null {
  if (cell.page || !cell.inputActive || !cell.source) return cell.page; const receipt = wire.takePageReceipt(); if (!receipt && !wire.terminalIsEmpty()) return null;
  if (receipt && receipt.ordinal !== cell.ordinal) throw new Error("Native input retirement ordinal mismatch"); cell.page = createInputRetirement(cell.source, cell.ordinal, cell.original); return cell.page;
}
//#endregion 🧬️InstanceContract

//#region 📥️InputRetirementAuthority
/** 📥️ A private accepted claim is issued only after the exact wire input has acquired its payload. */
export class OwnedUiPatchInputAcceptance {
  readonly #source: OwnedNativeUiPatchAuthority;
  readonly #ordinal: number;
  readonly #original: unknown;
  private constructor(mint: object, source: OwnedNativeUiPatchAuthority, ordinal: number, original: unknown) {
    if (mint !== MINT) throw new Error("Invalid UI patch input acceptance authority"); this.#source = source; this.#ordinal = ordinal; this.#original = original; Object.freeze(this);
  }
  static { createInputAcceptance = (source, ordinal, original) => new OwnedUiPatchInputAcceptance(MINT, source, ordinal, original); }
  static matches(claim: unknown, source: object, ordinal: number, original: unknown): claim is OwnedUiPatchInputAcceptance { return claim !== null && typeof claim === "object" && #source in claim && claim.#source === source && claim.#ordinal === ordinal && claim.#original === original; }
}

/** 📥️ Exact native source, ordinal and original operation bind the completed input-retirement obligation. */
export class OwnedUiPatchInputRetirement {
  readonly #source: OwnedNativeUiPatchAuthority;
  readonly #ordinal: number;
  readonly #original: unknown;
  private constructor(mint: object, source: OwnedNativeUiPatchAuthority, ordinal: number, original: unknown) {
    if (mint !== MINT) throw new Error("Invalid UI patch input retirement authority"); this.#source = source; this.#ordinal = ordinal; this.#original = original; Object.freeze(this);
  }
  static { createInputRetirement = (source, ordinal, original) => new OwnedUiPatchInputRetirement(MINT, source, ordinal, original); }
  static matches(token: unknown, source: object, ordinal: number, original: unknown): token is OwnedUiPatchInputRetirement { return token !== null && typeof token === "object" && #source in token && token.#source === source && token.#ordinal === ordinal && token.#original === original; }
  get ordinal(): number { return this.#ordinal; }
}
//#endregion 📥️InputRetirementAuthority

//#region 📨️PublicationAcknowledgementAuthority
export type OwnedUiPatchAcknowledgementValue = OwnedUiSurfaceAcknowledgement & { readonly lifetime: ActorInstanceLifetime; readonly receipt: ActorUiPatchReceipt };
/** 📨️ Private publication evidence remains bound to its original aggregate and exact native source authority. */
export class OwnedUiPatchAcknowledgement {
  readonly #owner: OwnedUiInstance;
  readonly #source: object;
  readonly #value: OwnedUiPatchAcknowledgementValue;
  private constructor(mint: object, owner: OwnedUiInstance, source: OwnedNativeUiPatchAuthority, lifetime: ActorInstanceLifetime, value: OwnedUiSurfaceAcknowledgement) {
    if (mint !== MINT) throw new Error("Invalid UI patch acknowledgement authority");
    const producer = source.value.receipt;
    if (producer.lifetime.activationGeneration !== lifetime.activationGeneration || producer.lifetime.instanceId !== lifetime.instanceId || producer.lifetime.guestLifetime !== lifetime.guestLifetime) throw new Error("Native UI receipt lifetime mismatch");
    const identity = Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime });
    this.#owner = owner; this.#source = source; this.#value = Object.freeze({ actor: value.actor, instance: value.instance, surface: value.surface, revision: value.revision, hash: value.hash, lifetime: identity, receipt: Object.freeze({ lifetime: identity, patchSequence: producer.patchSequence }) }); Object.freeze(this);
  }
  static { createAcknowledgement = (owner, source, lifetime, value) => new OwnedUiPatchAcknowledgement(MINT, owner, source, lifetime, value); }
  static matches(token: unknown, source: object): token is OwnedUiPatchAcknowledgement { return token !== null && typeof token === "object" && #source in token && token.#source === source; }
  get owner(): OwnedUiInstance { return this.#owner; }
  get value(): OwnedUiPatchAcknowledgementValue { return this.#value; }
}
//#endregion 📨️PublicationAcknowledgementAuthority

//#region 🏁️InstanceRetirementAuthority
/** 🏁️ Transfers the exact activation alias only after every UI descendant and receipt obligation has closed. */
export class OwnedUiInstanceRetirement {
  readonly #owner: OwnedUiInstance;
  readonly #activation: ShardActorActivationLease;
  readonly #lifetime: ActorInstanceLifetime;
  private constructor(mint: object, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime) {
    if (mint !== MINT) throw new Error("Invalid UI instance retirement authority");
    this.#owner = owner; this.#activation = activation; this.#lifetime = lifetime; Object.freeze(this);
  }
  static { createRetirement = (owner, activation, lifetime) => new OwnedUiInstanceRetirement(MINT, owner, activation, lifetime); }
  static matches(witness: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): witness is OwnedUiInstanceRetirement {
    return witness !== null && typeof witness === "object" && #owner in witness && witness.#owner === owner && witness.#activation === activation && witness.#lifetime.activationGeneration === lifetime.activationGeneration && witness.#lifetime.instanceId === lifetime.instanceId && witness.#lifetime.guestLifetime === lifetime.guestLifetime;
  }
}
//#endregion 🏁️InstanceRetirementAuthority

//#region 📖️FrozenSurfaceFacade
/** 📖️ Stable read facade exposes no surface mutation or replacement-root authority. */
export class OwnedUiInstanceSurface implements OwnedUiSceneReadSource {
  readonly #cell: Cell;
  private constructor(mint: object, cell: Cell) { if (mint !== MINT) throw new Error("Invalid instance surface authority"); this.#cell = cell; Object.freeze(this); }
  static { cellOf = value => value.#cell; createFacade = cell => new OwnedUiInstanceSurface(MINT, cell); }
  get view(): OwnedUiSurfaceView { return live(this.#cell).view; }
  subscribeView(notify: () => void): OwnedUiReadSubscription { const result = live(this.#cell, true).subscribeView(notify); enqueue(this.#cell.owner!, this.#cell); return result; }
  subscribeNode(id: number, notify: () => void): OwnedUiReadSubscription { const result = live(this.#cell, true).subscribeNode(id, notify); enqueue(this.#cell.owner!, this.#cell); return result; }
  retryNotification(subscription: OwnedUiReadSubscription): boolean { const result = live(this.#cell, true).retryNotification(subscription); if (result) enqueue(this.#cell.owner!, this.#cell); return result; }
  acknowledgeRead(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot): void { if (!this.#cell.owner) return; live(this.#cell).acknowledgeRead(subscription, snapshot); enqueue(this.#cell.owner, this.#cell); }
  unsubscribeNode(subscription: OwnedUiReadSubscription): void { if (!this.#cell.owner) return; live(this.#cell).unsubscribeNode(subscription); enqueue(this.#cell.owner, this.#cell); }
  retireSceneRead(subscription: OwnedUiReadSubscription, reader: OwnedUiIssuedSceneReader): boolean { if (!this.#cell.owner) return false; const result = live(this.#cell).retireSceneRead(subscription, reader); enqueue(this.#cell.owner, this.#cell); return result; }
  openSceneRecord(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source?: number): OwnedUiSceneRecordView | null {
    const result = live(this.#cell, true).openSceneRecord(subscription, snapshot, source); if (!result) return null;
    return Object.freeze({ advance: (grant: NumericIndexGrant) => result.advance(grant), close: () => { const closed = result.close(); if (this.#cell.owner) enqueue(this.#cell.owner, this.#cell); return closed; } });
  }
  openSceneText(subscription: OwnedUiReadSubscription, snapshot: OwnedUiNodeReadSnapshot, source: number): OwnedUiSceneTextView | null {
    const result = live(this.#cell, true).openSceneText(subscription, snapshot, source); if (!result) return null;
    return Object.freeze({ advance: (grant: NumericIndexGrant) => result.advance(grant), close: () => { const closed = result.close(); if (this.#cell.owner) enqueue(this.#cell.owner, this.#cell); return closed; } });
  }
}
//#endregion 📖️FrozenSurfaceFacade

//#region 🔍️OneCellLookup
export class OwnedUiSurfaceLookup {
  #owner: OwnedUiInstance | null;
  #name: string;
  #cell: Cell | null;
  #result: OwnedUiInstanceSurface | null = null;
  #ready = false;
  #closing = false;
  #failure: string | null = null;
  private constructor(mint: object, owner: OwnedUiInstance, name: string, first: Cell | null) { if (mint !== MINT) throw new Error("Invalid instance lookup authority"); this.#owner = owner; this.#name = name; this.#cell = first; Object.freeze(this); }
  static { createLookup = (owner, name, first) => new OwnedUiSurfaceLookup(MINT, owner, name, first); }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-surface-lookup"); if (this.#closing || this.#failure) return step("rejected", "instance-surface-lookup"); if (this.#ready) return step("ready", "instance-surface-lookup");
    try {
      operationAuthority(this.#owner!);
      if (this.#cell) { if (this.#cell.name === this.#name) { this.#result = this.#cell.facade; this.#ready = true; } else this.#cell = this.#cell.next; return step(this.#ready ? "ready" : "pending", "instance-surface-lookup", 2112); }
      this.#result = appendSurface(this.#owner!, this.#name).facade; this.#ready = true; return step("ready", "instance-surface-create", 1024);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Instance lookup failed"; return step("rejected", "instance-surface-lookup", 128); }
  }
  takeResult(): OwnedUiInstanceSurface | null { if (!this.#ready || this.#closing || this.#failure) return null; const result = this.#result; this.#result = null; return result; }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-lookup-close"); if (!this.#closing) throw new Error("Instance lookup close has not begun");
    if (this.#owner) { closeLookup(this.#owner, this); this.#owner = null; this.#cell = null; this.#result = null; this.#name = ""; return step("pending", "instance-lookup-close", 1152); }
    return step("complete", "instance-lookup-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#owner && !this.#cell && !this.#result && this.#name.length === 0; }
}
//#endregion 🔍️OneCellLookup

//#region 📥️OwnedPatchFacade
/** 📥️ Patch work cannot consume its aggregate's outstanding receipt records. */
export class OwnedUiInstancePatch {
  readonly #cell: Cell;
  #closing = false;
  private constructor(mint: object, cell: Cell) { if (mint !== MINT) throw new Error("Invalid instance patch authority"); this.#cell = cell; Object.freeze(this); }
  static { createPatch = cell => new OwnedUiInstancePatch(MINT, cell); }
  #wire(): OwnedUiWirePatchCursor { if (!this.#cell.owner || !this.#cell.wire || this.#cell.patch !== this) throw new Error("Instance patch owner is retired"); return this.#cell.wire; }
  get failure(): string | null { return this.#wire().failure; }
  offer(ordinal: number): boolean {
    const wire = this.#wire(); operationAuthority(this.#cell.owner!); const source = this.#cell.source;
    if (this.#closing || !source || this.#cell.inputActive || this.#cell.page || this.#cell.ack || ordinal !== this.#cell.ordinal || ordinal >= source.value.operationCount) return false;
    const original = source.operation(ordinal); if (!wire.offer(ordinal, original)) return false;
    this.#cell.inputActive = true; this.#cell.original = original;
    if (!source.acceptInput(createInputAcceptance(source, ordinal, original))) throw new Error("Native UI input acceptance claim was refused"); return true;
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-patch"); operationAuthority(this.#cell.owner!); const wire = this.#wire();
    if (this.#cell.page) return step("ready", "instance-input-retirement"); if (prepareInputRetirement(this.#cell, wire)) return step("ready", "instance-input-retirement", 128);
    const current = wire.advance(grant); enqueue(this.#cell.owner!, this.#cell); return this.#cell.inputActive && current.kind === "ready" ? { ...current, kind: "pending" } : current;
  }
  finishInput(): void { operationAuthority(this.#cell.owner!); if (this.#cell.page) throw new Error("Instance still owns the input release receipt"); this.#wire().finishInput(); }
  peekInputReceipt(): OwnedUiPatchInputRetirement | null { this.#wire(); return this.#cell.page; }
  releaseInputReceipt(receipt: OwnedUiPatchInputRetirement): boolean { this.#wire(); if (this.#cell.page !== receipt || !this.#cell.source || !this.#cell.source.releaseInput(receipt)) return false; this.#cell.page = null; this.#cell.original = null; this.#cell.inputActive = false; this.#cell.ordinal++; return true; }
  peekAcknowledgement(): OwnedUiPatchAcknowledgement | null {
    const wire = this.#wire(); if (this.#cell.ack) return this.#cell.ack; const value = wire.takeAcknowledgement(); if (!value) return null; const source = this.#cell.source;
    if (!source || value.actor !== source.value.activation.actorId || value.instance !== source.value.lifetime.instanceId || value.surface !== source.value.surface || value.revision !== source.value.revision) throw new Error("Native UI publication acknowledgement mismatch");
    this.#cell.ack = createAcknowledgement(this.#cell.owner!, source, source.value.lifetime, value); return this.#cell.ack;
  }
  acceptAcknowledgement(receipt: unknown): boolean { this.#wire(); if (!this.#cell.source || !this.#cell.ack || !OwnedNativeUiPatchSubmissionReceipt.matches(receipt, this.#cell.source, this.#cell.ack)) return false; this.#cell.ack = null; return true; }
  beginClose(): void { this.#wire().beginClose(); this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-patch-close"); const wire = this.#wire(); if (this.#cell.page) return step("blocked", "instance-input-retirement"); if (this.#cell.ack) return step("blocked", "instance-receipt-outbox");
    if (wire.terminalIsEmpty()) return prepareInputRetirement(this.#cell, wire) ? step("pending", "instance-input-retirement", 128) : step("complete", "instance-patch-close");
    return closeChild(wire.closeStep(grant), grant);
  }
  terminalIsEmpty(): boolean { return this.#cell.patch !== this || (!this.#cell.inputActive && !this.#cell.page && !this.#cell.ack && this.#wire().terminalIsEmpty()); }
}
//#endregion 📥️OwnedPatchFacade

//#region 🏘️ActivationOwnedInstance
/** 🏘️ Holds one exact guest lifetime and its UI descendants; retirement never requires a live activation. */
export class OwnedUiInstance {
  #activation: ShardActorActivationLease | null;
  readonly #lifetime: ActorInstanceLifetime;
  readonly #limits: UiDocumentLimits;
  readonly #profile: OwnedUiSceneHostProfile;
  #head: Cell | null = null;
  #tail: Cell | null = null;
  #work: Cell | null = null;
  #workTail: Cell | null = null;
  #maintenanceWorked = false;
  #maintenanceFailure: string | null = null;
  #lookup: OwnedUiSurfaceLookup | null = null;
  #closing = false;
  #closed = false;
  #retirement: OwnedUiInstanceRetirement | null = null;
  #resident: OwnedUiResidentInstance | null = null;
  constructor(activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime, limits: UiDocumentLimits, profile: OwnedUiSceneHostProfile) {
    activation.assertActive();
    const activationGeneration = generation(lifetime.activationGeneration); const guestLifetime = generation(lifetime.guestLifetime); const instanceId = lifetime.instanceId;
    if (activation.activationGeneration !== activationGeneration || !Number.isInteger(instanceId) || instanceId < 0 || instanceId > 0xffffffff) throw new Error("Instance capture does not match native lifetime");
    this.#activation = activation; this.#lifetime = Object.freeze({ activationGeneration, instanceId, guestLifetime });
    this.#limits = Object.freeze({ maxNodes: limits.maxNodes, maxDepth: limits.maxDepth, maxChildren: limits.maxChildren, maxTextBytes: limits.maxTextBytes, maxPatchOps: limits.maxPatchOps, maxPatchBytes: limits.maxPatchBytes }); this.#profile = Object.freeze({ usizeBits: profile.usizeBits }); Object.freeze(this);
  }
  static {
    operationAuthority = owner => { if (!owner || owner.#closing || owner.#closed || !owner.#activation) throw new Error("Instance operation owner is closing"); owner.#activation.assertActive(); };
    closeLookup = (owner, lookup) => { if (owner.#lookup !== lookup) throw new Error("Foreign instance lookup close"); owner.#lookup = null; };
    appendSurface = (owner, name) => {
      operationAuthority(owner);
      const surface = new OwnedUiSurface({ actor: owner.#activation!.actorId, instance: owner.#lifetime.instanceId, surface: name }, owner.#limits, owner.#profile);
      const cell: Cell = { owner, name, surface, facade: null, wire: null, patch: null, source: null, ordinal: 0, inputActive: false, original: null, page: null, ack: null, next: null, workNext: null, queued: false }; cell.facade = createFacade(cell);
      if (owner.#tail) owner.#tail.next = cell; else owner.#head = cell; owner.#tail = cell; return cell;
    };
    enqueue = (owner, cell) => { if (owner.#closing || cell.queued || !cell.surface?.maintenancePending) return; cell.queued = true; if (owner.#workTail) owner.#workTail.workNext = cell; else owner.#work = cell; owner.#workTail = cell; };
  }
  #matches(activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): boolean { return activation === this.#activation && lifetime.activationGeneration === this.#lifetime.activationGeneration && lifetime.instanceId === this.#lifetime.instanceId && lifetime.guestLifetime === this.#lifetime.guestLifetime; }
  static matches(owner: unknown, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): owner is OwnedUiInstance { return owner !== null && typeof owner === "object" && #activation in owner && owner.#matches(activation, lifetime); }
  attachResidentScope(scope: OwnedUiResidentInstance): boolean {
    if (this.#closing || this.#closed || !this.#activation || (this.#resident !== null && this.#resident !== scope) || !OwnedUiResidentInstance.matches(scope, this, this.#activation, this.#lifetime)) return false;
    this.#resident = scope; return true;
  }
  beginSurfaceLookup(activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime, name: string): OwnedUiSurfaceLookup | null {
    if (!this.#matches(activation, lifetime)) return null; operationAuthority(this); if (this.#lookup) return null;
    const lookup = createLookup(this, surfaceName(name), this.#head); this.#lookup = lookup; return lookup;
  }
  beginPatch(source: OwnedNativeUiPatchAuthority, facade: OwnedUiInstanceSurface): OwnedUiInstancePatch {
    operationAuthority(this); if (!OwnedNativeUiPatchAuthority.matches(source, this.#activation!, this.#lifetime) || !OwnedNativeUiPatchAuthority.matchesOwner(source, this)) throw new Error("Foreign native instance patch owner"); const cell = cellOf(facade); const value = source.value;
    if (cell.owner !== this || cell.name !== value.surface || !cell.surface || (cell.wire && !cell.wire.terminalIsEmpty()) || cell.page || cell.ack) throw new Error("Foreign or busy instance surface owner");
    const wire = new OwnedUiWirePatchCursor(cell.surface, value.baseRevision, value.revision, value.operationCount); cell.wire = wire; cell.source = source; cell.ordinal = 0; cell.inputActive = false; cell.patch = createPatch(cell); return cell.patch;
  }
  get maintenancePending(): boolean { return this.#work !== null; }
  get maintenanceFailure(): string | null { return this.#maintenanceFailure; }
  advanceMaintenance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-maintenance"); if (!this.#work) return step("complete", "instance-maintenance");
    const cell = this.#work;
    if (this.#maintenanceWorked) { this.#work = cell.workNext; if (!this.#work) this.#workTail = null; cell.workNext = null; cell.queued = false; this.#maintenanceWorked = false; enqueue(this, cell); return step("pending", "instance-maintenance-queue", 64); }
    try {
      const current = live(cell).advanceMaintenance(grant);
      if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) { this.#maintenanceFailure = "Instance maintenance child exceeded its grant"; return { ...current, kind: "rejected" }; }
      if (current.kind === "blocked" || current.kind === "rejected") { if (current.kind === "rejected") this.#maintenanceFailure = current.phase; return current; }
      this.#maintenanceFailure = null; this.#maintenanceWorked = true; return { ...current, kind: "pending" };
    } catch (error) { this.#maintenanceFailure = error instanceof Error ? error.message : "Instance maintenance failed"; return step("rejected", "instance-maintenance-failed"); }
  }
  beginClose(): void { if (this.#closing) return; this.#closing = true; this.#maintenanceWorked = false; this.#resident?.beginClose(); this.#lookup?.beginClose(); }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "instance-close"); if (!this.#closing) throw new Error("Instance close has not begun"); if (this.#closed) return step("complete", "instance-close");
    if (this.#lookup) return closeChild(this.#lookup.closeStep(grant), grant);
    if (this.#work) { const cell = this.#work; this.#work = cell.workNext; if (!this.#work) this.#workTail = null; cell.workNext = null; cell.queued = false; return step("pending", "instance-work-release", 64); }
    const cell = this.#head;
    if (cell) {
      if (cell.page) return step("blocked", "instance-input-retirement"); if (cell.ack) return step("blocked", "instance-receipt-outbox");
      if (cell.wire) {
        if (cell.wire.terminalIsEmpty()) { if (prepareInputRetirement(cell, cell.wire)) return step("pending", "instance-input-retirement", 128); cell.wire = null; cell.patch = null; cell.source = null; return step("pending", "instance-wire-release", 128); }
        cell.wire.beginClose(); return closeChild(cell.wire.closeStep(grant), grant);
      }
      if (cell.surface!.terminalIsEmpty()) { this.#head = cell.next; if (!this.#head) this.#tail = null; cell.next = null; cell.surface = null; cell.facade = null; cell.owner = null; cell.name = ""; return step("pending", "instance-surface-release", 1152); }
      cell.surface!.beginClose(); return closeChild(cell.surface!.closeStep(grant), grant);
    }
    if (this.#resident) {
      if (this.#resident.terminalIsEmpty()) { this.#resident = null; return step("pending", "instance-resident-release", 64); }
      return closeChild(this.#resident.closeStep(grant), grant);
    }
    this.#retirement = createRetirement(this, this.#activation!, this.#lifetime); this.#activation = null; this.#closed = true; return step("complete", "instance-close", 128);
  }
  takeRetirementWitness(): OwnedUiInstanceRetirement | null { if (!this.#closed) return null; const witness = this.#retirement; this.#retirement = null; return witness; }
  terminalIsEmpty(): boolean { return this.#closed && !this.#activation && !this.#head && !this.#tail && !this.#work && !this.#workTail && !this.#lookup && !this.#maintenanceWorked && !this.#resident; }
}
//#endregion 🏘️ActivationOwnedInstance
