//#region 💾️ResidentDomain
export type ResidentResources = { readonly bytes: number; readonly slots: number; readonly owners: number };
export type ResidentCapacity = ResidentResources & { readonly control: ResidentResources };
export type ResidentGrant = { readonly maxItems: number; readonly maxBytes: number };
export type ResidentStep = { readonly kind: "blocked" | "pending" | "ready" | "complete" | "rejected"; readonly phase: string; readonly items: number; readonly bytes: number };
export type ResidentPartition = "data" | "control";
export type ResidentOwnerAdmission = { readonly step: ResidentStep; readonly owner: OwnedResidentOwner | null };
export type ResidentPageAdmission = { readonly step: ResidentStep; readonly page: OwnedResidentPage | null };
export type ResidentReaderAdmission = { readonly step: ResidentStep; readonly reader: OwnedResidentReader | null };
type Counts = { bytes: number; slots: number; owners: number };
type Ledger = { readonly capacity: ResidentCapacity; readonly maximum: { readonly data: ResidentResources; readonly control: ResidentResources }; readonly used: { data: Counts; control: Counts }; head: Owner | null; tail: Owner | null; cursor: Owner | null; records: DomainRecord | null; recordTail: DomainRecord | null; closing: boolean; closed: boolean };
type DomainRecord = { ledger: Ledger | null; readonly partition: ResidentPartition; readonly charge: ResidentResources; readonly final: Final; facade: OwnedResidentRecord | null; observation: OwnedResidentRecordDetachment | null; previous: DomainRecord | null; next: DomainRecord | null; shell: object | null; original: object | null; installed: boolean; detached: boolean; closing: boolean; closed: boolean };
type Final = { root: object | null; facade: OwnedResidentRetirement | null; terminal: boolean };
type Owner = { ledger: Ledger | null; readonly partition: ResidentPartition; readonly final: Final; facade: OwnedResidentOwner | null; previous: Owner | null; next: Owner | null; pages: Page | null; pageTail: Page | null; readers: Reader | null; readerTail: Reader | null; external: External | null; externalTail: External | null; closing: boolean; closed: boolean };
type Page = { owner: Owner | null; readonly final: Final; readonly length: number; facade: OwnedResidentPage | null; previous: Page | null; next: Page | null; data: Uint8Array | null; written: number; scrubbed: number; references: number; sealed: boolean; closing: boolean; closed: boolean };
type External = { owner: Owner | null; readonly final: Final; readonly maximumBytes: number; readonly charge: ResidentResources; facade: OwnedResidentExternalBacking | null; custody: OwnedResidentBackingCustody | null; previous: External | null; next: External | null; backing: ArrayBuffer | null; data: Uint8Array | null; length: number; scrubbed: number; references: number; receiving: boolean; receivedBacking: boolean; sealed: boolean; failed: boolean; closing: boolean; closed: boolean };
type Reader = { owner: Owner | null; page: Page | External | null; readonly final: Final; facade: OwnedResidentReader | null; previous: Reader | null; next: Reader | null; closing: boolean; closed: boolean };
const MINT = Object.freeze({});
const OWNER = Object.freeze({ bytes: 192, slots: 2, owners: 2 });
const PAGE = Object.freeze({ bytes: 512, slots: 3, owners: 2 });
const READER = Object.freeze({ bytes: 128, slots: 2, owners: 2 });
const admitted = (grant: ResidentGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
const step = (kind: ResidentStep["kind"], phase: string, bytes = 0): ResidentStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let createOwner: (state: Owner) => OwnedResidentOwner;
let createPage: (state: Page) => OwnedResidentPage;
let createReader: (state: Reader) => OwnedResidentReader;
let pageState: (value: unknown) => Page | null;
let createWitness: (state: Final, root: object) => OwnedResidentRetirement;
let createExternal: (state: External) => OwnedResidentExternalBacking;
let externalState: (value: unknown) => External | null;
let createCustody: (state: External) => OwnedResidentBackingCustody;
let createRecord: (state: DomainRecord) => OwnedResidentRecord;
let createDetachment: (state: DomainRecord) => OwnedResidentRecordDetachment;
const transferBacking = globalThis.structuredClone.bind(globalThis);
const bufferExtent = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength")!.get!;
const bufferResizable = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "resizable")?.get;
function checkedBuffer(value: unknown): value is ArrayBuffer {
  try { const extent: unknown = Reflect.apply(bufferExtent, value, []); return typeof extent === "number" && Number.isSafeInteger(extent) && extent >= 0 && (!bufferResizable || Reflect.apply(bufferResizable, value, []) === false); } catch { return false; }
}
const finalState = (): Final => ({ root: null, facade: null, terminal: false });
function reserve(ledger: Ledger, partition: ResidentPartition, charge: ResidentResources): boolean {
  const used = ledger.used[partition]; const maximum = ledger.maximum[partition];
  if (charge.bytes > maximum.bytes - used.bytes || charge.slots > maximum.slots - used.slots || charge.owners > maximum.owners - used.owners) return false;
  used.bytes += charge.bytes; used.slots += charge.slots; used.owners += charge.owners; return true;
}
function refund(ledger: Ledger, partition: ResidentPartition, charge: ResidentResources): void { const used = ledger.used[partition]; used.bytes -= charge.bytes; used.slots -= charge.slots; used.owners -= charge.owners; }
function forward(current: ResidentStep, grant: ResidentGrant): ResidentStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function live(owner: Owner): boolean { return owner.ledger !== null && !owner.closing && !owner.closed && !owner.ledger.closing; }
//#endregion 💾️ResidentDomain

