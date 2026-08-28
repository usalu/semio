//#region 💾️ResidentAdmission
import type { ActorInstanceLifetime } from "../../../../🎭️actor/🚪️lifetime/🟦️component.ts";
import { ShardClient, type ShardActorActivationLease } from "../../../../🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { OwnedResidentLedger, type ResidentResources } from "../../../../🌱️value/💾️resident/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";
import { OwnedUiInstance } from "../🏘️instance/🟦️component.ts";
import { OwnedUiOperationPayloadBuilder } from "../🩹️operations/📥️wire/📄️pages/🟦️component.ts";
import { uiResidentMetadataEnvelope } from "./🪪️metadata/🟦️component.ts";

export type OwnedUiResidentPoolAdmission = { readonly step: RetainedUiWireStep; readonly pool: OwnedUiResidentPool | null };
type Pool = { ledger: OwnedResidentLedger | null; composition: ShardClient | null; failure: unknown; phase: "open" | "closing" | "closed"; readonly bindings: WeakMap<OwnedUiInstance, OwnedUiResidentInstance>; head: Instance | null; tail: Instance | null; pending: Instance | null; closing: boolean; closed: boolean; facade: OwnedUiResidentPool | null; witness: OwnedUiResidentPoolRetirement | null };
type Instance = { pool: Pool | null; owner: OwnedUiInstance | null; facade: OwnedUiResidentInstance | null; activation: ShardActorActivationLease | null; readonly lifetime: ActorInstanceLifetime; previous: Instance | null; next: Instance | null; head: Payload | null; tail: Payload | null; children: number; closing: boolean; closed: boolean };
type Payload = { instance: Instance | null; facade: OwnedUiResidentPayload | null; previous: Payload | null; next: Payload | null; head: Page | null; tail: Page | null; cursor: Page | null; builder: OwnedUiOperationPayloadBuilder | null; builderReserved: boolean; readerReserved: boolean; pages: number; closing: boolean; closed: boolean };
type Page = { payload: Payload; writer: OwnedUiResidentPage | null; readers: OwnedUiResidentPage | null; previous: Page | null; next: Page | null; readonly length: number; data: Uint8Array | null; written: number; sealed: boolean; references: number; scrubbed: number };
const MINT = Object.freeze({});
const NO_POOL_FAULT = Object.freeze({});
const PAGE_BYTES = 256;
const admitted = (grant: NumericIndexGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= bytes;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let instanceOwner: (state: Instance) => OwnedUiResidentInstance;
let payloadOwner: (state: Payload) => OwnedUiResidentPayload;
let pageOwner: (state: Page) => OwnedUiResidentPage;
let installBuilder: (payload: OwnedUiResidentPayload, builder: OwnedUiOperationPayloadBuilder) => boolean;
let poolWitness: (state: Pool, pool: OwnedUiResidentPool) => OwnedUiResidentPoolRetirement;
let finishPoolWitness: (witness: OwnedUiResidentPoolRetirement, pool: OwnedUiResidentPool) => void;
/** 🔒️ Installs only an already privately minted builder in its exact reserved parent slot. */
export function retainOwnedUiResidentBuilder(payload: OwnedUiResidentPayload, builder: OwnedUiOperationPayloadBuilder): boolean { return installBuilder(payload, builder); }
function active(instance: Instance): boolean {
  if (!instance.pool || instance.pool.closing || instance.closing || instance.closed || !instance.owner || !instance.activation || !OwnedUiInstance.matches(instance.owner, instance.activation, instance.lifetime)) return false;
  try { instance.activation.assertActive(); return true; } catch { return false; }
}
function activePayload(payload: Payload): boolean { return !payload.closing && !payload.closed && payload.instance !== null && active(payload.instance); }
function ownerAvailable(pool: Pool): boolean { return pool.owners < pool.capacity.maxOwners; }
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
//#endregion 💾️ResidentAdmission

//#region 🏦️SharedPool
/** 🏦️ The exact Shard composition owns this charged pool before any fallible finalization. */
export class OwnedUiResidentPool {
  readonly #state: Pool;
  private constructor(mint: object, client: ShardClient, ledger: OwnedResidentLedger, grant: NumericIndexGrant) {
    if (mint !== MINT) throw new Error("Invalid resident pool authority");
    const state: Pool = { ledger, composition: client, failure: NO_POOL_FAULT, phase: "open", bindings: new WeakMap(), head: null, tail: null, pending: null, closing: false, closed: false, facade: this, witness: null }; this.#state = state;
    try {
      const current = client.installUiResidentPool(this, grant);
      if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64 || !client.ownsUiResidentPool(this)) throw new Error("Resident pool installation refused");
      poolWitness(state, this); Object.freeze(this);
    } catch (error) { state.failure = error; state.closing = true; state.phase = "closing"; throw error; }
  }
  static begin(client: ShardClient, ledger: OwnedResidentLedger, grant: NumericIndexGrant): OwnedUiResidentPoolAdmission {
    if (!ShardClient.matchesResidentLedger(client, ledger)) return { step: step("rejected", "resident-pool-composition"), pool: null };
    if (!admitted(grant, 1)) return { step: step("blocked", "resident-pool-admission"), pool: null };
    const prepared = client.prepareUiResidentPool(ledger, grant); const current = childStep(prepared, grant);
    if (current.kind !== "ready" || current.items !== 0 || current.bytes !== 0) return { step: current.kind === "ready" ? { ...current, kind: "pending" } : current, pool: null };
    const bytes = uiResidentMetadataEnvelope("pool").bytes + 64; if (!admitted(grant, bytes)) return { step: step("blocked", "resident-pool-construction"), pool: null };
    try { return { step: step("ready", "resident-pool-construction", bytes), pool: new OwnedUiResidentPool(MINT, client, ledger, { maxItems: 1, maxBytes: 64 }) }; }
    catch { return { step: step("rejected", "resident-pool-construction", bytes), pool: null }; }
  }
  static matchesComposition(pool: unknown, client: ShardClient, ledger: OwnedResidentLedger): pool is OwnedUiResidentPool {
    return pool !== null && typeof pool === "object" && #state in pool && pool.#state.composition === client && pool.#state.ledger === ledger && ShardClient.matchesResidentLedger(client, ledger);
  }
  get usage(): ResidentResources { const state = this.#state; if (!state.ledger) throw new Error("Resident pool is retired"); return state.ledger.usage.data; }
  bindInstance(owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): OwnedUiResidentInstance | null {
    const pool = this.#state; if (pool.closing || !OwnedUiInstance.matches(owner, activation, lifetime)) return null;
    try { activation.assertActive(); } catch { return null; }
    const previous = pool.bindings.get(owner); if (previous) return previous.terminalIsEmpty() ? null : previous;
    if (!ownerAvailable(pool)) return null;
    const state: Instance = { pool, owner, facade: null, activation, lifetime: Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime }), previous: pool.tail, next: null, head: null, tail: null, children: 0, closing: false, closed: false }; const child = instanceOwner(state);
    if (!owner.attachResidentScope(child)) return null; state.facade = child; if (pool.tail) pool.tail.next = state; else pool.head = state; pool.tail = state; pool.owners++; pool.bindings.set(owner, child); return child;
  }
  beginClose(): void { const state = this.#state; state.closing = true; if (state.phase === "open") state.phase = "closing"; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-pool-close"); const state = this.#state;
    if (!state.closing) throw new Error("Resident pool close has not begun"); if (state.closed) return step("complete", "resident-pool-close");
    if (!state.witness) { try { poolWitness(state, this); return step("pending", "resident-pool-witness", 64); } catch { return step("rejected", "resident-pool-witness"); } }
    if (state.head) { state.head.facade!.beginClose(); return childStep(state.head.facade!.closeStep(grant), grant); }
    if (state.pending || state.tail) return step("blocked", "resident-pool-children");
    if (state.failure !== NO_POOL_FAULT) return step("rejected", "resident-pool-unretired-fault");
    state.ledger = null; state.composition = null; state.facade = null; state.phase = "closed"; state.closed = true; finishPoolWitness(state.witness, this); return step("complete", "resident-pool-close", 64);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.head && !state.tail && !state.pending && !state.ledger && !state.composition && !state.facade; }
  get retirement(): OwnedUiResidentPoolRetirement | null { return this.#state.closed ? this.#state.witness : null; }
}
/** 🧾️ Preadmitted proof of this exact pool's domain emptiness, separate from its composition-held intrinsic record. */
export class OwnedUiResidentPoolRetirement {
  readonly #pool: OwnedUiResidentPool;
  #terminal = false;
  private constructor(mint: object, state: Pool, pool: OwnedUiResidentPool) { if (mint !== MINT) throw new Error("Invalid pool retirement authority"); this.#pool = pool; state.witness = this; Object.freeze(this); }
  static {
    poolWitness = (state, pool) => new OwnedUiResidentPoolRetirement(MINT, state, pool);
    finishPoolWitness = (witness, pool) => { if (witness.#pool !== pool) throw new Error("Pool retirement identity differs"); witness.#terminal = true; };
  }
  static matches(witness: unknown, pool: OwnedUiResidentPool, client: ShardClient, ledger: OwnedResidentLedger): witness is OwnedUiResidentPoolRetirement {
    return witness !== null && typeof witness === "object" && #pool in witness && witness.#pool === pool && witness.#terminal && ShardClient.matchesResidentLedger(client, ledger) && Reflect.apply(ShardClient.prototype.ownsUiResidentPool, client, [pool]);
  }
}
//#endregion 🏦️SharedPool

//#region 🪪️LifetimeScope
/** 🪪️ One exact native lifetime owns all payload scopes admitted through this host pool. */
export class OwnedUiResidentInstance {
  readonly #state: Instance;
  private constructor(mint: object, state: Instance) { if (mint !== MINT) throw new Error("Invalid resident instance authority"); this.#state = state; Object.freeze(this); }
  static { instanceOwner = state => new OwnedUiResidentInstance(MINT, state); }
  static matches(scope: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): scope is OwnedUiResidentInstance {
    if (scope === null || typeof scope !== "object" || !(#state in scope)) return false; const state = scope.#state;
    return !state.closed && state.owner === owner && state.activation === activation && state.lifetime.activationGeneration === lifetime.activationGeneration && state.lifetime.instanceId === lifetime.instanceId && state.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  beginPayload(): OwnedUiResidentPayload | null {
    const state = this.#state; if (!active(state) || !ownerAvailable(state.pool!)) return null;
    const payload: Payload = { instance: state, facade: null, previous: state.tail, next: null, head: null, tail: null, cursor: null, builder: null, builderReserved: false, readerReserved: false, pages: 0, closing: false, closed: false }; const child = payloadOwner(payload); payload.facade = child;
    if (state.tail) state.tail.next = payload; else state.head = payload; state.tail = payload; state.pool!.owners++; state.children++; return child;
  }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-instance-close"); const state = this.#state;
    if (!state.closing) throw new Error("Resident instance close has not begun"); if (state.closed) return step("complete", "resident-instance-close");
    if (state.head) { state.head.facade!.beginClose(); return childStep(state.head.facade!.closeStep(grant), grant); }
    if (state.children) return step("blocked", "resident-instance-children"); const pool = state.pool!;
    if (state.previous) state.previous.next = state.next; else pool.head = state.next; if (state.next) state.next.previous = state.previous; else pool.tail = state.previous;
    pool.bindings.delete(state.owner!); pool.owners--; state.previous = null; state.next = null; state.facade = null; state.pool = null; state.owner = null; state.activation = null; state.closed = true; return step("complete", "resident-instance-close", 64);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.pool && !state.owner && !state.facade && !state.activation && !state.head && !state.tail && !state.previous && !state.next && state.children === 0; }
}
//#endregion 🪪️LifetimeScope

//#region 📦️PayloadScope
/** 📦️ Keeps page reservations and shared aliases charged until their final explicit retirement. */
export class OwnedUiResidentPayload {
  readonly #state: Payload;
  private constructor(mint: object, state: Payload) { if (mint !== MINT) throw new Error("Invalid resident payload authority"); this.#state = state; Object.freeze(this); }
  static {
    payloadOwner = state => new OwnedUiResidentPayload(MINT, state);
    installBuilder = (payload, builder) => {
      const state = payload.#state; if (!state.builderReserved || state.builder || !OwnedUiOperationPayloadBuilder.matchesResident(builder, payload)) return false;
      state.builder = builder; return true;
    };
  }
  static matchesOwner(payload: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): payload is OwnedUiResidentPayload {
    if (payload === null || typeof payload !== "object" || !(#state in payload)) return false; const state = payload.#state; const instance = state.instance;
    return activePayload(state) && instance !== null && instance.owner === owner && instance.activation === activation && instance.lifetime.activationGeneration === lifetime.activationGeneration && instance.lifetime.instanceId === lifetime.instanceId && instance.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  reserveBuilder(): boolean {
    const state = this.#state; if (!activePayload(state) || state.builderReserved || !ownerAvailable(state.instance!.pool!)) return false;
    state.instance!.pool!.owners++; state.builderReserved = true; return true;
  }
  reserveReader(builder: OwnedUiOperationPayloadBuilder): boolean {
    const state = this.#state; if (!activePayload(state) || state.builder !== builder || state.readerReserved || !ownerAvailable(state.instance!.pool!)) return false;
    state.instance!.pool!.owners++; state.readerReserved = true; return true;
  }
  releaseReader(builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-reader-release"); const state = this.#state;
    if (state.builder !== builder || !state.readerReserved || !OwnedUiOperationPayloadBuilder.readerIsEmpty(builder)) return step("rejected", "resident-reader-release");
    state.readerReserved = false; state.instance!.pool!.owners--; return step("complete", "resident-reader-release", 64);
  }
  reservePage(length: number): OwnedUiResidentPage | null {
    const state = this.#state; if (!Number.isInteger(length) || length < 1 || length > PAGE_BYTES || !activePayload(state)) return null;
    const pool = state.instance!.pool!; if (!ownerAvailable(pool) || pool.pages >= pool.capacity.maxPages || PAGE_BYTES > pool.capacity.maxResidentBytes - pool.bytes) return null;
    const page: Page = { payload: state, writer: null, readers: null, previous: state.tail, next: null, length, data: null, written: 0, sealed: false, references: 1, scrubbed: 0 }; const child = pageOwner(page); page.writer = child;
    if (state.tail) state.tail.next = page; else state.head = page; state.tail = page; pool.bytes += PAGE_BYTES; pool.pages++; pool.owners++; state.pages++; return child;
  }
  beginClose(): void { const state = this.#state; if (state.closing) return; state.closing = true; state.cursor = state.head; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-payload-close"); const state = this.#state;
    if (!state.closing) throw new Error("Resident payload close has not begun"); if (state.closed) return step("complete", "resident-payload-close");
    if (state.builder && !state.builder.terminalIsEmpty()) {
      try { state.builder.beginClose(); return childStep(state.builder.closeStep(grant), grant); } catch { return step("rejected", "resident-builder-close-fault"); }
    }
    if (state.readerReserved) { state.readerReserved = false; state.instance!.pool!.owners--; return step("pending", "resident-reader-release", 64); }
    if (state.builderReserved) { state.builder = null; state.builderReserved = false; state.instance!.pool!.owners--; return step("pending", "resident-builder-release", 64); }
    if (state.cursor) { const page = state.cursor; if (page.writer) { page.writer.beginClose(); return childStep(page.writer.closeStep(grant), grant); } state.cursor = page.next; return step("pending", "resident-page-next", 64); }
    if (state.pages || state.head) return step("blocked", "resident-payload-readers"); const instance = state.instance!;
    if (state.previous) state.previous.next = state.next; else instance.head = state.next; if (state.next) state.next.previous = state.previous; else instance.tail = state.previous;
    instance.pool!.owners--; instance.children--; state.previous = null; state.next = null; state.facade = null; state.instance = null; state.closed = true; return step("complete", "resident-payload-close", 64);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.instance && !state.facade && !state.head && !state.tail && !state.cursor && !state.previous && !state.next && !state.builder && !state.builderReserved && !state.readerReserved && state.pages === 0; }
}
//#endregion 📦️PayloadScope

//#region 📄️FixedPageOwner
/** 📄️ Private page storage has no exposed buffer, mutable alias or caller-supplied release certificate. */
export class OwnedUiResidentPage {
  #state: Page | null;
  #previous: OwnedUiResidentPage | null = null;
  #next: OwnedUiResidentPage | null = null;
  #closing = false;
  private constructor(mint: object, state: Page) { if (mint !== MINT) throw new Error("Invalid resident page authority"); this.#state = state; Object.freeze(this); }
  static { pageOwner = state => new OwnedUiResidentPage(MINT, state); }
  get length(): number { if (!this.#state) throw new Error("Resident page is retired"); return this.#state.length; }
  allocate(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, PAGE_BYTES)) return step("blocked", "resident-page-allocate"); const state = this.#state;
    if (!state || this.#closing || !activePayload(state.payload)) return step("rejected", "resident-page-allocate"); if (state.data) return step("ready", "resident-page-allocate");
    try { state.data = new Uint8Array(PAGE_BYTES); return step("ready", "resident-page-allocate", PAGE_BYTES); } catch { return step("rejected", "resident-page-allocation-failed"); }
  }
  writeByte(value: number, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 1)) return step("blocked", "resident-page-write"); const state = this.#state;
    if (!state || this.#closing || !state.data || state.sealed || state.written >= state.length || !activePayload(state.payload) || !Number.isInteger(value) || value < 0 || value > 255) return step("rejected", "resident-page-write");
    state.data[state.written++] = value; return step("pending", "resident-page-write", 1);
  }
  seal(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-page-seal"); const state = this.#state;
    if (!state || this.#closing || !state.data || state.written !== state.length || !activePayload(state.payload)) return step("rejected", "resident-page-seal"); state.sealed = true; return step("ready", "resident-page-seal", 64);
  }
  capture(): OwnedUiResidentPage | null {
    const state = this.#state; if (!state || this.#closing || !state.sealed || !activePayload(state.payload)) return null;
    const pool = state.payload.instance!.pool!; if (!ownerAvailable(pool) || state.references >= Number.MAX_SAFE_INTEGER) return null;
    const child = pageOwner(state); child.#next = state.readers; if (state.readers) state.readers.#previous = child; state.readers = child; pool.owners++; state.references++; return child;
  }
  byteAt(index: number): number {
    const state = this.#state; if (!state || this.#closing || !state.sealed || !state.data || !Number.isInteger(index) || index < 0 || index >= state.length) throw new Error("Invalid resident page read"); return state.data[index]!;
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 1)) return step("blocked", "resident-page-close"); if (!this.#closing) throw new Error("Resident page close has not begun"); const state = this.#state;
    if (!state) return step("complete", "resident-page-close"); const pool = state.payload.instance!.pool!;
    if (state.references === 1 && state.data) {
      const bytes = Math.min(PAGE_BYTES - state.scrubbed, grant.maxBytes); state.data.fill(0, state.scrubbed, state.scrubbed + bytes); state.scrubbed += bytes; if (state.scrubbed === PAGE_BYTES) state.data = null; return step("pending", "resident-page-scrub", bytes);
    }
    if (!admitted(grant, 64)) return step("blocked", "resident-page-release"); state.references--; pool.owners--; this.#state = null;
    if (state.writer === this) state.writer = null; else { if (this.#previous) this.#previous.#next = this.#next; else state.readers = this.#next; if (this.#next) this.#next.#previous = this.#previous; this.#previous = null; this.#next = null; }
    if (state.references === 0) { const payload = state.payload; if (payload.cursor === state) payload.cursor = state.next; if (state.previous) state.previous.next = state.next; else payload.head = state.next; if (state.next) state.next.previous = state.previous; else payload.tail = state.previous; state.previous = null; state.next = null; pool.bytes -= PAGE_BYTES; pool.pages--; payload.pages--; } return step("complete", "resident-page-close", 64);
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#state === null && !this.#previous && !this.#next; }
}
//#endregion 📄️FixedPageOwner