//#region 🏦️CompositionLedger
/** 💾️ One explicitly supplied composition capacity; no actor or UI authority is minted. */
export class OwnedResidentLedger {
  readonly #state: Ledger;
  constructor(capacity: ResidentCapacity) {
    const bytes = capacity.bytes; const slots = capacity.slots; const owners = capacity.owners;
    const control = { bytes: capacity.control.bytes, slots: capacity.control.slots, owners: capacity.control.owners };
    if (![bytes, slots, owners, control.bytes, control.slots, control.owners].every(value => Number.isSafeInteger(value) && value >= 0) || control.bytes > bytes || control.slots > slots || control.owners > owners) throw new Error("Invalid shared resident capacity");
    const exact = Object.freeze({ bytes, slots, owners, control: Object.freeze(control) });
    this.#state = { capacity: exact, maximum: { data: { bytes: bytes - control.bytes, slots: slots - control.slots, owners: owners - control.owners }, control: exact.control }, used: { data: { bytes: 0, slots: 0, owners: 0 }, control: { bytes: 0, slots: 0, owners: 0 } }, head: null, tail: null, cursor: null, records: null, recordTail: null, closing: false, closed: false }; Object.freeze(this);
  }
  get capacity(): ResidentCapacity { return this.#state.capacity; }
  get usage(): { readonly data: ResidentResources; readonly control: ResidentResources } { const state = this.#state; return Object.freeze({ data: Object.freeze({ ...state.used.data }), control: Object.freeze({ ...state.used.control }) }); }
  beginOwner(partition: ResidentPartition, grant: ResidentGrant): ResidentOwnerAdmission {
    const ledger = this.#state; if (!admitted(grant, OWNER.bytes)) return { step: step("blocked", "resident-owner-admission"), owner: null };
    if (ledger.closing || (partition !== "data" && partition !== "control")) return { step: step("rejected", "resident-owner-admission"), owner: null };
    if (!reserve(ledger, partition, OWNER)) return { step: step("blocked", "resident-owner-capacity"), owner: null };
    const state: Owner = { ledger, partition, final: finalState(), facade: null, previous: ledger.tail, next: null, pages: null, pageTail: null, readers: null, readerTail: null, external: null, externalTail: null, closing: false, closed: false };
    try { return { step: step("ready", "resident-owner-admission", OWNER.bytes), owner: createOwner(state) }; }
    catch { state.closing = true; return { step: step("rejected", "resident-owner-construction", OWNER.bytes), owner: state.facade }; }
  }
  reserveRecord(partition: ResidentPartition, envelope: ResidentResources, grant: ResidentGrant): { readonly step: ResidentStep; readonly record: OwnedResidentRecord | null } {
    const state = this.#state; if (!admitted(grant, 256)) return { step: step("blocked", "resident-record-admission"), record: null };
    const bytes = envelope.bytes; const slots = envelope.slots; const owners = envelope.owners;
    if (state.closing || (partition !== "data" && partition !== "control") || !Number.isSafeInteger(bytes) || bytes < 0 || bytes > Number.MAX_SAFE_INTEGER - 256 || !Number.isSafeInteger(slots) || slots < 0 || slots > Number.MAX_SAFE_INTEGER - 3 || !Number.isSafeInteger(owners) || owners < 0 || owners > Number.MAX_SAFE_INTEGER - 3) return { step: step("rejected", "resident-record-admission"), record: null };
    const charge = { bytes: bytes + 256, slots: slots + 3, owners: owners + 3 }; if (!reserve(state, partition, charge)) return { step: step("blocked", "resident-record-capacity"), record: null };
    const recordState: DomainRecord = { ledger: state, partition, charge, final: finalState(), facade: null, observation: null, previous: state.recordTail, next: null, shell: null, original: null, installed: false, detached: false, closing: false, closed: false };
    try { return { step: step("ready", "resident-record-admission", 256), record: createRecord(recordState) }; }
    catch { recordState.closing = true; return { step: step("rejected", "resident-record-construction", 256), record: recordState.facade }; }
  }
  beginClose(): void { const state = this.#state; if (!state.closing) { state.closing = true; state.cursor = state.head; } }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-ledger-close"); if (!state.closing) return step("rejected", "resident-ledger-not-closing"); if (state.closed) return step("complete", "resident-ledger-close");
    const current = state.cursor ?? state.head;
    if (current) { state.cursor = current.next ?? state.head; try { current.facade!.beginClose(); return forward(current.facade!.closeStep(grant), grant); } catch { return step("rejected", "resident-owner-close-fault"); } }
    if (state.records) { try { state.records.facade!.beginClose(); return forward(state.records.facade!.closeStep(grant), grant); } catch { return step("rejected", "resident-record-close-fault"); } }
    if (!admitted(grant, 128)) return step("blocked", "resident-ledger-unlink"); const used = state.used;
    if (used.data.bytes || used.data.slots || used.data.owners || used.control.bytes || used.control.slots || used.control.owners) return step("rejected", "resident-ledger-invariant");
    state.cursor = null; state.closed = true; return step("complete", "resident-ledger-close", 128);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.head && !state.tail && !state.cursor && !state.records && !state.recordTail && state.used.data.bytes === 0 && state.used.data.slots === 0 && state.used.data.owners === 0 && state.used.control.bytes === 0 && state.used.control.slots === 0 && state.used.control.owners === 0; }
}
//#endregion 🏦️CompositionLedger

//#region 🪪️DomainRecordCapability
/** 🪪️ Kept in typed composition private state; this capability never validates a domain terminal predicate. */
export class OwnedResidentRecord {
  readonly #state: DomainRecord;
  private constructor(mint: object, state: DomainRecord) {
    if (mint !== MINT) throw new Error("Invalid resident record authority"); this.#state = state; state.facade = this;
    const ledger = state.ledger!; if (ledger.recordTail) ledger.recordTail.next = state; else ledger.records = state; ledger.recordTail = state; createWitness(state.final, this); createDetachment(state); Object.freeze(this);
  }
  static { createRecord = state => new OwnedResidentRecord(MINT, state); }
  install(shell: object, grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 64)) return step("blocked", "resident-record-install");
    if (shell === null || typeof shell !== "object" || state.installed || state.closing || !state.ledger || state.ledger.closing || !state.final.facade || !state.observation) return step("rejected", "resident-record-install");
    state.shell = shell; state.original = shell; state.installed = true; return step("ready", "resident-record-install", 64);
  }
  matchesShell(shell: unknown): boolean { const state = this.#state; return state.installed && !state.detached && !state.closed && state.shell === shell; }
  beginClose(): void { this.#state.closing = true; }
  detach(shell: object, grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 64)) return step("blocked", "resident-record-detach");
    if (!state.closing || !state.installed || state.detached || state.closed || state.shell !== shell || !state.observation) return step("rejected", "resident-record-detach");
    state.shell = null; state.detached = true; return step("pending", "resident-record-detach", 64);
  }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 256)) return step("blocked", "resident-record-close"); if (!state.closing) return step("rejected", "resident-record-not-closing"); if (state.closed) return step("complete", "resident-record-close");
    if (state.shell) return step("blocked", "resident-record-installed"); if (!state.final.facade || (state.installed && (!state.detached || !state.observation))) return step("rejected", "resident-record-witness");
    const ledger = state.ledger!; if (state.previous) state.previous.next = state.next; else ledger.records = state.next; if (state.next) state.next.previous = state.previous; else ledger.recordTail = state.previous;
    state.final.terminal = true; refund(ledger, state.partition, state.charge); state.previous = null; state.next = null; state.facade = null; state.ledger = null; state.closed = true; return step("complete", "resident-record-close", 256);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.ledger && !state.facade && !state.previous && !state.next && !state.shell; }
  get retirement(): OwnedResidentRetirement | null { return this.#state.final.terminal ? this.#state.final.facade : null; }
  get detachment(): OwnedResidentRecordDetachment | null { return this.#state.detached ? this.#state.observation : null; }
}
/** 🧾️ Stable exact detach observation; recovering a lost return does not repeat mutation or certify the shell's domain. */
export class OwnedResidentRecordDetachment {
  readonly #state: DomainRecord;
  private constructor(mint: object, state: DomainRecord) { if (mint !== MINT) throw new Error("Invalid record detachment authority"); this.#state = state; state.observation = this; Object.freeze(this); }
  static { createDetachment = state => new OwnedResidentRecordDetachment(MINT, state); }
  static matches(value: unknown, record: unknown, shell: unknown): value is OwnedResidentRecordDetachment { return value !== null && typeof value === "object" && #state in value && value.#state.detached && value.#state.final.root === record && value.#state.original === shell; }
}
//#endregion 🪪️DomainRecordCapability

//#region 🪪️StrongOwner
/** 🪪️ Intrinsic storage registration, not an actor/UI lifetime or semantic close certificate. */
export class OwnedResidentOwner {
  readonly #state: Owner;
  private constructor(mint: object, state: Owner) {
    if (mint !== MINT) throw new Error("Invalid resident owner authority"); this.#state = state; state.facade = this;
    const ledger = state.ledger!; if (ledger.tail) ledger.tail.next = state; else ledger.head = state; ledger.tail = state; createWitness(state.final, this); Object.freeze(this);
  }
  static { createOwner = state => new OwnedResidentOwner(MINT, state); }
  reservePage(length: number, grant: ResidentGrant): ResidentPageAdmission {
    const owner = this.#state; if (!admitted(grant, 256)) return { step: step("blocked", "resident-page-admission"), page: null };
    if (!Number.isInteger(length) || length < 0 || length > 256 || !live(owner)) return { step: step("rejected", "resident-page-admission"), page: null };
    if (!reserve(owner.ledger!, owner.partition, PAGE)) return { step: step("blocked", "resident-page-capacity"), page: null };
    const state: Page = { owner, final: finalState(), length, facade: null, previous: owner.pageTail, next: null, data: null, written: 0, scrubbed: 0, references: 0, sealed: false, closing: false, closed: false };
    try { return { step: step("ready", "resident-page-admission", 256), page: createPage(state) }; }
    catch { state.closing = true; return { step: step("rejected", "resident-page-construction", 256), page: state.facade }; }
  }
  beginRead(page: unknown, grant: ResidentGrant): ResidentReaderAdmission {
    const owner = this.#state; const source = pageState(page) ?? externalState(page);
    if (!admitted(grant, READER.bytes)) return { step: step("blocked", "resident-reader-admission"), reader: null };
    if (!live(owner) || !source || source.closing || source.closed || !source.sealed || !source.owner || !live(source.owner) || source.owner.ledger !== owner.ledger || source.references >= Number.MAX_SAFE_INTEGER) return { step: step("rejected", "resident-reader-admission"), reader: null };
    if (!reserve(owner.ledger!, owner.partition, READER)) return { step: step("blocked", "resident-reader-capacity"), reader: null };
    source.references++; const state: Reader = { owner, page: source, final: finalState(), facade: null, previous: owner.readerTail, next: null, closing: false, closed: false };
    try { return { step: step("ready", "resident-reader-admission", READER.bytes), reader: createReader(state) }; }
    catch { state.closing = true; return { step: step("rejected", "resident-reader-construction", READER.bytes), reader: state.facade }; }
  }
  reserveExternalBacking(maximumBytes: number, grant: ResidentGrant): { readonly step: ResidentStep; readonly slot: OwnedResidentExternalBacking | null } {
    const owner = this.#state; if (!admitted(grant, 320)) return { step: step("blocked", "resident-external-admission"), slot: null };
    if (!live(owner) || !Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || maximumBytes > Number.MAX_SAFE_INTEGER - 320) return { step: step("rejected", "resident-external-admission"), slot: null };
    const charge = { bytes: maximumBytes + 320, slots: 4, owners: 3 };
    if (!reserve(owner.ledger!, owner.partition, charge)) return { step: step("blocked", "resident-external-capacity"), slot: null };
    const state: External = { owner, final: finalState(), maximumBytes, charge, facade: null, custody: null, previous: owner.externalTail, next: null, backing: null, data: null, length: 0, scrubbed: 0, references: 0, receiving: false, receivedBacking: false, sealed: false, failed: false, closing: false, closed: false };
    try { return { step: step("ready", "resident-external-admission", 320), slot: createExternal(state) }; }
    catch { state.closing = true; return { step: step("rejected", "resident-external-construction", 320), slot: state.facade }; }
  }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-owner-close"); if (!state.closing) return step("rejected", "resident-owner-not-closing"); if (state.closed) return step("complete", "resident-owner-close");
    if (state.readers) { try { state.readers.facade!.beginClose(); return forward(state.readers.facade!.closeStep(grant), grant); } catch { return step("rejected", "resident-reader-close-fault"); } }
    if (state.pages) { try { state.pages.facade!.beginClose(); return forward(state.pages.facade!.closeStep(grant), grant); } catch { return step("rejected", "resident-page-close-fault"); } }
    if (state.external) { try { state.external.facade!.beginClose(); return forward(state.external.facade!.closeStep(grant), grant); } catch { return step("rejected", "resident-external-close-fault"); } }
    if (!admitted(grant, OWNER.bytes)) return step("blocked", "resident-owner-unlink"); if (!state.final.facade) return step("rejected", "resident-owner-witness"); const ledger = state.ledger!;
    if (state.previous) state.previous.next = state.next; else ledger.head = state.next; if (state.next) state.next.previous = state.previous; else ledger.tail = state.previous; if (ledger.cursor === state) ledger.cursor = state.next;
    state.final.terminal = true; refund(ledger, state.partition, OWNER); state.previous = null; state.next = null; state.facade = null; state.ledger = null; state.closed = true; return step("complete", "resident-owner-close", OWNER.bytes);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.ledger && !state.facade && !state.previous && !state.next && !state.pages && !state.pageTail && !state.readers && !state.readerTail && !state.external && !state.externalTail; }
  get retirement(): OwnedResidentRetirement | null { return this.#state.final.terminal ? this.#state.final.facade : null; }
}
//#endregion 🪪️StrongOwner

//#region 📄️IntrinsicPage
/** 📄️ Fixed, fully charged backing with no public array or caller-only capture operation. */
export class OwnedResidentPage {
  readonly #state: Page;
  private constructor(mint: object, state: Page) {
    if (mint !== MINT) throw new Error("Invalid resident page authority"); this.#state = state; state.facade = this;
    const owner = state.owner!; if (owner.pageTail) owner.pageTail.next = state; else owner.pages = state; owner.pageTail = state; createWitness(state.final, this); Object.freeze(this);
  }
  static { createPage = state => new OwnedResidentPage(MINT, state); pageState = value => value !== null && typeof value === "object" && #state in value ? value.#state : null; }
  allocate(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 256)) return step("blocked", "resident-page-allocate"); if (!state.owner || !live(state.owner) || state.closing || state.closed) return step("rejected", "resident-page-allocate"); if (state.data) return step("ready", "resident-page-allocate");
    try { state.data = new Uint8Array(256); return step("ready", "resident-page-allocate", 256); } catch { return step("rejected", "resident-page-allocation"); }
  }
  writeByte(value: number, grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-page-write");
    if (!state.owner || !live(state.owner) || state.closing || state.closed || !state.data || state.sealed || state.written >= state.length || !Number.isInteger(value) || value < 0 || value > 255) return step("rejected", "resident-page-write");
    state.data[state.written++] = value; return step("pending", "resident-page-write", 1);
  }
  seal(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 64)) return step("blocked", "resident-page-seal"); if (!state.owner || !live(state.owner) || state.closing || !state.data || state.written !== state.length) return step("rejected", "resident-page-seal"); state.sealed = true; return step("ready", "resident-page-seal", 64);
  }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-page-close"); if (!state.closing) return step("rejected", "resident-page-not-closing"); if (state.closed) return step("complete", "resident-page-close");
    if (state.references) return step("blocked", "resident-page-readers");
    if (state.data) { const bytes = Math.min(256 - state.scrubbed, grant.maxBytes); state.data.fill(0, state.scrubbed, state.scrubbed + bytes); state.scrubbed += bytes; if (state.scrubbed === 256) state.data = null; return step("pending", "resident-page-scrub", bytes); }
    if (!admitted(grant, 256)) return step("blocked", "resident-page-unlink"); if (!state.final.facade) return step("rejected", "resident-page-witness"); const owner = state.owner!;
    if (state.previous) state.previous.next = state.next; else owner.pages = state.next; if (state.next) state.next.previous = state.previous; else owner.pageTail = state.previous;
    state.final.terminal = true; refund(owner.ledger!, owner.partition, PAGE); state.previous = null; state.next = null; state.facade = null; state.owner = null; state.closed = true; return step("complete", "resident-page-close", 256);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.owner && !state.facade && !state.previous && !state.next && !state.data && state.references === 0; }
  get retirement(): OwnedResidentRetirement | null { return this.#state.final.terminal ? this.#state.final.facade : null; }
}
//#endregion 📄️IntrinsicPage

//#region 📖️RegisteredReader
/** 📖️ A consuming owner holds this exact reader before exposure; writer close preserves its bytes. */
export class OwnedResidentReader {
  readonly #state: Reader;
  private constructor(mint: object, state: Reader) {
    if (mint !== MINT) throw new Error("Invalid resident reader authority"); this.#state = state; state.facade = this;
    const owner = state.owner!; if (owner.readerTail) owner.readerTail.next = state; else owner.readers = state; owner.readerTail = state; createWitness(state.final, this); Object.freeze(this);
  }
  static { createReader = state => new OwnedResidentReader(MINT, state); }
  byteAt(index: number): number {
    const state = this.#state; if (state.closing || state.closed || !state.page?.data || !Number.isInteger(index) || index < 0 || index >= state.page.length) throw new Error("Invalid resident reader access"); return state.page.data[index]!;
  }
  get length(): number { const state = this.#state; if (!state.page || state.closed) throw new Error("Resident reader is closed"); return state.page.length; }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, READER.bytes)) return step("blocked", "resident-reader-close"); if (!state.closing) return step("rejected", "resident-reader-not-closing"); if (state.closed) return step("complete", "resident-reader-close");
    if (!state.final.facade) return step("rejected", "resident-reader-witness"); const owner = state.owner!; if (state.previous) state.previous.next = state.next; else owner.readers = state.next; if (state.next) state.next.previous = state.previous; else owner.readerTail = state.previous;
    state.final.terminal = true; state.page!.references--; refund(owner.ledger!, owner.partition, READER); state.page = null; state.owner = null; state.facade = null; state.previous = null; state.next = null; state.closed = true; return step("complete", "resident-reader-close", READER.bytes);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.owner && !state.page && !state.facade && !state.previous && !state.next; }
  get retirement(): OwnedResidentRetirement | null { return this.#state.final.terminal ? this.#state.final.facade : null; }
}
//#endregion 📖️RegisteredReader

//#region 📥️ExternalBacking
/** 📥️ A strongly registered receiving reservation fences cancellation before posting; custody requires actual detachment. */
export class OwnedResidentExternalBacking {
  readonly #state: External;
  private constructor(mint: object, state: External) {
    if (mint !== MINT) throw new Error("Invalid resident external authority"); this.#state = state; state.facade = this;
    const owner = state.owner!; if (owner.externalTail) owner.externalTail.next = state; else owner.external = state; owner.externalTail = state; createWitness(state.final, this); createCustody(state); Object.freeze(this);
  }
  static { createExternal = state => new OwnedResidentExternalBacking(MINT, state); externalState = value => value !== null && typeof value === "object" && #state in value ? value.#state : null; }
  beginReceive(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 64)) return step("blocked", "resident-external-fence");
    if (!state.owner || !live(state.owner) || state.receiving || state.closed || state.closing || !state.custody || !state.final.facade) return step("rejected", "resident-external-fence");
    state.receiving = true; return step("pending", "resident-external-fence", 64);
  }
  adoptTransferred(backing: unknown, grant: ResidentGrant): { readonly step: ResidentStep; readonly receipt: OwnedResidentBackingCustody | null } {
    const state = this.#state; if (!admitted(grant, 128)) return { step: step("blocked", "resident-external-adopt"), receipt: null };
    if (!state.receiving || state.closed || state.backing || state.sealed || state.failed || !checkedBuffer(backing)) return { step: step("rejected", "resident-external-adopt"), receipt: null };
    const length: number = Reflect.apply(bufferExtent, backing, []); if (length > state.maximumBytes) return { step: step("rejected", "resident-external-extent"), receipt: null };
    try {
      state.backing = transferBacking(backing, { transfer: [backing] }); state.receivedBacking = true; state.length = length;
      if (Reflect.apply(bufferExtent, backing, []) !== 0 || Reflect.apply(bufferExtent, state.backing, []) !== length) { state.failed = true; return { step: step("rejected", "resident-external-transfer"), receipt: null }; }
      state.data = new Uint8Array(state.backing); state.sealed = true;
      if (state.closing || !state.owner || !live(state.owner)) return { step: step("pending", "resident-external-retirement-custody", 128), receipt: null };
      return { step: step("ready", "resident-external-adopt", 128), receipt: state.custody };
    } catch { state.failed = true; return { step: step("rejected", "resident-external-transfer"), receipt: null }; }
  }
  get length(): number { const state = this.#state; if (!state.sealed || state.closed) throw new Error("External backing has no readable custody"); return state.length; }
  byteAt(index: number): number { const state = this.#state; if (state.closed || state.closing || !state.sealed || !state.data || !Number.isSafeInteger(index) || index < 0 || index >= state.length) throw new Error("Invalid external backing read"); return state.data[index]!; }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: ResidentGrant): ResidentStep {
    const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-external-close"); if (!state.closing) return step("rejected", "resident-external-not-closing"); if (state.closed) return step("complete", "resident-external-close");
    if (state.receiving && !state.receivedBacking) return step("blocked", "resident-external-awaiting-custody"); if (state.references) return step("blocked", "resident-external-readers");
    if (state.backing && !state.data) {
      if (!admitted(grant, 64)) return step("blocked", "resident-external-view"); try { state.data = new Uint8Array(state.backing); return step("pending", "resident-external-view", 64); } catch { return step("rejected", "resident-external-view"); }
    }
    if (state.data && state.scrubbed < state.length) { const bytes = Math.min(state.length - state.scrubbed, grant.maxBytes); state.data.fill(0, state.scrubbed, state.scrubbed + bytes); state.scrubbed += bytes; return step("pending", "resident-external-scrub", bytes); }
    if (state.backing || state.data) { if (!admitted(grant, 64)) return step("blocked", "resident-external-detach"); state.data = null; state.backing = null; return step("pending", "resident-external-detach", 64); }
    if (!admitted(grant, 320)) return step("blocked", "resident-external-unlink"); if (!state.final.facade || (state.receiving && !state.custody)) return step("rejected", "resident-external-witness"); const owner = state.owner!;
    if (state.previous) state.previous.next = state.next; else owner.external = state.next; if (state.next) state.next.previous = state.previous; else owner.externalTail = state.previous;
    state.final.terminal = true; refund(owner.ledger!, owner.partition, state.charge); state.previous = null; state.next = null; state.facade = null; state.owner = null; state.closed = true; return step("complete", "resident-external-close", 320);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.owner && !state.facade && !state.previous && !state.next && !state.backing && !state.data && state.references === 0; }
  get retirement(): OwnedResidentRetirement | null { return this.#state.final.terminal ? this.#state.final.facade : null; }
}
/** 🧾️ Exact intrinsic backing custody, not worker/request correlation or a native input acknowledgement. */
export class OwnedResidentBackingCustody {
  readonly #state: External;
  private constructor(mint: object, state: External) { if (mint !== MINT) throw new Error("Invalid resident custody authority"); this.#state = state; state.custody = this; Object.freeze(this); }
  static { createCustody = state => new OwnedResidentBackingCustody(MINT, state); }
  static matches(value: unknown, slot: unknown): value is OwnedResidentBackingCustody {
    if (value === null || typeof value !== "object" || !(#state in value)) return false; const state = value.#state;
    return state.final.root === slot && state.sealed && !state.closed && !state.closing && !state.failed && state.owner !== null && live(state.owner);
  }
}
//#endregion 📥️ExternalBacking

//#region 🧾️ExactRetirement
/** 🧾️ Preadmitted fixed proof of intrinsic retirement; retained empty facades are not a physical-GC certificate. */
export class OwnedResidentRetirement {
  readonly #state: Final;
  private constructor(mint: object, state: Final, root: object) { if (mint !== MINT) throw new Error("Invalid resident retirement authority"); this.#state = state; state.root = root; state.facade = this; Object.freeze(this); }
  static { createWitness = (state, root) => new OwnedResidentRetirement(MINT, state, root); }
  static matches(value: unknown, root: unknown): value is OwnedResidentRetirement { return value !== null && typeof value === "object" && #state in value && value.#state.terminal && value.#state.root === root; }
}
//#endregion 🧾️ExactRetirement
