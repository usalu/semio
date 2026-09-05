/* ../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts */
if (undefined) {}
/* ../../../../../../../../../🔨️modules/🧬️schema/🟦️.ts */
var GRAPHQL_STATE_PREAMBLE = `enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }
` + `directive @state(class: StateClass!) on FIELD_DEFINITION
` + "directive @derived on FIELD_DEFINITION";
var GRAPHQL_COMPOSITION_PREAMBLE = `type ArtifactLink { targetId: String! kind: String! }
` + `directive @child(kind: String!) on FIELD_DEFINITION
` + "directive @link(roles: [String!]) on FIELD_DEFINITION";

class ArtifactSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
}

class ArtifactInferenceRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
}

class AppSchemaRegistry {
  #byId = new Map;
  register(descriptor) {
    this.#byId.set(descriptor.id, descriptor);
  }
  get(id) {
    return this.#byId.get(id);
  }
  *iter() {
    yield* this.#byId.values();
  }
  get size() {
    return this.#byId.size;
  }
  get isEmpty() {
    return this.#byId.size === 0;
  }
}
/* ../../../../../../../../../🔨️modules/🖥️platform/🟦️.ts */
class Store {
  listeners = new Set;
  disposed = false;
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  notify() {
    if (this.disposed)
      return;
    for (const listener of this.listeners)
      listener();
  }
  dispose() {
    this.disposed = true;
    this.listeners.clear();
  }
}
/* ../../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts */
var ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = 44;
function decodeActorInstanceLifecycle(bytes) {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES)
    throw new Error("actor-lifecycle.envelope");
  const kind = bytes[0];
  if (kind === undefined || kind > 7)
    throw new Error("actor-lifecycle.tag");
  let offset = 1;
  const get = (maximum, nonzero) => {
    let value2 = 0n;
    for (let index = 0;index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined)
        throw new Error("actor-lifecycle.truncated");
      value2 |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if (index !== 0 && byte === 0 || value2 > maximum || nonzero && value2 === 0n)
          throw new Error("actor-lifecycle.noncanonical-authority");
        return value2;
      }
    }
    throw new Error("actor-lifecycle.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const guestLifetime = kind === 0 ? null : get(0xffffffffffffffffn, true);
  const requestSequence = Number(get(BigInt(Number.MAX_SAFE_INTEGER), true));
  let value;
  if (guestLifetime === null)
    value = { kind: "open", activationGeneration, instanceId, requestSequence };
  else {
    const lifetime = { activationGeneration, instanceId, guestLifetime };
    if (kind === 2)
      value = { kind: "close", lifetime, requestSequence };
    else {
      const receipt = kind === 1 || kind === 5 ? { kind: "captured", lifetime, requestSequence } : { kind: kind === 3 || kind === 6 ? "accepted" : "retired", lifetime, requestSequence, closeGeneration: get(0xffffffffffffffffn, true) };
      value = kind >= 5 ? { kind: "ack", receipt } : receipt;
    }
  }
  if (offset !== bytes.length)
    throw new Error("actor-lifecycle.trailing");
  return value;
}
function actorInstanceLifetimeEquals(left, right) {
  return left.activationGeneration === right.activationGeneration && left.instanceId === right.instanceId && left.guestLifetime === right.guestLifetime;
}
function actorInstanceLifecycleReceiptEquals(left, right) {
  return left.kind === right.kind && actorInstanceLifetimeEquals(left.lifetime, right.lifetime) && left.requestSequence === right.requestSequence && (left.kind === "captured" || right.kind !== "captured" && left.closeGeneration === right.closeGeneration);
}
function actorInstanceCapturedReceiptMatches(request, receipt) {
  return receipt.kind === "captured" && request.activationGeneration === receipt.lifetime.activationGeneration && request.instanceId === receipt.lifetime.instanceId && request.requestSequence === receipt.requestSequence;
}
function actorInstanceCloseReceiptMatches(request, accepted, receipt) {
  return receipt.kind !== "captured" && actorInstanceLifetimeEquals(request.lifetime, receipt.lifetime) && request.requestSequence === receipt.requestSequence && (accepted === null ? receipt.kind === "accepted" : accepted.kind === "accepted" && actorInstanceLifetimeEquals(accepted.lifetime, receipt.lifetime) && accepted.requestSequence === receipt.requestSequence && accepted.closeGeneration === receipt.closeGeneration);
}
if (undefined) {}

/* ../../../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts */
var ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = 35;
function encodeActorUiPatchReceipt(receipt) {
  const lifetime = receipt.lifetime;
  const valid = (value) => typeof value === "bigint" && value > 0n && value <= 0xffffffffffffffffn;
  if (!valid(lifetime.activationGeneration) || !valid(lifetime.guestLifetime) || !valid(receipt.patchSequence) || !Number.isInteger(lifetime.instanceId) || lifetime.instanceId < 0 || lifetime.instanceId > 4294967295)
    throw new Error("actor-ui-patch.invalid-authority");
  const output = new Uint8Array(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES);
  let length = 0;
  const put = (initial) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  put(lifetime.activationGeneration);
  put(BigInt(lifetime.instanceId));
  put(lifetime.guestLifetime);
  put(receipt.patchSequence);
  return output.slice(0, length);
}
function decodeActorUiPatchReceipt(bytes) {
  if (!(bytes instanceof Uint8Array) || bytes.length < 4 || bytes.length > ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES)
    throw new Error("actor-ui-patch.envelope");
  let offset = 0;
  const get = (maximum, nonzero) => {
    let value = 0n;
    for (let index = 0;index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined)
        throw new Error("actor-ui-patch.truncated");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if (index !== 0 && byte === 0 || value > maximum || nonzero && value === 0n)
          throw new Error("actor-ui-patch.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-ui-patch.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const guestLifetime = get(0xffffffffffffffffn, true);
  const patchSequence = get(0xffffffffffffffffn, true);
  if (offset !== bytes.length)
    throw new Error("actor-ui-patch.trailing");
  return { lifetime: { activationGeneration, instanceId, guestLifetime }, patchSequence };
}
function actorUiPatchReceiptEquals(left, right) {
  return left.lifetime.activationGeneration === right.lifetime.activationGeneration && left.lifetime.instanceId === right.lifetime.instanceId && left.lifetime.guestLifetime === right.lifetime.guestLifetime && left.patchSequence === right.patchSequence;
}
function validateActorUiPatchPairing(patchCount, receipt) {
  if (patchCount !== 0 && patchCount !== 1 || patchCount === 1 !== (receipt != null))
    throw new Error("actor-ui-patch.pairing");
  if (receipt != null)
    encodeActorUiPatchReceipt(receipt);
}
if (undefined) {}

/* ../../../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts */
var MINT = Object.freeze({});
var OWNER = Object.freeze({ bytes: 200, slots: 2, owners: 2 });
var PAGE = Object.freeze({ bytes: 520, slots: 3, owners: 2 });
var READER = Object.freeze({ bytes: 136, slots: 2, owners: 2 });
var ADMISSION = Object.freeze({ bytes: 296, slots: 6, owners: 6 });
var ADMISSION_LINK = Object.freeze({ bytes: 8, slots: 0, owners: 0 });
var admitted = (grant, bytes) => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
var step = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var createOwner;
var createPage;
var createReader;
var pageState;
var createWitness;
var createExternal;
var externalState;
var createCustody;
var createRecord;
var createDetachment;
var createAdmission;
var admissionState;
var advanceAdmission;
var createAdmissionResult;
var installAdmissionResult;
var finishAdmissionResult;
var admissionResultRoot;
var detachAdmissionResult;
var exactOwner;
var exactRecord;
var exactPage;
var exactReader;
var exactExternal;
var advanceOwner;
var advanceRecord;
var advancePage;
var advanceReader;
var advanceExternal;
var immutable = Object.freeze.bind(Object);
var transferBacking = globalThis.structuredClone.bind(globalThis);
var bufferExtent = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength").get;
var bufferResizable = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "resizable")?.get;
function checkedBuffer(value) {
  try {
    const extent = Reflect.apply(bufferExtent, value, []);
    return typeof extent === "number" && Number.isSafeInteger(extent) && extent >= 0 && (!bufferResizable || Reflect.apply(bufferResizable, value, []) === false);
  } catch {
    return false;
  }
}
var finalState = () => ({ root: null, facade: null, terminal: false });
function reserve(ledger, partition, charge) {
  const used = ledger.used[partition];
  const maximum = ledger.maximum[partition];
  if (charge.bytes > maximum.bytes - used.bytes || charge.slots > maximum.slots - used.slots || charge.owners > maximum.owners - used.owners)
    return false;
  used.bytes += charge.bytes;
  used.slots += charge.slots;
  used.owners += charge.owners;
  return true;
}
function refund(ledger, partition, charge) {
  const used = ledger.used[partition];
  used.bytes -= charge.bytes;
  used.slots -= charge.slots;
  used.owners -= charge.owners;
}
function refundResource(ledger, partition, charge) {
  const used = ledger.used[partition];
  used.bytes -= charge.bytes - ADMISSION_LINK.bytes;
  used.slots -= charge.slots;
  used.owners -= charge.owners;
}
function forward(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function resourceLive(resource) {
  return resource.admission !== null && !resource.admission.hasFailure && resource.admission.phase === "claimed";
}
function live(owner) {
  return owner.ledger !== null && !owner.closing && !owner.closed && !owner.ledger.closing && resourceLive(owner);
}
function resourceAdmission(value, ledger, partition) {
  const state = admissionState(value);
  return state?.ledger === ledger && state.partition === partition && state.claimed && state.phase === "claimed" && !state.hasFailure && state.result !== null && admissionResultRoot(state.result) === null && state.result.kind === null ? state : null;
}
function retainFault(state, error) {
  if (!state || state.hasFailure)
    throw error;
  state.failure = error;
  state.hasFailure = true;
  state.phase = "closing";
}
function retireResource(state, grant) {
  if ("pages" in state)
    return advanceOwner(state, grant);
  if ("shell" in state)
    return advanceRecord(state, grant);
  if ("page" in state)
    return advanceReader(state, grant);
  if ("maximumBytes" in state)
    return advanceExternal(state, grant);
  return advancePage(state, grant);
}

class OwnedResidentLedger {
  #state;
  constructor(capacity) {
    const bytes = capacity.bytes;
    const slots = capacity.slots;
    const owners = capacity.owners;
    const control = { bytes: capacity.control.bytes, slots: capacity.control.slots, owners: capacity.control.owners };
    if (![bytes, slots, owners, control.bytes, control.slots, control.owners].every((value) => Number.isSafeInteger(value) && value >= 0) || control.bytes > bytes || control.slots > slots || control.owners > owners)
      throw new Error("Invalid shared resident capacity");
    const exact = Object.freeze({ bytes, slots, owners, control: Object.freeze(control) });
    this.#state = { capacity: exact, maximum: { data: { bytes: bytes - control.bytes, slots: slots - control.slots, owners: owners - control.owners }, control: exact.control }, used: { data: { bytes: 0, slots: 0, owners: 0 }, control: { bytes: 0, slots: 0, owners: 0 } }, head: null, tail: null, cursor: null, records: null, recordTail: null, admissions: null, admissionTail: null, admissionCursor: null, pendingAdmission: null, closing: false, closed: false };
    Object.freeze(this);
  }
  get capacity() {
    return this.#state.capacity;
  }
  get usage() {
    const state = this.#state;
    return Object.freeze({ data: Object.freeze({ ...state.used.data }), control: Object.freeze({ ...state.used.control }) });
  }
  prepareAdmission(consumer, partition, grant) {
    const ledger = this.#state;
    if (!admitted(grant, ADMISSION.bytes))
      return step("blocked", "resident-admission-bootstrap");
    if (ledger.closing || consumer === null || typeof consumer !== "object" || partition !== "data" && partition !== "control")
      return step("rejected", "resident-admission-bootstrap");
    if (ledger.pendingAdmission) {
      const current = ledger.pendingAdmission;
      return step(current.consumer !== consumer ? "blocked" : current.hasFailure || current.phase !== "prepared" ? "rejected" : "ready", "resident-admission-bootstrap-held");
    }
    if (!reserve(ledger, partition, ADMISSION))
      return step("blocked", "resident-admission-capacity");
    const state = { ledger, partition, consumer, facade: null, previous: ledger.admissionTail, next: null, result: null, failure: undefined, hasFailure: false, phase: "prepared", claimed: false, resourceDetached: false, final: finalState() };
    try {
      createAdmission(state);
      return step("pending", "resident-admission-bootstrap", ADMISSION.bytes);
    } catch (error) {
      state.failure = error;
      state.hasFailure = true;
      state.phase = "closing";
      return step("rejected", "resident-admission-construction", ADMISSION.bytes);
    }
  }
  preparedAdmission(consumer) {
    const state = this.#state.pendingAdmission;
    return state?.consumer === consumer ? state.facade : null;
  }
  claimAdmission(consumer, value, grant) {
    if (!admitted(grant, 64))
      return step("blocked", "resident-admission-claim");
    const ledger = this.#state;
    const state = admissionState(value);
    if (ledger.closing || !state || state.ledger !== ledger || ledger.pendingAdmission !== state || state.consumer !== consumer || state.phase !== "prepared" || state.hasFailure || state.claimed || !state.result || !state.final.facade)
      return step("rejected", "resident-admission-claim");
    state.claimed = true;
    state.phase = "claimed";
    ledger.pendingAdmission = null;
    return step("ready", "resident-admission-claim", 64);
  }
  beginOwner(partition, cell, grant) {
    const ledger = this.#state;
    if (!admitted(grant, OWNER.bytes))
      return { step: step("blocked", "resident-owner-admission"), owner: null };
    if (ledger.closing || partition !== "data" && partition !== "control")
      return { step: step("rejected", "resident-owner-admission"), owner: null };
    const admission = resourceAdmission(cell, ledger, partition);
    if (!admission)
      return { step: step("rejected", "resident-owner-cell"), owner: null };
    if (!reserve(ledger, partition, OWNER))
      return { step: step("blocked", "resident-owner-capacity"), owner: null };
    const state = { admission, ledger, partition, final: finalState(), facade: null, previous: ledger.tail, next: null, pages: null, pageTail: null, readers: null, readerTail: null, external: null, externalTail: null, closing: false, closed: false };
    installAdmissionResult(admission.result, "owner", state, step("pending", "resident-owner-construction", OWNER.bytes));
    try {
      createOwner(state);
      const current = step("ready", "resident-owner-admission", OWNER.bytes);
      finishAdmissionResult(admission.result, current);
      return { step: current, owner: state.facade };
    } catch (error) {
      state.closing = true;
      retainFault(admission, error);
      const current = step("rejected", "resident-owner-construction", OWNER.bytes);
      finishAdmissionResult(admission.result, current);
      return { step: current, owner: state.facade };
    }
  }
  reserveRecord(partition, envelope, cell, grant) {
    const state = this.#state;
    if (!admitted(grant, 264))
      return { step: step("blocked", "resident-record-admission"), record: null };
    const bytes = envelope.bytes;
    const slots = envelope.slots;
    const owners = envelope.owners;
    if (state.closing || partition !== "data" && partition !== "control" || !Number.isSafeInteger(bytes) || bytes < 0 || bytes > Number.MAX_SAFE_INTEGER - 264 || !Number.isSafeInteger(slots) || slots < 0 || slots > Number.MAX_SAFE_INTEGER - 3 || !Number.isSafeInteger(owners) || owners < 0 || owners > Number.MAX_SAFE_INTEGER - 3)
      return { step: step("rejected", "resident-record-admission"), record: null };
    const admission = resourceAdmission(cell, state, partition);
    if (!admission)
      return { step: step("rejected", "resident-record-cell"), record: null };
    const charge = { bytes: bytes + 264, slots: slots + 3, owners: owners + 3 };
    if (!reserve(state, partition, charge))
      return { step: step("blocked", "resident-record-capacity"), record: null };
    const recordState = { admission, ledger: state, partition, charge, final: finalState(), facade: null, observation: null, previous: state.recordTail, next: null, shell: null, original: null, installed: false, detached: false, closing: false, closed: false };
    installAdmissionResult(admission.result, "record", recordState, step("pending", "resident-record-construction", 264));
    try {
      createRecord(recordState);
      const current = step("ready", "resident-record-admission", 264);
      finishAdmissionResult(admission.result, current);
      return { step: current, record: recordState.facade };
    } catch (error) {
      recordState.closing = true;
      retainFault(admission, error);
      const current = step("rejected", "resident-record-construction", 264);
      finishAdmissionResult(admission.result, current);
      return { step: current, record: recordState.facade };
    }
  }
  beginClose() {
    const state = this.#state;
    if (!state.closing) {
      state.closing = true;
      state.cursor = state.head;
    }
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-ledger-close");
    if (!state.closing)
      return step("rejected", "resident-ledger-not-closing");
    if (state.closed)
      return step("complete", "resident-ledger-close");
    const admission = state.admissionCursor ?? state.admissions;
    if (admission) {
      state.admissionCursor = admission.next ?? state.admissions;
      admission.phase = "closing";
      return forward(advanceAdmission(admission, grant), grant);
    }
    if (state.head || state.records)
      return step("rejected", "resident-ledger-resource-without-admission");
    if (!admitted(grant, 128))
      return step("blocked", "resident-ledger-unlink");
    const used = state.used;
    if (used.data.bytes || used.data.slots || used.data.owners || used.control.bytes || used.control.slots || used.control.owners)
      return step("rejected", "resident-ledger-invariant");
    state.cursor = null;
    state.closed = true;
    return step("complete", "resident-ledger-close", 128);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.head && !state.tail && !state.cursor && !state.records && !state.recordTail && !state.admissions && !state.admissionTail && !state.admissionCursor && !state.pendingAdmission && state.used.data.bytes === 0 && state.used.data.slots === 0 && state.used.data.owners === 0 && state.used.control.bytes === 0 && state.used.control.slots === 0 && state.used.control.owners === 0;
  }
}

class OwnedResidentAdmission {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident admission authority");
    this.#state = state;
    state.facade = this;
    const ledger = state.ledger;
    if (ledger.admissionTail)
      ledger.admissionTail.next = state;
    else
      ledger.admissions = state;
    ledger.admissionTail = state;
    ledger.pendingAdmission = state;
    createAdmissionResult(state);
    createWitness(state.final, this);
    Object.freeze(this);
  }
  static {
    createAdmission = (state) => new OwnedResidentAdmission(MINT, state);
    admissionState = (value) => value !== null && typeof value === "object" && (#state in value) ? value.#state : null;
    advanceAdmission = (state, grant) => state.facade.#close(grant);
  }
  get claimed() {
    return this.#state.claimed;
  }
  get hasFailure() {
    return this.#state.hasFailure;
  }
  get failure() {
    return this.#state.failure;
  }
  get result() {
    const result = this.#state.result;
    return result?.kind ? result : null;
  }
  retainFailure(error, grant) {
    if (!admitted(grant, 64))
      return step("blocked", "resident-admission-fault-handoff");
    const state = this.#state;
    if (state.phase === "closed")
      return step("rejected", "resident-admission-fault-handoff");
    if (state.hasFailure)
      return step(Object.is(state.failure, error) ? "ready" : "rejected", "resident-admission-fault-held");
    state.failure = error;
    state.hasFailure = true;
    state.phase = "closing";
    return step("pending", "resident-admission-fault-handoff", 64);
  }
  beginClose() {
    if (this.#state.phase !== "closed")
      this.#state.phase = "closing";
  }
  closeStep(grant) {
    return this.#close(grant);
  }
  #close(grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-admission-close");
    if (state.phase === "closed")
      return step("complete", "resident-admission-close");
    if (state.phase !== "closing")
      return step("rejected", "resident-admission-not-closing");
    const ledger = state.ledger;
    if (ledger.pendingAdmission === state) {
      if (!admitted(grant, 64))
        return step("blocked", "resident-admission-bootstrap-release");
      ledger.pendingAdmission = null;
      return step("pending", "resident-admission-bootstrap-release", 64);
    }
    const root = state.result && admissionResultRoot(state.result);
    if (root && !root.final.terminal) {
      if (!root.facade)
        return step("rejected", "resident-admission-unconstructed-resource");
      try {
        return forward(retireResource(root, grant), grant);
      } catch (error) {
        retainFault(state, error);
        return step("rejected", "resident-admission-resource-close");
      }
    }
    if (root) {
      if (!admitted(grant, 64))
        return step("blocked", "resident-admission-result-detach");
      root.admission = null;
      detachAdmissionResult(state.result);
      refund(ledger, state.partition, ADMISSION_LINK);
      state.resourceDetached = true;
      return step("pending", "resident-admission-result-detach", 64);
    }
    if (state.hasFailure)
      return step("rejected", "resident-admission-fault-held");
    if (!admitted(grant, ADMISSION.bytes))
      return step("blocked", "resident-admission-unlink");
    if (!state.final.facade)
      return step("rejected", "resident-admission-witness");
    if (state.previous)
      state.previous.next = state.next;
    else
      ledger.admissions = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      ledger.admissionTail = state.previous;
    if (ledger.admissionCursor === state)
      ledger.admissionCursor = state.next;
    state.final.terminal = true;
    refund(ledger, state.partition, ADMISSION);
    state.previous = null;
    state.next = null;
    state.facade = null;
    state.ledger = null;
    state.consumer = null;
    state.phase = "closed";
    return step("complete", "resident-admission-close", ADMISSION.bytes);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.phase === "closed" && !state.ledger && !state.consumer && !state.facade && !state.previous && !state.next && !state.hasFailure;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
}

class OwnedResidentAdmissionResult {
  #kind = null;
  #root = null;
  #step = step("pending", "resident-admission-unused");
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident result authority");
    state.result = this;
    Object.freeze(this);
  }
  static {
    createAdmissionResult = (state) => new OwnedResidentAdmissionResult(MINT, state);
    installAdmissionResult = (result, kind, root, current) => {
      result.#kind = kind;
      result.#root = root;
      result.#step = immutable(current);
    };
    finishAdmissionResult = (result, current) => {
      result.#step = immutable(current);
    };
    admissionResultRoot = (result) => result.#root;
    detachAdmissionResult = (result) => {
      result.#root = null;
    };
  }
  get kind() {
    return this.#kind;
  }
  get root() {
    return this.#root?.final.root ?? null;
  }
  get owner() {
    return this.#kind === "owner" ? exactOwner(this.#root?.final.root ?? null) : null;
  }
  get record() {
    return this.#kind === "record" ? exactRecord(this.#root?.final.root ?? null) : null;
  }
  get page() {
    return this.#kind === "page" ? exactPage(this.#root?.final.root ?? null) : null;
  }
  get reader() {
    return this.#kind === "reader" ? exactReader(this.#root?.final.root ?? null) : null;
  }
  get slot() {
    return this.#kind === "external" ? exactExternal(this.#root?.final.root ?? null) : null;
  }
  get step() {
    return this.#step;
  }
}

class OwnedResidentRecord {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident record authority");
    this.#state = state;
    state.facade = this;
    const ledger = state.ledger;
    if (ledger.recordTail)
      ledger.recordTail.next = state;
    else
      ledger.records = state;
    ledger.recordTail = state;
    createWitness(state.final, this);
    createDetachment(state);
    Object.freeze(this);
  }
  static {
    createRecord = (state) => new OwnedResidentRecord(MINT, state);
    exactRecord = (value) => value !== null && (#state in value) ? value : null;
    const close = this.prototype.closeStep;
    advanceRecord = (state, grant) => {
      state.closing = true;
      return Reflect.apply(close, state.facade, [grant]);
    };
  }
  install(shell, grant) {
    const state = this.#state;
    if (!admitted(grant, 64))
      return step("blocked", "resident-record-install");
    if (shell === null || typeof shell !== "object" || state.installed || state.closing || !state.ledger || state.ledger.closing || !state.final.facade || !state.observation || state.admission?.hasFailure || state.admission?.phase !== "claimed")
      return step("rejected", "resident-record-install");
    state.shell = shell;
    state.original = shell;
    state.installed = true;
    return step("ready", "resident-record-install", 64);
  }
  matchesShell(shell) {
    const state = this.#state;
    return state.installed && !state.detached && !state.closed && state.shell === shell;
  }
  matchesLiveShell(shell) {
    const state = this.#state;
    const admission = state.admission;
    const ledger = state.ledger;
    return state.installed && !state.detached && !state.closing && !state.closed && state.shell === shell && state.facade === this && !state.final.terminal && ledger !== null && !ledger.closing && !ledger.closed && admission !== null && admission.ledger === ledger && admission.phase === "claimed" && admission.claimed && !admission.hasFailure;
  }
  beginClose() {
    this.#state.closing = true;
  }
  detach(shell, grant) {
    const state = this.#state;
    if (!admitted(grant, 64))
      return step("blocked", "resident-record-detach");
    if (!state.closing || !state.installed || state.detached || state.closed || state.shell !== shell || !state.observation)
      return step("rejected", "resident-record-detach");
    state.shell = null;
    state.detached = true;
    return step("pending", "resident-record-detach", 64);
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, 264))
      return step("blocked", "resident-record-close");
    if (!state.closing)
      return step("rejected", "resident-record-not-closing");
    if (state.closed)
      return step("complete", "resident-record-close");
    if (state.shell)
      return step("blocked", "resident-record-installed");
    if (!state.final.facade || state.installed && (!state.detached || !state.observation))
      return step("rejected", "resident-record-witness");
    const ledger = state.ledger;
    if (state.previous)
      state.previous.next = state.next;
    else
      ledger.records = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      ledger.recordTail = state.previous;
    state.final.terminal = true;
    refundResource(ledger, state.partition, state.charge);
    state.previous = null;
    state.next = null;
    state.facade = null;
    state.ledger = null;
    state.closed = true;
    return step("complete", "resident-record-close", 264);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.admission && !state.ledger && !state.facade && !state.previous && !state.next && !state.shell;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
  get detachment() {
    return this.#state.detached ? this.#state.observation : null;
  }
}

class OwnedResidentRecordDetachment {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid record detachment authority");
    this.#state = state;
    state.observation = this;
    Object.freeze(this);
  }
  static {
    createDetachment = (state) => new OwnedResidentRecordDetachment(MINT, state);
  }
  static matches(value, record, shell) {
    return value !== null && typeof value === "object" && #state in value && value.#state.detached && value.#state.final.root === record && value.#state.original === shell;
  }
}

class OwnedResidentOwner {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident owner authority");
    this.#state = state;
    state.facade = this;
    const ledger = state.ledger;
    if (ledger.tail)
      ledger.tail.next = state;
    else
      ledger.head = state;
    ledger.tail = state;
    createWitness(state.final, this);
    Object.freeze(this);
  }
  static {
    createOwner = (state) => new OwnedResidentOwner(MINT, state);
    exactOwner = (value) => value !== null && (#state in value) ? value : null;
    const close = this.prototype.closeStep;
    advanceOwner = (state, grant) => {
      state.closing = true;
      return Reflect.apply(close, state.facade, [grant]);
    };
  }
  reservePage(length, cell, grant) {
    const owner = this.#state;
    if (!admitted(grant, 264))
      return { step: step("blocked", "resident-page-admission"), page: null };
    if (!Number.isInteger(length) || length < 0 || length > 256 || !live(owner))
      return { step: step("rejected", "resident-page-admission"), page: null };
    const admission = resourceAdmission(cell, owner.ledger, owner.partition);
    if (!admission)
      return { step: step("rejected", "resident-page-cell"), page: null };
    if (!reserve(owner.ledger, owner.partition, PAGE))
      return { step: step("blocked", "resident-page-capacity"), page: null };
    const state = { admission, owner, final: finalState(), length, facade: null, previous: owner.pageTail, next: null, data: null, written: 0, scrubbed: 0, references: 0, sealed: false, closing: false, closed: false };
    installAdmissionResult(admission.result, "page", state, step("pending", "resident-page-construction", 264));
    try {
      createPage(state);
      const current = step("ready", "resident-page-admission", 264);
      finishAdmissionResult(admission.result, current);
      return { step: current, page: state.facade };
    } catch (error) {
      state.closing = true;
      retainFault(admission, error);
      const current = step("rejected", "resident-page-construction", 264);
      finishAdmissionResult(admission.result, current);
      return { step: current, page: state.facade };
    }
  }
  beginRead(page, cell, grant) {
    const owner = this.#state;
    const source = pageState(page) ?? externalState(page);
    if (!admitted(grant, READER.bytes))
      return { step: step("blocked", "resident-reader-admission"), reader: null };
    if (!live(owner) || !source || !resourceLive(source) || source.closing || source.closed || !source.sealed || !source.owner || !live(source.owner) || source.owner.ledger !== owner.ledger || source.references >= Number.MAX_SAFE_INTEGER)
      return { step: step("rejected", "resident-reader-admission"), reader: null };
    const admission = resourceAdmission(cell, owner.ledger, owner.partition);
    if (!admission)
      return { step: step("rejected", "resident-reader-cell"), reader: null };
    if (!reserve(owner.ledger, owner.partition, READER))
      return { step: step("blocked", "resident-reader-capacity"), reader: null };
    source.references++;
    const state = { admission, owner, page: source, final: finalState(), facade: null, previous: owner.readerTail, next: null, closing: false, closed: false };
    installAdmissionResult(admission.result, "reader", state, step("pending", "resident-reader-construction", READER.bytes));
    try {
      createReader(state);
      const current = step("ready", "resident-reader-admission", READER.bytes);
      finishAdmissionResult(admission.result, current);
      return { step: current, reader: state.facade };
    } catch (error) {
      state.closing = true;
      retainFault(admission, error);
      const current = step("rejected", "resident-reader-construction", READER.bytes);
      finishAdmissionResult(admission.result, current);
      return { step: current, reader: state.facade };
    }
  }
  reserveExternalBacking(maximumBytes, cell, grant) {
    const owner = this.#state;
    if (!admitted(grant, 328))
      return { step: step("blocked", "resident-external-admission"), slot: null };
    if (!live(owner) || !Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || maximumBytes > Number.MAX_SAFE_INTEGER - 328)
      return { step: step("rejected", "resident-external-admission"), slot: null };
    const admission = resourceAdmission(cell, owner.ledger, owner.partition);
    if (!admission)
      return { step: step("rejected", "resident-external-cell"), slot: null };
    const charge = { bytes: maximumBytes + 328, slots: 4, owners: 3 };
    if (!reserve(owner.ledger, owner.partition, charge))
      return { step: step("blocked", "resident-external-capacity"), slot: null };
    const state = { admission, owner, final: finalState(), maximumBytes, charge, facade: null, custody: null, previous: owner.externalTail, next: null, backing: null, data: null, length: 0, scrubbed: 0, references: 0, receiving: false, receivedBacking: false, sealed: false, failed: false, closing: false, closed: false };
    installAdmissionResult(admission.result, "external", state, step("pending", "resident-external-construction", 328));
    try {
      createExternal(state);
      const current = step("ready", "resident-external-admission", 328);
      finishAdmissionResult(admission.result, current);
      return { step: current, slot: state.facade };
    } catch (error) {
      state.closing = true;
      retainFault(admission, error);
      const current = step("rejected", "resident-external-construction", 328);
      finishAdmissionResult(admission.result, current);
      return { step: current, slot: state.facade };
    }
  }
  beginClose() {
    this.#state.closing = true;
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-owner-close");
    if (!state.closing)
      return step("rejected", "resident-owner-not-closing");
    if (state.closed)
      return step("complete", "resident-owner-close");
    if (state.readers) {
      try {
        return forward(advanceReader(state.readers, grant), grant);
      } catch (error) {
        retainFault(state.admission, error);
        return step("rejected", "resident-reader-close-fault");
      }
    }
    if (state.pages) {
      try {
        return forward(advancePage(state.pages, grant), grant);
      } catch (error) {
        retainFault(state.admission, error);
        return step("rejected", "resident-page-close-fault");
      }
    }
    if (state.external) {
      try {
        return forward(advanceExternal(state.external, grant), grant);
      } catch (error) {
        retainFault(state.admission, error);
        return step("rejected", "resident-external-close-fault");
      }
    }
    if (!admitted(grant, OWNER.bytes))
      return step("blocked", "resident-owner-unlink");
    if (!state.final.facade)
      return step("rejected", "resident-owner-witness");
    const ledger = state.ledger;
    if (state.previous)
      state.previous.next = state.next;
    else
      ledger.head = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      ledger.tail = state.previous;
    if (ledger.cursor === state)
      ledger.cursor = state.next;
    state.final.terminal = true;
    refundResource(ledger, state.partition, OWNER);
    state.previous = null;
    state.next = null;
    state.facade = null;
    state.ledger = null;
    state.closed = true;
    return step("complete", "resident-owner-close", OWNER.bytes);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.admission && !state.ledger && !state.facade && !state.previous && !state.next && !state.pages && !state.pageTail && !state.readers && !state.readerTail && !state.external && !state.externalTail;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
}

class OwnedResidentPage {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident page authority");
    this.#state = state;
    state.facade = this;
    const owner = state.owner;
    if (owner.pageTail)
      owner.pageTail.next = state;
    else
      owner.pages = state;
    owner.pageTail = state;
    createWitness(state.final, this);
    Object.freeze(this);
  }
  static {
    createPage = (state) => new OwnedResidentPage(MINT, state);
    pageState = (value) => value !== null && typeof value === "object" && (#state in value) ? value.#state : null;
    exactPage = (value) => value !== null && (#state in value) ? value : null;
    const close = this.prototype.closeStep;
    advancePage = (state, grant) => {
      state.closing = true;
      return Reflect.apply(close, state.facade, [grant]);
    };
  }
  allocate(grant) {
    const state = this.#state;
    if (!admitted(grant, 256))
      return step("blocked", "resident-page-allocate");
    if (!resourceLive(state) || !state.owner || !live(state.owner) || state.closing || state.closed)
      return step("rejected", "resident-page-allocate");
    if (state.data)
      return step("ready", "resident-page-allocate");
    try {
      state.data = new Uint8Array(256);
      return step("ready", "resident-page-allocate", 256);
    } catch (error) {
      retainFault(state.admission, error);
      state.closing = true;
      return step("rejected", "resident-page-allocation");
    }
  }
  writeByte(value, grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-page-write");
    if (!resourceLive(state) || !state.owner || !live(state.owner) || state.closing || state.closed || !state.data || state.sealed || state.written >= state.length || !Number.isInteger(value) || value < 0 || value > 255)
      return step("rejected", "resident-page-write");
    state.data[state.written++] = value;
    return step("pending", "resident-page-write", 1);
  }
  seal(grant) {
    const state = this.#state;
    if (!admitted(grant, 64))
      return step("blocked", "resident-page-seal");
    if (!resourceLive(state) || !state.owner || !live(state.owner) || state.closing || !state.data || state.written !== state.length)
      return step("rejected", "resident-page-seal");
    state.sealed = true;
    return step("ready", "resident-page-seal", 64);
  }
  beginClose() {
    this.#state.closing = true;
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-page-close");
    if (!state.closing)
      return step("rejected", "resident-page-not-closing");
    if (state.closed)
      return step("complete", "resident-page-close");
    if (state.references)
      return step("blocked", "resident-page-readers");
    if (state.data) {
      const bytes = Math.min(256 - state.scrubbed, grant.maxBytes);
      state.data.fill(0, state.scrubbed, state.scrubbed + bytes);
      state.scrubbed += bytes;
      if (state.scrubbed === 256)
        state.data = null;
      return step("pending", "resident-page-scrub", bytes);
    }
    if (!admitted(grant, 264))
      return step("blocked", "resident-page-unlink");
    if (!state.final.facade)
      return step("rejected", "resident-page-witness");
    const owner = state.owner;
    if (state.previous)
      state.previous.next = state.next;
    else
      owner.pages = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      owner.pageTail = state.previous;
    state.final.terminal = true;
    refundResource(owner.ledger, owner.partition, PAGE);
    state.previous = null;
    state.next = null;
    state.facade = null;
    state.owner = null;
    state.closed = true;
    return step("complete", "resident-page-close", 264);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.admission && !state.owner && !state.facade && !state.previous && !state.next && !state.data && state.references === 0;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
}

class OwnedResidentReader {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident reader authority");
    this.#state = state;
    state.facade = this;
    const owner = state.owner;
    if (owner.readerTail)
      owner.readerTail.next = state;
    else
      owner.readers = state;
    owner.readerTail = state;
    createWitness(state.final, this);
    Object.freeze(this);
  }
  static {
    createReader = (state) => new OwnedResidentReader(MINT, state);
    exactReader = (value) => value !== null && (#state in value) ? value : null;
    const close = this.prototype.closeStep;
    advanceReader = (state, grant) => {
      state.closing = true;
      return Reflect.apply(close, state.facade, [grant]);
    };
  }
  byteAt(index) {
    const state = this.#state;
    if (!resourceLive(state) || state.closing || state.closed || !state.page?.data || !Number.isInteger(index) || index < 0 || index >= state.page.length)
      throw new Error("Invalid resident reader access");
    return state.page.data[index];
  }
  get length() {
    const state = this.#state;
    if (!resourceLive(state) || !state.page || state.closing || state.closed)
      throw new Error("Resident reader is closed");
    return state.page.length;
  }
  beginClose() {
    this.#state.closing = true;
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, READER.bytes))
      return step("blocked", "resident-reader-close");
    if (!state.closing)
      return step("rejected", "resident-reader-not-closing");
    if (state.closed)
      return step("complete", "resident-reader-close");
    if (!state.final.facade)
      return step("rejected", "resident-reader-witness");
    const owner = state.owner;
    if (state.previous)
      state.previous.next = state.next;
    else
      owner.readers = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      owner.readerTail = state.previous;
    state.final.terminal = true;
    state.page.references--;
    refundResource(owner.ledger, owner.partition, READER);
    state.page = null;
    state.owner = null;
    state.facade = null;
    state.previous = null;
    state.next = null;
    state.closed = true;
    return step("complete", "resident-reader-close", READER.bytes);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.admission && !state.owner && !state.page && !state.facade && !state.previous && !state.next;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
}

class OwnedResidentExternalBacking {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident external authority");
    this.#state = state;
    state.facade = this;
    const owner = state.owner;
    if (owner.externalTail)
      owner.externalTail.next = state;
    else
      owner.external = state;
    owner.externalTail = state;
    createWitness(state.final, this);
    createCustody(state);
    Object.freeze(this);
  }
  static {
    createExternal = (state) => new OwnedResidentExternalBacking(MINT, state);
    externalState = (value) => value !== null && typeof value === "object" && (#state in value) ? value.#state : null;
    exactExternal = (value) => value !== null && (#state in value) ? value : null;
    const close = this.prototype.closeStep;
    advanceExternal = (state, grant) => {
      state.closing = true;
      return Reflect.apply(close, state.facade, [grant]);
    };
  }
  beginReceive(grant) {
    const state = this.#state;
    if (!admitted(grant, 64))
      return step("blocked", "resident-external-fence");
    if (!resourceLive(state) || !state.owner || !live(state.owner) || state.receiving || state.closed || state.closing || !state.custody || !state.final.facade)
      return step("rejected", "resident-external-fence");
    state.receiving = true;
    return step("pending", "resident-external-fence", 64);
  }
  adoptTransferred(backing, grant) {
    const state = this.#state;
    if (!admitted(grant, 128))
      return { step: step("blocked", "resident-external-adopt"), receipt: null };
    if (!state.receiving || state.closed || state.backing || state.sealed || state.failed || !checkedBuffer(backing))
      return { step: step("rejected", "resident-external-adopt"), receipt: null };
    const length = Reflect.apply(bufferExtent, backing, []);
    if (length > state.maximumBytes)
      return { step: step("rejected", "resident-external-extent"), receipt: null };
    try {
      state.backing = transferBacking(backing, { transfer: [backing] });
      state.receivedBacking = true;
      state.length = length;
      if (Reflect.apply(bufferExtent, backing, []) !== 0 || Reflect.apply(bufferExtent, state.backing, []) !== length) {
        state.failed = true;
        return { step: step("rejected", "resident-external-transfer"), receipt: null };
      }
      state.data = new Uint8Array(state.backing);
      state.sealed = true;
      if (!resourceLive(state) || state.closing || !state.owner || !live(state.owner))
        return { step: step("pending", "resident-external-retirement-custody", 128), receipt: null };
      return { step: step("ready", "resident-external-adopt", 128), receipt: state.custody };
    } catch (error) {
      state.failed = true;
      retainFault(state.admission, error);
      return { step: step("rejected", "resident-external-transfer"), receipt: null };
    }
  }
  get length() {
    const state = this.#state;
    if (!resourceLive(state) || !state.sealed || state.closing || state.closed)
      throw new Error("External backing has no readable custody");
    return state.length;
  }
  byteAt(index) {
    const state = this.#state;
    if (!resourceLive(state) || state.closed || state.closing || !state.sealed || !state.data || !Number.isSafeInteger(index) || index < 0 || index >= state.length)
      throw new Error("Invalid external backing read");
    return state.data[index];
  }
  beginClose() {
    this.#state.closing = true;
  }
  closeStep(grant) {
    const state = this.#state;
    if (!admitted(grant, 1))
      return step("blocked", "resident-external-close");
    if (!state.closing)
      return step("rejected", "resident-external-not-closing");
    if (state.closed)
      return step("complete", "resident-external-close");
    if (state.receiving && !state.receivedBacking)
      return step("blocked", "resident-external-awaiting-custody");
    if (state.references)
      return step("blocked", "resident-external-readers");
    if (state.backing && !state.data) {
      if (!admitted(grant, 64))
        return step("blocked", "resident-external-view");
      try {
        state.data = new Uint8Array(state.backing);
        return step("pending", "resident-external-view", 64);
      } catch (error) {
        retainFault(state.admission, error);
        return step("rejected", "resident-external-view");
      }
    }
    if (state.data && state.scrubbed < state.length) {
      const bytes = Math.min(state.length - state.scrubbed, grant.maxBytes);
      state.data.fill(0, state.scrubbed, state.scrubbed + bytes);
      state.scrubbed += bytes;
      return step("pending", "resident-external-scrub", bytes);
    }
    if (state.backing || state.data) {
      if (!admitted(grant, 64))
        return step("blocked", "resident-external-detach");
      state.data = null;
      state.backing = null;
      return step("pending", "resident-external-detach", 64);
    }
    if (!admitted(grant, 328))
      return step("blocked", "resident-external-unlink");
    if (!state.final.facade || state.receiving && !state.custody)
      return step("rejected", "resident-external-witness");
    const owner = state.owner;
    if (state.previous)
      state.previous.next = state.next;
    else
      owner.external = state.next;
    if (state.next)
      state.next.previous = state.previous;
    else
      owner.externalTail = state.previous;
    state.final.terminal = true;
    refundResource(owner.ledger, owner.partition, state.charge);
    state.previous = null;
    state.next = null;
    state.facade = null;
    state.owner = null;
    state.closed = true;
    return step("complete", "resident-external-close", 328);
  }
  terminalIsEmpty() {
    const state = this.#state;
    return state.closed && !state.admission && !state.owner && !state.facade && !state.previous && !state.next && !state.backing && !state.data && state.references === 0;
  }
  get retirement() {
    return this.#state.final.terminal ? this.#state.final.facade : null;
  }
}

class OwnedResidentBackingCustody {
  #state;
  constructor(mint, state) {
    if (mint !== MINT)
      throw new Error("Invalid resident custody authority");
    this.#state = state;
    state.custody = this;
    Object.freeze(this);
  }
  static {
    createCustody = (state) => new OwnedResidentBackingCustody(MINT, state);
  }
  static matches(value, slot) {
    if (value === null || typeof value !== "object" || !(#state in value))
      return false;
    const state = value.#state;
    return state.final.root === slot && resourceLive(state) && state.sealed && !state.closed && !state.closing && !state.failed && state.owner !== null && live(state.owner);
  }
}

class OwnedResidentRetirement {
  #state;
  constructor(mint, state, root) {
    if (mint !== MINT)
      throw new Error("Invalid resident retirement authority");
    this.#state = state;
    state.root = root;
    state.facade = this;
    Object.freeze(this);
  }
  static {
    createWitness = (state, root) => new OwnedResidentRetirement(MINT, state, root);
  }
  static matches(value, root) {
    return value !== null && typeof value === "object" && #state in value && value.#state.terminal && value.#state.root === root;
  }
}

/* ../../../../../../../../../🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts */
var MINT2 = Object.freeze({});
var NO_OUTPUT_FAULT = Symbol("actor-output.no-fault");
var MAX_SEQUENCE = 0xffffffffffffffffn;
var OUTPUT_ENVELOPE = Object.freeze({ bytes: 448, slots: 3, owners: 3 });
var createOutput;
var cancelEmpty;
var canRun;
function granted(grant, bytes) {
  return Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
}
function admission(kind, phase, bytes = 0, output = null) {
  return { step: { kind, phase, items: bytes ? 1 : 0, bytes }, output };
}
function retainOutputFault(slot, error) {
  if (slot.fault === NO_OUTPUT_FAULT)
    slot.fault = error;
  else if (!Object.is(slot.fault, error))
    throw error;
}
function settleOutput(slot, kind, value) {
  slot.outcome = { kind, value };
  slot.phase = "returned";
  try {
    Object.freeze(slot.outcome);
  } catch (error) {
    retainOutputFault(slot, error);
    throw error;
  }
}

class OwnedActorTurnOutput {
  #slot;
  constructor(mint, slot) {
    if (mint !== MINT2)
      throw new Error("actor-output.private-mint");
    this.#slot = slot;
    slot.handle = this;
    Object.freeze(this);
  }
  static {
    createOutput = (slot) => new OwnedActorTurnOutput(MINT2, slot);
  }
  static matches(output, owner) {
    return output !== null && typeof output === "object" && #slot in output && output.#slot.owner === owner;
  }
  static reserved(output, owner) {
    return OwnedActorTurnOutput.matches(output, owner) && output.#slot.phase === "reserved" && output.#slot.fault === NO_OUTPUT_FAULT && canRun(output.#slot);
  }
  static matchesFault(output, fault) {
    return output !== null && typeof output === "object" && #slot in output && output.#slot.fault !== NO_OUTPUT_FAULT && Object.is(output.#slot.fault, fault);
  }
  get state() {
    return Object.freeze({ capacity: this.#slot.capacity, sequence: this.#slot.sequence.toString(), phase: this.#slot.phase, retained: this.#slot.outcome !== null || this.#slot.response !== null || this.#slot.fault !== NO_OUTPUT_FAULT });
  }
  get outcome() {
    return this.#slot.outcome;
  }
  get responseEnvelope() {
    return this.#slot.response;
  }
  captureResponse(response) {
    const slot = this.#slot;
    if (slot.phase !== "pending" || slot.response !== null || response === null || typeof response !== "object")
      return false;
    slot.response = response;
    slot.phase = "returned";
    return true;
  }
  async run(submit) {
    const slot = this.#slot;
    if (slot.fault !== NO_OUTPUT_FAULT)
      throw new Error("actor-output.faulted");
    if (slot.phase !== "reserved")
      throw new Error("actor-output.already-submitted");
    if (!canRun(slot))
      throw new Error("actor-output.closed");
    slot.phase = "pending";
    let value;
    try {
      value = await submit();
    } catch (error) {
      settleOutput(slot, "refused", error);
      throw error;
    }
    settleOutput(slot, "returned", value);
    return value;
  }
  cancelEmpty() {
    return cancelEmpty(this.#slot);
  }
}

class OwnedActorTurnOutputs {
  #owner;
  #capacity;
  #sequence;
  #head = null;
  #tail = null;
  #pending = 0;
  #closed = false;
  #ledger;
  #admissionCell = null;
  #admissionRecord = null;
  #admissionPhase = "idle";
  #admissionFault = NO_OUTPUT_FAULT;
  constructor(owner, capacity, ledger, sequence = 0n) {
    if (!owner || typeof owner !== "object" || !Number.isSafeInteger(capacity) || capacity < 1 || capacity > 4294967295 || !(ledger instanceof OwnedResidentLedger) || typeof sequence !== "bigint" || sequence < 0n || sequence > MAX_SEQUENCE)
      throw new Error("actor-output.invalid-admission");
    this.#owner = owner;
    this.#capacity = capacity;
    this.#sequence = sequence;
    this.#ledger = ledger;
  }
  static {
    canRun = (slot) => slot.queue !== null && !slot.queue.#closed && slot.queue.#admissionFault === NO_OUTPUT_FAULT && (slot.queue.#admissionCell !== slot.cell || slot.queue.#admissionPhase === "published") && slot.record.matchesLiveShell(slot.queue);
    cancelEmpty = (slot) => {
      const queue = slot.queue;
      if (!queue || slot.phase !== "reserved" || slot.outcome !== null || slot.fault !== NO_OUTPUT_FAULT)
        return false;
      slot.phase = "cancelled";
      return true;
    };
  }
  get pending() {
    return this.#pending;
  }
  peek() {
    return this.#head?.handle ?? null;
  }
  reserve(grant) {
    if (!granted(grant, 64))
      return admission("blocked", "actor-output.grant");
    try {
      const ledger = this.#ledger;
      if (this.#admissionPhase === "preparing") {
        const cell2 = ledger.preparedAdmission(this);
        if (!cell2)
          return admission("rejected", "actor-output.missing-cell");
        this.#admissionCell = cell2;
        this.#admissionPhase = "cell-held";
        return admission("pending", "actor-output.cell-held", 64);
      }
      const cell = this.#admissionCell;
      if (this.#admissionPhase === "claiming" && cell?.claimed) {
        this.#admissionPhase = "claimed";
        return admission("pending", "actor-output.claimed", 64);
      }
      if (this.#admissionPhase === "record-admitting") {
        const record2 = cell?.result?.record;
        if (!record2)
          return admission("rejected", "actor-output.missing-record");
        this.#admissionRecord = record2;
        this.#admissionPhase = "record-held";
        return admission("pending", "actor-output.record-held", 64);
      }
      const record = this.#admissionRecord;
      if (this.#admissionPhase === "installing" && record?.matchesShell(this)) {
        this.#admissionPhase = "installed";
        return admission("pending", "actor-output.installed", 64);
      }
      if (this.#admissionFault !== NO_OUTPUT_FAULT) {
        if (cell && !cell.hasFailure) {
          const current = cell.retainFailure(this.#admissionFault, grant);
          return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null };
        }
        return admission("rejected", "actor-output.fault-held");
      }
      if (cell?.hasFailure || cell?.result?.step.kind === "rejected")
        return admission("rejected", "actor-output.admission-fault");
      if (this.#closed)
        return admission("rejected", "actor-output.closed");
      if (["installed", "slot-held", "facade-held", "published"].includes(this.#admissionPhase) && !record?.matchesLiveShell(this))
        return admission("rejected", "actor-output.parent-not-live");
      if (this.#admissionPhase === "published") {
        const slot = this.#tail;
        if (slot?.phase === "reserved")
          return admission("ready", "actor-output.published", 0, slot.handle);
        this.#admissionCell = null;
        this.#admissionRecord = null;
        this.#admissionPhase = "idle";
        return admission("pending", "actor-output.next-admission", 64);
      }
      if (this.#admissionPhase === "idle") {
        if (this.#pending >= this.#capacity || this.#sequence === MAX_SEQUENCE)
          return admission("blocked", "actor-output.capacity");
        if (!granted(grant, 296))
          return admission("blocked", "actor-output.bootstrap");
        this.#admissionPhase = "preparing";
        const current = ledger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected" && current.bytes === 0)
          this.#admissionPhase = "idle";
        return { step: current, output: null };
      }
      if (this.#admissionPhase === "cell-held" && cell) {
        this.#admissionPhase = "claiming";
        const current = ledger.claimAdmission(this, cell, grant);
        if (current.kind !== "ready")
          this.#admissionPhase = "cell-held";
        return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null };
      }
      if (this.#admissionPhase === "claimed" && cell) {
        if (!granted(grant, 264))
          return admission("blocked", "actor-output.record");
        this.#admissionPhase = "record-admitting";
        const result = ledger.reserveRecord("data", OUTPUT_ENVELOPE, cell, grant);
        if (result.step.kind === "blocked" || result.step.kind === "rejected" && result.step.bytes === 0)
          this.#admissionPhase = "claimed";
        return { step: { ...result.step, kind: result.step.kind === "ready" ? "pending" : result.step.kind }, output: null };
      }
      if (this.#admissionPhase === "record-held" && record) {
        this.#admissionPhase = "installing";
        const current = record.install(this, grant);
        if (current.kind !== "ready")
          this.#admissionPhase = "record-held";
        return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null };
      }
      if (this.#admissionPhase === "installed" && cell && record) {
        if (!granted(grant, 272))
          return admission("blocked", "actor-output.slot");
        const slot = { owner: this.#owner, capacity: this.#capacity, sequence: this.#sequence + 1n, queue: this, handle: null, phase: "reserved", response: null, outcome: null, fault: NO_OUTPUT_FAULT, previous: this.#tail, next: null, cell, record };
        if (this.#tail)
          this.#tail.next = slot;
        else
          this.#head = slot;
        this.#tail = slot;
        this.#sequence = slot.sequence;
        this.#pending++;
        this.#admissionPhase = "slot-held";
        return admission("pending", "actor-output.slot-held", 272);
      }
      if (this.#admissionPhase === "slot-held" && this.#tail) {
        if (!granted(grant, 80))
          return admission("blocked", "actor-output.facade");
        createOutput(this.#tail);
        this.#admissionPhase = "facade-held";
        return admission("pending", "actor-output.facade-held", 80);
      }
      if (this.#admissionPhase === "facade-held" && this.#tail?.handle) {
        this.#admissionPhase = "published";
        return admission("ready", "actor-output.published", 64, this.#tail.handle);
      }
      return admission("rejected", "actor-output.admission-phase");
    } catch (error) {
      if (this.#admissionFault !== NO_OUTPUT_FAULT && !Object.is(this.#admissionFault, error))
        throw error;
      this.#admissionFault = error;
      if (this.#tail?.cell === this.#admissionCell)
        retainOutputFault(this.#tail, error);
      throw error;
    }
  }
  beginClose() {
    this.#closed = true;
  }
}
if (undefined) {
  async function fixtureOutput(queue) {}
}

/* ../../../../../../../../../🔨️modules/🎭️actor/📃️page/🟦️.ts */
var ACTOR_BYTE_PAGE_BYTES = 4096;
function createActorBytePage(bytes) {
  if (!(bytes instanceof Uint8Array) || bytes.length > ACTOR_BYTE_PAGE_BYTES)
    throw new Error("actor-byte-page.input");
  const page = { length: bytes.length };
  for (let blockIndex = 0;blockIndex < 64; blockIndex++) {
    const block = {};
    for (let wordIndex = 0;wordIndex < 8; wordIndex++) {
      let word = 0n;
      const start = blockIndex * 64 + wordIndex * 8;
      for (let byteIndex = 0;byteIndex < 8; byteIndex++)
        word |= BigInt(bytes[start + byteIndex] ?? 0) << BigInt(byteIndex * 8);
      block[`word${wordIndex}`] = word;
    }
    page[`block${blockIndex.toString().padStart(2, "0")}`] = Object.freeze(block);
  }
  return Object.freeze(page);
}
if (undefined) {}

/* ../../../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts */
var ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES = 41;
var ACTOR_RETURN_DRIVE_MAXIMUM_BYTES = 43;
var ACTOR_RETURN_RESULT_MAXIMUM_BYTES = 1 + ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES + ACTOR_BYTE_PAGE_BYTES;
var U64_MAXIMUM = 0xffffffffffffffffn;
var RETURN_REASONS = ["working", "blocked", "awaitingInput", "closing"];
var RETURN_COMPLETIONS = ["complete", "cancelled", "faulted"];
var RETURN_OUTCOMES = ["accepted", "duplicate", "blocked", "refused"];
var RETURN_FAULTS = ["none", "capacity", "sequenceExhausted", "staleOrigin", "staleIdentity", "wrongPage", "inputNotRetired", "notRetired", "clockUnavailable", "clockBackward", "deadline", "ownerFault", "malformedControl", "mixedControl"];

class ReturnWriter {
  bytes;
  length = 0;
  constructor(maximum) {
    this.bytes = new Uint8Array(maximum);
  }
  byte(value) {
    if (this.length === this.bytes.length)
      throw new Error("actor-return.envelope");
    this.bytes[this.length++] = value;
  }
  uint(value, maximum = U64_MAXIMUM, positive = true) {
    if (typeof value !== "bigint" || value < (positive ? 1n : 0n) || value > maximum)
      throw new Error("actor-return.authority");
    do {
      const byte = Number(value & 127n);
      value >>= 7n;
      this.byte(byte | (value === 0n ? 0 : 128));
    } while (value !== 0n);
  }
  finish() {
    return this.bytes.subarray(0, this.length);
  }
}
function writeOrigin(writer, origin) {
  if (!Number.isSafeInteger(origin.requestSequence) || origin.requestSequence < 1)
    throw new Error("actor-return.request-sequence");
  writer.uint(origin.activationGeneration);
  writer.uint(BigInt(origin.requestSequence), BigInt(Number.MAX_SAFE_INTEGER));
}
function writeIdentity(writer, identity) {
  writeOrigin(writer, identity.origin);
  writer.uint(identity.returnSequence);
}
function writePageReceipt(writer, receipt) {
  if (!Number.isInteger(receipt.length) || receipt.length < 0 || receipt.length > ACTOR_BYTE_PAGE_BYTES || typeof receipt.final !== "boolean" || !receipt.final && receipt.length === 0)
    throw new Error("actor-return.page-receipt");
  writeIdentity(writer, receipt.identity);
  writer.uint(receipt.pageSequence);
  writer.uint(BigInt(receipt.length), BigInt(ACTOR_BYTE_PAGE_BYTES), false);
  writer.byte(receipt.final ? 1 : 0);
}
function writeControl(writer, control) {
  switch (control.kind) {
    case "poll":
      writer.byte(0);
      writeIdentity(writer, control.identity);
      return;
    case "inputAck":
      writer.byte(1);
      writePageReceipt(writer, control.receipt);
      return;
    case "cancel":
      writer.byte(2);
      writeIdentity(writer, control.identity);
      return;
    case "retiredAck":
      writer.byte(3);
      writeIdentity(writer, control.identity);
      return;
    default:
      throw new Error("actor-return.control-tag");
  }
}
function encodeActorReturnDrive(drive) {
  const writer = new ReturnWriter(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES);
  switch (drive.kind) {
    case "execute":
      writer.byte(0);
      writeOrigin(writer, drive.origin);
      break;
    case "control":
      writer.byte(1);
      writeControl(writer, drive.control);
      break;
    default:
      throw new Error("actor-return.drive-tag");
  }
  return writer.finish();
}
class ActorReturnResultFraming {
  #stage = "tag";
  #tag = -1;
  #controlTag = -1;
  #offset = 0;
  #payloadOffset = 0;
  #payloadRead = 0;
  #accumulator = 0n;
  #digits = 0;
  #activation = 0n;
  #request = 0;
  #returnSequence = 0n;
  #pageSequence = 0n;
  #length = 0;
  #final = false;
  #reason = "working";
  #completion = "complete";
  #outcome = "accepted";
  #faultValue = "none";
  #failed = false;
  #value = null;
  get value() {
    return this.#value;
  }
  #fail() {
    this.#failed = true;
    this.#value = null;
    throw new Error("actor-return.result-framing");
  }
  #enum(byte, values) {
    const value = values[byte];
    if (value === undefined)
      return this.#fail();
    return value;
  }
  push(byte) {
    if (this.#failed || this.#value !== null || this.#stage === "done" || !Number.isInteger(byte) || byte < 0 || byte > 255 || this.#offset === ACTOR_RETURN_RESULT_MAXIMUM_BYTES)
      this.#fail();
    this.#offset++;
    switch (this.#stage) {
      case "tag":
        if (byte > 5)
          this.#fail();
        this.#tag = byte;
        this.#stage = byte === 4 ? "control" : byte === 5 ? "fault" : "activation";
        return;
      case "control":
        if (byte > 3)
          this.#fail();
        this.#controlTag = byte;
        this.#stage = "activation";
        return;
      case "final":
        if (byte > 1 || byte === 0 && this.#length === 0)
          this.#fail();
        this.#final = byte === 1;
        this.#payloadOffset = this.#offset;
        this.#stage = this.#tag === 2 ? "padding" : "outcome";
        return;
      case "padding":
        if (this.#payloadRead >= this.#length && byte !== 0)
          this.#fail();
        this.#payloadRead++;
        if (this.#payloadRead === ACTOR_BYTE_PAGE_BYTES)
          this.#stage = "done";
        return;
      case "reason":
        this.#reason = this.#enum(byte, RETURN_REASONS);
        this.#stage = "done";
        return;
      case "completion":
        this.#completion = this.#enum(byte, RETURN_COMPLETIONS);
        this.#stage = "done";
        return;
      case "outcome":
        this.#outcome = this.#enum(byte, RETURN_OUTCOMES);
        this.#stage = "fault";
        return;
      case "fault":
        this.#faultValue = this.#enum(byte, RETURN_FAULTS);
        this.#stage = "done";
        return;
      default:
        this.#uint(byte);
        return;
    }
  }
  #uint(byte) {
    const stage = this.#stage;
    const limit = stage === "request" ? 8 : 10;
    if (this.#digits >= limit || this.#digits === 9 && byte > 1)
      this.#fail();
    this.#accumulator |= BigInt(byte & 127) << BigInt(this.#digits * 7);
    this.#digits++;
    if (byte & 128) {
      if (this.#digits === limit)
        this.#fail();
      return;
    }
    const value = this.#accumulator;
    const maximum = stage === "request" ? BigInt(Number.MAX_SAFE_INTEGER) : stage === "length" ? BigInt(ACTOR_BYTE_PAGE_BYTES) : U64_MAXIMUM;
    if (this.#digits > 1 && byte === 0 || value > maximum || stage !== "length" && value === 0n)
      this.#fail();
    this.#accumulator = 0n;
    this.#digits = 0;
    switch (stage) {
      case "activation":
        this.#activation = value;
        this.#stage = "request";
        return;
      case "request":
        this.#request = Number(value);
        this.#stage = this.#tag === 0 ? "fault" : "return";
        return;
      case "return":
        this.#returnSequence = value;
        this.#stage = this.#tag === 2 || this.#controlTag === 1 ? "page" : this.#tag === 1 ? "reason" : this.#tag === 3 ? "completion" : "outcome";
        return;
      case "page":
        this.#pageSequence = value;
        this.#stage = "length";
        return;
      case "length":
        this.#length = Number(value);
        this.#stage = "final";
        return;
      default:
        this.#fail();
    }
  }
  finish() {
    if (this.#failed || this.#stage !== "done")
      this.#fail();
    if (this.#value !== null)
      return this.#value;
    const fault = this.#faultValue;
    if (this.#tag === 5) {
      if (fault !== "malformedControl" && fault !== "mixedControl")
        this.#fail();
      return this.#value = Object.freeze({ kind: "protocolFault", fault });
    }
    if (this.#tag === 0 && fault === "none")
      this.#fail();
    if (this.#tag === 4) {
      const success = this.#outcome === "accepted" || this.#outcome === "duplicate";
      if (success ? fault !== "none" || this.#controlTag === 0 : fault === "none")
        this.#fail();
    }
    const origin = Object.freeze({ activationGeneration: this.#activation, requestSequence: this.#request });
    if (this.#tag === 0)
      return this.#value = Object.freeze({ kind: "refused", origin, fault });
    const identity = Object.freeze({ origin, returnSequence: this.#returnSequence });
    if (this.#tag === 1)
      return this.#value = Object.freeze({ kind: "pending", identity, reason: this.#reason });
    if (this.#tag === 3)
      return this.#value = Object.freeze({ kind: "retired", identity, completion: this.#completion });
    if (this.#tag === 2 || this.#controlTag === 1) {
      const receipt = Object.freeze({ identity, pageSequence: this.#pageSequence, length: this.#length, final: this.#final });
      if (this.#tag === 2)
        return this.#value = Object.freeze({ kind: "page", receipt, payloadOffset: this.#payloadOffset });
      return this.#value = Object.freeze({ kind: "control", control: Object.freeze({ kind: "inputAck", receipt }), outcome: this.#outcome, fault });
    }
    const kind = this.#controlTag === 0 ? "poll" : this.#controlTag === 2 ? "cancel" : "retiredAck";
    return this.#value = Object.freeze({ kind: "control", control: Object.freeze({ kind, identity }), outcome: this.#outcome, fault });
  }
}
function decodeActorReturnResult(bytes) {
  if (!(bytes instanceof Uint8Array) || bytes.length < 1 || bytes.length > ACTOR_RETURN_RESULT_MAXIMUM_BYTES)
    throw new Error("actor-return.envelope");
  const parser = new ActorReturnResultFraming;
  for (const byte of bytes)
    parser.push(byte);
  const value = parser.finish();
  return value.kind === "page" ? Object.freeze({ kind: "page", receipt: value.receipt, page: createActorBytePage(bytes.subarray(value.payloadOffset, value.payloadOffset + value.receipt.length)) }) : value;
}
if (undefined) {}
/* ../../../../../../../../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts */
var queueTask;
var ownersOf;
var closeOwner;
var beginEdit;
var beginRead;
var MIN_STEP_BYTES = 256;
var idle = (kind) => ({ kind, items: 0, bytes: 0 });
var pending = (bytes) => ({ kind: "pending", items: 1, bytes });
var admitted2 = (grant) => grant.maxItems >= 1 && grant.maxBytes >= MIN_STEP_BYTES;
function idKey(id) {
  if (!Number.isSafeInteger(id) || id < 0)
    throw new RangeError("Numeric index IDs must be nonnegative safe integers");
  return { high: 0, low: id === 0 ? 0 : id };
}
function compare(a, b) {
  return a.high < b.high ? -1 : a.high > b.high ? 1 : a.low < b.low ? -1 : a.low > b.low ? 1 : 0;
}
function nextOrdinal(key) {
  if (key.low < Number.MAX_SAFE_INTEGER)
    return { high: key.high, low: key.low + 1 };
  if (key.high === Number.MAX_SAFE_INTEGER)
    throw new RangeError("Numeric index insertion ordinal exhausted");
  return { high: key.high + 1, low: 0 };
}
function retain(node) {
  if (node) {
    checkReferences(node, 1);
    node.refs++;
  }
  return node;
}
function checkReferences(owner, count) {
  if (!Number.isSafeInteger(owner.refs) || owner.refs < 1 || owner.refs > Number.MAX_SAFE_INTEGER - count)
    throw new RangeError("Numeric index reference capacity exhausted");
}
function valueEntry(node) {
  if (!node.entry || node.refs < 1)
    throw new Error("Retired numeric index node");
  return node.entry;
}
function entryValue(entry) {
  if (!entry.payload)
    throw new Error("Retired numeric index payload");
  return entry.payload.value;
}

class NumericIndexRetirement {
  #head = null;
  constructor() {
    Object.freeze(this);
  }
  static {
    queueTask = (owner, task) => owner.#enqueue(task);
  }
  #enqueue(task) {
    if (task)
      this.#head = { task, next: this.#head };
  }
  terminalIsEmpty() {
    return this.#head === null;
  }
  advance(grant) {
    if (!this.#head)
      return idle("complete");
    if (!admitted2(grant))
      return idle("blocked");
    const cell = this.#head;
    this.#head = cell.next;
    cell.next = null;
    const task = cell.task;
    cell.task = null;
    if (task.kind === "allocation") {
      const bytes = task.allocation.closeStep(this);
      if (!task.allocation.terminalIsEmpty())
        this.#enqueue(task);
      return pending(bytes + 24);
    }
    if (task.kind === "frame") {
      const frame = task.frame;
      const next = frame.next;
      frame.node = null;
      frame.next = null;
      if (next)
        this.#enqueue({ kind: "frame", frame: next });
      return pending(64);
    }
    if (task.kind === "entry") {
      const entry2 = task.entry;
      if (--entry2.refs < 0)
        throw new Error("Numeric index entry released twice");
      if (entry2.refs !== 0)
        return pending(24);
      const value = entryValue(entry2);
      entry2.payload = null;
      return { kind: "retired", value, items: 1, bytes: 40 };
    }
    const node = task.node;
    if (--node.refs < 0)
      throw new Error("Numeric index node released twice");
    if (node.refs !== 0)
      return pending(24);
    const left = node.left;
    const right = node.right;
    const entry = node.entry;
    node.left = null;
    node.right = null;
    node.entry = null;
    if (entry)
      this.#enqueue({ kind: "entry", entry });
    if (right)
      this.#enqueue({ kind: "node", node: right });
    if (left)
      this.#enqueue({ kind: "node", node: left });
    return pending(128);
  }
}
function allocationNode(key, entry, left, right) {
  return { key, entry, left, right, built: null, allocated: false };
}

class TreeAllocation {
  #reservations = [];
  #slot = 0;
  #scan = 0;
  #target = null;
  #reserved = false;
  #allocated = 0;
  #closed = false;
  nodes;
  constructor(nodes) {
    this.nodes = nodes;
  }
  advance() {
    if (!this.#reserved) {
      if (this.#target) {
        const reservation = this.#reservations[this.#scan];
        if (!reservation) {
          this.#reservations.push({ owner: this.#target, count: 1 });
          this.#target = null;
          this.#scan = 0;
          return 40;
        }
        if (reservation.owner === this.#target) {
          reservation.count++;
          this.#target = null;
          this.#scan = 0;
        } else
          this.#scan++;
        return 24;
      }
      if (this.#slot < this.nodes.length * 3) {
        const spec2 = this.nodes[Math.floor(this.#slot / 3)];
        const value = this.#slot % 3 === 0 ? spec2.entry : this.#slot % 3 === 1 ? spec2.left : spec2.right;
        this.#slot++;
        if (value !== null && typeof value !== "number")
          this.#target = value;
        return 32;
      }
      for (const reservation of this.#reservations)
        checkReferences(reservation.owner, reservation.count);
      for (const reservation of this.#reservations)
        reservation.owner.refs += reservation.count;
      this.#reserved = true;
      return 32 + this.#reservations.length * 16;
    }
    if (this.#allocated === this.nodes.length)
      return 0;
    const spec = this.nodes[this.#allocated];
    const take = (child) => {
      if (typeof child !== "number")
        return child;
      const source = this.nodes[child];
      const result = source.built;
      source.built = null;
      return result;
    };
    const left = take(spec.left);
    const right = take(spec.right);
    spec.built = { refs: 1, key: spec.key, height: 1 + Math.max(left?.height ?? 0, right?.height ?? 0), left, right, entry: spec.entry };
    spec.allocated = true;
    this.#allocated++;
    return 96;
  }
  get ready() {
    return this.#reserved && this.#allocated === this.nodes.length;
  }
  get reservationsReady() {
    return this.#target === null && this.#slot === this.nodes.length * 3;
  }
  takeRoot() {
    if (!this.ready)
      throw new Error("Numeric allocation is incomplete");
    const last = this.nodes[this.nodes.length - 1];
    if (!last.built)
      throw new Error("Numeric allocation already transferred");
    const root = last.built;
    last.built = null;
    return root;
  }
  closeStep(retirement) {
    this.#target = null;
    const spec = this.nodes.pop();
    if (spec) {
      if (spec.built)
        queueTask(retirement, { kind: "node", node: spec.built });
      if (this.#reserved && !spec.allocated) {
        queueTask(retirement, { kind: "entry", entry: spec.entry });
        if (spec.left !== null && typeof spec.left !== "number")
          queueTask(retirement, { kind: "node", node: spec.left });
        if (spec.right !== null && typeof spec.right !== "number")
          queueTask(retirement, { kind: "node", node: spec.right });
      }
      spec.built = null;
      return 136;
    }
    if (this.#reservations.length) {
      this.#reservations.pop();
      return 24;
    }
    this.#closed = true;
    return 16;
  }
  terminalIsEmpty() {
    return this.#closed;
  }
}
function balancedAllocation(key, entry, left, right) {
  const balance = (left?.height ?? 0) - (right?.height ?? 0);
  if (balance > 1 && left) {
    if ((left.left?.height ?? 0) >= (left.right?.height ?? 0))
      return new TreeAllocation([allocationNode(key, entry, left.right, right), allocationNode(left.key, valueEntry(left), left.left, 0)]);
    const pivot = left.right;
    return new TreeAllocation([allocationNode(left.key, valueEntry(left), left.left, pivot.left), allocationNode(key, entry, pivot.right, right), allocationNode(pivot.key, valueEntry(pivot), 0, 1)]);
  }
  if (balance < -1 && right) {
    if ((right.right?.height ?? 0) >= (right.left?.height ?? 0))
      return new TreeAllocation([allocationNode(key, entry, left, right.left), allocationNode(right.key, valueEntry(right), 0, right.right)]);
    const pivot = right.left;
    return new TreeAllocation([allocationNode(key, entry, left, pivot.left), allocationNode(right.key, valueEntry(right), pivot.right, right.right), allocationNode(pivot.key, valueEntry(pivot), 0, 1)]);
  }
  return new TreeAllocation([allocationNode(key, entry, left, right)]);
}

class TreeEdit {
  #scan;
  #path = null;
  #successorPath = null;
  #target = null;
  #replacement = null;
  #replacementKey = null;
  #work = null;
  #phase = "search";
  #allocation = null;
  key;
  entry;
  retirement;
  constructor(root, key, entry, retirement) {
    this.key = key;
    this.entry = entry;
    this.retirement = retirement;
    this.#scan = root;
  }
  #replaceWork(node) {
    if (this.#work)
      queueTask(this.retirement, { kind: "node", node: this.#work });
    this.#work = node;
  }
  advance() {
    if (this.#phase === "ready" || this.#phase === "closed")
      return 0;
    if (this.#allocation) {
      if (!this.#allocation.ready)
        return this.#allocation.advance();
      const allocation = this.#allocation;
      this.#allocation = null;
      this.#replaceWork(allocation.takeRoot());
      queueTask(this.retirement, { kind: "allocation", allocation });
      return 64;
    }
    if (this.#phase === "search") {
      const node = this.#scan;
      if (!node) {
        if (this.entry)
          this.#allocation = balancedAllocation(this.key, this.entry, null, null);
        this.#phase = "rebuild";
      } else {
        const order = compare(this.key, node.key);
        if (order !== 0) {
          this.#path = { node, right: order > 0, next: this.#path };
          this.#scan = order > 0 ? node.right : node.left;
        } else if (this.entry) {
          this.#allocation = balancedAllocation(this.key, this.entry, node.left, node.right);
          this.#scan = null;
          this.#phase = "rebuild";
        } else if (!node.left || !node.right) {
          this.#work = retain(node.left ?? node.right);
          this.#scan = null;
          this.#phase = "rebuild";
        } else {
          this.#target = node;
          this.#scan = node.right;
          this.#phase = "successor";
        }
      }
      return 112;
    }
    if (this.#phase === "successor") {
      const node = this.#scan;
      if (node.left) {
        this.#successorPath = { node, right: false, next: this.#successorPath };
        this.#scan = node.left;
      } else {
        this.#replacement = valueEntry(node);
        this.#replacementKey = node.key;
        this.#work = retain(node.right);
        this.#scan = null;
        this.#phase = "successor-rebuild";
      }
      return 48;
    }
    if (this.#phase === "successor-rebuild") {
      const frame2 = this.#successorPath;
      if (frame2) {
        const node = frame2.node;
        this.#successorPath = frame2.next;
        frame2.next = null;
        frame2.node = null;
        this.#allocation = balancedAllocation(node.key, valueEntry(node), this.#work, node.right);
      } else {
        this.#allocation = balancedAllocation(this.#replacementKey, this.#replacement, this.#target.left, this.#work);
        this.#target = null;
        this.#replacement = null;
        this.#replacementKey = null;
        this.#phase = "rebuild";
      }
      return 240;
    }
    const frame = this.#path;
    if (frame) {
      const node = frame.node;
      this.#path = frame.next;
      frame.next = null;
      frame.node = null;
      this.#allocation = balancedAllocation(node.key, valueEntry(node), frame.right ? node.left : this.#work, frame.right ? this.#work : node.right);
    } else
      this.#phase = "ready";
    return 240;
  }
  get ready() {
    return this.#phase === "ready";
  }
  takeRoot() {
    if (!this.ready)
      throw new Error("Numeric tree edit is not ready");
    const root = this.#work;
    this.#work = null;
    this.#phase = "closed";
    return root;
  }
  closeInto(retirement) {
    if (this.#work)
      queueTask(retirement, { kind: "node", node: this.#work });
    if (this.#path)
      queueTask(retirement, { kind: "frame", frame: this.#path });
    if (this.#successorPath)
      queueTask(retirement, { kind: "frame", frame: this.#successorPath });
    if (this.#allocation)
      queueTask(retirement, { kind: "allocation", allocation: this.#allocation });
    this.#allocation = null;
    this.#work = null;
    this.#path = null;
    this.#successorPath = null;
    this.#target = null;
    this.#replacement = null;
    this.#replacementKey = null;
    this.#scan = null;
    this.#phase = "closed";
  }
}
var adopt;

class NumericIndex {
  #owners;
  constructor(owners) {
    this.#owners = owners;
    Object.freeze(this);
  }
  static {
    adopt = (owners) => new NumericIndex(owners);
    ownersOf = (index) => index.#live();
    closeOwner = (index, retirement) => index.#closeInto(retirement);
  }
  static empty(firstOrdinal = { high: 0, low: 0 }) {
    idKey(firstOrdinal.high);
    idKey(firstOrdinal.low);
    return new NumericIndex({ ids: null, order: null, size: 0, next: { high: firstOrdinal.high, low: firstOrdinal.low } });
  }
  #live() {
    if (!this.#owners)
      throw new Error("Numeric index owner is closed");
    return this.#owners;
  }
  get size() {
    return this.#live().size;
  }
  terminalIsEmpty() {
    return this.#owners === null;
  }
  nextOrdinal() {
    const next = this.#live().next;
    return { high: next.high, low: next.low };
  }
  capture() {
    const source = this.#live();
    this.#checkCaptureCapacity();
    return adopt({ ids: retain(source.ids), order: retain(source.order), size: source.size, next: source.next });
  }
  #checkCaptureCapacity() {
    const source = this.#live();
    if (source.ids)
      checkReferences(source.ids, source.ids === source.order ? 2 : 1);
    if (source.order && source.order !== source.ids)
      checkReferences(source.order, 1);
  }
  assertCaptureCapacity() {
    this.#checkCaptureCapacity();
  }
  get(id) {
    const key = idKey(id);
    let node = this.#live().ids;
    while (node) {
      const order = compare(key, node.key);
      if (order === 0)
        return entryValue(valueEntry(node));
      node = order > 0 ? node.right : node.left;
    }
    return;
  }
  *[Symbol.iterator]() {
    let node = this.#live().order;
    let path = null;
    while (node || path) {
      this.#live();
      if (node) {
        path = { node, right: false, next: path };
        node = node.left;
        continue;
      }
      const frame = path;
      path = frame.next;
      node = frame.node;
      const entry = valueEntry(node);
      yield [entry.id, entryValue(entry)];
      this.#live();
      node = node.right;
    }
  }
  beginSet(id, value) {
    const key = idKey(id);
    return beginEdit(this.capture(), key, { value });
  }
  beginRemove(id) {
    const key = idKey(id);
    return beginEdit(this.capture(), key, null);
  }
  beginRead() {
    return beginRead(this.capture(), null, true);
  }
  beginSortedRead() {
    return beginRead(this.capture(), null, false);
  }
  beginLookup(id) {
    const key = idKey(id);
    return beginRead(this.capture(), key, false);
  }
  beginClose() {
    const retirement = new NumericIndexRetirement;
    this.#closeInto(retirement);
    return retirement;
  }
  #closeInto(retirement) {
    const owners = this.#owners;
    this.#owners = null;
    if (owners?.ids)
      queueTask(retirement, { kind: "node", node: owners.ids });
    if (owners?.order)
      queueTask(retirement, { kind: "node", node: owners.order });
  }
}

class NumericIndexReader {
  #source;
  #scan;
  #path = null;
  #complete = false;
  key;
  static {
    beginRead = (source, key, ordered) => new NumericIndexReader(source, key, ordered);
  }
  constructor(source, key, ordered) {
    this.key = key;
    this.#source = source;
    const owners = ownersOf(source);
    this.#scan = ordered ? owners.order : owners.ids;
    Object.freeze(this);
  }
  advance(grant) {
    if (!this.#source)
      throw new Error("Numeric index reader is closed");
    if (this.#complete)
      return { kind: "complete", items: 0, bytes: 0 };
    if (!admitted2(grant))
      return { kind: "blocked", items: 0, bytes: 0 };
    if (this.key) {
      const node2 = this.#scan;
      if (!node2) {
        this.#complete = true;
        return { kind: "complete", items: 1, bytes: 16 };
      }
      const order = compare(this.key, node2.key);
      if (order !== 0) {
        this.#scan = order > 0 ? node2.right : node2.left;
        return { kind: "pending", items: 1, bytes: 32 };
      }
      this.#scan = null;
      this.#complete = true;
      const entry2 = valueEntry(node2);
      return { kind: "value", id: entry2.id, ordinal: { ...entry2.ordinal }, value: entryValue(entry2), items: 1, bytes: 64 };
    }
    if (this.#scan) {
      this.#path = { node: this.#scan, right: false, next: this.#path };
      this.#scan = this.#scan.left;
      return { kind: "pending", items: 1, bytes: 64 };
    }
    const frame = this.#path;
    if (!frame) {
      this.#complete = true;
      return { kind: "complete", items: 1, bytes: 16 };
    }
    const node = frame.node;
    this.#path = frame.next;
    frame.next = null;
    frame.node = null;
    this.#scan = node.right;
    const entry = valueEntry(node);
    return { kind: "value", id: entry.id, ordinal: { ...entry.ordinal }, value: entryValue(entry), items: 1, bytes: 80 };
  }
  terminalIsEmpty() {
    return this.#source === null && this.#path === null && this.#scan === null;
  }
  beginClose() {
    if (!this.#source)
      throw new Error("Numeric index reader is already closed");
    const retirement = new NumericIndexRetirement;
    closeOwner(this.#source, retirement);
    if (this.#path)
      queueTask(retirement, { kind: "frame", frame: this.#path });
    this.#source = null;
    this.#scan = null;
    this.#path = null;
    this.#complete = true;
    return retirement;
  }
}

class NumericIndexEdit {
  #input;
  #source;
  #scan;
  #old = null;
  #entry = null;
  #tree = null;
  #ids = null;
  #order = null;
  #result = null;
  #retirement = new NumericIndexRetirement;
  #phase = "lookup";
  key;
  static {
    beginEdit = (source, key, input) => new NumericIndexEdit(source, key, input);
  }
  constructor(source, key, input) {
    this.key = key;
    this.#source = source;
    this.#scan = ownersOf(source).ids;
    this.#input = input;
    Object.freeze(this);
  }
  advance(grant) {
    if (this.#phase === "closed")
      return idle("complete");
    if (!admitted2(grant))
      return idle("blocked");
    const retirement = this.#retirement;
    if (!retirement.terminalIsEmpty())
      return retirement.advance(grant);
    if (this.#phase === "ready")
      return idle("ready");
    if (this.#phase === "rejected")
      return { kind: "rejected", reason: "ordinal-exhausted", items: 0, bytes: 0 };
    const source = ownersOf(this.#source);
    if (this.#phase === "lookup") {
      const node = this.#scan;
      if (!node)
        this.#phase = "entry";
      else {
        const order = compare(this.key, node.key);
        if (order === 0) {
          this.#old = valueEntry(node);
          this.#scan = null;
          this.#phase = "entry";
        } else
          this.#scan = order > 0 ? node.right : node.left;
      }
      return pending(32);
    }
    if (this.#phase === "entry") {
      if (!this.#input && !this.#old) {
        this.#result = this.#source.capture();
        this.#phase = "ready";
        return pending(32);
      }
      if (this.#input) {
        if (!this.#old && source.next.high === Number.MAX_SAFE_INTEGER && source.next.low === Number.MAX_SAFE_INTEGER) {
          this.#phase = "rejected";
          return { kind: "rejected", reason: "ordinal-exhausted", items: 1, bytes: 16 };
        }
        this.#entry = { refs: 1, id: this.key.low, ordinal: this.#old?.ordinal ?? source.next, payload: this.#input };
        this.#input = null;
      }
      this.#tree = new TreeEdit(source.ids, this.key, this.#entry, retirement);
      this.#phase = "ids";
      return pending(96);
    }
    if (this.#phase === "ids" || this.#phase === "order") {
      if (!this.#tree.ready)
        return pending(this.#tree.advance());
      const root = this.#tree.takeRoot();
      this.#tree = null;
      if (this.#phase === "ids") {
        this.#ids = root;
        this.#tree = new TreeEdit(source.order, this.#old?.ordinal ?? this.#entry.ordinal, this.#entry, retirement);
        this.#phase = "order";
      } else {
        this.#order = root;
        this.#phase = "publish";
      }
      return pending(64);
    }
    const inserted = this.#entry !== null && this.#old === null;
    this.#result = adopt({ ids: this.#ids, order: this.#order, size: source.size + (inserted ? 1 : this.#entry ? 0 : -1), next: inserted ? nextOrdinal(source.next) : source.next });
    this.#ids = null;
    this.#order = null;
    this.#phase = "ready";
    return pending(64);
  }
  takeResult() {
    if (this.#phase !== "ready")
      return null;
    const result = this.#result;
    this.#result = null;
    return result;
  }
  terminalIsEmpty() {
    return this.#phase === "closed" && this.#retirement === null;
  }
  beginClose() {
    if (this.#phase === "closed")
      throw new Error("Numeric index edit is already closed");
    const retirement = this.#retirement;
    this.#retirement = null;
    if (this.#source)
      closeOwner(this.#source, retirement);
    if (this.#result)
      closeOwner(this.#result, retirement);
    if (this.#ids)
      queueTask(retirement, { kind: "node", node: this.#ids });
    if (this.#order)
      queueTask(retirement, { kind: "node", node: this.#order });
    if (this.#entry)
      queueTask(retirement, { kind: "entry", entry: this.#entry });
    if (this.#input)
      queueTask(retirement, { kind: "entry", entry: { refs: 1, id: this.key.low, ordinal: this.key, payload: this.#input } });
    this.#tree?.closeInto(retirement);
    this.#source = null;
    this.#result = null;
    this.#ids = null;
    this.#order = null;
    this.#entry = null;
    this.#input = null;
    this.#tree = null;
    this.#scan = null;
    this.#old = null;
    this.#phase = "closed";
    return retirement;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️.ts */
function finite(component) {
  switch (component.type) {
    case "slider":
      return Number.isFinite(component.value) && Number.isFinite(component.min) && Number.isFinite(component.max) && Number.isFinite(component.step);
    case "numberStepper":
      return Number.isFinite(component.value) && Number.isFinite(component.step);
    case "ring":
      return Number.isFinite(component.t);
    case "input":
      return (component.min == null || Number.isFinite(component.min)) && (component.max == null || Number.isFinite(component.max)) && (component.step == null || Number.isFinite(component.step));
    default:
      return true;
  }
}
function* violation(value, frontier, violations) {
  if (frontier.count === Number.MAX_SAFE_INTEGER)
    throw new RangeError("Retained UI violation ordinal exhausted");
  yield* violations.set(frontier.count++, value);
}
function closeRetainedUiGraphFrame(frontier) {
  const cell = frontier.stack;
  if (!cell)
    return false;
  frontier.stack = cell.next;
  cell.next = null;
  cell.value = null;
  return true;
}
function* retainedUiGraphValidation(nodes, root, limits, marks, keys, violations, frontier) {
  if (nodes.size > limits.maxNodes) {
    yield* violation({ type: "nodeQuota", count: nodes.size, max: limits.maxNodes }, frontier, violations);
    return;
  }
  if (root !== null && (yield* nodes.lookup(root)))
    frontier.stack = { value: { kind: "enter", id: root, depth: 0, section: false }, next: null };
  while (frontier.stack) {
    const cell = frontier.stack;
    frontier.stack = cell.next;
    cell.next = null;
    const frame = cell.value;
    cell.value = null;
    yield 48;
    const flags = (yield* marks.lookup(frame.id)) ?? 0;
    if (frame.kind === "exit") {
      yield* marks.set(frame.id, flags & ~2);
      continue;
    }
    if (flags & 2) {
      yield* violation({ type: "cycle", node: frame.id }, frontier, violations);
      continue;
    }
    if (flags & 1)
      continue;
    yield* marks.set(frame.id, 1);
    const record = yield* nodes.lookup(frame.id);
    if (!record)
      continue;
    const section = record.component.type === "container" && record.component.role === "section";
    if (frame.section && section)
      yield* violation({ type: "sectionNested", node: frame.id }, frontier, violations);
    if (!finite(record.component))
      yield* violation({ type: "nonFiniteNumber", node: frame.id }, frontier, violations);
    if (frame.depth > limits.maxDepth) {
      yield* violation({ type: "depthQuota", node: frame.id, depth: frame.depth, max: limits.maxDepth }, frontier, violations);
      continue;
    }
    yield* marks.set(frame.id, 3);
    frontier.stack = { value: { ...frame, kind: "exit" }, next: frontier.stack };
    yield 48;
    for (const childId of record.children ?? []) {
      const child = yield* nodes.lookup(childId);
      if (!child) {
        yield* violation({ type: "orphanChild", parent: frame.id, child: childId }, frontier, violations);
        continue;
      }
      if (yield* keys.insert(child.key))
        yield* violation({ type: "duplicateSiblingKey", parent: frame.id, key: child.key }, frontier, violations);
      frontier.stack = { value: { kind: "enter", id: childId, depth: frame.depth + 1, section: frame.section || section }, next: frontier.stack };
      yield 64;
    }
    yield* keys.clear();
  }
  const entries = nodes.entries();
  for (;; ) {
    const step2 = entries.next();
    if (step2.done)
      break;
    if (typeof step2.value === "number") {
      yield step2.value;
      continue;
    }
    const id = step2.value[0];
    yield 64;
    if (!((yield* marks.lookup(id)) ?? 0))
      yield* violation({ type: "danglingRoot", node: id }, frontier, violations);
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts */
var granted2 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 256;
var quota = (quota2, actual, max) => ({ type: "quotaExceeded", quota: quota2, actual, max });
function copyLimits(value) {
  return { maxNodes: value.maxNodes, maxDepth: value.maxDepth, maxChildren: value.maxChildren, maxTextBytes: value.maxTextBytes, maxPatchOps: value.maxPatchOps, maxPatchBytes: value.maxPatchBytes };
}
function copyRecord(value) {
  return { id: value.id, key: value.key, component: value.component, layout: value.layout, style: value.style, activity: value.activity, disabled: value.disabled, transition: value.transition, accessibility: value.accessibility, bindings: value.bindings, menu: value.menu, children: value.children };
}

class Table {
  #index;
  #edit = null;
  #reader = null;
  #retirement = null;
  #old = null;
  grant;
  retired;
  constructor(index, grant, retired = () => {}) {
    this.grant = grant;
    this.retired = retired;
    this.#index = index;
  }
  get index() {
    if (!this.#index)
      throw new Error("Retained table owner is closed");
    return this.#index;
  }
  get size() {
    return this.index.size;
  }
  *#drain() {
    while (this.#retirement) {
      const step2 = this.#retirement.advance(this.grant());
      if (step2.kind === "retired")
        this.retired(step2.value);
      if (step2.kind === "complete")
        this.#retirement = null;
      yield step2.bytes;
    }
  }
  *lookup(id) {
    this.#reader = this.index.beginLookup(id);
    let result;
    for (;; ) {
      const step2 = this.#reader.advance(this.grant());
      if (step2.kind === "value")
        result = step2.value;
      yield step2.bytes;
      if (step2.kind === "complete")
        break;
    }
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield* this.#drain();
    return result;
  }
  *entries() {
    this.#reader = this.index.beginRead();
    for (;; ) {
      const step2 = this.#reader.advance(this.grant());
      if (step2.kind === "value")
        yield [step2.id, step2.value];
      else
        yield step2.bytes;
      if (step2.kind === "complete")
        break;
    }
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield* this.#drain();
  }
  *set(id, value) {
    yield* this.#change(this.index.beginSet(id, value));
  }
  *remove(id) {
    yield* this.#change(this.index.beginRemove(id));
  }
  *#change(edit) {
    this.#edit = edit;
    for (;; ) {
      const step2 = edit.advance(this.grant());
      if (step2.kind === "retired")
        this.retired(step2.value);
      if (step2.kind === "rejected")
        throw new RangeError("Retained UI insertion ordinal exhausted");
      yield step2.bytes;
      if (step2.kind === "ready")
        break;
    }
    this.#old = this.#index;
    this.#index = edit.takeResult();
    this.#retirement = edit.beginClose();
    this.#edit = null;
    yield* this.#drain();
    this.#retirement = this.#old.beginClose();
    this.#old = null;
    yield* this.#drain();
  }
  take() {
    if (!this.#index || this.#edit || this.#reader || this.#retirement || this.#old)
      throw new Error("Retained table is not transferable");
    const result = this.#index;
    this.#index = null;
    return result;
  }
  closeStep(grant) {
    if (this.#retirement) {
      const step2 = this.#retirement.advance(grant);
      if (step2.kind === "retired")
        this.retired(step2.value);
      if (step2.kind === "complete")
        this.#retirement = null;
      return { complete: false, bytes: step2.bytes };
    }
    if (this.#reader) {
      this.#retirement = this.#reader.beginClose();
      this.#reader = null;
      return { complete: false, bytes: 64 };
    }
    if (this.#edit) {
      this.#retirement = this.#edit.beginClose();
      this.#edit = null;
      return { complete: false, bytes: 128 };
    }
    if (this.#old) {
      this.#retirement = this.#old.beginClose();
      this.#old = null;
      return { complete: false, bytes: 64 };
    }
    if (this.#index) {
      this.#retirement = this.#index.beginClose();
      this.#index = null;
      return { complete: false, bytes: 64 };
    }
    return { complete: true, bytes: 0 };
  }
}
function* componentStrings(component) {
  switch (component.type) {
    case "container":
      yield component.label ?? "";
      yield component.description ?? "";
      yield component.error ?? "";
      break;
    case "text":
      yield component.value;
      break;
    case "button":
      yield component.label;
      break;
    case "input":
      yield component.value;
      yield component.placeholder ?? "";
      break;
    case "select":
      for (const item of component.items)
        yield item.label;
      yield component.placeholder ?? "";
      break;
    case "toggle":
      yield component.text ?? "";
      break;
    case "keyValueList":
      for (const entry of component.entries) {
        yield entry.label;
        yield entry.value;
      }
      break;
    case "treeSection":
      yield component.label ?? "";
      break;
    case "treeItem":
      yield component.label;
      yield component.description ?? "";
      break;
    case "image":
      yield component.alt ?? "";
      break;
    case "extension":
      yield component.extension;
      break;
  }
}
function* accessibilityStrings(value) {
  yield value.label ?? "";
  yield value.description ?? "";
  yield value.shortcut ?? "";
}
function* bindingStrings(values) {
  for (const value of values) {
    yield value.action.scope;
    yield value.action.name;
    yield value.capability ?? "";
  }
}
function* opStrings(op) {
  switch (op.type) {
    case "upsert":
      yield op.key;
      yield* componentStrings(op.component);
      yield* accessibilityStrings(op.accessibility);
      yield* bindingStrings(op.bindings ?? []);
      yield op.menu?.id ?? "";
      break;
    case "setComponent":
      yield* componentStrings(op.component);
      break;
    case "setAccessibility":
      yield* accessibilityStrings(op.accessibility);
      break;
    case "setBindings":
      yield* bindingStrings(op.bindings);
      break;
    case "setMenu":
      yield op.menu?.id ?? "";
      break;
  }
}
function* stringBytes(value, grant) {
  let offset = 0;
  let total = 0;
  yield 16;
  while (offset < value.length) {
    let consumed = 0;
    const budget = Math.min(4096, grant().maxBytes);
    while (offset < value.length && consumed + 8 <= budget) {
      const code = value.charCodeAt(offset++);
      if (code < 128)
        total++;
      else if (code < 2048)
        total += 2;
      else if (code >= 55296 && code <= 56319 && offset < value.length && value.charCodeAt(offset) >= 56320 && value.charCodeAt(offset) <= 57343) {
        offset++;
        total += 4;
      } else
        total += 3;
      consumed += 8;
    }
    yield consumed;
  }
  return total;
}
function* measure(values, grant) {
  let total = 0;
  for (const value of values)
    total += yield* stringBytes(value, grant);
  return total;
}
class SiblingKeys {
  #table = null;
  #owned = null;
  grant;
  constructor(grant) {
    this.grant = grant;
  }
  *insert(key) {
    this.#table ??= new Table(NumericIndex.empty(), this.grant);
    let hash = 2166136261;
    let offset = 0;
    yield 32;
    while (offset < key.length) {
      let work = 0;
      while (offset < key.length && work + 8 <= this.grant().maxBytes) {
        const unit = key.charCodeAt(offset++);
        hash = Math.imul(hash ^ unit & 255, 16777619) >>> 0;
        hash = Math.imul(hash ^ unit >>> 8, 16777619) >>> 0;
        work += 8;
      }
      yield work;
    }
    const head = yield* this.#table.lookup(hash);
    let cell = head;
    while (cell) {
      const existing = cell.key;
      yield 16;
      if (existing.length === key.length) {
        offset = 0;
        let mismatch = false;
        while (offset < key.length && !mismatch) {
          let work = 0;
          while (offset < key.length && work + 8 <= this.grant().maxBytes) {
            const left = key.charCodeAt(offset);
            const right = existing.charCodeAt(offset);
            work += 8;
            if (left !== right) {
              mismatch = true;
              break;
            }
            offset++;
          }
          yield work;
        }
        if (!mismatch)
          return true;
      }
      cell = cell.collision ?? undefined;
    }
    const inserted = { key, collision: head ?? null, ownedNext: this.#owned };
    this.#owned = inserted;
    yield 48;
    yield* this.#table.set(hash, inserted);
    return false;
  }
  *clear() {
    for (;; ) {
      const step2 = this.closeStep(this.grant());
      yield step2.bytes;
      if (step2.complete)
        return;
    }
  }
  closeStep(grant) {
    if (this.#table) {
      const step2 = this.#table.closeStep(grant);
      if (step2.complete)
        this.#table = null;
      return { complete: false, bytes: step2.bytes };
    }
    if (this.#owned) {
      const cell = this.#owned;
      this.#owned = cell.ownedNext;
      cell.key = null;
      cell.collision = null;
      cell.ownedNext = null;
      return { complete: false, bytes: 48 };
    }
    return { complete: true, bytes: 0 };
  }
}
class RetainedUiPatchCursor {
  #grant = { maxItems: 0, maxBytes: 0 };
  #nodes;
  #touched;
  #marks;
  #violations;
  #keys;
  #resources = null;
  #graph = { stack: null, count: 0 };
  #remove = null;
  #program = null;
  #result = null;
  #root;
  #closing = false;
  #closed = false;
  #outcome = null;
  #phase = "admission";
  source;
  patch;
  limits;
  constructor(source, patch, limits) {
    this.source = { surface: source.surface, revision: source.revision, root: source.root, nodes: source.nodes };
    this.patch = patch;
    this.limits = copyLimits(limits);
    this.#nodes = this.#table(source.nodes.capture());
    this.#touched = this.#table(NumericIndex.empty());
    this.#marks = this.#table(NumericIndex.empty());
    this.#violations = this.#table(NumericIndex.empty());
    this.#keys = new SiblingKeys(() => this.#grant);
    this.#resources = { value: this.#keys, next: this.#resources };
    this.#root = source.root;
  }
  #table(index) {
    const table = new Table(index, () => this.#grant);
    this.#resources = { value: table, next: this.#resources };
    return table;
  }
  *#apply(op) {
    if (op.type === "setRoot") {
      this.#root = op.id;
      yield 16;
      return null;
    }
    if (op.type === "upsert" || op.type === "setComponent") {
      if (op.type === "upsert" && (op.children?.length ?? 0) > this.limits.maxChildren)
        return quota("children", op.children.length, this.limits.maxChildren);
      const bytes = yield* measure(componentStrings(op.component), () => this.#grant);
      if (bytes > this.limits.maxTextBytes)
        return quota("textBytes", bytes, this.limits.maxTextBytes);
    }
    if (op.type === "setChildren" && op.children.length > this.limits.maxChildren)
      return quota("children", op.children.length, this.limits.maxChildren);
    if (op.type === "remove") {
      this.#remove = { value: op.id, next: null };
      while (this.#remove) {
        const item = this.#remove;
        this.#remove = item.next;
        item.next = null;
        const id = item.value;
        item.value = null;
        yield 32;
        const record2 = yield* this.#nodes.lookup(id);
        if (!record2)
          continue;
        yield* this.#nodes.remove(id);
        yield* this.#touched.set(id, true);
        for (const child of record2.children ?? []) {
          this.#remove = { value: child, next: this.#remove };
          yield 32;
        }
      }
      return null;
    }
    let record;
    if (op.type === "upsert") {
      record = { id: op.id, key: op.key, component: op.component, layout: op.layout, style: op.style, activity: op.activity, disabled: op.disabled, transition: op.transition, accessibility: op.accessibility, bindings: op.bindings, menu: op.menu, children: op.children };
    } else {
      const current = yield* this.#nodes.lookup(op.id);
      if (!current)
        return { type: "unknownNode", id: op.id };
      record = copyRecord(current);
      switch (op.type) {
        case "setComponent":
          record.component = op.component;
          break;
        case "setLayout":
          record.layout = op.layout;
          break;
        case "setActivity":
          record.activity = op.activity;
          record.disabled = op.disabled;
          break;
        case "setChildren":
          record.children = op.children;
          break;
        case "setStyle":
          record.style = op.style;
          break;
        case "setAccessibility":
          record.accessibility = op.accessibility;
          break;
        case "setBindings":
          record.bindings = op.bindings;
          break;
        case "setMenu":
          record.menu = op.menu;
          break;
      }
    }
    yield 192;
    yield* this.#nodes.set(op.id, record);
    yield* this.#touched.set(op.id, true);
    return null;
  }
  *#validate() {
    yield* retainedUiGraphValidation(this.#nodes, this.#root, this.limits, this.#marks, this.#keys, this.#violations, this.#graph);
  }
  *#run() {
    const patch = this.patch;
    const source = this.source;
    if (patch.baseRevision !== source.revision)
      return { ok: false, rejection: { type: "revisionMismatch", expected: source.revision, actual: patch.baseRevision } };
    if (patch.ops.length > this.limits.maxPatchOps)
      return { ok: false, rejection: quota("patchOps", patch.ops.length, this.limits.maxPatchOps) };
    this.#phase = "accounting";
    let bytes = 0;
    for (const op of patch.ops) {
      bytes += 16 + (op.type === "setChildren" ? op.children.length * 8 : 0);
      yield 32;
      bytes += yield* measure(opStrings(op), () => this.#grant);
    }
    if (bytes > this.limits.maxPatchBytes)
      return { ok: false, rejection: quota("patchBytes", bytes, this.limits.maxPatchBytes) };
    this.#phase = "application";
    for (const op of patch.ops) {
      yield 16;
      const rejection = yield* this.#apply(op);
      if (rejection)
        return { ok: false, rejection };
    }
    this.#phase = "validation";
    yield* this.#validate();
    if (this.#graph.count)
      return { ok: false, rejection: { type: "invariantViolated", violations: this.#violations.take() } };
    this.#phase = "candidate";
    return { ok: true, state: { surface: source.surface, revision: patch.revision, root: this.#root, nodes: this.#nodes.take() }, touched: this.#touched.take() };
  }
  advance(grant) {
    if (this.#closed)
      return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#closing)
      throw new Error("Use closeStep after cancelling a retained UI patch");
    if (this.#outcome)
      return { kind: this.#outcome, phase: this.#phase, items: 0, bytes: 0 };
    if (!granted2(grant))
      return { kind: "blocked", phase: this.#phase, items: 0, bytes: 0 };
    this.#grant = grant;
    this.#program ??= this.#run();
    const step2 = this.#program.next();
    if (step2.done) {
      if (!step2.value)
        throw new Error("Retained UI work ended without an outcome");
      this.#result = step2.value;
      this.#program = null;
      this.#outcome = step2.value.ok ? "ready" : "rejected";
      return { kind: step2.value.ok ? "ready" : "rejected", phase: this.#phase, items: 1, bytes: 128 };
    }
    if (step2.value > grant.maxBytes)
      throw new Error("Retained UI work exceeded its byte grant");
    return { kind: "pending", phase: this.#phase, items: 1, bytes: step2.value };
  }
  takeResult() {
    const result = this.#result;
    this.#result = null;
    return result;
  }
  beginClose() {
    if (this.#closing || this.#closed)
      return;
    this.#closing = true;
    this.#program?.return(undefined);
    this.#program = null;
    if (this.#result?.ok) {
      this.#table(this.#result.state.nodes);
      this.#table(this.#result.touched);
    } else if (this.#result?.rejection.type === "invariantViolated")
      this.#table(this.#result.rejection.violations);
    this.#result = null;
    this.source = null;
    this.patch = null;
  }
  closeStep(grant) {
    if (!this.#closing)
      throw new Error("Retained UI close has not begun");
    if (!granted2(grant))
      return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    if (closeRetainedUiGraphFrame(this.#graph))
      return { kind: "pending", phase: "retirement", items: 1, bytes: 48 };
    if (this.#remove) {
      const cell = this.#remove;
      this.#remove = cell.next;
      cell.next = null;
      cell.value = null;
      return { kind: "pending", phase: "retirement", items: 1, bytes: 32 };
    }
    if (this.#resources) {
      const cell = this.#resources;
      const step2 = cell.value.closeStep(grant);
      if (step2.complete) {
        this.#resources = cell.next;
        cell.next = null;
        cell.value = null;
      }
      return { kind: "pending", phase: "retirement", items: 1, bytes: step2.bytes };
    }
    this.#closed = true;
    return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
  }
  terminalIsEmpty() {
    return this.#closed && !this.#resources && !this.#program && !this.#graph.stack && !this.#remove && !this.#result;
  }
}

class RetainedUiSnapshotCursor {
  #grant = { maxItems: 0, maxBytes: 0 };
  #table;
  #program = null;
  #offset = 0;
  #closing = false;
  #closed = false;
  snapshot;
  constructor(snapshot) {
    this.snapshot = snapshot;
    this.#table = new Table(NumericIndex.empty(), () => this.#grant);
  }
  advance(grant) {
    if (this.#closed)
      return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#closing)
      throw new Error("Snapshot hydration is closing");
    if (!granted2(grant))
      return { kind: "blocked", phase: "hydration", items: 0, bytes: 0 };
    this.#grant = grant;
    if (!this.#program) {
      if (this.#offset === this.snapshot.nodes.length)
        return { kind: "ready", phase: "hydration", items: 0, bytes: 0 };
      const record = this.snapshot.nodes[this.#offset++];
      this.#program = this.#table.set(record.id, record);
    }
    const step2 = this.#program.next();
    if (step2.done)
      this.#program = null;
    return { kind: "pending", phase: "hydration", items: 1, bytes: step2.done ? 16 : step2.value };
  }
  takeResult() {
    if (this.#closed || this.#closing || this.#program || this.#offset !== this.snapshot.nodes.length)
      return null;
    const snapshot = this.snapshot;
    const nodes = this.#table.take();
    this.snapshot = null;
    this.#closed = true;
    return { surface: snapshot.surface, revision: snapshot.revision, root: snapshot.root, nodes };
  }
  beginClose() {
    this.#closing = true;
    this.#program?.return();
    this.#program = null;
    this.snapshot = null;
  }
  closeStep(grant) {
    if (!this.#closing)
      throw new Error("Snapshot close has not begun");
    if (!granted2(grant))
      return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    const step2 = this.#table.closeStep(grant);
    if (step2.complete)
      this.#closed = true;
    return { kind: step2.complete ? "complete" : "pending", phase: "retirement", items: step2.complete ? 0 : 1, bytes: step2.bytes };
  }
  terminalIsEmpty() {
    return this.#closed && this.snapshot === null && this.#program === null;
  }
}
var publishPrepared;
var beginTransaction;
var acceptRoot;

class RetainedUiSurfaceOwner {
  #current;
  identity;
  #limits;
  static {
    acceptRoot = (owner, source, candidate) => {
      if (owner.#current !== source)
        return false;
      owner.#current = candidate;
      return true;
    };
  }
  constructor(actor, instance, initial, limits) {
    if (!Number.isSafeInteger(instance) || instance < 0 || instance > 4294967295)
      throw new RangeError("Retained UI instance is not a u32");
    this.identity = Object.freeze({ actor, instance, surface: initial.surface });
    this.#current = Object.freeze({ surface: initial.surface, revision: initial.revision, root: initial.root, nodes: initial.nodes });
    this.#limits = copyLimits(limits);
  }
  capture() {
    const current = this.#current;
    if (!current)
      throw new Error("Retained UI surface is closed");
    return { surface: current.surface, revision: current.revision, root: current.root, nodes: current.nodes.capture() };
  }
  get revision() {
    if (!this.#current)
      throw new Error("Retained UI surface is closed");
    return this.#current.revision;
  }
  getNode(id) {
    if (!this.#current)
      throw new Error("Retained UI surface is closed");
    return this.#current.nodes.get(id);
  }
  beginPatch(patch) {
    if (!this.#current)
      throw new Error("Retained UI surface is closed");
    return beginTransaction(this, this.#current, patch, this.#limits);
  }
  publish(transaction) {
    return publishPrepared(this, transaction);
  }
  beginClose() {
    if (!this.#current)
      throw new Error("Retained UI surface is already closed");
    const retirement = this.#current.nodes.beginClose();
    this.#current = null;
    return retirement;
  }
  terminalIsEmpty() {
    return this.#current === null;
  }
}

class RetainedUiTransaction {
  #owner;
  #source;
  #job;
  #patch;
  #identityOffset = 0;
  #identityReady = false;
  #candidate = null;
  #rejection = null;
  #ack = null;
  #nodesRetirement = null;
  #touchedRetirement = null;
  #violationRetirement = null;
  #previousRetirement = null;
  #status = "pending";
  static {
    beginTransaction = (owner, source, patch, limits) => new RetainedUiTransaction(owner, source, patch, limits);
    publishPrepared = (owner, transaction) => transaction.#publish(owner);
  }
  constructor(owner, source, patch, limits) {
    this.#owner = owner;
    this.#source = source;
    this.#patch = patch;
    this.#job = new RetainedUiPatchCursor(source, patch, limits);
  }
  advance(grant) {
    if (this.#status === "closed")
      return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#status === "closing")
      throw new Error("Retained transaction is closing");
    if (this.#status !== "pending")
      return { kind: this.#status === "rejected" ? "rejected" : "ready", phase: this.#status, items: 0, bytes: 0 };
    if (!granted2(grant))
      return { kind: "blocked", phase: "identity", items: 0, bytes: 0 };
    if (!this.#identityReady) {
      const expected = this.#owner.identity.surface;
      const actual = this.#patch.surface;
      if (expected.length !== actual.length) {
        this.#status = "rejected";
        return { kind: "rejected", phase: "identity", items: 1, bytes: 16 };
      }
      let bytes = 0;
      while (this.#identityOffset < expected.length && bytes + 8 <= grant.maxBytes) {
        const offset = this.#identityOffset++;
        bytes += 8;
        if (expected.charCodeAt(offset) !== actual.charCodeAt(offset)) {
          this.#status = "rejected";
          return { kind: "rejected", phase: "identity", items: 1, bytes };
        }
      }
      this.#identityReady = this.#identityOffset === expected.length;
      return { kind: "pending", phase: "identity", items: 1, bytes };
    }
    const step2 = this.#job.advance(grant);
    if (step2.kind === "ready" || step2.kind === "rejected") {
      const result = this.#job.takeResult();
      if (result.ok) {
        this.#candidate = result;
        this.#status = "ready";
      } else {
        this.#rejection = result.rejection;
        this.#status = "rejected";
      }
    }
    return step2;
  }
  #publish(owner) {
    if (this.#status !== "ready" || owner !== this.#owner || !this.#candidate || !this.#source)
      return false;
    if (!acceptRoot(owner, this.#source, this.#candidate.state)) {
      this.#status = "rejected";
      return false;
    }
    this.#previousRetirement = this.#source.nodes.beginClose();
    this.#touchedRetirement = this.#candidate.touched.beginClose();
    this.#ack = { identity: owner.identity, revision: this.#candidate.state.revision };
    this.#candidate = null;
    this.#status = "published";
    return true;
  }
  takeAcknowledgement() {
    const ack = this.#ack;
    this.#ack = null;
    return ack;
  }
  beginClose() {
    if (this.#status === "closing" || this.#status === "closed")
      return;
    this.#job.beginClose();
    if (this.#candidate) {
      this.#nodesRetirement = this.#candidate.state.nodes.beginClose();
      this.#touchedRetirement = this.#candidate.touched.beginClose();
      this.#candidate = null;
    }
    if (this.#rejection?.type === "invariantViolated")
      this.#violationRetirement = this.#rejection.violations.beginClose();
    this.#rejection = null;
    this.#ack = null;
    this.#owner = null;
    this.#source = null;
    this.#patch = null;
    this.#status = "closing";
  }
  closeStep(grant) {
    if (this.#status === "closed")
      return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#status !== "closing")
      throw new Error("Retained transaction close has not begun");
    if (!granted2(grant))
      return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    if (this.#job) {
      const step2 = this.#job.closeStep(grant);
      if (step2.kind === "complete")
        this.#job = null;
      return { ...step2, kind: "pending" };
    }
    const advance = (cursor) => cursor.advance(grant);
    if (this.#nodesRetirement) {
      const step2 = advance(this.#nodesRetirement);
      if (step2.kind === "complete")
        this.#nodesRetirement = null;
      return { kind: "pending", phase: "retirement", items: step2.items, bytes: step2.bytes };
    }
    if (this.#touchedRetirement) {
      const step2 = advance(this.#touchedRetirement);
      if (step2.kind === "complete")
        this.#touchedRetirement = null;
      return { kind: "pending", phase: "retirement", items: step2.items, bytes: step2.bytes };
    }
    if (this.#violationRetirement) {
      const step2 = advance(this.#violationRetirement);
      if (step2.kind === "complete")
        this.#violationRetirement = null;
      return { kind: "pending", phase: "retirement", items: step2.items, bytes: step2.bytes };
    }
    if (this.#previousRetirement) {
      const step2 = advance(this.#previousRetirement);
      if (step2.kind === "complete")
        this.#previousRetirement = null;
      return { kind: "pending", phase: "retirement", items: step2.items, bytes: step2.bytes };
    }
    this.#status = "closed";
    return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
  }
  terminalIsEmpty() {
    return this.#status === "closed" && !this.#job && !this.#nodesRetirement && !this.#touchedRetirement && !this.#violationRetirement && !this.#previousRetirement;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️.ts */
var admitted3 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var state = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var adoptIndex;
var adoptRetirement;
var adoptEdit;
var adoptRead;

class OwnedUiNodeIndex {
  #index;
  constructor(index) {
    this.#index = index;
    Object.freeze(this);
  }
  static {
    adoptIndex = (index) => new OwnedUiNodeIndex(index);
  }
  static empty() {
    return adoptIndex(NumericIndex.empty());
  }
  #live() {
    if (!this.#index)
      throw new Error("Owned UI node index is closed");
    return this.#index;
  }
  get size() {
    return this.#live().size;
  }
  capture() {
    return adoptIndex(this.#live().capture());
  }
  beginSet(node) {
    const index = this.#live();
    const id = node.value.id;
    index.assertCaptureCapacity();
    return adoptEdit(index.beginSet(id, node.capture()));
  }
  beginRemove(id) {
    return adoptEdit(this.#live().beginRemove(id));
  }
  beginRead() {
    return adoptRead(this.#live().beginRead());
  }
  beginSortedRead() {
    return adoptRead(this.#live().beginSortedRead());
  }
  beginLookup(id) {
    return adoptRead(this.#live().beginLookup(id));
  }
  beginClose() {
    const index = this.#live();
    this.#index = null;
    return adoptRetirement(index.beginClose());
  }
  terminalIsEmpty() {
    return this.#index === null;
  }
}

class OwnedUiNodeIndexRetirement {
  #index;
  #node;
  constructor(index, node) {
    this.#index = index;
    this.#node = node;
    Object.freeze(this);
  }
  static {
    adoptRetirement = (index, node = null) => new OwnedUiNodeIndexRetirement(index, node);
  }
  advance(grant) {
    if (!admitted3(grant))
      return state("blocked", "node-index-close");
    if (this.#node) {
      const result = this.#node.advance(grant);
      if (result.kind === "complete")
        this.#node = null;
      return { ...result, kind: "pending" };
    }
    if (this.#index) {
      const result = this.#index.advance(grant);
      if (result.kind === "retired") {
        this.#node = result.value.beginClose();
        return state("pending", "node-index-entry-close", result.bytes + 64);
      }
      if (result.kind === "complete")
        this.#index = null;
      return { kind: "pending", phase: "node-index-close", items: result.items, bytes: result.bytes };
    }
    return state("complete", "node-index-close");
  }
  terminalIsEmpty() {
    return this.#index === null && this.#node === null;
  }
}

class OwnedUiNodeIndexEdit {
  #edit;
  #node = null;
  #failure = null;
  constructor(edit) {
    this.#edit = edit;
    Object.freeze(this);
  }
  static {
    adoptEdit = (edit) => new OwnedUiNodeIndexEdit(edit);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted3(grant))
      return state("blocked", "node-index-edit");
    if (!this.#edit)
      throw new Error("Owned UI node edit is closed");
    if (this.#node) {
      const result2 = this.#node.advance(grant);
      if (result2.kind === "complete")
        this.#node = null;
      return { ...result2, kind: "pending" };
    }
    const result = this.#edit.advance(grant);
    if (result.kind === "retired") {
      this.#node = result.value.beginClose();
      return state("pending", "node-index-entry-close", result.bytes + 64);
    }
    if (result.kind === "rejected")
      this.#failure = result.reason;
    return { kind: result.kind, phase: "node-index-edit", items: result.items, bytes: result.bytes };
  }
  takeResult() {
    if (!this.#edit || this.#node)
      return null;
    const result = this.#edit.takeResult();
    return result ? adoptIndex(result) : null;
  }
  beginClose() {
    if (!this.#edit)
      throw new Error("Owned UI node edit is already closed");
    const result = adoptRetirement(this.#edit.beginClose(), this.#node);
    this.#edit = null;
    this.#node = null;
    return result;
  }
  terminalIsEmpty() {
    return this.#edit === null && this.#node === null;
  }
}

class OwnedUiNodeIndexReader {
  #reader;
  constructor(reader) {
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    adoptRead = (reader) => new OwnedUiNodeIndexReader(reader);
  }
  advance(grant) {
    if (!admitted3(grant))
      return state("blocked", "node-index-read");
    if (!this.#reader)
      throw new Error("Owned UI node reader is closed");
    const result = this.#reader.advance(grant);
    if (result.kind === "value")
      return { ...result, value: result.value.capture(), bytes: result.bytes + 64 };
    return { ...result, phase: "node-index-read" };
  }
  beginClose() {
    if (!this.#reader)
      throw new Error("Owned UI node reader is already closed");
    const result = adoptRetirement(this.#reader.beginClose());
    this.#reader = null;
    return result;
  }
  terminalIsEmpty() {
    return this.#reader === null;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️.ts */
var PAGE_BYTES = 256;
var MAXIMUM_BYTES = 32768;
var admitted4 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step2 = (kind, bytes = 0, accepted = false) => ({ kind, accepted, items: bytes ? 1 : 0, bytes });
var own;
var retire;

class UiSurfaceBytes {
  #root;
  constructor(root) {
    this.#root = root;
  }
  static {
    own = (root) => new UiSurfaceBytes(root);
  }
  get length() {
    if (!this.#root)
      throw new Error("Surface byte owner is closed");
    return this.#root.length;
  }
  byteAt(index) {
    if (!this.#root)
      throw new Error("Surface byte owner is closed");
    if (!Number.isSafeInteger(index) || index < 0 || index >= this.#root.length)
      throw new RangeError("Surface byte index is outside the exact range");
    return this.#root.pages[Math.floor(index / PAGE_BYTES)][index % PAGE_BYTES];
  }
  capture() {
    if (!this.#root)
      throw new Error("Surface byte owner is closed");
    if (this.#root.references === Number.MAX_SAFE_INTEGER)
      throw new RangeError("Surface byte owner reference overflow");
    this.#root.references++;
    return own(this.#root);
  }
  beginClose() {
    const root = this.#root;
    this.#root = null;
    return retire(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class UiSurfaceByteBuilder {
  #root = null;
  #written = 0;
  #taken = false;
  #closing = false;
  #failed = false;
  #length;
  constructor(length) {
    if (!Number.isSafeInteger(length) || length < 0 || length > MAXIMUM_BYTES)
      throw new RangeError("Surface byte length exceeds the native envelope");
    this.#length = length;
  }
  advance(grant, value) {
    if (!admitted4(grant))
      return step2("blocked");
    if (this.#closing || this.#taken || this.#failed)
      return step2("rejected");
    if (value !== undefined && (!Number.isInteger(value) || value < 0 || value > 255)) {
      this.#failed = true;
      return step2("rejected", 16);
    }
    if (!this.#root) {
      const count = Math.ceil(this.#length / PAGE_BYTES);
      this.#root = { length: this.#length, pages: new Array(count).fill(null), references: 1 };
      return step2(this.#length ? "pending" : "ready", 64 + count * 8);
    }
    if (this.#written === this.#length)
      return step2("ready");
    if (value === undefined)
      return step2("blocked");
    const index = Math.floor(this.#written / PAGE_BYTES);
    if (!this.#root.pages[index]) {
      const length = Math.min(PAGE_BYTES, this.#length - index * PAGE_BYTES);
      this.#root.pages[index] = new Uint8Array(length);
      return step2("pending", length + 16);
    }
    this.#root.pages[index][this.#written % PAGE_BYTES] = value;
    this.#written++;
    return step2(this.#written === this.#length ? "ready" : "pending", 1, true);
  }
  takeResult() {
    if (!this.#root || this.#written !== this.#length || this.#closing || this.#failed || this.#taken)
      return null;
    const root = this.#root;
    this.#root = null;
    this.#taken = true;
    return own(root);
  }
  beginClose() {
    this.#closing = true;
    const root = this.#root;
    this.#root = null;
    return retire(root);
  }
  terminalIsEmpty() {
    return (this.#taken || this.#closing) && this.#root === null;
  }
}

class UiSurfaceByteRetirement {
  #root;
  #released = false;
  #page = 0;
  #offset = 0;
  constructor(root) {
    this.#root = root;
  }
  static {
    retire = (root) => new UiSurfaceByteRetirement(root);
  }
  advance(grant) {
    if (!admitted4(grant))
      return step2("blocked");
    if (!this.#root)
      return step2("complete");
    if (!this.#released) {
      this.#root.references--;
      this.#released = true;
      if (this.#root.references > 0)
        this.#root = null;
      return step2(this.#root ? "pending" : "complete", 32);
    }
    if (this.#page < this.#root.pages.length) {
      const page = this.#root.pages[this.#page];
      if (!page) {
        this.#page++;
        return step2("pending", 8);
      }
      const end = Math.min(page.length, this.#offset + grant.maxBytes - 16);
      const bytes2 = end - this.#offset;
      page.fill(0, this.#offset, end);
      this.#offset = end;
      if (end === page.length) {
        this.#root.pages[this.#page++] = null;
        this.#offset = 0;
      }
      return step2("pending", bytes2 + 16);
    }
    const bytes = this.#root.pages.length * 8 + 32;
    this.#root.pages = [];
    this.#root = null;
    return step2("complete", bytes);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔒️transport/🟦️.ts */
function getter(prototype, key) {
  const read = Object.getOwnPropertyDescriptor(prototype, key)?.get;
  if (!read)
    throw new Error("Required native buffer intrinsic is unavailable");
  return (owner) => Reflect.apply(read, owner, []);
}
var viewPrototype = Object.getPrototypeOf(Uint8Array.prototype);
var viewKind = getter(viewPrototype, Symbol.toStringTag);
var viewBuffer = getter(viewPrototype, "buffer");
var viewOffset = getter(viewPrototype, "byteOffset");
var viewBytes = getter(viewPrototype, "byteLength");
var bufferBytes = getter(ArrayBuffer.prototype, "byteLength");
function ordinaryBuffer(value) {
  try {
    return typeof bufferBytes(value) === "number";
  } catch {
    return false;
  }
}
function takeOwnedNativeBuffer(input, kind, maximumBytes) {
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || viewKind(input) !== kind)
    throw new Error("Invalid native buffer admission");
  const buffer = viewBuffer(input);
  const bytes = viewBytes(input);
  if (!ordinaryBuffer(buffer) || viewOffset(input) !== 0 || typeof bytes !== "number" || bytes !== bufferBytes(buffer) || bytes > maximumBytes)
    throw new Error("Native ownership requires an entire non-shared admitted buffer");
  try {
    new Uint8Array(buffer);
    const moved = structuredClone(buffer, { transfer: [buffer] });
    if (!ordinaryBuffer(moved) || bufferBytes(buffer) !== 0 || bufferBytes(moved) !== bytes)
      throw new Error("Native buffer transfer did not preserve exact ownership");
    return moved;
  } catch {
    throw new Error("Native buffer ownership transfer failed");
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️.ts */
var TEXT_BYTES = 512;
var COLLECTION_ITEMS = 256;
var MINIMUM_GRANT = 4096;
var admitted5 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= MINIMUM_GRANT;
var result = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function compareBytes(left, right) {
  const count = Math.min(left.length, right.length);
  for (let i = 0;i < count; i++)
    if (left[i] !== right[i])
      return left[i] - right[i];
  return left.length - right.length;
}

class RetainedUiWireValueCursor {
  #input;
  #offset = 0;
  #phase = "symbol-count";
  #failure = null;
  #closing = false;
  #ready = false;
  #root;
  #pending;
  #frame = null;
  #head = null;
  #tail = null;
  #symbols = NumericIndex.empty();
  #symbolEdit = null;
  #symbolReader = null;
  #retirement = null;
  #oldSymbols = null;
  #symbolCount = 0;
  #symbolIndex = 0;
  #previousSymbol = null;
  #text = "";
  #textKind = "symbol";
  #textLength = 0;
  #number = 0;
  #natural = 0;
  #multiplier = 1;
  #naturalBytes = 0;
  #array = false;
  #closeOffset = 0;
  #byteRetirement = null;
  #profile;
  constructor(input, profile = "value") {
    this.#profile = profile;
    this.#input = new Uint8Array(takeOwnedNativeBuffer(input, "Uint8Array", Number.MAX_SAFE_INTEGER));
  }
  get value() {
    return this.#ready && !this.#closing ? this.#root : undefined;
  }
  get failure() {
    return this.#failure;
  }
  #byte() {
    if (!this.#input || this.#offset >= this.#input.length)
      throw new Error("Truncated UI wire value");
    return this.#input[this.#offset++];
  }
  #nat(next) {
    const byte = this.#byte();
    const digit = byte & 127;
    const value = this.#natural + digit * this.#multiplier;
    if (!Number.isSafeInteger(value) || this.#naturalBytes >= 8)
      throw new Error("UI wire integer exceeds the exact range");
    this.#natural = value;
    this.#naturalBytes++;
    if (byte < 128) {
      if (this.#naturalBytes > 1 && digit === 0)
        throw new Error("Noncanonical UI wire integer");
      this.#number = value;
      this.#natural = 0;
      this.#multiplier = 1;
      this.#naturalBytes = 0;
      this.#phase = next;
    } else
      this.#multiplier *= 128;
    return 1;
  }
  #own(value) {
    const owner = { value, next: null };
    if (this.#tail)
      this.#tail.next = owner;
    else
      this.#head = owner;
    this.#tail = owner;
    return owner;
  }
  #surfaceBytesPath() {
    const document = this.#frame;
    const component = document?.parent;
    if (!this.#array || document?.key !== "bytes" || component?.key !== "doc")
      return false;
    if (this.#profile === "component")
      return component.parent === null;
    return this.#profile === "node" && component.parent?.key === "component" && component.parent.parent === null;
  }
  #finishBytes() {
    const frame = this.#frame;
    const value = frame.bytes.takeResult();
    if (!value)
      throw new Error("Surface byte preparation is incomplete");
    frame.bytes = null;
    frame.owner.value = value;
    this.#pending = value;
    this.#frame = frame.parent;
    frame.parent = null;
    this.#phase = "attach";
  }
  #advance(grant) {
    switch (this.#phase) {
      case "symbol-count":
        return this.#nat("symbol-count-done");
      case "symbol-count-done":
        if (this.#number > this.#input.length - this.#offset)
          throw new Error("Impossible UI symbol count");
        this.#symbolCount = this.#number;
        this.#textKind = "symbol";
        this.#phase = this.#symbolCount ? "text-length" : "field-count";
        return 32;
      case "text-length":
        return this.#nat("text-length-done");
      case "text-length-done":
        if (this.#number > TEXT_BYTES || this.#number > this.#input.length - this.#offset)
          throw new Error("UI text exceeds native bounds or input");
        this.#textLength = this.#number;
        this.#phase = "text-body";
        return 16;
      case "text-body": {
        const bytes = this.#input.subarray(this.#offset, this.#offset + this.#textLength);
        this.#stepBytes = 4 * bytes.length + 64;
        const text = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes);
        this.#offset += bytes.length;
        if (this.#textKind === "symbol") {
          if (this.#previousSymbol && compareBytes(this.#previousSymbol, bytes) >= 0)
            throw new Error("UI symbols must be strictly ordered");
          this.#previousSymbol = bytes;
          this.#text = text;
          this.#phase = "symbol-store";
        } else if (this.#textKind === "key") {
          const frame = this.#frame;
          if (frame.previousKey && compareBytes(frame.previousKey, bytes) >= 0)
            throw new Error("UI map keys must be strictly ordered");
          frame.previousKey = bytes;
          frame.key = text;
          this.#phase = "value-tag";
        } else {
          this.#pending = text;
          this.#phase = "attach";
        }
        return 4 * bytes.length + 64;
      }
      case "symbol-store":
        this.#symbolEdit = this.#symbols.beginSet(this.#symbolIndex, this.#text);
        this.#text = "";
        this.#phase = "symbol-edit";
        return 128;
      case "symbol-edit": {
        const step3 = this.#symbolEdit.advance(grant);
        if (step3.kind === "rejected")
          throw new Error("UI symbol ordinal exhausted");
        if (step3.kind === "ready") {
          this.#oldSymbols = this.#symbols;
          this.#symbols = this.#symbolEdit.takeResult();
          this.#retirement = this.#symbolEdit.beginClose();
          this.#symbolEdit = null;
          this.#phase = "symbol-edit-close";
        }
        return step3.bytes;
      }
      case "symbol-edit-close": {
        const step3 = this.#retirement.advance(grant);
        if (step3.kind === "complete") {
          this.#retirement = this.#oldSymbols.beginClose();
          this.#oldSymbols = null;
          this.#phase = "symbol-old-close";
        }
        return step3.bytes;
      }
      case "symbol-old-close": {
        const step3 = this.#retirement.advance(grant);
        if (step3.kind === "complete") {
          this.#retirement = null;
          this.#symbolIndex++;
          this.#phase = this.#symbolIndex < this.#symbolCount ? "text-length" : "field-count";
        }
        return step3.bytes;
      }
      case "field-count":
        return this.#nat("field-count-done");
      case "field-count-done":
        if (this.#number !== 1)
          throw new Error("UI wire requires one bridge field");
        this.#phase = "field-id";
        return 8;
      case "field-id":
        return this.#nat("field-id-done");
      case "field-id-done":
        if (this.#number !== 1)
          throw new Error("UI wire bridge field identity differs");
        this.#phase = "outer-tag";
        return 8;
      case "outer-tag":
        if (this.#byte() !== 17)
          throw new Error("UI wire bridge tag differs");
        this.#phase = "value-tag";
        return 1;
      case "value-tag": {
        const tag = this.#byte();
        if (tag === 18 || tag === 1 || tag === 2) {
          this.#pending = tag === 18 ? null : tag === 2;
          this.#phase = "attach";
        } else if (tag === 5)
          this.#phase = "float";
        else if (tag === 6)
          this.#phase = "symbol-reference";
        else if (tag === 7) {
          this.#textKind = "value";
          this.#phase = "text-length";
        } else if (tag === 12 || tag === 16) {
          this.#array = tag === 12;
          this.#phase = "collection-count";
        } else
          throw new Error("Unknown UI value tag");
        return 1;
      }
      case "float": {
        this.#stepBytes = 16;
        if (this.#input.length - this.#offset < 8)
          throw new Error("Truncated UI number");
        const value = new DataView(this.#input.buffer, this.#offset, 8).getFloat64(0, true);
        if (!Number.isFinite(value) || Object.is(value, -0))
          throw new Error("Noncanonical UI number");
        this.#offset += 8;
        this.#pending = value;
        this.#phase = "attach";
        return 16;
      }
      case "symbol-reference":
        return this.#nat("symbol-reference-done");
      case "symbol-reference-done":
        if (this.#number >= this.#symbolCount)
          throw new Error("UI symbol reference exceeds table");
        this.#symbolReader = this.#symbols.beginLookup(this.#number);
        this.#phase = "symbol-lookup";
        return 64;
      case "symbol-lookup": {
        const step3 = this.#symbolReader.advance(grant);
        if (step3.kind === "value")
          this.#pending = step3.value;
        if (step3.kind === "complete") {
          this.#retirement = this.#symbolReader.beginClose();
          this.#symbolReader = null;
          this.#phase = "symbol-lookup-close";
        }
        return step3.bytes;
      }
      case "symbol-lookup-close": {
        const step3 = this.#retirement.advance(grant);
        if (step3.kind === "complete") {
          this.#retirement = null;
          this.#phase = "attach";
        }
        return step3.bytes;
      }
      case "collection-count":
        return this.#nat("collection-create");
      case "collection-create": {
        if (this.#surfaceBytesPath()) {
          const bytes = new UiSurfaceByteBuilder(this.#number);
          this.#frame = { owner: this.#own(null), count: this.#number, index: 0, key: null, previousKey: null, array: true, bytes, parent: this.#frame };
          this.#phase = "surface-bytes-reserve";
          return 128;
        }
        if (this.#number > COLLECTION_ITEMS || this.#number > this.#input.length - this.#offset)
          throw new Error("UI collection exceeds native bounds or input");
        const owner = this.#own(this.#array ? new Array(this.#number) : {});
        this.#frame = { owner, count: this.#number, index: 0, key: null, previousKey: null, array: this.#array, bytes: null, parent: this.#frame };
        this.#phase = this.#number ? this.#array ? "value-tag" : "map-key-tag" : "collection-finish";
        return 128 + this.#number * 8;
      }
      case "surface-bytes-reserve": {
        const step3 = this.#frame.bytes.advance(grant);
        if (step3.kind === "ready")
          this.#finishBytes();
        else
          this.#phase = "value-tag";
        return step3.bytes + 64;
      }
      case "map-key-tag":
        if (this.#byte() !== 7)
          throw new Error("UI map keys must use inline canonical text");
        this.#textKind = "key";
        this.#phase = "text-length";
        return 1;
      case "attach": {
        if (this.#pending === undefined)
          throw new Error("Missing decoded UI value");
        const frame = this.#frame;
        if (!frame) {
          this.#root = this.#pending;
          this.#pending = undefined;
          this.#phase = "finish";
          return 32;
        }
        if (frame.bytes) {
          if (typeof this.#pending !== "number")
            throw new Error("Surface bytes require unsigned byte values");
          const step3 = frame.bytes.advance(grant, this.#pending);
          if (step3.kind === "rejected")
            throw new Error("Surface byte value is outside its native range");
          if (step3.accepted) {
            this.#pending = undefined;
            frame.index++;
            this.#phase = "value-tag";
          }
          if (step3.kind === "ready")
            this.#finishBytes();
          return step3.bytes + 64;
        }
        const key = frame.array ? String(frame.index) : frame.key;
        this.#stepBytes = 128 + frame.count * 8 + key.length * 2;
        Object.defineProperty(frame.owner.value, key, { value: this.#pending, enumerable: true, configurable: false, writable: false });
        this.#pending = undefined;
        frame.key = null;
        frame.index++;
        this.#phase = frame.index === frame.count ? "collection-finish" : frame.array ? "value-tag" : "map-key-tag";
        return 128 + frame.count * 8 + key.length * 2;
      }
      case "collection-finish": {
        const frame = this.#frame;
        if (frame.array)
          Object.defineProperty(frame.owner.value, "length", { writable: false });
        Object.preventExtensions(frame.owner.value);
        this.#pending = frame.owner.value;
        this.#frame = frame.parent;
        frame.parent = null;
        frame.previousKey = null;
        this.#phase = "attach";
        return 64 + frame.count * 8;
      }
      case "finish":
        if (this.#offset !== this.#input.length)
          throw new Error("Trailing UI wire bytes");
        this.#ready = true;
        return 16;
      default:
        throw new Error("Invalid UI decoder phase");
    }
  }
  advance(grant) {
    if (!admitted5(grant))
      return result("blocked", this.#phase);
    if (this.#closing)
      return result("rejected", "closing");
    if (this.#failure)
      return result("rejected", this.#phase);
    if (this.#ready)
      return result("ready", this.#phase);
    const phase = this.#phase;
    this.#stepBytes = 16;
    try {
      const bytes = this.#advance(grant);
      return result(this.#ready ? "ready" : "pending", phase, bytes);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "UI wire decoding failed";
      return result("rejected", phase, this.#stepBytes);
    }
  }
  #stepBytes = 0;
  beginClose() {
    this.#closing = true;
    this.#ready = false;
  }
  closeStep(grant) {
    if (!admitted5(grant))
      return result("blocked", "close");
    if (!this.#closing)
      throw new Error("UI wire retirement was not started");
    if (this.#byteRetirement) {
      const step3 = this.#byteRetirement.advance(grant);
      if (step3.kind === "complete")
        this.#byteRetirement = null;
      return result("pending", "close-surface-bytes", step3.bytes);
    }
    if (this.#frame?.bytes) {
      this.#byteRetirement = this.#frame.bytes.beginClose();
      this.#frame.bytes = null;
      return result("pending", "close-byte-builder", 64);
    }
    if (this.#frame) {
      const frame = this.#frame;
      this.#frame = frame.parent;
      frame.parent = null;
      frame.previousKey = null;
      frame.key = null;
      return result("pending", "close-frame", 128);
    }
    if (this.#root !== undefined || this.#pending !== undefined) {
      this.#root = undefined;
      this.#pending = undefined;
      return result("pending", "close-root", 64);
    }
    if (this.#head) {
      const owner = this.#head;
      this.#head = owner.next;
      owner.next = null;
      if (owner.value instanceof UiSurfaceBytes)
        this.#byteRetirement = owner.value.beginClose();
      owner.value = null;
      if (!this.#head)
        this.#tail = null;
      return result("pending", "close-owner", 64 + COLLECTION_ITEMS * 8);
    }
    if (this.#retirement) {
      const step3 = this.#retirement.advance(grant);
      if (step3.kind === "complete")
        this.#retirement = null;
      return result("pending", "close-index", step3.bytes);
    }
    if (this.#symbolReader) {
      this.#retirement = this.#symbolReader.beginClose();
      this.#symbolReader = null;
      return result("pending", "close-reader", 64);
    }
    if (this.#symbolEdit) {
      this.#retirement = this.#symbolEdit.beginClose();
      this.#symbolEdit = null;
      return result("pending", "close-edit", 64);
    }
    if (this.#oldSymbols) {
      this.#retirement = this.#oldSymbols.beginClose();
      this.#oldSymbols = null;
      return result("pending", "close-old", 64);
    }
    if (this.#symbols) {
      this.#retirement = this.#symbols.beginClose();
      this.#symbols = null;
      return result("pending", "close-symbols", 64);
    }
    if (this.#input) {
      this.#text = "";
      this.#previousSymbol = null;
      const end = Math.min(this.#input.length, this.#closeOffset + MINIMUM_GRANT);
      const bytes = end - this.#closeOffset;
      this.#input.fill(0, this.#closeOffset, end);
      this.#closeOffset = end;
      if (end === this.#input.length)
        this.#input = null;
      return result("pending", "close-bytes", bytes);
    }
    return result("complete", "closed");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#input && !this.#frame && !this.#head && !this.#tail && !this.#symbols && !this.#symbolReader && !this.#symbolEdit && !this.#retirement && !this.#byteRetirement && !this.#oldSymbols && this.#root === undefined && this.#pending === undefined && !this.#text && !this.#previousSymbol;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts */
var OWNER_MINT = Object.freeze({});
var admitted6 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step3 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function isRecord(value) {
  return typeof value === "object" && value !== null && !Array.isArray(value) && Object.getPrototypeOf(value) === Object.prototype;
}
function text(value) {
  if (typeof value !== "string")
    throw new Error("Expected UI text");
  return value;
}
function boolean(value) {
  if (typeof value !== "boolean")
    throw new Error("Expected UI boolean");
  return value;
}
function number(value) {
  if (typeof value !== "number" || !Number.isFinite(value))
    throw new Error("Expected finite UI number");
  return value;
}
function natural(value, max = Number.MAX_SAFE_INTEGER) {
  const result2 = number(value);
  if (!Number.isSafeInteger(result2) || result2 < 0 || result2 > max)
    throw new Error("UI integer exceeds its exact domain");
  return result2 === 0 ? 0 : result2;
}
function optional(value, read) {
  return value == null ? null : read(value);
}
function choice(value, choices) {
  for (const candidate of choices)
    if (value === candidate)
      return candidate;
  throw new Error("Unknown UI schema discriminator");
}
function defaulted(value, fallback, read) {
  return value === undefined ? fallback : read(value);
}
var space = (value) => choice(value, ["none", "xs", "sm", "md", "lg", "xl", "xxl"]);
var activity = (value) => choice(value, ["waiting", "loading", "idle", "finished"]);

class ByteView {
  #source;
  constructor(source) {
    this.#source = source;
    Object.freeze(this);
  }
  get length() {
    return this.#source.length;
  }
  byteAt(index) {
    return this.#source.byteAt(index);
  }
}
var ownPayload;
var retirePayload;
var payloadFields;
var movedPayload;
var checkPayload;
var checkCapture;
var exactPayload;
var nodeFields;
class OwnedUiPayload {
  #root;
  constructor(mint, root) {
    if (mint !== OWNER_MINT)
      throw new Error("Typed payload requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    ownPayload = (root) => new OwnedUiPayload(OWNER_MINT, root);
    payloadFields = (payload) => {
      if (payload.#root?.kind !== "node" || !payload.#root.fields)
        throw new Error("Expected an exact typed node field owner");
      return payload.#root.fields;
    };
    checkPayload = (payload, kind) => {
      if (!payload.#root || payload.#root.kind !== kind)
        throw new Error("Typed UI payload field authority mismatch");
    };
    checkCapture = (payload, kind) => {
      checkPayload(payload, kind);
      if (payload.#root.references === Number.MAX_SAFE_INTEGER)
        throw new Error("Typed UI payload cannot be captured");
    };
    exactPayload = (payload) => {
      if (!payload.#root || payload.#root.value === undefined)
        throw new Error("Typed UI payload owner is closed");
      return payload.#root.value;
    };
    movedPayload = (payload, kind) => {
      if (!payload.#root || payload.#root.kind !== kind)
        throw new Error("Typed UI payload field authority mismatch");
      const root = payload.#root;
      payload.#root = null;
      return new OwnedUiPayload(OWNER_MINT, root);
    };
    if (undefined)
      ;
  }
  get value() {
    if (!this.#root || this.#root.value === undefined)
      throw new Error("Typed UI payload owner is closed");
    return this.#root.value;
  }
  capture() {
    if (!this.#root || this.#root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Typed UI payload cannot be captured");
    this.#root.references++;
    return ownPayload(this.#root);
  }
  beginClose() {
    if (!this.#root)
      throw new Error("Typed UI payload already closed");
    const root = this.#root;
    this.#root = null;
    return retirePayload(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class UiPayloadRetirement {
  #root;
  #released = false;
  #bytes = null;
  #child = null;
  constructor(root) {
    this.#root = root;
  }
  static {
    retirePayload = (root) => new UiPayloadRetirement(root);
  }
  advance(grant) {
    if (!admitted6(grant))
      return step3("blocked", "typed-retire");
    if (!this.#root)
      return step3("complete", "typed-retire");
    const root = this.#root;
    if (!this.#released) {
      this.#released = true;
      if (--root.references)
        this.#root = null;
      else {
        root.value = undefined;
        root.fields = null;
      }
      return step3("pending", "typed-release", 128);
    }
    if (root.owned) {
      const owned = root.owned;
      root.owned = owned.next;
      owned.value = null;
      owned.next = null;
      return step3("pending", "typed-object-retire", 2112);
    }
    if (this.#bytes) {
      const result2 = this.#bytes.advance(grant);
      if (result2.kind === "complete")
        this.#bytes = null;
      return { ...result2, kind: "pending", phase: "typed-bytes-retire" };
    }
    if (root.bytes) {
      const bytes = root.bytes;
      root.bytes = bytes.next;
      bytes.next = null;
      this.#bytes = bytes.value.beginClose();
      bytes.value = null;
      return step3("pending", "typed-bytes-retire", 64);
    }
    if (this.#child) {
      const result2 = this.#child.advance(grant);
      if (result2.kind === "complete")
        this.#child = null;
      return { ...result2, kind: "pending" };
    }
    if (root.children) {
      const child = root.children;
      root.children = child.next;
      child.next = null;
      this.#child = child.value.beginClose();
      child.value = null;
      return step3("pending", "typed-field-retire", 64);
    }
    this.#root = null;
    return step3("complete", "typed-retire");
  }
  terminalIsEmpty() {
    return this.#root === null && this.#bytes === null && this.#child === null;
  }
}
var retireNode;
function capturedChange(change) {
  if (!change)
    return null;
  const field = change.field;
  switch (field) {
    case "component":
      return { field, payload: change.payload };
    case "layout":
      return { field, payload: change.payload };
    case "style":
      return { field, payload: change.payload };
    case "accessibility":
      return { field, payload: change.payload };
    case "bindings":
      return { field, payload: change.payload };
    case "menu":
      return { field, payload: change.payload };
    case "children":
      return { field, payload: change.payload };
    default:
      throw new Error("Unknown typed UI field change");
  }
}
function captureTypedUiPayload(kind, payload) {
  checkPayload(payload, kind);
  return payload.capture();
}
function captureUiFieldChange(requested) {
  const change = capturedChange(requested);
  switch (change.field) {
    case "component":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "layout":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "style":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "accessibility":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "bindings":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "menu":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "children":
      return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
  }
}
function copyFields(fields, requested) {
  const change = capturedChange(requested);
  if (change)
    checkPayload(change.payload, change.field);
  if (change?.field !== "component")
    checkCapture(fields.component, "component");
  if (change?.field !== "layout")
    checkCapture(fields.layout, "layout");
  if (change?.field !== "style")
    checkCapture(fields.style, "style");
  if (change?.field !== "accessibility")
    checkCapture(fields.accessibility, "accessibility");
  if (change?.field !== "bindings")
    checkCapture(fields.bindings, "bindings");
  if (change?.field !== "menu")
    checkCapture(fields.menu, "menu");
  if (change?.field !== "children")
    checkCapture(fields.children, "children");
  return {
    component: change?.field === "component" ? movedPayload(change.payload, "component") : fields.component.capture(),
    layout: change?.field === "layout" ? movedPayload(change.payload, "layout") : fields.layout.capture(),
    style: change?.field === "style" ? movedPayload(change.payload, "style") : fields.style.capture(),
    accessibility: change?.field === "accessibility" ? movedPayload(change.payload, "accessibility") : fields.accessibility.capture(),
    bindings: change?.field === "bindings" ? movedPayload(change.payload, "bindings") : fields.bindings.capture(),
    menu: change?.field === "menu" ? movedPayload(change.payload, "menu") : fields.menu.capture(),
    children: change?.field === "children" ? movedPayload(change.payload, "children") : fields.children.capture()
  };
}
function nodeRoot(base, fields, state2) {
  return { references: 1, fields, value: Object.freeze({ id: base.id, key: base.key, component: fields.component.value, layout: fields.layout.value, style: fields.style.value, activity: state2?.activity ?? base.activity, disabled: state2?.disabled ?? base.disabled, transition: base.transition, accessibility: fields.accessibility.value, bindings: fields.bindings.value, menu: fields.menu.value, children: fields.children.value }) };
}

class OwnedUiNode {
  #root;
  constructor(mint, root) {
    if (mint !== OWNER_MINT)
      throw new Error("Typed node requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    nodeFields = (node) => {
      if (!node.#root?.fields)
        throw new Error("Retained UI node owner is closed");
      return node.#root.fields;
    };
  }
  static captureFrom(payload) {
    const fields = payloadFields(payload);
    return new OwnedUiNode(OWNER_MINT, nodeRoot(exactPayload(payload), copyFields(fields, null)));
  }
  #value() {
    if (!this.#root?.value)
      throw new Error("Retained UI node owner is closed");
    return this.#root.value;
  }
  get value() {
    return this.#value();
  }
  capture() {
    if (!this.#root || this.#root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Retained UI node cannot be captured");
    this.#root.references++;
    return new OwnedUiNode(OWNER_MINT, this.#root);
  }
  captureComponent() {
    return nodeFields(this).component.capture();
  }
  replace(change) {
    const value = this.#value();
    return new OwnedUiNode(OWNER_MINT, nodeRoot(value, copyFields(this.#root.fields, change)));
  }
  withActivity(payload) {
    checkPayload(payload, "activity");
    return new OwnedUiNode(OWNER_MINT, nodeRoot(this.#value(), copyFields(this.#root.fields, null), exactPayload(payload)));
  }
  beginClose() {
    if (!this.#root)
      throw new Error("Retained UI node already closed");
    const root = this.#root;
    this.#root = null;
    return retireNode(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class UiNodeRetirement {
  #root;
  #child = null;
  #released = false;
  #index = 0;
  constructor(root) {
    this.#root = root;
  }
  static {
    retireNode = (root) => new UiNodeRetirement(root);
  }
  advance(grant) {
    if (!admitted6(grant))
      return step3("blocked", "node-retire");
    if (!this.#root)
      return step3("complete", "node-retire");
    const root = this.#root;
    if (!this.#released) {
      this.#released = true;
      if (--root.references)
        this.#root = null;
      else
        root.value = null;
      return step3("pending", "node-release", 128);
    }
    if (this.#child) {
      const result2 = this.#child.advance(grant);
      if (result2.kind === "complete")
        this.#child = null;
      return { ...result2, kind: "pending" };
    }
    const fields = root.fields;
    const next = this.#index === 0 ? fields.component : this.#index === 1 ? fields.layout : this.#index === 2 ? fields.style : this.#index === 3 ? fields.accessibility : this.#index === 4 ? fields.bindings : this.#index === 5 ? fields.menu : this.#index === 6 ? fields.children : undefined;
    if (next) {
      this.#index++;
      this.#child = next.beginClose();
      return step3("pending", "node-field-retire", 64);
    }
    root.fields = null;
    this.#root = null;
    return step3("complete", "node-retire", 64);
  }
  terminalIsEmpty() {
    return this.#root === null && this.#child === null;
  }
}

class Builder {
  owned = null;
  bytes = null;
  json = null;
  children = null;
  fields = null;
  active = null;
  own(value) {
    this.owned = { value, next: this.owned };
    return value;
  }
  fixed(value) {
    return this.own(Object.freeze(value));
  }
  *field(kind, value) {
    const builder = new Builder;
    this.active = builder;
    yield 128;
    const result2 = yield* readers[kind](builder, value);
    const payload = ownPayload({ value: result2, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: null, kind });
    builder.owned = null;
    builder.bytes = null;
    builder.children = null;
    this.active = null;
    this.children = { value: payload, next: this.children };
    yield 128;
    return payload;
  }
  *record(value, fields) {
    yield 64;
    if (!isRecord(value))
      throw new Error("Expected a UI schema object");
    const keys = Object.keys(value);
    if (keys.length > 256)
      throw new Error("UI schema object exceeds native slots");
    yield 64 + keys.length * 8;
    for (const key of keys) {
      yield 512;
      if (!fields.includes(key))
        throw new Error(`Unknown UI field: ${key}`);
    }
    return value;
  }
  *list(value, read, maximum = 256) {
    yield 64;
    if (!Array.isArray(value) || value.length > maximum)
      throw new Error("UI list exceeds native slots or is missing");
    const input = value;
    const output = new Array(input.length);
    yield 64 + input.length * 8;
    for (let i = 0;i < input.length; i++) {
      const item = yield* read(input[i]);
      Object.defineProperty(output, i, { value: item, enumerable: true });
      yield 32;
    }
    Object.defineProperty(output, "length", { writable: false });
    Object.preventExtensions(output);
    return this.own(output);
  }
  *stringMap(value) {
    yield 64;
    if (!isRecord(value))
      throw new Error("Expected UI text map");
    const keys = Object.keys(value);
    if (keys.length > 256)
      throw new Error("UI text map exceeds native slots");
    yield 64 + keys.length * 8;
    const output = {};
    for (const key of keys) {
      Object.defineProperty(output, key, { value: text(value[key]), enumerable: true });
      yield 64;
    }
    Object.preventExtensions(output);
    return this.own(output);
  }
  *value(value) {
    let input = value;
    let output;
    for (;; ) {
      yield 64;
      if (input === null || typeof input === "boolean" || typeof input === "string")
        output = input;
      else if (typeof input === "number")
        output = number(input);
      else {
        if (!Array.isArray(input) && !isRecord(input))
          throw new Error("Invalid native UI value");
        const source = input;
        const keys = Array.isArray(source) ? null : isRecord(source) ? Object.keys(source) : null;
        const count = Array.isArray(source) ? source.length : keys.length;
        if (count > 256)
          throw new Error("UI value exceeds native slots");
        yield 64 + count * 8;
        if (Array.isArray(source))
          this.json = { input: source, output: new Array(count), keys: null, index: 0, count, parent: this.json };
        else if (isRecord(source))
          this.json = { input: source, output: {}, keys, index: 0, count, parent: this.json };
        if (count) {
          input = this.jsonInput(this.json);
          continue;
        }
        output = this.finishJson();
      }
      for (;; ) {
        yield 64;
        if (!this.json)
          return output;
        const frame = this.json;
        Object.defineProperty(frame.output, frame.keys ? frame.keys[frame.index] : frame.index, { value: output, enumerable: true });
        frame.index++;
        if (frame.index < frame.count) {
          input = this.jsonInput(frame);
          break;
        }
        output = this.finishJson();
      }
    }
  }
  jsonInput(frame) {
    return frame.keys === null ? frame.input[frame.index] : frame.input[frame.keys[frame.index]];
  }
  finishJson() {
    const frame = this.json;
    this.json = frame.parent;
    frame.parent = null;
    if (Array.isArray(frame.output))
      Object.defineProperty(frame.output, "length", { writable: false });
    Object.preventExtensions(frame.output);
    return this.own(frame.output);
  }
  *binding(value) {
    const v = yield* this.record(value, ["trigger", "action", "args", "capability"]);
    const action = yield* this.record(v.action, ["scope", "name", "version"]);
    const address = this.fixed({ scope: text(action.scope), name: text(action.name), version: natural(action.version, 65535) });
    yield 128;
    const args = v.args == null ? null : yield* this.value(v.args);
    yield 128;
    return this.fixed({ trigger: choice(v.trigger, ["activate", "change", "commit", "delta", "drop", "submit", "abort", "repeatLast", "hoverPreview"]), action: address, args, capability: optional(v.capability, text) });
  }
  *bindings(value) {
    return yield* this.list(value, (item) => this.binding(item), 32);
  }
  *menu(value) {
    yield 32;
    if (value === null)
      return null;
    const v = yield* this.record(value, ["id", "args"]);
    const args = v.args == null ? null : yield* this.value(v.args);
    yield 64;
    return this.fixed({ id: text(v.id), args });
  }
  *component(value) {
    yield 64;
    if (!isRecord(value))
      throw new Error("Expected UI component");
    switch (value.type) {
      case "container": {
        const v = yield* this.record(value, ["type", "role", "label", "description", "required", "error", "defaultOpen", "dropOverlay"]);
        let dropOverlay = null;
        if (v.dropOverlay != null) {
          const d = yield* this.record(v.dropOverlay, ["title", "hint", "accept"]);
          dropOverlay = this.fixed({ title: text(d.title), hint: text(d.hint), accept: optional(d.accept, text) });
          yield 128;
        }
        return this.fixed({ type: "container", role: defaulted(v.role, "plain", (v2) => choice(v2, ["plain", "section", "group", "field", "form", "toolbar"])), label: optional(v.label, text), description: optional(v.description, text), required: optional(v.required, boolean), error: optional(v.error, text), defaultOpen: optional(v.defaultOpen, boolean), dropOverlay });
      }
      case "text": {
        const v = yield* this.record(value, ["type", "value", "emphasize", "dataAttributes"]);
        const dataAttributes = v.dataAttributes == null ? null : yield* this.stringMap(v.dataAttributes);
        yield 128;
        return this.fixed({ type: "text", value: text(v.value), emphasize: optional(v.emphasize, boolean), dataAttributes });
      }
      case "button": {
        const v = yield* this.record(value, ["type", "icon", "label"]);
        return this.fixed({ type: "button", icon: text(v.icon), label: text(v.label) });
      }
      case "separator": {
        yield* this.record(value, ["type"]);
        return this.fixed({ type: "separator" });
      }
      case "input": {
        const v = yield* this.record(value, ["type", "kind", "value", "placeholder", "commit", "min", "max", "step", "accept"]);
        return this.fixed({ type: "input", kind: defaulted(v.kind, "text", (v2) => choice(v2, ["text", "longText", "number", "date", "color", "file"])), value: text(v.value), placeholder: optional(v.placeholder, text), commit: optional(v.commit, text), min: optional(v.min, number), max: optional(v.max, number), step: optional(v.step, number), accept: optional(v.accept, text) });
      }
      case "select": {
        const v = yield* this.record(value, ["type", "value", "items", "placeholder"]);
        const items = yield* this.list(v.items, (item) => this.selectItem(item));
        yield 128;
        return this.fixed({ type: "select", value: text(v.value), items, placeholder: optional(v.placeholder, text) });
      }
      case "toggle": {
        const v = yield* this.record(value, ["type", "on", "icon", "text"]);
        return this.fixed({ type: "toggle", on: boolean(v.on), icon: text(v.icon), text: optional(v.text, text) });
      }
      case "keyValueList": {
        const v = yield* this.record(value, ["type", "entries"]);
        const entries = yield* this.list(v.entries, (item) => this.entry(item));
        yield 128;
        return this.fixed({ type: "keyValueList", entries });
      }
      case "slider": {
        const v = yield* this.record(value, ["type", "value", "min", "max", "step", "unit"]);
        return this.fixed({ type: "slider", value: number(v.value), min: number(v.min), max: number(v.max), step: number(v.step), unit: optional(v.unit, text) });
      }
      case "numberStepper": {
        const v = yield* this.record(value, ["type", "value", "step", "uniform"]);
        return this.fixed({ type: "numberStepper", value: number(v.value), step: number(v.step), uniform: boolean(v.uniform) });
      }
      case "ring": {
        const v = yield* this.record(value, ["type", "orbId", "t"]);
        return this.fixed({ type: "ring", orbId: text(v.orbId), t: number(v.t) });
      }
      case "iconSelect": {
        const v = yield* this.record(value, ["type", "value", "uniform", "classifierKind"]);
        return this.fixed({ type: "iconSelect", value: text(v.value), uniform: boolean(v.uniform), classifierKind: text(v.classifierKind) });
      }
      case "tree": {
        const v = yield* this.record(value, ["type", "interactionDomain"]);
        return this.fixed({ type: "tree", interactionDomain: optional(v.interactionDomain, text) });
      }
      case "treeSection": {
        const v = yield* this.record(value, ["type", "label", "defaultOpen"]);
        return this.fixed({ type: "treeSection", label: optional(v.label, text), defaultOpen: optional(v.defaultOpen, boolean) });
      }
      case "treeItem": {
        const v = yield* this.record(value, ["type", "label", "description", "icon", "defaultOpen", "draggable", "dragData", "dimmed", "rowActions"]);
        const dragData = v.dragData == null ? null : yield* this.stringMap(v.dragData);
        const rowActions = yield* this.list(v.rowActions === undefined ? [] : v.rowActions, (item) => this.rowAction(item));
        yield 128;
        return this.fixed({ type: "treeItem", label: text(v.label), description: optional(v.description, text), icon: optional(v.icon, text), defaultOpen: optional(v.defaultOpen, boolean), draggable: optional(v.draggable, boolean), dragData, dimmed: optional(v.dimmed, boolean), rowActions });
      }
      case "image": {
        const v = yield* this.record(value, ["type", "src", "alt"]);
        return this.fixed({ type: "image", src: text(v.src), alt: optional(v.alt, text) });
      }
      case "surface": {
        const v = yield* this.record(value, ["type", "kind", "docSchema", "doc", "bindings"]);
        const d = yield* this.record(v.doc, ["bytes"]);
        if (!(d.bytes instanceof UiSurfaceBytes))
          throw new Error("Surface document requires exact owned byte pages");
        const source = d.bytes.capture();
        this.bytes = { value: source, next: this.bytes };
        const bytes = this.fixed(new ByteView(source));
        const doc = this.fixed({ bytes });
        yield 128;
        const bindings = yield* this.bindings(v.bindings === undefined ? [] : v.bindings);
        yield 128;
        return this.fixed({ type: "surface", kind: choice(v.kind, ["canvas-2d", "world-3d", "node-graph", "text-editor", "table", "paint-2d", "virtual-file-system", "tiled-map", "board-2d", "icon-render", "ink-canvas", "graph-timeline", "block-list", "diff-view", "event-feed"]), docSchema: text(v.docSchema), doc, bindings });
      }
      case "extension": {
        const v = yield* this.record(value, ["type", "extension", "props"]);
        const props = yield* this.value(v.props);
        yield 128;
        return this.fixed({ type: "extension", extension: text(v.extension), props });
      }
      default:
        throw new Error("Unknown UI component type");
    }
  }
  *selectItem(value) {
    const v = yield* this.record(value, ["value", "label"]);
    return this.fixed({ value: text(v.value), label: text(v.label) });
  }
  *entry(value) {
    const v = yield* this.record(value, ["label", "value"]);
    return this.fixed({ label: text(v.label), value: text(v.value) });
  }
  *rowAction(value) {
    const v = yield* this.record(value, ["icon", "label", "action", "placement"]);
    const action = yield* this.binding(v.action);
    yield 128;
    return this.fixed({ icon: text(v.icon), label: optional(v.label, text), action, placement: defaulted(v.placement, "row", (v2) => choice(v2, ["row", "menu"])) });
  }
  *sizing(value) {
    yield 32;
    if (value === "hug" || value === "fill")
      return value;
    const v = yield* this.record(value, ["fixed"]);
    return this.fixed({ fixed: space(v.fixed) });
  }
  *edges(value) {
    yield 32;
    if (!isRecord(value) || Object.keys(value).length !== 1)
      throw new Error("Expected exactly one edge-space variant");
    if (Object.hasOwn(value, "all"))
      return this.fixed({ all: space(value.all) });
    if (Object.hasOwn(value, "symmetric")) {
      const v2 = yield* this.record(value.symmetric, ["vertical", "horizontal"]);
      return this.fixed({ symmetric: this.fixed({ vertical: space(v2.vertical), horizontal: space(v2.horizontal) }) });
    }
    const v = yield* this.record(value.each, ["top", "right", "bottom", "left"]);
    return this.fixed({ each: this.fixed({ top: space(v.top), right: space(v.right), bottom: space(v.bottom), left: space(v.left) }) });
  }
  *track(value) {
    yield 32;
    if (value === "auto" || value === "minContent" || value === "maxContent")
      return value;
    if (!isRecord(value) || Object.keys(value).length !== 1)
      throw new Error("Expected exactly one grid-track variant");
    if (Object.hasOwn(value, "fraction"))
      return this.fixed({ fraction: natural(value.fraction, 255) });
    const v = yield* this.record(value, ["fixed"]);
    return this.fixed({ fixed: space(v.fixed) });
  }
  *layout(value) {
    yield 64;
    if (!isRecord(value))
      throw new Error("Expected UI layout");
    const align = (value2) => choice(value2, ["start", "center", "end", "stretch", "baseline"]);
    const justify = (value2) => choice(value2, ["start", "center", "end", "spaceBetween", "spaceAround", "spaceEvenly"]);
    switch (value.kind) {
      case "leaf": {
        const v = yield* this.record(value, ["kind", "width", "height"]);
        const width = yield* this.sizing(v.width);
        const height = yield* this.sizing(v.height);
        yield 128;
        return this.fixed({ kind: "leaf", width, height });
      }
      case "stack": {
        const v = yield* this.record(value, ["kind", "axis", "gap", "padding", "align", "justify", "grow", "wrap"]);
        const padding = yield* this.edges(v.padding);
        yield 128;
        return this.fixed({ kind: "stack", axis: choice(v.axis, ["horizontal", "vertical"]), gap: space(v.gap), padding, align: align(v.align), justify: justify(v.justify), grow: boolean(v.grow), wrap: boolean(v.wrap) });
      }
      case "grid": {
        const v = yield* this.record(value, ["kind", "columns", "rows", "columnGap", "rowGap", "padding", "align", "justify"]);
        const columns = yield* this.list(v.columns, (item) => this.track(item), 32);
        const rows = yield* this.list(v.rows, (item) => this.track(item), 32);
        const padding = yield* this.edges(v.padding);
        yield 128;
        return this.fixed({ kind: "grid", columns, rows, columnGap: space(v.columnGap), rowGap: space(v.rowGap), padding, align: align(v.align), justify: justify(v.justify) });
      }
      case "overlay": {
        const v = yield* this.record(value, ["kind", "anchor", "inset", "dismissible"]);
        const inset = yield* this.edges(v.inset);
        yield 128;
        return this.fixed({ kind: "overlay", anchor: choice(v.anchor, ["topStart", "top", "topEnd", "start", "center", "end", "bottomStart", "bottom", "bottomEnd"]), inset, dismissible: boolean(v.dismissible) });
      }
      case "scroll": {
        const v = yield* this.record(value, ["kind", "axes", "padding", "sizing"]);
        const padding = yield* this.edges(v.padding);
        const sizing = yield* this.sizing(v.sizing);
        yield 128;
        return this.fixed({ kind: "scroll", axes: choice(v.axes, ["none", "horizontal", "vertical", "both"]), padding, sizing });
      }
      case "absolute": {
        const v = yield* this.record(value, ["kind", "sizingWidth", "sizingHeight"]);
        const sizingWidth = yield* this.sizing(v.sizingWidth);
        const sizingHeight = yield* this.sizing(v.sizingHeight);
        yield 128;
        return this.fixed({ kind: "absolute", sizingWidth, sizingHeight });
      }
      default:
        throw new Error("Unknown UI layout kind");
    }
  }
  *style(value) {
    const v = yield* this.record(value, ["variant", "size", "density", "tone", "emphasis"]);
    return this.fixed({ variant: defaulted(v.variant, "solid", (v2) => choice(v2, ["solid", "outline", "ghost", "plain"])), size: defaulted(v.size, "md", (v2) => choice(v2, ["xs", "sm", "md", "lg", "xl"])), density: defaulted(v.density, "standard", (v2) => choice(v2, ["compact", "standard", "touch"])), tone: defaulted(v.tone, "neutral", (v2) => choice(v2, ["neutral", "primary", "secondary", "tertiary", "info", "success", "warning", "danger"])), emphasis: defaulted(v.emphasis, "regular", (v2) => choice(v2, ["subtle", "regular", "strong"])) });
  }
  *accessibility(value) {
    const v = yield* this.record(value, ["label", "description", "live", "shortcut", "hidden"]);
    return this.fixed({ label: optional(v.label, text), description: optional(v.description, text), live: defaulted(v.live, "off", (v2) => choice(v2, ["off", "polite", "assertive"])), shortcut: optional(v.shortcut, text), hidden: defaulted(v.hidden, false, boolean) });
  }
  *activity(value) {
    const v = yield* this.record(value, ["activity", "disabled"]);
    return this.fixed({ activity: activity(v.activity), disabled: boolean(v.disabled) });
  }
  *childIds(value) {
    return yield* this.list(value, function* (item) {
      yield 32;
      return natural(item);
    }, 128);
  }
  *node(value) {
    const v = yield* this.record(value, ["id", "key", "component", "layout", "style", "activity", "disabled", "transition", "accessibility", "bindings", "menu", "children"]);
    const component = yield* this.field("component", v.component);
    const layout = yield* this.field("layout", v.layout);
    const style = yield* this.field("style", v.style);
    const accessibility = yield* this.field("accessibility", v.accessibility);
    const bindings = yield* this.field("bindings", v.bindings === undefined ? [] : v.bindings);
    const menu = yield* this.field("menu", v.menu === undefined ? null : v.menu);
    const children = yield* this.field("children", v.children === undefined ? [] : v.children);
    this.fields = { component, layout, style, accessibility, bindings, menu, children };
    yield 128;
    return this.fixed({ id: natural(v.id), key: text(v.key), component: component.value, layout: layout.value, style: style.value, activity: activity(v.activity), disabled: defaulted(v.disabled, false, boolean), transition: optional(v.transition, (v2) => choice(v2, ["introducing", "celebrating"])), accessibility: accessibility.value, bindings: bindings.value, menu: menu.value, children: children.value });
  }
}
var readers = {
  component: (b, v) => b.component(v),
  node: (b, v) => b.node(v),
  layout: (b, v) => b.layout(v),
  style: (b, v) => b.style(v),
  activity: (b, v) => b.activity(v),
  accessibility: (b, v) => b.accessibility(v),
  bindings: (b, v) => b.bindings(v),
  menu: (b, v) => b.menu(v),
  children: (b, v) => b.childIds(v)
};

class RetainedUiTypedCursor {
  #decoder;
  #builder = new Builder;
  #program = null;
  #payload = null;
  #retirement = null;
  #closing = false;
  #failure = null;
  #phase = "typed-decode";
  #profile;
  constructor(input, profile) {
    this.#profile = profile;
    this.#decoder = new RetainedUiWireValueCursor(input, profile === "node" || profile === "component" ? profile : "value");
  }
  get profile() {
    return this.#profile;
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted6(grant))
      return step3("blocked", this.#phase);
    if (this.#closing || this.#failure)
      return step3("rejected", this.#phase);
    if (this.#payload)
      return step3("ready", this.#phase);
    try {
      if (this.#phase === "typed-decode") {
        const result2 = this.#decoder.advance(grant);
        if (result2.kind === "rejected")
          throw new Error(this.#decoder.failure);
        if (result2.kind === "ready") {
          this.#program = readers[this.#profile](this.#builder, this.#decoder.value);
          this.#phase = "typed-normalize";
        }
        return { ...result2, kind: result2.kind === "ready" ? "pending" : result2.kind };
      }
      if (this.#phase === "typed-normalize") {
        const result2 = this.#program.next();
        if (!result2.done)
          return step3("pending", this.#phase, result2.value);
        this.#program = null;
        this.#payload = ownPayload({ value: result2.value, references: 1, owned: this.#builder.owned, bytes: this.#builder.bytes, children: this.#builder.children, fields: this.#builder.fields, kind: this.#profile });
        this.#builder.owned = null;
        this.#builder.bytes = null;
        this.#builder.children = null;
        this.#builder.fields = null;
        this.#phase = "typed-ready";
        return step3("ready", this.#phase, 128);
      }
      throw new Error("Invalid typed UI cursor phase");
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Typed UI normalization failed";
      return step3("rejected", this.#phase, 2112);
    }
  }
  takeResult() {
    if (this.#closing || this.#failure || this.#phase !== "typed-ready")
      return null;
    const payload = this.#payload;
    this.#payload = null;
    this.#phase = "typed-taken";
    return payload;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted6(grant))
      return step3("blocked", "typed-close");
    if (!this.#closing)
      throw new Error("Begin typed UI close before advancing retirement");
    const builder = this.#builder.active ?? this.#builder;
    if (builder.json) {
      const frame = builder.json;
      builder.json = frame.parent;
      frame.parent = null;
      return step3("pending", "typed-json-frame-close", 2112);
    }
    if (this.#program) {
      this.#program = null;
      return step3("pending", "typed-program-close", 2112);
    }
    if (this.#payload) {
      this.#retirement = this.#payload.beginClose();
      this.#payload = null;
      return step3("pending", "typed-payload-close", 64);
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { ...result2, kind: "pending" };
    }
    if (builder.owned || builder.bytes || builder.children) {
      this.#retirement = retirePayload({ value: undefined, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: null, kind: null });
      builder.owned = null;
      builder.bytes = null;
      builder.children = null;
      builder.fields = null;
      return step3("pending", "typed-partial-close", 128);
    }
    if (this.#builder.active) {
      this.#builder.active = null;
      return step3("pending", "typed-field-builder-close", 64);
    }
    if (this.#decoder) {
      this.#decoder.beginClose();
      const result2 = this.#decoder.closeStep(grant);
      if (result2.kind === "complete")
        this.#decoder = null;
      return { ...result2, kind: "pending" };
    }
    return step3("complete", "typed-close");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#decoder && !this.#program && !this.#payload && !this.#retirement && !this.#builder.owned && !this.#builder.bytes && !this.#builder.json && !this.#builder.active && !this.#builder.children && !this.#builder.fields;
  }
}

class RetainedUiChildIdsCursor {
  #input;
  #output;
  #index = 0;
  #payload = null;
  #retirement = null;
  #failure = null;
  #closing = false;
  #ready = false;
  constructor(input) {
    this.#input = new BigUint64Array(takeOwnedNativeBuffer(input, "BigUint64Array", 1024));
    this.#output = new Array(this.#input.length);
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted6(grant))
      return step3("blocked", "native-child-field");
    if (this.#closing || this.#failure)
      return step3("rejected", "native-child-field");
    if (this.#ready)
      return step3("ready", "native-child-field");
    try {
      if (this.#index < this.#input.length) {
        const value = this.#input[this.#index];
        if (value > 9007199254740991n)
          throw new Error("Native child ID exceeds the exact renderer range");
        Object.defineProperty(this.#output, this.#index++, { value: Number(value), enumerable: true });
        return step3("pending", "native-child-field", 64);
      }
      const output = this.#output;
      Object.defineProperty(output, "length", { writable: false });
      Object.preventExtensions(output);
      this.#payload = ownPayload({ value: output, references: 1, owned: { value: output, next: null }, bytes: null, children: null, fields: null, kind: "children" });
      this.#output = null;
      this.#input = null;
      this.#ready = true;
      return step3("ready", "native-child-field", 1280);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Native child field failed";
      return step3("rejected", "native-child-field", 64);
    }
  }
  takeResult() {
    if (!this.#ready || this.#closing || this.#failure)
      return null;
    const result2 = this.#payload;
    this.#payload = null;
    return result2;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted6(grant))
      return step3("blocked", "native-child-close");
    if (!this.#closing)
      throw new Error("Native child close has not begun");
    if (this.#payload) {
      this.#retirement = this.#payload.beginClose();
      this.#payload = null;
      return step3("pending", "native-child-close", 64);
    }
    if (this.#retirement) {
      const current = this.#retirement.advance(grant);
      if (current.kind === "complete")
        this.#retirement = null;
      return { ...current, kind: "pending" };
    }
    if (this.#input || this.#output) {
      this.#input = null;
      this.#output = null;
      return step3("pending", "native-child-close", 2112);
    }
    return step3("complete", "native-child-close");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#input && !this.#output && !this.#payload && !this.#retirement;
  }
}
if (undefined) {
  let prepared = function(kind, value) {};
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️.ts */
var MINT3 = Symbol("owned-ui-operation");
var admitted7 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var state2 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function id(value) {
  if (!Number.isSafeInteger(value) || value < 0)
    throw new RangeError("UI node ID is not a nonnegative safe integer");
  return value === 0 ? 0 : value;
}
var takeOperation;
var checkOperation;
function* operationStrings(operation) {
  if (operation.kind === "upsert") {
    const node = operation.node.value;
    yield node.key;
    yield* componentStrings(node.component);
    yield* accessibilityStrings(node.accessibility);
    yield* bindingStrings(node.bindings);
    yield node.menu?.id ?? "";
  }
  if (operation.kind === "field") {
    const change = operation.change;
    if (change.field === "component")
      yield* componentStrings(change.payload.value);
    if (change.field === "accessibility")
      yield* accessibilityStrings(change.payload.value);
    if (change.field === "bindings")
      yield* bindingStrings(change.payload.value);
    if (change.field === "menu")
      yield change.payload.value?.id ?? "";
  }
}

class OwnedUiOperation {
  #value;
  constructor(mint, value) {
    if (mint !== MINT3)
      throw new Error("Invalid owned UI operation authority");
    this.#value = value;
    Object.freeze(this);
  }
  static {
    checkOperation = (owner) => {
      if (!owner.#value)
        throw new Error("Owned UI operation is closed");
    };
    takeOperation = (owner) => {
      checkOperation(owner);
      const value = owner.#value;
      owner.#value = null;
      return value;
    };
  }
  static upsert(payload) {
    return new OwnedUiOperation(MINT3, { kind: "upsert", node: OwnedUiNode.captureFrom(payload) });
  }
  static field(node, change) {
    const exact = id(node);
    return new OwnedUiOperation(MINT3, { kind: "field", id: exact, change: captureUiFieldChange(change) });
  }
  static activity(node, payload) {
    const exact = id(node);
    return new OwnedUiOperation(MINT3, { kind: "activity", id: exact, payload: captureTypedUiPayload("activity", payload) });
  }
  static remove(node) {
    return new OwnedUiOperation(MINT3, { kind: "remove", id: id(node) });
  }
  static setRoot(node) {
    return new OwnedUiOperation(MINT3, { kind: "root", id: node === null ? null : id(node) });
  }
  beginClose() {
    return new OwnedUiOperationRetirement(MINT3, takeOperation(this));
  }
  terminalIsEmpty() {
    return this.#value === null;
  }
}

class OwnedUiOperationRetirement {
  #retirement;
  constructor(mint, value) {
    if (mint !== MINT3)
      throw new Error("Invalid owned UI operation retirement authority");
    const owner = value.kind === "upsert" ? value.node : value.kind === "field" ? value.change.payload : value.kind === "activity" ? value.payload : null;
    this.#retirement = owner && !owner.terminalIsEmpty() ? owner.beginClose() : null;
    Object.freeze(this);
  }
  advance(grant) {
    if (!admitted7(grant))
      return state2("blocked", "operation-close");
    if (!this.#retirement)
      return state2("complete", "operation-close");
    const result2 = this.#retirement.advance(grant);
    if (result2.kind === "complete")
      this.#retirement = null;
    return { ...result2, kind: "pending" };
  }
  terminalIsEmpty() {
    return this.#retirement === null;
  }
}

class OwnedUiOperationCursor {
  #nodes;
  #root;
  #operation;
  #program = null;
  #reader = null;
  #edit = null;
  #old = null;
  #retirement = null;
  #node = null;
  #replacement = null;
  #nodeRetirement = null;
  #operationRetirement = null;
  #stack = null;
  #grant = { maxItems: 0, maxBytes: 0 };
  #touched;
  #status = "pending";
  #failure = null;
  #estimatedBytes = 16;
  #maxChildren;
  #maxTextBytes;
  constructor(source, root, operation, limits = { maxChildren: Number.MAX_SAFE_INTEGER, maxTextBytes: Number.MAX_SAFE_INTEGER }) {
    this.#maxChildren = id(limits.maxChildren);
    this.#maxTextBytes = id(limits.maxTextBytes);
    checkOperation(operation);
    this.#root = root === null ? null : id(root);
    this.#nodes = source.capture();
    this.#operation = takeOperation(operation);
    this.#touched = new Table(NumericIndex.empty(), () => this.#grant);
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  *#drain() {
    while (this.#retirement) {
      const result2 = this.#retirement.advance(this.#grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      yield result2.bytes;
    }
  }
  *#releaseNode() {
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      yield 64;
    }
    while (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(this.#grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      yield result2.bytes;
    }
  }
  *#lookup(node) {
    this.#reader = this.#nodes.beginLookup(node);
    yield 64;
    for (;; ) {
      const result2 = this.#reader.advance(this.#grant);
      if (result2.kind === "value")
        this.#node = result2.value;
      yield result2.bytes;
      if (result2.kind === "complete")
        break;
    }
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield 64;
    yield* this.#drain();
  }
  *#change(edit) {
    this.#edit = edit;
    yield 64;
    for (;; ) {
      const result2 = edit.advance(this.#grant);
      if (result2.kind === "rejected")
        throw new Error(edit.failure ?? "Owned UI index rejected operation");
      yield result2.bytes;
      if (result2.kind === "ready")
        break;
    }
    this.#old = this.#nodes;
    this.#nodes = edit.takeResult();
    this.#retirement = edit.beginClose();
    this.#edit = null;
    yield 128;
    yield* this.#drain();
    this.#retirement = this.#old.beginClose();
    this.#old = null;
    yield 64;
    yield* this.#drain();
  }
  *#run() {
    const operation = this.#operation;
    const children = operation.kind === "upsert" ? operation.node.value.children : operation.kind === "field" && operation.change.field === "children" ? operation.change.payload.value : null;
    if (children && children.length > this.#maxChildren)
      throw new Error("Owned UI children quota exceeded");
    const component = operation.kind === "upsert" ? operation.node.value.component : operation.kind === "field" && operation.change.field === "component" ? operation.change.payload.value : null;
    if (component && (yield* measure(componentStrings(component), () => this.#grant)) > this.#maxTextBytes)
      throw new Error("Owned UI text quota exceeded");
    this.#estimatedBytes += yield* measure(operationStrings(operation), () => this.#grant);
    if (operation.kind === "field" && operation.change.field === "children")
      this.#estimatedBytes += operation.change.payload.value.length * 8;
    if (operation.kind === "root") {
      this.#root = operation.id;
      yield 16;
    } else if (operation.kind === "upsert") {
      yield* this.#change(this.#nodes.beginSet(operation.node));
      yield* this.#touched.set(operation.node.value.id, true);
    } else if (operation.kind === "remove") {
      this.#stack = { id: operation.id, next: null };
      yield 32;
      while (this.#stack) {
        const cell = this.#stack;
        this.#stack = cell.next;
        cell.next = null;
        yield 32;
        yield* this.#lookup(cell.id);
        if (!this.#node)
          continue;
        const children2 = this.#node.value.children;
        yield* this.#change(this.#nodes.beginRemove(cell.id));
        yield* this.#touched.set(cell.id, true);
        for (const child of children2) {
          this.#stack = { id: child, next: this.#stack };
          yield 32;
        }
        yield* this.#releaseNode();
      }
    } else {
      yield* this.#lookup(operation.id);
      if (!this.#node)
        throw new Error(`Unknown UI node: ${operation.id}`);
      this.#replacement = operation.kind === "field" ? this.#node.replace(operation.change) : this.#node.withActivity(operation.payload);
      yield 512;
      yield* this.#change(this.#nodes.beginSet(this.#replacement));
      yield* this.#touched.set(operation.id, true);
      yield* this.#releaseNode();
      this.#node = this.#replacement;
      this.#replacement = null;
      yield* this.#releaseNode();
    }
    this.#operationRetirement = new OwnedUiOperationRetirement(MINT3, operation);
    this.#operation = null;
    yield 64;
    while (this.#operationRetirement) {
      const result2 = this.#operationRetirement.advance(this.#grant);
      if (result2.kind === "complete")
        this.#operationRetirement = null;
      yield result2.bytes;
    }
  }
  advance(grant) {
    if (this.#status !== "pending") {
      if (this.#status === "closing")
        throw new Error("Owned UI operation is closing");
      return state2(this.#status === "closed" ? "complete" : this.#status, this.#status);
    }
    if (!admitted7(grant))
      return state2("blocked", "operation");
    this.#grant = grant;
    this.#program ??= this.#run();
    try {
      const result2 = this.#program.next();
      if (result2.done) {
        this.#program = null;
        this.#status = "ready";
        return state2("ready", "operation", 32);
      }
      if (result2.value > grant.maxBytes)
        throw new Error("Owned UI operation exceeded its grant");
      return state2("pending", "operation", result2.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Owned UI operation failed";
      this.#status = "rejected";
      this.#program = null;
      return state2("rejected", "operation", 64);
    }
  }
  takeResult() {
    if (this.#status !== "ready" || !this.#nodes)
      return null;
    const nodes = this.#nodes;
    this.#nodes = null;
    return { nodes, root: this.#root, touched: this.#touched.take(), estimatedBytes: this.#estimatedBytes };
  }
  beginClose() {
    if (this.#status === "closed" || this.#status === "closing")
      return;
    this.#status = "closing";
    this.#program = null;
  }
  closeStep(grant) {
    if (this.#status === "closed")
      return state2("complete", "operation-close");
    if (this.#status !== "closing")
      throw new Error("Owned UI close has not begun");
    if (!admitted7(grant))
      return state2("blocked", "operation-close");
    if (this.#stack) {
      const cell = this.#stack;
      this.#stack = cell.next;
      cell.next = null;
      return state2("pending", "operation-stack-close", 32);
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#reader) {
      this.#retirement = this.#reader.beginClose();
      this.#reader = null;
      return state2("pending", "operation-reader-close", 64);
    }
    if (this.#edit) {
      this.#retirement = this.#edit.beginClose();
      this.#edit = null;
      return state2("pending", "operation-edit-close", 64);
    }
    if (this.#old) {
      this.#retirement = this.#old.beginClose();
      this.#old = null;
      return state2("pending", "operation-old-close", 64);
    }
    if (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      return state2("pending", "operation-node-close", 64);
    }
    if (this.#replacement) {
      this.#nodeRetirement = this.#replacement.beginClose();
      this.#replacement = null;
      return state2("pending", "operation-replacement-close", 64);
    }
    if (this.#operationRetirement) {
      const result2 = this.#operationRetirement.advance(grant);
      if (result2.kind === "complete")
        this.#operationRetirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#operation) {
      this.#operationRetirement = new OwnedUiOperationRetirement(MINT3, this.#operation);
      this.#operation = null;
      return state2("pending", "operation-payload-close", 64);
    }
    if (this.#nodes) {
      this.#retirement = this.#nodes.beginClose();
      this.#nodes = null;
      return state2("pending", "operation-index-close", 64);
    }
    const touched = this.#touched.closeStep(grant);
    if (!touched.complete)
      return state2("pending", "operation-touched-close", touched.bytes);
    this.#status = "closed";
    return state2("complete", "operation-close");
  }
  terminalIsEmpty() {
    return this.#status === "closed" && !this.#program && !this.#stack && !this.#retirement && !this.#reader && !this.#edit && !this.#old && !this.#nodeRetirement && !this.#node && !this.#replacement && !this.#operationRetirement && !this.#operation && !this.#nodes;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️.ts */
var admitted8 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var state3 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });

class GraphNodes {
  #index;
  #reader = null;
  #retirement = null;
  #node = null;
  #nodeRetirement = null;
  grant;
  constructor(source, grant) {
    this.grant = grant;
    this.#index = source.capture();
  }
  get size() {
    return this.#index.size;
  }
  *#releaseNode() {
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      yield 64;
    }
    while (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(this.grant());
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      yield result2.bytes;
    }
  }
  *#releaseReader() {
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield 64;
    while (this.#retirement) {
      const result2 = this.#retirement.advance(this.grant());
      if (result2.kind === "complete")
        this.#retirement = null;
      yield result2.bytes;
    }
  }
  *lookup(id2) {
    this.#reader = this.#index.beginLookup(id2);
    yield 64;
    let value;
    for (;; ) {
      const result2 = this.#reader.advance(this.grant());
      if (result2.kind === "value") {
        this.#node = result2.value;
        value = result2.value.value;
      }
      yield result2.bytes;
      if (result2.kind === "complete")
        break;
    }
    yield* this.#releaseNode();
    yield* this.#releaseReader();
    return value;
  }
  *entries() {
    this.#reader = this.#index.beginRead();
    yield 64;
    for (;; ) {
      const result2 = this.#reader.advance(this.grant());
      if (result2.kind === "value") {
        this.#node = result2.value;
        yield result2.bytes;
        yield [result2.id, result2.value.value];
        yield* this.#releaseNode();
      } else
        yield result2.bytes;
      if (result2.kind === "complete")
        break;
    }
    yield* this.#releaseReader();
  }
  closeStep(grant) {
    if (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      return { complete: false, bytes: result2.bytes };
    }
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      return { complete: false, bytes: 64 };
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { complete: false, bytes: result2.bytes };
    }
    if (this.#reader) {
      this.#retirement = this.#reader.beginClose();
      this.#reader = null;
      return { complete: false, bytes: 64 };
    }
    if (this.#index) {
      this.#retirement = this.#index.beginClose();
      this.#index = null;
      return { complete: false, bytes: 64 };
    }
    return { complete: true, bytes: 0 };
  }
}

class OwnedUiValidationCursor {
  #grant = { maxItems: 0, maxBytes: 0 };
  #nodes;
  #marks;
  #violations;
  #keys;
  #frontier = { stack: null, count: 0 };
  #program;
  #status = "pending";
  #failure = null;
  #taken = false;
  #close = 0;
  constructor(source, root, limits) {
    if (root !== null && (!Number.isSafeInteger(root) || root < 0))
      throw new RangeError("Invalid UI graph root");
    const exact = { maxNodes: limits.maxNodes, maxDepth: limits.maxDepth, maxChildren: limits.maxChildren, maxTextBytes: limits.maxTextBytes, maxPatchOps: limits.maxPatchOps, maxPatchBytes: limits.maxPatchBytes };
    for (const limit of [exact.maxNodes, exact.maxDepth, exact.maxChildren, exact.maxTextBytes, exact.maxPatchOps, exact.maxPatchBytes])
      if (!Number.isSafeInteger(limit) || limit < 0)
        throw new RangeError("Invalid UI document limit");
    this.#nodes = new GraphNodes(source, () => this.#grant);
    this.#marks = new Table(NumericIndex.empty(), () => this.#grant);
    this.#violations = new Table(NumericIndex.empty(), () => this.#grant);
    this.#keys = new SiblingKeys(() => this.#grant);
    this.#program = retainedUiGraphValidation(this.#nodes, root, exact, this.#marks, this.#keys, this.#violations, this.#frontier);
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (this.#status !== "pending") {
      if (this.#status === "closing")
        throw new Error("Owned UI validation is closing");
      return state3(this.#status === "closed" ? "complete" : this.#status, "validation");
    }
    if (!admitted8(grant))
      return state3("blocked", "validation");
    this.#grant = grant;
    try {
      const result2 = this.#program.next();
      if (result2.done) {
        this.#program = null;
        this.#status = "ready";
        return state3("ready", "validation", 32);
      }
      if (result2.value > grant.maxBytes)
        throw new Error("Owned UI validation exceeded its byte grant");
      return state3("pending", "validation", result2.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Owned UI validation failed";
      this.#program = null;
      this.#status = "rejected";
      return state3("rejected", "validation", 64);
    }
  }
  takeResult() {
    if (this.#status !== "ready" || this.#taken)
      return null;
    this.#taken = true;
    return this.#violations.take();
  }
  beginClose() {
    if (this.#status === "closing" || this.#status === "closed")
      return;
    this.#status = "closing";
    this.#program = null;
  }
  closeStep(grant) {
    if (this.#status === "closed")
      return state3("complete", "validation-close");
    if (this.#status !== "closing")
      throw new Error("Owned UI validation close has not begun");
    if (!admitted8(grant))
      return state3("blocked", "validation-close");
    if (closeRetainedUiGraphFrame(this.#frontier))
      return state3("pending", "validation-stack-close", 48);
    if (this.#close < 4) {
      const owner = this.#close === 0 ? this.#keys : this.#close === 1 ? this.#marks : this.#close === 2 ? this.#violations : this.#nodes;
      const result2 = owner.closeStep(grant);
      if (result2.complete)
        this.#close++;
      return state3("pending", "validation-owner-close", result2.bytes);
    }
    this.#status = "closed";
    return state3("complete", "validation-close");
  }
  terminalIsEmpty() {
    return this.#status === "closed" && this.#close === 4 && !this.#program && !this.#frontier.stack;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️.ts */
var admitted9 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var state4 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function utf8(code) {
  if (code < 128)
    return [code];
  if (code < 2048)
    return [192 | code >>> 6, 128 | code & 63];
  if (code < 65536)
    return [224 | code >>> 12, 128 | code >>> 6 & 63, 128 | code & 63];
  return [240 | code >>> 18, 128 | code >>> 12 & 63, 128 | code >>> 6 & 63, 128 | code & 63];
}
function escaped(code) {
  const digits = "0123456789abcdef";
  return [92, 117, digits.charCodeAt(code >>> 12), digits.charCodeAt(code >>> 8 & 15), digits.charCodeAt(code >>> 4 & 15), digits.charCodeAt(code & 15)];
}

class JsonBytes {
  #stack;
  surface;
  constructor(value, surface = null, raw = false) {
    this.surface = surface;
    this.#stack = raw && typeof value === "string" ? { kind: "raw", value, offset: 0, opened: true, next: null } : { kind: "value", value, next: null };
  }
  #pop() {
    const frame = this.#stack;
    this.#stack = frame.next;
    frame.next = null;
  }
  #raw(value) {
    this.#stack = { kind: "raw", value, offset: 0, opened: true, next: this.#stack };
  }
  #text(value) {
    this.#stack = { kind: "text", value, offset: 0, opened: false, next: this.#stack };
  }
  #atom() {
    const frame = this.#stack;
    if (frame.kind === "value") {
      this.#pop();
      const value = frame.value;
      if (value === null)
        this.#raw("null");
      else if (typeof value === "string")
        this.#text(value);
      else if (typeof value === "boolean")
        this.#raw(value ? "true" : "false");
      else if (typeof value === "number" && Number.isFinite(value))
        this.#raw(String(value));
      else if (value === this.surface)
        this.#stack = { kind: "bytes", value: this.surface, offset: 0, opened: false, next: this.#stack };
      else if (Array.isArray(value))
        this.#stack = { kind: "array", value, offset: 0, opened: false, next: this.#stack };
      else if (typeof value === "object" && value !== null) {
        const keys = Object.keys(value);
        if (keys.length > 256)
          throw new Error("Owned JSON object exceeds its normalized native envelope");
        this.#stack = { kind: "object", value, keys, offset: 0, opened: false, next: this.#stack };
        return { cost: 64 + keys.length * 8, bytes: [] };
      } else
        throw new Error("Owned UI JSON contains an unsupported value");
      return { cost: 96, bytes: [] };
    }
    if (frame.kind === "raw" || frame.kind === "text") {
      if (!frame.opened) {
        frame.opened = true;
        return { cost: 32, bytes: [34] };
      }
      if (frame.offset === frame.value.length) {
        this.#pop();
        return { cost: 64, bytes: frame.kind === "text" ? [34] : [] };
      }
      const code = frame.value.charCodeAt(frame.offset++);
      if (frame.kind === "raw")
        return { cost: 32, bytes: [code] };
      if (code === 34 || code === 92)
        return { cost: 48, bytes: [92, code] };
      if (code === 8 || code === 9 || code === 10 || code === 12 || code === 13)
        return { cost: 48, bytes: [92, code === 8 ? 98 : code === 9 ? 116 : code === 10 ? 110 : code === 12 ? 102 : 114] };
      if (code < 32)
        return { cost: 64, bytes: escaped(code) };
      if (code >= 55296 && code <= 56319) {
        const low = frame.value.charCodeAt(frame.offset);
        if (low >= 56320 && low <= 57343) {
          frame.offset++;
          return { cost: 64, bytes: utf8(65536 + (code - 55296) * 1024 + low - 56320) };
        }
        return { cost: 64, bytes: escaped(code) };
      }
      return { cost: 64, bytes: code >= 56320 && code <= 57343 ? escaped(code) : utf8(code) };
    }
    if (!frame.opened) {
      frame.opened = true;
      return { cost: 32, bytes: [frame.kind === "object" ? 123 : 91] };
    }
    const length = frame.kind === "object" ? frame.keys.length : frame.value.length;
    if (frame.offset === length) {
      this.#pop();
      return { cost: 64 + (frame.kind === "object" ? frame.keys.length * 8 : 0), bytes: [frame.kind === "object" ? 125 : 93] };
    }
    const index = frame.offset++;
    if (frame.kind === "object") {
      const key = frame.keys[index];
      this.#stack = { kind: "value", value: frame.value[key], next: this.#stack };
      this.#raw(":");
      this.#text(key);
      if (index)
        this.#raw(",");
      return { cost: 256, bytes: [] };
    }
    this.#stack = { kind: "value", value: frame.kind === "bytes" ? frame.value.byteAt(index) : frame.value[index], next: this.#stack };
    if (index)
      this.#raw(",");
    return { cost: 128, bytes: [] };
  }
  advance(grant) {
    const output = new Uint8Array(256);
    let written = 0;
    let cost = 256;
    let failure = null;
    while (this.#stack && cost + 2200 <= grant.maxBytes && written + 16 <= output.length) {
      try {
        const atom = this.#atom();
        cost += atom.cost + atom.bytes.length * 3;
        for (const byte of atom.bytes)
          output[written++] = byte;
      } catch (error) {
        failure = error instanceof Error ? error.message : "Owned JSON encoding failed";
        cost += 64;
        break;
      }
    }
    return { done: this.#stack === null, cost, chunk: output.subarray(0, written), failure };
  }
  closeStep() {
    const frame = this.#stack;
    if (!frame)
      return { done: true, cost: 0 };
    const cost = 64 + (frame.kind === "object" ? frame.keys.length * 8 : 0);
    this.#pop();
    return { done: false, cost };
  }
}

class OwnedUiSnapshotHashCursor {
  #source;
  #reader = null;
  #retirement = null;
  #node = null;
  #nodeRetirement = null;
  #encoder = null;
  #program = null;
  #grant = { maxItems: 0, maxBytes: 0 };
  #hash = 2166136261;
  #bytes = 0;
  #metadata;
  #status = "pending";
  #failure = null;
  #taken = false;
  constructor(source, metadata) {
    const exact = { surface: metadata.surface, revision: metadata.revision, root: metadata.root };
    if (typeof exact.surface !== "string" || !Number.isSafeInteger(exact.revision) || exact.revision < 0 || exact.root !== null && (!Number.isSafeInteger(exact.root) || exact.root < 0))
      throw new Error("Invalid owned UI snapshot identity");
    this.#metadata = exact;
    this.#source = source.capture();
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  *#encode(value, raw = false, surface = null) {
    this.#encoder = new JsonBytes(value, surface, raw);
    yield state4("pending", "hash-encoder", 64);
    for (;; ) {
      const result2 = this.#encoder.advance(this.#grant);
      if (!Number.isSafeInteger(this.#bytes + result2.chunk.length))
        throw new RangeError("Owned snapshot byte count exhausted");
      for (const byte of result2.chunk)
        this.#hash = Math.imul(this.#hash ^ byte, 16777619) >>> 0;
      this.#bytes += result2.chunk.length;
      yield { ...state4("pending", "hash-bytes", result2.cost), chunk: result2.chunk };
      if (result2.failure)
        throw new Error(result2.failure);
      if (result2.done)
        break;
    }
    this.#encoder = null;
    yield state4("pending", "hash-encoder-close", 64);
  }
  *#run() {
    const metadata = this.#metadata;
    yield* this.#encode('{"surface":', true);
    yield* this.#encode(metadata.surface);
    yield* this.#encode(',"revision":', true);
    yield* this.#encode(metadata.revision);
    yield* this.#encode(',"root":', true);
    yield* this.#encode(metadata.root ?? 0);
    yield* this.#encode(',"nodes":[', true);
    this.#reader = this.#source.beginRead();
    yield state4("pending", "hash-reader", 64);
    let first = true;
    for (;; ) {
      const result2 = this.#reader.advance(this.#grant);
      if (result2.kind === "value") {
        this.#node = result2.value;
        yield state4("pending", "hash-node", result2.bytes);
        if (!first)
          yield* this.#encode(",", true);
        first = false;
        const record = this.#node.value;
        yield* this.#encode(record, false, record.component.type === "surface" ? record.component.doc.bytes : null);
        this.#nodeRetirement = this.#node.beginClose();
        this.#node = null;
        yield state4("pending", "hash-node-close", 64);
        while (this.#nodeRetirement) {
          const closed = this.#nodeRetirement.advance(this.#grant);
          if (closed.kind === "complete")
            this.#nodeRetirement = null;
          yield { ...closed, kind: "pending" };
        }
      } else
        yield state4("pending", "hash-reader", result2.bytes);
      if (result2.kind === "complete")
        break;
    }
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield state4("pending", "hash-reader-close", 64);
    while (this.#retirement) {
      const closed = this.#retirement.advance(this.#grant);
      if (closed.kind === "complete")
        this.#retirement = null;
      yield { ...closed, kind: "pending" };
    }
    yield* this.#encode('],"layoutEpoch":"0"}', true);
  }
  advance(grant) {
    if (this.#status !== "pending") {
      if (this.#status === "closing")
        throw new Error("Owned UI hash is closing");
      return state4(this.#status === "closed" ? "complete" : this.#status, "hash");
    }
    if (!admitted9(grant))
      return state4("blocked", "hash");
    this.#grant = grant;
    this.#program ??= this.#run();
    try {
      const result2 = this.#program.next();
      if (result2.done) {
        this.#program = null;
        this.#status = "ready";
        return state4("ready", "hash", 32);
      }
      if (result2.value.bytes > grant.maxBytes || result2.value.items > grant.maxItems)
        throw new Error("Owned UI hash exceeded its grant");
      return result2.value;
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Owned UI hash failed";
      this.#program = null;
      this.#status = "rejected";
      return state4("rejected", "hash", 64);
    }
  }
  takeResult() {
    if (this.#status !== "ready" || this.#taken)
      return null;
    this.#taken = true;
    return { hash: `${this.#hash.toString(16)}:${this.#metadata.revision}`, byteLength: this.#bytes };
  }
  beginClose() {
    if (this.#status === "closing" || this.#status === "closed")
      return;
    this.#program = null;
    this.#status = "closing";
  }
  closeStep(grant) {
    if (this.#status === "closed")
      return state4("complete", "hash-close");
    if (this.#status !== "closing")
      throw new Error("Owned UI hash close has not begun");
    if (!admitted9(grant))
      return state4("blocked", "hash-close");
    if (this.#encoder) {
      const result2 = this.#encoder.closeStep();
      if (result2.done)
        this.#encoder = null;
      return state4("pending", "hash-frame-close", result2.cost);
    }
    if (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      return state4("pending", "hash-node-close", 64);
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#reader) {
      this.#retirement = this.#reader.beginClose();
      this.#reader = null;
      return state4("pending", "hash-reader-close", 64);
    }
    if (this.#source) {
      this.#retirement = this.#source.beginClose();
      this.#source = null;
      return state4("pending", "hash-source-close", 64);
    }
    this.#metadata = null;
    this.#status = "closed";
    return state4("complete", "hash-close");
  }
  terminalIsEmpty() {
    return this.#status === "closed" && !this.#encoder && !this.#node && !this.#nodeRetirement && !this.#reader && !this.#retirement && !this.#source && !this.#metadata && !this.#program;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️.ts */
var GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
var MINT4 = Object.freeze({});
var admitted10 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step4 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var ownDocument;
var ownReader;
var ownRetirement;
function sourceComponent(root) {
  const value = root.source?.value;
  if (value?.type !== "surface")
    throw new Error("Scene source owner is closed");
  return value;
}
function sourceBytes(root) {
  return sourceComponent(root).doc.bytes;
}
function captureRoot(root) {
  if (root.references === Number.MAX_SAFE_INTEGER || !root.source || !root.index)
    throw new Error("Scene root cannot be captured");
  root.references++;
  return root;
}

class Utf8Scalar {
  #remaining = 0;
  #value = 0;
  #minimum = 0;
  get complete() {
    return this.#remaining === 0;
  }
  push(byte) {
    if (this.#remaining) {
      if ((byte & 192) !== 128)
        throw new Error("Invalid scene UTF-8 continuation");
      this.#value = this.#value * 64 + (byte & 63);
      if (--this.#remaining)
        return null;
      if (this.#value < this.#minimum || this.#value > 1114111 || this.#value >= 55296 && this.#value <= 57343)
        throw new Error("Invalid scene Unicode scalar");
      return this.#value;
    }
    if (byte < 128)
      return byte;
    if (byte >= 194 && byte <= 223) {
      this.#remaining = 1;
      this.#value = byte & 31;
      this.#minimum = 128;
      return null;
    }
    if (byte >= 224 && byte <= 239) {
      this.#remaining = 2;
      this.#value = byte & 15;
      this.#minimum = 2048;
      return null;
    }
    if (byte >= 240 && byte <= 244) {
      this.#remaining = 3;
      this.#value = byte & 7;
      this.#minimum = 65536;
      return null;
    }
    throw new Error("Invalid scene UTF-8 leader");
  }
}

class OwnedUiSceneDocument {
  #root;
  constructor(mint, root) {
    if (mint !== MINT4)
      throw new Error("Scene document requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    ownDocument = (root) => new OwnedUiSceneDocument(MINT4, root);
  }
  #live() {
    if (!this.#root)
      throw new Error("Scene document is closed");
    return this.#root;
  }
  get size() {
    return this.#live().index.size;
  }
  get kind() {
    return sourceComponent(this.#live()).kind;
  }
  get schema() {
    return sourceComponent(this.#live()).docSchema;
  }
  capture() {
    return ownDocument(captureRoot(this.#live()));
  }
  #read(id2, mode, offset = 0, length = null) {
    const root = this.#live();
    if (!Number.isSafeInteger(id2) || id2 < 0 || root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Scene read cannot be admitted");
    if (!Number.isSafeInteger(offset) || offset < 0 || length !== null && (!Number.isSafeInteger(length) || length < 0))
      throw new Error("Invalid scene text byte range");
    const reader = root.index.beginLookup(id2);
    return ownReader(captureRoot(root), reader, mode, offset, length);
  }
  beginRead(id2 = 0) {
    return this.#read(id2, "value");
  }
  beginText(id2) {
    return this.#read(id2, "text");
  }
  beginBytes(id2) {
    return this.#read(id2, "bytes");
  }
  beginTextBytes(id2, offset = 0, length) {
    return this.#read(id2, "text-bytes", offset, length ?? null);
  }
  beginClose() {
    const root = this.#live();
    this.#root = null;
    return ownRetirement(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class OwnedUiSceneReader {
  #root;
  #reader;
  #readerClose = null;
  #value = null;
  #offset = 0;
  #done = false;
  #failure = null;
  #mode;
  #start;
  #length;
  #utf8 = new Utf8Scalar;
  constructor(mint, root, reader, mode, offset, length) {
    if (mint !== MINT4)
      throw new Error("Scene reader requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    this.#mode = mode;
    this.#start = offset;
    this.#length = length;
    Object.freeze(this);
  }
  static {
    ownReader = (root, reader, mode, offset, length) => new OwnedUiSceneReader(MINT4, root, reader, mode, offset, length);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted10(grant))
      return step4("blocked", "scene-read");
    if (!this.#root || this.#failure)
      return step4("rejected", "scene-read");
    if (this.#done)
      return step4("complete", "scene-read");
    if (this.#reader) {
      const current = this.#reader.advance(GRANT);
      if (current.kind === "value")
        this.#value = current.value.value;
      if (current.kind === "complete") {
        this.#readerClose = this.#reader.beginClose();
        this.#reader = null;
      }
      return step4("pending", "scene-read-lookup", current.bytes + 64);
    }
    if (this.#readerClose) {
      const current = this.#readerClose.advance(GRANT);
      if (current.kind === "complete")
        this.#readerClose = null;
      return step4("pending", "scene-read-lookup-close", current.bytes);
    }
    const value = this.#value;
    if (!value) {
      this.#done = true;
      return step4("complete", "scene-read");
    }
    if (this.#mode === "value") {
      this.#done = true;
      return { kind: "value", value, items: 1, bytes: 80 };
    }
    if (value.kind !== "text" && value.kind !== "bytes" || (this.#mode === "text-bytes" ? value.kind !== "text" : value.kind !== this.#mode)) {
      this.#failure = "Scene read kind mismatch";
      return step4("rejected", "scene-read-kind", 16);
    }
    const length = this.#length ?? value.length - this.#start;
    if (this.#start > value.length || length > value.length - this.#start) {
      this.#failure = "Scene text byte range exceeds its exact field";
      return step4("rejected", "scene-read-range", 32);
    }
    if (this.#offset === length) {
      this.#done = true;
      return step4("complete", "scene-read");
    }
    const source = sourceBytes(this.#root);
    if (value.kind === "bytes" || this.#mode === "text-bytes") {
      const count = Math.min(256, length - this.#offset);
      const bytes = new Uint8Array(count);
      for (let index = 0;index < count; index++)
        bytes[index] = source.byteAt(value.offset + this.#start + this.#offset++);
      return { kind: "bytes", value: bytes, items: 1, bytes: count * 2 + 32 };
    }
    let text2 = "";
    let read = 0;
    while (read < 128 && this.#offset < value.length) {
      const scalar = this.#utf8.push(source.byteAt(value.offset + this.#offset++));
      read++;
      if (scalar !== null)
        text2 += String.fromCodePoint(scalar);
    }
    return { kind: "text", value: text2, items: 1, bytes: read + text2.length * 2 + 64 };
  }
  beginClose() {
    if (!this.#root)
      throw new Error("Scene reader is already closed");
    const retirement = ownRetirement(this.#root, this.#reader ? this.#reader.beginClose() : this.#readerClose);
    this.#root = null;
    this.#reader = null;
    this.#readerClose = null;
    this.#value = null;
    return retirement;
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null && this.#readerClose === null && this.#value === null;
  }
}

class OwnedUiSceneRetirement {
  #root;
  #reader;
  #released = false;
  #index = null;
  #source = null;
  constructor(mint, root, reader) {
    if (mint !== MINT4)
      throw new Error("Scene retirement requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    ownRetirement = (root, reader = null) => new OwnedUiSceneRetirement(MINT4, root, reader);
  }
  advance(grant) {
    if (!admitted10(grant))
      return step4("blocked", "scene-close");
    if (this.#reader) {
      const current = this.#reader.advance(GRANT);
      if (current.kind === "complete")
        this.#reader = null;
      return step4("pending", "scene-read-close", current.bytes);
    }
    if (this.#root && !this.#released) {
      this.#released = true;
      if (--this.#root.references)
        this.#root = null;
      return step4("pending", "scene-root-release", 32);
    }
    if (this.#index) {
      const current = this.#index.advance(GRANT);
      if (current.kind === "complete")
        this.#index = null;
      return step4("pending", "scene-index-close", current.bytes);
    }
    if (this.#root?.index) {
      this.#index = this.#root.index.beginClose();
      this.#root.index = null;
      return step4("pending", "scene-index-close", 64);
    }
    if (this.#source) {
      const current = this.#source.advance(GRANT);
      if (current.kind === "complete")
        this.#source = null;
      return { ...current, kind: "pending" };
    }
    if (this.#root?.source) {
      this.#source = this.#root.source.beginClose();
      this.#root.source = null;
      return step4("pending", "scene-source-close", 64);
    }
    this.#root = null;
    return step4("complete", "scene-close");
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null && this.#index === null && this.#source === null;
  }
}

class OwnedUiSceneCursor {
  #source;
  #entries = NumericIndex.empty();
  #buckets = NumericIndex.empty();
  #entryEdit = null;
  #bucketEdit = null;
  #entryReader = null;
  #bucketReader = null;
  #retirements = null;
  #frames = null;
  #program = null;
  #position = 0;
  #phase = "scene-start";
  #complete = false;
  #closing = false;
  #taken = false;
  #failure = null;
  constructor(source) {
    if (source.value.type !== "surface")
      throw new Error("Scene preparation requires a Surface component");
    this.#source = captureTypedUiPayload("component", source);
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  get completedBytes() {
    return this.#position;
  }
  get completedRecords() {
    return this.#entries?.size ?? 0;
  }
  #bytes() {
    const value = this.#source?.value;
    if (value?.type !== "surface")
      throw new Error("Scene preparation source is closed");
    return value.doc.bytes;
  }
  #byte() {
    if (this.#position >= this.#bytes().length)
      throw new Error("Truncated scene packet");
    return this.#bytes().byteAt(this.#position++);
  }
  #queue(owner) {
    this.#retirements = { owner, next: this.#retirements };
  }
  #drain() {
    const cell = this.#retirements;
    const result2 = cell.owner.advance(GRANT);
    if (result2.kind === "complete") {
      this.#retirements = cell.next;
      cell.next = null;
      cell.owner = null;
    }
    return result2.bytes + 32;
  }
  *#varint() {
    let value = 0n;
    for (let index = 0;index < 10; index++) {
      this.#phase = "scene-varint";
      const byte = this.#byte();
      const digit = byte & 127;
      if (index === 9 && digit > 1)
        throw new Error("Scene integer exceeds u64");
      if (index > 0 && !(byte & 128) && digit === 0)
        throw new Error("Noncanonical scene varint");
      value |= BigInt(digit) << BigInt(index * 7);
      yield 32;
      if (!(byte & 128))
        return value;
    }
    throw new Error("Scene varint exceeds ten bytes");
  }
  *#length() {
    const value = yield* this.#varint();
    if (value > BigInt(this.#bytes().length - this.#position))
      throw new Error("Scene length exceeds remaining bytes");
    return Number(value);
  }
  *#lookup(id2) {
    this.#entryReader = this.#entries.beginLookup(id2);
    let value = null;
    yield 64;
    for (;; ) {
      const current = this.#entryReader.advance(GRANT);
      if (current.kind === "value")
        value = current.value;
      yield current.bytes;
      if (current.kind === "complete")
        break;
    }
    this.#queue(this.#entryReader.beginClose());
    this.#entryReader = null;
    yield 64;
    while (this.#retirements)
      yield this.#drain();
    if (!value)
      throw new Error("Scene key arena is inconsistent");
    return value;
  }
  *#key(value, hash) {
    const frame = this.#frames;
    if (value.kind !== "text")
      throw new Error("Scene map key is not text");
    const bucket = frame.start * 4294967296 + hash;
    this.#phase = "scene-key-lookup";
    this.#bucketReader = this.#buckets.beginLookup(bucket);
    let head = null;
    yield 64;
    for (;; ) {
      const current = this.#bucketReader.advance(GRANT);
      if (current.kind === "value")
        head = current.value;
      yield current.bytes;
      if (current.kind === "complete")
        break;
    }
    this.#queue(this.#bucketReader.beginClose());
    this.#bucketReader = null;
    yield 64;
    while (this.#retirements)
      yield this.#drain();
    let previous = head;
    while (previous !== null) {
      const entry = yield* this.#lookup(previous);
      const candidate = entry.value;
      if (candidate.kind !== "text")
        throw new Error("Scene key arena kind mismatch");
      if (candidate.length === value.length) {
        let equal = true;
        this.#phase = "scene-key-compare";
        for (let offset = 0;offset < value.length; offset++) {
          const same = this.#bytes().byteAt(candidate.offset + offset) === this.#bytes().byteAt(value.offset + offset);
          yield 2;
          if (!same) {
            equal = false;
            break;
          }
        }
        if (equal)
          throw new Error("Duplicate scene map key");
      }
      previous = entry.collision;
      yield 32;
    }
    this.#phase = "scene-key-insert";
    this.#bucketEdit = this.#buckets.beginSet(bucket, value.start);
    yield 64;
    for (;; ) {
      const current = this.#bucketEdit.advance(GRANT);
      yield current.bytes;
      if (current.kind === "ready")
        break;
      if (current.kind === "rejected")
        throw new Error("Scene key ordinal exhausted");
    }
    const next = this.#bucketEdit.takeResult();
    this.#queue(this.#bucketEdit.beginClose());
    this.#bucketEdit = null;
    this.#queue(this.#buckets.beginClose());
    this.#buckets = next;
    yield 128;
    while (this.#retirements)
      yield this.#drain();
    return head;
  }
  *#save(value, hash = 0) {
    const parent = this.#frames;
    let collision = null;
    if (parent?.kind === "map" && parent.remaining % 2 === 0)
      collision = yield* this.#key(value, hash);
    if (parent?.kind === "variant" && parent.remaining === 2 && value.kind !== "text")
      throw new Error("Scene variant name is not text");
    this.#phase = "scene-record-insert";
    this.#entryEdit = this.#entries.beginSet(value.start, Object.freeze({ value: Object.freeze(value), collision }));
    yield 160;
    for (;; ) {
      const current = this.#entryEdit.advance(GRANT);
      yield current.bytes;
      if (current.kind === "ready")
        break;
      if (current.kind === "rejected")
        throw new Error("Scene record ordinal exhausted");
    }
    const next = this.#entryEdit.takeResult();
    this.#queue(this.#entryEdit.beginClose());
    this.#entryEdit = null;
    this.#queue(this.#entries.beginClose());
    this.#entries = next;
    yield 128;
    while (this.#retirements)
      yield this.#drain();
    if (parent)
      parent.remaining--;
  }
  *#parse() {
    let root = false;
    while (!root) {
      const frame = this.#frames;
      if (frame && frame.remaining === 0) {
        this.#frames = frame.parent;
        frame.parent = null;
        const value2 = frame.kind === "map" || frame.kind === "sequence" ? { kind: frame.kind, start: frame.start, end: this.#position, first: frame.first, count: frame.count } : { kind: frame.kind, start: frame.start, end: this.#position, first: frame.first };
        yield 96;
        yield* this.#save(value2);
        root = this.#frames === null;
        continue;
      }
      this.#phase = "scene-tag";
      const start = this.#position;
      const tag = this.#byte();
      yield 16;
      let value;
      let hash = 2166136261;
      if (tag === 0 || tag === 8)
        value = { kind: tag === 0 ? "unit" : "none", start, end: this.#position };
      else if (tag === 1 || tag === 2)
        value = { kind: "boolean", start, end: this.#position, value: tag === 2 };
      else if (tag === 3 || tag === 4 || tag === 11) {
        const raw = yield* this.#varint();
        if (tag === 11) {
          if (raw > 0x10ffffn || raw >= 0xd800n && raw <= 0xdfffn)
            throw new Error("Invalid scene character");
          value = { kind: "char", start, end: this.#position, value: String.fromCodePoint(Number(raw)) };
        } else
          value = { kind: "integer", start, end: this.#position, value: tag === 3 ? raw : raw >> 1n ^ -(raw & 1n) };
      } else if (tag === 5) {
        const bytes = new Uint8Array(8);
        yield 24;
        for (let index = 0;index < 8; index++) {
          bytes[index] = this.#byte();
          yield 1;
        }
        value = { kind: "float", start, end: this.#position, value: new DataView(bytes.buffer).getFloat64(0, true) };
      } else if (tag === 6 || tag === 7) {
        const length = yield* this.#length();
        const offset = this.#position;
        if (tag === 6) {
          const utf82 = new Utf8Scalar;
          this.#phase = "scene-text";
          yield 32;
          for (let index = 0;index < length; index++) {
            const byte = this.#byte();
            utf82.push(byte);
            hash = Math.imul(hash ^ byte, 16777619) >>> 0;
            yield 16;
          }
          if (!utf82.complete)
            throw new Error("Truncated scene Unicode scalar");
        } else {
          this.#position += length;
          yield 32;
        }
        value = { kind: tag === 6 ? "text" : "bytes", start, end: this.#position, offset, length };
      } else if (tag === 9 || tag === 10 || tag === 12 || tag === 13) {
        const count = tag === 10 || tag === 13 ? yield* this.#length() : 1;
        this.#frames = { start, kind: tag === 9 ? "some" : tag === 10 ? "sequence" : tag === 12 ? "variant" : "map", first: this.#position, count, remaining: tag === 12 ? 2 : tag === 13 ? count * 2 : count, parent: this.#frames };
        this.#phase = "scene-frame";
        yield 80;
        continue;
      } else
        throw new Error("Unknown scene packet tag");
      yield* this.#save(value, hash);
      root = this.#frames === null;
    }
    if (this.#position !== this.#bytes().length)
      throw new Error("Trailing scene packet bytes");
    this.#phase = "scene-prepare-close";
    this.#queue(this.#buckets.beginClose());
    this.#buckets = null;
    yield 64;
    while (this.#retirements)
      yield this.#drain();
  }
  advance(grant) {
    if (!admitted10(grant))
      return step4("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure)
      return step4("rejected", this.#phase);
    if (this.#complete)
      return step4("ready", "scene-ready");
    try {
      this.#program ??= this.#parse();
      const current = this.#program.next();
      if (current.done) {
        this.#program = null;
        this.#complete = true;
        return step4("ready", "scene-ready", 32);
      }
      return step4("pending", this.#phase, current.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Scene preparation failed";
      return step4("rejected", this.#phase, 128);
    }
  }
  takeResult() {
    if (!this.#complete || this.#closing || this.#failure || this.#taken || !this.#source || !this.#entries)
      return null;
    const document = ownDocument({ references: 1, index: this.#entries, source: this.#source });
    this.#entries = null;
    this.#source = null;
    this.#taken = true;
    return document;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted10(grant))
      return step4("blocked", "scene-close");
    if (!this.#closing)
      return step4("blocked", "scene-close-not-started");
    if (this.#program) {
      this.#program.return(undefined);
      this.#program = null;
      return step4("pending", "scene-program-close", 128);
    }
    if (this.#frames) {
      const frame = this.#frames;
      this.#frames = frame.parent;
      frame.parent = null;
      return step4("pending", "scene-frame-close", 80);
    }
    if (this.#entryReader) {
      this.#queue(this.#entryReader.beginClose());
      this.#entryReader = null;
      return step4("pending", "scene-reader-close", 64);
    }
    if (this.#bucketReader) {
      this.#queue(this.#bucketReader.beginClose());
      this.#bucketReader = null;
      return step4("pending", "scene-reader-close", 64);
    }
    if (this.#entryEdit) {
      this.#queue(this.#entryEdit.beginClose());
      this.#entryEdit = null;
      return step4("pending", "scene-edit-close", 64);
    }
    if (this.#bucketEdit) {
      this.#queue(this.#bucketEdit.beginClose());
      this.#bucketEdit = null;
      return step4("pending", "scene-edit-close", 64);
    }
    if (this.#retirements)
      return step4("pending", "scene-arena-close", this.#drain());
    if (this.#buckets) {
      this.#queue(this.#buckets.beginClose());
      this.#buckets = null;
      return step4("pending", "scene-key-index-close", 64);
    }
    if (this.#entries) {
      this.#queue(this.#entries.beginClose());
      this.#entries = null;
      return step4("pending", "scene-record-index-close", 64);
    }
    if (this.#source) {
      this.#queue(this.#source.beginClose());
      this.#source = null;
      return step4("pending", "scene-source-close", 64);
    }
    return step4("complete", "scene-close");
  }
  terminalIsEmpty() {
    return this.#closing && this.#program === null && this.#frames === null && this.#entryReader === null && this.#bucketReader === null && this.#entryEdit === null && this.#bucketEdit === null && this.#retirements === null && this.#buckets === null && this.#entries === null && this.#source === null;
  }
}
/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json */
var _catalog_default = {
  version: 1,
  surfaces: [
    { kind: "canvas-2d", schema: "canvas-2d@1", record: "Canvas2dScene" },
    { kind: "world-3d", schema: "world-3d@1", record: "World3dScene" },
    { kind: "node-graph", schema: "node-graph@1", record: "NodeGraphScene" },
    { kind: "text-editor", schema: "text-editor@1", record: "TextEditorScene" },
    { kind: "table", schema: "table@1", record: "TableScene" },
    { kind: "paint-2d", schema: "paint-2d@1", record: "Paint2dScene" },
    { kind: "virtual-file-system", schema: "virtual-file-system@1", record: "VirtualFileSystemScene" },
    { kind: "tiled-map", schema: "tiled-map@1", record: "TiledMapScene" },
    { kind: "board-2d", schema: "board-2d@1", record: "Board2dScene" },
    { kind: "icon-render", schema: "icon-render@1", record: "IconRenderScene" },
    { kind: "ink-canvas", schema: "ink-canvas@1", record: "InkCanvasScene" },
    { kind: "graph-timeline", schema: "graph-timeline@1", record: "GraphTimelineScene" },
    { kind: "block-list", schema: "block-list@1", record: "BlockListScene" },
    { kind: "diff-view", schema: "diff-view@1", record: "DiffViewScene" },
    { kind: "event-feed", schema: "event-feed@1", record: "EventFeedScene" }
  ],
  records: {
    Canvas2dScene: { fields: [["cameraX", "f64"], ["cameraY", "f64"], ["zoom", "f64"], ["layersJson", "text"], ["snapshot", "?#Canvas2dSnapshotLease"]] },
    World3dScene: { fields: [["snapshot", "?#World3dSnapshotLease"], ["cameraJson", "text"], ["meshesJson", "text"], ["instancesJson", "text"], ["selectionJson", "text"], ["vorticesJson", "?text"], ["attractionsJson", "?text"], ["targetVolumesJson", "?text"], ["referencesJson", "?text"], ["brushPreviewJson", "?text"], ["interactionJson", "?text"], ["engagementPreviewJson", "?text"], ["lodJson", "?text"], ["chunkingJson", "?text"], ["environmentJson", "?text"], ["frameJson", "?text"], ["fitJson", "?text"], ["terrainJson", "?text"], ["pointsJson", "?text"], ["statusJson", "?text"], ["domainId", "?text"], ["domainGranularityId", "?text"]] },
    NodeGraphScene: { fields: [["nodes", "[#NodeGraphNodeRecord]"], ["edges", "[#NodeGraphEdgeRecord]"], ["viewport", "?#NodeGraphViewport"], ["editable", "?bool"], ["operators", "[#NodeGraphOperatorRecord]"], ["findItems", "[#NodeGraphFindItem]"], ["selection", "[text]"], ["hover", "?#NodeGraphHover"], ["previewOffJson", "?text"], ["lodJson", "?text"], ["catalogueJson", "?text"], ["controlsJson", "?text"], ["clustersJson", "?text"], ["computingJson", "?text"], ["statusJson", "?text"], ["capabilitiesJson", "?text"], ["fixtureJson", "?text"], ["presencePeersJson", "?text"], ["evalJson", "?text"]], defaults: { nodes: [], edges: [], operators: [], findItems: [], selection: [] } },
    TextEditorScene: { fields: [["buffer", "text"], ["language", "?text"], ["selectionJson", "?text"], ["tokensJson", "?text"], ["diagnosticsJson", "?text"], ["completionsJson", "?text"], ["overlaysJson", "?text"], ["occurrencesJson", "?text"], ["placeholdersJson", "?text"], ["extraCaretsJson", "?text"], ["selectableSpansJson", "?text"], ["settingsJson", "?text"], ["cameraJson", "?text"], ["hoverJson", "?text"], ["newlineGatesJson", "?text"], ["renameJson", "?text"]] },
    TableScene: { fields: [["columnsJson", "text"], ["rowsJson", "text"], ["selectionJson", "?text"], ["rowDragMime", "?text"], ["dropActionJson", "?text"], ["sortJson", "?text"], ["domainId", "?text"]] },
    Paint2dScene: { fields: [["documentSyncJson", "text"], ["assetsJson", "text"], ["cameraJson", "text"], ["selectionJson", "text"], ["hoveredId", "?text"], ["activeUtility", "text"], ["brushSize", "f64"], ["brushOpacity", "f64"], ["viewMode", "text"], ["compositeViewportJson", "?text"]] },
    VirtualFileSystemScene: { fields: [["schemaJson", "text"], ["rowsJson", "text"], ["selectedRowIdsJson", "?text"], ["hoveredRowId", "?text"], ["emptyMessage", "?text"], ["dragDropEnabled", "?bool"]] },
    TiledMapScene: { fields: [["mapFixtureJson", "text"], ["cameraJson", "text"], ["renderMode", "text"], ["vectorStyle", "text"], ["lodMode", "text"], ["tileUrlTemplate", "text"], ["vectorTileUrlTemplate", "text"], ["layerVisibilityJson", "text"], ["layerStrokeScaleJson", "text"], ["selectionJson", "text"], ["hoverJson", "text"], ["selectionMethod", "text"], ["selectionMode", "text"]], defaults: { renderMode: "combined", vectorStyle: "colored", lodMode: "automatic", tileUrlTemplate: "/osm/{z}/{x}/{y}.png", vectorTileUrlTemplate: "/vt/{z}/{x}/{y}.pbf", layerVisibilityJson: "{}", layerStrokeScaleJson: "{}", selectionJson: "{}", hoverJson: "null", selectionMethod: "rectangle", selectionMode: "default" } },
    Board2dScene: { fields: [["fixtureJson", "text"], ["cameraJson", "text"], ["glyphCatalogsJson", "text"], ["selectionJson", "text"], ["interactive", "bool"], ["hoveredId", "?text"], ["activeUtility", "?text"], ["selectionMethod", "text"], ["gridSnapEnabled", "bool"], ["gridFactor", "f64"], ["suggestionOffset", "f64"], ["brushWeightsJson", "text"], ["placementCompatibilityJson", "text"], ["lodMode", "text"]] },
    IconRenderScene: { fields: [["requestJson", "text"], ["footer", "?text"], ["frameJson", "?text"]] },
    InkCanvasScene: { fields: [["documentJson", "text"], ["selectionJson", "text"], ["hoveredId", "?text"], ["activeUtility", "text"], ["viewMode", "text"], ["interactive", "bool"]], defaults: { selectionJson: "[]", interactive: false } },
    GraphTimelineScene: { fields: [["columnsJson", "text"]] },
    BlockListScene: { fields: [["stepsJson", "text"], ["paletteJson", "text"], ["selectedId", "?text"], ["draggingId", "?text"], ["domainId", "?text"]] },
    DiffViewScene: { fields: [["before", "text"], ["after", "text"], ["language", "?text"], ["mode", "?text"], ["domainId", "?text"]] },
    EventFeedScene: { fields: [["entriesJson", "text"], ["follow", "?bool"], ["activateAction", "?text"], ["domainId", "?text"]] },
    Canvas2dSnapshotLease: { fields: [["slot", "u8"], ["epoch", "u64"], ["revision", "u64"], ["generation", "u64"], ["pageCount", "u8"], ["byteCount", "u32"]] },
    World3dSnapshotLease: { fields: [["slot", "u8"], ["epoch", "u64"], ["revision", "u64"], ["generation", "u64"], ["pageCount", "u16"], ["itemCount", "u32"], ["byteCount", "u32"]] },
    NodeGraphPortRecord: { fields: [["id", "text"], ["label", "?text"], ["code", "?text"], ["abbreviation", "?text"], ["fullName", "?text"], ["resourceKind", "?text"]] },
    NodeGraphNodeRecord: { fields: [["id", "text"], ["label", "?text"], ["x", "f64"], ["y", "f64"], ["width", "f64"], ["height", "f64"], ["inputs", "[#NodeGraphPortRecord]"], ["outputs", "[#NodeGraphPortRecord]"], ["instanceId", "?text"], ["pluginId", "?text"], ["appId", "?text"], ["icon", "?text"]], defaults: { inputs: [], outputs: [] } },
    NodeGraphEdgeRecord: { fields: [["id", "text"], ["sourceNodeId", "text"], ["sourcePortId", "text"], ["targetNodeId", "text"], ["targetPortId", "text"], ["label", "?text"]] },
    NodeGraphViewport: { fields: [["x", "f64"], ["y", "f64"], ["zoom", "f64"]], defaults: { x: 0, y: 0, zoom: 1 } },
    NodeGraphFindItem: { fields: [["id", "text"], ["label", "text"], ["category", "text"]] },
    NodeGraphHover: { fields: [["nodeId", "?text"]] },
    NodeGraphOperatorVariadicRecord: { fields: [["slotKey", "text"], ["min", "usize"], ["max", "?usize"]] },
    NodeGraphOperatorChannelRecord: { fields: [["code", "text"], ["abbreviation", "text"], ["name", "text"], ["fullName", "text"], ["operators", "[text]"], ["defaultJson", "?text"], ["label", "?text"], ["cardinality", "text"]], defaults: { operators: [], cardinality: "" } },
    NodeGraphOperatorRecord: { fields: [["id", "text"], ["extension", "text"], ["name", "text"], ["abbreviation", "text"], ["icon", "text"], ["summary", "text"], ["inputs", "[#NodeGraphOperatorChannelRecord]"], ["outputs", "[#NodeGraphOperatorChannelRecord]"], ["variadicInput", "?#NodeGraphOperatorVariadicRecord"], ["variadicOutput", "?#NodeGraphOperatorVariadicRecord"], ["group", "[text]"]], defaults: { inputs: [], outputs: [], group: [] } }
  }
};

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️.ts */
var SPECS = _catalog_default.records;
var GRANT2 = Object.freeze({ maxItems: 1, maxBytes: 4096 });
var MINT5 = Object.freeze({});
var admitted11 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step5 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var ownDocument2;
var ownReader2;
var ownRetirement2;
function freezeCatalog() {
  for (const spec of Object.values(SPECS)) {
    if (spec.fields.length > 32)
      throw new Error("Static scene schema exceeds fixed field metadata capacity");
    for (const field of spec.fields) {
      if (field.length !== 2 || field[0].length > 64 || field[1].length > 64)
        throw new Error("Static scene schema field exceeds its metadata bound");
      Object.freeze(field);
    }
    if (spec.defaults) {
      for (const value of Object.values(spec.defaults))
        if (Array.isArray(value))
          Object.freeze(value);
      Object.freeze(spec.defaults);
    }
    Object.freeze(spec.fields);
    Object.freeze(spec);
  }
  for (const surface of _catalog_default.surfaces)
    Object.freeze(surface);
  Object.freeze(_catalog_default.surfaces);
  Object.freeze(_catalog_default.records);
  Object.freeze(_catalog_default);
}
freezeCatalog();

class OwnedUiPreparedScene {
  #root;
  constructor(mint, root) {
    if (mint !== MINT5)
      throw new Error("Prepared scene requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    ownDocument2 = (root) => new OwnedUiPreparedScene(MINT5, root);
  }
  #live() {
    if (!this.#root)
      throw new Error("Prepared scene is closed");
    return this.#root;
  }
  get kind() {
    return this.#live().source.kind;
  }
  get schema() {
    return this.#live().source.schema;
  }
  capture() {
    const root = this.#live();
    if (root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Prepared scene reference overflow");
    root.references++;
    return ownDocument2(root);
  }
  beginRecord(source = 0) {
    const root = this.#live();
    if (root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Prepared scene reference overflow");
    const reader = root.records.beginLookup(source);
    root.references++;
    return ownReader2(root, reader);
  }
  beginText(source) {
    return this.#live().source.beginText(source);
  }
  beginTextBytes(source, offset = 0, length) {
    return this.#live().source.beginTextBytes(source, offset, length);
  }
  beginValue(source) {
    return this.#live().source.beginRead(source);
  }
  beginClose() {
    const root = this.#live();
    this.#root = null;
    return ownRetirement2(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class OwnedUiPreparedSceneReader {
  #root;
  #reader;
  constructor(mint, root, reader) {
    if (mint !== MINT5)
      throw new Error("Prepared scene reader requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    ownReader2 = (root, reader) => new OwnedUiPreparedSceneReader(MINT5, root, reader);
  }
  advance(grant) {
    if (!admitted11(grant))
      return step5("blocked", "prepared-scene-read");
    if (!this.#reader)
      return step5("rejected", "prepared-scene-read");
    const current = this.#reader.advance(GRANT2);
    return current.kind === "value" ? { ...current, bytes: current.bytes + 64 } : { ...current, phase: "prepared-scene-read" };
  }
  beginClose() {
    if (!this.#root || !this.#reader)
      throw new Error("Prepared scene reader is closed");
    const result2 = ownRetirement2(this.#root, this.#reader.beginClose());
    this.#root = null;
    this.#reader = null;
    return result2;
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null;
  }
}

class OwnedUiPreparedSceneRetirement {
  #root;
  #reader;
  #records = null;
  #source = null;
  #released = false;
  constructor(mint, root, reader) {
    if (mint !== MINT5)
      throw new Error("Prepared scene retirement requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    ownRetirement2 = (root, reader = null) => new OwnedUiPreparedSceneRetirement(MINT5, root, reader);
  }
  advance(grant) {
    if (!admitted11(grant))
      return step5("blocked", "prepared-scene-close");
    if (this.#reader) {
      const current = this.#reader.advance(GRANT2);
      if (current.kind === "complete")
        this.#reader = null;
      return step5("pending", "prepared-scene-reader-close", current.bytes + (current.kind === "retired" ? 3072 : 0));
    }
    if (this.#root && !this.#released) {
      this.#released = true;
      if (--this.#root.references)
        this.#root = null;
      return step5("pending", "prepared-scene-release", 32);
    }
    if (this.#records) {
      const current = this.#records.advance(GRANT2);
      if (current.kind === "complete")
        this.#records = null;
      return step5("pending", "prepared-scene-record-close", current.bytes + (current.kind === "retired" ? 3072 : 0));
    }
    if (this.#root?.records) {
      this.#records = this.#root.records.beginClose();
      this.#root.records = null;
      return step5("pending", "prepared-scene-record-close", 64);
    }
    if (this.#source) {
      const current = this.#source.advance(GRANT2);
      if (current.kind === "complete")
        this.#source = null;
      return step5("pending", "prepared-scene-source-close", current.bytes);
    }
    if (this.#root?.source) {
      this.#source = this.#root.source.beginClose();
      this.#root.source = null;
      return step5("pending", "prepared-scene-source-close", 64);
    }
    this.#root = null;
    return step5("complete", "prepared-scene-close");
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null && this.#records === null && this.#source === null;
  }
}

class OwnedUiSceneProjectionCursor {
  #usizeBits;
  #source;
  #records = NumericIndex.empty();
  #reader = null;
  #edit = null;
  #retirements = null;
  #tasks = null;
  #program = null;
  #closing = false;
  #complete = false;
  #taken = false;
  #failure = null;
  #phase = "scene-schema";
  constructor(source, profile) {
    const bits = profile.usizeBits;
    if (bits !== 32 && bits !== 64)
      throw new Error("Scene projection requires an owning host width");
    this.#usizeBits = bits;
    this.#source = source.capture();
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  #queue(owner) {
    this.#retirements = { owner, next: this.#retirements };
  }
  #push(task) {
    task.next = this.#tasks;
    this.#tasks = task;
  }
  #drain() {
    const link = this.#retirements;
    const current = link.owner.advance(GRANT2);
    if (current.kind === "complete") {
      this.#retirements = link.next;
      link.next = null;
      link.owner = null;
    }
    return current.bytes + (current.kind === "retired" ? 3072 : 32);
  }
  *#lookup(source) {
    this.#reader = this.#source.beginRead(source);
    let value = null;
    yield 64;
    for (;; ) {
      const current = this.#reader.advance(GRANT2);
      if (current.kind === "value")
        value = current.value;
      yield current.bytes;
      if (current.kind === "complete")
        break;
      if (current.kind === "rejected")
        throw new Error("scene-read-failed");
    }
    this.#queue(this.#reader.beginClose());
    this.#reader = null;
    yield 64;
    while (this.#retirements)
      yield this.#drain();
    if (!value)
      throw new Error("scene-record-missing");
    return value;
  }
  *#fieldName(value) {
    if (value.kind !== "text")
      throw new Error("scene-field-name-invalid");
    if (value.length > 64)
      return null;
    this.#reader = this.#source.beginText(value.start);
    let name = "";
    yield 64;
    for (;; ) {
      const current = this.#reader.advance(GRANT2);
      if (current.kind === "text")
        name += current.value;
      yield current.bytes;
      if (current.kind === "complete")
        break;
      if (current.kind === "rejected")
        throw new Error("scene-field-name-invalid");
    }
    this.#queue(this.#reader.beginClose());
    this.#reader = null;
    yield 64;
    while (this.#retirements)
      yield this.#drain();
    return name;
  }
  *#save(record) {
    this.#phase = "scene-typed-record";
    this.#edit = this.#records.beginSet(record.source, record);
    yield 64;
    for (;; ) {
      const current = this.#edit.advance(GRANT2);
      yield current.bytes + (current.kind === "retired" ? 3072 : 0);
      if (current.kind === "ready")
        break;
      if (current.kind === "rejected")
        throw new Error("scene-typed-index-exhausted");
    }
    const next = this.#edit.takeResult();
    this.#queue(this.#edit.beginClose());
    this.#edit = null;
    this.#queue(this.#records.beginClose());
    this.#records = next;
    yield 128;
    while (this.#retirements)
      yield this.#drain();
  }
  *#value(task) {
    const value = yield* this.#lookup(task.source);
    const type = task.type;
    if (type.startsWith("?")) {
      if (value.kind === "none")
        return;
      if (value.kind !== "some")
        throw new Error("scene-option-tag-invalid");
      this.#push({ kind: "value", source: value.first, type: type.slice(1), next: null });
      yield 64;
      return;
    }
    if (type.startsWith("[")) {
      if (value.kind !== "sequence")
        throw new Error("scene-sequence-type-invalid");
      this.#push({ kind: "sequence", type: type.slice(1, -1), remaining: value.count, position: value.first, next: null });
      yield 80;
      return;
    }
    if (type.startsWith("#")) {
      const name = type.slice(1);
      const spec = SPECS[name];
      if (!spec || value.kind !== "map")
        throw new Error("scene-record-type-invalid");
      this.#push({ kind: "record", source: value.start, name, spec, fields: new Array(spec.fields.length).fill(null), remaining: value.count, position: value.first, missing: 0, next: null });
      yield 128 + spec.fields.length * 8;
      return;
    }
    if (type === "text" && value.kind === "text")
      return;
    if (type === "bool" && value.kind === "boolean")
      return;
    if (type === "f64" && value.kind === "float" && Number.isFinite(value.value))
      return;
    if (value.kind === "integer") {
      const maximum = type === "u8" ? 255n : type === "u16" ? 65535n : type === "u32" || type === "usize" && this.#usizeBits === 32 ? 4294967295n : type === "u64" || type === "usize" ? 18446744073709551615n : -1n;
      if (value.value >= 0n && value.value <= maximum)
        return;
    }
    throw new Error("scene-field-type-invalid");
  }
  *#record(task) {
    if (task.remaining) {
      const key = yield* this.#lookup(task.position);
      const name = yield* this.#fieldName(key);
      const value = yield* this.#lookup(key.end);
      task.position = value.end;
      task.remaining--;
      let field = -1;
      for (let index = 0;index < task.spec.fields.length; index++) {
        if (task.spec.fields[index][0] === name)
          field = index;
        yield 64;
      }
      this.#push(task);
      if (field >= 0) {
        const spec = task.spec.fields[field];
        task.fields[field] = Object.freeze({ name: spec[0], type: spec[1], source: value.start, literal: null });
        this.#push({ kind: "value", source: value.start, type: spec[1], next: null });
      }
      yield 160;
      return;
    }
    if (task.missing < task.spec.fields.length) {
      const index = task.missing++;
      const field = task.spec.fields[index];
      if (!task.fields[index]) {
        const name = field[0];
        const type = field[1];
        const literal = task.spec.defaults && Object.hasOwn(task.spec.defaults, name) ? task.spec.defaults[name] : null;
        if (literal === null && !type.startsWith("?"))
          throw new Error("scene-required-field-missing");
        task.fields[index] = Object.freeze({ name, type, source: null, literal });
      }
      this.#push(task);
      yield 128;
      return;
    }
    const fields = [];
    for (const field of task.fields) {
      if (!field)
        throw new Error("scene-field-normalization-incomplete");
      fields.push(field);
    }
    yield 64 + fields.length * 16;
    yield* this.#save(Object.freeze({ schema: task.name, source: task.source, fields: Object.freeze(fields) }));
  }
  *#prepare() {
    let record = null;
    for (const surface of _catalog_default.surfaces) {
      if (surface.kind === this.#source.kind && surface.schema === this.#source.schema)
        record = surface.record;
      yield 1152;
    }
    if (!record)
      throw new Error("unsupported-scene-schema");
    this.#push({ kind: "value", source: 0, type: `#${record}`, next: null });
    yield 96;
    while (this.#tasks) {
      const task = this.#tasks;
      this.#tasks = task.next;
      task.next = null;
      this.#phase = "scene-typed-validate";
      yield 64;
      if (task.kind === "value")
        yield* this.#value(task);
      else if (task.kind === "record")
        yield* this.#record(task);
      else if (task.remaining) {
        const value = yield* this.#lookup(task.position);
        task.position = value.end;
        task.remaining--;
        this.#push(task);
        this.#push({ kind: "value", source: value.start, type: task.type, next: null });
        yield 128;
      }
    }
  }
  advance(grant) {
    if (!admitted11(grant))
      return step5("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure)
      return step5("rejected", this.#phase);
    if (this.#complete)
      return step5("ready", "scene-typed-ready");
    try {
      this.#program ??= this.#prepare();
      const current = this.#program.next();
      if (current.done) {
        this.#complete = true;
        this.#program = null;
        return step5("ready", "scene-typed-ready", 32);
      }
      return step5("pending", this.#phase, current.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "scene-projection-failed";
      return step5("rejected", this.#phase, 128);
    }
  }
  takeResult() {
    if (!this.#complete || this.#closing || this.#failure || this.#taken || !this.#source || !this.#records)
      return null;
    const document = ownDocument2({ references: 1, source: this.#source, records: this.#records });
    this.#source = null;
    this.#records = null;
    this.#taken = true;
    return document;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted11(grant) || !this.#closing)
      return step5("blocked", "scene-typed-close");
    if (this.#program) {
      this.#program.return(undefined);
      this.#program = null;
      return step5("pending", "scene-typed-program-close", 3072);
    }
    if (this.#tasks) {
      const task = this.#tasks;
      this.#tasks = task.next;
      task.next = null;
      return step5("pending", "scene-typed-frame-close", 3072);
    }
    if (this.#reader) {
      this.#queue(this.#reader.beginClose());
      this.#reader = null;
      return step5("pending", "scene-typed-reader-close", 64);
    }
    if (this.#edit) {
      this.#queue(this.#edit.beginClose());
      this.#edit = null;
      return step5("pending", "scene-typed-edit-close", 64);
    }
    if (this.#retirements)
      return step5("pending", "scene-typed-retirement", this.#drain());
    if (this.#records) {
      this.#queue(this.#records.beginClose());
      this.#records = null;
      return step5("pending", "scene-typed-records-close", 64);
    }
    if (this.#source) {
      this.#queue(this.#source.beginClose());
      this.#source = null;
      return step5("pending", "scene-typed-source-close", 64);
    }
    return step5("complete", "scene-typed-close");
  }
  terminalIsEmpty() {
    return this.#closing && this.#program === null && this.#tasks === null && this.#reader === null && this.#edit === null && this.#retirements === null && this.#source === null && this.#records === null;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️.ts */
var GRANT3 = Object.freeze({ maxItems: 1, maxBytes: 4096 });
var MINT6 = Object.freeze({});
var admitted12 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step6 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var own2;
var retire2;
var exact;

class OwnedUiSceneBinding {
  #root;
  constructor(mint, root) {
    if (mint !== MINT6)
      throw new Error("Scene binding requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    own2 = (root) => new OwnedUiSceneBinding(MINT6, root);
    exact = (owner) => owner.#live();
  }
  #live() {
    if (!this.#root)
      throw new Error("Scene binding is closed");
    return this.#root;
  }
  get value() {
    return this.#live().node.value;
  }
  get diagnostic() {
    return this.#live().diagnostic;
  }
  get prepared() {
    return this.#live().scene !== null;
  }
  capture() {
    const root = this.#live();
    if (root.references === Number.MAX_SAFE_INTEGER)
      throw new Error("Scene binding reference overflow");
    root.references++;
    return own2(root);
  }
  beginRecord(source = 0) {
    return this.#live().scene?.beginRecord(source) ?? null;
  }
  beginText(source) {
    return this.#live().scene?.beginText(source) ?? null;
  }
  beginValue(source) {
    return this.#live().scene?.beginValue(source) ?? null;
  }
  beginClose() {
    const root = this.#live();
    this.#root = null;
    return retire2(root);
  }
  terminalIsEmpty() {
    return this.#root === null;
  }
}

class OwnedUiSceneBindingRetirement {
  #root;
  #active = null;
  #released = false;
  constructor(mint, root) {
    if (mint !== MINT6)
      throw new Error("Scene binding retirement requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    retire2 = (root) => new OwnedUiSceneBindingRetirement(MINT6, root);
  }
  advance(grant) {
    if (!admitted12(grant))
      return step6("blocked", "scene-binding-close");
    if (this.#active) {
      const current = this.#active.advance(GRANT3);
      if (current.kind === "complete")
        this.#active = null;
      return { ...current, kind: "pending" };
    }
    if (!this.#root)
      return step6("complete", "scene-binding-close");
    if (!this.#released) {
      this.#released = true;
      if (--this.#root.references)
        this.#root = null;
      return step6("pending", "scene-binding-release", 32);
    }
    if (this.#root.scene) {
      this.#active = this.#root.scene.beginClose();
      this.#root.scene = null;
      return step6("pending", "scene-binding-projection-close", 64);
    }
    if (this.#root.node) {
      this.#active = this.#root.node.beginClose();
      this.#root.node = null;
      return step6("pending", "scene-binding-node-close", 64);
    }
    this.#root.diagnostic = null;
    this.#root = null;
    return step6("complete", "scene-binding-close", 64);
  }
  terminalIsEmpty() {
    return this.#root === null && this.#active === null;
  }
}

class OwnedUiSceneBindingCursor {
  #profile;
  #node;
  #previous = null;
  #component = null;
  #parser = null;
  #raw = null;
  #projection = null;
  #scene = null;
  #retirements = null;
  #diagnostic = null;
  #program = null;
  #started = false;
  #ready = false;
  #closing = false;
  #taken = false;
  #failure = null;
  #phase = "scene-binding-admission";
  constructor(node, profile) {
    const bits = profile.usizeBits;
    if (bits !== 32 && bits !== 64)
      throw new Error("Scene binding requires an owning host width");
    this.#profile = Object.freeze({ usizeBits: bits });
    this.#node = node.capture();
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  considerPrevious(previous) {
    if (this.#started || this.#closing || this.#previous)
      throw new Error("Scene binding reuse admission is closed");
    this.#previous = previous.capture();
  }
  #queue(owner) {
    this.#retirements = { owner, next: this.#retirements };
  }
  #drain() {
    const link = this.#retirements;
    if (!link.owner) {
      this.#retirements = link.next;
      link.next = null;
      return 32;
    }
    const current = link.owner.advance(GRANT3);
    if (current.kind === "complete")
      link.owner = null;
    return current.bytes;
  }
  *#prepare() {
    this.#started = true;
    if (this.#previous) {
      const previous = exact(this.#previous);
      if (previous.usizeBits === this.#profile.usizeBits && previous.node.value.component === this.#node.value.component) {
        this.#scene = previous.scene?.capture() ?? null;
        this.#diagnostic = previous.diagnostic;
      }
      this.#queue(this.#previous.beginClose());
      this.#previous = null;
      yield 128;
      while (this.#retirements)
        yield this.#drain();
      if (this.#scene || this.#diagnostic)
        return;
    }
    if (this.#node.value.component.type !== "surface")
      return;
    this.#component = this.#node.captureComponent();
    yield 64;
    this.#parser = new OwnedUiSceneCursor(this.#component);
    this.#queue(this.#component.beginClose());
    this.#component = null;
    yield 128;
    while (this.#retirements)
      yield this.#drain();
    this.#phase = "scene-binding-packet";
    for (;; ) {
      const current = this.#parser.advance(GRANT3);
      yield current.bytes;
      if (current.kind === "ready") {
        this.#raw = this.#parser.takeResult();
        break;
      }
      if (current.kind === "rejected") {
        this.#diagnostic = Object.freeze({ code: "invalid-scene-packet" });
        break;
      }
    }
    this.#parser.beginClose();
    yield 32;
    while (this.#parser) {
      const current = this.#parser.closeStep(GRANT3);
      if (current.kind === "complete")
        this.#parser = null;
      yield current.bytes;
    }
    if (!this.#raw)
      return;
    this.#projection = new OwnedUiSceneProjectionCursor(this.#raw, this.#profile);
    this.#queue(this.#raw.beginClose());
    this.#raw = null;
    yield 128;
    while (this.#retirements)
      yield this.#drain();
    this.#phase = "scene-binding-fields";
    for (;; ) {
      const current = this.#projection.advance(GRANT3);
      yield current.bytes;
      if (current.kind === "ready") {
        this.#scene = this.#projection.takeResult();
        break;
      }
      if (current.kind === "rejected") {
        this.#diagnostic = Object.freeze({ code: this.#projection.failure === "unsupported-scene-schema" ? "unsupported-scene-schema" : "invalid-scene-fields" });
        break;
      }
    }
    this.#projection.beginClose();
    yield 32;
    while (this.#projection) {
      const current = this.#projection.closeStep(GRANT3);
      if (current.kind === "complete")
        this.#projection = null;
      yield current.bytes;
    }
  }
  advance(grant) {
    if (!admitted12(grant))
      return step6("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure)
      return step6("rejected", this.#phase);
    if (this.#ready)
      return step6("ready", "scene-binding-ready");
    try {
      this.#program ??= this.#prepare();
      const current = this.#program.next();
      if (current.done) {
        this.#program = null;
        this.#ready = true;
        return step6("ready", "scene-binding-ready", 32);
      }
      return step6("pending", this.#phase, current.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "scene-binding-failed";
      return step6("rejected", this.#phase, 128);
    }
  }
  takeResult() {
    if (!this.#ready || this.#taken || this.#closing || this.#failure || !this.#node)
      return null;
    const result2 = own2({ references: 1, usizeBits: this.#profile.usizeBits, node: this.#node, scene: this.#scene, diagnostic: this.#diagnostic });
    this.#node = null;
    this.#scene = null;
    this.#diagnostic = null;
    this.#taken = true;
    return result2;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!this.#closing || !admitted12(grant))
      return step6("blocked", "scene-binding-close");
    if (this.#program) {
      this.#program.return(undefined);
      this.#program = null;
      return step6("pending", "scene-binding-program-close", 128);
    }
    if (this.#parser) {
      this.#parser.beginClose();
      const current = this.#parser.closeStep(GRANT3);
      if (current.kind === "complete")
        this.#parser = null;
      return { ...current, kind: "pending" };
    }
    if (this.#projection) {
      this.#projection.beginClose();
      const current = this.#projection.closeStep(GRANT3);
      if (current.kind === "complete")
        this.#projection = null;
      return { ...current, kind: "pending" };
    }
    if (this.#retirements)
      return step6("pending", "scene-binding-retirement", this.#drain());
    if (this.#raw) {
      this.#queue(this.#raw.beginClose());
      this.#raw = null;
      return step6("pending", "scene-binding-raw-close", 64);
    }
    if (this.#component) {
      this.#queue(this.#component.beginClose());
      this.#component = null;
      return step6("pending", "scene-binding-component-close", 64);
    }
    if (this.#scene) {
      this.#queue(this.#scene.beginClose());
      this.#scene = null;
      return step6("pending", "scene-binding-scene-close", 64);
    }
    if (this.#previous) {
      this.#queue(this.#previous.beginClose());
      this.#previous = null;
      return step6("pending", "scene-binding-previous-close", 64);
    }
    if (this.#node) {
      this.#queue(this.#node.beginClose());
      this.#node = null;
      return step6("pending", "scene-binding-node-close", 64);
    }
    this.#diagnostic = null;
    return step6("complete", "scene-binding-close");
  }
  terminalIsEmpty() {
    return this.#closing && this.#program === null && this.#parser === null && this.#projection === null && this.#retirements === null && this.#raw === null && this.#component === null && this.#scene === null && this.#previous === null && this.#node === null && this.#diagnostic === null;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️.ts */
var MINT7 = Object.freeze({});
var admitted13 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step7 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var own3;
var edit;
var read;
var retire3;

class OwnedUiSceneBindingIndex {
  #index;
  constructor(mint, index) {
    if (mint !== MINT7)
      throw new Error("Binding index requires exact mint authority");
    this.#index = index;
    Object.freeze(this);
  }
  static {
    own3 = (index) => new OwnedUiSceneBindingIndex(MINT7, index);
  }
  static empty() {
    return own3(NumericIndex.empty());
  }
  #live() {
    if (!this.#index)
      throw new Error("Binding index is closed");
    return this.#index;
  }
  get size() {
    return this.#live().size;
  }
  capture() {
    return own3(this.#live().capture());
  }
  beginSet(binding) {
    const index = this.#live();
    const id2 = binding.value.id;
    index.assertCaptureCapacity();
    return edit(index.beginSet(id2, binding.capture()));
  }
  beginRemove(id2) {
    return edit(this.#live().beginRemove(id2));
  }
  beginLookup(id2) {
    return read(this.#live().beginLookup(id2));
  }
  beginRead() {
    return read(this.#live().beginRead());
  }
  beginClose() {
    const index = this.#live();
    this.#index = null;
    return retire3(index.beginClose());
  }
  terminalIsEmpty() {
    return this.#index === null;
  }
}

class OwnedUiSceneBindingIndexRetirement {
  #owner;
  #active;
  constructor(mint, owner, active) {
    if (mint !== MINT7)
      throw new Error("Binding index retirement requires exact mint authority");
    this.#owner = owner;
    this.#active = active;
    Object.freeze(this);
  }
  static {
    retire3 = (owner, active = null) => new OwnedUiSceneBindingIndexRetirement(MINT7, owner, active);
  }
  advance(grant) {
    if (!admitted13(grant))
      return step7("blocked", "binding-index-close");
    if (this.#active) {
      const current2 = this.#active.advance(grant);
      if (current2.kind === "complete")
        this.#active = null;
      return { ...current2, kind: "pending" };
    }
    if (!this.#owner)
      return step7("complete", "binding-index-close");
    const current = this.#owner.advance(grant);
    if (current.kind === "retired") {
      this.#active = current.value.beginClose();
      return step7("pending", "binding-index-entry-close", current.bytes + 64);
    }
    if (current.kind === "complete")
      this.#owner = null;
    return { kind: "pending", phase: "binding-index-close", items: current.items, bytes: current.bytes };
  }
  terminalIsEmpty() {
    return this.#owner === null && this.#active === null;
  }
}

class OwnedUiSceneBindingIndexEdit {
  #owner;
  #active = null;
  #failure = null;
  constructor(mint, owner) {
    if (mint !== MINT7)
      throw new Error("Binding edit requires exact mint authority");
    this.#owner = owner;
    Object.freeze(this);
  }
  static {
    edit = (owner) => new OwnedUiSceneBindingIndexEdit(MINT7, owner);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted13(grant))
      return step7("blocked", "binding-index-edit");
    if (!this.#owner)
      throw new Error("Binding edit is closed");
    if (this.#active) {
      const current2 = this.#active.advance(grant);
      if (current2.kind === "complete")
        this.#active = null;
      return { ...current2, kind: "pending" };
    }
    const current = this.#owner.advance(grant);
    if (current.kind === "retired") {
      this.#active = current.value.beginClose();
      return step7("pending", "binding-index-entry-close", current.bytes + 64);
    }
    if (current.kind === "rejected")
      this.#failure = current.reason;
    return { kind: current.kind, phase: "binding-index-edit", items: current.items, bytes: current.bytes };
  }
  takeResult() {
    if (!this.#owner || this.#active)
      return null;
    const result2 = this.#owner.takeResult();
    return result2 ? own3(result2) : null;
  }
  beginClose() {
    if (!this.#owner)
      throw new Error("Binding edit is closed");
    const result2 = retire3(this.#owner.beginClose(), this.#active);
    this.#owner = null;
    this.#active = null;
    return result2;
  }
  terminalIsEmpty() {
    return this.#owner === null && this.#active === null;
  }
}

class OwnedUiSceneBindingIndexReader {
  #owner;
  constructor(mint, owner) {
    if (mint !== MINT7)
      throw new Error("Binding reader requires exact mint authority");
    this.#owner = owner;
    Object.freeze(this);
  }
  static {
    read = (owner) => new OwnedUiSceneBindingIndexReader(MINT7, owner);
  }
  advance(grant) {
    if (!admitted13(grant))
      return step7("blocked", "binding-index-read");
    if (!this.#owner)
      throw new Error("Binding reader is closed");
    const current = this.#owner.advance(grant);
    if (current.kind === "value")
      return { ...current, value: current.value.capture(), bytes: current.bytes + 64 };
    return { ...current, phase: "binding-index-read" };
  }
  beginClose() {
    if (!this.#owner)
      throw new Error("Binding reader is closed");
    const result2 = retire3(this.#owner.beginClose());
    this.#owner = null;
    return result2;
  }
  terminalIsEmpty() {
    return this.#owner === null;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts */
var ISSUED_MINT = Object.freeze({});
var admitted14 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var state5 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function natural2(value) {
  if (!Number.isSafeInteger(value) || value < 0)
    throw new RangeError("Invalid owned UI read identity");
  return value === 0 ? 0 : value;
}
var issue;
var release;
var retireIssued;
var ownSceneRecord;
var ownSceneText;
var retireScene;
var recordLease;
var textLease;
var snapshotLease;
var commitState;
var createCommit;
var pendingCommit;

class OwnedUiReadCommit {
  #state;
  constructor(mint, owner, version) {
    if (mint !== ISSUED_MINT)
      throw new Error("Read commit requires exact mint authority");
    this.#state = { owner, version, status: "pending" };
    Object.freeze(this);
  }
  static {
    createCommit = (owner, version) => new OwnedUiReadCommit(ISSUED_MINT, owner, version);
    commitState = (commit) => commit.#state;
  }
}

class OwnedUiReadPublication {
  #version;
  #pending = null;
  constructor(version) {
    this.#version = natural2(version);
    Object.freeze(this);
  }
  static {
    pendingCommit = (owner, commit) => owner.#pending === commit;
  }
  get version() {
    return this.#version;
  }
  begin(version) {
    const next = natural2(version);
    if (next <= this.#version || this.#pending)
      throw new Error("Owned UI read publication is not available");
    this.#pending = createCommit(this, next);
    return this.#pending;
  }
  publish(commit) {
    if (this.#pending === null || commit !== this.#pending)
      return false;
    const exact2 = commitState(commit);
    exact2.status = "published";
    this.#version = exact2.version;
    this.#pending = null;
    return true;
  }
  cancel(commit) {
    if (this.#pending === null || commit !== this.#pending)
      return false;
    commitState(commit).status = "cancelled";
    this.#pending = null;
    return true;
  }
}

class OwnedUiNodeReadSnapshot {
  #root;
  version;
  constructor(mint, lease, version, node) {
    this.version = version;
    if (mint !== ISSUED_MINT)
      throw new Error("Issued snapshot requires exact mint authority");
    this.#root = { lease, node, active: true, readers: 0 };
    Object.freeze(this);
  }
  static {
    issue = (lease, version, node) => new OwnedUiNodeReadSnapshot(ISSUED_MINT, lease, version, node);
    release = (snapshot) => {
      if (!snapshot.#root.active)
        throw new Error("Owned UI read snapshot already retired");
      snapshot.#root.active = false;
      return retireIssued(snapshot.#root);
    };
    snapshotLease = (snapshot) => snapshot.#root.active ? snapshot.#root.lease : null;
  }
  #live() {
    if (!this.#root.active)
      throw new Error("Owned UI read snapshot is retired");
    return this.#root;
  }
  get record() {
    return this.#live().node?.value;
  }
  get sceneDiagnostic() {
    const node = this.#live().node;
    return node instanceof OwnedUiSceneBinding ? node.diagnostic : null;
  }
  get hasPreparedScene() {
    const node = this.#live().node;
    return node instanceof OwnedUiSceneBinding && node.prepared;
  }
  beginSceneRecord(source = 0) {
    const root = this.#live();
    if (root.readers === 2 || !(root.node instanceof OwnedUiSceneBinding))
      return null;
    const reader = root.node.beginRecord(source);
    if (!reader)
      return null;
    root.readers++;
    return ownSceneRecord(root, reader);
  }
  beginSceneText(source) {
    const root = this.#live();
    if (root.readers === 2 || !(root.node instanceof OwnedUiSceneBinding))
      return null;
    const reader = root.node.beginText(source);
    if (!reader)
      return null;
    root.readers++;
    return ownSceneText(root, reader);
  }
}

class OwnedUiIssuedSceneRecord {
  #root;
  #reader;
  constructor(mint, root, reader) {
    if (mint !== ISSUED_MINT)
      throw new Error("Issued scene reader requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    ownSceneRecord = (root, reader) => new OwnedUiIssuedSceneRecord(ISSUED_MINT, root, reader);
    recordLease = (reader) => reader.#root?.lease ?? null;
  }
  advance(grant) {
    if (!this.#reader)
      throw new Error("Issued scene reader is closed");
    return this.#reader.advance(grant);
  }
  beginClose() {
    if (!this.#reader || !this.#root)
      throw new Error("Issued scene reader is closed");
    const result2 = retireScene(this.#root, this.#reader.beginClose());
    this.#reader = null;
    this.#root = null;
    return result2;
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null;
  }
}

class OwnedUiIssuedSceneText {
  #root;
  #reader;
  constructor(mint, root, reader) {
    if (mint !== ISSUED_MINT)
      throw new Error("Issued scene text requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    ownSceneText = (root, reader) => new OwnedUiIssuedSceneText(ISSUED_MINT, root, reader);
    textLease = (reader) => reader.#root?.lease ?? null;
  }
  advance(grant) {
    if (!this.#reader)
      throw new Error("Issued scene text is closed");
    return this.#reader.advance(grant);
  }
  beginClose() {
    if (!this.#reader || !this.#root)
      throw new Error("Issued scene text is closed");
    const result2 = retireScene(this.#root, this.#reader.beginClose());
    this.#reader = null;
    this.#root = null;
    return result2;
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null;
  }
}

class OwnedUiIssuedSceneRetirement {
  #root;
  #reader;
  constructor(mint, root, reader) {
    if (mint !== ISSUED_MINT)
      throw new Error("Issued scene retirement requires exact mint authority");
    this.#root = root;
    this.#reader = reader;
    Object.freeze(this);
  }
  static {
    retireScene = (root, reader) => new OwnedUiIssuedSceneRetirement(ISSUED_MINT, root, reader);
  }
  advance(grant) {
    if (!admitted14(grant))
      return state5("blocked", "issued-scene-close");
    if (this.#reader) {
      const current = this.#reader.advance(grant);
      if (current.kind === "complete")
        this.#reader = null;
      return { ...current, kind: "pending" };
    }
    if (this.#root) {
      this.#root.readers--;
      this.#root = null;
      return state5("pending", "issued-scene-slot-release", 32);
    }
    return state5("complete", "issued-scene-close");
  }
  terminalIsEmpty() {
    return this.#root === null && this.#reader === null;
  }
}

class OwnedUiIssuedRetirement {
  #root;
  #node = null;
  constructor(mint, root) {
    if (mint !== ISSUED_MINT)
      throw new Error("Issued retirement requires exact mint authority");
    this.#root = root;
    Object.freeze(this);
  }
  static {
    retireIssued = (root) => new OwnedUiIssuedRetirement(ISSUED_MINT, root);
  }
  advance(grant) {
    if (!admitted14(grant))
      return state5("blocked", "issued-read-close");
    if (this.#root?.readers)
      return state5("blocked", "issued-scene-readers");
    if (this.#node) {
      const current = this.#node.advance(grant);
      if (current.kind === "complete")
        this.#node = null;
      return { ...current, kind: "pending" };
    }
    if (this.#root) {
      this.#node = this.#root.node?.beginClose() ?? null;
      this.#root.node = null;
      this.#root = null;
      return state5("pending", "issued-node-close", 64);
    }
    return state5("complete", "issued-read-close");
  }
  terminalIsEmpty() {
    return this.#root === null && this.#node === null;
  }
}

class OwnedUiNodeReadLease {
  #authority = Object.freeze({});
  #id;
  #first;
  #second = null;
  #retirement = null;
  #releaseFirst = false;
  #started = false;
  #closing = false;
  #closed = false;
  #publication;
  #commit = null;
  #discard = false;
  constructor(id2, version, node, publication = null) {
    this.#id = natural2(id2);
    natural2(version);
    if (publication && publication.version !== version)
      throw new Error("Owned UI read publication version mismatch");
    if (node && node.value.id !== this.#id)
      throw new Error("Owned UI read node identity mismatch");
    this.#publication = publication;
    this.#first = issue(this.#authority, version, node?.capture() ?? null);
    Object.freeze(this);
  }
  #visible() {
    return this.#commit === null || commitState(this.#commit).status === "published";
  }
  #cancelled() {
    return this.#commit !== null && commitState(this.#commit).status === "cancelled";
  }
  get snapshot() {
    if (this.#closing || this.#closed)
      throw new Error("Owned UI read lease is closing");
    return this.#second && this.#visible() ? this.#second : this.#first;
  }
  get retirementPending() {
    return this.#releaseFirst || this.#cancelled() || this.#discard;
  }
  get hasCapacity() {
    return !this.#closing && !this.#closed && !this.#second && !this.#releaseFirst;
  }
  canReadSnapshot(snapshot) {
    return !this.#closing && !this.#closed && snapshot instanceof OwnedUiNodeReadSnapshot && snapshotLease(snapshot) === this.#authority;
  }
  takeSceneRetirement(reader) {
    const owner = reader instanceof OwnedUiIssuedSceneRecord ? recordLease(reader) : reader instanceof OwnedUiIssuedSceneText ? textLease(reader) : null;
    return !this.#closed && owner === this.#authority ? reader.beginClose() : null;
  }
  offer(version, node) {
    if (this.#publication)
      throw new Error("Owned UI publication reads require an exact staging token");
    return this.#offer(version, node);
  }
  stage(commit, node) {
    const exact2 = commitState(commit);
    if (exact2.owner !== this.#publication || exact2.status !== "pending" || !pendingCommit(exact2.owner, commit))
      throw new Error("Foreign or terminal owned UI read publication");
    if (!this.#offer(exact2.version, node))
      return false;
    this.#commit = commit;
    return true;
  }
  #offer(version, node) {
    natural2(version);
    if (this.#closing || this.#closed)
      return false;
    const latest = this.#second ?? this.#first;
    if (version <= latest.version)
      throw new Error("Owned UI read version did not advance");
    if (node && node.value.id !== this.#id)
      throw new Error("Owned UI read node identity mismatch");
    if (this.#second || this.#releaseFirst)
      return false;
    this.#second = issue(this.#authority, version, node?.capture() ?? null);
    return true;
  }
  acknowledge(snapshot) {
    if (this.#closing || this.#closed)
      return false;
    if (this.#second !== null && snapshot === this.#second && this.#visible()) {
      this.#releaseFirst = true;
      return true;
    }
    return this.#first !== null && snapshot === this.#first && !this.#releaseFirst;
  }
  #advance(grant) {
    if (!this.#closing && (this.#cancelled() || this.#discard)) {
      if (!this.#discard) {
        this.#retirement = release(this.#second);
        this.#discard = true;
        return state5("pending", "read-staging-release", 64);
      }
      if (this.#retirement) {
        const result2 = this.#retirement.advance(grant);
        if (result2.kind === "complete")
          this.#retirement = null;
        return { ...result2, kind: "pending" };
      }
      this.#second = null;
      this.#commit = null;
      this.#discard = false;
      return state5("pending", "read-staging-capacity", 64);
    }
    if (!this.#releaseFirst)
      return state5("ready", "read-lease-idle");
    if (!this.#started) {
      this.#retirement = release(this.#first);
      this.#started = true;
      return state5("pending", "read-snapshot-release", 64);
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { ...result2, kind: "pending" };
    }
    this.#first = this.#second;
    this.#second = null;
    this.#commit = null;
    this.#releaseFirst = false;
    this.#started = false;
    return state5("pending", "read-capacity-release", 64);
  }
  advanceRetirement(grant) {
    if (this.#closing || this.#closed)
      throw new Error("Use closeStep after closing an owned UI read lease");
    if (!admitted14(grant))
      return state5("blocked", "read-retirement");
    return this.#advance(grant);
  }
  beginClose() {
    if (this.#closed || this.#closing)
      return;
    this.#closing = true;
    if (this.#first)
      this.#releaseFirst = true;
  }
  closeStep(grant) {
    if (this.#closed)
      return state5("complete", "read-lease-close");
    if (!this.#closing)
      throw new Error("Owned UI read close has not begun");
    if (!admitted14(grant))
      return state5("blocked", "read-lease-close");
    if (this.#discard) {
      if (this.#retirement) {
        const result2 = this.#retirement.advance(grant);
        if (result2.kind === "complete")
          this.#retirement = null;
        return { ...result2, kind: "pending" };
      }
      this.#second = null;
      this.#commit = null;
      this.#discard = false;
      return state5("pending", "read-staging-close", 64);
    }
    if (this.#first) {
      this.#releaseFirst = true;
      return this.#advance(grant);
    }
    this.#publication = null;
    this.#closed = true;
    return state5("complete", "read-lease-close");
  }
  terminalIsEmpty() {
    return this.#closed && !this.#first && !this.#second && !this.#retirement && !this.#releaseFirst && !this.#started && !this.#commit && !this.#discard && !this.#publication;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts */
var admitted15 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var MINT8 = Object.freeze({});
var state6 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var validWork = (current, grant) => Number.isSafeInteger(current.items) && current.items >= 0 && current.items <= 1 && Number.isSafeInteger(current.bytes) && current.bytes >= 0 && current.bytes <= grant.maxBytes;
function childStep(current, grant) {
  if (!validWork(current, grant))
    return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function natural3(value) {
  if (!Number.isSafeInteger(value) || value < 0)
    throw new RangeError("Invalid owned UI identity");
  return value === 0 ? 0 : value;
}
function limitsOf(value) {
  return Object.freeze({ maxNodes: natural3(value.maxNodes), maxDepth: natural3(value.maxDepth), maxChildren: natural3(value.maxChildren), maxTextBytes: natural3(value.maxTextBytes), maxPatchOps: natural3(value.maxPatchOps), maxPatchBytes: natural3(value.maxPatchBytes) });
}
var cellOf;
var subscriptionOf;
var sourceOf;
var firstCell;
var lastOrdinal;
var epochOf;
var profileOf;
var enqueueCell;
var notifyCell;
var publish;
var releasePatch;
var createPatch;
var appended;
var detached;

class Subscription {
  #cell;
  constructor(mint, cell) {
    if (mint !== MINT8)
      throw new Error("Surface subscription requires exact mint authority");
    this.#cell = cell;
    Object.freeze(this);
  }
  static {
    subscriptionOf = (cell) => new Subscription(MINT8, cell);
    cellOf = (value) => value instanceof Subscription ? value.#cell : null;
  }
  get snapshot() {
    if (!this.#cell.owner)
      throw new Error("Owned UI subscription is retired");
    return this.#cell.lease?.snapshot ?? null;
  }
}

class OwnedUiSurface {
  identity;
  #limits;
  #profile;
  #state;
  #epoch = new OwnedUiReadPublication(0);
  #patch = null;
  #head = null;
  #tail = null;
  #queue = null;
  #queueTail = null;
  #queueWorked = false;
  #queueComplete = false;
  #maintenanceFailure = null;
  #ordinal = 0;
  #retirement = null;
  #bindingRetirement = null;
  #closing = false;
  #closed = false;
  #notificationFailures = 0;
  #failureHead = null;
  #failureTail = null;
  constructor(identity, limits, profile) {
    if (typeof identity.actor !== "string" || typeof identity.surface !== "string" || natural3(identity.instance) > 4294967295)
      throw new Error("Invalid owned UI surface identity");
    const bits = profile.usizeBits;
    if (bits !== 32 && bits !== 64)
      throw new Error("Owned surface requires an owning host width");
    this.#profile = Object.freeze({ usizeBits: bits });
    this.identity = Object.freeze({ actor: identity.actor, instance: identity.instance === 0 ? 0 : identity.instance, surface: identity.surface });
    this.#limits = limitsOf(limits);
    this.#state = { nodes: OwnedUiNodeIndex.empty(), bindings: OwnedUiSceneBindingIndex.empty(), view: Object.freeze({ revision: 0, root: null, hash: null }) };
    Object.freeze(this);
  }
  static {
    sourceOf = (owner) => owner.#live();
    firstCell = (owner) => owner.#head;
    lastOrdinal = (owner) => owner.#tail?.ordinal ?? 0;
    epochOf = (owner) => owner.#epoch;
    profileOf = (owner) => owner.#profile;
    enqueueCell = (owner, cell) => owner.#enqueue(cell);
    notifyCell = (owner, cell) => owner.#notify(cell);
    releasePatch = (owner, patch) => {
      if (owner.#patch !== patch)
        throw new Error("Foreign owned UI patch release");
      owner.#patch = null;
    };
    publish = (owner, patch, source, nodes, bindings, root, revision, hash, epoch) => {
      if (owner.#closing || owner.#patch !== patch || owner.#state !== source)
        throw new Error("Stale owned UI publication authority");
      const next = { nodes, bindings, view: Object.freeze({ revision, root, hash }) };
      if (!owner.#epoch.publish(epoch))
        throw new Error("Stale owned UI read publication");
      owner.#state = next;
      return source;
    };
  }
  #live() {
    if (!this.#state || this.#closed)
      throw new Error("Owned UI surface is closed");
    return this.#state;
  }
  get view() {
    return this.#live().view;
  }
  get maintenancePending() {
    return this.#queue !== null;
  }
  get maintenanceFailure() {
    return this.#maintenanceFailure;
  }
  get notificationFailures() {
    return this.#notificationFailures;
  }
  beginPatch(baseRevision, revision) {
    const source = this.#live();
    const base = natural3(baseRevision);
    const next = natural3(revision);
    if (this.#closing || this.#patch || base !== source.view.revision || next <= base)
      throw new Error("Owned UI patch admission rejected");
    const patch = createPatch(this, source, next, this.#limits);
    this.#patch = patch;
    return patch;
  }
  subscribeNode(id2, notify) {
    return this.#subscribe(natural3(id2), notify);
  }
  subscribeView(notify) {
    return this.#subscribe(null, notify);
  }
  #subscribe(id2, notify) {
    this.#live();
    if (this.#closing || this.#ordinal === Number.MAX_SAFE_INTEGER || typeof notify !== "function")
      throw new Error("Owned UI subscription admission rejected");
    const cell = { owner: this, id: id2, ordinal: ++this.#ordinal, notify, active: true, initialized: false, initialNotify: true, queued: false, previous: this.#tail, next: null, queueNext: null, lease: null, reader: null, readerComplete: false, retirement: null, node: null, nodeRetirement: null, lookupVersion: 0, subscription: null, failed: false, failureQueued: false, failurePrevious: null, failureNext: null, sceneClose: [null, null, null, null], sceneActive: [null, null, null, null] };
    cell.subscription = subscriptionOf(cell);
    if (this.#tail)
      this.#tail.next = cell;
    else
      this.#head = cell;
    this.#tail = cell;
    this.#enqueue(cell);
    if (this.#patch)
      appended(this.#patch, cell);
    return cell.subscription;
  }
  acknowledgeRead(subscription, snapshot) {
    const cell = cellOf(subscription);
    if (!cell || cell.owner !== this || !cell.active || !cell.lease)
      return;
    if (cell.lease.acknowledge(snapshot) && cell.lease.retirementPending)
      this.#enqueue(cell);
  }
  unsubscribeNode(subscription) {
    const cell = cellOf(subscription);
    if (cell?.owner === this && cell.active)
      this.#detach(cell);
  }
  retireSceneRead(subscription, reader) {
    const cell = cellOf(subscription);
    if (!cell || cell.owner !== this || !cell.lease)
      return false;
    for (let index = 0;index < 4; index++)
      if (cell.sceneClose[index] === null) {
        const retirement = cell.lease.takeSceneRetirement(reader);
        if (!retirement)
          return false;
        cell.sceneClose[index] = retirement;
        for (let active = 0;active < 4; active++)
          if (cell.sceneActive[active] === reader)
            cell.sceneActive[active] = null;
        this.#enqueue(cell);
        return true;
      }
    return false;
  }
  #sceneCell(subscription, snapshot) {
    const cell = cellOf(subscription);
    return cell?.owner === this && cell.active && cell.lease?.canReadSnapshot(snapshot) ? cell : null;
  }
  openSceneRecord(subscription, snapshot, source = 0) {
    const cell = this.#sceneCell(subscription, snapshot);
    if (!cell)
      return null;
    for (let index = 0;index < 4; index++)
      if (!cell.sceneActive[index]) {
        const reader = snapshot.beginSceneRecord(source);
        if (!reader)
          return null;
        cell.sceneActive[index] = reader;
        return Object.freeze({ advance: (grant) => reader.advance(grant), close: () => this.retireSceneRead(subscription, reader) });
      }
    return null;
  }
  openSceneText(subscription, snapshot, source) {
    const cell = this.#sceneCell(subscription, snapshot);
    if (!cell)
      return null;
    for (let index = 0;index < 4; index++)
      if (!cell.sceneActive[index]) {
        const reader = snapshot.beginSceneText(source);
        if (!reader)
          return null;
        cell.sceneActive[index] = reader;
        return Object.freeze({ advance: (grant) => reader.advance(grant), close: () => this.retireSceneRead(subscription, reader) });
      }
    return null;
  }
  #detach(cell) {
    let free = 0;
    let active = 0;
    for (let index = 0;index < 4; index++) {
      if (cell.sceneClose[index] === null)
        free++;
      if (cell.sceneActive[index] !== null)
        active++;
    }
    if (free < active)
      throw new Error("Owned UI scene retirement reservation violated");
    for (let index = 0;index < 4; index++) {
      const reader = cell.sceneActive[index];
      if (reader && !this.retireSceneRead(cell.subscription, reader))
        throw new Error("Owned UI scene retirement authority violated");
    }
    this.#removeFailure(cell);
    if (this.#patch)
      detached(this.#patch, cell);
    if (cell.previous)
      cell.previous.next = cell.next;
    else
      this.#head = cell.next;
    if (cell.next)
      cell.next.previous = cell.previous;
    else
      this.#tail = cell.previous;
    cell.previous = null;
    cell.next = null;
    cell.active = false;
    cell.notify = null;
    this.#enqueue(cell);
  }
  #enqueue(cell) {
    if (cell.queued) {
      if (this.#queue === cell)
        this.#queueComplete = false;
      return;
    }
    cell.queued = true;
    if (this.#queueTail)
      this.#queueTail.queueNext = cell;
    else
      this.#queue = cell;
    this.#queueTail = cell;
  }
  #dequeue() {
    const cell = this.#queue;
    this.#queue = cell.queueNext;
    if (!this.#queue)
      this.#queueTail = null;
    cell.queueNext = null;
    cell.queued = false;
    return cell;
  }
  #removeFailure(cell) {
    if (!cell.failureQueued)
      return;
    if (cell.failurePrevious)
      cell.failurePrevious.failureNext = cell.failureNext;
    else
      this.#failureHead = cell.failureNext;
    if (cell.failureNext)
      cell.failureNext.failurePrevious = cell.failurePrevious;
    else
      this.#failureTail = cell.failurePrevious;
    cell.failurePrevious = null;
    cell.failureNext = null;
    cell.failureQueued = false;
  }
  #notify(cell) {
    cell.initialNotify = false;
    if (cell.active && cell.notify && !cell.failed) {
      try {
        cell.notify();
      } catch {
        if (this.#notificationFailures < Number.MAX_SAFE_INTEGER)
          this.#notificationFailures++;
        if (cell.active) {
          cell.failed = true;
          cell.failureQueued = true;
          cell.failurePrevious = this.#failureTail;
          if (this.#failureTail)
            this.#failureTail.failureNext = cell;
          else
            this.#failureHead = cell;
          this.#failureTail = cell;
        }
      }
    }
  }
  takeNotificationFailure() {
    const cell = this.#failureHead;
    if (!cell)
      return null;
    this.#removeFailure(cell);
    return { subscription: cell.subscription, reason: "callback-threw" };
  }
  retryNotification(subscription) {
    const cell = cellOf(subscription);
    if (!cell || cell.owner !== this || !cell.active || !cell.failed || this.#closing)
      return false;
    this.#removeFailure(cell);
    cell.failed = false;
    cell.initialNotify = true;
    this.#enqueue(cell);
    return true;
  }
  #maintain(cell, grant) {
    for (let index = 0;index < 4; index++) {
      const child = cell.sceneClose[index];
      if (!child)
        continue;
      if (child.terminalIsEmpty()) {
        cell.sceneClose[index] = null;
        return state6("pending", "subscription-scene-release", 64);
      }
      return childStep(child.advance(grant), grant);
    }
    if (cell.nodeRetirement) {
      if (cell.nodeRetirement.terminalIsEmpty()) {
        cell.nodeRetirement = null;
        return state6("pending", "subscription-node-release", 64);
      }
      return childStep(cell.nodeRetirement.advance(grant), grant);
    }
    if (cell.node && !cell.reader) {
      cell.nodeRetirement = cell.node.beginClose();
      cell.node = null;
      return state6("pending", "subscription-node-close", 64);
    }
    if (cell.retirement) {
      if (cell.retirement.terminalIsEmpty()) {
        cell.retirement = null;
        return state6("pending", "subscription-reader-release", 64);
      }
      return childStep(cell.retirement.advance(grant), grant);
    }
    if (!cell.active) {
      if (cell.reader) {
        cell.retirement = cell.reader.beginClose();
        cell.reader = null;
        cell.readerComplete = false;
        return state6("pending", "subscription-reader-close", 64);
      }
      if (cell.lease) {
        if (cell.lease.terminalIsEmpty()) {
          cell.lease = null;
          return state6("pending", "subscription-lease-release", 64);
        }
        cell.lease.beginClose();
        return childStep(cell.lease.closeStep(grant), grant);
      }
      cell.owner = null;
      cell.subscription = null;
      cell.failed = false;
      return state6("complete", "subscription-close", 64);
    }
    if (!cell.initialized) {
      if (cell.id === null) {
        cell.initialized = true;
        return state6("pending", "view-subscription", 32);
      }
      if (!cell.reader) {
        const source = this.#live();
        cell.lookupVersion = source.view.revision;
        cell.reader = source.bindings.beginLookup(cell.id);
        return state6("pending", "subscription-lookup", 64);
      }
      if (cell.readerComplete) {
        if (cell.lookupVersion === this.view.revision) {
          cell.lease = new OwnedUiNodeReadLease(cell.id, cell.lookupVersion, cell.node, this.#epoch);
          cell.initialized = true;
        }
        cell.retirement = cell.reader.beginClose();
        cell.reader = null;
        cell.readerComplete = false;
        return state6("pending", "subscription-read-captured", 512);
      }
      const result2 = cell.reader.advance(grant);
      if (result2.kind === "value")
        cell.node = result2.value;
      if (!validWork(result2, grant))
        return { kind: "rejected", phase: "subscription-read-grant", items: result2.items, bytes: result2.bytes };
      if (result2.kind === "value")
        return { kind: "pending", phase: "subscription-read", items: result2.items, bytes: result2.bytes };
      if (result2.kind === "complete") {
        cell.readerComplete = true;
        return { ...result2, kind: "pending" };
      }
      return result2;
    }
    if (cell.initialNotify) {
      this.#notify(cell);
      return state6("pending", "subscription-initial-notify", 64);
    }
    if (cell.lease?.retirementPending)
      return cell.lease.advanceRetirement(grant);
    return state6("complete", "subscription-idle");
  }
  advanceMaintenance(grant) {
    if (!admitted15(grant))
      return state6("blocked", "surface-maintenance");
    if (!this.#queue)
      return state6("complete", "surface-maintenance");
    if (this.#queueWorked) {
      const cell = this.#dequeue();
      const complete = this.#queueComplete;
      this.#queueWorked = false;
      this.#queueComplete = false;
      if (!complete)
        this.#enqueue(cell);
      return state6("pending", "surface-maintenance-queue", 64);
    }
    try {
      const result2 = this.#maintain(this.#queue, grant);
      if (!validWork(result2, grant)) {
        this.#maintenanceFailure = "Surface maintenance child exceeded its grant";
        return { ...result2, kind: "rejected" };
      }
      if (result2.kind === "blocked" || result2.kind === "rejected") {
        if (result2.kind === "rejected")
          this.#maintenanceFailure = result2.phase;
        return result2;
      }
      this.#maintenanceFailure = null;
      this.#queueWorked = true;
      this.#queueComplete = result2.kind === "complete";
      return { ...result2, kind: "pending" };
    } catch (error) {
      this.#maintenanceFailure = error instanceof Error ? error.message : "Surface maintenance failed";
      return state6("rejected", "surface-maintenance-failed");
    }
  }
  beginClose() {
    if (this.#closing || this.#closed)
      return;
    this.#closing = true;
    this.#patch?.beginClose();
  }
  takePendingAcknowledgement() {
    return this.#patch?.takeAcknowledgement() ?? null;
  }
  closeStep(grant) {
    if (this.#closed)
      return state6("complete", "surface-close");
    if (!this.#closing)
      throw new Error("Owned UI surface close has not begun");
    if (!admitted15(grant))
      return state6("blocked", "surface-close");
    if (this.#patch)
      return childStep(this.#patch.closeStep(grant), grant);
    if (this.#head)
      return state6("blocked", "surface-readers");
    if (this.#queue)
      return this.advanceMaintenance(grant);
    if (this.#retirement) {
      if (this.#retirement.terminalIsEmpty()) {
        this.#retirement = null;
        return state6("pending", "surface-nodes-release", 64);
      }
      return childStep(this.#retirement.advance(grant), grant);
    }
    if (this.#bindingRetirement) {
      if (this.#bindingRetirement.terminalIsEmpty()) {
        this.#bindingRetirement = null;
        return state6("pending", "surface-bindings-release", 64);
      }
      return childStep(this.#bindingRetirement.advance(grant), grant);
    }
    if (this.#state) {
      this.#retirement = this.#state.nodes.beginClose();
      this.#bindingRetirement = this.#state.bindings.beginClose();
      this.#state = null;
      return state6("pending", "surface-root-close", 128);
    }
    this.#closed = true;
    return state6("complete", "surface-close");
  }
  terminalIsEmpty() {
    return this.#closed && !this.#state && !this.#patch && !this.#head && !this.#tail && !this.#queue && !this.#queueTail && !this.#queueWorked && !this.#queueComplete && !this.#retirement && !this.#bindingRetirement && !this.#failureHead && !this.#failureTail;
  }
}

class OwnedUiSurfacePatch {
  #owner;
  #source;
  #nodes;
  #bindings = null;
  #binding = null;
  #bindingClose = null;
  #bindingRetirement = null;
  #bindingReader = null;
  #bindingEdit = null;
  #bindingPreparation = null;
  #root;
  #revision;
  #limits;
  #operation = null;
  #operationTouched = null;
  #touched;
  #validation = null;
  #violations = null;
  #hash = null;
  #retirement = null;
  #reader = null;
  #node = null;
  #nodeRetirement = null;
  #program = null;
  #grant = { maxItems: 0, maxBytes: 0 };
  #status = "input";
  #phase = "input";
  #finished = false;
  #count = 0;
  #estimatedBytes = 0;
  #failure = null;
  #epoch = null;
  #scan = null;
  #cell = null;
  #notify = null;
  #notifyLimit = 0;
  #ack = null;
  #published = false;
  #maintenanceTurn = true;
  #running = false;
  #staged = null;
  #closeRequested = false;
  constructor(mint, owner, source, revision, limits) {
    if (mint !== MINT8)
      throw new Error("Surface patch requires exact mint authority");
    this.#owner = owner;
    this.#source = source;
    this.#nodes = source.nodes.capture();
    this.#root = source.view.root;
    this.#revision = revision;
    this.#limits = limits;
    this.#touched = new Table(NumericIndex.empty(), () => this.#grant);
    Object.freeze(this);
  }
  static {
    createPatch = (owner, source, revision, limits) => new OwnedUiSurfacePatch(MINT8, owner, source, revision, limits);
    appended = (patch, cell) => {
      if (patch.#phase === "staging" && !patch.#scan)
        patch.#scan = cell;
    };
    detached = (patch, cell) => {
      if (patch.#scan === cell)
        patch.#scan = cell.next;
      if (patch.#notify === cell)
        patch.#notify = cell.next;
    };
  }
  get failure() {
    return this.#failure;
  }
  get phase() {
    return this.#phase;
  }
  pushOperation(operation) {
    if (this.#status !== "input" || this.#finished || this.#operation || this.#count >= this.#limits.maxPatchOps)
      throw new Error("Owned UI operation admission rejected");
    this.#operation = new OwnedUiOperationCursor(this.#nodes, this.#root, operation, this.#limits);
    this.#count++;
    this.#status = "operation";
    this.#program = this.#apply();
  }
  finishInput() {
    if (this.#status !== "input" || this.#finished)
      throw new Error("Owned UI input is not available");
    this.#finished = true;
  }
  *#drainIndex() {
    while (this.#retirement) {
      const result2 = this.#retirement.advance(this.#grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      yield result2.bytes;
    }
  }
  *#releaseNode() {
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      yield 64;
    }
    while (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(this.#grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      yield result2.bytes;
    }
  }
  *#apply() {
    for (;; ) {
      const result3 = this.#operation.advance(this.#grant);
      yield result3.bytes;
      if (result3.kind === "rejected")
        throw new Error(this.#operation.failure ?? "Owned operation failed");
      if (result3.kind === "ready")
        break;
    }
    const result2 = this.#operation.takeResult();
    this.#retirement = this.#nodes.beginClose();
    this.#nodes = result2.nodes;
    this.#root = result2.root;
    this.#operationTouched = new Table(result2.touched, () => this.#grant);
    this.#estimatedBytes += result2.estimatedBytes;
    yield 192;
    if (this.#estimatedBytes > this.#limits.maxPatchBytes)
      throw new Error("Owned UI patch byte quota exceeded");
    yield* this.#drainIndex();
    this.#operation.beginClose();
    yield 32;
    while (this.#operation) {
      const step8 = this.#operation.closeStep(this.#grant);
      if (step8.kind === "complete")
        this.#operation = null;
      yield step8.bytes;
    }
    for (const entry of this.#operationTouched.entries()) {
      if (typeof entry === "number")
        yield entry;
      else
        yield* this.#touched.set(entry[0], true);
    }
    while (this.#operationTouched) {
      const step8 = this.#operationTouched.closeStep(this.#grant);
      if (step8.complete)
        this.#operationTouched = null;
      yield step8.bytes;
    }
  }
  *#lookup(id2) {
    this.#reader = this.#nodes.beginLookup(id2);
    yield 64;
    for (;; ) {
      const step8 = this.#reader.advance(this.#grant);
      if (step8.kind === "value")
        this.#node = step8.value;
      yield step8.bytes;
      if (step8.kind === "complete")
        break;
    }
    this.#retirement = this.#reader.beginClose();
    this.#reader = null;
    yield 64;
    yield* this.#drainIndex();
  }
  *#drainBindings() {
    while (this.#bindingRetirement) {
      const current = this.#bindingRetirement.advance(this.#grant);
      if (current.kind === "complete")
        this.#bindingRetirement = null;
      yield current.bytes;
    }
  }
  *#releaseBinding() {
    if (this.#binding) {
      this.#bindingClose = this.#binding.beginClose();
      this.#binding = null;
      yield 64;
    }
    while (this.#bindingClose) {
      const current = this.#bindingClose.advance(this.#grant);
      if (current.kind === "complete")
        this.#bindingClose = null;
      yield current.bytes;
    }
  }
  *#lookupBinding(index, id2) {
    this.#bindingReader = index.beginLookup(id2);
    yield 64;
    for (;; ) {
      const current = this.#bindingReader.advance(this.#grant);
      if (current.kind === "value")
        this.#binding = current.value;
      yield current.bytes;
      if (current.kind === "complete")
        break;
    }
    this.#bindingRetirement = this.#bindingReader.beginClose();
    this.#bindingReader = null;
    yield 64;
    yield* this.#drainBindings();
  }
  *#prepareBindings() {
    this.#phase = "scenes";
    for (const entry of this.#touched.entries()) {
      if (typeof entry === "number") {
        yield entry;
        continue;
      }
      const id2 = entry[0];
      yield* this.#lookup(id2);
      if (this.#node) {
        yield* this.#lookupBinding(this.#source.bindings, id2);
        this.#bindingPreparation = new OwnedUiSceneBindingCursor(this.#node, profileOf(this.#owner));
        yield 64;
        if (this.#binding)
          this.#bindingPreparation.considerPrevious(this.#binding);
        yield 64;
        yield* this.#releaseBinding();
        for (;; ) {
          const current = this.#bindingPreparation.advance(this.#grant);
          yield current.bytes;
          if (current.kind === "ready")
            break;
          if (current.kind === "rejected")
            throw new Error(this.#bindingPreparation.failure ?? "Scene ownership failed");
        }
        this.#binding = this.#bindingPreparation.takeResult();
        this.#bindingPreparation.beginClose();
        yield 64;
        while (this.#bindingPreparation) {
          const current = this.#bindingPreparation.closeStep(this.#grant);
          if (current.kind === "complete")
            this.#bindingPreparation = null;
          yield current.bytes;
        }
        this.#bindingEdit = this.#bindings.beginSet(this.#binding);
        yield 64;
        yield* this.#releaseBinding();
      } else {
        this.#bindingEdit = this.#bindings.beginRemove(id2);
        yield 64;
      }
      yield* this.#releaseNode();
      for (;; ) {
        const current = this.#bindingEdit.advance(this.#grant);
        yield current.bytes;
        if (current.kind === "ready")
          break;
        if (current.kind === "rejected")
          throw new Error(this.#bindingEdit.failure ?? "Scene index failed");
      }
      const next = this.#bindingEdit.takeResult();
      this.#bindingRetirement = this.#bindings.beginClose();
      this.#bindings = next;
      yield 128;
      yield* this.#drainBindings();
      this.#bindingRetirement = this.#bindingEdit.beginClose();
      this.#bindingEdit = null;
      yield 64;
      yield* this.#drainBindings();
    }
  }
  *#prepare() {
    this.#bindings = this.#source.bindings.capture();
    yield 64;
    this.#phase = "validation";
    this.#validation = new OwnedUiValidationCursor(this.#nodes, this.#root, this.#limits);
    yield 256;
    for (;; ) {
      const step8 = this.#validation.advance(this.#grant);
      yield step8.bytes;
      if (step8.kind === "rejected")
        throw new Error(this.#validation.failure ?? "Owned UI validation failed");
      if (step8.kind === "ready")
        break;
    }
    const violations = this.#validation.takeResult();
    const valid = violations.size === 0;
    this.#violations = new Table(violations, () => this.#grant);
    this.#validation.beginClose();
    yield 64;
    if (!valid)
      throw new Error("Owned UI graph invariants violated");
    while (this.#validation) {
      const step8 = this.#validation.closeStep(this.#grant);
      if (step8.kind === "complete")
        this.#validation = null;
      yield step8.bytes;
    }
    while (this.#violations) {
      const step8 = this.#violations.closeStep(this.#grant);
      if (step8.complete)
        this.#violations = null;
      yield step8.bytes;
    }
    yield* this.#prepareBindings();
    this.#phase = "hash";
    this.#hash = new OwnedUiSnapshotHashCursor(this.#nodes, { surface: this.#owner.identity.surface, revision: this.#revision, root: this.#root });
    yield 128;
    for (;; ) {
      const step8 = this.#hash.advance(this.#grant);
      yield step8.bytes;
      if (step8.kind === "rejected")
        throw new Error(this.#hash.failure ?? "Owned UI hash failed");
      if (step8.kind === "ready")
        break;
    }
    const digest = this.#hash.takeResult();
    this.#hash.beginClose();
    yield 64;
    while (this.#hash) {
      const step8 = this.#hash.closeStep(this.#grant);
      if (step8.kind === "complete")
        this.#hash = null;
      yield step8.bytes;
    }
    this.#phase = "staging";
    this.#epoch = epochOf(this.#owner).begin(this.#revision);
    this.#scan = firstCell(this.#owner);
    yield 96;
    while (this.#scan) {
      this.#cell = this.#scan;
      this.#scan = this.#cell.next;
      yield 32;
      while (this.#cell.active && !this.#cell.initialized)
        yield 16;
      if (this.#cell.active && this.#cell.id !== null && (yield* this.#touched.lookup(this.#cell.id))) {
        while (this.#cell.active && !this.#cell.lease.hasCapacity)
          yield 16;
        if (this.#cell.active) {
          yield* this.#lookupBinding(this.#bindings, this.#cell.id);
          if (this.#cell.active) {
            if (!this.#cell.lease.stage(this.#epoch, this.#binding))
              throw new Error("Owned UI staging reservation changed");
            this.#staged = { cell: this.#cell, next: this.#staged };
          }
          yield 544;
          yield* this.#releaseBinding();
        }
      }
      this.#cell = null;
      yield 16;
    }
    this.#phase = "publication";
    const old = publish(this.#owner, this, this.#source, this.#nodes, this.#bindings, this.#root, this.#revision, digest.hash, this.#epoch);
    this.#nodes = null;
    this.#bindings = null;
    this.#published = true;
    this.#retirement = old.nodes.beginClose();
    this.#bindingRetirement = old.bindings.beginClose();
    this.#notify = firstCell(this.#owner);
    this.#notifyLimit = lastOrdinal(this.#owner);
    this.#phase = "notifications";
    yield 256;
    while (this.#notify && this.#notify.ordinal <= this.#notifyLimit) {
      const cell = this.#notify;
      this.#notify = cell.next;
      if (cell.active && (cell.id === null || cell.lease?.snapshot.version === this.#revision))
        notifyCell(this.#owner, cell);
      yield 64;
    }
    this.#notify = null;
    yield* this.#drainIndex();
    yield* this.#drainBindings();
    this.#ack = Object.freeze({ ...this.#owner.identity, revision: this.#revision, hash: digest.hash });
    this.#phase = "accepted";
    yield 128;
  }
  advance(grant) {
    if (this.#running)
      throw new Error("Reentrant owned UI patch drive");
    if (this.#status === "closing")
      throw new Error("Owned UI patch is closing");
    if (this.#status === "closed" || this.#status === "ready" || this.#status === "rejected")
      return state6(this.#status === "closed" ? "complete" : this.#status, this.#phase);
    if (!admitted15(grant))
      return state6("blocked", this.#phase);
    this.#grant = grant;
    this.#running = true;
    try {
      if (this.#maintenanceTurn && this.#owner.maintenancePending) {
        this.#maintenanceTurn = false;
        return this.#owner.advanceMaintenance(grant);
      }
      this.#maintenanceTurn = true;
      if (!this.#program) {
        if (!this.#finished)
          return state6("ready", "input");
        this.#status = "preparing";
        this.#program = this.#prepare();
      }
      const result2 = this.#program.next();
      if (result2.done) {
        this.#program = null;
        this.#status = this.#status === "operation" ? "input" : "ready";
        return state6("ready", this.#phase, 32);
      }
      if (result2.value > grant.maxBytes)
        throw new Error("Owned UI surface exceeded its byte grant");
      return state6("pending", this.#phase, result2.value);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Owned UI surface failed";
      this.#status = "rejected";
      this.#program = null;
      return state6("rejected", this.#phase, 64);
    } finally {
      this.#running = false;
    }
  }
  takeAcknowledgement() {
    if (this.#status !== "ready" || !this.#published)
      return null;
    const result2 = this.#ack;
    this.#ack = null;
    return result2;
  }
  beginClose() {
    if (this.#status === "closed" || this.#status === "closing")
      return;
    this.#closeRequested = true;
    if (this.#published && (this.#status !== "ready" || this.#ack))
      return;
    this.#startClose();
  }
  #startClose() {
    this.#status = "closing";
    this.#program = null;
    this.#scan = null;
    this.#cell = null;
    this.#notify = null;
    if (this.#epoch && !this.#published)
      epochOf(this.#owner).cancel(this.#epoch);
    this.#operation?.beginClose();
    this.#validation?.beginClose();
    this.#hash?.beginClose();
    this.#bindingPreparation?.beginClose();
  }
  closeStep(grant) {
    if (this.#status === "closed")
      return state6("complete", "surface-patch-close");
    if (!this.#closeRequested)
      throw new Error("Owned UI patch close has not begun");
    if (!admitted15(grant))
      return state6("blocked", "surface-patch-close");
    if (this.#status !== "closing") {
      if (this.#status === "rejected")
        return state6("rejected", "committed-publication-fault");
      if (this.#status !== "ready") {
        const result2 = this.advance(grant);
        return { ...result2, kind: result2.kind === "rejected" ? "rejected" : "pending" };
      }
      if (this.#ack)
        return state6("blocked", "surface-acknowledgement");
      this.#startClose();
      return state6("pending", "committed-publication-close", 64);
    }
    if (this.#staged) {
      const entry = this.#staged;
      const cell = entry.cell;
      if (!this.#published && cell.lease) {
        if (!cell.active) {
          enqueueCell(this.#owner, cell);
          return this.#owner.advanceMaintenance(grant);
        }
        if (cell.lease.retirementPending)
          return { ...cell.lease.advanceRetirement(grant), kind: "pending" };
      }
      this.#staged = entry.next;
      entry.next = null;
      entry.cell = null;
      return state6("pending", "staged-read-release", 48);
    }
    if (this.#bindingClose) {
      const current = this.#bindingClose.advance(grant);
      if (current.kind === "complete")
        this.#bindingClose = null;
      return { ...current, kind: "pending" };
    }
    if (this.#binding) {
      this.#bindingClose = this.#binding.beginClose();
      this.#binding = null;
      return state6("pending", "scene-read-close", 64);
    }
    if (this.#bindingRetirement) {
      const current = this.#bindingRetirement.advance(grant);
      if (current.kind === "complete")
        this.#bindingRetirement = null;
      return { ...current, kind: "pending" };
    }
    if (this.#bindingReader) {
      this.#bindingRetirement = this.#bindingReader.beginClose();
      this.#bindingReader = null;
      return state6("pending", "scene-reader-close", 64);
    }
    if (this.#bindingEdit) {
      this.#bindingRetirement = this.#bindingEdit.beginClose();
      this.#bindingEdit = null;
      return state6("pending", "scene-edit-close", 64);
    }
    if (this.#bindingPreparation) {
      const current = this.#bindingPreparation.closeStep(grant);
      if (current.kind === "complete")
        this.#bindingPreparation = null;
      return { ...current, kind: "pending" };
    }
    if (this.#bindings) {
      this.#bindingRetirement = this.#bindings.beginClose();
      this.#bindings = null;
      return state6("pending", "scene-index-close", 64);
    }
    if (this.#nodeRetirement) {
      const result2 = this.#nodeRetirement.advance(grant);
      if (result2.kind === "complete")
        this.#nodeRetirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#node) {
      this.#nodeRetirement = this.#node.beginClose();
      this.#node = null;
      return state6("pending", "staging-node-close", 64);
    }
    if (this.#retirement) {
      const result2 = this.#retirement.advance(grant);
      if (result2.kind === "complete")
        this.#retirement = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#reader) {
      this.#retirement = this.#reader.beginClose();
      this.#reader = null;
      return state6("pending", "staging-reader-close", 64);
    }
    if (this.#operation) {
      const result2 = this.#operation.closeStep(grant);
      if (result2.kind === "complete")
        this.#operation = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#operationTouched) {
      const result2 = this.#operationTouched.closeStep(grant);
      if (result2.complete)
        this.#operationTouched = null;
      return state6("pending", "operation-touched-close", result2.bytes);
    }
    if (this.#validation) {
      const result2 = this.#validation.closeStep(grant);
      if (result2.kind === "complete")
        this.#validation = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#violations) {
      const result2 = this.#violations.closeStep(grant);
      if (result2.complete)
        this.#violations = null;
      return state6("pending", "violations-close", result2.bytes);
    }
    if (this.#hash) {
      const result2 = this.#hash.closeStep(grant);
      if (result2.kind === "complete")
        this.#hash = null;
      return { ...result2, kind: "pending" };
    }
    if (this.#nodes) {
      this.#retirement = this.#nodes.beginClose();
      this.#nodes = null;
      return state6("pending", "candidate-close", 64);
    }
    const touched = this.#touched.closeStep(grant);
    if (!touched.complete)
      return state6("pending", "touched-close", touched.bytes);
    releasePatch(this.#owner, this);
    this.#owner = null;
    this.#source = null;
    this.#epoch = null;
    this.#status = "closed";
    return state6("complete", "surface-patch-close");
  }
  terminalIsEmpty() {
    return this.#status === "closed" && !this.#owner && !this.#source && !this.#nodes && !this.#bindings && !this.#binding && !this.#bindingClose && !this.#bindingRetirement && !this.#bindingReader && !this.#bindingEdit && !this.#bindingPreparation && !this.#operation && !this.#operationTouched && !this.#validation && !this.#violations && !this.#hash && !this.#retirement && !this.#reader && !this.#node && !this.#nodeRetirement && !this.#program && !this.#epoch && !this.#scan && !this.#cell && !this.#notify && !this.#ack && !this.#staged;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️.ts */
var admitted16 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step8 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function childStep2(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return { ...current, kind: "rejected" };
  return current.kind === "complete" || current.kind === "ready" ? { ...current, kind: "pending" } : current;
}
function nodeId(value) {
  if (typeof value === "bigint") {
    if (value < 0n || value > 9007199254740991n)
      throw new Error("Native node ID exceeds the exact renderer range");
    return Number(value);
  }
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0)
    throw new Error("Native node ID is not a nonnegative safe integer");
  return value === 0 ? 0 : value;
}
function data(value, key) {
  if (!value || typeof value !== "object")
    throw new Error("Native operation requires its exact data record");
  const descriptor = Object.getOwnPropertyDescriptor(value, key);
  if (!descriptor || !("value" in descriptor))
    throw new Error("Native operation field cannot be an accessor or inherited");
  return descriptor.value;
}
function nativeOperation(value) {
  const tag = data(value, "tag");
  if (typeof tag !== "string")
    throw new Error("Native operation tag is missing");
  const payload = data(value, "val");
  if (tag === "remove" || tag === "set-root")
    return new OwnedUiWireOperationCursor(tag, payload);
  if (tag === "upsert")
    return new OwnedUiWireOperationCursor(tag, undefined, data(payload, "node"));
  switch (tag) {
    case "set-component":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "component"));
    case "set-layout":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "layout"));
    case "set-activity":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "activity"));
    case "set-children":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "children"));
    case "set-style":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "style"));
    case "set-accessibility":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "accessibility"));
    case "set-bindings":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "bindings"));
    case "set-menu":
      return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "menu"));
    default:
      throw new Error("Unknown native patch tag");
  }
}

class OwnedUiWireOperationCursor {
  static fromNative(value) {
    return nativeOperation(value);
  }
  #id;
  #tag;
  #packed = null;
  #children = null;
  #payload = null;
  #payloadClose = null;
  #operation = null;
  #operationClose = null;
  #phase = "native-operation-decode";
  #failure = null;
  #closing = false;
  #ready = false;
  constructor(tag, target, payload) {
    if (typeof tag !== "string" || tag.length > 32)
      throw new Error("Invalid native patch tag");
    this.#tag = tag;
    this.#id = tag === "upsert" ? null : nodeId(target);
    if (tag === "upsert" && target !== undefined)
      throw new Error("Upsert identity is inside its exact node payload");
    if (tag === "remove" || tag === "set-root") {
      if (payload !== undefined)
        throw new Error("Scalar native operation cannot carry a payload");
    } else if (tag === "set-children")
      this.#children = new RetainedUiChildIdsCursor(payload);
    else {
      switch (tag) {
        case "upsert":
          this.#packed = { kind: "node", cursor: new RetainedUiTypedCursor(payload, "node") };
          break;
        case "set-component":
          this.#packed = { kind: "component", cursor: new RetainedUiTypedCursor(payload, "component") };
          break;
        case "set-layout":
          this.#packed = { kind: "layout", cursor: new RetainedUiTypedCursor(payload, "layout") };
          break;
        case "set-activity":
          this.#packed = { kind: "activity", cursor: new RetainedUiTypedCursor(payload, "activity") };
          break;
        case "set-style":
          this.#packed = { kind: "style", cursor: new RetainedUiTypedCursor(payload, "style") };
          break;
        case "set-accessibility":
          this.#packed = { kind: "accessibility", cursor: new RetainedUiTypedCursor(payload, "accessibility") };
          break;
        case "set-bindings":
          this.#packed = { kind: "bindings", cursor: new RetainedUiTypedCursor(payload, "bindings") };
          break;
        case "set-menu":
          this.#packed = { kind: "menu", cursor: new RetainedUiTypedCursor(payload, "menu") };
          break;
        default:
          throw new Error("Unknown native patch tag");
      }
    }
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  #capture(payload, create) {
    if (!payload)
      throw new Error("Native patch decoder lost its exact payload");
    this.#payload = payload;
    this.#operation = create(payload);
  }
  #capturePacked(packed) {
    switch (packed.kind) {
      case "node":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.upsert(value));
        break;
      case "component":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "component", payload: value }));
        break;
      case "layout":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "layout", payload: value }));
        break;
      case "style":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "style", payload: value }));
        break;
      case "accessibility":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "accessibility", payload: value }));
        break;
      case "bindings":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "bindings", payload: value }));
        break;
      case "menu":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "menu", payload: value }));
        break;
      case "activity":
        this.#capture(packed.cursor.takeResult(), (value) => OwnedUiOperation.activity(this.#id, value));
        break;
    }
  }
  advance(grant) {
    if (!admitted16(grant))
      return step8("blocked", this.#phase);
    if (this.#closing || this.#failure)
      return step8("rejected", this.#phase);
    if (this.#ready)
      return step8("ready", this.#phase);
    try {
      if (this.#phase === "native-operation-decode") {
        const decoder = this.#packed?.cursor ?? this.#children;
        if (decoder) {
          const current = decoder.advance(grant);
          const forwarded = childStep2(current, grant);
          if (forwarded.kind === "rejected")
            this.#failure = decoder.failure ?? "Native patch payload failed or exceeded its grant";
          else if (current.kind === "ready")
            this.#phase = "native-operation-capture";
          return forwarded;
        }
        this.#phase = "native-operation-capture";
        return step8("pending", this.#phase, 32);
      }
      if (this.#phase === "native-operation-capture") {
        if (this.#packed)
          this.#capturePacked(this.#packed);
        else if (this.#children)
          this.#capture(this.#children.takeResult(), (value) => OwnedUiOperation.field(this.#id, { field: "children", payload: value }));
        else
          this.#operation = this.#tag === "remove" ? OwnedUiOperation.remove(this.#id) : OwnedUiOperation.setRoot(this.#id);
        this.#packed?.cursor.beginClose();
        this.#children?.beginClose();
        this.#phase = "native-operation-retire";
        return step8("pending", this.#phase, 1024);
      }
      const cleanup = this.#closeInput(grant);
      if (cleanup)
        return cleanup;
      this.#ready = true;
      this.#phase = "native-operation-ready";
      return step8("ready", this.#phase, 32);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Native operation intake failed";
      return step8("rejected", this.#phase, 128);
    }
  }
  #closeInput(grant) {
    if (this.#payload) {
      this.#payloadClose = this.#payload.beginClose();
      this.#payload = null;
      return step8("pending", "native-payload-release", 64);
    }
    if (this.#payloadClose) {
      if (this.#payloadClose.terminalIsEmpty()) {
        this.#payloadClose = null;
        return step8("pending", "native-payload-retirement-release", 64);
      }
      return childStep2(this.#payloadClose.advance(grant), grant);
    }
    const decoder = this.#packed?.cursor ?? this.#children;
    if (decoder) {
      if (decoder.terminalIsEmpty()) {
        this.#packed = null;
        this.#children = null;
        return step8("pending", "native-decoder-release", 64);
      }
      return childStep2(decoder.closeStep(grant), grant);
    }
    return null;
  }
  takeResult() {
    if (!this.#ready || this.#closing || this.#failure)
      return null;
    const result2 = this.#operation;
    this.#operation = null;
    return result2;
  }
  beginClose() {
    if (this.#closing)
      return;
    this.#closing = true;
    this.#packed?.cursor.beginClose();
    this.#children?.beginClose();
  }
  closeStep(grant) {
    if (!admitted16(grant))
      return step8("blocked", "native-operation-close");
    if (!this.#closing)
      throw new Error("Native operation close has not begun");
    try {
      return this.#closeStep(grant);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Native operation close failed";
      return step8("rejected", "native-operation-close-failed");
    }
  }
  #closeStep(grant) {
    if (this.#operation) {
      this.#operationClose = this.#operation.beginClose();
      this.#operation = null;
      return step8("pending", "native-operation-close", 64);
    }
    if (this.#operationClose) {
      if (this.#operationClose.terminalIsEmpty()) {
        this.#operationClose = null;
        return step8("pending", "native-operation-retirement-release", 64);
      }
      return childStep2(this.#operationClose.advance(grant), grant);
    }
    return this.#closeInput(grant) ?? step8("complete", "native-operation-close");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#packed && !this.#children && !this.#payload && !this.#payloadClose && !this.#operation && !this.#operationClose;
  }
}

class OwnedUiWirePatchCursor {
  #patch;
  #count;
  #next = 0;
  #input = null;
  #operation = null;
  #operationClose = null;
  #receipt = null;
  #phase = "input";
  #closing = false;
  #failure = null;
  constructor(surface, baseRevision, revision, count) {
    if (!Number.isSafeInteger(count) || count < 0)
      throw new Error("Invalid native operation count");
    this.#count = count;
    this.#patch = surface.beginPatch(baseRevision, revision);
    Object.freeze(this);
  }
  get failure() {
    return this.#failure;
  }
  offer(ordinal, value) {
    if (this.#closing || this.#failure || this.#phase !== "input" || this.#receipt || this.#input || ordinal !== this.#next || ordinal >= this.#count)
      return false;
    this.#input = OwnedUiWireOperationCursor.fromNative(value);
    this.#phase = "decode";
    return true;
  }
  finishInput() {
    if (this.#closing || this.#failure || this.#phase !== "input" || this.#receipt || this.#next !== this.#count)
      throw new Error("Native patch still owns unconsumed input obligations");
    this.#patch.finishInput();
    this.#phase = "publish";
  }
  takePageReceipt() {
    const result2 = this.#receipt;
    this.#receipt = null;
    return result2;
  }
  takeAcknowledgement() {
    return this.#patch?.takeAcknowledgement() ?? null;
  }
  advance(grant) {
    if (!admitted16(grant))
      return step8("blocked", "native-patch");
    if (this.#closing || this.#failure)
      return step8("rejected", "native-patch");
    if (this.#phase === "input" || this.#phase === "ready")
      return step8("ready", `native-patch-${this.#phase}`);
    try {
      if (this.#phase === "decode") {
        const current2 = this.#input.advance(grant);
        const forwarded2 = childStep2(current2, grant);
        if (forwarded2.kind === "rejected")
          this.#failure = this.#input.failure ?? "Native operation rejected or exceeded its grant";
        else if (current2.kind === "ready")
          this.#phase = "decode-result";
        return forwarded2;
      }
      if (this.#phase === "decode-result") {
        this.#operation = this.#input.takeResult();
        if (!this.#operation)
          throw new Error("Native operation transfer is missing");
        this.#input.beginClose();
        this.#phase = "input-close";
        return step8("pending", "native-patch-result", 128);
      }
      if (this.#phase === "input-close") {
        if (this.#input.terminalIsEmpty()) {
          this.#input = null;
          this.#phase = "transfer";
          return step8("pending", "native-input-release", 64);
        }
        return childStep2(this.#input.closeStep(grant), grant);
      }
      if (this.#phase === "transfer") {
        this.#patch.pushOperation(this.#operation);
        this.#operation = null;
        this.#phase = "apply";
        return step8("pending", "native-patch-transfer", 128);
      }
      if (this.#phase === "apply-receipt") {
        this.#receipt = Object.freeze({ ordinal: this.#next++ });
        this.#phase = "input";
        return step8("ready", "native-patch-receipt", 128);
      }
      const current = this.#patch.advance(grant);
      const forwarded = childStep2(current, grant);
      if (forwarded.kind === "rejected") {
        this.#failure = this.#patch.failure ?? "Native patch rejected or exceeded its grant";
        return forwarded;
      }
      if (current.kind === "ready") {
        if (this.#phase === "apply") {
          this.#phase = "apply-receipt";
          return forwarded;
        } else
          this.#phase = "ready";
      }
      return current.kind === "ready" ? current : forwarded;
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Native patch intake failed";
      return step8("rejected", "native-patch", 128);
    }
  }
  beginClose() {
    if (this.#closing)
      return;
    this.#closing = true;
    this.#input?.beginClose();
    this.#patch?.beginClose();
  }
  closeStep(grant) {
    if (!admitted16(grant))
      return step8("blocked", "native-patch-close");
    if (!this.#closing)
      throw new Error("Native patch close has not begun");
    try {
      return this.#closeStep(grant);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Native patch close failed";
      return step8("rejected", "native-patch-close-failed");
    }
  }
  #closeStep(grant) {
    if (this.#receipt)
      return step8("blocked", "native-page-receipt");
    if (this.#operation) {
      this.#operationClose = this.#operation.beginClose();
      this.#operation = null;
      return step8("pending", "native-operation-release", 64);
    }
    if (this.#operationClose) {
      if (this.#operationClose.terminalIsEmpty()) {
        this.#operationClose = null;
        return step8("pending", "native-operation-retirement-release", 64);
      }
      return childStep2(this.#operationClose.advance(grant), grant);
    }
    if (this.#input) {
      if (this.#input.terminalIsEmpty()) {
        this.#input = null;
        return step8("pending", "native-input-release", 64);
      }
      return childStep2(this.#input.closeStep(grant), grant);
    }
    if (this.#patch) {
      if (this.#patch.terminalIsEmpty()) {
        this.#patch = null;
        return step8("pending", "native-surface-release", 64);
      }
      return childStep2(this.#patch.closeStep(grant), grant);
    }
    return step8("complete", "native-patch-close");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#patch && !this.#input && !this.#operation && !this.#operationClose && !this.#receipt;
  }
}

/* ../../../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts */
var CONTENT_MAGIC = [115, 114, 116, 1];
var CONTENT_STATUS = ["idle", "moreWork", "checkpointReady", "faulted"];
function contentFault(reason) {
  throw new Error(`return-content.${reason}`);
}

class ContentUnsigned {
  #value = 0n;
  #count = 0;
  get complete() {
    return this.#count === 0;
  }
  push(byte) {
    if (this.#count === 9 && byte > 1)
      return contentFault("integer-overflow");
    this.#value |= BigInt(byte & 127) << BigInt(this.#count * 7);
    if (byte & 128) {
      this.#count++;
      return null;
    }
    if (this.#count && byte === 0)
      return contentFault("noncanonical-integer");
    const value = this.#value;
    this.#value = 0n;
    this.#count = 0;
    return value;
  }
}

class ContentSections {
  #unsigned = new ContentUnsigned;
  #section = -1;
  #field = 0;
  #status = 0;
  #nextWake = null;
  #fuelUsed = 0n;
  #effects = 0n;
  #presence = 0n;
  #operations = 0n;
  #surface = 0n;
  #metadata = null;
  #patchActivation = 0n;
  #patchInstance = 0;
  #patchGuest = 0n;
  #patchSequence = 0n;
  #uiReceipt = null;
  get metadata() {
    return this.#metadata;
  }
  get uiReceipt() {
    return this.#uiReceipt;
  }
  begin(tag, length) {
    const beforeEffects = this.#section === 0 || this.#section === 1 || this.#section === 4 || this.#section === 5;
    const beforePresence = beforeEffects || this.#section === 6;
    let allowed = false;
    switch (tag) {
      case 0:
        allowed = this.#section === -1;
        if (length < 5n || length > 42n)
          contentFault("metadata-length");
        break;
      case 1:
        allowed = this.#section === 0;
        if (length === 0n || length > 44n)
          contentFault("lifecycle-length");
        break;
      case 2:
        allowed = this.#section === 0 || this.#section === 1;
        if (length < 9n)
          contentFault("ui-begin-length");
        break;
      case 3:
        allowed = (this.#section === 2 || this.#section === 3) && this.#operations > 0n;
        if (length < 2n)
          contentFault("ui-operation-length");
        break;
      case 4:
        allowed = (this.#section === 2 || this.#section === 3) && this.#operations === 0n;
        break;
      case 5:
        allowed = beforeEffects && this.#effects > 0n;
        break;
      case 6:
        allowed = beforePresence && this.#effects === 0n && this.#presence > 0n;
        break;
      case 7:
        allowed = beforePresence && this.#effects === 0n && this.#presence === 0n;
        break;
      case 8:
        allowed = this.#section === 7 && this.#status >= 2;
        break;
      case 9:
        allowed = this.#section === (this.#status >= 2 ? 8 : 7);
        break;
    }
    if (!allowed)
      contentFault("section-order");
    if (tag === 4 || tag === 9) {
      if (length !== 0n)
        contentFault("empty-record-length");
    } else if (length === 0n)
      contentFault("empty-body");
    this.#section = tag;
    this.#field = 0;
  }
  byte(byte) {
    if (this.#section === 0)
      this.#metadataByte(byte);
    else if (this.#section === 2)
      this.#uiBeginByte(byte);
    else if (this.#section === 3 && this.#field === 0) {
      if (byte > 10)
        contentFault("ui-opcode");
      this.#field = 1;
    }
  }
  #metadataByte(byte) {
    if (this.#field === 0) {
      if (byte > 3)
        contentFault("status");
      this.#status = byte;
      this.#field = 1;
      return;
    }
    if (this.#field === 1) {
      if (byte > 1)
        contentFault("next-wake-option");
      this.#field = byte === 0 ? 3 : 2;
      return;
    }
    if (this.#field > 5)
      contentFault("metadata-trailing");
    const value = this.#unsigned.push(byte);
    if (value === null)
      return;
    switch (this.#field++) {
      case 2:
        this.#nextWake = value;
        break;
      case 3:
        this.#fuelUsed = value;
        break;
      case 4:
        this.#effects = value;
        break;
      case 5:
        this.#presence = value;
        break;
    }
  }
  #uiBeginByte(byte) {
    if (this.#surface > 0n) {
      this.#surface--;
      return;
    }
    if (this.#field > 7)
      contentFault("ui-begin-trailing");
    const value = this.#unsigned.push(byte);
    if (value === null)
      return;
    const field = this.#field++;
    if ((field === 0 || field === 2 || field === 3) && value === 0n)
      contentFault("patch-authority");
    if (field === 1 && value > 0xffffffffn)
      contentFault("instance-overflow");
    if (field === 0)
      this.#patchActivation = value;
    if (field === 1)
      this.#patchInstance = Number(value);
    if (field === 2)
      this.#patchGuest = value;
    if (field === 3)
      this.#patchSequence = value;
    if (field === 4) {
      if (value === 0n)
        contentFault("surface-length");
      this.#surface = value;
    }
    if (field === 7) {
      if (value > 1153n)
        contentFault("operation-count");
      this.#operations = value;
    }
  }
  end() {
    if (!this.#unsigned.complete)
      contentFault("truncated-integer");
    switch (this.#section) {
      case 0:
        if (this.#field !== 6)
          contentFault("truncated-metadata");
        this.#metadata = Object.freeze({ status: CONTENT_STATUS[this.#status], nextWake: this.#nextWake, fuelUsed: this.#fuelUsed, effectCount: this.#effects, presenceCount: this.#presence });
        break;
      case 2:
        if (this.#field !== 8 || this.#surface !== 0n)
          contentFault("truncated-ui-begin");
        this.#uiReceipt = Object.freeze({ lifetime: Object.freeze({ activationGeneration: this.#patchActivation, instanceId: this.#patchInstance, guestLifetime: this.#patchGuest }), patchSequence: this.#patchSequence });
        break;
      case 3:
        this.#operations--;
        break;
      case 5:
        this.#effects--;
        break;
      case 6:
        this.#presence--;
        break;
    }
  }
}

class KernelReturnContentFraming {
  #unsigned = new ContentUnsigned;
  #sections = new ContentSections;
  #phase = "magic";
  #magic = 0;
  #tag = -1;
  #length = 0n;
  #remaining = 0n;
  #failure = null;
  constructor() {
    Object.freeze(this);
  }
  get tag() {
    return this.#tag;
  }
  get length() {
    return this.#length;
  }
  get remaining() {
    return this.#remaining;
  }
  get complete() {
    return this.#phase === "done" && this.#failure === null;
  }
  get failure() {
    return this.#failure;
  }
  get metadata() {
    return this.#sections.metadata;
  }
  get uiReceipt() {
    return this.#sections.uiReceipt;
  }
  #end() {
    this.#sections.end();
    this.#phase = this.#tag === 9 ? "done" : "tag";
  }
  push(byte) {
    if (this.#failure !== null)
      throw new Error(this.#failure);
    try {
      if (!Number.isInteger(byte) || byte < 0 || byte > 255)
        contentFault("byte");
      switch (this.#phase) {
        case "magic":
          if (byte !== CONTENT_MAGIC[this.#magic++])
            contentFault("magic");
          if (this.#magic === CONTENT_MAGIC.length)
            this.#phase = "tag";
          return "prefix";
        case "tag":
          if (byte > 9)
            contentFault("record-tag");
          this.#tag = byte;
          this.#phase = "length";
          return "prefix";
        case "length": {
          const length = this.#unsigned.push(byte);
          if (length === null)
            return "prefix";
          this.#sections.begin(this.#tag, length);
          this.#length = length;
          this.#remaining = length;
          this.#phase = "body";
          if (length === 0n)
            this.#end();
          return "header";
        }
        case "body":
          this.#sections.byte(byte);
          this.#remaining--;
          if (this.#remaining === 0n)
            this.#end();
          return "body";
        case "done":
          return contentFault("trailing");
      }
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "return-content.fault";
      throw error;
    }
  }
  finish() {
    if (this.#failure !== null)
      throw new Error(this.#failure);
    if (!this.complete) {
      this.#failure = "return-content.truncated";
      throw new Error(this.#failure);
    }
  }
}
var UI_FIELDS = ["node", "component", "layout", "activity", "children", "style", "accessibility", "bindings", "menu", null, null];

class KernelReturnUiOperationHeader {
  #unsigned = new ContentUnsigned;
  #remaining;
  #phase = "opcode";
  #opcode = -1;
  #node = null;
  #count = 0;
  #value = null;
  #failure = null;
  constructor(bodyLength) {
    if (typeof bodyLength !== "bigint" || bodyLength < 2n || bodyLength > 0xffffffffffffffffn)
      contentFault("ui-header-length");
    this.#remaining = bodyLength;
    Object.freeze(this);
  }
  get value() {
    return this.#value;
  }
  get failure() {
    return this.#failure;
  }
  #complete() {
    this.#value = Object.freeze({ opcode: this.#opcode, node: this.#node, field: UI_FIELDS[this.#opcode], payloadLength: this.#remaining, headerLength: this.#count });
    this.#phase = "done";
  }
  push(byte) {
    if (this.#failure !== null)
      throw new Error(this.#failure);
    try {
      if (!Number.isInteger(byte) || byte < 0 || byte > 255)
        contentFault("byte");
      if (this.#phase === "done")
        contentFault("ui-header-complete");
      this.#remaining--;
      this.#count++;
      if (this.#phase === "opcode") {
        if (byte > 10)
          contentFault("ui-opcode");
        this.#opcode = byte;
        this.#phase = byte === 0 ? "length" : "node";
      } else {
        const value = this.#unsigned.push(byte);
        if (value !== null && this.#phase === "node") {
          this.#node = value;
          if (this.#opcode >= 9) {
            if (this.#remaining !== 0n)
              contentFault("ui-scalar-trailing");
            this.#complete();
          } else if (this.#opcode === 4) {
            if (this.#remaining === 0n)
              contentFault("ui-children-count");
            this.#complete();
          } else
            this.#phase = "length";
        } else if (value !== null) {
          if (value !== this.#remaining)
            contentFault("ui-payload-length");
          this.#complete();
        }
      }
      if (this.#remaining === 0n && this.#value === null)
        contentFault("ui-header-truncated");
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "return-content.ui-header-fault";
      throw error;
    }
  }
  finish() {
    if (this.#failure !== null)
      throw new Error(this.#failure);
    if (this.#value === null) {
      this.#failure = "return-content.ui-header-truncated";
      throw new Error(this.#failure);
    }
  }
}
if (undefined) {}
if (undefined) {}

/* ../../../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts */
var MINT9 = Object.freeze({});
var NO_INPUT_FAULT = Symbol("return-input.no-fault");
var BUILDER_CONSUMED = Symbol("return-input.builder-consumed");
var result2 = (kind, bytes = 0) => ({ kind, items: bytes ? 1 : 0, bytes });
var payloadGrant = (grant) => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= 64;
function inputRejected(state7, code, bytes) {
  state7.failure = code;
  return result2("rejected", bytes);
}
function inputFault(state7, error, bytes) {
  if (state7.fault === NO_INPUT_FAULT)
    state7.fault = error;
  else if (!Object.is(state7.fault, error))
    throw error;
  return inputRejected(state7, "return-input.fault", bytes);
}
var mintField;
var mintFragment;
var fieldBuilder;
var fieldReadable;
var recordFieldRelease;
var fieldOwnsRelease;
var retainFieldFault;
var mintRelease;
var installFragmentRelease;
var detachFragmentField;
var clearFragmentRelease;
var releaseKind;
var detachInputRelease;
var settleInputRelease;
var installFieldFragment;
var mintPayloadDetachment;
var installPayloadDetachment;
var ownsPayloadDetachment;
var payloadDetachmentPhase;
var bindPayloadDetachment;
var detachPayload;
var settlePayload;

class OwnedKernelReturnContent {
  #state;
  constructor(source, owner, activation, lifetime) {
    if (!OwnedShardReturn.matchesOwner(source, owner, activation, lifetime))
      throw new Error("return-input.owner");
    this.#state = { source, owner, activation, lifetime: Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime }), framing: new KernelReturnContentFraming, content: this, page: null, cursor: 0, operation: 0, header: null, field: null, failure: null, fault: NO_INPUT_FAULT, closing: false };
    if (!source.bindContent(this))
      throw new Error("return-input.content-owned");
    Object.freeze(this);
  }
  static matches(content, source, owner, activation, lifetime) {
    if (content === null || typeof content !== "object" || !(#state in content))
      return false;
    const state7 = content.#state;
    return state7.source === source && state7.owner === owner && state7.activation === activation && actorInstanceLifetimeEquals(state7.lifetime, lifetime);
  }
  get field() {
    return this.#state.field;
  }
  get failure() {
    return this.#state.failure;
  }
  static matchesFault(content, fault) {
    return content !== null && typeof content === "object" && #state in content && content.#state.fault !== NO_INPUT_FAULT && Object.is(content.#state.fault, fault);
  }
  advance(grant) {
    const state7 = this.#state;
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || state7.closing)
      return result2("blocked");
    if (state7.failure !== null)
      return result2("rejected");
    if (state7.field !== null)
      return result2("ready");
    try {
      const page = state7.source.page;
      if (page === null)
        return result2("blocked");
      if (!OwnedShardReturnPage.matchesOwner(page, state7.owner, state7.activation, state7.lifetime))
        return inputRejected(state7, "return-input.page-owner", 1);
      if (state7.page === null) {
        if (page.receipt.pageSequence !== 1n)
          return inputRejected(state7, "return-input.page-sequence", 1);
        state7.page = page;
      }
      if (state7.page !== page)
        return inputRejected(state7, "return-input.page-pending", 1);
      if (state7.cursor >= page.receipt.length) {
        if (page.receipt.final)
          state7.framing.finish();
        return result2("blocked");
      }
      const byte = page.byteAt(state7.cursor);
      const kind = state7.framing.push(byte);
      state7.cursor++;
      if (kind === "header" && state7.framing.tag === 3)
        state7.header = new KernelReturnUiOperationHeader(state7.framing.length);
      if (kind === "body" && state7.framing.tag === 2 && state7.framing.remaining === 0n) {
        const receipt = state7.framing.uiReceipt;
        if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state7.lifetime))
          return inputRejected(state7, "return-input.patch-lifetime", 1);
      }
      if (kind === "body" && state7.framing.tag === 3 && state7.header !== null) {
        state7.header.push(byte);
        const fields = state7.header.value;
        if (fields !== null) {
          const receipt = state7.framing.uiReceipt;
          if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state7.lifetime))
            return inputRejected(state7, "return-input.patch-lifetime", 1);
          if (fields.field !== null)
            state7.field = mintField(state7, Object.freeze({ operation: state7.operation, opcode: fields.opcode, node: fields.node, name: fields.field, byteLength: fields.payloadLength, receipt }), page, state7.cursor);
          state7.operation++;
          state7.header = null;
        }
      }
      return result2(state7.field ? "ready" : "pending", 1);
    } catch (error) {
      return inputFault(state7, error, 1);
    }
  }
  beginClose() {
    const state7 = this.#state;
    state7.closing = true;
    state7.field?.beginClose();
  }
}

class OwnedKernelReturnInputField {
  #state;
  #value;
  #builder = null;
  #fragment = null;
  #release = null;
  #consumed = 0n;
  #advanced = 0;
  #start;
  #complete = false;
  #closing = false;
  #payload = null;
  #payloadDetachment = null;
  constructor(mint, state7, value, page, start) {
    if (mint !== MINT9)
      throw new Error("return-input.private-field");
    this.#state = state7;
    this.#value = value;
    this.#start = start;
    state7.field = this;
    mintPayloadDetachment(this);
    const available = page.receipt.length - start;
    this.#fragment = mintFragment(this, page, start, 0n, Number(value.byteLength < BigInt(available) ? value.byteLength : BigInt(available)));
    Object.freeze(this);
  }
  static {
    mintField = (state7, value, page, start) => new OwnedKernelReturnInputField(MINT9, state7, value, page, start);
    fieldBuilder = (field) => OwnedUiOperationPayloadBuilder.hasBrand(field.#builder) ? field.#builder : null;
    fieldReadable = (field) => !field.#closing && !field.#state.closing && field.#state.failure === null;
    installFieldFragment = (field, fragment) => {
      if (field.#fragment !== null)
        throw new Error("return-input.fragment-owned");
      field.#fragment = fragment;
    };
    installPayloadDetachment = (field, observation) => {
      if (field.#payloadDetachment !== null)
        throw new Error("return-input.payload-observation-owned");
      field.#payloadDetachment = observation;
    };
    ownsPayloadDetachment = (field, observation) => field !== null && typeof field === "object" && (#payloadDetachment in field) && field.#payloadDetachment === observation;
    fieldOwnsRelease = (field, release2) => field !== null && typeof field === "object" && (#release in field) && field.#release === release2 && field.#fragment !== null;
    retainFieldFault = (field, error) => {
      inputFault(field.#state, error, 0);
    };
    recordFieldRelease = (field, fragment, release2) => {
      if (field.#fragment !== fragment || field.#release !== null || field.#state.field !== field)
        return false;
      field.#release = release2;
      if (releaseKind(release2) === "cancelled")
        field.#closing = true;
      return true;
    };
  }
  static matchesOwner(field, owner, activation, lifetime) {
    if (field === null || typeof field !== "object" || !(#state in field))
      return false;
    const state7 = field.#state;
    return state7.field === field && state7.owner === owner && state7.activation === activation && actorInstanceLifetimeEquals(state7.lifetime, lifetime) && OwnedShardReturn.matchesOwner(state7.source, owner, activation, lifetime);
  }
  static matchesBuilder(field, builder) {
    return field !== null && typeof field === "object" && #builder in field && OwnedUiOperationPayloadBuilder.hasBrand(builder) && field.#builder === builder;
  }
  static matchesBuilderDetached(field, proof) {
    return field !== null && typeof field === "object" && #builder in field && field.#builder !== null && field.#builder !== BUILDER_CONSUMED && !OwnedUiOperationPayloadBuilder.hasBrand(field.#builder) && field.#builder === proof;
  }
  static matchesBuilderSettled(field, proof) {
    return field !== null && typeof field === "object" && #builder in field && field.#builder === BUILDER_CONSUMED && BuilderWitness.matchesSourceBinding(proof, field);
  }
  detachBuilder(builder, proof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    if (this.#state.field !== this || this.#release !== null || this.#builder !== null && this.#builder !== builder || !BuilderWitness.matchesBody(proof, builder, this))
      return result2("rejected");
    this.#closing = true;
    this.#builder = proof;
    return result2("pending", 64);
  }
  settleBuilder(proof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    if (this.#state.field !== this || this.#builder !== proof || !BuilderWitness.matchesDetached(proof, this))
      return result2("rejected");
    this.#builder = BUILDER_CONSUMED;
    return result2("complete", 64);
  }
  static matchesResidentPayload(field, payload) {
    return field !== null && typeof field === "object" && #payload in field && field.#payload !== null && field.#payload === payload;
  }
  installResidentPayload(payload, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    const observation = this.#payloadDetachment;
    const state7 = this.#state;
    if (!observation || !fieldReadable(this) || state7.field !== this || !OwnedUiResidentPayload.matchesField(payload, this) || !OwnedUiResidentPayload.matchesOwner(payload, state7.owner, state7.activation, state7.lifetime))
      return result2("rejected");
    if (payloadDetachmentPhase(observation) === "bound" && this.#payload === payload)
      return result2("ready");
    if (payloadDetachmentPhase(observation) !== "unbound" || this.#payload !== null)
      return result2("rejected");
    bindPayloadDetachment(observation, payload);
    this.#payload = payload;
    return result2("ready", 64);
  }
  residentPayload(scope) {
    const payload = this.#payload;
    return payload !== null && OwnedUiResidentPayload.matchesScope(payload, scope) ? payload : null;
  }
  get residentPayloadDetachment() {
    const observation = this.#payloadDetachment;
    if (!observation)
      return null;
    const phase = payloadDetachmentPhase(observation);
    return phase === "detached" || phase === "settled" ? observation : null;
  }
  detachResidentPayload(payload, proof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    const observation = this.#payloadDetachment;
    if (!observation || !OwnedUiResidentPayloadSourceRelease.matches(proof, payload, this))
      return result2("rejected");
    const phase = payloadDetachmentPhase(observation);
    if (!(phase === "bound" && this.#payload === payload || phase === "unbound" && this.#payload === null))
      return result2("rejected");
    detachPayload(observation, payload, proof);
    this.#payload = null;
    return result2("pending", 64);
  }
  settleResidentPayload(detachment, sourceDetachedProof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    if (detachment !== this.#payloadDetachment || this.#payload !== null || !settlePayload(detachment, sourceDetachedProof))
      return result2("rejected");
    return result2("complete", 64);
  }
  get value() {
    return this.#value;
  }
  get owner() {
    return this.#state.owner;
  }
  get activation() {
    return this.#state.activation;
  }
  get lifetime() {
    return this.#state.lifetime;
  }
  get fragment() {
    return this.#fragment;
  }
  get consumed() {
    return this.#consumed;
  }
  get complete() {
    return this.#complete;
  }
  bind(builder) {
    if (!fieldReadable(this) || this.#builder !== null && this.#builder !== builder || !OwnedUiOperationPayloadBuilder.matchesField(builder, this))
      return false;
    this.#builder = builder;
    return true;
  }
  detachInputRelease(release2, proof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    const fragment = this.#fragment;
    if (!fragment || this.#release !== release2 || !OwnedKernelReturnInputRelease.matches(release2, fragment, proof) || !uiMatchesRelease(proof, release2))
      return result2("rejected");
    if (releaseKind(release2) === "copied" && !this.#closing && !this.#state.closing && (this.#advanced !== fragment.length || this.#consumed !== fragment.offset + BigInt(fragment.length)))
      return result2("rejected");
    detachFragmentField(fragment);
    detachInputRelease(release2);
    return result2("pending", 64);
  }
  settleInputRelease(release2, proof, grant) {
    if (!payloadGrant(grant))
      return result2("blocked");
    const fragment = this.#fragment;
    if (!fragment || !OwnedKernelReturnInputRelease.matchesSourceDetached(release2, this, proof) || !uiMatchesSourceDetached(proof, release2))
      return result2("rejected");
    clearFragmentRelease(fragment, release2);
    this.#fragment = null;
    this.#release = null;
    this.#advanced = 0;
    this.#start = this.#state.cursor;
    settleInputRelease(release2);
    return result2("complete", 64);
  }
  advance(grant, builder) {
    const state7 = this.#state;
    if (!OwnedUiOperationPayloadBuilder.hasBrand(builder) || this.#builder !== builder)
      return result2("rejected");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || this.#closing || state7.closing)
      return result2("blocked");
    if (state7.failure !== null)
      return result2("rejected");
    if (this.#complete)
      return result2("complete");
    const fragment = this.#fragment;
    if (!fragment || !this.#release || releaseKind(this.#release) !== "copied")
      return result2("blocked");
    let bytes = 0;
    try {
      const page = state7.page;
      if (state7.field !== this || !page || state7.source.page !== page || state7.cursor !== this.#start + this.#advanced || fragment.offset + BigInt(this.#advanced) !== this.#consumed)
        return inputRejected(state7, "return-input.continuation-owner", bytes);
      if (this.#advanced < fragment.length) {
        if (state7.cursor >= page.receipt.length || state7.framing.tag !== 3 || state7.framing.remaining === 0n)
          return inputRejected(state7, "return-input.continuation-range", bytes);
        bytes = 1;
        if (state7.framing.push(page.byteAt(state7.cursor)) !== "body")
          return inputRejected(state7, "return-input.continuation-framing", bytes);
        state7.cursor++;
        this.#advanced++;
        this.#consumed++;
      }
      if (this.#advanced < fragment.length)
        return result2("pending", bytes);
      if (this.#consumed === this.#value.byteLength) {
        if (state7.framing.remaining !== 0n)
          return inputRejected(state7, "return-input.continuation-trailing", bytes);
        this.#complete = true;
      }
      if (this.#consumed > this.#value.byteLength)
        return inputRejected(state7, "return-input.continuation-overflow", bytes);
      return result2(this.#complete ? "complete" : "pending", bytes);
    } catch (error) {
      return inputFault(state7, error, bytes);
    }
  }
  beginClose() {
    this.#closing = true;
  }
}

class OwnedKernelReturnPayloadDetachment {
  #payload = null;
  #proof = null;
  #phase = "unbound";
  constructor(mint, field) {
    if (mint !== MINT9)
      throw new Error("return-input.private-payload-detachment");
    installPayloadDetachment(field, this);
    Object.freeze(this);
  }
  static {
    mintPayloadDetachment = (field) => new OwnedKernelReturnPayloadDetachment(MINT9, field);
    payloadDetachmentPhase = (observation) => observation.#phase;
    bindPayloadDetachment = (observation, payload) => {
      observation.#payload = payload;
      observation.#phase = "bound";
    };
    detachPayload = (observation, payload, proof) => {
      observation.#payload = payload;
      observation.#proof = proof;
      observation.#phase = "detached";
    };
    settlePayload = (observation, proof) => {
      const payload = observation.#payload;
      if (observation.#phase !== "detached" || payload === null || observation.#proof !== proof || !OwnedUiResidentPayloadSourceRelease.matchesDetached(proof, payload) || !OwnedUiResidentPayload.matchesSourceDetachment(payload, observation))
        return false;
      observation.#payload = null;
      observation.#proof = null;
      observation.#phase = "settled";
      return true;
    };
  }
  static matchesOwner(observation, field) {
    return observation !== null && typeof observation === "object" && #phase in observation && ownsPayloadDetachment(field, observation);
  }
  static matches(observation, field, payload) {
    return OwnedKernelReturnPayloadDetachment.matchesOwner(observation, field) && observation.#phase === "detached" && observation.#payload !== null && observation.#payload === payload;
  }
  static matchesSettled(observation, payload) {
    return observation !== null && typeof observation === "object" && #phase in observation && observation.#phase === "settled" && observation.#payload === null && observation.#proof === null && OwnedUiResidentPayload.matchesSourceDetachment(payload, observation);
  }
}

class OwnedKernelReturnInputFragment {
  #field;
  #page;
  #start;
  #offset;
  #length;
  #release = null;
  constructor(mint, field, page, start, offset, length) {
    if (mint !== MINT9)
      throw new Error("return-input.private-fragment");
    this.#field = field;
    this.#page = page;
    this.#start = start;
    this.#offset = offset;
    this.#length = length;
    installFieldFragment(field, this);
    Object.freeze(this);
  }
  static {
    mintFragment = (field, page, start, offset, length) => new OwnedKernelReturnInputFragment(MINT9, field, page, start, offset, length);
    installFragmentRelease = (fragment, release2) => {
      if (!fragment.#field || fragment.#release !== null || !recordFieldRelease(fragment.#field, fragment, release2))
        throw new Error("return-input.release-owned");
      fragment.#release = release2;
      fragment.#page = null;
    };
    detachFragmentField = (fragment) => {
      fragment.#field = null;
      fragment.#page = null;
    };
    clearFragmentRelease = (fragment, release2) => {
      if (fragment.#release !== release2 || fragment.#field !== null || fragment.#page !== null)
        throw new Error("return-input.release-detachment");
      fragment.#release = null;
    };
  }
  static matches(fragment, field) {
    return fragment !== null && typeof fragment === "object" && #field in fragment && fragment.#field === field;
  }
  get field() {
    return this.#field;
  }
  get offset() {
    return this.#offset;
  }
  get length() {
    return this.#length;
  }
  byteAt(index, builder) {
    if (!this.#field || fieldBuilder(this.#field) !== builder || !OwnedUiOperationPayloadBuilder.matchesField(builder, this.#field))
      throw new Error("return-input.builder");
    if (!fieldReadable(this.#field) || this.#page === null || !Number.isSafeInteger(index) || index < 0 || index >= this.#length)
      throw new Error("return-input.fragment-read");
    return this.#page.byteAt(this.#start + index);
  }
  release(proof) {
    if (this.#release)
      return OwnedKernelReturnInputRelease.matches(this.#release, this, proof) ? this.#release : null;
    const field = this.#field;
    if (!field)
      return null;
    const builder = fieldBuilder(field);
    if (!builder)
      return null;
    const copied = OwnedUiOperationInputCopied.matches(proof, this, field, builder, this.#offset, this.#length);
    const cancelled = !copied && OwnedUiOperationInputCancelled.matches(proof, this, field, builder, this.#offset, this.#length);
    if (!copied && !cancelled)
      return null;
    try {
      return mintRelease(this, proof, copied ? "copied" : "cancelled");
    } catch (error) {
      retainFieldFault(field, error);
      throw error;
    }
  }
}
function uiMatchesRelease(proof, release2) {
  return OwnedUiOperationInputCopied.matchesRelease(proof, release2) || OwnedUiOperationInputCancelled.matchesRelease(proof, release2);
}
function uiMatchesSourceDetached(proof, release2) {
  return OwnedUiOperationInputCopied.matchesSourceDetached(proof, release2) || OwnedUiOperationInputCancelled.matchesSourceDetached(proof, release2);
}

class OwnedKernelReturnInputRelease {
  #fragment;
  #proof;
  #kind;
  #phase = "issued";
  constructor(mint, fragment, proof, kind) {
    if (mint !== MINT9)
      throw new Error("return-input.private-release");
    this.#fragment = fragment;
    this.#proof = proof;
    this.#kind = kind;
    installFragmentRelease(fragment, this);
    Object.freeze(this);
  }
  static {
    mintRelease = (fragment, proof, kind) => new OwnedKernelReturnInputRelease(MINT9, fragment, proof, kind);
    releaseKind = (release2) => release2.#kind;
    detachInputRelease = (release2) => {
      release2.#fragment = null;
      release2.#proof = null;
      release2.#phase = "sourceDetached";
    };
    settleInputRelease = (release2) => {
      release2.#phase = "settled";
    };
  }
  static matches(receipt, fragment, proof) {
    return receipt !== null && typeof receipt === "object" && #fragment in receipt && receipt.#phase === "issued" && receipt.#fragment === fragment && receipt.#proof === proof;
  }
  static matchesSourceDetached(receipt, field, proof) {
    return receipt !== null && typeof receipt === "object" && #phase in receipt && receipt.#phase === "sourceDetached" && receipt.#fragment === null && receipt.#proof === null && fieldOwnsRelease(field, receipt) && uiMatchesRelease(proof, receipt);
  }
  static matchesSettled(receipt, proof) {
    return receipt !== null && typeof receipt === "object" && #phase in receipt && receipt.#phase === "settled" && receipt.#fragment === null && receipt.#proof === null && uiMatchesRelease(proof, receipt);
  }
  get kind() {
    return this.#kind;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts */
var MINT10 = Object.freeze({});
var NO_FAILURE = Object.freeze({});
var admitted17 = (grant, bytes) => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
var step9 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var cancelledProof;
var copiedProof;
var copiedState;
var cancelledState;
var builderBrand;
function evidenceState(token) {
  const value = copiedState(token) ?? cancelledState(token);
  return value !== null && !builderBrand(value) ? value : null;
}
function childStep3(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return { ...current, kind: "rejected" };
  return current.kind === "ready" || current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function exact2(proof, fragment, field, builder, offset, length) {
  return typeof offset === "bigint" && offset >= 0n && offset <= 18446744073709551615n && Number.isSafeInteger(length) && length >= 0 && length <= 4096 && offset + BigInt(length) <= 18446744073709551615n && (proof.phase === "available" || proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing") && proof.fragment === fragment && proof.field === field && proof.builder === builder && proof.offset === offset && proof.length === length && OwnedUiOperationPayloadBuilder.matchesField(builder, field);
}
function evidenceRelease(proof, token, release2) {
  return proof.token === token && proof.release !== null && proof.release === release2 && (proof.phase === "issued" || proof.phase === "source-consuming" || proof.phase === "source-observing" || proof.phase === "source-detached" || proof.phase === "source-settled");
}

class OwnedUiOperationPayloadBuilder {
  #field;
  #resident;
  #bound = false;
  #closing = false;
  #phase = "open";
  #fragment = null;
  #proof = null;
  #release = null;
  #failure = NO_FAILURE;
  #copyPhase = "idle";
  #sourceKind = null;
  #input = null;
  #copyFragment = null;
  #lastFragment = null;
  #inputOffset = 0;
  #copied = 0n;
  #copyProof = null;
  #copyRelease = null;
  #head = null;
  #tail = null;
  #writer = null;
  #written = 0;
  #reader = null;
  #readerPhase = "none";
  static {
    builderBrand = (value) => value !== null && typeof value === "object" && (#field in value);
  }
  constructor(mint, field, resident) {
    if (mint !== MINT10)
      throw new Error("Invalid paged builder authority");
    this.#field = field;
    this.#resident = resident;
    const installed = resident.installBuilder(this, { maxItems: 1, maxBytes: 64 });
    if (installed.kind !== "ready" || installed.bytes !== 64)
      throw new Error("Paged builder registration refused");
  }
  static matchesField(builder, field) {
    return builder !== null && typeof builder === "object" && #field in builder && builder.#field === field && builder.#resident !== null;
  }
  static matchesResident(builder, resident) {
    return builder !== null && typeof builder === "object" && #resident in builder && builder.#resident === resident;
  }
  static hasBrand(value) {
    return builderBrand(value);
  }
  static activeInput(builder) {
    return builder.#phase === "open" && (builder.#input !== null || builder.#copyFragment !== null);
  }
  static cancellationPrepared(builder) {
    return builder.#closing && builder.#phase === "proof" && builder.#fragment !== null && !builder.#input && !builder.#copyFragment && builder.#sourceKind === null;
  }
  static prepareInputCancellation(builder, resident, grant) {
    if (!admitted17(grant, 128))
      return step9("blocked", "paged-active-input-detach");
    if (!OwnedUiResidentPayload.matchesInputCancellation(resident, builder) || builder.#resident !== resident || builder.#failure !== NO_FAILURE || !OwnedUiOperationPayloadBuilder.activeInput(builder) || builder.#proof || builder.#release || builder.#copyProof || builder.#copyRelease)
      return step9("rejected", "paged-active-input-owner");
    const fragment = builder.#copyFragment ?? builder.#input;
    if (!fragment || !builder.#field || !OwnedKernelReturnInputFragment.matches(fragment, builder.#field) || builder.#input !== null && builder.#input !== fragment)
      return step9("rejected", "paged-active-input-fragment");
    builder.#closing = true;
    builder.#fragment = fragment;
    builder.#input = null;
    builder.#copyFragment = null;
    builder.#sourceKind = null;
    builder.#copyPhase = "idle";
    builder.#phase = "proof";
    return step9("pending", "paged-active-input-detach", 128);
  }
  static evidenceEligible(builder, resident) {
    return builderBrand(builder) && builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#bound && builder.#field !== null && (builder.#closing && !builder.#input && !builder.#copyProof || builder.#copyPhase === "proof" && builder.#input === null) && (builder.#copyFragment !== null || builder.#fragment !== null || builder.#field.fragment !== null);
  }
  static matchesEvidenceConstruction(token, builder) {
    return copiedState(token) === builder || cancelledState(token) === builder;
  }
  static matchesEvidence(token, builder) {
    const proof = evidenceState(token);
    return proof !== null && proof.token === token && proof.builder === builder;
  }
  static constructEvidence(builder, resident, grant) {
    if (!admitted17(grant, 168) || !OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "constructing") || !OwnedUiOperationPayloadBuilder.evidenceEligible(builder, resident))
      throw new Error("Invalid original evidence reservation");
    const fragment = builder.#copyFragment ?? builder.#fragment ?? builder.#field.fragment;
    if (!fragment || !OwnedKernelReturnInputFragment.matches(fragment, builder.#field))
      throw new Error("Missing exact evidence fragment");
    return builder.#copyPhase === "proof" ? copiedProof(builder, resident, fragment) : cancelledProof(builder, resident, fragment);
  }
  static finalizeEvidence(token, builder, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-evidence-finalization");
    if (!OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "witness-ready") || !OwnedUiOperationPayloadBuilder.matchesEvidence(token, builder))
      return step9("rejected", "paged-evidence-finalization");
    Object.freeze(token);
    return step9("pending", "paged-evidence-finalization", 64);
  }
  static publishEvidence(token, builder, resident) {
    const proof = evidenceState(token);
    if (!OwnedUiResidentPayload.matchesEvidencePhase(resident, builder, "finalized") || !proof || proof.builder !== builder || proof.token !== token || proof.phase !== "constructed")
      throw new Error("Invalid evidence publication");
    proof.phase = "available";
  }
  static evidenceEmpty(token) {
    const proof = evidenceState(token);
    return proof !== null && proof.phase === "domain-retired" && !proof.fragment && !proof.field && !proof.builder && !proof.release && !proof.token;
  }
  static advanceEvidence(token, builder, resident, grant) {
    const proof = evidenceState(token);
    if (!proof || !OwnedUiResidentPayload.matchesEvidenceRetirement(resident, builder, token) || builder.#failure !== NO_FAILURE || !builder.#field)
      return step9("rejected", "paged-evidence-owner");
    const cancelling = OwnedUiResidentPayload.matchesEvidenceCancellation(resident, builder, token);
    if (!admitted17(grant, proof.phase === "available" ? 128 : proof.phase === "source-consuming" && !cancelling ? 1 : 64))
      return step9("blocked", "paged-evidence-retirement");
    const field = builder.#field;
    if (proof.phase === "available") {
      const fragment = proof.fragment;
      if (!OwnedKernelReturnInputFragment.matches(fragment, field) || proof.builder !== builder || proof.token !== token)
        return step9("rejected", "paged-evidence-release-owner");
      const receipt = fragment.release(token);
      if (!receipt || !OwnedKernelReturnInputRelease.matches(receipt, fragment, token))
        return step9("rejected", "paged-evidence-release-refused", 128);
      proof.release = receipt;
      if (copiedState(token) === proof) {
        builder.#inputOffset = 0;
        proof.phase = "source-consuming";
      } else
        proof.phase = "issued";
      return step9("pending", "paged-evidence-release", 128);
    }
    if (cancelling && (proof.phase === "source-consuming" || proof.phase === "source-observing")) {
      field.beginClose();
      builder.#sourceKind = null;
      proof.phase = "issued";
      return step9("pending", "paged-evidence-source-close", 64);
    }
    if (proof.phase === "source-consuming") {
      if (copiedState(token) !== proof || proof.builder !== builder || !proof.release || builder.#input || builder.#inputOffset > proof.length)
        return step9("rejected", "paged-evidence-source-owner");
      const current = field.advance(grant, builder);
      const forwarded = childStep3({ ...current, phase: "paged-evidence-source-advance" }, grant);
      if (forwarded.kind === "rejected") {
        builder.#retainFailure(current);
        return forwarded;
      }
      if (forwarded.kind === "blocked")
        return forwarded;
      if (current.kind !== "pending" && current.kind !== "complete") {
        builder.#retainFailure(current);
        return { ...forwarded, kind: "rejected" };
      }
      builder.#sourceKind = current.kind;
      proof.phase = "source-observing";
      return forwarded;
    }
    if (proof.phase === "source-observing") {
      const next = builder.#inputOffset + (builder.#inputOffset < proof.length ? 1 : 0);
      const consumed = proof.offset + BigInt(next);
      if (copiedState(token) !== proof || proof.builder !== builder || field.consumed !== consumed || field.complete !== (builder.#sourceKind === "complete") || field.complete !== (consumed === field.value.byteLength) || consumed > builder.#copied) {
        const failure = step9("rejected", "paged-evidence-source-observation");
        builder.#retainFailure(failure);
        return failure;
      }
      builder.#inputOffset = next;
      builder.#sourceKind = null;
      proof.phase = next === proof.length ? "issued" : "source-consuming";
      return step9("pending", "paged-evidence-source-observe", 64);
    }
    if (proof.phase === "issued") {
      if (!proof.release)
        return step9("rejected", "paged-evidence-release-missing");
      if (OwnedKernelReturnInputRelease.matchesSourceDetached(proof.release, field, token)) {
        const fragment = proof.fragment;
        if (builder.#input === fragment && fragment !== null)
          return step9("rejected", "paged-evidence-input-held");
        if (builder.#fragment === fragment)
          builder.#fragment = null;
        if (builder.#copyFragment === fragment)
          builder.#copyFragment = null;
        if (builder.#lastFragment === fragment)
          builder.#lastFragment = null;
        if (builder.#proof === token)
          builder.#proof = null;
        if (builder.#copyProof === token)
          builder.#copyProof = null;
        if (builder.#release === proof.release)
          builder.#release = null;
        if (builder.#copyRelease === proof.release)
          builder.#copyRelease = null;
        proof.fragment = null;
        proof.field = null;
        proof.builder = null;
        proof.phase = "source-detached";
        return step9("pending", "paged-evidence-ui-detach", 64);
      }
      return childStep3({ ...field.detachInputRelease(proof.release, token, grant), phase: "paged-evidence-source-detach" }, grant);
    }
    if (proof.phase === "source-detached") {
      if (!proof.release)
        return step9("rejected", "paged-evidence-release-missing");
      if (OwnedKernelReturnInputRelease.matchesSettled(proof.release, token)) {
        proof.phase = "source-settled";
        return step9("pending", "paged-evidence-ui-settle", 64);
      }
      return childStep3({ ...field.settleInputRelease(proof.release, token, grant), phase: "paged-evidence-source-settle" }, grant);
    }
    if (proof.phase === "source-settled") {
      if (!proof.release || !OwnedKernelReturnInputRelease.matchesSettled(proof.release, token))
        return step9("rejected", "paged-evidence-settled-owner");
      proof.release = null;
      proof.token = null;
      proof.phase = "domain-retired";
      return step9("pending", "paged-evidence-capsule-clear", 64);
    }
    return step9(OwnedUiOperationPayloadBuilder.evidenceEmpty(token) ? "complete" : "rejected", "paged-evidence-body");
  }
  static matchesPage(builder, page, resident) {
    return builder.#resident === resident && builder.#head === page && builder.#tail === page && (builder.#writer === page || builder.#writer === null);
  }
  static installPage(builder, page, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-page-install");
    if (builder.#resident !== resident || builder.#head || builder.#tail || builder.#writer || builder.#failure !== NO_FAILURE || builder.#closing || !OwnedUiResidentPayload.matchesPageConstruction(resident, page))
      return step9("rejected", "paged-page-install");
    builder.#head = page;
    builder.#tail = page;
    builder.#writer = page;
    builder.#written = 0;
    return step9("pending", "paged-page-install", 64);
  }
  static matchesPageDetached(builder, page, proof, resident) {
    return builder.#resident === resident && builder.#head === null && builder.#tail === null && builder.#writer === null && builder.#failure === NO_FAILURE && OwnedUiResidentPayload.matchesPageRetirement(resident, page, proof);
  }
  static detachPage(builder, page, proof, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-page-detach");
    if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.matchesPage(builder, page, resident) || builder.#failure !== NO_FAILURE || !OwnedUiResidentPayload.matchesPageRetirement(resident, page, proof))
      return step9("rejected", "paged-page-detach");
    builder.#head = null;
    builder.#tail = null;
    builder.#writer = null;
    return step9("complete", "paged-page-detach", 64);
  }
  static readerIsEmpty(builder) {
    return builder.#reader === null;
  }
  static readerAvailable(builder, resident) {
    return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "none" && builder.#failure === NO_FAILURE && !builder.#closing;
  }
  static matchesReader(builder, reader, resident) {
    return builder.#resident === resident && builder.#reader === reader && builder.#readerPhase === "held";
  }
  static installReader(builder, reader, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-reader-install");
    if (!OwnedUiOperationPayloadBuilder.readerAvailable(builder, resident) || !OwnedUiResidentPayload.matchesReaderConstruction(resident, reader))
      return step9("rejected", "paged-reader-install");
    builder.#reader = reader;
    builder.#readerPhase = "held";
    return step9("pending", "paged-reader-install", 64);
  }
  static readerEof(builder, resident, consumed) {
    return builder.#resident === resident && builder.#failure === NO_FAILURE && builder.#copyPhase === "ready" && builder.#field !== null && builder.#field.complete && consumed === builder.#field.value.byteLength && consumed === builder.#copied;
  }
  static detachReader(builder, reader, proof, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-reader-detach");
    if (builder.#resident !== resident || builder.#failure !== NO_FAILURE || builder.#reader !== reader && !(builder.#reader === null && builder.#readerPhase === "none") || !OwnedUiResidentReaderRetirement.matchesBody(proof, reader, resident))
      return step9("rejected", "paged-reader-detach");
    builder.#reader = proof;
    builder.#readerPhase = "detached";
    return step9("pending", "paged-reader-detach", 64);
  }
  static matchesReaderDetached(builder, reader, proof, resident) {
    return builder.#resident === resident && builder.#reader === proof && builder.#readerPhase === "detached" && OwnedUiResidentPayload.matchesReaderBinding(resident, reader, proof);
  }
  static settleReader(builder, reader, proof, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-reader-settle");
    if (!OwnedUiOperationPayloadBuilder.matchesReaderDetached(builder, reader, proof, resident) || !OwnedUiResidentReaderRetirement.matchesDetached(proof, reader, resident))
      return step9("rejected", "paged-reader-settle");
    builder.#reader = null;
    builder.#readerPhase = "settled";
    return step9("complete", "paged-reader-settle", 64);
  }
  static matchesReaderSettled(builder, reader, proof, resident) {
    return builder.#resident === resident && builder.#reader === null && builder.#readerPhase === "settled" && OwnedUiResidentPayload.matchesReaderBinding(resident, reader, proof);
  }
  static construct(field, resident, grant) {
    if (!admitted17(grant, 272) || !OwnedUiResidentPayload.matchesBuilderConstruction(resident, field))
      throw new Error("Invalid original builder reservation");
    return new OwnedUiOperationPayloadBuilder(MINT10, field, resident);
  }
  static bindSource(builder, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-source-bind");
    if (!OwnedUiResidentPayload.matchesBuilderPhase(resident, builder, "source-installing") || builder.#closing || builder.#failure !== NO_FAILURE || !builder.#field)
      return step9("rejected", "paged-source-bind");
    if (!builder.#field.bind(builder))
      return step9("rejected", "paged-source-bind", 64);
    builder.#bound = OwnedKernelReturnInputField.matchesBuilder(builder.#field, builder);
    return step9(builder.#bound ? "pending" : "rejected", "paged-source-bind", 64);
  }
  static finalize(builder, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-finalization");
    if (!OwnedUiResidentPayload.matchesBuilderPhase(resident, builder, "witness-ready") || !builder.#bound || builder.#closing || builder.#failure !== NO_FAILURE)
      return step9("rejected", "paged-finalization");
    Object.freeze(builder);
    return step9("pending", "paged-finalization", 64);
  }
  static healthy(builder) {
    return !builder.#closing && builder.#failure === NO_FAILURE && builder.#bound;
  }
  static empty(builder) {
    return builder.#empty();
  }
  static bodyEmpty(builder) {
    return builder.#failure === NO_FAILURE && builder.#closing && !builder.#fragment && !builder.#proof && !builder.#release && !builder.#input && !builder.#copyFragment && !builder.#lastFragment && !builder.#copyProof && !builder.#copyRelease && !builder.#head && !builder.#tail && !builder.#writer && !builder.#reader && builder.#readerPhase !== "held" && builder.#readerPhase !== "detached";
  }
  static sourceDetached(builder) {
    return builder.#field === null && !builder.#bound && OwnedUiOperationPayloadBuilder.bodyEmpty(builder);
  }
  static matchesRetirementOwner(builder, field, witness) {
    return builder.#resident !== null && OwnedUiResidentPayload.matchesBuilderRetirement(builder.#resident, builder, field, witness);
  }
  static detachRetirementSource(builder, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-builder-source-detach");
    if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.bodyEmpty(builder) || !OwnedUiResidentPayload.matchesBuilderRetirementPhase(resident, builder, "binding-detaching"))
      return step9("rejected", "paged-builder-source-detach");
    builder.#field = null;
    builder.#bound = false;
    return step9("pending", "paged-builder-source-detach", 64);
  }
  static finishRetirement(builder, resident, grant) {
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-builder-binding-finish");
    if (builder.#resident !== resident || !OwnedUiOperationPayloadBuilder.sourceDetached(builder) || !OwnedUiResidentPayload.matchesBuilderRetirementPhase(resident, builder, "binding-settled"))
      return step9("rejected", "paged-builder-binding-finish");
    builder.#resident = null;
    builder.#sourceKind = null;
    builder.#phase = "closed";
    return step9("complete", "paged-builder-binding-finish", 64);
  }
  static begin(owner, activation, lifetime, field, resident, grant) {
    if (!admitted17(grant, 32))
      return { step: step9("blocked", "paged-admission"), builder: null };
    if (!OwnedKernelReturnInputField.matchesOwner(field, owner, activation, lifetime) || !OwnedUiResidentPayload.matchesOwner(resident, owner, activation, lifetime))
      return { step: step9("rejected", "paged-admission-owner"), builder: null };
    return resident.beginBuilder(field, grant);
  }
  get failure() {
    return this.#failure === NO_FAILURE ? null : this.#failure;
  }
  #retainFailure(error) {
    if (this.#failure !== NO_FAILURE && !Object.is(this.#failure, error))
      throw error;
    this.#failure = error;
  }
  #copyChild(current, grant) {
    const result3 = childStep3(current, grant);
    if (result3.kind === "rejected" && this.#failure === NO_FAILURE)
      this.#retainFailure(current);
    return result3;
  }
  #settleCopy(grant) {
    if (this.#copyPhase === "proof") {
      const current = this.#resident.beginEvidence(this, grant);
      const forwarded = this.#copyChild(current.step, grant);
      if (forwarded.kind !== "rejected" && current.step.kind === "ready") {
        const fragment = this.#copyFragment;
        if (!fragment || !this.#field || !OwnedUiOperationInputCopied.matches(current.evidence, fragment, this.#field, this, fragment.offset, fragment.length))
          throw new Error("Missing original copied evidence");
        this.#copyProof = current.evidence;
        this.#copyPhase = "receipt";
      }
      return forwarded;
    }
    if (this.#copyPhase === "receipt") {
      const current = this.#resident.advanceEvidence(this, grant);
      const forwarded = this.#copyChild(current, grant);
      if (forwarded.kind !== "rejected" && current.kind === "complete")
        this.#copyPhase = "range-observe";
      return forwarded;
    }
    if (!admitted17(grant, 64))
      return step9("blocked", "paged-range-observation");
    if (!this.#field || this.#input || this.#copyFragment || this.#copyProof || this.#copyRelease || this.#sourceKind !== null || this.#field.consumed !== this.#copied || this.#field.complete !== (this.#copied === this.#field.value.byteLength))
      throw new Error("Copied range differs from original source");
    this.#inputOffset = 0;
    this.#copyPhase = this.#field.complete ? "ready" : "idle";
    return step9("pending", "paged-range-observation", 64);
  }
  advance(grant) {
    if (!admitted17(grant, 1))
      return step9("blocked", "paged-copy");
    if (this.#closing || !this.#bound || this.#failure !== NO_FAILURE || !OwnedUiResidentPayload.matchesBuilderLive(this.#resident, this))
      return step9("rejected", "paged-copy");
    try {
      if (this.#copyPhase === "ready")
        return step9("ready", "paged-copy");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe")
        return this.#settleCopy(grant);
      if (this.#copyPhase === "idle") {
        if (!admitted17(grant, 128))
          return step9("blocked", "paged-input-admit");
        const fragment = this.#field.fragment;
        if (!fragment || fragment === this.#lastFragment)
          return step9("blocked", "paged-source-continuation");
        if (!OwnedKernelReturnInputFragment.matches(fragment, this.#field) || fragment.offset !== this.#copied || fragment.offset + BigInt(fragment.length) > this.#field.value.byteLength)
          return step9("rejected", "paged-source-range");
        this.#input = fragment;
        this.#copyFragment = fragment;
        this.#inputOffset = 0;
        this.#copyPhase = "copy";
        return step9("pending", "paged-input-admit", 128);
      }
      if (this.#copyPhase === "copy" && this.#inputOffset !== this.#input.length && !this.#writer) {
        if (this.#head)
          return step9("blocked", "paged-page-window");
        this.#copyPhase = "page";
      }
      if (this.#copyPhase === "page") {
        const remaining = this.#field.value.byteLength - this.#copied;
        const length = Number(remaining < 256n ? remaining : 256n);
        const current = this.#resident.beginPage(this, length, grant);
        const result3 = this.#copyChild(current.step, grant);
        if (result3.kind !== "rejected" && current.step.kind === "ready")
          this.#copyPhase = "page-observe";
        return result3;
      }
      if (this.#copyPhase === "page-observe") {
        if (!admitted17(grant, 64))
          return step9("blocked", "paged-page-observation");
        if (!this.#writer || !OwnedUiOperationPayloadBuilder.matchesPage(this, this.#writer, this.#resident) || OwnedUiResidentPayload.pageLength(this.#resident, this, this.#writer) === null)
          return step9("rejected", "paged-page-owner");
        this.#copyPhase = "allocate";
        return step9("pending", "paged-page-observation", 64);
      }
      if (this.#copyPhase === "allocate") {
        const current = this.#writer.allocate(grant);
        const result3 = this.#copyChild(current, grant);
        if (result3.kind !== "rejected" && current.kind === "ready")
          this.#copyPhase = "allocate-observe";
        return result3;
      }
      if (this.#copyPhase === "allocate-observe") {
        if (!admitted17(grant, 64))
          return step9("blocked", "paged-allocation-observation");
        this.#copyPhase = "copy";
        return step9("pending", "paged-allocation-observation", 64);
      }
      if (this.#copyPhase === "seal") {
        const current = this.#writer.seal(grant);
        const result3 = this.#copyChild(current, grant);
        if (result3.kind !== "rejected" && current.kind === "ready")
          this.#copyPhase = "seal-observe";
        return result3;
      }
      if (this.#copyPhase === "seal-observe") {
        if (!admitted17(grant, 64))
          return step9("blocked", "paged-seal-observation");
        this.#writer = null;
        this.#copyPhase = "copy";
        return step9("pending", "paged-seal-observation", 64);
      }
      if (this.#copyPhase === "write") {
        if (typeof this.#sourceKind !== "number")
          return step9("rejected", "paged-byte-latch");
        const current = this.#writer.writeByte(this.#sourceKind, grant);
        const result3 = this.#copyChild(current, grant);
        if (result3.kind !== "rejected" && current.kind === "pending" && current.items === 1 && current.bytes === 1) {
          this.#inputOffset++;
          this.#copied++;
          this.#written++;
          this.#sourceKind = null;
          const length = OwnedUiResidentPayload.pageLength(this.#resident, this, this.#writer);
          if (length === null || this.#written > length)
            return step9("rejected", "paged-page-length", 1);
          this.#copyPhase = this.#written === length ? "seal" : "copy";
        }
        return result3;
      }
      if (this.#inputOffset === this.#input.length) {
        if (!admitted17(grant, 128))
          return step9("blocked", "paged-input-copy-detach");
        this.#input = null;
        this.#copyPhase = "proof";
        return step9("pending", "paged-input-copy-detach", 128);
      }
      this.#sourceKind = this.#input.byteAt(this.#inputOffset, this);
      this.#copyPhase = "write";
      return step9("pending", "paged-input-byte", 1);
    } catch (error) {
      this.#retainFailure(error);
      return step9("rejected", "paged-copy-fault");
    }
  }
  beginRead(grant) {
    if (!this.#resident || this.#closing || this.#failure !== NO_FAILURE)
      return { step: step9("rejected", "paged-reader-owner"), reader: null };
    return this.#resident.beginReader(this, grant);
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted17(grant, 128))
      return step9("blocked", "paged-builder-close");
    if (!this.#closing)
      throw new Error("Paged builder close has not begun");
    try {
      if (this.#phase === "closed")
        return step9("complete", "paged-builder-close");
      if (this.#failure !== NO_FAILURE)
        return step9("rejected", "paged-builder-fault-held");
      if (OwnedUiOperationPayloadBuilder.bodyEmpty(this))
        return step9("blocked", "paged-builder-binding-retirement");
      if (this.#field && OwnedKernelReturnInputField.matchesBuilder(this.#field, this))
        return step9("blocked", "paged-evidence-retirement-admission");
      if (this.#reader instanceof OwnedUiResidentPayloadReader)
        return childStep3(this.#resident.closeReader(this.#reader, grant), grant);
      if (this.#reader)
        return step9("blocked", "paged-reader-binding");
      if (this.#copyPhase === "proof" || this.#copyPhase === "receipt" || this.#copyPhase === "range-observe")
        return this.#settleCopy(grant);
      if (this.#phase === "open") {
        this.#bound = OwnedKernelReturnInputField.matchesBuilder(this.#field, this);
        if (this.#bound) {
          const fragment = this.#copyFragment ?? this.#field.fragment;
          this.#fragment = fragment === this.#lastFragment ? null : fragment;
          this.#field.beginClose();
        }
        this.#input = null;
        this.#copyFragment = null;
        this.#phase = this.#fragment ? "proof" : "pages";
        return step9("pending", "paged-input-detach", 128);
      }
      if (this.#phase === "proof")
        return step9("blocked", "paged-evidence-admission");
      if (this.#phase === "release") {
        const receipt = this.#fragment.release(this.#proof);
        if (!receipt)
          return step9("rejected", "paged-input-release-refused", 128);
        this.#release = receipt;
        this.#phase = "receipt";
        return step9("pending", "paged-input-release", 128);
      }
      if (this.#phase === "receipt") {
        if (!OwnedKernelReturnInputRelease.matches(this.#release, this.#fragment, this.#proof))
          return step9("rejected", "paged-input-release-authority");
        this.#fragment = null;
        this.#proof = null;
        this.#release = null;
        this.#bound = false;
        this.#phase = "pages";
        return step9("pending", "paged-input-release-retire", 128);
      }
      if (this.#phase === "pages" && this.#head)
        return childStep3(this.#resident.closePage(this.#head, grant), grant);
      this.#lastFragment = null;
      this.#reader = null;
      this.#sourceKind = null;
      this.#field = null;
      this.#resident = null;
      this.#bound = false;
      this.#phase = "closed";
      return step9("complete", "paged-builder-close", 128);
    } catch (error) {
      this.#retainFailure(error);
      return step9("rejected", "paged-builder-close-fault");
    }
  }
  terminalIsEmpty() {
    return this.#empty();
  }
  #empty() {
    return this.#failure === NO_FAILURE && this.#closing && this.#phase === "closed" && !this.#field && !this.#resident && !this.#fragment && !this.#proof && !this.#release && !this.#input && !this.#copyFragment && !this.#lastFragment && !this.#copyProof && !this.#copyRelease && !this.#head && !this.#tail && !this.#writer && !this.#reader && this.#readerPhase !== "held" && this.#readerPhase !== "detached";
  }
}

class OwnedUiOperationInputCopied {
  #proof;
  constructor(mint, builder, resident, fragment) {
    if (mint !== MINT10)
      throw new Error("Invalid copied input authority");
    this.#proof = builder;
    const installed = resident.installEvidence(this, builder, { maxItems: 1, maxBytes: 64 });
    if (installed.kind !== "ready")
      throw new Error("Evidence installation refused");
    this.#proof = { fragment, field: fragment.field, builder, offset: fragment.offset, length: fragment.length, release: null, phase: "constructed", token: this };
    Object.seal(this.#proof);
  }
  static {
    copiedProof = (builder, resident, fragment) => new OwnedUiOperationInputCopied(MINT10, builder, resident, fragment);
    copiedState = (token) => token !== null && typeof token === "object" && (#proof in token) ? token.#proof : null;
  }
  static matches(token, fragment, field, builder, offset, length) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && exact2(token.#proof, fragment, field, builder, offset, length);
  }
  static matchesRelease(token, release2) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release2);
  }
  static matchesSourceDetached(token, release2) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release2) && token.#proof.phase === "source-detached" && !token.#proof.fragment && !token.#proof.field && !token.#proof.builder;
  }
}

class OwnedUiOperationInputCancelled {
  #proof;
  constructor(mint, builder, resident, fragment) {
    if (mint !== MINT10)
      throw new Error("Invalid cancelled input authority");
    this.#proof = builder;
    const installed = resident.installEvidence(this, builder, { maxItems: 1, maxBytes: 64 });
    if (installed.kind !== "ready")
      throw new Error("Evidence installation refused");
    this.#proof = { fragment, field: fragment.field, builder, offset: fragment.offset, length: fragment.length, release: null, phase: "constructed", token: this };
    Object.seal(this.#proof);
  }
  static {
    cancelledProof = (builder, resident, fragment) => new OwnedUiOperationInputCancelled(MINT10, builder, resident, fragment);
    cancelledState = (token) => token !== null && typeof token === "object" && (#proof in token) ? token.#proof : null;
  }
  static matches(token, fragment, field, builder, offset, length) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && exact2(token.#proof, fragment, field, builder, offset, length);
  }
  static matchesRelease(token, release2) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release2);
  }
  static matchesSourceDetached(token, release2) {
    return token !== null && typeof token === "object" && #proof in token && !builderBrand(token.#proof) && evidenceRelease(token.#proof, token, release2) && token.#proof.phase === "source-detached" && !token.#proof.fragment && !token.#proof.field && !token.#proof.builder;
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️.ts */
var ENVELOPES = Object.freeze({
  pool: Object.freeze({ bytes: 264, slots: 5, owners: 5 }),
  instance: Object.freeze({ bytes: 376, slots: 6, owners: 6 }),
  payload: Object.freeze({ bytes: 312, slots: 4, owners: 4 }),
  builder: Object.freeze({ bytes: 296, slots: 3, owners: 3 }),
  reader: Object.freeze({ bytes: 160, slots: 3, owners: 3 }),
  page: Object.freeze({ bytes: 160, slots: 3, owners: 3 }),
  evidence: Object.freeze({ bytes: 192, slots: 4, owners: 4 })
});
function uiResidentMetadataEnvelope(kind) {
  if (typeof kind !== "string" || !Object.hasOwn(ENVELOPES, kind))
    throw new Error("Invalid UI resident metadata kind");
  return ENVELOPES[kind];
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts */
var readerState;
var createReader2;
var createReaderWitness;
var moveReaderWitness;
var MINT11 = Object.freeze({});
var NO_POOL_FAULT = Object.freeze({});
var admitted18 = (grant, bytes) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= bytes;
var step10 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var instanceOwner;
var payloadOwner;
var pageOwner;
var pageState2;
var createPageWitness;
var markPageWitness;
var createBuilderWitness;
var markBuilderWitness;
var moveBuilderWitness;
var createEvidenceWitness;
var markEvidenceWitness;
var poolWitness;
var finishPoolWitness;
var createInstanceWitness;
var markInstanceDomainClosed;
var instanceDomainClosed;
var closeInstance;
var scopeAvailable;
var payloadState;
var payloadWitness;
var movePayloadWitness;
var payloadWitnessOriginal;
var closePayload;
function active(instance) {
  if (instance.phase !== "live" || instance.failure !== NO_POOL_FAULT || instance.cell?.hasFailure || !instance.pool || instance.pool.closing || instance.closing || instance.closed || !instance.owner || !instance.activation || !instance.lifetime || !OwnedUiInstance.matches(instance.owner, instance.activation, instance.lifetime))
    return false;
  try {
    instance.activation.assertActive();
    return true;
  } catch {
    return false;
  }
}
function activePayload(payload) {
  return payload.phase === "live" && payload.failure === NO_POOL_FAULT && !payload.cell?.hasFailure && !payload.closing && !payload.closed && payload.instance !== null && active(payload.instance);
}
function payloadSlotEmpty(slot) {
  return slot.phase === "empty" && slot.requestOwner === null && !slot.cell && !slot.record && !slot.entry && !slot.witness && slot.failure === NO_POOL_FAULT;
}
function evidenceSlot(slot) {
  return OwnedUiOperationPayloadBuilder.hasBrand(slot.requestOwner);
}
function payloadBodyEmpty(state7) {
  return !state7.head && !state7.tail && !state7.cursor && !state7.builder && !state7.storageCell && !state7.reader && !state7.evidence && (!state7.pending || payloadSlotEmpty(state7.pending)) && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure;
}
function childStep4(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function admissionStep(current, grant) {
  const result3 = childStep4(current, grant);
  return result3.kind === "ready" ? { ...result3, kind: "pending" } : result3;
}
function clearSlot(slot) {
  slot.requestOwner = null;
  slot.cell = null;
  slot.record = null;
  slot.entry = null;
  slot.witness = null;
  slot.phase = "empty";
}
function slotFault(slot, error) {
  if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error))
    throw error;
  slot.failure = error;
}
function observeSlot(pool, slot) {
  if (slot.phase === "bootstrap-rejected") {
    const cell = pool.ledger.preparedAdmission(slot);
    if (cell) {
      slot.cell = cell;
      slot.phase = "cell-held";
    } else
      clearSlot(slot);
    return step10("pending", "resident-scope-rejection-observation", 64);
  }
  if (slot.phase === "preparing") {
    const cell = pool.ledger.preparedAdmission(slot);
    if (!cell)
      return step10("blocked", "resident-scope-cell-handoff");
    slot.cell = cell;
    slot.phase = "cell-held";
    return step10("pending", "resident-scope-cell-observation", 64);
  }
  if (slot.phase === "claiming") {
    if (!slot.cell?.claimed)
      return step10("rejected", "resident-scope-claim");
    slot.phase = "claimed";
    return step10("pending", "resident-scope-claim-observation", 64);
  }
  if (slot.phase === "record-admitting") {
    slot.record = slot.cell.result?.record ?? null;
    slot.phase = slot.record && slot.cell.result?.step.kind === "ready" && !slot.cell.hasFailure ? "record-held" : "fault-held";
    return step10(slot.phase === "record-held" ? "pending" : "rejected", "resident-scope-record-observation", 64);
  }
  return null;
}
function closeSlot(pool, slot, grant) {
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-scope-slot-close");
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) {
      const cell = pool?.ledger?.preparedAdmission(slot);
      if (!cell)
        return step10("rejected", "resident-scope-fault-held");
      slot.cell = cell;
      return step10("pending", "resident-scope-fault-cell-observation", 64);
    }
    if (!slot.cell.hasFailure)
      return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
    if (!Object.is(slot.cell.failure, slot.failure))
      return step10("rejected", "resident-scope-distinct-fault");
    if (!slot.record && slot.cell.result?.record) {
      slot.record = slot.cell.result.record;
      return step10("pending", "resident-scope-fault-record-observation", 64);
    }
    if (!slot.entry && slot.phase !== "record-closing" && slot.phase !== "record-observing" && slot.phase !== "cell-closing" && slot.phase !== "cell-observing") {
      if (slot.record) {
        slot.record.beginClose();
        slot.phase = "record-closing";
      } else {
        slot.cell.beginClose();
        slot.phase = "cell-closing";
      }
      return step10("pending", "resident-scope-fault-intrinsic-begin", 64);
    }
  } else if (pool) {
    const observed = observeSlot(pool, slot);
    if (observed)
      return observed;
  }
  const entry = slot.entry;
  if (entry && entry.phase !== "domain-closed" && entry.phase !== "retired")
    return closeInstance(entry, grant);
  if (slot.phase === "handoff-observing") {
    if (!entry || !slot.witness || !instanceDomainClosed(slot.witness, entry))
      return step10("rejected", "resident-scope-domain-proof");
    slot.record.beginClose();
    slot.phase = slot.record.matchesShell(slot.witness.scope) ? "detaching" : "record-closing";
    return step10("pending", "resident-scope-record-begin", 64);
  }
  if (slot.phase === "detaching") {
    const record = slot.record;
    if (OwnedResidentRecordDetachment.matches(record.detachment, record, slot.witness.scope)) {
      slot.phase = "record-closing";
      return step10("pending", "resident-scope-detach-observation", 64);
    }
    return childStep4(record.detach(slot.witness.scope, grant), grant);
  }
  if (slot.phase === "record-closing") {
    const current = slot.record.closeStep(grant);
    const result3 = childStep4(current, grant);
    if (current.kind === "complete" && result3.kind === "pending")
      slot.phase = "record-observing";
    return result3;
  }
  if (slot.phase === "record-observing") {
    if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
      return step10("rejected", "resident-scope-record-proof");
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-scope-cell-begin", 64);
  }
  if (slot.phase === "cell-closing") {
    const current = slot.cell.closeStep(grant);
    const result3 = childStep4(current, grant);
    if (current.kind === "complete" && result3.kind === "pending")
      slot.phase = "cell-observing";
    return result3;
  }
  if (slot.phase === "cell-observing") {
    if (slot.failure !== NO_POOL_FAULT || !slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
      return step10("rejected", "resident-scope-cell-proof");
    if (entry) {
      entry.slot = null;
      entry.witness = null;
      entry.phase = "retired";
      entry.closed = true;
    }
    clearSlot(slot);
    return step10("complete", "resident-scope-slot-close", 64);
  }
  if (entry)
    return step10("rejected", "resident-scope-slot-phase");
  if (slot.record) {
    slot.record.beginClose();
    slot.phase = "record-closing";
    return step10("pending", "resident-scope-unused-record", 64);
  }
  if (slot.cell) {
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-scope-unused-cell", 64);
  }
  if (slot.failure !== NO_POOL_FAULT)
    return step10("rejected", "resident-scope-fault-held");
  clearSlot(slot);
  return step10("complete", "resident-scope-slot-close", 64);
}

class OwnedUiResidentPool {
  #state;
  constructor(mint, client, ledger, grant) {
    if (mint !== MINT11)
      throw new Error("Invalid resident pool authority");
    const state7 = { ledger, composition: client, failure: NO_POOL_FAULT, phase: "open", bindings: null, head: null, tail: null, pending: null, closing: false, closed: false, facade: this, witness: null };
    this.#state = state7;
    try {
      const current = client.installUiResidentPool(this, grant);
      if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64 || !client.ownsUiResidentPool(this))
        throw new Error("Resident pool installation refused");
      state7.pending = { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null };
      state7.bindings = new WeakMap;
      poolWitness(state7, this);
      Object.freeze(this);
    } catch (error) {
      state7.failure = error;
      state7.closing = true;
      state7.phase = "closing";
      throw error;
    }
  }
  static begin(client, ledger, grant) {
    if (!ShardClient.matchesResidentLedger(client, ledger))
      return { step: step10("rejected", "resident-pool-composition"), pool: null };
    if (!admitted18(grant, 1))
      return { step: step10("blocked", "resident-pool-admission"), pool: null };
    const prepared = client.prepareUiResidentPool(ledger, grant);
    const current = childStep4(prepared, grant);
    if (current.kind !== "ready" || current.items !== 0 || current.bytes !== 0)
      return { step: current.kind === "ready" ? { ...current, kind: "pending" } : current, pool: null };
    const bytes = uiResidentMetadataEnvelope("pool").bytes + 64;
    if (!admitted18(grant, bytes))
      return { step: step10("blocked", "resident-pool-construction"), pool: null };
    try {
      return { step: step10("ready", "resident-pool-construction", bytes), pool: new OwnedUiResidentPool(MINT11, client, ledger, { maxItems: 1, maxBytes: 64 }) };
    } catch (error) {
      client.captureUiResidentPoolFault(error);
      return { step: step10("rejected", "resident-pool-construction", bytes), pool: null };
    }
  }
  static matchesComposition(pool, client, ledger) {
    return pool !== null && typeof pool === "object" && #state in pool && pool.#state.composition === client && pool.#state.ledger === ledger && ShardClient.matchesResidentLedger(client, ledger);
  }
  get usage() {
    const state7 = this.#state;
    if (!state7.ledger)
      throw new Error("Resident pool is retired");
    return state7.ledger.usage.data;
  }
  bindInstance(owner, activation, lifetime, grant) {
    const result3 = (current) => ({ step: current, scope: null });
    if (!admitted18(grant, 64))
      return result3(step10("blocked", "resident-scope-admission"));
    const pool = this.#state;
    const slot = pool.pending;
    if (pool.closing || !pool.bindings || !slot || !ShardClient.matchesActivation(pool.composition, activation) || !OwnedUiInstance.matches(owner, activation, lifetime))
      return result3(step10("rejected", "resident-scope-authority"));
    try {
      activation.assertActive();
      const previous = pool.bindings.get(owner);
      if (previous) {
        const usable = scopeAvailable(previous);
        return { step: step10(usable ? "ready" : "rejected", "resident-scope-held"), scope: usable ? previous : null };
      }
      if (slot.requestOwner && slot.requestOwner !== owner)
        return result3(step10("blocked", "resident-scope-slot-busy"));
      if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure)
        return result3(step10("rejected", "resident-scope-fault-held"));
      const observed = observeSlot(pool, slot);
      if (observed)
        return result3(observed);
      if (slot.phase === "empty") {
        if (!admitted18(grant, 296))
          return result3(step10("blocked", "resident-scope-bootstrap"));
        slot.requestOwner = owner;
        slot.phase = "preparing";
        const current = pool.ledger.prepareAdmission(slot, "data", grant);
        if (current.kind === "blocked")
          clearSlot(slot);
        else if (current.kind === "rejected")
          slot.phase = "bootstrap-rejected";
        return result3(admissionStep(current, grant));
      }
      if (slot.phase === "cell-held") {
        slot.phase = "claiming";
        const current = pool.ledger.claimAdmission(slot, slot.cell, grant);
        if (current.kind === "blocked")
          slot.phase = "cell-held";
        return result3(admissionStep(current, grant));
      }
      if (slot.phase === "claimed") {
        if (!admitted18(grant, 264))
          return result3(step10("blocked", "resident-scope-record"));
        slot.phase = "record-admitting";
        const current = pool.ledger.reserveRecord("data", uiResidentMetadataEnvelope("instance"), slot.cell, grant);
        if (current.step.kind === "blocked")
          slot.phase = "claimed";
        return result3(admissionStep(current.step, grant));
      }
      if (slot.phase === "record-held") {
        const bytes = uiResidentMetadataEnvelope("instance").bytes + 64;
        if (!admitted18(grant, bytes))
          return result3(step10("blocked", "resident-scope-construction"));
        const state7 = { pool, owner, facade: null, activation, lifetime: null, previous: pool.tail, next: null, head: null, tail: null, children: 0, closing: false, closed: false, record: slot.record, cell: slot.cell, phase: "constructing", witness: null, failure: NO_POOL_FAULT, slot, bindingInstalled: false, pending: null };
        slot.entry = state7;
        instanceOwner(state7);
        state7.lifetime = Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime });
        slot.phase = "shell-installed";
        return result3(step10("pending", "resident-scope-construction", bytes));
      }
      if (slot.phase === "shell-installed") {
        const state7 = slot.entry;
        if (!owner.attachResidentScope(state7.facade))
          return result3(step10("rejected", "resident-scope-owner-refused"));
        slot.phase = "roster-observing";
        return result3(step10("pending", "resident-scope-owner-attachment", 64));
      }
      if (slot.phase === "roster-observing") {
        if (!admitted18(grant, 128))
          return result3(step10("blocked", "resident-scope-publish"));
        const state7 = slot.entry;
        const scope = state7.facade;
        pool.bindings.set(owner, scope);
        state7.bindingInstalled = true;
        state7.slot = null;
        state7.phase = "live";
        clearSlot(slot);
        return { step: step10("ready", "resident-scope-admission", 128), scope };
      }
      return result3(step10("rejected", "resident-scope-phase"));
    } catch (error) {
      slotFault(slot, error);
      if (slot.entry) {
        slot.entry.failure = error;
        slot.entry.closing = true;
      }
      return result3(step10("rejected", "resident-scope-fault"));
    }
  }
  beginClose() {
    const state7 = this.#state;
    state7.closing = true;
    if (state7.phase === "open")
      state7.phase = "closing";
  }
  closeStep(grant) {
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-pool-close");
    const state7 = this.#state;
    if (!state7.closing)
      throw new Error("Resident pool close has not begun");
    if (state7.closed)
      return step10("complete", "resident-pool-close");
    if (!state7.witness) {
      try {
        poolWitness(state7, this);
        return step10("pending", "resident-pool-witness", 64);
      } catch (error) {
        if (state7.failure !== NO_POOL_FAULT && !Object.is(state7.failure, error))
          throw error;
        state7.failure = error;
        state7.composition.captureUiResidentPoolFault(error);
        return step10("rejected", "resident-pool-witness");
      }
    }
    if (state7.pending && state7.pending.phase !== "empty") {
      try {
        return childStep4(closeSlot(state7, state7.pending, grant), grant);
      } catch (error) {
        slotFault(state7.pending, error);
        return step10("rejected", "resident-scope-slot-fault");
      }
    }
    if (state7.head)
      return childStep4(closeInstance(state7.head, grant), grant);
    if (state7.tail)
      return step10("blocked", "resident-pool-children");
    if (state7.failure !== NO_POOL_FAULT)
      return step10("rejected", "resident-pool-unretired-fault");
    state7.pending = null;
    state7.bindings = null;
    state7.ledger = null;
    state7.composition = null;
    state7.facade = null;
    state7.phase = "closed";
    state7.closed = true;
    finishPoolWitness(state7.witness, this);
    return step10("complete", "resident-pool-close", 64);
  }
  terminalIsEmpty() {
    const state7 = this.#state;
    return state7.closed && !state7.bindings && !state7.head && !state7.tail && !state7.pending && !state7.ledger && !state7.composition && !state7.facade;
  }
  get retirement() {
    return this.#state.closed ? this.#state.witness : null;
  }
}

class OwnedUiResidentPoolRetirement {
  #pool;
  #terminal = false;
  constructor(mint, state7, pool) {
    if (mint !== MINT11)
      throw new Error("Invalid pool retirement authority");
    this.#pool = pool;
    state7.witness = this;
    Object.freeze(this);
  }
  static {
    poolWitness = (state7, pool) => new OwnedUiResidentPoolRetirement(MINT11, state7, pool);
    finishPoolWitness = (witness, pool) => {
      if (witness.#pool !== pool)
        throw new Error("Pool retirement identity differs");
      witness.#terminal = true;
    };
  }
  static matches(witness, pool, client, ledger) {
    return witness !== null && typeof witness === "object" && #pool in witness && witness.#pool === pool && witness.#terminal && ShardClient.matchesResidentLedger(client, ledger) && Reflect.apply(ShardClient.prototype.ownsUiResidentPool, client, [pool]);
  }
}

class InstanceWitness {
  #scope;
  #terminal = false;
  constructor(mint, state7) {
    if (mint !== MINT11 || !state7.facade)
      throw new Error("Invalid resident instance witness");
    this.#scope = state7.facade;
    state7.witness = this;
    Object.freeze(this);
  }
  static {
    createInstanceWitness = (state7) => new InstanceWitness(MINT11, state7);
    markInstanceDomainClosed = (witness, state7) => {
      if (state7.witness !== witness || state7.phase !== "domain-closed")
        throw new Error("Invalid resident domain transition");
      witness.#terminal = true;
    };
    instanceDomainClosed = (witness, state7) => state7.witness === witness && witness.#terminal && state7.phase === "domain-closed" && !state7.pool && !state7.owner && !state7.activation && !state7.lifetime && !state7.head && !state7.tail && !state7.previous && !state7.next && !state7.record && !state7.cell && !state7.pending && state7.children === 0 && state7.failure === NO_POOL_FAULT;
  }
  get scope() {
    return this.#scope;
  }
}

class OwnedUiResidentInstance {
  #state;
  constructor(mint, state7) {
    if (mint !== MINT11)
      throw new Error("Invalid resident instance authority");
    this.#state = state7;
    state7.facade = this;
    const pool = state7.pool;
    if (pool.tail)
      pool.tail.next = state7;
    else
      pool.head = state7;
    pool.tail = state7;
    const installed = state7.record.install(this, { maxItems: 1, maxBytes: 64 });
    if (installed.kind !== "ready")
      throw new Error("Resident instance record refused installation");
    state7.pending = newPayloadSlot();
    createInstanceWitness(state7);
    Object.freeze(this);
  }
  static {
    instanceOwner = (state7) => new OwnedUiResidentInstance(MINT11, state7);
    scopeAvailable = (scope) => active(scope.#state);
    closeInstance = (state7, grant) => {
      state7.closing = true;
      return state7.facade ? state7.facade.#close(grant) : closeSlot(null, state7.slot, grant);
    };
  }
  static matches(scope, owner, activation, lifetime) {
    if (scope === null || typeof scope !== "object" || !(#state in scope))
      return false;
    const state7 = scope.#state;
    return !state7.closed && state7.owner === owner && state7.activation === activation && state7.lifetime !== null && state7.lifetime.activationGeneration === lifetime.activationGeneration && state7.lifetime.instanceId === lifetime.instanceId && state7.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  beginPayload(field, grant) {
    return admitPayload(this.#state, field, grant);
  }
  beginClose() {
    this.#state.closing = true;
  }
  closeStep(grant) {
    return this.#close(grant);
  }
  #close(grant) {
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-instance-close");
    const state7 = this.#state;
    if (!state7.closing)
      throw new Error("Resident instance close has not begun");
    if (state7.closed)
      return step10("complete", "resident-instance-close");
    if (state7.phase === "domain-closed") {
      try {
        return closeSlot(null, state7.slot, grant);
      } catch (error) {
        slotFault(state7.slot, error);
        return step10("rejected", "resident-scope-slot-fault");
      }
    }
    if (state7.pending && state7.pending.phase !== "empty") {
      try {
        return childStep4(closePayloadSlot(state7, state7.pending, grant), grant);
      } catch (error) {
        payloadSlotFault(state7.pending, error);
        return step10("rejected", "resident-payload-slot-fault");
      }
    }
    if (state7.head)
      return childStep4(closePayload(state7.head, grant), grant);
    if (state7.children || state7.tail)
      return step10("blocked", "resident-instance-children");
    const pool = state7.pool;
    const slot = pool.pending;
    if (slot.phase !== "empty" && slot.entry !== state7)
      return step10("blocked", "resident-scope-slot-busy");
    if (!state7.slot) {
      slot.requestOwner = state7.owner;
      slot.cell = state7.cell;
      slot.record = state7.record;
      slot.entry = state7;
      slot.witness = state7.witness;
      slot.phase = "closing-domain";
      state7.slot = slot;
      return step10("pending", "resident-scope-retirement-capture", 64);
    }
    if (state7.failure !== NO_POOL_FAULT) {
      slotFault(slot, state7.failure);
      if (!slot.cell.hasFailure)
        return childStep4(slot.cell.retainFailure(state7.failure, grant), grant);
      if (!Object.is(slot.cell.failure, state7.failure))
        return step10("rejected", "resident-scope-distinct-fault");
    }
    if (!state7.witness) {
      try {
        createInstanceWitness(state7);
        slot.witness = state7.witness;
        return step10("pending", "resident-scope-domain-witness", 64);
      } catch (error) {
        slotFault(slot, error);
        return step10("rejected", "resident-scope-domain-witness");
      }
    }
    if (!admitted18(grant, 128))
      return step10("blocked", "resident-scope-domain-unlink");
    slot.witness = state7.witness;
    if (pool.bindings.get(state7.owner) === state7.facade)
      pool.bindings.delete(state7.owner);
    if (state7.previous)
      state7.previous.next = state7.next;
    else
      pool.head = state7.next;
    if (state7.next)
      state7.next.previous = state7.previous;
    else
      pool.tail = state7.previous;
    state7.record = null;
    state7.cell = null;
    state7.previous = null;
    state7.next = null;
    state7.facade = null;
    state7.pool = null;
    state7.owner = null;
    state7.activation = null;
    state7.lifetime = null;
    state7.failure = NO_POOL_FAULT;
    state7.bindingInstalled = false;
    state7.pending = null;
    state7.phase = "domain-closed";
    slot.requestOwner = null;
    slot.phase = "handoff-observing";
    markInstanceDomainClosed(state7.witness, state7);
    return step10("pending", "resident-scope-domain-unlink", 128);
  }
  terminalIsEmpty() {
    const state7 = this.#state;
    return state7.closed && !state7.pool && !state7.owner && !state7.facade && !state7.activation && !state7.lifetime && !state7.head && !state7.tail && !state7.previous && !state7.next && !state7.record && !state7.cell && !state7.slot && !state7.witness && !state7.pending && state7.failure === NO_POOL_FAULT && state7.children === 0;
  }
}
function newPayloadSlot() {
  return { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null };
}
function clearPayloadSlot(slot) {
  slot.requestOwner = null;
  slot.cell = null;
  slot.record = null;
  slot.entry = null;
  slot.witness = null;
  slot.phase = "empty";
}
function payloadSlotFault(slot, error) {
  if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error))
    throw error;
  slot.failure = error;
  if (slot.entry) {
    slot.entry.failure = error;
    slot.entry.closing = true;
  }
}
function observePayloadSlot(instance, slot) {
  if (slot.phase === "bootstrap-rejected") {
    const cell = instance.pool.ledger.preparedAdmission(slot);
    if (cell) {
      slot.cell = cell;
      slot.phase = "cell-held";
    } else
      clearPayloadSlot(slot);
    return step10("pending", "resident-payload-rejection-observation", 64);
  }
  if (slot.phase === "preparing") {
    const cell = instance.pool.ledger.preparedAdmission(slot);
    if (!cell)
      return step10("blocked", "resident-payload-cell-handoff");
    slot.cell = cell;
    slot.phase = "cell-held";
    return step10("pending", "resident-payload-cell-observation", 64);
  }
  if (slot.phase === "claiming") {
    if (!slot.cell?.claimed)
      return step10("rejected", "resident-payload-claim");
    slot.phase = "claimed";
    return step10("pending", "resident-payload-claim-observation", 64);
  }
  if (slot.phase === "record-admitting") {
    slot.record = slot.cell.result?.record ?? null;
    slot.phase = slot.record && slot.cell.result?.step.kind === "ready" && !slot.cell.hasFailure ? "record-held" : "fault-held";
    return step10(slot.phase === "record-held" ? "pending" : "rejected", "resident-payload-record-observation", 64);
  }
  return null;
}
function admitPayload(instance, field, grant) {
  const result3 = (current) => ({ step: current, payload: null });
  if (!admitted18(grant, 64))
    return result3(step10("blocked", "resident-payload-admission"));
  const slot = instance.pending;
  if (!active(instance) || !slot || !OwnedKernelReturnInputField.matchesOwner(field, instance.owner, instance.activation, instance.lifetime))
    return result3(step10("rejected", "resident-payload-authority"));
  try {
    if (OwnedKernelReturnPayloadDetachment.matchesOwner(field.residentPayloadDetachment, field))
      return result3(step10("rejected", "resident-payload-source-retired"));
    const previous = field.residentPayload(instance.facade);
    if (previous) {
      const state7 = payloadState(previous);
      if (state7 && activePayload(state7))
        return { step: step10("ready", "resident-payload-held"), payload: previous };
      if (slot.entry !== state7)
        return result3(step10("rejected", "resident-payload-held"));
    }
    if (slot.requestOwner && slot.requestOwner !== field)
      return result3(step10("blocked", "resident-payload-slot-busy"));
    if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure)
      return result3(step10("rejected", "resident-payload-fault-held"));
    const observed = observePayloadSlot(instance, slot);
    if (observed)
      return result3(observed);
    const ledger = instance.pool.ledger;
    if (slot.phase === "empty") {
      if (!admitted18(grant, 296))
        return result3(step10("blocked", "resident-payload-bootstrap"));
      slot.requestOwner = field;
      slot.phase = "preparing";
      const current = ledger.prepareAdmission(slot, "data", grant);
      if (current.kind === "blocked")
        clearPayloadSlot(slot);
      else if (current.kind === "rejected")
        slot.phase = "bootstrap-rejected";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "cell-held") {
      slot.phase = "claiming";
      const current = ledger.claimAdmission(slot, slot.cell, grant);
      if (current.kind === "blocked")
        slot.phase = "cell-held";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "claimed") {
      if (!admitted18(grant, 264))
        return result3(step10("blocked", "resident-payload-record"));
      slot.phase = "record-admitting";
      const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("payload"), slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "record-held") {
      if (!admitted18(grant, 272))
        return result3(step10("blocked", "resident-payload-shell"));
      const state7 = { instance, facade: null, previous: instance.tail, next: null, head: null, tail: null, cursor: null, builder: null, storageCell: null, reader: null, evidence: null, closing: false, closed: false, field, record: slot.record, cell: slot.cell, phase: "constructing", witness: null, parentSlot: slot, failure: NO_POOL_FAULT, pending: null };
      slot.entry = state7;
      slot.phase = "shell-installed";
      payloadOwner(state7);
      return result3(step10("pending", "resident-payload-shell", 272));
    }
    if (slot.phase === "shell-installed") {
      slot.phase = "source-installing";
      const current = field.installResidentPayload(slot.entry.facade, grant);
      if (current.kind === "blocked")
        slot.phase = "shell-installed";
      return result3(admissionStep({ ...current, phase: "resident-payload-source-install" }, grant));
    }
    if (slot.phase === "source-installing") {
      if (!OwnedKernelReturnInputField.matchesResidentPayload(field, slot.entry.facade))
        return result3(step10("rejected", "resident-payload-source-observation"));
      slot.phase = "source-bound";
      return result3(step10("pending", "resident-payload-source-observation", 64));
    }
    if (slot.phase === "source-bound") {
      if (!admitted18(grant, 104))
        return result3(step10("blocked", "resident-payload-finalization"));
      const state7 = slot.entry;
      state7.pending = newBuilderSlot();
      payloadWitness(state7);
      Object.freeze(state7.facade);
      slot.phase = "finalized";
      return result3(step10("pending", "resident-payload-finalization", 104));
    }
    if (slot.phase === "finalized") {
      const state7 = slot.entry;
      state7.phase = "live";
      state7.parentSlot = null;
      const payload = state7.facade;
      clearPayloadSlot(slot);
      return { step: step10("ready", "resident-payload-publication", 64), payload };
    }
    return result3(step10("rejected", "resident-payload-phase"));
  } catch (error) {
    payloadSlotFault(slot, error);
    return result3(step10("rejected", "resident-payload-fault"));
  }
}
function closePayloadSlot(instance, slot, grant) {
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-payload-slot-close");
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) {
      const cell = instance?.pool?.ledger?.preparedAdmission(slot);
      if (!cell)
        return step10("rejected", "resident-payload-fault-held");
      slot.cell = cell;
      return step10("pending", "resident-payload-fault-cell-observation", 64);
    }
    if (!slot.cell.hasFailure)
      return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
    if (!Object.is(slot.cell.failure, slot.failure))
      return step10("rejected", "resident-payload-distinct-fault");
    if (!slot.record && slot.cell.result?.record) {
      slot.record = slot.cell.result.record;
      return step10("pending", "resident-payload-fault-record-observation", 64);
    }
    if (slot.entry)
      return step10("rejected", "resident-payload-fault-held");
  } else if (instance) {
    const observed = observePayloadSlot(instance, slot);
    if (observed)
      return observed;
  }
  const entry = slot.entry;
  if (entry && entry.phase !== "domain-retired" && entry.phase !== "registration-retired")
    return closePayload(entry, grant);
  if (slot.phase === "handoff-observing") {
    if (!entry || !entry.witness || slot.witness !== entry.witness || !payloadDomainEmpty(entry))
      return step10("rejected", "resident-payload-domain-proof");
    slot.record.beginClose();
    slot.phase = slot.record.matchesShell(payloadWitnessOriginal(entry.witness)) ? "detaching" : "record-closing";
    return step10("pending", "resident-payload-record-begin", 64);
  }
  if (slot.phase === "detaching") {
    const original = payloadWitnessOriginal(entry.witness);
    if (OwnedResidentRecordDetachment.matches(slot.record.detachment, slot.record, original)) {
      slot.phase = "record-closing";
      return step10("pending", "resident-payload-detach-observation", 64);
    }
    return childStep4(slot.record.detach(original, grant), grant);
  }
  if (slot.phase === "record-closing") {
    const current = slot.record.closeStep(grant);
    const forwarded = childStep4(current, grant);
    if (current.kind === "complete" && forwarded.kind === "pending")
      slot.phase = "record-observing";
    return forwarded;
  }
  if (slot.phase === "record-observing") {
    if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
      return step10("rejected", "resident-payload-record-proof");
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-payload-cell-begin", 64);
  }
  if (slot.phase === "cell-closing") {
    const current = slot.cell.closeStep(grant);
    const forwarded = childStep4(current, grant);
    if (current.kind === "complete" && forwarded.kind === "pending")
      slot.phase = "cell-observing";
    return forwarded;
  }
  if (slot.phase === "cell-observing") {
    if (slot.failure !== NO_POOL_FAULT || !slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
      return step10("rejected", "resident-payload-cell-proof");
    if (entry) {
      movePayloadWitness(entry, "registration-retired");
      entry.parentSlot = null;
      entry.witness = null;
      entry.closed = true;
    }
    clearPayloadSlot(slot);
    return step10("complete", "resident-payload-slot-close", 64);
  }
  if (entry)
    return step10("rejected", "resident-payload-slot-phase");
  if (slot.record) {
    slot.record.beginClose();
    slot.phase = "record-closing";
    return step10("pending", "resident-payload-unused-record", 64);
  }
  if (slot.cell) {
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-payload-unused-cell", 64);
  }
  if (slot.failure !== NO_POOL_FAULT)
    return step10("rejected", "resident-payload-fault-held");
  clearPayloadSlot(slot);
  return step10("complete", "resident-payload-slot-close", 64);
}
function payloadDomainEmpty(state7) {
  return state7.phase === "domain-retired" && payloadBodyEmpty(state7) && !state7.field && !state7.instance && !state7.facade && !state7.previous && !state7.next && !state7.record && !state7.cell && !state7.pending;
}
function newBuilderSlot() {
  return { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null };
}

class BuilderWitness {
  #builder;
  #phase = "constructed";
  constructor(mint, entry) {
    if (mint !== MINT11 || !entry.facade)
      throw new Error("Invalid builder witness");
    this.#builder = entry.facade;
    entry.witness = this;
    Object.freeze(this);
  }
  static {
    createBuilderWitness = (entry) => new BuilderWitness(MINT11, entry);
    markBuilderWitness = (witness, builder) => {
      if (witness.#builder !== builder || !OwnedUiOperationPayloadBuilder.empty(builder))
        throw new Error("Invalid builder terminal proof");
      witness.#phase = "terminal";
    };
    moveBuilderWitness = (witness, builder, phase) => {
      if (witness.#builder !== builder || !OwnedUiOperationPayloadBuilder.bodyEmpty(builder) || phase === "body-retired" && witness.#phase !== "constructed" || phase === "source-detached" && (witness.#phase !== "body-retired" || !OwnedUiOperationPayloadBuilder.sourceDetached(builder)) || phase === "source-settled" && witness.#phase !== "source-detached")
        throw new Error("Invalid builder binding phase");
      witness.#phase = phase;
    };
  }
  static matchesBody(proof, builder, field) {
    return proof !== null && typeof proof === "object" && #phase in proof && proof.#phase === "body-retired" && proof.#builder === builder && OwnedUiOperationPayloadBuilder.bodyEmpty(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof);
  }
  static matchesDetached(proof, field) {
    return proof !== null && typeof proof === "object" && #phase in proof && proof.#phase === "source-detached" && OwnedUiOperationPayloadBuilder.sourceDetached(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof);
  }
  static matchesSourceBinding(proof, field) {
    return proof !== null && typeof proof === "object" && #phase in proof && (proof.#phase === "source-detached" || proof.#phase === "source-settled") && OwnedUiOperationPayloadBuilder.sourceDetached(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof);
  }
  get builder() {
    return this.#builder;
  }
  get terminal() {
    return this.#phase === "terminal" && OwnedUiOperationPayloadBuilder.empty(this.#builder);
  }
}
function clearBuilderSlot(slot) {
  slot.requestOwner = null;
  slot.cell = null;
  slot.record = null;
  slot.entry = null;
  slot.witness = null;
  slot.phase = "empty";
}
function builderFault(slot, error) {
  if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error))
    throw error;
  slot.failure = error;
}
function observeBuilderSlot(state7, slot, grant) {
  if ((slot.phase === "bootstrap-rejected" || slot.phase === "preparing" || slot.phase === "claiming" || slot.phase === "record-admitting") && !admitted18(grant, 64))
    return step10("blocked", "resident-builder-observation");
  const ledger = state7.instance.pool.ledger;
  if (slot.phase === "bootstrap-rejected") {
    const cell = ledger.preparedAdmission(slot);
    if (cell) {
      slot.cell = cell;
      slot.phase = "cell-held";
    } else
      clearBuilderSlot(slot);
    return step10("pending", "resident-builder-rejection-observation", 64);
  }
  if (slot.phase === "preparing") {
    const cell = ledger.preparedAdmission(slot);
    if (!cell)
      return step10("blocked", "resident-builder-cell-handoff");
    slot.cell = cell;
    slot.phase = "cell-held";
    return step10("pending", "resident-builder-cell-observation", 64);
  }
  if (slot.phase === "claiming") {
    if (!slot.cell?.claimed)
      return step10("rejected", "resident-builder-claim");
    slot.phase = "claimed";
    return step10("pending", "resident-builder-claim-observation", 64);
  }
  if (slot.phase === "record-admitting") {
    slot.record = slot.cell.result?.record ?? null;
    slot.phase = slot.record && slot.cell.result?.step.kind === "ready" && !slot.cell.hasFailure ? "record-held" : "fault-held";
    return step10(slot.phase === "record-held" ? "pending" : "rejected", "resident-builder-record-observation", 64);
  }
  return null;
}
function admitBuilder(state7, field, grant) {
  const result3 = (current) => ({ step: current, builder: null });
  if (!admitted18(grant, 32))
    return result3(step10("blocked", "resident-builder-admission"));
  const slot = state7.pending;
  if (!activePayload(state7) || !slot || state7.field !== field || !OwnedKernelReturnInputField.matchesResidentPayload(field, state7.facade))
    return result3(step10("rejected", "resident-builder-owner"));
  if (pageSlot(slot) || readerSlot(slot) || evidenceSlot(slot))
    return result3(step10("blocked", "resident-builder-slot-busy"));
  try {
    if (state7.builder?.phase === "live") {
      const builder = state7.builder.facade;
      return OwnedUiOperationPayloadBuilder.healthy(builder) && !state7.builder.cell?.hasFailure ? { step: step10("ready", "resident-builder-held"), builder } : result3(step10("rejected", "resident-builder-held"));
    }
    if (slot.requestOwner && slot.requestOwner !== field)
      return result3(step10("blocked", "resident-builder-slot-busy"));
    if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure)
      return result3(step10("rejected", "resident-builder-fault-held"));
    const observed = observeBuilderSlot(state7, slot, grant);
    if (observed)
      return result3(observed);
    const ledger = state7.instance.pool.ledger;
    if (slot.phase === "empty") {
      if (!admitted18(grant, 296))
        return result3(step10("blocked", "resident-builder-bootstrap"));
      slot.requestOwner = field;
      slot.phase = "preparing";
      const current = ledger.prepareAdmission(slot, "data", grant);
      if (current.kind === "blocked")
        clearBuilderSlot(slot);
      else if (current.kind === "rejected")
        slot.phase = "bootstrap-rejected";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "cell-held") {
      slot.phase = "claiming";
      const current = ledger.claimAdmission(slot, slot.cell, grant);
      if (current.kind === "blocked")
        slot.phase = "cell-held";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "claimed") {
      if (!admitted18(grant, 264))
        return result3(step10("blocked", "resident-builder-record"));
      slot.phase = "record-admitting";
      const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("builder"), slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "record-held") {
      if (!admitted18(grant, 56))
        return result3(step10("blocked", "resident-builder-entry"));
      const entry = { facade: null, record: slot.record, cell: slot.cell, phase: "constructing", witness: null };
      slot.entry = entry;
      state7.builder = entry;
      slot.phase = "entry-held";
      return result3(step10("pending", "resident-builder-entry", 56));
    }
    if (slot.phase === "entry-held") {
      if (!admitted18(grant, 272))
        return result3(step10("blocked", "resident-builder-shell"));
      slot.phase = "constructing";
      OwnedUiOperationPayloadBuilder.construct(field, state7.facade, grant);
      slot.phase = "shell-installed";
      return result3(step10("pending", "resident-builder-shell", 272));
    }
    if (slot.phase === "shell-installed") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-builder-source-bind"));
      slot.phase = "source-installing";
      return result3(admissionStep(OwnedUiOperationPayloadBuilder.bindSource(slot.entry.facade, state7.facade, grant), grant));
    }
    if (slot.phase === "source-installing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-builder-source-observation"));
      if (!OwnedKernelReturnInputField.matchesBuilder(field, slot.entry.facade))
        return result3(step10("rejected", "resident-builder-source-observation"));
      slot.phase = "source-bound";
      return result3(step10("pending", "resident-builder-source-observation", 64));
    }
    if (slot.phase === "source-bound") {
      if (!admitted18(grant, 32))
        return result3(step10("blocked", "resident-builder-witness"));
      createBuilderWitness(slot.entry);
      slot.phase = "witness-ready";
      return result3(step10("pending", "resident-builder-witness", 32));
    }
    if (slot.phase === "witness-ready") {
      const current = OwnedUiOperationPayloadBuilder.finalize(slot.entry.facade, state7.facade, grant);
      if (current.kind === "pending")
        slot.phase = "finalized";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "finalized") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-builder-publication"));
      const entry = slot.entry;
      entry.phase = "live";
      clearBuilderSlot(slot);
      return { step: step10("ready", "resident-builder-publication", 64), builder: entry.facade };
    }
    return result3(step10("rejected", "resident-builder-phase"));
  } catch (error) {
    builderFault(slot, error);
    return result3(step10("rejected", "resident-builder-fault"));
  }
}
function closeBuilderSlot(state7, grant) {
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-builder-close");
  const slot = state7.pending;
  const ledger = state7.instance.pool.ledger;
  if (readerSlot(slot) || state7.reader && payloadSlotEmpty(slot))
    return closeReaderSlot(state7, grant);
  if (pageSlot(slot) || state7.head && payloadSlotEmpty(slot))
    return closePageSlot(state7, grant);
  if (evidenceSlot(slot)) {
    if (slot.failure !== NO_POOL_FAULT) {
      if (!slot.cell) {
        const cell = ledger.preparedAdmission(slot);
        if (!cell)
          return step10("rejected", "resident-evidence-fault-held");
        slot.cell = cell;
        return step10("pending", "resident-evidence-fault-observation", 64);
      }
      if (!slot.cell.hasFailure)
        return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
      return step10("rejected", "resident-evidence-fault-held");
    }
    if (OwnedUiOperationPayloadBuilder.cancellationPrepared(slot.requestOwner) && (!state7.evidence || state7.evidence.phase === "constructing"))
      return admissionStep(admitEvidence(state7, slot.requestOwner, grant).step, grant);
    return advanceEvidence(state7, slot.requestOwner, grant);
  }
  if (state7.evidence)
    return state7.builder?.facade ? advanceEvidence(state7, state7.builder.facade, grant) : step10("rejected", "resident-evidence-builder-missing");
  if (payloadSlotEmpty(slot) && state7.builder?.facade && state7.builder.phase === "live") {
    const builder = state7.builder.facade;
    if (OwnedUiOperationPayloadBuilder.activeInput(builder))
      return childStep4(OwnedUiOperationPayloadBuilder.prepareInputCancellation(builder, state7.facade, grant), grant);
    if (OwnedUiOperationPayloadBuilder.cancellationPrepared(builder))
      return admissionStep(admitEvidence(state7, builder, grant).step, grant);
  }
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) {
      const cell = ledger.preparedAdmission(slot);
      if (!cell)
        return step10("rejected", "resident-builder-fault-held");
      slot.cell = cell;
      return step10("pending", "resident-builder-fault-observation", 64);
    }
    if (!slot.cell.hasFailure)
      return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
    return step10("rejected", "resident-builder-fault-held");
  }
  const observed = observeBuilderSlot(state7, slot, grant);
  if (observed)
    return observed;
  if (state7.builder && !slot.entry) {
    const entry2 = state7.builder;
    slot.requestOwner = state7.field;
    slot.cell = entry2.cell;
    slot.record = entry2.record;
    slot.entry = entry2;
    slot.witness = entry2.witness;
    slot.phase = "closing-domain";
    return step10("pending", "resident-builder-close-capture", 64);
  }
  const entry = slot.entry;
  if (entry) {
    const builder = entry.facade;
    if (builder && !OwnedUiOperationPayloadBuilder.empty(builder)) {
      builder.beginClose();
      if (!OwnedUiOperationPayloadBuilder.bodyEmpty(builder))
        return childStep4(builder.closeStep(grant), grant);
      if (!entry.witness) {
        createBuilderWitness(entry);
        slot.witness = entry.witness;
        return step10("pending", "resident-builder-close-witness", 32);
      }
      slot.witness = entry.witness;
      const field = slot.requestOwner;
      if (!field)
        return step10("rejected", "resident-builder-binding-owner");
      try {
        if (slot.phase === "binding-detaching") {
          if (OwnedKernelReturnInputField.matchesBuilderDetached(field, entry.witness)) {
            if (!OwnedUiOperationPayloadBuilder.sourceDetached(builder))
              return childStep4(OwnedUiOperationPayloadBuilder.detachRetirementSource(builder, state7.facade, grant), grant);
            moveBuilderWitness(entry.witness, builder, "source-detached");
            slot.phase = "binding-settling";
            return step10("pending", "resident-builder-source-detachment-observation", 64);
          }
          return childStep4({ ...field.detachBuilder(builder, entry.witness, grant), phase: "resident-builder-source-detach" }, grant);
        }
        if (slot.phase === "binding-settling") {
          if (OwnedKernelReturnInputField.matchesBuilderSettled(field, entry.witness)) {
            moveBuilderWitness(entry.witness, builder, "source-settled");
            slot.phase = "binding-settled";
            return step10("pending", "resident-builder-source-settlement-observation", 64);
          }
          return childStep4({ ...field.settleBuilder(entry.witness, grant), phase: "resident-builder-source-settle" }, grant);
        }
        if (slot.phase === "binding-settled")
          return childStep4(OwnedUiOperationPayloadBuilder.finishRetirement(builder, state7.facade, grant), grant);
        moveBuilderWitness(entry.witness, builder, "body-retired");
        slot.phase = "binding-detaching";
        return step10("pending", "resident-builder-body-proof", 64);
      } catch (error) {
        builderFault(slot, error);
        return step10("rejected", "resident-builder-binding-fault");
      }
    }
    if (builder && !entry.witness) {
      createBuilderWitness(entry);
      return step10("pending", "resident-builder-close-witness", 32);
    }
    if (builder) {
      markBuilderWitness(entry.witness, builder);
      slot.witness = entry.witness;
    }
    entry.facade = null;
    entry.cell = null;
    entry.record = null;
    entry.witness = null;
    entry.phase = "retired";
    state7.builder = null;
    slot.entry = null;
    slot.phase = "handoff-observing";
    return step10("pending", "resident-builder-domain-unlink", 64);
  }
  if (slot.phase === "handoff-observing") {
    if (slot.witness && !slot.witness.terminal)
      return step10("rejected", "resident-builder-proof");
    slot.record.beginClose();
    slot.phase = slot.witness ? "detaching" : "record-closing";
    return step10("pending", "resident-builder-record-begin", 64);
  }
  if (slot.phase === "detaching") {
    if (OwnedResidentRecordDetachment.matches(slot.record.detachment, slot.record, slot.witness.builder)) {
      slot.phase = "record-closing";
      return step10("pending", "resident-builder-detach-observation", 64);
    }
    return childStep4(slot.record.detach(slot.witness.builder, grant), grant);
  }
  if (slot.phase === "record-closing") {
    const current = slot.record.closeStep(grant);
    const forwarded = childStep4(current, grant);
    if (current.kind === "complete" && forwarded.kind === "pending")
      slot.phase = "record-observing";
    return forwarded;
  }
  if (slot.phase === "record-observing") {
    if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
      return step10("rejected", "resident-builder-record-proof");
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-builder-cell-begin", 64);
  }
  if (slot.phase === "cell-closing") {
    const current = slot.cell.closeStep(grant);
    const forwarded = childStep4(current, grant);
    if (current.kind === "complete" && forwarded.kind === "pending")
      slot.phase = "cell-observing";
    return forwarded;
  }
  if (slot.phase === "cell-observing") {
    if (!slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
      return step10("rejected", "resident-builder-cell-proof");
    clearBuilderSlot(slot);
    return step10("complete", "resident-builder-slot-close", 64);
  }
  if (slot.record) {
    slot.record.beginClose();
    slot.phase = "record-closing";
    return step10("pending", "resident-builder-unused-record", 64);
  }
  if (slot.cell) {
    slot.cell.beginClose();
    slot.phase = "cell-closing";
    return step10("pending", "resident-builder-unused-cell", 64);
  }
  clearBuilderSlot(slot);
  return step10("complete", "resident-builder-slot-close", 64);
}

class EvidenceWitness {
  #evidence;
  #terminal = false;
  constructor(mint, entry) {
    if (mint !== MINT11 || !entry.facade)
      throw new Error("Invalid evidence witness");
    this.#evidence = entry.facade;
    entry.witness = this;
    Object.freeze(this);
  }
  static {
    createEvidenceWitness = (entry) => new EvidenceWitness(MINT11, entry);
    markEvidenceWitness = (witness, token) => {
      if (witness.#evidence !== token || !OwnedUiOperationPayloadBuilder.evidenceEmpty(token))
        throw new Error("Invalid evidence terminal proof");
      witness.#terminal = true;
    };
  }
  get evidence() {
    return this.#evidence;
  }
  get terminal() {
    return this.#terminal && OwnedUiOperationPayloadBuilder.evidenceEmpty(this.#evidence);
  }
}
function admitEvidence(state7, builder, grant) {
  const result3 = (current) => ({ step: current, evidence: null });
  const slot = state7.pending;
  if (!admitted18(grant, 32))
    return result3(step10("blocked", "resident-evidence-admission"));
  if (!state7.instance?.pool?.ledger || !state7.facade || !slot || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure || state7.builder?.facade !== builder || state7.builder.phase !== "live" || state7.builder.cell?.hasFailure || !OwnedUiOperationPayloadBuilder.evidenceEligible(builder, state7.facade))
    return result3(step10("rejected", "resident-evidence-owner"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure)
    return result3(step10("rejected", "resident-evidence-fault-held"));
  if (state7.evidence?.phase === "live")
    return !state7.evidence.cell?.hasFailure && OwnedUiOperationPayloadBuilder.matchesEvidence(state7.evidence.facade, builder) ? { step: step10("ready", "resident-evidence-held"), evidence: state7.evidence.facade } : result3(step10("rejected", "resident-evidence-held"));
  if (slot.requestOwner !== null && slot.requestOwner !== builder)
    return result3(step10("blocked", "resident-evidence-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure)
    return result3(step10("rejected", "resident-evidence-fault-held"));
  try {
    const ledger = state7.instance.pool.ledger;
    if (slot.phase === "empty") {
      if (!admitted18(grant, 296))
        return result3(step10("blocked", "resident-evidence-bootstrap"));
      slot.requestOwner = builder;
      slot.phase = "preparing";
      const current = ledger.prepareAdmission(slot, "data", grant);
      if (current.kind === "blocked")
        clearBuilderSlot(slot);
      else if (current.kind === "rejected")
        slot.phase = "bootstrap-rejected";
      return result3(admissionStep(current, grant));
    }
    if (!evidenceSlot(slot))
      return result3(step10("rejected", "resident-evidence-slot-owner"));
    const observed = observeBuilderSlot(state7, slot, grant);
    if (observed)
      return result3(observed);
    if (slot.phase === "cell-held") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-evidence-claim"));
      slot.phase = "claiming";
      const current = ledger.claimAdmission(slot, slot.cell, grant);
      if (current.kind === "blocked")
        slot.phase = "cell-held";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "claimed") {
      if (!admitted18(grant, 264))
        return result3(step10("blocked", "resident-evidence-record"));
      slot.phase = "record-admitting";
      const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("evidence"), slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "record-held") {
      if (!admitted18(grant, 56))
        return result3(step10("blocked", "resident-evidence-entry"));
      const entry = { facade: null, record: slot.record, cell: slot.cell, phase: "constructing", witness: null };
      state7.evidence = entry;
      slot.entry = entry;
      slot.phase = "entry-held";
      return result3(step10("pending", "resident-evidence-entry", 56));
    }
    if (slot.phase === "entry-held") {
      if (!admitted18(grant, 168))
        return result3(step10("blocked", "resident-evidence-shell"));
      slot.phase = "constructing";
      OwnedUiOperationPayloadBuilder.constructEvidence(builder, state7.facade, grant);
      slot.phase = "shell-installed";
      return result3(step10("pending", "resident-evidence-shell", 168));
    }
    if (slot.phase === "shell-installed") {
      if (!admitted18(grant, 32))
        return result3(step10("blocked", "resident-evidence-witness"));
      createEvidenceWitness(slot.entry);
      slot.phase = "witness-ready";
      return result3(step10("pending", "resident-evidence-witness", 32));
    }
    if (slot.phase === "witness-ready") {
      const current = OwnedUiOperationPayloadBuilder.finalizeEvidence(slot.entry.facade, builder, state7.facade, grant);
      if (current.kind === "pending")
        slot.phase = "finalized";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "finalized") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-evidence-publication"));
      const entry = slot.entry;
      OwnedUiOperationPayloadBuilder.publishEvidence(entry.facade, builder, state7.facade);
      entry.phase = "live";
      clearBuilderSlot(slot);
      return { step: step10("ready", "resident-evidence-publication", 64), evidence: entry.facade };
    }
    return result3(step10("rejected", "resident-evidence-phase"));
  } catch (error) {
    builderFault(slot, error);
    return result3(step10("rejected", "resident-evidence-fault"));
  }
}
function advanceEvidence(state7, builder, grant) {
  if (!admitted18(grant, 1))
    return step10("blocked", "resident-evidence-retirement");
  const slot = state7.pending;
  if (!slot || !state7.facade || state7.builder?.facade !== builder || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
    return step10("rejected", "resident-evidence-retirement-owner");
  if (!state7.evidence && payloadSlotEmpty(slot))
    return step10("complete", "resident-evidence-retirement");
  if (slot.requestOwner !== null && slot.requestOwner !== builder)
    return step10("blocked", "resident-evidence-slot-busy");
  try {
    if (slot.phase === "closing-domain" && evidenceSlot(slot) && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure && slot.entry?.facade && !OwnedUiOperationPayloadBuilder.evidenceEmpty(slot.entry.facade))
      return childStep4(OwnedUiOperationPayloadBuilder.advanceEvidence(slot.entry.facade, builder, state7.facade, grant), grant);
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-evidence-retirement");
    if (slot.failure !== NO_POOL_FAULT) {
      if (slot.cell && !slot.cell.hasFailure)
        return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
      return step10("rejected", "resident-evidence-fault-held");
    }
    if (slot.cell?.hasFailure)
      return step10("rejected", "resident-evidence-fault-held");
    if (slot.phase === "empty") {
      const entry = state7.evidence;
      if (!entry || entry.phase !== "live" || !entry.facade || !OwnedUiOperationPayloadBuilder.matchesEvidence(entry.facade, builder))
        return step10("rejected", "resident-evidence-capture-owner");
      slot.requestOwner = builder;
      if (!evidenceSlot(slot))
        throw new Error("Invalid evidence slot");
      slot.entry = entry;
      slot.cell = entry.cell;
      slot.record = entry.record;
      slot.witness = entry.witness;
      slot.phase = "closing-domain";
      entry.phase = "closing";
      return step10("pending", "resident-evidence-capture", 64);
    }
    if (!evidenceSlot(slot))
      return step10("rejected", "resident-evidence-slot-owner");
    if (slot.phase === "closing-domain") {
      const entry = slot.entry;
      const token = entry?.facade;
      if (!entry || !token || !entry.witness || slot.witness !== entry.witness)
        return step10("rejected", "resident-evidence-entry-owner");
      if (!OwnedUiOperationPayloadBuilder.evidenceEmpty(token))
        return childStep4(OwnedUiOperationPayloadBuilder.advanceEvidence(token, builder, state7.facade, grant), grant);
      markEvidenceWitness(entry.witness, token);
      entry.facade = null;
      entry.record = null;
      entry.cell = null;
      entry.witness = null;
      entry.phase = "retired";
      state7.evidence = null;
      slot.entry = null;
      slot.phase = "handoff-observing";
      return step10("pending", "resident-evidence-domain-unlink", 64);
    }
    if (slot.phase === "handoff-observing") {
      if (!slot.witness?.terminal || !slot.record)
        return step10("rejected", "resident-evidence-body-proof");
      slot.record.beginClose();
      slot.phase = "detaching";
      return step10("pending", "resident-evidence-record-begin", 64);
    }
    if (slot.phase === "detaching") {
      if (OwnedResidentRecordDetachment.matches(slot.record.detachment, slot.record, slot.witness.evidence)) {
        slot.phase = "record-closing";
        return step10("pending", "resident-evidence-detach-observation", 64);
      }
      return childStep4(slot.record.detach(slot.witness.evidence, grant), grant);
    }
    if (slot.phase === "record-closing") {
      const current = slot.record.closeStep(grant);
      const forwarded = childStep4(current, grant);
      if (current.kind === "complete" && forwarded.kind === "pending")
        slot.phase = "record-observing";
      return forwarded;
    }
    if (slot.phase === "record-observing") {
      if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
        return step10("rejected", "resident-evidence-record-proof");
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-evidence-cell-begin", 64);
    }
    if (slot.phase === "cell-closing") {
      const current = slot.cell.closeStep(grant);
      const forwarded = childStep4(current, grant);
      if (current.kind === "complete" && forwarded.kind === "pending")
        slot.phase = "cell-observing";
      return forwarded;
    }
    if (slot.phase === "cell-observing") {
      if (!slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
        return step10("rejected", "resident-evidence-cell-proof");
      clearBuilderSlot(slot);
      return step10("complete", "resident-evidence-slot-close", 64);
    }
    const observed = observeBuilderSlot(state7, slot, grant);
    if (observed)
      return observed;
    if (slot.phase === "shell-installed" || slot.phase === "witness-ready" || slot.phase === "finalized")
      return admissionStep(admitEvidence(state7, builder, grant).step, grant);
    if (slot.phase === "entry-held") {
      const entry = slot.entry;
      if (!entry || entry.facade || entry.witness || entry.record !== slot.record || entry.cell !== slot.cell)
        return step10("rejected", "resident-evidence-unused-entry");
      entry.record = null;
      entry.cell = null;
      entry.phase = "retired";
      state7.evidence = null;
      slot.entry = null;
      slot.phase = "record-held";
      return step10("pending", "resident-evidence-unused-entry", 64);
    }
    if (slot.phase === "record-held") {
      if (!slot.record || slot.entry || slot.witness)
        return step10("rejected", "resident-evidence-unused-record");
      slot.record.beginClose();
      slot.phase = "record-closing";
      return step10("pending", "resident-evidence-unused-record", 64);
    }
    if (slot.phase === "cell-held" || slot.phase === "claimed") {
      if (!slot.cell || slot.record || slot.entry || slot.witness)
        return step10("rejected", "resident-evidence-unused-cell");
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-evidence-unused-cell", 64);
    }
    return step10("blocked", "resident-evidence-construction-held");
  } catch (error) {
    builderFault(slot, error);
    return step10("rejected", "resident-evidence-retirement-fault");
  }
}

class OwnedUiResidentPayload {
  #state;
  constructor(mint, state7) {
    if (mint !== MINT11)
      throw new Error("Invalid resident payload authority");
    this.#state = state7;
    state7.facade = this;
    const instance = state7.instance;
    if (instance.tail)
      instance.tail.next = state7;
    else
      instance.head = state7;
    instance.tail = state7;
    instance.children++;
    const installed = state7.record.install(this, { maxItems: 1, maxBytes: 64 });
    if (installed.kind !== "ready")
      throw new Error("Resident payload record refused installation");
  }
  static {
    payloadOwner = (state7) => new OwnedUiResidentPayload(MINT11, state7);
    payloadState = (payload) => payload !== null && typeof payload === "object" && (#state in payload) ? payload.#state : null;
    closePayload = (state7, grant) => {
      state7.closing = true;
      return state7.facade ? state7.facade.#close(grant) : closePayloadSlot(null, state7.parentSlot, grant);
    };
  }
  static matchesBuilderConstruction(payload, field) {
    const state7 = payloadState(payload);
    return state7 !== null && activePayload(state7) && state7.field === field && state7.pending?.phase === "constructing" && state7.pending.entry === state7.builder && state7.builder !== null && state7.builder.facade === null && state7.pending.failure === NO_POOL_FAULT && !state7.pending.cell?.hasFailure;
  }
  static matchesBuilderPhase(payload, builder, phase) {
    const state7 = payloadState(payload);
    return state7 !== null && activePayload(state7) && state7.builder?.facade === builder && state7.pending?.entry === state7.builder && state7.pending.phase === phase && state7.pending.failure === NO_POOL_FAULT && !state7.pending.cell?.hasFailure;
  }
  static matchesBuilderLive(payload, builder) {
    const state7 = payloadState(payload);
    const entry = state7?.builder;
    return state7 !== null && activePayload(state7) && entry !== null && entry !== undefined && entry.facade === builder && entry.phase === "live" && !entry.cell?.hasFailure;
  }
  static matchesBuilderRetirement(payload, builder, field, witness) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && !state7.builder?.cell?.hasFailure && slot !== null && slot !== undefined && !pageSlot(slot) && !readerSlot(slot) && !evidenceSlot(slot) && slot.requestOwner !== null && slot.requestOwner === field && slot.entry !== null && slot.entry === state7.builder && slot.entry.facade === builder && slot.entry.witness === witness && slot.witness === witness && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure && !state7.evidence;
  }
  static matchesBuilderRetirementPhase(payload, builder, phase) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    if (!state7 || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure || !slot || pageSlot(slot) || evidenceSlot(slot) || slot.phase !== phase || !slot.requestOwner || !slot.witness || !OwnedUiResidentPayload.matchesBuilderRetirement(payload, builder, slot.requestOwner, slot.witness))
      return false;
    return phase === "binding-detaching" ? OwnedKernelReturnInputField.matchesBuilderDetached(slot.requestOwner, slot.witness) : OwnedKernelReturnInputField.matchesBuilderSettled(slot.requestOwner, slot.witness);
  }
  installBuilder(builder, grant) {
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-builder-install");
    const state7 = this.#state;
    const slot = state7.pending;
    const entry = state7.builder;
    if (!slot || slot.phase !== "constructing" || slot.entry !== entry || !entry || entry.facade || !OwnedUiOperationPayloadBuilder.matchesResident(builder, this) || !OwnedUiOperationPayloadBuilder.matchesField(builder, state7.field))
      return step10("rejected", "resident-builder-install");
    entry.facade = builder;
    return entry.record.install(builder, grant);
  }
  beginBuilder(field, grant) {
    return admitBuilder(this.#state, field, grant);
  }
  beginEvidence(builder, grant) {
    return admitEvidence(this.#state, builder, grant);
  }
  static matchesInputCancellation(payload, builder) {
    const state7 = payloadState(payload);
    return state7 !== null && state7.closing && state7.phase === "live" && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && state7.builder !== null && state7.builder.facade === builder && state7.builder.phase === "live" && !state7.builder.cell?.hasFailure && state7.pending !== null && payloadSlotEmpty(state7.pending) && !state7.reader && !state7.head && !state7.evidence;
  }
  advanceEvidence(builder, grant) {
    return advanceEvidence(this.#state, builder, grant);
  }
  static matchesEvidenceRetirement(payload, builder, token) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && !state7.builder?.cell?.hasFailure && slot !== null && slot !== undefined && evidenceSlot(slot) && slot.requestOwner === builder && state7.builder?.facade === builder && slot.phase === "closing-domain" && slot.entry !== null && slot.entry === state7.evidence && slot.entry.facade === token && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  static matchesEvidenceCancellation(payload, builder, token) {
    const state7 = payloadState(payload);
    return state7 !== null && state7.closing && OwnedUiResidentPayload.matchesEvidenceRetirement(payload, builder, token);
  }
  static matchesEvidencePhase(payload, builder, phase) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && !state7.builder?.cell?.hasFailure && slot !== null && slot !== undefined && evidenceSlot(slot) && slot.requestOwner === builder && slot.phase === phase && slot.entry === state7.evidence && state7.evidence !== null && state7.builder?.facade === builder && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  installEvidence(token, builder, grant) {
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-evidence-install");
    const state7 = this.#state;
    const slot = state7.pending;
    const entry = state7.evidence;
    if (!slot || !evidenceSlot(slot) || slot.requestOwner !== builder || slot.phase !== "constructing" || slot.entry !== entry || !entry || entry.facade || !OwnedUiOperationPayloadBuilder.matchesEvidenceConstruction(token, builder))
      return step10("rejected", "resident-evidence-install");
    entry.facade = token;
    return entry.record.install(token, grant);
  }
  static matchesField(payload, field) {
    const state7 = payloadState(payload);
    return state7 !== null && state7.field !== null && state7.field === field;
  }
  static matchesScope(payload, scope) {
    const state7 = payloadState(payload);
    return state7 !== null && state7.instance !== null && state7.instance.facade !== null && state7.instance.facade === scope;
  }
  static matchesSourceDetachment(payload, observation) {
    const state7 = payloadState(payload);
    return state7 !== null && state7.field === null && state7.parentSlot !== null && state7.parentSlot.entry === state7 && state7.parentSlot.witness !== null && state7.parentSlot.witness === observation && (state7.phase === "source-detached" || state7.phase === "source-settled") && payloadBodyEmpty(state7);
  }
  static matchesOwner(payload, owner, activation, lifetime) {
    if (payload === null || typeof payload !== "object" || !(#state in payload))
      return false;
    const state7 = payload.#state;
    const instance = state7.instance;
    return !state7.closing && !state7.closed && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && instance !== null && active(instance) && instance.lifetime !== null && instance.owner === owner && instance.activation === activation && instance.lifetime.activationGeneration === lifetime.activationGeneration && instance.lifetime.instanceId === lifetime.instanceId && instance.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  beginReader(builder, grant) {
    return admitReader(this.#state, builder, grant);
  }
  closeReader(reader, grant) {
    const original = readerState(reader);
    const state7 = this.#state;
    if (!original || original.payload !== state7 && (!state7.pending || !readerSlot(state7.pending) || state7.pending.witness?.original !== reader))
      return step10("rejected", "resident-reader-close-owner");
    return closeReaderSlot(state7, grant, original);
  }
  static matchesReaderConstruction(payload, reader) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && slot !== null && slot !== undefined && readerSlot(slot) && slot.phase === "builder-installing" && slot.entry === state7.reader && slot.entry?.facade === reader && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  static matchesReaderBinding(payload, reader, witness) {
    const state7 = payloadState(payload);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && slot !== null && slot !== undefined && readerSlot(slot) && slot.entry === state7.reader && slot.entry?.facade === reader && slot.witness === witness && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  static matchesPageConstruction(payload, page) {
    const state7 = payloadState(payload);
    const original = pageState2(page);
    const slot = state7?.pending;
    return state7 !== null && activePayload(state7) && original !== null && original.payload === state7 && slot !== null && slot !== undefined && pageSlot(slot) && slot.phase === "page-builder-installing" && slot.entry === original && original.facade === page && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  static matchesPageRetirement(payload, page, proof) {
    const state7 = payloadState(payload);
    const original = pageState2(page);
    const slot = state7?.pending;
    return state7 !== null && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && original !== null && original.payload === state7 && original.failure === NO_POOL_FAULT && !original.cell?.hasFailure && state7.reader?.page !== original && slot !== null && slot !== undefined && pageSlot(slot) && slot.phase === "closing-domain" && slot.entry === original && original.facade === page && slot.witness !== null && slot.witness === proof && original.witness === proof && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure;
  }
  static pageLength(payload, builder, page) {
    const state7 = payloadState(payload);
    const original = pageState2(page);
    return state7 !== null && activePayload(state7) && state7.builder?.facade === builder && original !== null && original.payload === state7 && original.facade === page && (original.phase === "live" || original.phase === "sealed") && original.failure === NO_POOL_FAULT && !original.cell?.hasFailure ? original.length : null;
  }
  beginPage(builder, length, grant) {
    return admitPage(this.#state, builder, length, grant);
  }
  closePage(page, grant) {
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-page-close");
    const original = pageState2(page);
    const state7 = this.#state;
    if (!original)
      return step10("rejected", "resident-page-close-owner");
    const slot = state7.pending;
    if (original.payload !== state7 && (!slot || !pageSlot(slot) || slot.witness?.original !== page))
      return step10("rejected", "resident-page-close-owner");
    return closePageSlot(state7, grant, original);
  }
  beginClose() {
    const state7 = this.#state;
    if (state7.closing)
      return;
    state7.closing = true;
    state7.cursor = state7.head;
  }
  closeStep(grant) {
    try {
      return this.#close(grant);
    } catch (error) {
      const state7 = this.#state;
      if (state7.parentSlot)
        payloadSlotFault(state7.parentSlot, error);
      else {
        if (state7.failure !== NO_POOL_FAULT && !Object.is(state7.failure, error))
          throw error;
        state7.failure = error;
      }
      return step10("rejected", "resident-payload-close-fault");
    }
  }
  #close(grant) {
    const state7 = this.#state;
    if (admitted18(grant, 32) && state7.closing && state7.phase === "live" && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && state7.builder?.facade && state7.pending && (payloadSlotEmpty(state7.pending) || evidenceSlot(state7.pending)) && !state7.reader && !state7.head && (!state7.evidence || state7.evidence.phase === "constructing") && OwnedUiOperationPayloadBuilder.cancellationPrepared(state7.builder.facade))
      return admissionStep(admitEvidence(state7, state7.builder.facade, grant).step, grant);
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-payload-close");
    if (!state7.closing)
      throw new Error("Resident payload close has not begun");
    if (state7.closed)
      return step10("complete", "resident-payload-close");
    if (state7.phase === "domain-retired")
      return closePayloadSlot(null, state7.parentSlot, grant);
    if (state7.failure !== NO_POOL_FAULT) {
      if (state7.cell && !state7.cell.hasFailure)
        return childStep4(state7.cell.retainFailure(state7.failure, grant), grant);
      return step10("rejected", "resident-payload-fault-held");
    }
    if (state7.cell?.hasFailure)
      return step10("rejected", "resident-payload-fault-held");
    if (state7.pending && state7.pending.phase !== "empty" || state7.reader || state7.builder || state7.head)
      return childStep4(closeBuilderSlot(state7, grant), grant);
    if (state7.storageCell)
      return closeStorage(state7, grant);
    if (state7.reader || state7.builder)
      return step10("blocked", "resident-payload-child-registration");
    if (state7.cursor)
      return childStep4(closePageSlot(state7, grant, state7.cursor), grant);
    if (state7.evidence || state7.head || state7.tail)
      return step10("blocked", "resident-payload-readers");
    const instance = state7.instance;
    const slot = instance.pending;
    if (!state7.parentSlot) {
      if (slot.phase !== "empty")
        return step10("blocked", "resident-payload-slot-busy");
      slot.requestOwner = state7.field;
      slot.cell = state7.cell;
      slot.record = state7.record;
      slot.entry = state7;
      slot.witness = state7.witness;
      slot.phase = "body-proving";
      state7.parentSlot = slot;
      return step10("pending", "resident-payload-retirement-capture", 64);
    }
    if (!state7.witness) {
      payloadWitness(state7);
      return step10("pending", "resident-payload-witness", 32);
    }
    if (state7.phase === "constructing" || state7.phase === "live") {
      if (!payloadBodyEmpty(state7))
        return step10("rejected", "resident-payload-body-proof");
      movePayloadWitness(state7, "body-retired");
      slot.phase = slot.phase === "shell-installed" ? "source-never-installed" : "source-detaching";
      return step10("pending", "resident-payload-body-proof", 64);
    }
    const field = slot.requestOwner;
    if (slot.phase === "source-detaching" || slot.phase === "source-never-installed") {
      if (!field)
        return step10("rejected", "resident-payload-source-owner");
      const observation = field.residentPayloadDetachment;
      if (OwnedKernelReturnPayloadDetachment.matches(observation, field, this)) {
        slot.witness = observation;
        slot.phase = "source-clearing";
        return step10("pending", "resident-payload-source-observation", 64);
      }
      if (slot.phase === "source-never-installed" && !OwnedKernelReturnInputField.matchesResidentPayload(field, this)) {
        state7.field = null;
        movePayloadWitness(state7, "source-settled");
        slot.phase = "source-settled";
        return step10("pending", "resident-payload-never-installed-source", 64);
      }
      const current = field.detachResidentPayload(this, state7.witness, grant);
      const forwarded = childStep4({ ...current, phase: "resident-payload-source-detach" }, grant);
      if (current.kind === "pending" && forwarded.kind === "pending")
        slot.phase = "source-observing";
      return forwarded;
    }
    if (slot.phase === "source-observing") {
      const observation = field.residentPayloadDetachment;
      if (!OwnedKernelReturnPayloadDetachment.matches(observation, field, this))
        return step10("rejected", "resident-payload-source-observation");
      slot.witness = observation;
      slot.phase = "source-clearing";
      return step10("pending", "resident-payload-source-observation", 64);
    }
    if (slot.phase === "source-clearing") {
      if (!OwnedKernelReturnPayloadDetachment.matches(slot.witness, field, this))
        return step10("rejected", "resident-payload-source-proof");
      state7.field = null;
      movePayloadWitness(state7, "source-detached");
      slot.phase = "source-settling";
      return step10("pending", "resident-payload-ui-source-detach", 64);
    }
    if (slot.phase === "source-settling") {
      if (OwnedKernelReturnPayloadDetachment.matchesSettled(slot.witness, this)) {
        slot.phase = "source-settle-observing";
        return step10("pending", "resident-payload-settle-recovery", 64);
      }
      if (!OwnedKernelReturnPayloadDetachment.matchesOwner(slot.witness, field))
        return step10("rejected", "resident-payload-source-proof");
      const current = field.settleResidentPayload(slot.witness, state7.witness, grant);
      const forwarded = childStep4({ ...current, phase: "resident-payload-source-settle" }, grant);
      if (current.kind === "complete" && forwarded.kind === "pending")
        slot.phase = "source-settle-observing";
      return forwarded;
    }
    if (slot.phase === "source-settle-observing") {
      if (!OwnedKernelReturnPayloadDetachment.matchesSettled(slot.witness, this))
        return step10("rejected", "resident-payload-settled-proof");
      movePayloadWitness(state7, "source-settled");
      slot.phase = "source-settled";
      return step10("pending", "resident-payload-settle-observation", 64);
    }
    if (slot.phase === "source-settled") {
      slot.witness = state7.witness;
      slot.requestOwner = null;
      slot.phase = "closing-domain";
      return step10("pending", "resident-payload-domain-proof-handoff", 64);
    }
    if (slot.phase !== "closing-domain" || !admitted18(grant, 128))
      return step10("blocked", "resident-payload-domain-unlink");
    if (state7.previous)
      state7.previous.next = state7.next;
    else
      instance.head = state7.next;
    if (state7.next)
      state7.next.previous = state7.previous;
    else
      instance.tail = state7.previous;
    instance.children--;
    state7.previous = null;
    state7.next = null;
    state7.facade = null;
    state7.instance = null;
    state7.record = null;
    state7.cell = null;
    state7.pending = null;
    movePayloadWitness(state7, "domain-retired");
    slot.phase = "handoff-observing";
    return step10("pending", "resident-payload-domain-unlink", 128);
  }
  terminalIsEmpty() {
    const state7 = this.#state;
    return state7.closed && !state7.instance && !state7.facade && !state7.head && !state7.tail && !state7.cursor && !state7.previous && !state7.next && !state7.builder && !state7.storageCell && !state7.reader && !state7.field && !state7.record && !state7.cell && !state7.witness && !state7.parentSlot && !state7.pending && state7.failure === NO_POOL_FAULT && !state7.evidence;
  }
}

class OwnedUiResidentPayloadSourceRelease {
  #payload;
  #phase = "constructed";
  constructor(mint, state7) {
    if (mint !== MINT11 || !state7.facade)
      throw new Error("Invalid resident payload source authority");
    this.#payload = state7.facade;
    state7.witness = this;
    Object.freeze(this);
  }
  static {
    payloadWitness = (state7) => new OwnedUiResidentPayloadSourceRelease(MINT11, state7);
    payloadWitnessOriginal = (witness) => witness.#payload;
    movePayloadWitness = (state7, phase) => {
      const witness = state7.witness;
      if (!witness || payloadState(witness.#payload) !== state7)
        throw new Error("Invalid payload witness transition");
      state7.phase = phase;
      witness.#phase = phase;
    };
  }
  static matches(proof, payload, field) {
    if (proof === null || typeof proof !== "object" || !(#payload in proof) || proof.#payload !== payload || proof.#phase !== "body-retired")
      return false;
    const state7 = payloadState(payload);
    return state7 !== null && state7.witness === proof && state7.phase === "body-retired" && state7.field !== null && state7.field === field && payloadBodyEmpty(state7);
  }
  static matchesDetached(proof, payload) {
    if (proof === null || typeof proof !== "object" || !(#payload in proof) || proof.#payload !== payload || proof.#phase !== "source-detached")
      return false;
    const state7 = payloadState(payload);
    return state7 !== null && state7.witness === proof && state7.phase === "source-detached" && state7.field === null && state7.parentSlot !== null && state7.parentSlot.entry === state7 && state7.parentSlot.witness !== null && payloadBodyEmpty(state7);
  }
}
function pageSlot(slot) {
  return typeof slot.requestOwner === "number";
}
function capturePageSlot(slot, page) {
  if (!payloadSlotEmpty(slot))
    return false;
  slot.requestOwner = page.length;
  return pageSlot(slot);
}
function pageStorage(page) {
  return page.storageCell?.result?.page ?? null;
}
function storageOwner(state7) {
  return state7.storageCell?.result?.owner ?? null;
}
function closeStorage(state7, grant) {
  const cell = state7.storageCell;
  if (!cell)
    return step10("complete", "resident-storage-empty");
  if (cell.hasFailure)
    return step10("rejected", "resident-storage-fault-held");
  if (!cell.terminalIsEmpty()) {
    cell.beginClose();
    return childStep4(cell.closeStep(grant), grant);
  }
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-storage-observation");
  state7.storageCell = null;
  return step10("pending", "resident-storage-observation", 64);
}
function admitPage(state7, builder, length, grant) {
  const result3 = (current) => ({ step: current, page: null });
  const slot = state7.pending;
  if (!admitted18(grant, 32))
    return result3(step10("blocked", "resident-page-admission"));
  if (!slot || !activePayload(state7) || !OwnedUiResidentPayload.matchesBuilderLive(state7.facade, builder) || !Number.isInteger(length) || length < 0 || length > 256)
    return result3(step10("rejected", "resident-page-owner"));
  if (slot.requestOwner !== null && (!pageSlot(slot) || slot.requestOwner !== length))
    return result3(step10("blocked", "resident-page-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure || state7.storageCell?.hasFailure || state7.head && (state7.head.failure !== NO_POOL_FAULT || state7.head.cell?.hasFailure || state7.head.storageCell?.hasFailure))
    return result3(step10("rejected", "resident-page-fault-held"));
  if (state7.head && (state7.head.phase === "live" || state7.head.phase === "sealed") && state7.head.length === length && !state7.head.cell?.hasFailure && !state7.head.storageCell?.hasFailure)
    return { step: step10("ready", "resident-page-held"), page: state7.head.facade };
  try {
    const ledger = state7.instance.pool.ledger;
    if (slot.phase === "empty") {
      if (state7.head)
        return result3(step10("blocked", "resident-page-window"));
      if (!admitted18(grant, 296))
        return result3(step10("blocked", "resident-page-bootstrap"));
      slot.requestOwner = length;
      if (!pageSlot(slot))
        throw new Error("Invalid page slot");
      slot.phase = state7.storageCell ? "preparing" : "owner-preparing";
      const current = ledger.prepareAdmission(slot, "data", grant);
      if (current.kind === "blocked")
        clearBuilderSlot(slot);
      else if (current.kind === "rejected")
        slot.phase = "bootstrap-rejected";
      return result3(admissionStep(current, grant));
    }
    if (!pageSlot(slot))
      return result3(step10("rejected", "resident-page-slot"));
    if (slot.phase === "owner-preparing" || slot.phase === "preparing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-cell-observation"));
      const cell = ledger.preparedAdmission(slot);
      if (!cell)
        return result3(step10("rejected", "resident-page-cell-handoff"));
      slot.cell = cell;
      slot.phase = slot.phase === "owner-preparing" ? "owner-cell-held" : "cell-held";
      return result3(step10("pending", "resident-page-cell-observation", 64));
    }
    if (slot.phase === "owner-cell-held" || slot.phase === "cell-held") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-claim"));
      const previous = slot.phase;
      slot.phase = previous === "owner-cell-held" ? "owner-claiming" : "claiming";
      const current = ledger.claimAdmission(slot, slot.cell, grant);
      if (current.kind === "blocked")
        slot.phase = previous;
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "owner-claiming" || slot.phase === "claiming") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-claim-observation"));
      if (!slot.cell?.claimed)
        return result3(step10("rejected", "resident-page-claim"));
      slot.phase = slot.phase === "owner-claiming" ? "owner-claimed" : "claimed";
      return result3(step10("pending", "resident-page-claim-observation", 64));
    }
    if (slot.phase === "owner-claimed") {
      if (!admitted18(grant, 200))
        return result3(step10("blocked", "resident-storage-admission"));
      slot.phase = "owner-admitting";
      const current = ledger.beginOwner("data", slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "owner-claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "owner-admitting") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-storage-observation"));
      if (!slot.cell?.result?.owner || slot.cell.hasFailure || slot.cell.result.step.kind !== "ready")
        return result3(step10("rejected", "resident-storage-result"));
      state7.storageCell = slot.cell;
      clearBuilderSlot(slot);
      return result3(step10("pending", "resident-storage-observation", 64));
    }
    if (slot.phase === "claimed") {
      if (!admitted18(grant, 264))
        return result3(step10("blocked", "resident-page-record"));
      slot.phase = "record-admitting";
      const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("page"), slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "record-admitting") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-record-observation"));
      slot.record = slot.cell.result?.record ?? null;
      if (!slot.record || slot.cell.hasFailure || slot.cell.result?.step.kind !== "ready")
        return result3(step10("rejected", "resident-page-record-result"));
      slot.phase = "record-held";
      return result3(step10("pending", "resident-page-record-observation", 64));
    }
    if (slot.phase === "record-held") {
      if (!admitted18(grant, 104))
        return result3(step10("blocked", "resident-page-state"));
      slot.entry = { payload: state7, facade: null, previous: null, next: null, length, cell: slot.cell, record: slot.record, storageCell: null, phase: "constructing", witness: null, failure: NO_POOL_FAULT };
      slot.phase = "page-state";
      return result3(step10("pending", "resident-page-state", 104));
    }
    if (slot.phase === "page-state") {
      if (!admitted18(grant, 88))
        return result3(step10("blocked", "resident-page-shell"));
      pageOwner(slot.entry);
      slot.phase = "page-shell";
      return result3(step10("pending", "resident-page-shell", 88));
    }
    const page = slot.entry;
    if (!page?.facade)
      return result3(step10("rejected", "resident-page-entry"));
    if (slot.phase === "page-shell") {
      if (!admitted18(grant, 32))
        return result3(step10("blocked", "resident-page-witness"));
      createPageWitness(page);
      slot.phase = "page-storage";
      page.phase = "storage-empty";
      return result3(step10("pending", "resident-page-witness", 32));
    }
    if (slot.phase === "page-storage") {
      if (page.phase === "storage-empty") {
        if (!admitted18(grant, 296))
          return result3(step10("blocked", "resident-page-storage-bootstrap"));
        page.phase = "storage-preparing";
        const current = ledger.prepareAdmission(page.facade, "data", grant);
        if (current.kind === "blocked")
          page.phase = "storage-empty";
        else if (current.kind === "rejected")
          page.phase = "storage-rejected";
        return result3(admissionStep(current, grant));
      }
      if (page.phase === "storage-preparing") {
        if (!admitted18(grant, 64))
          return result3(step10("blocked", "resident-page-storage-observation"));
        const cell = ledger.preparedAdmission(page.facade);
        if (!cell)
          return result3(step10("rejected", "resident-page-storage-handoff"));
        page.storageCell = cell;
        page.phase = "storage-cell-held";
        return result3(step10("pending", "resident-page-storage-observation", 64));
      }
      if (page.phase === "storage-cell-held") {
        if (!admitted18(grant, 64))
          return result3(step10("blocked", "resident-page-storage-claim"));
        page.phase = "storage-claiming";
        const current = ledger.claimAdmission(page.facade, page.storageCell, grant);
        if (current.kind === "blocked")
          page.phase = "storage-cell-held";
        return result3(admissionStep(current, grant));
      }
      if (page.phase === "storage-claiming") {
        if (!admitted18(grant, 64))
          return result3(step10("blocked", "resident-page-storage-claim-observation"));
        if (!page.storageCell?.claimed)
          return result3(step10("rejected", "resident-page-storage-claim"));
        page.phase = "storage-claimed";
        return result3(step10("pending", "resident-page-storage-claim-observation", 64));
      }
      if (page.phase === "storage-claimed") {
        if (!admitted18(grant, 264))
          return result3(step10("blocked", "resident-page-storage"));
        const owner = storageOwner(state7);
        if (!owner)
          return result3(step10("rejected", "resident-storage-owner"));
        page.phase = "storage-admitting";
        const current = owner.reservePage(length, page.storageCell, grant);
        if (current.step.kind === "blocked")
          page.phase = "storage-claimed";
        return result3(admissionStep(current.step, grant));
      }
      if (page.phase === "storage-admitting") {
        if (!admitted18(grant, 64))
          return result3(step10("blocked", "resident-page-storage-result"));
        if (!pageStorage(page) || page.storageCell.hasFailure || page.storageCell.result?.step.kind !== "ready")
          return result3(step10("rejected", "resident-page-storage-result"));
        slot.phase = "page-binding";
        return result3(step10("pending", "resident-page-storage-result", 64));
      }
      return result3(step10("rejected", "resident-page-storage-phase"));
    }
    if (slot.phase === "page-binding") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-builder-install"));
      slot.phase = "page-builder-installing";
      const current = admissionStep(OwnedUiOperationPayloadBuilder.installPage(builder, page.facade, state7.facade, grant), grant);
      if (current.kind === "blocked" && !OwnedUiOperationPayloadBuilder.matchesPage(builder, page.facade, state7.facade))
        slot.phase = "page-binding";
      return result3(current);
    }
    if (slot.phase === "page-builder-installing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-builder-observation"));
      if (!OwnedUiOperationPayloadBuilder.matchesPage(builder, page.facade, state7.facade))
        return result3(step10("rejected", "resident-page-builder-owner"));
      slot.phase = "page-finalizing";
      return result3(step10("pending", "resident-page-builder-observation", 64));
    }
    if (slot.phase === "page-finalizing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-finalize"));
      Object.freeze(page.facade);
      slot.phase = "finalized";
      return result3(step10("pending", "resident-page-finalize", 64));
    }
    if (slot.phase === "finalized") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-page-publication"));
      page.phase = "live";
      clearBuilderSlot(slot);
      return { step: step10("ready", "resident-page-publication", 64), page: page.facade };
    }
    return result3(step10("rejected", "resident-page-phase"));
  } catch (error) {
    builderFault(slot, error);
    return result3(step10("rejected", "resident-page-admission-fault"));
  }
}
function pageDomainEmpty(page) {
  return (page.phase === "domain-retired" || page.phase === "registration-retired") && !page.payload && !page.facade && !page.previous && !page.next && !page.cell && !page.record && !page.storageCell && !page.witness && page.failure === NO_POOL_FAULT;
}

class PageWitness {
  #original;
  #phase = "constructed";
  constructor(mint, page) {
    if (mint !== MINT11 || !page.facade)
      throw new Error("Invalid page witness");
    this.#original = page.facade;
    page.witness = this;
    Object.freeze(this);
  }
  static {
    createPageWitness = (page) => new PageWitness(MINT11, page);
    markPageWitness = (witness, page) => {
      if (pageState2(witness.#original) !== page || !pageDomainEmpty(page))
        throw new Error("Invalid page terminal proof");
      witness.#phase = "terminal";
    };
  }
  get original() {
    return this.#original;
  }
  get terminal() {
    const page = pageState2(this.#original);
    return this.#phase === "terminal" && page !== null && pageDomainEmpty(page);
  }
}
function closePageSlot(state7, grant, requested = null) {
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-page-close");
  if (state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
    return step10("rejected", "resident-page-parent-fault");
  const slot = state7.pending;
  if (!slot)
    return step10("rejected", "resident-page-parent-slot");
  const ledger = state7.instance.pool.ledger;
  if (!pageSlot(slot)) {
    if (!payloadSlotEmpty(slot))
      return step10("blocked", "resident-page-slot-busy");
    const page = requested ?? state7.head;
    if (!page)
      return step10("complete", "resident-page-empty");
    if (!capturePageSlot(state7.pending, page))
      throw new Error("Invalid page retirement slot");
    state7.pending.entry = page;
    state7.pending.cell = page.cell;
    state7.pending.record = page.record;
    state7.pending.witness = page.witness;
    state7.pending.phase = "closing-domain";
    if (page.phase === "live" || page.phase === "sealed")
      page.phase = "closing";
    return step10("pending", "resident-page-close-capture", 64);
  }
  if (requested && slot.entry && slot.entry !== requested)
    return step10("blocked", "resident-page-other-owner");
  try {
    if (slot.failure !== NO_POOL_FAULT) {
      if (!slot.cell) {
        const cell = ledger.preparedAdmission(slot);
        if (!cell)
          return step10("rejected", "resident-page-fault-held");
        slot.cell = cell;
        return step10("pending", "resident-page-fault-observation", 64);
      }
      if (!slot.cell.hasFailure)
        return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
      return step10("rejected", "resident-page-fault-held");
    }
    if (slot.cell?.hasFailure)
      return step10("rejected", "resident-page-fault-held");
    if (slot.phase === "owner-preparing" || slot.phase === "preparing" || slot.phase === "bootstrap-rejected") {
      const cell = ledger.preparedAdmission(slot);
      if (!cell) {
        if (slot.phase !== "bootstrap-rejected")
          return step10("blocked", "resident-page-admission-handoff");
        clearBuilderSlot(slot);
        return step10("pending", "resident-page-no-cell-observation", 64);
      }
      slot.cell = cell;
      slot.phase = "cell-held";
      return step10("pending", "resident-page-cell-observation", 64);
    }
    if (slot.phase === "record-admitting") {
      slot.record = slot.cell.result?.record ?? null;
      slot.phase = "record-held";
      return step10("pending", "resident-page-record-observation", 64);
    }
    const page = slot.entry;
    if (page) {
      if (state7.reader?.page === page)
        return step10("blocked", "resident-page-reader-alias");
      if (page.failure !== NO_POOL_FAULT) {
        builderFault(slot, page.failure);
        return step10("rejected", "resident-page-body-fault");
      }
      if (!page.facade) {
        if (page.witness || page.storageCell)
          return step10("rejected", "resident-page-unconstructed-body");
        page.payload = null;
        page.record = null;
        page.cell = null;
        page.phase = "domain-retired";
        slot.entry = null;
        slot.phase = "record-held";
        return step10("pending", "resident-page-unused-state", 64);
      }
      if (!page.witness) {
        createPageWitness(page);
        slot.witness = page.witness;
        return step10("pending", "resident-page-close-witness", 32);
      }
      if (slot.phase !== "page-unbound") {
        slot.witness = page.witness;
        if (slot.phase !== "closing-domain") {
          slot.phase = "closing-domain";
          return step10("pending", "resident-page-binding-close", 64);
        }
        const builder = state7.builder?.facade;
        if (!builder)
          return step10("rejected", "resident-page-builder");
        if (OwnedUiOperationPayloadBuilder.matchesPageDetached(builder, page.facade, page.witness, state7.facade)) {
          slot.phase = "page-unbound";
          return step10("pending", "resident-page-builder-detach-observation", 64);
        }
        return childStep4(OwnedUiOperationPayloadBuilder.detachPage(builder, page.facade, page.witness, state7.facade, grant), grant);
      }
      if (page.phase === "storage-preparing" || page.phase === "storage-rejected") {
        const cell = ledger.preparedAdmission(page.facade);
        if (!cell && page.phase !== "storage-rejected")
          return step10("blocked", "resident-page-storage-handoff");
        page.storageCell = cell;
        page.phase = "closing";
        return step10("pending", "resident-page-storage-close-observation", 64);
      }
      if (page.storageCell) {
        if (page.storageCell.hasFailure)
          return step10("rejected", "resident-page-storage-fault-held");
        if (!page.storageCell.terminalIsEmpty()) {
          page.storageCell.beginClose();
          return childStep4(page.storageCell.closeStep(grant), grant);
        }
        page.storageCell = null;
        page.phase = "closing";
        return step10("pending", "resident-page-storage-release-observation", 64);
      }
      slot.witness = page.witness;
      const witness = page.witness;
      if (page.previous)
        page.previous.next = page.next;
      else
        state7.head = page.next;
      if (page.next)
        page.next.previous = page.previous;
      else
        state7.tail = page.previous;
      if (state7.cursor === page)
        state7.cursor = page.next;
      page.payload = null;
      page.facade = null;
      page.previous = null;
      page.next = null;
      page.record = null;
      page.cell = null;
      page.witness = null;
      page.phase = "domain-retired";
      markPageWitness(witness, page);
      slot.entry = null;
      slot.phase = "handoff-observing";
      return step10("pending", "resident-page-domain-unlink", 64);
    }
    if (slot.phase === "handoff-observing") {
      if (!slot.witness?.terminal)
        return step10("rejected", "resident-page-domain-proof");
      slot.record.beginClose();
      slot.phase = "detaching";
      return step10("pending", "resident-page-record-begin", 64);
    }
    if (slot.phase === "detaching") {
      if (OwnedResidentRecordDetachment.matches(slot.record.detachment, slot.record, slot.witness.original)) {
        slot.phase = "record-closing";
        return step10("pending", "resident-page-detachment-observation", 64);
      }
      return childStep4(slot.record.detach(slot.witness.original, grant), grant);
    }
    if (slot.phase === "record-closing") {
      const current = slot.record.closeStep(grant);
      const result3 = childStep4(current, grant);
      if (current.kind === "complete" && result3.kind === "pending")
        slot.phase = "record-observing";
      return result3;
    }
    if (slot.phase === "record-observing") {
      if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
        return step10("rejected", "resident-page-record-proof");
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-page-cell-begin", 64);
    }
    if (slot.phase === "cell-closing") {
      const current = slot.cell.closeStep(grant);
      const result3 = childStep4(current, grant);
      if (current.kind === "complete" && result3.kind === "pending")
        slot.phase = "cell-observing";
      return result3;
    }
    if (slot.phase === "cell-observing") {
      if (!slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
        return step10("rejected", "resident-page-cell-proof");
      if (slot.witness) {
        const original = pageState2(slot.witness.original);
        if (!original || !slot.witness.terminal)
          return step10("rejected", "resident-page-original-proof");
        original.phase = "registration-retired";
      }
      clearBuilderSlot(slot);
      return step10("complete", "resident-page-close", 64);
    }
    if (slot.record) {
      slot.record.beginClose();
      slot.phase = "record-closing";
      return step10("pending", "resident-page-unused-record", 64);
    }
    if (slot.cell) {
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-page-unused-cell", 64);
    }
    return step10("rejected", "resident-page-close-phase");
  } catch (error) {
    builderFault(slot, error);
    return step10("rejected", "resident-page-close-fault");
  }
}

class OwnedUiResidentPage {
  #state;
  constructor(mint, state7) {
    if (mint !== MINT11)
      throw new Error("Invalid resident page authority");
    this.#state = state7;
    state7.facade = this;
    const payload = state7.payload;
    state7.previous = payload.tail;
    if (payload.tail)
      payload.tail.next = state7;
    else
      payload.head = state7;
    payload.tail = state7;
    const current = state7.record.install(this, { maxItems: 1, maxBytes: 64 });
    if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64)
      throw new Error("Resident page record refused installation");
  }
  static {
    pageOwner = (state7) => new OwnedUiResidentPage(MINT11, state7);
    pageState2 = (value) => value !== null && typeof value === "object" && (#state in value) ? value.#state : null;
  }
  allocate(grant) {
    const state7 = this.#state;
    if (!admitted18(grant, 256))
      return step10("blocked", "resident-page-allocate");
    if (state7.phase !== "live" || !state7.payload || !activePayload(state7.payload) || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
      return step10("rejected", "resident-page-allocate");
    try {
      const page = pageStorage(state7);
      return page ? childStep4(page.allocate(grant), grant) : step10("rejected", "resident-page-storage");
    } catch (error) {
      this.#retain(error);
      return step10("rejected", "resident-page-allocation-fault");
    }
  }
  writeByte(value, grant) {
    const state7 = this.#state;
    if (!admitted18(grant, 1))
      return step10("blocked", "resident-page-write");
    if (state7.phase !== "live" || !state7.payload || !activePayload(state7.payload) || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
      return step10("rejected", "resident-page-write");
    try {
      const page = pageStorage(state7);
      return page ? childStep4(page.writeByte(value, grant), grant) : step10("rejected", "resident-page-storage");
    } catch (error) {
      this.#retain(error);
      return step10("rejected", "resident-page-write-fault");
    }
  }
  seal(grant) {
    const state7 = this.#state;
    if (!admitted18(grant, 64))
      return step10("blocked", "resident-page-seal");
    if (state7.phase !== "live" && state7.phase !== "sealed" || !state7.payload || !activePayload(state7.payload) || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
      return step10("rejected", "resident-page-seal");
    try {
      const page = pageStorage(state7);
      if (!page)
        return step10("rejected", "resident-page-storage");
      const result3 = childStep4(page.seal(grant), grant);
      if (result3.kind === "ready")
        state7.phase = "sealed";
      return result3;
    } catch (error) {
      this.#retain(error);
      return step10("rejected", "resident-page-seal-fault");
    }
  }
  beginClose() {
    if (this.#state.phase === "live" || this.#state.phase === "sealed")
      this.#state.phase = "closing";
  }
  terminalIsEmpty() {
    return this.#state.phase === "registration-retired" && pageDomainEmpty(this.#state);
  }
  #retain(error) {
    const state7 = this.#state;
    if (state7.failure !== NO_POOL_FAULT && !Object.is(state7.failure, error))
      throw error;
    state7.failure = error;
  }
}
function readerSlot(slot) {
  return payloadState(slot.requestOwner) !== null;
}
function readerSlotStart(slot, state7) {
  if (!payloadSlotEmpty(slot))
    return false;
  slot.requestOwner = state7.facade;
  return readerSlot(slot);
}
function readerHealthy(state7) {
  return state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && state7.payload !== null && state7.payload.failure === NO_POOL_FAULT && !state7.payload.cell?.hasFailure;
}
function readerPublished(state7) {
  const slot = state7.payload?.pending;
  return state7.phase !== "constructing" && state7.phase !== "alias-rejected" && state7.phase !== "closing" && state7.phase !== "body-retired" && state7.phase !== "domain-retired" && state7.phase !== "registration-retired" && (!slot || !readerSlot(slot));
}
function readerBodyEmpty(state7) {
  return !state7.page && !state7.storageCell && state7.failure === NO_POOL_FAULT && !state7.cell?.hasFailure && (state7.phase === "closing" || state7.phase === "body-retired");
}
function readerDomainEmpty(state7) {
  return !state7.payload && !state7.facade && !state7.cell && !state7.record && !state7.page && !state7.storageCell && !state7.witness && state7.failure === NO_POOL_FAULT && (state7.phase === "domain-retired" || state7.phase === "registration-retired");
}
function readerFault(state7, error) {
  if (state7.failure !== NO_POOL_FAULT && !Object.is(state7.failure, error))
    throw error;
  state7.failure = error;
}
function admitReader(state7, builder, grant) {
  const result3 = (current) => ({ step: current, reader: null });
  const slot = state7.pending;
  if (!admitted18(grant, 32))
    return result3(step10("blocked", "resident-reader-admission"));
  if (!slot || !activePayload(state7) || !OwnedUiResidentPayload.matchesBuilderLive(state7.facade, builder))
    return result3(step10("rejected", "resident-reader-owner"));
  if (slot.requestOwner !== null && (!readerSlot(slot) || slot.requestOwner !== state7.facade))
    return result3(step10("blocked", "resident-reader-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure || state7.reader && !readerHealthy(state7.reader))
    return result3(step10("rejected", "resident-reader-fault-held"));
  if (state7.reader && readerPublished(state7.reader))
    return { step: step10("ready", "resident-reader-held"), reader: state7.reader.facade };
  try {
    const ledger = state7.instance.pool.ledger;
    if (slot.phase === "empty") {
      if (state7.reader || !OwnedUiOperationPayloadBuilder.readerAvailable(builder, state7.facade))
        return result3(step10("rejected", "resident-reader-consumed"));
      if (!admitted18(grant, 296))
        return result3(step10("blocked", "resident-reader-bootstrap"));
      if (!readerSlotStart(slot, state7))
        return result3(step10("blocked", "resident-reader-slot-busy"));
      slot.phase = "preparing";
      const current = ledger.prepareAdmission(slot, "data", grant);
      if (current.kind === "blocked")
        clearBuilderSlot(slot);
      else if (current.kind === "rejected")
        slot.phase = "bootstrap-rejected";
      return result3(admissionStep(current, grant));
    }
    if (!readerSlot(slot))
      return result3(step10("rejected", "resident-reader-slot"));
    if (slot.phase === "preparing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-cell-observation"));
      const cell = ledger.preparedAdmission(slot);
      if (!cell)
        return result3(step10("rejected", "resident-reader-cell-handoff"));
      slot.cell = cell;
      slot.phase = "cell-held";
      return result3(step10("pending", "resident-reader-cell-observation", 64));
    }
    if (slot.phase === "cell-held") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-claim"));
      slot.phase = "claiming";
      const current = ledger.claimAdmission(slot, slot.cell, grant);
      if (current.kind === "blocked")
        slot.phase = "cell-held";
      return result3(admissionStep(current, grant));
    }
    if (slot.phase === "claiming") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-claim-observation"));
      if (!slot.cell?.claimed)
        return result3(step10("rejected", "resident-reader-claim"));
      slot.phase = "claimed";
      return result3(step10("pending", "resident-reader-claim-observation", 64));
    }
    if (slot.phase === "claimed") {
      if (!admitted18(grant, 264))
        return result3(step10("blocked", "resident-reader-record"));
      slot.phase = "record-admitting";
      const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("reader"), slot.cell, grant);
      if (current.step.kind === "blocked")
        slot.phase = "claimed";
      return result3(admissionStep(current.step, grant));
    }
    if (slot.phase === "record-admitting") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-record-observation"));
      slot.record = slot.cell.result?.record ?? null;
      if (!slot.record || slot.cell.hasFailure || slot.cell.result?.step.kind !== "ready")
        return result3(step10("rejected", "resident-reader-record-result"));
      slot.phase = "record-held";
      return result3(step10("pending", "resident-reader-record-observation", 64));
    }
    if (slot.phase === "record-held") {
      if (!admitted18(grant, 104))
        return result3(step10("blocked", "resident-reader-state"));
      const reader2 = { payload: state7, facade: null, cell: slot.cell, record: slot.record, page: null, storageCell: null, offset: 0, phase: "constructing", witness: null, failure: NO_POOL_FAULT, consumed: 0n };
      slot.entry = reader2;
      state7.reader = reader2;
      slot.phase = "reader-state";
      return result3(step10("pending", "resident-reader-state", 104));
    }
    const reader = slot.entry;
    if (!reader)
      return result3(step10("rejected", "resident-reader-entry"));
    if (slot.phase === "reader-state") {
      if (!admitted18(grant, 88))
        return result3(step10("blocked", "resident-reader-shell"));
      createReader2(reader);
      slot.phase = "reader-shell";
      return result3(step10("pending", "resident-reader-shell", 88));
    }
    if (slot.phase === "reader-shell") {
      if (!admitted18(grant, 32))
        return result3(step10("blocked", "resident-reader-witness"));
      createReaderWitness(reader);
      slot.phase = "reader-witness";
      return result3(step10("pending", "resident-reader-witness", 32));
    }
    if (slot.phase === "reader-witness") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-builder-install"));
      slot.phase = "builder-installing";
      return result3(admissionStep(OwnedUiOperationPayloadBuilder.installReader(builder, reader.facade, state7.facade, grant), grant));
    }
    if (slot.phase === "builder-installing") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-builder-observation"));
      if (!OwnedUiOperationPayloadBuilder.matchesReader(builder, reader.facade, state7.facade))
        return result3(step10("rejected", "resident-reader-builder-owner"));
      slot.phase = "builder-installed";
      return result3(step10("pending", "resident-reader-builder-observation", 64));
    }
    if (slot.phase === "builder-installed") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-finalize"));
      Object.freeze(reader.facade);
      slot.phase = "reader-finalized";
      return result3(step10("pending", "resident-reader-finalize", 64));
    }
    if (slot.phase === "reader-finalized") {
      if (!admitted18(grant, 64))
        return result3(step10("blocked", "resident-reader-publication"));
      reader.phase = "live";
      clearBuilderSlot(slot);
      return { step: step10("ready", "resident-reader-publication", 64), reader: reader.facade };
    }
    return result3(step10("rejected", "resident-reader-phase"));
  } catch (error) {
    builderFault(slot, error);
    return result3(step10("rejected", "resident-reader-admission-fault"));
  }
}
function closeReaderAlias(state7, grant) {
  const cell = state7.storageCell;
  if (!cell)
    return step10("complete", "resident-reader-alias-empty");
  if (cell.hasFailure)
    return step10("rejected", "resident-reader-alias-fault-held");
  if (!cell.terminalIsEmpty()) {
    cell.beginClose();
    return childStep4(cell.closeStep(grant), grant);
  }
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-reader-alias-observation");
  state7.storageCell = null;
  return step10("pending", "resident-reader-alias-observation", 64);
}
function advanceReader2(state7, grant) {
  if (!admitted18(grant, 1))
    return step10("blocked", "resident-reader");
  if (!readerHealthy(state7) || !readerPublished(state7) || state7.payload.closing)
    return step10("rejected", "resident-reader");
  const payload = state7.payload;
  const builder = payload.builder?.facade;
  if (!builder)
    return step10("rejected", "resident-reader-builder");
  try {
    const ledger = payload.instance.pool.ledger;
    if (state7.phase === "live") {
      const page = payload.head;
      if (!page)
        return step10(OwnedUiOperationPayloadBuilder.readerEof(builder, payload.facade, state7.consumed) ? "complete" : "blocked", "resident-reader-await-page");
      if (page.phase !== "sealed")
        return step10("blocked", "resident-reader-await-seal");
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-page-capture");
      state7.page = page;
      state7.offset = 0;
      state7.phase = "page-held";
      return step10("pending", "resident-reader-page-capture", 64);
    }
    if (state7.phase === "page-held") {
      if (!admitted18(grant, 296))
        return step10("blocked", "resident-reader-alias-bootstrap");
      state7.phase = "alias-preparing";
      const current = ledger.prepareAdmission(state7.facade, "data", grant);
      if (current.kind === "blocked")
        state7.phase = "page-held";
      else if (current.kind === "rejected")
        state7.phase = "alias-rejected";
      return admissionStep(current, grant);
    }
    if (state7.phase === "alias-preparing") {
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-alias-observation");
      const cell = ledger.preparedAdmission(state7.facade);
      if (!cell)
        return step10("rejected", "resident-reader-alias-handoff");
      state7.storageCell = cell;
      state7.phase = "alias-held";
      return step10("pending", "resident-reader-alias-observation", 64);
    }
    if (state7.phase === "alias-held") {
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-alias-claim");
      state7.phase = "alias-claiming";
      const current = ledger.claimAdmission(state7.facade, state7.storageCell, grant);
      if (current.kind === "blocked")
        state7.phase = "alias-held";
      return admissionStep(current, grant);
    }
    if (state7.phase === "alias-claiming") {
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-alias-claim-observation");
      if (!state7.storageCell?.claimed)
        return step10("rejected", "resident-reader-alias-claim");
      state7.phase = "alias-claimed";
      return step10("pending", "resident-reader-alias-claim-observation", 64);
    }
    if (state7.phase === "alias-claimed") {
      if (!admitted18(grant, 136))
        return step10("blocked", "resident-reader-alias-admission");
      const owner = storageOwner(payload);
      const page = state7.page && pageStorage(state7.page);
      if (!owner || !page)
        return step10("rejected", "resident-reader-alias-source");
      state7.phase = "alias-admitting";
      const current = owner.beginRead(page, state7.storageCell, grant);
      if (current.step.kind === "blocked")
        state7.phase = "alias-claimed";
      return admissionStep(current.step, grant);
    }
    if (state7.phase === "alias-admitting") {
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-alias-result");
      if (!state7.storageCell?.result?.reader || state7.storageCell.hasFailure || state7.storageCell.result.step.kind !== "ready")
        return step10("rejected", "resident-reader-alias-result");
      state7.phase = "reading";
      return step10("pending", "resident-reader-alias-result", 64);
    }
    if (state7.phase === "reading") {
      if (state7.offset === state7.page.length) {
        if (!admitted18(grant, 64))
          return step10("blocked", "resident-reader-page-end");
        state7.phase = "alias-closing";
        return step10("pending", "resident-reader-page-end", 64);
      }
      const value = state7.storageCell.result.reader.byteAt(state7.offset);
      state7.offset++;
      state7.consumed++;
      return { kind: "byte", value, items: 1, bytes: 1 };
    }
    if (state7.phase === "alias-closing") {
      if (state7.storageCell)
        return closeReaderAlias(state7, grant);
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-page-detach");
      state7.page = null;
      state7.phase = "page-retiring";
      return step10("pending", "resident-reader-page-detach", 64);
    }
    if (state7.phase === "page-retiring") {
      const current = closePageSlot(payload, grant);
      const forwarded = childStep4(current, grant);
      if (current.kind === "complete" && forwarded.kind === "pending")
        state7.phase = "page-observing";
      return forwarded;
    }
    if (state7.phase === "page-observing") {
      if (!admitted18(grant, 64))
        return step10("blocked", "resident-reader-page-observation");
      if (payload.head || !payload.pending || !payloadSlotEmpty(payload.pending))
        return step10("rejected", "resident-reader-page-observation");
      state7.phase = "live";
      return step10("pending", "resident-reader-page-observation", 64);
    }
    return step10("rejected", "resident-reader-phase");
  } catch (error) {
    readerFault(state7, error);
    return step10("rejected", "resident-reader-fault");
  }
}

class OwnedUiResidentPayloadReader {
  #state;
  constructor(mint, state7) {
    if (mint !== MINT11)
      throw new Error("Invalid resident reader authority");
    this.#state = state7;
    state7.facade = this;
    const current = state7.record.install(this, { maxItems: 1, maxBytes: 64 });
    if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64)
      throw new Error("Reader record installation refused");
  }
  static {
    createReader2 = (state7) => new OwnedUiResidentPayloadReader(MINT11, state7);
    readerState = (value) => value !== null && typeof value === "object" && (#state in value) ? value.#state : null;
  }
  advance(grant) {
    return advanceReader2(this.#state, grant);
  }
  terminalIsEmpty() {
    return this.#state.phase === "registration-retired" && readerDomainEmpty(this.#state);
  }
}

class OwnedUiResidentReaderRetirement {
  #original;
  #phase = "constructed";
  constructor(mint, state7) {
    if (mint !== MINT11 || !state7.facade)
      throw new Error("Invalid reader witness");
    this.#original = state7.facade;
    state7.witness = this;
    Object.freeze(this);
  }
  static {
    createReaderWitness = (state7) => new OwnedUiResidentReaderRetirement(MINT11, state7);
    moveReaderWitness = (witness, phase) => {
      witness.#phase = phase;
    };
  }
  static matchesBody(proof, reader, payload) {
    return proof !== null && typeof proof === "object" && #original in proof && proof.#original === reader && proof.#phase === "body-retired" && OwnedUiResidentPayload.matchesReaderBinding(payload, reader, proof) && readerBodyEmpty(readerState(reader));
  }
  static matchesDetached(proof, reader, payload) {
    return proof !== null && typeof proof === "object" && #original in proof && proof.#original === reader && proof.#phase === "detached" && OwnedUiResidentPayload.matchesReaderBinding(payload, reader, proof);
  }
  get original() {
    return this.#original;
  }
  get terminal() {
    const state7 = readerState(this.#original);
    return this.#phase === "terminal" && state7 !== null && readerDomainEmpty(state7);
  }
}
function closeReaderSlot(state7, grant, requested = null) {
  if (!admitted18(grant, 64))
    return step10("blocked", "resident-reader-close");
  const slot = state7.pending;
  if (!slot || state7.failure !== NO_POOL_FAULT || state7.cell?.hasFailure)
    return step10("rejected", "resident-reader-parent");
  const ledger = state7.instance.pool.ledger;
  if (!readerSlot(slot)) {
    const reader = requested ?? state7.reader;
    if (!reader || !readerSlotStart(state7.pending, state7))
      return step10("blocked", "resident-reader-slot-busy");
    const held = state7.pending;
    held.entry = reader;
    held.cell = reader.cell;
    held.record = reader.record;
    held.witness = reader.witness;
    held.phase = "closing-domain";
    return step10("pending", "resident-reader-close-capture", 64);
  }
  if (requested && slot.entry && requested !== slot.entry)
    return step10("blocked", "resident-reader-other-owner");
  try {
    if (slot.failure !== NO_POOL_FAULT) {
      if (!slot.cell) {
        const cell = ledger.preparedAdmission(slot);
        if (!cell)
          return step10("rejected", "resident-reader-fault-held");
        slot.cell = cell;
        return step10("pending", "resident-reader-fault-observation", 64);
      }
      if (!slot.cell.hasFailure)
        return childStep4(slot.cell.retainFailure(slot.failure, grant), grant);
      return step10("rejected", "resident-reader-fault-held");
    }
    if (slot.cell?.hasFailure)
      return step10("rejected", "resident-reader-fault-held");
    if (slot.phase === "preparing" || slot.phase === "bootstrap-rejected") {
      const cell = ledger.preparedAdmission(slot);
      if (!cell) {
        if (slot.phase !== "bootstrap-rejected")
          return step10("blocked", "resident-reader-cell-handoff");
        clearBuilderSlot(slot);
        return step10("pending", "resident-reader-no-cell-observation", 64);
      }
      slot.cell = cell;
      slot.phase = "cell-held";
      return step10("pending", "resident-reader-cell-observation", 64);
    }
    if (slot.phase === "record-admitting") {
      slot.record = slot.cell.result?.record ?? null;
      slot.phase = "record-held";
      return step10("pending", "resident-reader-record-observation", 64);
    }
    const reader = slot.entry;
    if (reader) {
      if (reader.failure !== NO_POOL_FAULT) {
        builderFault(slot, reader.failure);
        return step10("rejected", "resident-reader-body-fault");
      }
      if (!reader.facade) {
        reader.payload = null;
        reader.cell = null;
        reader.record = null;
        reader.phase = "domain-retired";
        state7.reader = null;
        slot.entry = null;
        slot.phase = "record-held";
        return step10("pending", "resident-reader-unused-state", 64);
      }
      if (reader.phase === "alias-preparing" || reader.phase === "alias-rejected") {
        const cell = ledger.preparedAdmission(reader.facade);
        if (!cell && reader.phase !== "alias-rejected")
          return step10("blocked", "resident-reader-alias-handoff");
        reader.storageCell = cell;
        reader.phase = "closing";
        return step10("pending", "resident-reader-alias-close-observation", 64);
      }
      if (reader.storageCell)
        return closeReaderAlias(reader, grant);
      if (reader.page) {
        reader.page = null;
        reader.phase = "closing";
        return step10("pending", "resident-reader-page-detach", 64);
      }
      if (!reader.witness) {
        createReaderWitness(reader);
        slot.witness = reader.witness;
        return step10("pending", "resident-reader-close-witness", 32);
      }
      slot.witness = reader.witness;
      const builder = state7.builder?.facade;
      if (!builder)
        return step10("rejected", "resident-reader-builder");
      if (slot.phase === "binding-detaching") {
        if (OwnedUiOperationPayloadBuilder.matchesReaderDetached(builder, reader.facade, reader.witness, state7.facade)) {
          moveReaderWitness(reader.witness, "detached");
          slot.phase = "binding-settling";
          return step10("pending", "resident-reader-binding-detach-observation", 64);
        }
        return childStep4(OwnedUiOperationPayloadBuilder.detachReader(builder, reader.facade, reader.witness, state7.facade, grant), grant);
      }
      if (slot.phase === "binding-settling") {
        if (OwnedUiOperationPayloadBuilder.matchesReaderSettled(builder, reader.facade, reader.witness, state7.facade)) {
          moveReaderWitness(reader.witness, "settled");
          slot.phase = "binding-settled";
          return step10("pending", "resident-reader-binding-settle-observation", 64);
        }
        return childStep4(OwnedUiOperationPayloadBuilder.settleReader(builder, reader.facade, reader.witness, state7.facade, grant), grant);
      }
      if (slot.phase !== "binding-settled") {
        reader.phase = "body-retired";
        moveReaderWitness(reader.witness, "body-retired");
        slot.phase = "binding-detaching";
        return step10("pending", "resident-reader-body-proof", 64);
      }
      const witness = reader.witness;
      reader.payload = null;
      reader.facade = null;
      reader.cell = null;
      reader.record = null;
      reader.witness = null;
      reader.phase = "domain-retired";
      state7.reader = null;
      slot.entry = null;
      moveReaderWitness(witness, "terminal");
      slot.phase = "handoff-observing";
      return step10("pending", "resident-reader-domain-unlink", 64);
    }
    if (slot.phase === "handoff-observing") {
      if (!slot.witness?.terminal)
        return step10("rejected", "resident-reader-domain-proof");
      slot.record.beginClose();
      slot.phase = "detaching";
      return step10("pending", "resident-reader-record-begin", 64);
    }
    if (slot.phase === "detaching") {
      if (OwnedResidentRecordDetachment.matches(slot.record.detachment, slot.record, slot.witness.original)) {
        slot.phase = "record-closing";
        return step10("pending", "resident-reader-detach-observation", 64);
      }
      return childStep4(slot.record.detach(slot.witness.original, grant), grant);
    }
    if (slot.phase === "record-closing") {
      const current = slot.record.closeStep(grant);
      const result3 = childStep4(current, grant);
      if (current.kind === "complete" && result3.kind === "pending")
        slot.phase = "record-observing";
      return result3;
    }
    if (slot.phase === "record-observing") {
      if (!OwnedResidentRetirement.matches(slot.record.retirement, slot.record))
        return step10("rejected", "resident-reader-record-proof");
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-reader-cell-begin", 64);
    }
    if (slot.phase === "cell-closing") {
      const current = slot.cell.closeStep(grant);
      const result3 = childStep4(current, grant);
      if (current.kind === "complete" && result3.kind === "pending")
        slot.phase = "cell-observing";
      return result3;
    }
    if (slot.phase === "cell-observing") {
      if (!slot.cell.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty())
        return step10("rejected", "resident-reader-cell-proof");
      if (slot.witness) {
        if (!slot.witness.terminal)
          return step10("rejected", "resident-reader-original-proof");
        readerState(slot.witness.original).phase = "registration-retired";
      }
      clearBuilderSlot(slot);
      return step10("complete", "resident-reader-close", 64);
    }
    if (slot.record) {
      slot.record.beginClose();
      slot.phase = "record-closing";
      return step10("pending", "resident-reader-unused-record", 64);
    }
    if (slot.cell) {
      slot.cell.beginClose();
      slot.phase = "cell-closing";
      return step10("pending", "resident-reader-unused-cell", 64);
    }
    return step10("rejected", "resident-reader-close-phase");
  } catch (error) {
    builderFault(slot, error);
    return step10("rejected", "resident-reader-close-fault");
  }
}

/* ../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts */
var MINT12 = Object.freeze({});
var admitted19 = (grant) => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
var step11 = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function closeChild(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function surfaceName(value) {
  if (typeof value !== "string" || value.length > 512 || new TextEncoder().encode(value).length > 512)
    throw new Error("Native surface name exceeds UiText capacity");
  return value;
}
function generation(value) {
  if (typeof value !== "bigint" || value <= 0n || value > 0xffffffffffffffffn)
    throw new Error("Invalid native instance generation");
  return value;
}
var cellOf2;
var createFacade;
var createPatch2;
var createLookup;
var closeLookup;
var appendSurface;
var operationAuthority;
var enqueue;
var createAcknowledgement;
var createRetirement;
var createInputRetirement;
var createInputAcceptance;
function live2(cell, operation = false) {
  if (!cell.owner || !cell.surface)
    throw new Error("Owned UI instance surface is retired");
  if (operation)
    operationAuthority(cell.owner);
  return cell.surface;
}
function prepareInputRetirement(cell, wire) {
  if (cell.page || !cell.inputActive || !cell.source)
    return cell.page;
  const receipt = wire.takePageReceipt();
  if (!receipt && !wire.terminalIsEmpty())
    return null;
  if (receipt && receipt.ordinal !== cell.ordinal)
    throw new Error("Native input retirement ordinal mismatch");
  cell.page = createInputRetirement(cell.source, cell.ordinal, cell.original);
  return cell.page;
}

class OwnedUiPatchInputAcceptance {
  #source;
  #ordinal;
  #original;
  constructor(mint, source, ordinal, original) {
    if (mint !== MINT12)
      throw new Error("Invalid UI patch input acceptance authority");
    this.#source = source;
    this.#ordinal = ordinal;
    this.#original = original;
    Object.freeze(this);
  }
  static {
    createInputAcceptance = (source, ordinal, original) => new OwnedUiPatchInputAcceptance(MINT12, source, ordinal, original);
  }
  static matches(claim, source, ordinal, original) {
    return claim !== null && typeof claim === "object" && #source in claim && claim.#source === source && claim.#ordinal === ordinal && claim.#original === original;
  }
}

class OwnedUiPatchInputRetirement {
  #source;
  #ordinal;
  #original;
  constructor(mint, source, ordinal, original) {
    if (mint !== MINT12)
      throw new Error("Invalid UI patch input retirement authority");
    this.#source = source;
    this.#ordinal = ordinal;
    this.#original = original;
    Object.freeze(this);
  }
  static {
    createInputRetirement = (source, ordinal, original) => new OwnedUiPatchInputRetirement(MINT12, source, ordinal, original);
  }
  static matches(token, source, ordinal, original) {
    return token !== null && typeof token === "object" && #source in token && token.#source === source && token.#ordinal === ordinal && token.#original === original;
  }
  get ordinal() {
    return this.#ordinal;
  }
}

class OwnedUiPatchAcknowledgement {
  #owner;
  #source;
  #value;
  constructor(mint, owner, source, lifetime, value) {
    if (mint !== MINT12)
      throw new Error("Invalid UI patch acknowledgement authority");
    const producer = source.value.receipt;
    if (producer.lifetime.activationGeneration !== lifetime.activationGeneration || producer.lifetime.instanceId !== lifetime.instanceId || producer.lifetime.guestLifetime !== lifetime.guestLifetime)
      throw new Error("Native UI receipt lifetime mismatch");
    const identity = Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime });
    this.#owner = owner;
    this.#source = source;
    this.#value = Object.freeze({ actor: value.actor, instance: value.instance, surface: value.surface, revision: value.revision, hash: value.hash, lifetime: identity, receipt: Object.freeze({ lifetime: identity, patchSequence: producer.patchSequence }) });
    Object.freeze(this);
  }
  static {
    createAcknowledgement = (owner, source, lifetime, value) => new OwnedUiPatchAcknowledgement(MINT12, owner, source, lifetime, value);
  }
  static matches(token, source) {
    return token !== null && typeof token === "object" && #source in token && token.#source === source;
  }
  get owner() {
    return this.#owner;
  }
  get value() {
    return this.#value;
  }
}

class OwnedUiInstanceRetirement {
  #owner;
  #activation;
  #lifetime;
  constructor(mint, owner, activation, lifetime) {
    if (mint !== MINT12)
      throw new Error("Invalid UI instance retirement authority");
    this.#owner = owner;
    this.#activation = activation;
    this.#lifetime = lifetime;
    Object.freeze(this);
  }
  static {
    createRetirement = (owner, activation, lifetime) => new OwnedUiInstanceRetirement(MINT12, owner, activation, lifetime);
  }
  static matches(witness, owner, activation, lifetime) {
    return witness !== null && typeof witness === "object" && #owner in witness && witness.#owner === owner && witness.#activation === activation && witness.#lifetime.activationGeneration === lifetime.activationGeneration && witness.#lifetime.instanceId === lifetime.instanceId && witness.#lifetime.guestLifetime === lifetime.guestLifetime;
  }
}

class OwnedUiInstanceSurface {
  #cell;
  constructor(mint, cell) {
    if (mint !== MINT12)
      throw new Error("Invalid instance surface authority");
    this.#cell = cell;
    Object.freeze(this);
  }
  static {
    cellOf2 = (value) => value.#cell;
    createFacade = (cell) => new OwnedUiInstanceSurface(MINT12, cell);
  }
  get view() {
    return live2(this.#cell).view;
  }
  subscribeView(notify) {
    const result3 = live2(this.#cell, true).subscribeView(notify);
    enqueue(this.#cell.owner, this.#cell);
    return result3;
  }
  subscribeNode(id2, notify) {
    const result3 = live2(this.#cell, true).subscribeNode(id2, notify);
    enqueue(this.#cell.owner, this.#cell);
    return result3;
  }
  retryNotification(subscription) {
    const result3 = live2(this.#cell, true).retryNotification(subscription);
    if (result3)
      enqueue(this.#cell.owner, this.#cell);
    return result3;
  }
  acknowledgeRead(subscription, snapshot) {
    if (!this.#cell.owner)
      return;
    live2(this.#cell).acknowledgeRead(subscription, snapshot);
    enqueue(this.#cell.owner, this.#cell);
  }
  unsubscribeNode(subscription) {
    if (!this.#cell.owner)
      return;
    live2(this.#cell).unsubscribeNode(subscription);
    enqueue(this.#cell.owner, this.#cell);
  }
  retireSceneRead(subscription, reader) {
    if (!this.#cell.owner)
      return false;
    const result3 = live2(this.#cell).retireSceneRead(subscription, reader);
    enqueue(this.#cell.owner, this.#cell);
    return result3;
  }
  openSceneRecord(subscription, snapshot, source) {
    const result3 = live2(this.#cell, true).openSceneRecord(subscription, snapshot, source);
    if (!result3)
      return null;
    return Object.freeze({ advance: (grant) => result3.advance(grant), close: () => {
      const closed = result3.close();
      if (this.#cell.owner)
        enqueue(this.#cell.owner, this.#cell);
      return closed;
    } });
  }
  openSceneText(subscription, snapshot, source) {
    const result3 = live2(this.#cell, true).openSceneText(subscription, snapshot, source);
    if (!result3)
      return null;
    return Object.freeze({ advance: (grant) => result3.advance(grant), close: () => {
      const closed = result3.close();
      if (this.#cell.owner)
        enqueue(this.#cell.owner, this.#cell);
      return closed;
    } });
  }
}

class OwnedUiSurfaceLookup {
  #owner;
  #name;
  #cell;
  #result = null;
  #ready = false;
  #closing = false;
  #failure = null;
  constructor(mint, owner, name, first) {
    if (mint !== MINT12)
      throw new Error("Invalid instance lookup authority");
    this.#owner = owner;
    this.#name = name;
    this.#cell = first;
    Object.freeze(this);
  }
  static {
    createLookup = (owner, name, first) => new OwnedUiSurfaceLookup(MINT12, owner, name, first);
  }
  get failure() {
    return this.#failure;
  }
  advance(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-surface-lookup");
    if (this.#closing || this.#failure)
      return step11("rejected", "instance-surface-lookup");
    if (this.#ready)
      return step11("ready", "instance-surface-lookup");
    try {
      operationAuthority(this.#owner);
      if (this.#cell) {
        if (this.#cell.name === this.#name) {
          this.#result = this.#cell.facade;
          this.#ready = true;
        } else
          this.#cell = this.#cell.next;
        return step11(this.#ready ? "ready" : "pending", "instance-surface-lookup", 2112);
      }
      this.#result = appendSurface(this.#owner, this.#name).facade;
      this.#ready = true;
      return step11("ready", "instance-surface-create", 1024);
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "Instance lookup failed";
      return step11("rejected", "instance-surface-lookup", 128);
    }
  }
  takeResult() {
    if (!this.#ready || this.#closing || this.#failure)
      return null;
    const result3 = this.#result;
    this.#result = null;
    return result3;
  }
  beginClose() {
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-lookup-close");
    if (!this.#closing)
      throw new Error("Instance lookup close has not begun");
    if (this.#owner) {
      closeLookup(this.#owner, this);
      this.#owner = null;
      this.#cell = null;
      this.#result = null;
      this.#name = "";
      return step11("pending", "instance-lookup-close", 1152);
    }
    return step11("complete", "instance-lookup-close");
  }
  terminalIsEmpty() {
    return this.#closing && !this.#owner && !this.#cell && !this.#result && this.#name.length === 0;
  }
}

class OwnedUiInstancePatch {
  #cell;
  #closing = false;
  constructor(mint, cell) {
    if (mint !== MINT12)
      throw new Error("Invalid instance patch authority");
    this.#cell = cell;
    Object.freeze(this);
  }
  static {
    createPatch2 = (cell) => new OwnedUiInstancePatch(MINT12, cell);
  }
  #wire() {
    if (!this.#cell.owner || !this.#cell.wire || this.#cell.patch !== this)
      throw new Error("Instance patch owner is retired");
    return this.#cell.wire;
  }
  get failure() {
    return this.#wire().failure;
  }
  offer(ordinal) {
    const wire = this.#wire();
    operationAuthority(this.#cell.owner);
    const source = this.#cell.source;
    if (this.#closing || !source || this.#cell.inputActive || this.#cell.page || this.#cell.ack || ordinal !== this.#cell.ordinal || ordinal >= source.value.operationCount)
      return false;
    const original = source.operation(ordinal);
    if (!wire.offer(ordinal, original))
      return false;
    this.#cell.inputActive = true;
    this.#cell.original = original;
    if (!source.acceptInput(createInputAcceptance(source, ordinal, original)))
      throw new Error("Native UI input acceptance claim was refused");
    return true;
  }
  advance(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-patch");
    operationAuthority(this.#cell.owner);
    const wire = this.#wire();
    if (this.#cell.page)
      return step11("ready", "instance-input-retirement");
    if (prepareInputRetirement(this.#cell, wire))
      return step11("ready", "instance-input-retirement", 128);
    const current = wire.advance(grant);
    enqueue(this.#cell.owner, this.#cell);
    return this.#cell.inputActive && current.kind === "ready" ? { ...current, kind: "pending" } : current;
  }
  finishInput() {
    operationAuthority(this.#cell.owner);
    if (this.#cell.page)
      throw new Error("Instance still owns the input release receipt");
    this.#wire().finishInput();
  }
  peekInputReceipt() {
    this.#wire();
    return this.#cell.page;
  }
  releaseInputReceipt(receipt) {
    this.#wire();
    if (this.#cell.page !== receipt || !this.#cell.source || !this.#cell.source.releaseInput(receipt))
      return false;
    this.#cell.page = null;
    this.#cell.original = null;
    this.#cell.inputActive = false;
    this.#cell.ordinal++;
    return true;
  }
  peekAcknowledgement() {
    const wire = this.#wire();
    if (this.#cell.ack)
      return this.#cell.ack;
    const value = wire.takeAcknowledgement();
    if (!value)
      return null;
    const source = this.#cell.source;
    if (!source || value.actor !== source.value.activation.actorId || value.instance !== source.value.lifetime.instanceId || value.surface !== source.value.surface || value.revision !== source.value.revision)
      throw new Error("Native UI publication acknowledgement mismatch");
    this.#cell.ack = createAcknowledgement(this.#cell.owner, source, source.value.lifetime, value);
    return this.#cell.ack;
  }
  acceptAcknowledgement(receipt) {
    this.#wire();
    if (!this.#cell.source || !this.#cell.ack || !OwnedNativeUiPatchSubmissionReceipt.matches(receipt, this.#cell.source, this.#cell.ack))
      return false;
    this.#cell.ack = null;
    return true;
  }
  beginClose() {
    this.#wire().beginClose();
    this.#closing = true;
  }
  closeStep(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-patch-close");
    const wire = this.#wire();
    if (this.#cell.page)
      return step11("blocked", "instance-input-retirement");
    if (this.#cell.ack)
      return step11("blocked", "instance-receipt-outbox");
    if (wire.terminalIsEmpty())
      return prepareInputRetirement(this.#cell, wire) ? step11("pending", "instance-input-retirement", 128) : step11("complete", "instance-patch-close");
    return closeChild(wire.closeStep(grant), grant);
  }
  terminalIsEmpty() {
    return this.#cell.patch !== this || !this.#cell.inputActive && !this.#cell.page && !this.#cell.ack && this.#wire().terminalIsEmpty();
  }
}

class OwnedUiInstance {
  #activation;
  #lifetime;
  #limits;
  #profile;
  #head = null;
  #tail = null;
  #work = null;
  #workTail = null;
  #maintenanceWorked = false;
  #maintenanceFailure = null;
  #lookup = null;
  #closing = false;
  #closed = false;
  #retirement = null;
  #resident = null;
  constructor(activation, lifetime, limits, profile) {
    activation.assertActive();
    const activationGeneration = generation(lifetime.activationGeneration);
    const guestLifetime = generation(lifetime.guestLifetime);
    const instanceId = lifetime.instanceId;
    if (activation.activationGeneration !== activationGeneration || !Number.isInteger(instanceId) || instanceId < 0 || instanceId > 4294967295)
      throw new Error("Instance capture does not match native lifetime");
    this.#activation = activation;
    this.#lifetime = Object.freeze({ activationGeneration, instanceId, guestLifetime });
    this.#limits = Object.freeze({ maxNodes: limits.maxNodes, maxDepth: limits.maxDepth, maxChildren: limits.maxChildren, maxTextBytes: limits.maxTextBytes, maxPatchOps: limits.maxPatchOps, maxPatchBytes: limits.maxPatchBytes });
    this.#profile = Object.freeze({ usizeBits: profile.usizeBits });
    Object.freeze(this);
  }
  static {
    operationAuthority = (owner) => {
      if (!owner || owner.#closing || owner.#closed || !owner.#activation)
        throw new Error("Instance operation owner is closing");
      owner.#activation.assertActive();
    };
    closeLookup = (owner, lookup) => {
      if (owner.#lookup !== lookup)
        throw new Error("Foreign instance lookup close");
      owner.#lookup = null;
    };
    appendSurface = (owner, name) => {
      operationAuthority(owner);
      const surface = new OwnedUiSurface({ actor: owner.#activation.actorId, instance: owner.#lifetime.instanceId, surface: name }, owner.#limits, owner.#profile);
      const cell = { owner, name, surface, facade: null, wire: null, patch: null, source: null, ordinal: 0, inputActive: false, original: null, page: null, ack: null, next: null, workNext: null, queued: false };
      cell.facade = createFacade(cell);
      if (owner.#tail)
        owner.#tail.next = cell;
      else
        owner.#head = cell;
      owner.#tail = cell;
      return cell;
    };
    enqueue = (owner, cell) => {
      if (owner.#closing || cell.queued || !cell.surface?.maintenancePending)
        return;
      cell.queued = true;
      if (owner.#workTail)
        owner.#workTail.workNext = cell;
      else
        owner.#work = cell;
      owner.#workTail = cell;
    };
  }
  #matches(activation, lifetime) {
    return activation === this.#activation && lifetime.activationGeneration === this.#lifetime.activationGeneration && lifetime.instanceId === this.#lifetime.instanceId && lifetime.guestLifetime === this.#lifetime.guestLifetime;
  }
  static matches(owner, activation, lifetime) {
    return owner !== null && typeof owner === "object" && #activation in owner && owner.#matches(activation, lifetime);
  }
  attachResidentScope(scope) {
    if (this.#closing || this.#closed || !this.#activation || this.#resident !== null && this.#resident !== scope || !OwnedUiResidentInstance.matches(scope, this, this.#activation, this.#lifetime))
      return false;
    this.#resident = scope;
    return true;
  }
  beginSurfaceLookup(activation, lifetime, name) {
    if (!this.#matches(activation, lifetime))
      return null;
    operationAuthority(this);
    if (this.#lookup)
      return null;
    const lookup = createLookup(this, surfaceName(name), this.#head);
    this.#lookup = lookup;
    return lookup;
  }
  beginPatch(source, facade) {
    operationAuthority(this);
    if (!OwnedNativeUiPatchAuthority.matches(source, this.#activation, this.#lifetime) || !OwnedNativeUiPatchAuthority.matchesOwner(source, this))
      throw new Error("Foreign native instance patch owner");
    const cell = cellOf2(facade);
    const value = source.value;
    if (cell.owner !== this || cell.name !== value.surface || !cell.surface || cell.wire && !cell.wire.terminalIsEmpty() || cell.page || cell.ack)
      throw new Error("Foreign or busy instance surface owner");
    const wire = new OwnedUiWirePatchCursor(cell.surface, value.baseRevision, value.revision, value.operationCount);
    cell.wire = wire;
    cell.source = source;
    cell.ordinal = 0;
    cell.inputActive = false;
    cell.patch = createPatch2(cell);
    return cell.patch;
  }
  get maintenancePending() {
    return this.#work !== null;
  }
  get maintenanceFailure() {
    return this.#maintenanceFailure;
  }
  advanceMaintenance(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-maintenance");
    if (!this.#work)
      return step11("complete", "instance-maintenance");
    const cell = this.#work;
    if (this.#maintenanceWorked) {
      this.#work = cell.workNext;
      if (!this.#work)
        this.#workTail = null;
      cell.workNext = null;
      cell.queued = false;
      this.#maintenanceWorked = false;
      enqueue(this, cell);
      return step11("pending", "instance-maintenance-queue", 64);
    }
    try {
      const current = live2(cell).advanceMaintenance(grant);
      if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) {
        this.#maintenanceFailure = "Instance maintenance child exceeded its grant";
        return { ...current, kind: "rejected" };
      }
      if (current.kind === "blocked" || current.kind === "rejected") {
        if (current.kind === "rejected")
          this.#maintenanceFailure = current.phase;
        return current;
      }
      this.#maintenanceFailure = null;
      this.#maintenanceWorked = true;
      return { ...current, kind: "pending" };
    } catch (error) {
      this.#maintenanceFailure = error instanceof Error ? error.message : "Instance maintenance failed";
      return step11("rejected", "instance-maintenance-failed");
    }
  }
  beginClose() {
    if (this.#closing)
      return;
    this.#closing = true;
    this.#maintenanceWorked = false;
    this.#resident?.beginClose();
    this.#lookup?.beginClose();
  }
  closeStep(grant) {
    if (!admitted19(grant))
      return step11("blocked", "instance-close");
    if (!this.#closing)
      throw new Error("Instance close has not begun");
    if (this.#closed)
      return step11("complete", "instance-close");
    if (this.#lookup)
      return closeChild(this.#lookup.closeStep(grant), grant);
    if (this.#work) {
      const cell2 = this.#work;
      this.#work = cell2.workNext;
      if (!this.#work)
        this.#workTail = null;
      cell2.workNext = null;
      cell2.queued = false;
      return step11("pending", "instance-work-release", 64);
    }
    const cell = this.#head;
    if (cell) {
      if (cell.page)
        return step11("blocked", "instance-input-retirement");
      if (cell.ack)
        return step11("blocked", "instance-receipt-outbox");
      if (cell.wire) {
        if (cell.wire.terminalIsEmpty()) {
          if (prepareInputRetirement(cell, cell.wire))
            return step11("pending", "instance-input-retirement", 128);
          cell.wire = null;
          cell.patch = null;
          cell.source = null;
          return step11("pending", "instance-wire-release", 128);
        }
        cell.wire.beginClose();
        return closeChild(cell.wire.closeStep(grant), grant);
      }
      if (cell.surface.terminalIsEmpty()) {
        this.#head = cell.next;
        if (!this.#head)
          this.#tail = null;
        cell.next = null;
        cell.surface = null;
        cell.facade = null;
        cell.owner = null;
        cell.name = "";
        return step11("pending", "instance-surface-release", 1152);
      }
      cell.surface.beginClose();
      return closeChild(cell.surface.closeStep(grant), grant);
    }
    if (this.#resident) {
      if (this.#resident.terminalIsEmpty()) {
        this.#resident = null;
        return step11("pending", "instance-resident-release", 64);
      }
      return closeChild(this.#resident.closeStep(grant), grant);
    }
    this.#retirement = createRetirement(this, this.#activation, this.#lifetime);
    this.#activation = null;
    this.#closed = true;
    return step11("complete", "instance-close", 128);
  }
  takeRetirementWitness() {
    if (!this.#closed)
      return null;
    const witness = this.#retirement;
    this.#retirement = null;
    return witness;
  }
  terminalIsEmpty() {
    return this.#closed && !this.#activation && !this.#head && !this.#tail && !this.#work && !this.#workTail && !this.#lookup && !this.#maintenanceWorked && !this.#resident;
  }
}

/* ../../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts */
var residentCapacityGetter;
var residentCapacity = () => residentCapacityGetter ??= Object.getOwnPropertyDescriptor(OwnedResidentLedger.prototype, "capacity").get;
var NO_RESIDENT_FAULT = Symbol("actor-resident.no-fault");
var poolUiEnvelope = uiResidentMetadataEnvelope("pool");
var poolRecordEnvelope = poolUiEnvelope;
var poolControllerEnvelope = Object.freeze({ bytes: 224, slots: 1, owners: 1 });
var workerControllerEnvelope = Object.freeze({ bytes: 128, slots: 0, owners: 0 });
var residentStep = (kind, phase, bytes = 0) => ({ kind, phase, items: bytes ? 1 : 0, bytes });
var residentGrant = (grant, bytes) => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
function residentChild(current, grant) {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes)
    return residentStep("rejected", "actor-resident.child-grant");
  return current.kind === "complete" || current.kind === "ready" ? { ...current, kind: "pending" } : current;
}
var SHARD_COMMAND_MAXIMUM_PAGES = 64;
function createShardCommandIngressPages(input) {
  if (input.command.length === 0)
    throw new Error("[DEBUG] command ingress cannot encode an empty command");
  const pageCount = Math.ceil(input.command.length / ACTOR_BYTE_PAGE_BYTES);
  if (pageCount > SHARD_COMMAND_MAXIMUM_PAGES)
    throw new Error(`[DEBUG] command ingress exceeds ${SHARD_COMMAND_MAXIMUM_PAGES} pages`);
  const pages = [];
  for (let pageIndex = 0;pageIndex < pageCount; pageIndex += 1) {
    const start = pageIndex * ACTOR_BYTE_PAGE_BYTES;
    const bytes = input.command.subarray(start, Math.min(start + ACTOR_BYTE_PAGE_BYTES, input.command.length));
    pages.push({
      cursor: {
        owner: input.owner,
        generation: input.generation,
        commandIndex: input.commandIndex,
        commandCount: input.commandCount,
        instance: input.instance,
        seq: input.seq,
        kind: input.command[0],
        pageIndex,
        pageCount,
        itemCount: 0,
        metadata: 0
      },
      page: createActorBytePage(bytes)
    });
  }
  return pages;
}
var MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
var SHARD_FRAME_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function orderEnvelopesByLane(envelopes) {
  return envelopes.map((envelope, index) => ({ envelope, index })).sort((left, right) => {
    const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
    return rank !== 0 ? rank : left.index - right.index;
  }).map((entry) => entry.envelope);
}
function formatQuotaBreachMessage(breach) {
  return `outstanding effect quota exceeded: ${breach.quota} limit=${breach.limit} actual=${breach.actual}`;
}
var MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096;
var MAX_SEGMENTED_DOWNLOAD_OPERATION_ID = (1n << 64n) - 1n;
var DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000;
var HEARTBEAT_MISSED_LIMIT = 3;
var DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;
function freshHeartbeatState(nowMs) {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}
function graftWorkerStack(actorId, reason, stack, kind, framesBytes) {
  const error = new Error(reason);
  if (stack)
    error.stack = `${stack}
    ↳ main: ${error.stack ?? ""}`;
  console.log(`[DEBUG] program worker ${actorId || "unknown"} error type=${kind ?? "unknown"} framesBytes=${framesBytes ?? "n/a"}`);
  return error;
}
var ACTIVATION_MINT = Symbol("actor-activation.private-lease");
var mintCapturedActivation;
var capturedActivationMatches;

class CapturedShardActivation {
  #client;
  actorId;
  activationGeneration;
  assertActive;
  turn;
  constructor(mint, client, actorId, generation2, assertActive, turn) {
    if (mint !== ACTIVATION_MINT)
      throw new Error("actor-activation.private-lease");
    this.#client = client;
    this.actorId = actorId;
    this.activationGeneration = generation2;
    this.assertActive = assertActive;
    this.turn = turn;
    Object.freeze(this);
  }
  static {
    mintCapturedActivation = (client, actorId, generation2, assertActive, turn) => new CapturedShardActivation(ACTIVATION_MINT, client, actorId, generation2, assertActive, turn);
    capturedActivationMatches = (activation, client) => activation !== null && typeof activation === "object" && (#client in activation) && activation.#client === client;
  }
}
var RETURN_MINT = Object.freeze({});
var NO_RETURN_FAULT = Object.freeze({});
var returnDomainEnvelope = Object.freeze({ bytes: 800, slots: 4, owners: 4 });
function returnAdmission(kind, phase, bytes = 0) {
  return { step: residentStep(kind, phase, bytes), source: null };
}
var mintCapturedReturn;
var capturedReturnState;
var submitCapturedReturn;
var reserveCapturedResponse;
var mintCapturedReturnPage;
function sameReturnOrigin(left, right) {
  return left.activationGeneration === right.activationGeneration && left.requestSequence === right.requestSequence;
}
function sameReturnIdentity(left, right) {
  return sameReturnOrigin(left.origin, right.origin) && left.returnSequence === right.returnSequence;
}

class OwnedShardReturn {
  #state;
  constructor(mint, state7) {
    if (mint !== RETURN_MINT)
      throw new Error("actor-return.private-owner");
    this.#state = state7;
    state7.facade = this;
    Object.freeze(this);
  }
  static {
    mintCapturedReturn = (state7) => new OwnedShardReturn(RETURN_MINT, state7);
    capturedReturnState = (owner) => owner.#state;
  }
  static matchesOwner(source, owner, activation, lifetime) {
    if (source === null || typeof source !== "object" || !(#state in source))
      return false;
    const instance = source.#state.instance;
    return instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime);
  }
  get origin() {
    return this.#state.origin;
  }
  get page() {
    return this.#state.page;
  }
  get content() {
    return this.#state.content;
  }
  bindContent(content) {
    const state7 = this.#state;
    const instance = state7.instance;
    if (state7.content !== null || !instance.host || !instance.lifetime || !OwnedKernelReturnContent.matches(content, this, instance.host, instance.operation, instance.lifetime))
      return false;
    state7.content = content;
    return true;
  }
  get retainedResponses() {
    return this.#state.outputs?.pending ?? 0;
  }
  reserveResponse(grant) {
    return reserveCapturedResponse(this.#state.client, this.#state, grant);
  }
  execute(events, budget) {
    return submitCapturedReturn(this.#state.client, this.#state, { kind: "execute", events }, budget);
  }
  retry(budget) {
    return submitCapturedReturn(this.#state.client, this.#state, { kind: "retry" }, budget);
  }
  poll(budget) {
    return submitCapturedReturn(this.#state.client, this.#state, { kind: "poll" }, budget);
  }
  cancel(budget) {
    return submitCapturedReturn(this.#state.client, this.#state, { kind: "cancel" }, budget);
  }
}

class OwnedShardReturnPage {
  #state;
  #output;
  #receipt;
  #page;
  constructor(mint, state7, output, receipt, page) {
    if (mint !== RETURN_MINT)
      throw new Error("actor-return.private-page");
    this.#state = state7;
    this.#output = output;
    this.#receipt = receipt;
    this.#page = page;
    Object.freeze(this);
  }
  static {
    mintCapturedReturnPage = (state7, output, receipt, page) => new OwnedShardReturnPage(RETURN_MINT, state7, output, receipt, page);
  }
  static matchesOwner(page, owner, activation, lifetime) {
    if (page === null || typeof page !== "object" || !(#state in page))
      return false;
    const instance = page.#state.instance;
    return instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime) && page.#receipt.identity.origin.activationGeneration === lifetime.activationGeneration && page.#output.responseEnvelope !== null;
  }
  get receipt() {
    return this.#receipt;
  }
  byteAt(index) {
    if (this.#state.failed || this.#state.cancelled || !Number.isInteger(index) || index < 0 || index >= this.#receipt.length)
      throw new Error("actor-return.page-read");
    const block = this.#page[`block${Math.floor(index / 64).toString().padStart(2, "0")}`];
    const word = block[`word${Math.floor(index % 64 / 8)}`];
    return Number(word >> BigInt(index % 8 * 8) & 255n);
  }
}
var NATIVE_PATCH_MINT = Object.freeze({});
var mintNativePatch;
var nativePatchState;
var mintNativeSubmission;

class OwnedNativeUiPatchAuthority {
  #state;
  constructor(mint, state7) {
    if (mint !== NATIVE_PATCH_MINT)
      throw new Error("actor-lifecycle.patch-mint");
    this.#state = state7;
    Object.freeze(this);
  }
  static {
    mintNativePatch = (state7) => new OwnedNativeUiPatchAuthority(NATIVE_PATCH_MINT, state7);
    nativePatchState = (source) => source.#state;
  }
  static matches(source, activation, lifetime) {
    return source !== null && typeof source === "object" && #state in source && source.#state.value.activation === activation && actorInstanceLifetimeEquals(source.#state.value.lifetime, lifetime);
  }
  static matchesOwner(source, owner) {
    return source !== null && typeof source === "object" && #state in source && source.#state.owner.host !== null && source.#state.owner.host === owner;
  }
  get value() {
    return this.#state.value;
  }
  operation(index) {
    const state7 = this.#state;
    if (!Number.isSafeInteger(index) || index !== state7.ordinal || index >= state7.value.operationCount)
      throw new Error("actor-lifecycle.patch-operation-index");
    if (!state7.read) {
      state7.original = state7.operations[index];
      state7.read = true;
    }
    return state7.original;
  }
  acceptInput(claim) {
    const state7 = this.#state;
    if (!state7.read || state7.input !== null && state7.input !== claim || state7.operations[state7.ordinal] !== state7.original || !OwnedUiPatchInputAcceptance.matches(claim, this, state7.ordinal, state7.original))
      return false;
    state7.input = claim;
    return true;
  }
  releaseInput(token) {
    const state7 = this.#state;
    if (!state7.read || !state7.input || state7.operations[state7.ordinal] !== state7.original || !OwnedUiPatchInputRetirement.matches(token, this, state7.ordinal, state7.original))
      return false;
    state7.read = false;
    state7.original = undefined;
    state7.input = null;
    state7.ordinal++;
    return true;
  }
  get inputRetired() {
    return this.#state.ordinal === this.#state.value.operationCount && !this.#state.read;
  }
}

class OwnedNativeUiPatchSubmissionReceipt {
  #source;
  #token;
  constructor(mint, source, token) {
    if (mint !== NATIVE_PATCH_MINT)
      throw new Error("actor-lifecycle.submission-mint");
    this.#source = source;
    this.#token = token;
    Object.freeze(this);
  }
  static {
    mintNativeSubmission = (source, token) => new OwnedNativeUiPatchSubmissionReceipt(NATIVE_PATCH_MINT, source, token);
  }
  static matches(receipt, source, token) {
    return receipt !== null && typeof receipt === "object" && #source in receipt && receipt.#source === source && receipt.#token === token;
  }
}

class ShardClient {
  #residentLedger;
  #uiResidentControllerCell = null;
  #uiResidentControllerRecord = null;
  #uiResidentCell = null;
  #uiResidentRecord = null;
  #uiResidentPool = null;
  #uiResidentPhase = "controller-empty";
  #uiResidentWitness = null;
  #uiResidentFault = NO_RESIDENT_FAULT;
  #uiResidentClosing = false;
  #clientAdmissionPurpose = "none";
  #workerBootstrapCell = null;
  #workerBootstrapRecord = null;
  #workerBootstrapPhase = "empty";
  #workerBootstrapFault = NO_RESIDENT_FAULT;
  #workerAdmissionCell = null;
  #workerAdmissionRecord = null;
  #workerAdmissionIndex = null;
  #workerAdmissionShell = null;
  shards = [];
  actorShard = new Map;
  actorActivations = new Map;
  instanceLifecycles = new Map;
  instanceTurns = new WeakMap;
  pending = new Map;
  exclusiveIndices;
  heartbeatSabView;
  heartbeatTimeoutMs;
  watchdogIntervalMs;
  now;
  createWorker;
  onShardLost;
  onActorTrap;
  onHostEffect;
  maxOutstandingEffectsPerActor;
  outstandingEffectsByActor = new Map;
  effectReplySeq = 0;
  nextRoundRobin = 0;
  requestSeq = 0;
  activationGeneration = 0n;
  watchdogHandle = null;
  constructor(options) {
    try {
      Reflect.apply(residentCapacity(), options.residentLedger, []);
    } catch {
      throw new Error("actor-resident.invalid-ledger");
    }
    this.#residentLedger = options.residentLedger;
    if (options.shardCount < 1)
      throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? DEFAULT_HEARTBEAT_TIMEOUT_MS;
    this.watchdogIntervalMs = options.watchdogIntervalMs ?? this.heartbeatTimeoutMs;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    this.onHostEffect = options.onHostEffect;
    this.maxOutstandingEffectsPerActor = options.maxOutstandingEffectsPerActor ?? DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set;
    for (let index = options.shardCount - exclusiveCount;index < options.shardCount; index += 1)
      exclusive.add(index);
    this.exclusiveIndices = exclusive;
    for (let index = 0;index < options.shardCount; index += 1)
      this.shards.push(this.spawnShard(index));
  }
  static {
    submitCapturedReturn = (client, state7, work, budget) => client.sendCapturedReturn(state7, work, budget);
    reserveCapturedResponse = (client, state7, grant) => client.reserveReturnResponse(state7, grant);
  }
  static matchesResidentLedger(client, ledger) {
    return client !== null && typeof client === "object" && #residentLedger in client && client.#residentLedger === ledger;
  }
  static matchesActivation(client, activation) {
    return client !== null && typeof client === "object" && #residentLedger in client && capturedActivationMatches(activation, client);
  }
  prepareWorkerBootstrap(grant) {
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-worker.prepare");
    switch (this.#workerBootstrapPhase) {
      case "close-preparing":
      case "close-prepare-refused":
      case "close-cell-held":
      case "close-claiming":
      case "close-record-admitting":
      case "close-record-refused":
      case "record-held":
      case "cell-closing":
      case "close-attempted":
      case "pending-release-observing":
      case "cell-observing":
      case "fault-held":
      case "cancelled":
        return residentStep("rejected", "actor-worker.stopped");
    }
    try {
      const recovered = this.#recoverWorkerBootstrap();
      if (recovered)
        return recovered;
      if (this.#workerBootstrapFault !== NO_RESIDENT_FAULT || this.#workerBootstrapCell?.hasFailure)
        return residentStep("rejected", "actor-worker.fault-held");
    } catch (error) {
      this.#captureWorkerBootstrapFault(error);
      return residentStep("rejected", "actor-worker.recovery-fault");
    }
    try {
      const shared = this.#prepareSharedResidentController(grant);
      if (shared)
        return shared;
    } catch (error) {
      this.captureUiResidentPoolFault(error);
      return residentStep("rejected", "actor-worker.shared-fault");
    }
    try {
      if (this.#workerBootstrapPhase === "ready")
        return residentStep(this.#workerBootstrapRecord?.matchesLiveShell(this) ? "ready" : "rejected", "actor-worker.prepared");
      if (this.#workerBootstrapPhase === "empty") {
        if (this.#clientAdmissionPurpose !== "none")
          return residentStep("blocked", "actor-worker.foreign-purpose");
        if (this.#workerAdmissionCell || this.#workerAdmissionRecord || this.#workerAdmissionIndex !== null || this.#workerAdmissionShell)
          return residentStep("rejected", "actor-worker.child-held");
        if (!residentGrant(grant, 296))
          return residentStep("blocked", "actor-worker.bootstrap");
        this.#clientAdmissionPurpose = "worker-root";
        this.#workerBootstrapPhase = "preparing";
        const current = this.#residentLedger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected")
          this.#workerBootstrapPhase = "prepare-refused";
        return residentChild(current, grant);
      }
      const cell = this.#workerBootstrapCell;
      if (!cell)
        return residentStep("rejected", "actor-worker.cell");
      if (this.#workerBootstrapPhase === "cell-held") {
        if (this.#clientAdmissionPurpose !== "worker-root")
          return residentStep("blocked", "actor-worker.foreign-purpose");
        this.#workerBootstrapPhase = "claiming";
        const current = this.#residentLedger.claimAdmission(this, cell, grant);
        if (current.kind === "blocked")
          this.#workerBootstrapPhase = "cell-held";
        else if (current.kind === "rejected")
          this.#workerBootstrapPhase = "claim-refused";
        return residentChild(current, grant);
      }
      if (this.#workerBootstrapPhase === "claimed") {
        if (!residentGrant(grant, 264))
          return residentStep("blocked", "actor-worker.record");
        this.#workerBootstrapPhase = "record-admitting";
        const admitted20 = this.#residentLedger.reserveRecord("data", workerControllerEnvelope, cell, grant);
        if (admitted20.step.kind === "blocked")
          this.#workerBootstrapPhase = "claimed";
        else if (admitted20.step.kind === "rejected")
          this.#workerBootstrapPhase = "record-refused";
        return residentChild(admitted20.step, grant);
      }
      if (this.#workerBootstrapPhase === "installing" && this.#workerBootstrapRecord) {
        this.#workerBootstrapPhase = "observing";
        return residentChild(this.#workerBootstrapRecord.install(this, grant), grant);
      }
      return residentStep("rejected", "actor-worker.admission");
    } catch (error) {
      this.#captureWorkerBootstrapFault(error);
      return residentStep("rejected", "actor-worker.prepare-fault");
    }
  }
  #captureWorkerBootstrapFault(error) {
    if (this.#workerBootstrapFault === NO_RESIDENT_FAULT)
      this.#workerBootstrapFault = error;
    else if (!Object.is(this.#workerBootstrapFault, error))
      throw error;
  }
  #recoverWorkerBootstrap() {
    const phase = this.#workerBootstrapPhase;
    if (phase === "preparing" || phase === "prepare-refused" || phase === "close-preparing" || phase === "close-prepare-refused") {
      if (this.#clientAdmissionPurpose !== "worker-root")
        return residentStep("blocked", "actor-worker.foreign-purpose");
      const cell2 = this.#residentLedger.preparedAdmission(this);
      if (!cell2) {
        if (phase !== "prepare-refused" && phase !== "close-prepare-refused" || this.#workerBootstrapFault !== NO_RESIDENT_FAULT)
          return residentStep("blocked", "actor-worker.admission-handoff");
        this.#clientAdmissionPurpose = "none";
        this.#workerBootstrapPhase = phase === "close-prepare-refused" ? "cancelled" : "empty";
        return residentStep("pending", "actor-worker.empty-admission-observation", 64);
      }
      this.#workerBootstrapCell = cell2;
      this.#workerBootstrapPhase = phase === "close-preparing" || phase === "close-prepare-refused" ? "close-cell-held" : "cell-held";
      return residentStep("pending", "actor-worker.cell-observation", 64);
    }
    const cell = this.#workerBootstrapCell;
    if ((phase === "claiming" || phase === "close-claiming") && cell) {
      if (this.#clientAdmissionPurpose !== "worker-root")
        return residentStep("blocked", "actor-worker.foreign-purpose");
      const pending2 = this.#residentLedger.preparedAdmission(this);
      if (cell.claimed && pending2 === null) {
        this.#clientAdmissionPurpose = "none";
        this.#workerBootstrapPhase = phase === "close-claiming" ? "close-cell-held" : "claimed";
        return residentStep("pending", "actor-worker.claim-observation", 64);
      }
      if (phase === "close-claiming" && pending2 === cell && !cell.claimed) {
        this.#workerBootstrapPhase = "close-cell-held";
        return residentStep("pending", "actor-worker.unclaimed-close-observation", 64);
      }
      return residentStep("blocked", "actor-worker.unclaimed", 64);
    }
    if ((phase === "record-admitting" || phase === "close-record-admitting" || phase === "record-refused" || phase === "close-record-refused") && cell) {
      const result3 = cell.result;
      this.#workerBootstrapRecord = result3?.record ?? null;
      if (!result3 && (phase === "record-refused" || phase === "close-record-refused") && this.#workerBootstrapFault === NO_RESIDENT_FAULT) {
        this.#workerBootstrapPhase = phase === "close-record-refused" ? "close-cell-held" : "claimed";
        return residentStep("pending", "actor-worker.unused-record-refusal-observation", 64);
      }
      const ready = phase !== "close-record-admitting" && this.#workerBootstrapRecord !== null && result3?.step.kind === "ready" && !cell.hasFailure && this.#workerBootstrapFault === NO_RESIDENT_FAULT;
      this.#workerBootstrapPhase = ready ? "installing" : "record-held";
      return residentStep(ready || phase === "close-record-admitting" ? "pending" : "rejected", "actor-worker.record-observation", 64);
    }
    if (phase === "observing" && this.#workerBootstrapRecord) {
      if (!this.#workerBootstrapRecord.matchesShell(this))
        return residentStep("blocked", "actor-worker.installation", 64);
      const live3 = this.#workerBootstrapFault === NO_RESIDENT_FAULT && this.#workerBootstrapRecord.matchesLiveShell(this);
      this.#workerBootstrapPhase = live3 ? "ready" : "record-held";
      return residentStep(live3 ? "pending" : "rejected", "actor-worker.installation-observation", 64);
    }
    return null;
  }
  closeWorkerBootstrapStep(grant) {
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-worker.close");
    if (this.#workerBootstrapPhase === "cancelled")
      return residentStep("complete", "actor-worker.close");
    switch (this.#workerBootstrapPhase) {
      case "empty":
        this.#workerBootstrapPhase = "cancelled";
        return residentStep("complete", "actor-worker.unstarted-close", 64);
      case "preparing":
        this.#workerBootstrapPhase = "close-preparing";
        break;
      case "prepare-refused":
        this.#workerBootstrapPhase = "close-prepare-refused";
        break;
      case "claiming":
        this.#workerBootstrapPhase = "close-claiming";
        break;
      case "record-admitting":
        this.#workerBootstrapPhase = "close-record-admitting";
        break;
      case "record-refused":
        this.#workerBootstrapPhase = "close-record-refused";
        break;
      case "cell-held":
      case "claim-refused":
      case "claimed":
        this.#workerBootstrapPhase = "close-cell-held";
        break;
      case "installing":
      case "observing":
      case "ready":
        this.#workerBootstrapPhase = "record-held";
        return residentStep("pending", "actor-worker.record-retained", 64);
    }
    try {
      const recovered = this.#recoverWorkerBootstrap();
      if (recovered)
        return recovered;
      const cell = this.#workerBootstrapCell;
      if (this.#workerBootstrapFault !== NO_RESIDENT_FAULT) {
        if (!cell)
          return residentStep("blocked", "actor-worker.fault-without-cell");
        if (!cell.hasFailure)
          return cell.retainFailure(this.#workerBootstrapFault, grant);
        if (!Object.is(cell.failure, this.#workerBootstrapFault))
          return residentStep("blocked", "actor-worker.distinct-fault");
      }
      if (this.#workerBootstrapRecord || cell?.result || this.#workerBootstrapPhase === "record-held")
        return residentStep("blocked", "actor-worker.record-retained");
      if (!cell)
        return residentStep("blocked", "actor-worker.cell-proof");
      if (this.#workerBootstrapPhase === "close-cell-held") {
        this.#workerBootstrapPhase = "cell-closing";
        cell.beginClose();
        return residentStep("pending", "actor-worker.cell-begin-close", 64);
      }
      if (this.#workerBootstrapPhase === "pending-release-observing") {
        if (this.#residentLedger.preparedAdmission(this) === cell)
          return residentStep("blocked", "actor-worker.pending-release-proof", 64);
        if (cell.hasFailure) {
          if (this.#clientAdmissionPurpose === "worker-root")
            this.#clientAdmissionPurpose = "none";
          this.#workerBootstrapPhase = "fault-held";
        } else
          this.#workerBootstrapPhase = "cell-closing";
        return residentStep("pending", "actor-worker.pending-release-observation", 64);
      }
      if (this.#workerBootstrapPhase === "cell-observing") {
        if (cell.hasFailure || this.#workerBootstrapFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty())
          return residentStep("blocked", "actor-worker.cell-retirement", 64);
        this.#workerBootstrapCell = null;
        if (this.#clientAdmissionPurpose === "worker-root")
          this.#clientAdmissionPurpose = "none";
        this.#workerBootstrapPhase = "cancelled";
        return residentStep("complete", "actor-worker.cell-unlink-observation", 64);
      }
      if (this.#workerBootstrapPhase === "close-attempted" || this.#workerBootstrapPhase === "fault-held")
        return residentStep("blocked", "actor-worker.close-handoff");
      if (this.#workerBootstrapPhase === "cell-closing") {
        this.#workerBootstrapPhase = "close-attempted";
        const current = cell.closeStep(grant);
        this.#workerBootstrapPhase = current.kind === "complete" ? "cell-observing" : current.kind === "pending" && current.phase === "resident-admission-bootstrap-release" ? "pending-release-observing" : "cell-closing";
        return residentChild(current, grant);
      }
      return residentStep("blocked", "actor-worker.close-phase");
    } catch (error) {
      this.#captureWorkerBootstrapFault(error);
      return residentStep("rejected", "actor-worker.close-fault");
    }
  }
  prepareUiResidentPool(ledger, grant) {
    if (ledger !== this.#residentLedger)
      return residentStep("rejected", "actor-resident.foreign-ledger");
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-resident.pool-prepare");
    if (this.#uiResidentClosing || this.#uiResidentPool)
      return residentStep("rejected", "actor-resident.pool-owned");
    try {
      const recovered = this.#recoverUiResidentPool();
      if (recovered)
        return recovered;
      const shared = this.#prepareSharedResidentController(grant);
      if (shared)
        return shared;
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentCell?.hasFailure)
        return residentStep("rejected", "actor-resident.pool-fault-retirement");
      if (this.#uiResidentPhase === "prepared")
        return residentStep("ready", "actor-resident.pool-prepare");
      if (this.#uiResidentPhase === "empty") {
        if (this.#clientAdmissionPurpose !== "none")
          return residentStep("blocked", "actor-resident.foreign-purpose");
        if (!residentGrant(grant, 296))
          return residentStep("blocked", "actor-resident.pool-bootstrap");
        this.#clientAdmissionPurpose = "ui-pool";
        this.#uiResidentPhase = "preparing";
        const current = ledger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected")
          this.#uiResidentPhase = "prepare-refused";
        return residentChild(current, grant);
      }
      const cell = this.#uiResidentCell;
      if (!cell)
        return residentStep("rejected", "actor-resident.pool-cell");
      if (this.#uiResidentPhase === "cell-held") {
        if (this.#clientAdmissionPurpose !== "ui-pool")
          return residentStep("blocked", "actor-resident.foreign-purpose");
        this.#uiResidentPhase = "claiming";
        const current = ledger.claimAdmission(this, cell, grant);
        if (current.kind === "blocked")
          this.#uiResidentPhase = "cell-held";
        return residentChild(current, grant);
      }
      if (this.#uiResidentPhase === "claimed") {
        if (!residentGrant(grant, 264))
          return residentStep("blocked", "actor-resident.pool-record");
        this.#uiResidentPhase = "record-admitting";
        const admitted20 = ledger.reserveRecord("data", poolRecordEnvelope, cell, grant);
        if (admitted20.step.kind === "blocked")
          this.#uiResidentPhase = "claimed";
        return residentChild(admitted20.step, grant);
      }
      return residentStep("rejected", "actor-resident.pool-owned");
    } catch (error) {
      this.captureUiResidentPoolFault(error);
      return residentStep("rejected", "actor-resident.pool-prepare-fault");
    }
  }
  captureUiResidentPoolFault(error) {
    if (this.#uiResidentFault === NO_RESIDENT_FAULT)
      this.#uiResidentFault = error;
    else if (!Object.is(this.#uiResidentFault, error))
      throw error;
  }
  #prepareSharedResidentController(grant) {
    const recovered = this.#recoverUiResidentController();
    if (recovered)
      return recovered;
    if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentControllerCell?.hasFailure)
      return residentStep("rejected", "actor-resident.controller-fault-held");
    if (this.#uiResidentPhase === "retired" && !this.#uiResidentControllerCell && !this.#uiResidentControllerRecord) {
      this.#uiResidentPhase = "controller-empty";
      return residentStep("pending", "actor-resident.shared-unstarted-observation", 64);
    }
    const prepared = this.#prepareUiResidentController(grant);
    if (prepared)
      return prepared;
    return this.#uiResidentControllerRecord?.matchesLiveShell(this) ? null : residentStep("rejected", "actor-resident.controller-not-live");
  }
  #recoverUiResidentController() {
    if (this.#uiResidentPhase === "controller-preparing" || this.#uiResidentPhase === "controller-prepare-refused") {
      const cell = this.#residentLedger.preparedAdmission(this);
      if (!cell) {
        if (this.#uiResidentPhase !== "controller-prepare-refused" || this.#uiResidentFault !== NO_RESIDENT_FAULT)
          return residentStep("blocked", "actor-resident.controller-admission-handoff");
        this.#uiResidentPhase = "controller-empty";
        return residentStep("pending", "actor-resident.controller-empty-admission", 64);
      }
      this.#uiResidentControllerCell = cell;
      this.#uiResidentPhase = "controller-cell-held";
      return residentStep("pending", "actor-resident.controller-cell-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-claiming" && this.#uiResidentControllerCell) {
      if (!this.#uiResidentControllerCell.claimed || this.#residentLedger.preparedAdmission(this) !== null)
        return residentStep("blocked", "actor-resident.controller-unclaimed", 64);
      this.#uiResidentPhase = "controller-claimed";
      return residentStep("pending", "actor-resident.controller-claim-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-record-admitting" && this.#uiResidentControllerCell) {
      const result3 = this.#uiResidentControllerCell.result;
      this.#uiResidentControllerRecord = result3?.record ?? null;
      const ready = this.#uiResidentControllerRecord !== null && result3?.step.kind === "ready" && !this.#uiResidentControllerCell.hasFailure && this.#uiResidentFault === NO_RESIDENT_FAULT;
      this.#uiResidentPhase = ready ? "controller-installing" : "controller-rejected";
      return residentStep(ready ? "pending" : "rejected", "actor-resident.controller-record-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-observing" && this.#uiResidentControllerRecord) {
      if (!this.#uiResidentControllerRecord.matchesShell(this))
        return residentStep("blocked", "actor-resident.controller-installation", 64);
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || !this.#uiResidentControllerRecord.matchesLiveShell(this)) {
        this.#uiResidentPhase = "controller-rejected";
        return residentStep("rejected", "actor-resident.controller-not-live", 64);
      }
      this.#uiResidentPhase = "empty";
      return residentStep("pending", "actor-resident.controller-installation", 64);
    }
    return null;
  }
  #prepareUiResidentController(grant) {
    if (this.#uiResidentPhase === "controller-empty") {
      if (!residentGrant(grant, 296))
        return residentStep("blocked", "actor-resident.controller-bootstrap");
      this.#uiResidentPhase = "controller-preparing";
      const current = this.#residentLedger.prepareAdmission(this, "data", grant);
      if (current.kind === "blocked" || current.kind === "rejected")
        this.#uiResidentPhase = "controller-prepare-refused";
      return residentChild(current, grant);
    }
    const cell = this.#uiResidentControllerCell;
    if (this.#uiResidentPhase === "controller-cell-held" && cell) {
      this.#uiResidentPhase = "controller-claiming";
      const current = this.#residentLedger.claimAdmission(this, cell, grant);
      if (current.kind === "blocked")
        this.#uiResidentPhase = "controller-cell-held";
      return residentChild(current, grant);
    }
    if (this.#uiResidentPhase === "controller-claimed" && cell) {
      if (!residentGrant(grant, 264))
        return residentStep("blocked", "actor-resident.controller-record");
      this.#uiResidentPhase = "controller-record-admitting";
      const admitted20 = this.#residentLedger.reserveRecord("data", poolControllerEnvelope, cell, grant);
      if (admitted20.step.kind === "blocked")
        this.#uiResidentPhase = "controller-claimed";
      return residentChild(admitted20.step, grant);
    }
    if (this.#uiResidentPhase === "controller-installing" && this.#uiResidentControllerRecord) {
      this.#uiResidentPhase = "controller-observing";
      return residentChild(this.#uiResidentControllerRecord.install(this, grant), grant);
    }
    return this.#uiResidentPhase === "controller-rejected" ? residentStep("rejected", "actor-resident.controller-admission") : null;
  }
  #recoverUiResidentPool() {
    if (!this.#uiResidentCell && (this.#uiResidentPhase === "preparing" || this.#uiResidentPhase === "prepare-refused")) {
      if (this.#clientAdmissionPurpose !== "ui-pool")
        return residentStep("blocked", "actor-resident.foreign-purpose");
      const cell = this.#residentLedger.preparedAdmission(this);
      if (!cell) {
        if (this.#uiResidentPhase !== "prepare-refused" || this.#uiResidentFault !== NO_RESIDENT_FAULT)
          return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.#clientAdmissionPurpose = "none";
        this.#uiResidentPhase = "empty";
        return residentStep("pending", "actor-resident.pool-empty-admission", 64);
      }
      this.#uiResidentCell = cell;
      this.#uiResidentPhase = "cell-held";
      return residentStep("pending", "actor-resident.pool-cell-observation", 64);
    }
    if (this.#uiResidentPhase === "claiming" && this.#uiResidentCell) {
      if (this.#clientAdmissionPurpose !== "ui-pool" || !this.#uiResidentCell.claimed || this.#residentLedger.preparedAdmission(this) !== null)
        return residentStep("blocked", "actor-resident.pool-unclaimed", 64);
      this.#clientAdmissionPurpose = "none";
      this.#uiResidentPhase = "claimed";
      return residentStep("pending", "actor-resident.pool-claim-observation", 64);
    }
    if (this.#uiResidentCell && this.#uiResidentPhase === "record-admitting") {
      const result3 = this.#uiResidentCell.result;
      this.#uiResidentRecord = result3?.record ?? null;
      const ready = this.#uiResidentRecord !== null && result3?.step.kind === "ready" && !this.#uiResidentCell.hasFailure && this.#uiResidentFault === NO_RESIDENT_FAULT;
      this.#uiResidentPhase = ready ? "prepared" : "rejected";
      return residentStep(ready ? "pending" : "rejected", "actor-resident.pool-record-observation", 64);
    }
    return null;
  }
  #handoffUiResidentFault(grant) {
    if (this.#uiResidentFault === NO_RESIDENT_FAULT)
      return null;
    const cell = this.#uiResidentCell ?? this.#uiResidentControllerCell;
    if (!cell)
      return residentStep("blocked", "actor-resident.pool-fault-retirement");
    if (cell.hasFailure)
      return Object.is(cell.failure, this.#uiResidentFault) ? null : residentStep("blocked", "actor-resident.pool-distinct-fault");
    return cell.retainFailure(this.#uiResidentFault, grant);
  }
  #closeUiResidentAdmission(grant) {
    const cell = this.#uiResidentCell;
    const record = this.#uiResidentRecord;
    if (!cell)
      return residentStep("blocked", "actor-resident.pool-cell-proof");
    if (this.#uiResidentPhase === "record-observing") {
      if (!record || !OwnedResidentRetirement.matches(record.retirement, record))
        return residentStep("blocked", "actor-resident.pool-record-proof", 64);
      cell.beginClose();
      this.#uiResidentPhase = "cell-closing";
      return residentStep("pending", "actor-resident.pool-record-observation", 64);
    }
    if (this.#uiResidentPhase === "cell-closing") {
      const current = cell.closeStep(grant);
      if (current.kind === "complete")
        this.#uiResidentPhase = "cell-observing";
      return residentChild(current, grant);
    }
    if (this.#uiResidentPhase === "cell-observing") {
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty() || record !== null && (!OwnedResidentRetirement.matches(record.retirement, record) || !record.terminalIsEmpty()))
        return residentStep("blocked", "actor-resident.pool-cell-proof", 64);
      this.#uiResidentRecord = null;
      this.#uiResidentCell = null;
      this.#uiResidentPool = null;
      this.#uiResidentWitness = null;
      if (this.#clientAdmissionPurpose === "ui-pool")
        this.#clientAdmissionPurpose = "none";
      this.#uiResidentPhase = "retired";
      return residentStep("complete", "actor-resident.pool-release", 64);
    }
    return residentStep("blocked", "actor-resident.pool-cell-phase");
  }
  ownsUiResidentPool(pool) {
    return pool !== null && this.#uiResidentPool === pool;
  }
  closeUiResidentPoolStep(grant) {
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-resident.pool-parent-close");
    if (this.#uiResidentPhase === "retired")
      return residentStep("complete", "actor-resident.pool-parent-close");
    this.#uiResidentClosing = true;
    try {
      const controller = this.#recoverUiResidentController();
      if (controller)
        return controller;
      const recovered = this.#recoverUiResidentPool();
      if (recovered)
        return recovered;
      const handoff = this.#handoffUiResidentFault(grant);
      if (handoff)
        return handoff;
      if (this.#uiResidentControllerCell?.hasFailure) {
        if (!this.#uiResidentControllerRecord && !this.#uiResidentControllerCell.claimed) {
          this.#uiResidentControllerCell.beginClose();
          return this.#uiResidentControllerCell.closeStep(grant);
        }
        return residentStep("rejected", "actor-resident.controller-fault-held");
      }
      if (this.#uiResidentPhase === "controller-empty" && !this.#uiResidentControllerCell) {
        this.#uiResidentPhase = "retired";
        return residentStep("complete", "actor-resident.pool-unstarted-close", 64);
      }
      const prepared = this.#prepareUiResidentController(grant);
      if (prepared)
        return prepared;
      const cell = this.#uiResidentCell;
      const record = this.#uiResidentRecord;
      const pool = this.#uiResidentPool;
      if (!cell) {
        if (this.#uiResidentPhase !== "empty")
          return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.#uiResidentPhase = "retired";
        return residentStep("complete", "actor-resident.pool-parent-close", 64);
      }
      if (this.#uiResidentPhase === "record-observing" || this.#uiResidentPhase === "cell-closing" || this.#uiResidentPhase === "cell-observing")
        return this.#closeUiResidentAdmission(grant);
      if (!pool) {
        if (!record) {
          cell.beginClose();
          this.#uiResidentPhase = "cell-closing";
          return residentStep("pending", "actor-resident.pool-unused-cell-close", 64);
        }
        if (this.#uiResidentPhase === "prepared" || this.#uiResidentPhase === "rejected") {
          this.#uiResidentPhase = "unused-closing";
          record.beginClose();
          return residentStep("pending", "actor-resident.pool-unused-close", 64);
        }
        if (this.#uiResidentPhase === "unused-closing") {
          const current = record.closeStep(grant);
          if (current.kind === "complete")
            this.#uiResidentPhase = "record-observing";
          return residentChild(current, grant);
        }
        return residentStep("blocked", "actor-resident.pool-unused-proof");
      }
      if (this.#uiResidentPhase === "owned") {
        this.#uiResidentPhase = "pool-closing";
        pool.beginClose();
        return residentStep("pending", "actor-resident.pool-begin-close", 64);
      }
      if (this.#uiResidentPhase === "pool-closing") {
        const current = pool.closeStep(grant);
        const forwarded = residentChild(current, grant);
        if (current.kind === "complete" && forwarded.kind === "pending")
          this.#uiResidentPhase = "pool-observing";
        return forwarded;
      }
      if (this.#uiResidentPhase === "pool-observing") {
        const witness = pool.retirement;
        if (!OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger))
          return residentStep("blocked", "actor-resident.pool-private-proof", 64);
        this.#uiResidentWitness = witness;
        this.#uiResidentPhase = "pool-proved";
        return residentStep("pending", "actor-resident.pool-observation", 64);
      }
      return this.releaseUiResidentPool(pool, this.#uiResidentWitness, grant);
    } catch (error) {
      this.captureUiResidentPoolFault(error);
      return residentStep("rejected", "actor-resident.pool-parent-fault");
    }
  }
  installUiResidentPool(pool, grant) {
    const record = this.#uiResidentRecord;
    if (!record || this.#uiResidentPhase !== "prepared" && this.#uiResidentPhase !== "owned" || !OwnedUiResidentPool.matchesComposition(pool, this, this.#residentLedger) || this.#uiResidentPool !== null && this.#uiResidentPool !== pool)
      return residentStep("rejected", "actor-resident.pool-install");
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-resident.pool-install");
    this.#uiResidentPool = pool;
    this.#uiResidentPhase = "owned";
    try {
      if (record.matchesShell(pool))
        return residentStep("ready", "actor-resident.pool-installed");
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentCell?.hasFailure)
        return residentStep("blocked", "actor-resident.pool-fault-retirement");
      return record.install(pool, grant);
    } catch (error) {
      this.captureUiResidentPoolFault(error);
      return residentStep("rejected", "actor-resident.pool-install-fault");
    }
  }
  releaseUiResidentPool(pool, witness, grant) {
    const record = this.#uiResidentRecord;
    if (!record || this.#uiResidentPool !== pool || !OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger) || this.#uiResidentWitness !== null && this.#uiResidentWitness !== witness)
      return residentStep("rejected", "actor-resident.pool-witness");
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-resident.pool-release");
    try {
      const handoff = this.#handoffUiResidentFault(grant);
      if (handoff)
        return handoff;
      this.#uiResidentWitness = witness;
      if (this.#uiResidentPhase === "closing" && OwnedResidentRecordDetachment.matches(record.detachment, record, pool)) {
        this.#uiResidentPhase = "detached";
        return residentStep("pending", "actor-resident.pool-detachment", 64);
      }
      if (this.#uiResidentPhase === "owned" || this.#uiResidentPhase === "pool-proved") {
        this.#uiResidentPhase = "closing";
        record.beginClose();
        return residentStep("pending", "actor-resident.pool-close-record", 64);
      }
      if (this.#uiResidentPhase === "closing")
        return record.detach(pool, grant);
      if (this.#uiResidentPhase === "detached") {
        const current = record.closeStep(grant);
        if (current.kind === "complete")
          this.#uiResidentPhase = "record-observing";
        return residentChild(current, grant);
      }
      return this.#closeUiResidentAdmission(grant);
    } catch (error) {
      this.captureUiResidentPoolFault(error);
      return residentStep("rejected", "actor-resident.pool-release-fault");
    }
  }
  spawnShard(index) {
    const worker = this.createWorker(index);
    const slot = { index, worker, available: true, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set, actorIds: new Set };
    worker.onmessage = (event) => this.handleMessage(slot, event.data);
    worker.onerror = (error) => {
      if (this.shards[index] !== slot)
        return;
      console.error(`[DEBUG] shard ${index} worker error`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed`));
    };
    if (this.heartbeatSabView)
      worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.heartbeatSabView.buffer });
    return slot;
  }
  handleMessage(slot, message) {
    if (!slot.available || this.shards[slot.index] !== slot)
      return;
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      if (message.actorId === "*" && message.activationGeneration === null || this.inboundActivation(slot, message.actorId, message.activationGeneration))
        this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      const activation = this.inboundActivation(slot, message.actorId, message.activationGeneration);
      if (activation)
        this.handleInboundFrame(activation, message.frame);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry || entry.slot !== slot || this.shards[slot.index] !== slot)
      return;
    if (entry.output && !entry.output.captureResponse(message)) {
      entry.reject(new Error("actor-output.response-refused"));
      return;
    }
    try {
      this.pending.delete(message.requestId);
      slot.pendingRequestIds.delete(message.requestId);
      this.recomputeOldestPending(slot);
      if (message.ok)
        entry.resolve(message.value);
      else
        entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
    } catch (error) {
      entry.reject(error);
    }
  }
  recomputeOldestPending(slot) {
    let oldest = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      if (oldest === null || entry.startedAtMs < oldest)
        oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }
  failShard(slot, error) {
    slot.available = false;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry)
        continue;
      this.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
    for (const actorId of slot.actorIds) {
      this.abortOutstandingEffects(actorId);
      const activation = this.actorActivations.get(actorId);
      if (activation?.slot === slot) {
        activation.available = false;
        if (activation.instance)
          activation.instance.failure = "worker-lost";
      }
      this.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
  }
  rejectActorPending(slot, actorId, error) {
    for (const requestId of [...slot.pendingRequestIds]) {
      const entry = this.pending.get(requestId);
      if (entry?.actorId !== actorId)
        continue;
      this.pending.delete(requestId);
      slot.pendingRequestIds.delete(requestId);
      entry.reject(error);
    }
    this.recomputeOldestPending(slot);
  }
  assignShard(actorId) {
    const existing = this.actorShard.get(actorId);
    if (existing !== undefined)
      return this.shards[existing];
    const roundRobinCount = this.shards.length - this.exclusiveIndices.size;
    let index = this.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.exclusiveIndices.has(index))
      index = (index + 1) % this.shards.length;
    this.nextRoundRobin = (this.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.actorShard.set(actorId, index);
    this.shards[index].actorIds.add(actorId);
    return this.shards[index];
  }
  leaseExclusive(actorId, options) {
    const already = this.actorShard.get(actorId);
    if (already !== undefined && this.exclusiveIndices.has(already))
      return already;
    for (const index of this.exclusiveIndices) {
      const slot = this.shards[index];
      if (slot.actorIds.size === 0 || options?.force) {
        const activation = this.actorActivations.get(actorId);
        if (activation)
          activation.operationsAllowed = false;
        this.abortOutstandingEffects(actorId);
        if (already !== undefined)
          this.shards[already].actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`[DEBUG] ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.exclusiveIndices.size} reserved, all leased)`);
  }
  releaseExclusive(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined || !this.exclusiveIndices.has(index))
      return;
    const activation = this.actorActivations.get(actorId);
    if (activation)
      activation.operationsAllowed = false;
    this.abortOutstandingEffects(actorId);
    this.shards[index].actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  shardIndexFor(actorId) {
    return this.actorShard.get(actorId);
  }
  nextRequestId() {
    if (this.requestSeq >= Number.MAX_SAFE_INTEGER)
      throw new Error("shard-request.sequence-exhausted");
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }
  send(slot, message, requestId, posted, output = null) {
    if (requestId === null) {
      slot.worker.postMessage(message);
      posted?.();
      return Promise.resolve(undefined);
    }
    return new Promise((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve, reject, slot, startedAtMs, actorId: "actorId" in message ? message.actorId : "", output });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null)
        slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      try {
        slot.worker.postMessage(message);
        posted?.();
      } catch (error) {
        this.pending.delete(requestId);
        slot.pendingRequestIds.delete(requestId);
        this.recomputeOldestPending(slot);
        reject(error);
      }
    });
  }
  async activate(actorId, moduleUrl, caps, budget, assets = []) {
    if (this.actorActivations.get(actorId)?.available)
      throw new Error("actor-close.activation-already-owned");
    if (this.activationGeneration >= 0xffffffffffffffffn)
      throw new Error("actor-close.activation-generation-exhausted");
    const requestId = this.nextRequestId();
    const generation2 = this.activationGeneration + 1n;
    const slot = this.assignShard(actorId);
    const activation = { slot, actorId, generation: generation2, available: true, activated: false, teardownPosted: false, operationsAllowed: true, operationGeneration: 0n, lastGuestLifetime: 0n, lastReturnSequence: 0n, returned: null, instance: null, close: null };
    this.activationGeneration = generation2;
    this.actorActivations.set(actorId, activation);
    await this.send(slot, { kind: "activate", requestId, actorId, activationGeneration: generation2, moduleUrl, caps, budget, assets }, requestId);
    activation.activated = true;
  }
  activationIsActive(activation) {
    return activation.available && activation.slot.available && activation.activated && activation.operationsAllowed && activation.close === null && this.actorActivations.get(activation.actorId) === activation && this.actorShard.get(activation.actorId) === activation.slot.index && this.shards[activation.slot.index] === activation.slot;
  }
  inboundActivation(slot, actorId, generation2) {
    const activation = this.actorActivations.get(actorId);
    return activation && activation.slot === slot && activation.generation === generation2 && this.activationIsActive(activation) ? activation : undefined;
  }
  captureActorActivation(actorId) {
    const activation = this.actorActivations.get(actorId);
    if (!activation?.activated)
      throw new Error("actor-activation.not-ready");
    const slot = activation.slot;
    const worker = slot.worker;
    const operationGeneration = activation.operationGeneration;
    const assertActive = () => {
      if (!activation.available || !slot.available || !activation.operationsAllowed || activation.operationGeneration !== operationGeneration || activation.close !== null || this.actorActivations.get(actorId) !== activation || this.actorShard.get(actorId) !== slot.index || this.shards[slot.index] !== slot || slot.worker !== worker)
        throw new Error("actor-activation.revoked");
    };
    assertActive();
    return mintCapturedActivation(this, actorId, activation.generation, assertActive, async (events, budget, commandPage) => {
      assertActive();
      if (activation.returned !== null)
        throw new Error("actor-return.already-owned");
      const owner = activation.instance;
      const requestId = this.nextRequestId();
      const result3 = await this.send(slot, { kind: "turn", requestId, actorId, activationGeneration: activation.generation, events, commandPage, budget }, requestId);
      if (owner)
        this.recordInstanceTurn(owner, result3);
      try {
        assertActive();
      } catch (error) {
        if (owner)
          owner.interruptedTurn = result3;
        throw error;
      }
      return result3;
    });
  }
  captureInstanceLifecycle(actorId, instanceId) {
    if (!Number.isInteger(instanceId) || instanceId < 0 || instanceId > 4294967295)
      throw new Error("actor-lifecycle.invalid-instance");
    const activation = this.actorActivations.get(actorId);
    if (!activation || !this.activationIsActive(activation))
      throw new Error("actor-lifecycle.activation-not-ready");
    if (activation.instance !== null)
      throw new Error("actor-lifecycle.instance-already-owned");
    const operation = this.captureActorActivation(actorId);
    this.nextRequestId();
    const open = Object.freeze({ kind: "open", activationGeneration: activation.generation, instanceId, requestSequence: this.requestSeq });
    const owner = { activation, operation, open, phase: "opening", lifetime: null, receipt: null, accepted: null, close: null, host: null, inFlight: false, failure: null, interruptedTurn: null, cancellation: null, lastPatchSequence: 0n, returnCell: null, returnRecord: null, returnPhase: "empty", returnFault: NO_RETURN_FAULT, returnCapacity: 0 };
    activation.instance = owner;
    this.instanceLifecycles.set(open.requestSequence, owner);
    return Object.freeze({
      activation: operation,
      openRequest: open,
      get lifetime() {
        return owner.lifetime;
      },
      get pendingReceipt() {
        return owner.receipt;
      },
      get interruptedTurn() {
        return owner.interruptedTurn;
      },
      get pendingReturn() {
        return owner.activation.returned?.instance === owner ? owner.activation.returned.facade : null;
      },
      reserveReturn: (maximumResponses, grant) => this.reserveInstanceReturn(owner, maximumResponses, grant),
      open: async (input, budget) => {
        if (owner.phase !== "opening")
          throw new Error("actor-lifecycle.open-already-captured");
        operation.assertActive();
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-open", payload: { instance: instanceId, activationGeneration: open.activationGeneration, requestSequence: open.requestSequence, appId: input.appId, actor: input.actor, config: input.config, assets: input.assets, capabilities: input.capabilities, quotas: input.quotas } }], budget);
      },
      poll: (budget) => this.sendInstanceLifecycle(owner, [], budget),
      beginClose: () => this.beginInstanceLifecycleClose(owner),
      close: async (budget) => {
        const request = this.beginInstanceLifecycleClose(owner);
        if (owner.receipt !== null)
          throw new Error("actor-lifecycle.receipt-ack-required");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-close", payload: request }], budget);
      },
      acknowledge: async (receipt, budget, retirement) => {
        if (!owner.receipt || !actorInstanceLifecycleReceiptEquals(owner.receipt, receipt))
          throw new Error("actor-lifecycle.ack-mismatch");
        if (receipt.kind === "retired" && (owner.cancellation !== null || !owner.host || !owner.lifetime || !OwnedUiInstanceRetirement.matches(retirement, owner.host, operation, owner.lifetime)))
          throw new Error("actor-lifecycle.host-retirement-pending");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: owner.receipt } }], budget, owner.receipt);
      },
      bindHostRetirement: (participant) => {
        if (owner.host !== null || owner.lifetime === null || !OwnedUiInstance.matches(participant, operation, owner.lifetime))
          throw new Error("actor-lifecycle.host-owner-mismatch");
        owner.host = participant;
      },
      captureUiPatchAuthority: (originalTurn, patchIndex) => this.captureInstanceUiPatch(owner, originalTurn, patchIndex),
      submitUiAcknowledgement: (source, token, budget) => this.submitInstanceUiAcknowledgement(owner, source, token, budget),
      dispose: () => {
        if (owner.phase !== "complete")
          throw new Error("actor-close.native-retirement-pending");
        this.disposeActivation(owner.activation);
      },
      progress: () => ({ kind: owner.failure === null ? owner.phase : "blocked", failure: owner.failure })
    });
  }
  reserveInstanceReturn(instance, maximumResponses, grant) {
    if (!Number.isSafeInteger(maximumResponses) || maximumResponses < 1 || maximumResponses > 4294967295)
      return returnAdmission("rejected", "actor-return.capacity");
    if (!residentGrant(grant, 64))
      return returnAdmission("blocked", "actor-return.admission");
    if (instance.returnCapacity !== 0 && instance.returnCapacity !== maximumResponses || instance.activation.returned !== null && instance.activation.returned.instance !== instance)
      return returnAdmission("rejected", "actor-return.original-owner");
    let spent = 64;
    try {
      const ledger = this.#residentLedger;
      if (instance.returnPhase === "preparing") {
        const cell2 = ledger.preparedAdmission(instance);
        if (!cell2) {
          if (instance.returnFault !== NO_RETURN_FAULT)
            return returnAdmission("blocked", "actor-return.cell-handoff");
          instance.returnPhase = "empty";
          instance.returnCapacity = 0;
          return returnAdmission("pending", "actor-return.empty-admission", 64);
        }
        instance.returnCell = cell2;
        instance.returnPhase = "cell-held";
        return returnAdmission("pending", "actor-return.cell-observation", 64);
      }
      const cell = instance.returnCell;
      if (instance.returnPhase === "record-admitting" && cell) {
        const result3 = cell.result;
        instance.returnRecord = result3?.record ?? null;
        const ready = instance.returnRecord !== null && result3?.step.kind === "ready" && !cell.hasFailure && instance.returnFault === NO_RETURN_FAULT;
        instance.returnPhase = ready ? "record-held" : "rejected";
        return returnAdmission(ready ? "pending" : "rejected", "actor-return.record-observation", 64);
      }
      if (instance.returnPhase === "installing" && instance.returnRecord) {
        if (!instance.returnRecord.matchesShell(instance))
          return returnAdmission("blocked", "actor-return.parent-installation", 64);
        instance.returnPhase = "installed";
        return returnAdmission("pending", "actor-return.parent-observation", 64);
      }
      if (instance.returnFault !== NO_RETURN_FAULT) {
        if (cell && !cell.hasFailure)
          return { step: residentChild(cell.retainFailure(instance.returnFault, grant), grant), source: null };
        return returnAdmission("rejected", "actor-return.construction-fault");
      }
      if (cell?.hasFailure || instance.returnPhase === "rejected")
        return returnAdmission("rejected", "actor-return.admission-refused");
      const state7 = instance.activation.returned;
      if ((state7 !== null || instance.returnPhase === "installed") && !instance.returnRecord?.matchesLiveShell(instance))
        return returnAdmission("rejected", "actor-return.parent-not-live");
      if (instance.returnPhase === "published")
        return state7?.facade && !state7.failed ? { step: residentStep("ready", "actor-return.original-source"), source: state7.facade } : returnAdmission("rejected", "actor-return.owner-fault");
      instance.operation.assertActive();
      if (instance.inFlight)
        return returnAdmission("blocked", "actor-return.request-pending");
      if (instance.returnPhase === "empty") {
        if (!residentGrant(grant, 296))
          return returnAdmission("blocked", "actor-return.bootstrap");
        spent = 296;
        instance.returnCapacity = maximumResponses;
        instance.returnPhase = "preparing";
        const current = ledger.prepareAdmission(instance, "data", grant);
        if (current.kind === "blocked") {
          instance.returnPhase = "empty";
          instance.returnCapacity = 0;
        }
        return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "cell-held" && cell) {
        instance.returnPhase = "claiming";
        const current = ledger.claimAdmission(instance, cell, grant);
        if (current.kind === "blocked")
          instance.returnPhase = "cell-held";
        return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "claiming" && cell) {
        if (!cell.claimed)
          return returnAdmission("rejected", "actor-return.unclaimed");
        instance.returnPhase = "claimed";
        return returnAdmission("pending", "actor-return.claim-observation", 64);
      }
      if (instance.returnPhase === "claimed" && cell) {
        if (!residentGrant(grant, 264))
          return returnAdmission("blocked", "actor-return.record");
        spent = 264;
        instance.returnPhase = "record-admitting";
        const admitted20 = ledger.reserveRecord("data", returnDomainEnvelope, cell, grant);
        if (admitted20.step.kind === "blocked")
          instance.returnPhase = "claimed";
        return { step: residentChild(admitted20.step, grant), source: null };
      }
      if (instance.returnPhase === "record-held" && instance.returnRecord) {
        instance.returnPhase = "installing";
        const current = instance.returnRecord.install(instance, grant);
        if (current.kind === "blocked")
          instance.returnPhase = "record-held";
        return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "installed") {
        if (!residentGrant(grant, 320))
          return returnAdmission("blocked", "actor-return.state");
        spent = 320;
        const created = { instance, outputs: null, client: this, facade: null, origin: null, identity: null, events: null, latest: null, page: null, content: null, inFlight: false, retry: false, failed: false, fault: NO_RETURN_FAULT, cancelled: false, retired: false };
        instance.activation.returned = created;
        instance.returnPhase = "state-held";
        Object.seal(created);
        return returnAdmission("pending", "actor-return.state", 320);
      }
      if (instance.returnPhase === "state-held" && state7) {
        if (!residentGrant(grant, 256))
          return returnAdmission("blocked", "actor-return.roster");
        spent = 256;
        state7.outputs = new OwnedActorTurnOutputs(instance, instance.returnCapacity, ledger);
        instance.returnPhase = "roster-held";
        Object.freeze(state7.outputs);
        return returnAdmission("pending", "actor-return.roster", 256);
      }
      if (instance.returnPhase === "roster-held" && state7) {
        if (!residentGrant(grant, 80))
          return returnAdmission("blocked", "actor-return.facade");
        spent = 80;
        instance.returnPhase = "facade-held";
        mintCapturedReturn(state7);
        return returnAdmission("pending", "actor-return.facade", 80);
      }
      if (instance.returnPhase === "facade-held" && state7?.facade && state7.outputs && instance.returnRecord?.matchesShell(instance)) {
        instance.returnPhase = "published";
        return { step: residentStep("ready", "actor-return.publication", 64), source: state7.facade };
      }
      return returnAdmission("rejected", "actor-return.admission-phase");
    } catch (error) {
      if (instance.returnFault !== NO_RETURN_FAULT && !Object.is(instance.returnFault, error))
        throw error;
      instance.returnFault = error;
      const state7 = instance.activation.returned;
      if (state7?.instance === instance) {
        state7.failed = true;
        state7.fault = error;
      }
      return returnAdmission("rejected", "actor-return.construction-fault", spent);
    }
  }
  reserveReturnResponse(state7, grant) {
    const instance = state7.instance;
    const activation = instance.activation;
    const slot = activation.slot;
    if (!residentGrant(grant, 64))
      return residentStep("blocked", "actor-return.response-grant");
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot)
      return residentStep("rejected", "actor-return.worker-lost");
    if (state7.inFlight || instance.inFlight)
      return residentStep("blocked", "actor-return.request-pending");
    if (instance.returnPhase !== "published" || !state7.outputs || !instance.returnRecord?.matchesLiveShell(instance))
      return residentStep("rejected", "actor-return.parent-not-live");
    if (OwnedActorTurnOutput.reserved(state7.latest, instance))
      return residentStep("ready", "actor-return.response-ready");
    try {
      const current = state7.outputs.reserve(grant);
      if (current.step.kind === "ready" && current.output)
        state7.latest = current.output;
      if (current.step.kind === "rejected")
        state7.failed = true;
      return current.step;
    } catch (error) {
      state7.failed = true;
      if (state7.fault === NO_RETURN_FAULT)
        state7.fault = error;
      throw error;
    }
  }
  async sendCapturedReturn(state7, work, budget) {
    const instance = state7.instance;
    const activation = instance.activation;
    const slot = activation.slot;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot)
      throw new Error("actor-return.worker-lost");
    if (state7.inFlight || instance.inFlight)
      throw new Error("actor-return.request-pending");
    if (state7.failed && work.kind !== "cancel")
      throw new Error("actor-return.owner-fault");
    if (instance.returnPhase !== "published" || state7.outputs === null)
      throw new Error("actor-return.construction-pending");
    if (work.kind === "execute" && state7.origin !== null)
      throw new Error("actor-return.execute-already-owned");
    const execution = work.kind === "execute" || work.kind === "retry";
    if (execution) {
      instance.operation.assertActive();
      if (work.kind === "retry" && !state7.retry)
        throw new Error("actor-return.retry-not-admitted");
    } else if (state7.identity === null)
      throw new Error("actor-return.identity-pending");
    const output = state7.latest;
    if (!OwnedActorTurnOutput.reserved(output, instance))
      throw new Error("actor-return.response-admission-required");
    let requestId;
    try {
      requestId = this.nextRequestId();
    } catch (error) {
      output.cancelEmpty();
      throw error;
    }
    if (work.kind === "execute") {
      state7.origin = Object.freeze({ activationGeneration: activation.generation, requestSequence: this.requestSeq });
      state7.events = work.events;
    }
    const drive = execution ? { kind: "execute", origin: state7.origin } : { kind: "control", control: { kind: work.kind, identity: state7.identity } };
    const message = { kind: "turn", requestId, actorId: activation.actorId, activationGeneration: activation.generation, events: execution ? state7.events : [], budget, returnDrive: encodeActorReturnDrive(drive) };
    state7.inFlight = true;
    state7.retry = false;
    state7.latest = output;
    let posted = false;
    try {
      const raw = await output.run(() => this.send(slot, message, requestId, () => {
        posted = true;
      }, output));
      if (!activation.available || !slot.available || this.shards[slot.index] !== slot)
        throw new Error("actor-return.worker-lost");
      if (!(raw instanceof Uint8Array))
        throw new Error("actor-return.fixed-result-required");
      const result3 = decodeActorReturnResult(raw);
      this.acceptCapturedReturn(state7, drive, result3, output);
      return result3.kind === "page" ? Object.freeze({ kind: "page", receipt: result3.receipt }) : result3;
    } catch (error) {
      state7.retry = execution && !posted;
      if (posted)
        state7.failed = true;
      throw error;
    } finally {
      state7.inFlight = false;
    }
  }
  acceptCapturedReturn(state7, drive, result3, output) {
    if (result3.kind === "protocolFault") {
      state7.failed = true;
      return;
    }
    const identity = result3.kind === "page" ? result3.receipt.identity : result3.kind === "pending" || result3.kind === "retired" ? result3.identity : result3.kind === "control" ? result3.control.kind === "inputAck" ? result3.control.receipt.identity : result3.control.identity : null;
    const origin = result3.kind === "refused" ? result3.origin : identity.origin;
    if (!state7.origin || !sameReturnOrigin(state7.origin, origin))
      throw new Error("actor-return.foreign-origin");
    if (drive.kind === "execute") {
      if (result3.kind === "control")
        throw new Error("actor-return.unexpected-control");
      if (result3.kind === "refused") {
        state7.retry = true;
        return;
      }
    } else {
      if (result3.kind === "refused")
        throw new Error("actor-return.unexpected-refusal");
      if (drive.control.kind !== "poll" && result3.kind !== "control")
        throw new Error("actor-return.control-result-required");
      if (result3.kind === "control") {
        const expected = encodeActorReturnDrive(drive);
        const actual = encodeActorReturnDrive({ kind: "control", control: result3.control });
        if (expected.length !== actual.length || expected.some((byte, index) => byte !== actual[index]))
          throw new Error("actor-return.foreign-control");
      }
    }
    if (identity) {
      if (state7.identity === null) {
        if (identity.returnSequence <= state7.instance.activation.lastReturnSequence)
          throw new Error("actor-return.stale-sequence");
        state7.identity = identity;
        state7.instance.activation.lastReturnSequence = identity.returnSequence;
      } else if (!sameReturnIdentity(state7.identity, identity))
        throw new Error("actor-return.foreign-identity");
    }
    if (result3.kind === "page") {
      if (state7.page !== null || state7.cancelled || state7.retired)
        throw new Error("actor-return.page-already-owned");
      state7.page = mintCapturedReturnPage(state7, output, result3.receipt, result3.page);
    } else if (result3.kind === "control" && result3.control.kind === "cancel" && (result3.outcome === "accepted" || result3.outcome === "duplicate"))
      state7.cancelled = true;
    else if (result3.kind === "retired") {
      if (state7.page !== null)
        throw new Error("actor-return.input-retirement-pending");
      state7.retired = true;
    }
  }
  beginInstanceLifecycleClose(owner) {
    if (owner.close)
      return owner.close;
    if (!owner.activation.available || !owner.activation.slot.available || this.shards[owner.activation.slot.index] !== owner.activation.slot) {
      owner.failure = "worker-lost";
      throw new Error("actor-lifecycle.worker-lost");
    }
    if (owner.lifetime === null || owner.phase !== "open")
      throw new Error("actor-lifecycle.capture-pending");
    if (owner.activation.operationGeneration >= 0xffffffffffffffffn)
      throw new Error("actor-lifecycle.operation-generation-exhausted");
    this.nextRequestId();
    owner.close = Object.freeze({ kind: "close", lifetime: owner.lifetime, requestSequence: this.requestSeq });
    owner.phase = "closing";
    owner.activation.close = owner;
    owner.activation.operationGeneration += 1n;
    const ledger = this.outstandingEffectsByActor.get(owner.activation.actorId);
    if (ledger?.activation === owner.activation) {
      this.outstandingEffectsByActor.delete(owner.activation.actorId);
      owner.cancellation = ledger;
    }
    return owner.close;
  }
  async sendInstanceLifecycle(owner, events, budget, acknowledged) {
    const { activation } = owner;
    const { slot } = activation;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) {
      owner.failure = "worker-lost";
      throw new Error("actor-lifecycle.worker-lost");
    }
    if (activation.returned !== null)
      throw new Error("actor-return.retirement-pending");
    if (owner.phase === "complete")
      throw new Error("actor-lifecycle.already-complete");
    if (owner.inFlight)
      throw new Error("actor-lifecycle.turn-already-pending");
    const requestId = this.nextRequestId();
    owner.inFlight = true;
    let posted = false;
    try {
      if (owner.cancellation) {
        this.cancelOneEffect(owner.cancellation);
        if (owner.cancellation.head === null)
          owner.cancellation = null;
      }
      const result3 = await this.send(slot, { kind: "turn", requestId, actorId: activation.actorId, activationGeneration: activation.generation, events, budget }, requestId, () => {
        posted = true;
      });
      this.recordInstanceTurn(owner, result3);
      if (!activation.available || !slot.available || this.shards[slot.index] !== slot) {
        owner.failure = "worker-lost";
        throw new Error("actor-lifecycle.worker-lost");
      }
      const status = result3 !== null && typeof result3 === "object" ? Reflect.get(result3, "status") : undefined;
      const admitted20 = status !== null && typeof status === "object" && ["idle", "more-work", "checkpoint-ready"].includes(Reflect.get(status, "tag"));
      try {
        this.acceptInstanceLifecycleResult(owner, result3, admitted20 ? acknowledged : undefined);
      } catch (error) {
        owner.failure = "invalid-receipt";
        throw error;
      }
      if (!admitted20) {
        owner.interruptedTurn = result3;
        owner.failure = "worker-refused";
        throw new Error(acknowledged ? "actor-lifecycle.ack-not-admitted" : "actor-lifecycle.turn-not-admitted");
      }
      owner.failure = null;
      return result3;
    } catch (error) {
      owner.failure ??= !activation.available || !slot.available || this.shards[slot.index] !== slot ? "worker-lost" : posted ? "worker-refused" : "transport-refused";
      throw error;
    } finally {
      owner.inFlight = false;
    }
  }
  recordInstanceTurn(owner, result3) {
    if (result3 !== null && typeof result3 === "object")
      this.instanceTurns.set(result3, { owner, patches: new WeakMap });
  }
  captureInstanceUiPatch(owner, turn, patchIndex) {
    const captured = this.instanceTurns.get(turn);
    if (!captured || captured.owner !== owner || owner.lifetime === null)
      throw new Error("actor-lifecycle.foreign-turn");
    const patches = Reflect.get(turn, "uiPatches");
    if (!Array.isArray(patches) || !Number.isSafeInteger(patchIndex) || patchIndex < 0 || patchIndex >= patches.length)
      throw new Error("actor-lifecycle.patch-index");
    const wire = Reflect.get(turn, "uiPatchReceipt");
    const decoded = wire == null ? null : decodeActorUiPatchReceipt(wire);
    validateActorUiPatchPairing(patches.length, decoded);
    if (!decoded || !actorInstanceLifetimeEquals(decoded.lifetime, owner.lifetime))
      throw new Error("actor-ui-patch.lifetime-mismatch");
    const patch = patches[patchIndex];
    if (patch === null || typeof patch !== "object")
      throw new Error("actor-lifecycle.patch-envelope");
    const existing = captured.patches.get(patch);
    if (existing) {
      if (!actorUiPatchReceiptEquals(existing.value.receipt, decoded))
        throw new Error("actor-ui-patch.receipt-mismatch");
      return existing;
    }
    if (decoded.patchSequence <= owner.lastPatchSequence)
      throw new Error("actor-ui-patch.duplicate-sequence");
    const surface = Reflect.get(patch, "surface");
    const operations = Reflect.get(patch, "ops");
    const revision = Reflect.get(patch, "revision");
    const base = Reflect.get(patch, "baseRevision");
    const exactRevision = (value2) => {
      if (typeof value2 === "bigint" && value2 >= 0n && value2 <= BigInt(Number.MAX_SAFE_INTEGER))
        return Number(value2);
      if (typeof value2 === "number" && Number.isSafeInteger(value2) && value2 >= 0)
        return value2;
      throw new Error("actor-lifecycle.patch-revision");
    };
    if (!surface || typeof surface !== "object" || Reflect.get(surface, "instance") !== owner.lifetime.instanceId || !Array.isArray(operations))
      throw new Error("actor-lifecycle.patch-envelope");
    const name = Reflect.get(surface, "surface");
    if (typeof name !== "string" || name.length > 512 || new TextEncoder().encode(name).length > 512)
      throw new Error("actor-lifecycle.patch-surface");
    const receipt = Object.freeze({ lifetime: Object.freeze(decoded.lifetime), patchSequence: decoded.patchSequence });
    const value = Object.freeze({ activation: owner.operation, lifetime: owner.lifetime, receipt, surface: name, revision: exactRevision(revision), baseRevision: exactRevision(base), operationCount: operations.length });
    const authority = mintNativePatch({ owner, turn, patch, operations, value, ordinal: 0, read: false, original: undefined, input: null, token: null, submission: null });
    captured.patches.set(patch, authority);
    owner.lastPatchSequence = receipt.patchSequence;
    return authority;
  }
  async submitInstanceUiAcknowledgement(owner, source, token, budget) {
    if (!owner.lifetime || !OwnedNativeUiPatchAuthority.matches(source, owner.operation, owner.lifetime) || !OwnedUiPatchAcknowledgement.matches(token, source))
      throw new Error("actor-lifecycle.ui-ack-mismatch");
    const state7 = nativePatchState(source);
    const value = token.value;
    if (state7.owner !== owner || state7.token !== null && state7.token !== token || value.actor !== owner.activation.actorId || value.instance !== owner.lifetime.instanceId || value.surface !== state7.value.surface || value.revision !== state7.value.revision || !actorInstanceLifetimeEquals(value.lifetime, owner.lifetime) || !actorUiPatchReceiptEquals(value.receipt, state7.value.receipt))
      throw new Error("actor-lifecycle.ui-ack-mismatch");
    if (!source.inputRetired)
      throw new Error("actor-lifecycle.ui-input-pending");
    if (state7.submission)
      return state7.submission;
    state7.token = token;
    state7.submission = (async () => {
      const result3 = await this.sendInstanceLifecycle(owner, [{ kind: "patch-ack", payload: { receipt: state7.value.receipt, surface: { instance: owner.lifetime.instanceId, surface: state7.value.surface }, revision: BigInt(state7.value.revision) } }], budget);
      const status = result3 !== null && typeof result3 === "object" ? Reflect.get(result3, "status") : undefined;
      if (!status || typeof status !== "object" || !["idle", "more-work"].includes(Reflect.get(status, "tag")))
        throw new Error("actor-lifecycle.ui-ack-not-admitted");
      return Object.freeze({ receipt: mintNativeSubmission(source, token), result: result3 });
    })();
    try {
      return await state7.submission;
    } catch (error) {
      state7.submission = null;
      throw error;
    }
  }
  acceptInstanceLifecycleResult(owner, result3, acknowledged) {
    const wire = result3 && typeof result3 === "object" ? Reflect.get(result3, "lifecycleReceipt") : undefined;
    let incoming = null;
    if (wire !== undefined && wire !== null) {
      const decoded = decodeActorInstanceLifecycle(wire);
      if (decoded.kind !== "captured" && decoded.kind !== "accepted" && decoded.kind !== "retired")
        throw new Error("actor-lifecycle.receipt-required");
      incoming = Object.freeze({ ...decoded, lifetime: Object.freeze(decoded.lifetime) });
    }
    let phase = owner.phase;
    let pending2 = owner.receipt;
    if (acknowledged && (!incoming || !actorInstanceLifecycleReceiptEquals(acknowledged, incoming))) {
      pending2 = null;
      phase = acknowledged.kind === "captured" ? "open" : acknowledged.kind === "retired" ? "complete" : "accepted";
    }
    if (incoming) {
      if (phase === "complete" || pending2 !== null && !actorInstanceLifecycleReceiptEquals(pending2, incoming))
        throw new Error("actor-lifecycle.receipt-mismatch");
      if (incoming.kind === "captured") {
        if (!actorInstanceCapturedReceiptMatches(owner.open, incoming) || owner.lifetime !== null && !actorInstanceLifetimeEquals(owner.lifetime, incoming.lifetime) || owner.lifetime === null && incoming.lifetime.guestLifetime <= owner.activation.lastGuestLifetime)
          throw new Error("actor-lifecycle.receipt-mismatch");
        phase = "captured";
      } else {
        if (!owner.close || !actorInstanceCloseReceiptMatches(owner.close, owner.accepted, incoming))
          throw new Error("actor-lifecycle.receipt-mismatch");
        phase = incoming.kind;
      }
      pending2 = incoming;
    }
    if (incoming?.kind === "captured") {
      owner.lifetime = incoming.lifetime;
      owner.activation.lastGuestLifetime = incoming.lifetime.guestLifetime;
    }
    if (incoming?.kind === "accepted")
      owner.accepted = incoming;
    owner.receipt = pending2;
    owner.phase = phase;
    if (phase === "complete") {
      if (owner.activation.instance === owner)
        owner.activation.instance = null;
      if (owner.activation.close === owner)
        owner.activation.close = null;
      this.instanceLifecycles.delete(owner.open.requestSequence);
    }
  }
  async turn(actorId, events, budget, commandPage) {
    if (!this.actorShard.has(actorId))
      throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    return this.captureActorActivation(actorId).turn(events, budget, commandPage);
  }
  async envelope(shardEnvelope) {
    const slot = this.requireShard(shardEnvelope.to);
    const activation = this.captureActorActivation(shardEnvelope.to);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, activationGeneration: activation.activationGeneration, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }
  async grant(actorId, budget, envelopes) {
    const slot = this.requireShard(actorId);
    const activation = this.captureActorActivation(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, activationGeneration: activation.activationGeneration, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
  }
  async startJob(actorId, job, jobKind, input) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }
  async stepJob(actorId, job, budget) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }
  cancelJob(actorId, job) {
    const slot = this.requireShard(actorId);
    this.send(slot, { kind: "cancelJob", actorId, job }, null);
  }
  async takeSegmentedDownloadChunk(actorId, instanceId, operationId) {
    if (!Number.isSafeInteger(instanceId) || instanceId < 0 || typeof operationId !== "bigint" || operationId <= 0n || operationId > MAX_SEGMENTED_DOWNLOAD_OPERATION_ID)
      throw new Error("segmented-download-authority-invalid");
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const value = await this.send(slot, { kind: "takeSegmentedDownloadChunk", requestId, actorId, instanceId, operationId }, requestId);
    if (value === undefined || value === null)
      return;
    if (Object.prototype.toString.call(value) !== "[object Uint8Array]")
      throw new Error("segmented-download-transport-type");
    const chunk = value;
    if (chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES)
      throw new Error("segmented-download-transport-limit");
    return chunk;
  }
  async checkpoint(actorId) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }
  async restore(actorId, state7) {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send(slot, { kind: "restore", requestId, actorId, state: state7 }, requestId);
  }
  dispose(actorId) {
    const activation = this.actorActivations.get(actorId);
    if (activation) {
      this.disposeActivation(activation);
      return;
    }
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined)
      return;
    this.shards[shardIndex].actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }
  disposeActivation(activation) {
    if (activation.teardownPosted)
      return;
    if (activation.instance !== null || activation.close !== null)
      throw new Error("actor-close.native-retirement-pending");
    if (activation.returned !== null)
      throw new Error("actor-return.retirement-pending");
    const { actorId, slot } = activation;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot)
      throw new Error("actor-close.worker-lost");
    activation.operationsAllowed = false;
    slot.worker.postMessage({ kind: "dispose", actorId, activationGeneration: activation.generation });
    activation.teardownPosted = true;
    activation.available = false;
    if (this.outstandingEffectsByActor.get(actorId)?.activation === activation)
      this.abortOutstandingEffects(actorId);
    this.rejectActorPending(slot, actorId, new Error(`ShardClient actor disposed: ${actorId}`));
    slot.actorIds.delete(actorId);
    if (this.actorActivations.get(actorId) === activation) {
      const route = this.actorShard.get(actorId);
      if (route !== undefined)
        this.shards[route].actorIds.delete(actorId);
      this.actorShard.delete(actorId);
      this.actorActivations.delete(actorId);
    }
  }
  requireShard(actorId) {
    const index = this.actorShard.get(actorId);
    if (index === undefined)
      throw new Error(`[DEBUG] ShardClient: actor ${actorId} is not activated on any shard`);
    return this.shards[index];
  }
  handleInboundFrame(activation, frame) {
    if (frame.kind !== "Envelope")
      return;
    if (frame.envelope.to !== "kernel" || frame.envelope.from.kind !== "actor" || frame.envelope.from.id !== activation.actorId)
      return;
    const payload = frame.envelope.payload;
    if (payload.kind !== "effect-request")
      return;
    const request = payload.payload;
    this.handleEffectRequest(activation, request.effect, request.requestId, request.params);
  }
  handleEffectRequest(activation, effect, requestId, params) {
    const actorId = activation.actorId;
    const outstanding = this.outstandingEffectsByActor.get(actorId) ?? { activation, requests: new Map, head: null, tail: null };
    if (outstanding.activation !== activation || outstanding.requests.has(requestId))
      return;
    if (outstanding.requests.size >= this.maxOutstandingEffectsPerActor) {
      const breach = { quota: "outstandingRequests", limit: this.maxOutstandingEffectsPerActor, actual: outstanding.requests.size };
      this.replyEffectError(activation, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.onHostEffect) {
      this.replyEffectError(activation, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController;
    const entry = { activation, controller, requestId, previous: outstanding.tail, next: null };
    if (outstanding.tail)
      outstanding.tail.next = entry;
    else
      outstanding.head = entry;
    outstanding.tail = entry;
    outstanding.requests.set(requestId, entry);
    this.outstandingEffectsByActor.set(actorId, outstanding);
    this.onHostEffect(actorId, effect, params, controller.signal).then((value) => {
      if (this.settleEffect(requestId, entry))
        this.replyEffectComplete(activation, requestId, value);
    }, (error) => {
      if (this.settleEffect(requestId, entry))
        this.replyEffectError(activation, requestId, error instanceof Error ? error.message : String(error));
    });
  }
  settleEffect(requestId, entry) {
    const actorId = entry.activation.actorId;
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (outstanding?.activation !== entry.activation || outstanding.requests.get(requestId) !== entry)
      return false;
    this.removeEffect(outstanding, entry);
    if (outstanding.requests.size === 0)
      this.outstandingEffectsByActor.delete(actorId);
    return !entry.controller.signal.aborted && this.activationIsActive(entry.activation);
  }
  abortOutstandingEffects(actorId) {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding)
      return;
    this.outstandingEffectsByActor.delete(actorId);
    while (outstanding.head)
      this.cancelOneEffect(outstanding);
  }
  removeEffect(ledger, entry) {
    if (entry.previous)
      entry.previous.next = entry.next;
    else
      ledger.head = entry.next;
    if (entry.next)
      entry.next.previous = entry.previous;
    else
      ledger.tail = entry.previous;
    ledger.requests.delete(entry.requestId);
    entry.previous = null;
    entry.next = null;
  }
  cancelOneEffect(ledger) {
    const entry = ledger.head;
    if (!entry)
      return;
    entry.controller.abort();
    this.removeEffect(ledger, entry);
  }
  postEffectReply(activation, kind, innerPayload) {
    if (!this.activationIsActive(activation))
      return;
    const { actorId, slot, generation: generation2 } = activation;
    this.effectReplySeq += 1;
    const frame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } }
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.nextRequestId(), actorId, activationGeneration: generation2, frame });
  }
  replyEffectComplete(activation, requestId, value) {
    this.postEffectReply(activation, "effect-complete", { requestId, value });
  }
  replyEffectError(activation, requestId, message) {
    this.postEffectReply(activation, "effect-error", { requestId, message });
  }
  recordHeartbeat(slot, turnSeq, atMs) {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }
  pollHeartbeatSab(nowMs = this.now()) {
    if (!this.heartbeatSabView)
      return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      if (seq !== slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }
  checkHeartbeats(nowMs = this.now()) {
    for (const slot of this.shards) {
      const pendingSince = slot.heartbeat.oldestPendingStartedAtMs;
      if (pendingSince === null)
        continue;
      if (slot.heartbeat.lastHeartbeatAtMs >= pendingSince)
        continue;
      const silentForMs = nowMs - pendingSince;
      if (silentForMs <= this.heartbeatTimeoutMs)
        continue;
      if (nowMs - slot.heartbeat.lastMissCountedAtMs < this.heartbeatTimeoutMs)
        continue;
      slot.heartbeat.missedCount += 1;
      slot.heartbeat.lastMissCountedAtMs = nowMs;
      if (slot.heartbeat.missedCount >= HEARTBEAT_MISSED_LIMIT) {
        const actorIds = [...slot.actorIds];
        this.terminate(slot.index);
        this.rebuild(slot.index);
        this.onShardLost?.(slot.index, actorIds);
      }
    }
  }
  startWatchdog(intervalMs = this.watchdogIntervalMs) {
    if (this.watchdogHandle !== null)
      return;
    this.watchdogHandle = setInterval(() => {
      this.pollHeartbeatSab();
      this.checkHeartbeats();
    }, intervalMs);
  }
  stopWatchdog() {
    if (this.watchdogHandle === null)
      return;
    clearInterval(this.watchdogHandle);
    this.watchdogHandle = null;
  }
  shardMetricsSamples(nowMs = this.now()) {
    return this.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
  terminate(index) {
    const slot = this.shards[index];
    if (!slot)
      throw new Error(`[DEBUG] ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.failShard(slot, new Error(`shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }
  rebuild(index) {
    const old = this.shards[index];
    if (!old)
      throw new Error(`[DEBUG] ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds)
      this.actorShard.delete(actorId);
    this.shards[index] = this.spawnShard(index);
  }
  disposeAll() {
    this.stopWatchdog();
    for (const slot of this.shards) {
      this.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
if (undefined) {
  let prepareWorkerFixture = function(client, rows) {}, prepareResidentFixture = function(client, ledger, bytes) {}, harness = function(shardCount = 2, extra) {}, bindFixtureHost = function(lease) {}, fixtureRetirement = function(lease) {}, makeEnvelope = function(to, lane, seq, kind = "wake") {}, makeEffectRequestFrame = function(actorId, effect, requestId, params, activationGeneration = 1n) {}, findEffectReply = function(sent, requestId, kind) {}, flushMicrotasks = function() {};
  async function workerPreparationFixture() {}
  async function fixtureResidentPool(client, ledger) {}
  async function fixtureResidentScope(pool, ledger, lease) {}
  async function fixtureResidentPayload(scope, ledger, field) {}
  async function fixtureResidentBuilder(ledger, field, resident) {}
  async function answerLifecycle(worker, pending2, receipt) {}
  async function captureFixtureInstance(client, worker, actorId, instanceId = 7, guestLifetime = 13n) {}
  async function retireFixtureInstance(worker, lease) {}
  async function fixtureOutputReservation(queue) {}
  async function activateActor(client, workers, actorId, shardIndex = 0) {}
}

/* ../../../../../../../../../🔨️modules/🎭️actor/📬️mailbox/🟦️.ts */
var MAILBOX_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function laneRank(lane) {
  return MAILBOX_LANE_ORDER.indexOf(lane);
}
function createBoundedMailbox(capacity) {
  const lanes = MAILBOX_LANE_ORDER.map(() => []);
  let len = 0;
  return {
    enqueue(envelope) {
      const incomingRank = laneRank(envelope.lane);
      if (envelope.coalesce !== undefined) {
        const lane = lanes[incomingRank];
        const existingIndex = lane.findIndex((queued) => queued.coalesce === envelope.coalesce);
        if (existingIndex !== -1) {
          lane[existingIndex] = envelope;
          return { kind: "coalesced" };
        }
      }
      if (len >= capacity) {
        let victimRank = -1;
        for (let rank = MAILBOX_LANE_ORDER.length - 1;rank > incomingRank; rank--) {
          if (lanes[rank].length > 0) {
            victimRank = rank;
            break;
          }
        }
        if (victimRank === -1)
          return { kind: "rejected" };
        lanes[victimRank].shift();
        len -= 1;
        lanes[incomingRank].push(envelope);
        len += 1;
        return { kind: "dropped", lane: MAILBOX_LANE_ORDER[victimRank] };
      }
      lanes[incomingRank].push(envelope);
      len += 1;
      return { kind: "accept" };
    },
    popNext() {
      for (const lane of lanes) {
        if (lane.length > 0) {
          len -= 1;
          return lane.shift();
        }
      }
      return;
    },
    get length() {
      return len;
    },
    get isEmpty() {
      return len === 0;
    }
  };
}
if (undefined) {}

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts */
var LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
function freshLaneCounts() {
  return { Interactive: 0, UserVisible: 0, Background: 0, Maintenance: 0 };
}

class TurnScheduler {
  mailboxes = new Map;
  laneCounts = new Map;
  busyActors = new Set;
  options;
  pumpScheduled = false;
  constructor(options) {
    this.options = options;
  }
  enqueue(actorId, turn) {
    const mailbox = this.mailboxFor(actorId);
    const backpressure = mailbox.enqueue({ lane: turn.lane, coalesce: turn.coalesce, payload: turn.payload });
    this.applyLaneDelta(actorId, turn.lane, backpressure);
    if (backpressure.kind !== "rejected")
      this.schedulePump();
    return backpressure;
  }
  mailboxFor(actorId) {
    let mailbox = this.mailboxes.get(actorId);
    if (!mailbox) {
      mailbox = createBoundedMailbox(this.options.mailboxCapacity);
      this.mailboxes.set(actorId, mailbox);
      this.laneCounts.set(actorId, freshLaneCounts());
    }
    return mailbox;
  }
  applyLaneDelta(actorId, incomingLane, backpressure) {
    const counts = this.laneCounts.get(actorId);
    if (backpressure.kind === "accept") {
      counts[incomingLane] += 1;
    } else if (backpressure.kind === "dropped") {
      counts[backpressure.lane] -= 1;
      counts[incomingLane] += 1;
    }
  }
  cancelQueued(actorId, onCancelled) {
    const mailbox = this.mailboxes.get(actorId);
    if (!mailbox)
      return 0;
    const counts = this.laneCounts.get(actorId);
    let cancelled = 0;
    let envelope;
    while ((envelope = mailbox.popNext()) !== undefined) {
      counts[envelope.lane] -= 1;
      onCancelled?.(envelope.payload);
      cancelled += 1;
    }
    return cancelled;
  }
  teardownActor(actorId, onCancelled) {
    const cancelled = this.cancelQueued(actorId, onCancelled);
    this.mailboxes.delete(actorId);
    this.laneCounts.delete(actorId);
    return cancelled;
  }
  isBusy(actorId) {
    return this.busyActors.has(actorId);
  }
  pendingCount(actorId) {
    return this.mailboxes.get(actorId)?.length ?? 0;
  }
  schedulePump() {
    if (this.pumpScheduled)
      return;
    this.pumpScheduled = true;
    queueMicrotask(() => {
      this.pumpScheduled = false;
      this.pump();
    });
  }
  pickNextReadyActor() {
    for (const lane of LANE_ORDER) {
      for (const actorId of this.mailboxes.keys()) {
        if (this.busyActors.has(actorId))
          continue;
        const counts = this.laneCounts.get(actorId);
        if (counts && counts[lane] > 0)
          return actorId;
      }
    }
    return;
  }
  pump() {
    for (;; ) {
      const actorId = this.pickNextReadyActor();
      if (actorId === undefined)
        return;
      const mailbox = this.mailboxes.get(actorId);
      const envelope = mailbox.popNext();
      if (!envelope)
        continue;
      this.laneCounts.get(actorId)[envelope.lane] -= 1;
      this.busyActors.add(actorId);
      const budget = this.options.budgetFor(actorId);
      this.options.runTurn(actorId, envelope.payload, budget).catch((error) => this.options.onTurnError?.(actorId, error)).finally(() => {
        this.busyActors.delete(actorId);
        this.schedulePump();
      });
    }
  }
}
if (undefined) {
  let deferred = function() {}, harness = function(mailboxCapacity = 10) {};
}

/* ../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts */
class OsTransient {
  boxes = new Map;
  maps = new Map;
  sets = new Map;
  weakMaps = new Map;
  box(key, init) {
    let box = this.boxes.get(key);
    if (!box) {
      box = { current: init };
      this.boxes.set(key, box);
    }
    return box;
  }
  map(key) {
    let map = this.maps.get(key);
    if (!map) {
      map = new Map;
      this.maps.set(key, map);
    }
    return map;
  }
  set(key) {
    let set = this.sets.get(key);
    if (!set) {
      set = new Set;
      this.sets.set(key, set);
    }
    return set;
  }
  weakMap(key) {
    let map = this.weakMaps.get(key);
    if (!map) {
      map = new WeakMap;
      this.weakMaps.set(key, map);
    }
    return map;
  }
  reset() {
    this.boxes.clear();
    this.maps.clear();
    this.sets.clear();
    this.weakMaps.clear();
  }
}
var defaultOsTransient = new OsTransient;
async function fetchDescriptorManifest(pluginId, moduleUrl, signal) {
  signal?.throwIfAborted();
  const path = moduleUrl.split(/[?#]/u)[0];
  const descriptorUrl = path.slice(0, path.lastIndexOf("/") + 1) + "🔣️.json";
  const fault = (code, detail) => new SemioFaultError({
    origin: "os",
    code,
    severity: "error",
    message: `${code}: ${detail}`,
    scope: { pluginId },
    retryable: true
  });
  const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
  signal?.throwIfAborted();
  if (!response.ok)
    throw fault("plugin.descriptor-unavailable", `${descriptorUrl} (HTTP ${response.status})`);
  if (response.headers?.get?.("content-type")?.toLowerCase().includes("text/html"))
    throw fault("plugin.descriptor-invalid", `${descriptorUrl} returned HTML`);
  let descriptor;
  try {
    descriptor = await response.json();
  } catch {
    signal?.throwIfAborted();
    throw fault("plugin.descriptor-invalid", `${descriptorUrl} is not JSON`);
  }
  signal?.throwIfAborted();
  const manifest = descriptor && typeof descriptor === "object" && "manifest" in descriptor ? descriptor.manifest : undefined;
  if (!manifest || typeof manifest !== "object" || !("pluginId" in manifest) || typeof manifest.pluginId !== "string")
    throw fault("plugin.descriptor-invalid", "missing manifest owner");
  if (manifest.pluginId !== pluginId)
    throw fault("plugin.descriptor-identity-mismatch", `expected ${pluginId}, received ${manifest.pluginId}`);
  if (!("apps" in manifest) || !Array.isArray(manifest.apps))
    throw fault("plugin.descriptor-invalid", "missing app roster");
  return manifest;
}
function createTurnOutcomeBroadcast() {
  const subscribers = new Set;
  return {
    push: (value) => {
      for (const subscriber of subscribers) {
        if (subscriber.resolve) {
          const resolve = subscriber.resolve;
          subscriber.resolve = null;
          resolve({ value, done: false });
        } else {
          subscriber.queue.push(value);
        }
      }
    },
    complete: () => {
      for (const subscriber of subscribers)
        subscriber.resolve?.({ value: undefined, done: true });
      subscribers.clear();
    },
    stream: {
      [Symbol.asyncIterator]() {
        const subscriber = { queue: [], resolve: null };
        subscribers.add(subscriber);
        return {
          next: () => {
            if (subscriber.queue.length > 0)
              return Promise.resolve({ value: subscriber.queue.shift(), done: false });
            return new Promise((resolve) => {
              subscriber.resolve = resolve;
            });
          },
          return: () => {
            subscribers.delete(subscriber);
            return Promise.resolve({ value: undefined, done: true });
          }
        };
      }
    }
  };
}
if (undefined) {}
function expandPluginRegistry(plugins, primaryPluginId, hostMode = false) {
  if (hostMode || !primaryPluginId)
    return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry]));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  const selected = new Map;
  const queue = [...primaryEntries, ...contributorEntries];
  for (const entry of queue)
    selected.set(entry.pluginId, entry);
  for (let index = 0;index < queue.length; index++) {
    const entry = queue[index];
    for (const dependency of entry.dependencies ?? []) {
      if (selected.has(dependency.pluginId))
        continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry)
        continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}
function dependsOnToPluginDependencies(dependsOn) {
  return dependsOn?.map((pluginId) => ({ pluginId, version: "*" }));
}
if (undefined) {}
class SemioFaultError extends Error {
  fault;
  constructor(fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}
var MERGE_POLICY_ORDER = ["LaissezFaire", "Normal", "Vigilant"];
function mergePolicyAsU8(policy) {
  return MERGE_POLICY_ORDER.indexOf(policy);
}
var CONFLICT_RESOLUTION_ORDER = ["accept", "discard"];
function conflictResolutionAsU8(resolution) {
  return CONFLICT_RESOLUTION_ORDER.indexOf(resolution);
}
function relayPluginBackboneOutbound(uri, message) {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}
globalThis.__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;
function pluginBackboneDocumentIdFromUri(uri) {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}
var pluginBackboneRoutes = new Map;
function intersectCapabilityGrants(granted3, requested) {
  const grantedIds = new Set(granted3.map((grant) => grant.id));
  return requested.filter((request) => grantedIds.has(request.id));
}
function defaultGuestSlimAssetFetcher(moduleUrl) {
  const vendorUrl = moduleUrl.split(/[?#]/)[0].replace(/\/[^/]+\/[^/]+\.js$/, "/🪞️vendor/🔤️guestslim-typst-fonts.bin");
  return fetch(vendorUrl).then((response) => {
    if (!response.ok)
      throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  }).then((buffer) => [["guestslim-typst-fonts", buffer]]);
}
var DEFAULT_MAX_RESIDENT_ACTORS = 24;
var MIN_MAX_RESIDENT_ACTORS = 4;
var MAX_MAX_RESIDENT_ACTORS = 96;
var RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB = 6;
var BYTES_PER_RESIDENT_ACTOR = 64 * 1024 * 1024;
function clampResidentActors(value) {
  return Math.min(MAX_MAX_RESIDENT_ACTORS, Math.max(MIN_MAX_RESIDENT_ACTORS, Math.round(value)));
}
function defaultMemoryProbe() {
  const nav = globalThis.navigator;
  const perf = globalThis.performance;
  return { deviceMemoryGiB: nav?.deviceMemory, jsHeapSizeLimitBytes: perf?.memory?.jsHeapSizeLimit };
}
function residentActorCapFromMemory(reading, fallback = DEFAULT_MAX_RESIDENT_ACTORS) {
  if (typeof reading.deviceMemoryGiB === "number" && reading.deviceMemoryGiB > 0)
    return clampResidentActors(reading.deviceMemoryGiB * RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB);
  if (typeof reading.jsHeapSizeLimitBytes === "number" && reading.jsHeapSizeLimitBytes > 0)
    return clampResidentActors(reading.jsHeapSizeLimitBytes / BYTES_PER_RESIDENT_ACTOR);
  return fallback;
}
var DEFAULT_TURN_MAILBOX_CAPACITY = 32;

class ActivationRegistry {
  manifests = new Map;
  resident = new Map;
  residencyOrder = [];
  checkpoints = new Map;
  actorPlugin = new Map;
  actorGeneration = new Map;
  extensionsByParent = new Map;
  extensionChildren = new Map;
  shardClient;
  defaultBudget;
  maxResidentActors;
  fetchAssets;
  assetsPromise = null;
  now;
  lastRuntimeMetricsPublishMs = null;
  turnScheduler;
  onTurnResult;
  stopMetricsPublisher;
  metricsBus = new EventTarget;
  constructor(options) {
    this.shardClient = options.shardClient;
    this.defaultBudget = options.defaultBudget;
    this.maxResidentActors = options.maxResidentActors ?? residentActorCapFromMemory((options.memoryProbe ?? defaultMemoryProbe)());
    this.fetchAssets = options.fetchAssets ?? defaultGuestSlimAssetFetcher;
    this.now = options.now ?? (() => Date.now());
    this.onTurnResult = options.onTurnResult ?? (() => {});
    const onTurnError = options.onTurnError ?? ((actorId, error) => console.error(`[DEBUG] ActivationRegistry: turn failed for ${actorId}`, error));
    this.turnScheduler = new TurnScheduler({
      mailboxCapacity: options.turnMailboxCapacity ?? DEFAULT_TURN_MAILBOX_CAPACITY,
      budgetFor: () => this.defaultBudget,
      runTurn: (actorId, payload, budget) => this.runQueuedTurn(actorId, payload, budget),
      onTurnError
    });
    this.stopMetricsPublisher = options.autoStartMetricsPublisher === true ? this.startRuntimeMetricsPublisher((topic, snapshot) => this.metricsBus.dispatchEvent(new CustomEvent(topic, { detail: snapshot }))) : () => {};
  }
  registerManifest(entry) {
    this.manifests.set(entry.pluginId, entry);
  }
  registerCatalog(catalog) {
    for (const target of catalog.plugins)
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.moduleUrl(target.pluginId), caps: [] });
    for (const target of catalog.extensions) {
      this.registerManifest({ pluginId: target.pluginId, moduleUrl: catalog.extensionModuleUrl(target.pluginId), caps: [] });
      const parentId = target.dependsOn?.[0];
      if (!parentId)
        continue;
      const siblings = this.extensionsByParent.get(parentId) ?? [];
      siblings.push(target.pluginId);
      this.extensionsByParent.set(parentId, siblings);
    }
  }
  manifestFor(pluginId) {
    return this.manifests.get(pluginId);
  }
  loadAssets(moduleUrl) {
    this.assetsPromise ??= this.fetchAssets(moduleUrl).catch((error) => {
      console.warn("[DEBUG] ActivationRegistry: guestSlim asset fetch failed; affected actors render without it", error);
      this.assetsPromise = null;
      return [];
    });
    return this.assetsPromise;
  }
  markResident(actorId, pluginId) {
    this.resident.set(actorId, { actorId, pluginId });
    this.actorPlugin.set(actorId, pluginId);
    this.touch(actorId);
  }
  touch(actorId) {
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    this.residencyOrder.push(actorId);
  }
  async activate(pluginId, actorId, _reason) {
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.activate: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    this.markResident(actorId, pluginId);
    await this.activateExtensionsOf(pluginId, actorId);
  }
  async activateExtensionsOf(pluginId, parentActorId) {
    const extensionIds = this.extensionsByParent.get(pluginId);
    if (!extensionIds || extensionIds.length === 0)
      return;
    const parentCaps = this.manifests.get(pluginId)?.caps ?? [];
    const children = [];
    for (const extensionId of extensionIds) {
      const manifest = this.manifests.get(extensionId);
      if (!manifest) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} has no registered manifest, skipping`);
        continue;
      }
      const childActorId = `${parentActorId}::${extensionId}`;
      try {
        const scopedCaps = intersectCapabilityGrants(parentCaps, manifest.caps);
        const assets = await this.loadAssets(manifest.moduleUrl);
        await this.shardClient.activate(childActorId, manifest.moduleUrl, scopedCaps, this.defaultBudget, assets);
        this.markResident(childActorId, extensionId);
        children.push(childActorId);
      } catch (error) {
        console.warn(`[DEBUG] ActivationRegistry: extension ${extensionId} of ${pluginId} failed to activate`, error);
      }
    }
    if (children.length > 0)
      this.extensionChildren.set(parentActorId, children);
  }
  enqueueTurn(actorId, lane, events, options) {
    const generation2 = this.actorGeneration.get(actorId) ?? 0;
    return this.turnScheduler.enqueue(actorId, { lane, coalesce: options?.coalesce, payload: { events, generation: generation2 } });
  }
  async runQueuedTurn(actorId, payload, budget) {
    const currentGeneration = this.actorGeneration.get(actorId) ?? 0;
    if (payload.generation !== currentGeneration) {
      console.warn(`[DEBUG] ActivationRegistry: dropping turn for ${actorId} queued against generation ${payload.generation}, now at ${currentGeneration} (restored in between)`);
      return;
    }
    this.touch(actorId);
    const result3 = await this.shardClient.turn(actorId, payload.events, budget);
    this.onTurnResult(actorId, result3);
  }
  async evictForMemoryPressure() {
    while (this.residencyOrder.length >= this.maxResidentActors) {
      await this.suspend(this.residencyOrder[0]);
    }
  }
  async suspend(actorId) {
    if (!this.resident.has(actorId))
      return;
    this.turnScheduler.cancelQueued(actorId);
    await this.suspendExtensionsOf(actorId);
    const checkpoint = await this.shardClient.checkpoint(actorId);
    this.checkpoints.set(actorId, checkpoint);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  async suspendExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children)
      await this.suspend(child);
  }
  async resume(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      throw new Error(`[DEBUG] ActivationRegistry.resume: unknown actor ${actorId} (never activated)`);
    const manifest = this.manifests.get(pluginId);
    if (!manifest)
      throw new Error(`[DEBUG] ActivationRegistry.resume: no manifest for plugin ${pluginId}`);
    await this.evictForMemoryPressure();
    const assets = await this.loadAssets(manifest.moduleUrl);
    await this.shardClient.activate(actorId, manifest.moduleUrl, manifest.caps, this.defaultBudget, assets);
    const checkpoint = this.checkpoints.get(actorId);
    if (checkpoint)
      await this.shardClient.restore(actorId, checkpoint);
    this.markResident(actorId, pluginId);
    await this.resumeExtensionsOf(actorId);
  }
  async resumeExtensionsOf(parentActorId) {
    const children = this.extensionChildren.get(parentActorId);
    if (!children)
      return;
    for (const child of children) {
      if (this.checkpoints.has(child) && !this.resident.has(child))
        await this.resume(child);
    }
  }
  async restoreActor(actorId) {
    const pluginId = this.actorPlugin.get(actorId);
    if (!pluginId)
      return;
    this.actorGeneration.set(actorId, (this.actorGeneration.get(actorId) ?? 0) + 1);
    this.turnScheduler.cancelQueued(actorId);
    this.resident.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
    await this.resume(actorId);
  }
  async restoreActors(actorIds) {
    await Promise.all(actorIds.map((actorId) => this.restoreActor(actorId).catch((error) => console.error(`[DEBUG] ActivationRegistry.restoreActors: failed to restore ${actorId}`, error))));
  }
  handleShardLost = (_shardIndex, actorIds) => {
    this.restoreActors(actorIds);
  };
  cancel(actorId) {
    if (!this.actorPlugin.has(actorId))
      return;
    const children = this.extensionChildren.get(actorId);
    if (children) {
      for (const child of children)
        this.cancel(child);
      this.extensionChildren.delete(actorId);
    }
    this.turnScheduler.teardownActor(actorId);
    this.actorGeneration.delete(actorId);
    this.shardClient.dispose(actorId);
    this.resident.delete(actorId);
    this.checkpoints.delete(actorId);
    this.actorPlugin.delete(actorId);
    const index = this.residencyOrder.indexOf(actorId);
    if (index !== -1)
      this.residencyOrder.splice(index, 1);
  }
  isResident(actorId) {
    return this.resident.has(actorId);
  }
  dispose() {
    this.stopMetricsPublisher();
  }
  runtimeMetricsActorRows() {
    return [...this.actorPlugin.entries()].map(([actorId, pluginId]) => ({ actorId, pluginId, resident: this.resident.has(actorId), shard: this.shardClient.shardIndexFor(actorId) ?? null }));
  }
  runtimeMetricsSnapshot(sampledAtMs = this.now()) {
    return { actors: this.runtimeMetricsActorRows(), shards: this.shardClient.shardMetricsSamples(sampledAtMs), sampledAtMs };
  }
  startRuntimeMetricsPublisher(sink) {
    const interval = setInterval(() => {
      const nowMs = this.now();
      if (!runtimeMetricsDue(this.lastRuntimeMetricsPublishMs, nowMs))
        return;
      this.lastRuntimeMetricsPublishMs = nowMs;
      sink("os.runtime.metrics", this.runtimeMetricsSnapshot(nowMs));
    }, RUNTIME_METRICS_PUBLISH_INTERVAL_MS);
    return () => clearInterval(interval);
  }
}
var RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500;
function runtimeMetricsDue(lastPublishedMs, nowMs) {
  if (lastPublishedMs === null)
    return true;
  return nowMs - lastPublishedMs >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
}
if (undefined) {
  let createAutoReplyWorker = function() {}, fakeShardClient = function(shardCount = 1) {}, fixtureResidentLedger = function() {}, catalogWithOneExtension = function() {};
  async function flushMicrotasks(n = 10) {}
}
function findPlaygroundVariant(catalog, playgroundPluginId) {
  return catalog.playgrounds.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}
function resolvePluginRegistryId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.pluginId ?? playgroundPluginId;
}
function resolvePlaygroundDefaultAppId(catalog, playgroundPluginId) {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.app;
}
function resolvePlaygroundBoot(catalog, variant, session) {
  const defaultAppId = resolvePlaygroundDefaultAppId(catalog, variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins, dependencyErrors: [] };
  }
  const registryPluginId = resolvePluginRegistryId(catalog, variant);
  const hostMode = resolvePluginHostConfig(catalog, variant) !== undefined;
  const catalogPlugins = [...catalog.plugins, ...catalog.extensions].map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: target.role === "extension" ? catalog.extensionModuleUrl(target.pluginId) : catalog.moduleUrl(target.pluginId),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn)
  }));
  const expanded = expandPluginRegistry(catalogPlugins, hostMode ? undefined : registryPluginId, hostMode);
  const { order, errors } = orderPluginRegistryEntries(expanded);
  if (errors.length > 0) {
    for (const error of errors)
      console.error(`[DEBUG] resolvePlaygroundBoot(${variant}): ${pluginGraphErrorMessage(error, "en")}`);
  }
  return {
    variant,
    defaultAppId,
    plugins: order,
    dependencyErrors: errors
  };
}
function resolvePluginHostConfig(catalog, playgroundPluginId) {
  const registryId = resolvePluginRegistryId(catalog, playgroundPluginId);
  return catalog.hosts.find((entry) => entry.pluginId === registryId);
}
function parseVersion(raw) {
  if (!raw)
    return null;
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match)
    return null;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]) };
}
function compareVersions(a, b) {
  if (a.major !== b.major)
    return a.major - b.major;
  if (a.minor !== b.minor)
    return a.minor - b.minor;
  return a.patch - b.patch;
}
function parseVersionReq(raw) {
  const trimmed = raw.trim();
  if (trimmed === "*")
    return { kind: "any" };
  const opMatch = /^(=|\^|~|>=)(\d+\.\d+\.\d+)$/.exec(trimmed);
  if (!opMatch)
    return null;
  const version = parseVersion(opMatch[2]);
  if (!version)
    return null;
  switch (opMatch[1]) {
    case "=":
      return { kind: "exact", version };
    case "^":
      return { kind: "caret", version };
    case "~":
      return { kind: "tilde", version };
    case ">=":
      return { kind: "atLeast", version };
    default:
      return null;
  }
}
function versionSatisfies(actual, requirement) {
  const req = parseVersionReq(requirement);
  if (!req)
    return false;
  if (req.kind === "any")
    return true;
  const version = parseVersion(actual);
  if (!version)
    return false;
  if (req.kind === "exact")
    return compareVersions(version, req.version) === 0;
  if (req.kind === "atLeast")
    return compareVersions(version, req.version) >= 0;
  if (req.kind === "tilde") {
    return version.major === req.version.major && version.minor === req.version.minor && version.patch >= req.version.patch;
  }
  if (compareVersions(version, req.version) < 0)
    return false;
  if (req.version.major > 0)
    return version.major === req.version.major;
  if (req.version.minor > 0)
    return version.major === 0 && version.minor === req.version.minor;
  return version.major === 0 && version.minor === 0 && version.patch === req.version.patch;
}
function validatePluginDependencyGraph(nodes) {
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const errors = [];
  for (const node of nodes) {
    for (const dependency of node.dependencies ?? []) {
      const target = byId.get(dependency.pluginId);
      if (!target) {
        errors.push({ code: "transaction.dependency-missing", pluginId: node.pluginId, dependsOn: dependency.pluginId });
        continue;
      }
      if (target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {
        errors.push({ code: "transaction.version-mismatch", pluginId: node.pluginId, dependsOn: dependency.pluginId, required: dependency.version, actual: target.version });
      }
    }
  }
  return errors;
}
function findCycleMembers(byId, leftover) {
  const visiting = new Set;
  const visited = new Set;
  const stack = [];
  let cycle = null;
  function visit(id2) {
    if (cycle || !leftover.has(id2) || visited.has(id2))
      return;
    if (visiting.has(id2)) {
      const start = stack.indexOf(id2);
      cycle = stack.slice(start);
      return;
    }
    visiting.add(id2);
    stack.push(id2);
    for (const dependency of byId.get(id2)?.dependencies ?? []) {
      if (leftover.has(dependency.pluginId))
        visit(dependency.pluginId);
      if (cycle)
        return;
    }
    stack.pop();
    visiting.delete(id2);
    visited.add(id2);
  }
  for (const id2 of [...leftover].sort()) {
    visit(id2);
    if (cycle)
      break;
  }
  return cycle ?? [...leftover].sort();
}
function resolvePluginLoadOrder(nodes) {
  const structural = validatePluginDependencyGraph(nodes);
  if (structural.length > 0)
    return { order: [], errors: structural };
  const byId = new Map(nodes.map((node) => [node.pluginId, node]));
  const indegree = new Map;
  const dependents = new Map;
  for (const node of nodes) {
    indegree.set(node.pluginId, indegree.get(node.pluginId) ?? 0);
    for (const dependency of node.dependencies ?? []) {
      indegree.set(node.pluginId, (indegree.get(node.pluginId) ?? 0) + 1);
      const list = dependents.get(dependency.pluginId) ?? [];
      list.push(node.pluginId);
      dependents.set(dependency.pluginId, list);
    }
  }
  const order = [];
  const remaining = new Map(indegree);
  const queue = [...indegree.entries()].filter(([, count]) => count === 0).map(([id2]) => id2);
  while (queue.length > 0) {
    queue.sort();
    const id2 = queue.shift();
    order.push(id2);
    for (const dependent of dependents.get(id2) ?? []) {
      const next = (remaining.get(dependent) ?? 0) - 1;
      remaining.set(dependent, next);
      if (next === 0)
        queue.push(dependent);
    }
  }
  if (order.length === nodes.length)
    return { order, errors: [] };
  const leftover = new Set(nodes.map((node) => node.pluginId).filter((id2) => !order.includes(id2)));
  return { order: [], errors: [{ code: "transaction.cycle", members: findCycleMembers(byId, leftover) }] };
}
function pluginDependents(nodes, pluginId) {
  return nodes.filter((node) => (node.dependencies ?? []).some((dependency) => dependency.pluginId === pluginId)).map((node) => node.pluginId).sort();
}

class PluginGraph {
  nodes;
  constructor(nodes) {
    this.nodes = nodes;
  }
  validate() {
    return validatePluginDependencyGraph(this.nodes);
  }
  loadOrder() {
    return resolvePluginLoadOrder(this.nodes);
  }
  dependents(pluginId) {
    return pluginDependents(this.nodes, pluginId);
  }
  canUnload(pluginId, loadedIds) {
    return this.dependents(pluginId).every((dependent) => !loadedIds.has(dependent));
  }
}
function orderPluginRegistryEntries(entries) {
  const nodes = entries.map((entry) => ({ pluginId: entry.pluginId, dependencies: entry.dependencies }));
  const { order, errors } = new PluginGraph(nodes).loadOrder();
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  if (errors.length === 0) {
    return { order: order.map((id2) => byId.get(id2)).filter((entry) => entry !== undefined), errors: [] };
  }
  const blocked = new Set(errors.flatMap((error) => error.code === "transaction.cycle" ? error.members : [error.pluginId]));
  const remaining = entries.filter((entry) => !blocked.has(entry.pluginId));
  const retried = orderPluginRegistryEntries(remaining);
  return { order: retried.order, errors: [...errors, ...retried.errors] };
}
function resolveLocalizedLabel(label, locale) {
  return label[locale] ?? label.en ?? Object.values(label)[0] ?? "";
}
function pluginGraphErrorMessage(error, locale) {
  switch (error.code) {
    case "transaction.dependency-missing":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}", which is not installed.`,
        de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“, welches nicht installiert ist.`
      }, locale);
    case "transaction.version-mismatch":
      return resolveLocalizedLabel({
        en: `Plugin "${error.pluginId}" needs "${error.dependsOn}" ${error.required}, but ${error.actual} is installed.`,
        de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“ ${error.required}, installiert ist jedoch ${error.actual}.`
      }, locale);
    case "transaction.cycle":
      return resolveLocalizedLabel({
        en: `Plugin dependency cycle: ${error.members.join(" → ")}.`,
        de: `Zyklische Plugin-Abhängigkeit: ${error.members.join(" → ")}.`
      }, locale);
  }
}

class InstanceDirectory {
  byArtifactId = new Map;
  register(artifactId, ref) {
    this.byArtifactId.set(artifactId, ref);
  }
  unregister(artifactId) {
    this.byArtifactId.delete(artifactId);
  }
  resolve(artifactId) {
    return this.byArtifactId.get(artifactId);
  }
  entries() {
    return [...this.byArtifactId.entries()];
  }
}

class ArtifactRouterConflictError extends Error {
  code = "artifact-router.conflict";
  constructor(artifactKind, key) {
    super(`[DEBUG] router conflict: ${artifactKind}#${key} already registered with different metadata`);
    this.name = "ArtifactRouterConflictError";
  }
}

class ArtifactContributionNotPermittedError extends Error {
  code = "transaction.contribution-not-permitted";
  constructor(contributorPluginId, ownerPluginId) {
    super(`[DEBUG] "${contributorPluginId}" may not contribute onto "${ownerPluginId}"'s artifact kind — not a direct dependency`);
    this.name = "ArtifactContributionNotPermittedError";
  }
}
function stableStringify(value) {
  if (value === null || typeof value !== "object")
    return JSON.stringify(value);
  if (Array.isArray(value))
    return `[${value.map(stableStringify).join(",")}]`;
  const record = value;
  const keys = Object.keys(record).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableStringify(record[key])}`).join(",")}}`;
}

class ConflictCheckedRegistry {
  entries = new Map;
  register(artifactKind, key, ownership, metadata) {
    const compositeKey = `${artifactKind} ${key}`;
    const fingerprint = stableStringify(metadata);
    const existing = this.entries.get(compositeKey);
    if (existing && existing.fingerprint !== fingerprint)
      throw new ArtifactRouterConflictError(artifactKind, key);
    this.entries.set(compositeKey, { ownership, fingerprint });
  }
  resolve(artifactKind, key) {
    return this.entries.get(`${artifactKind} ${key}`)?.ownership;
  }
}

class ArtifactMutationRouter {
  registry = new ConflictCheckedRegistry;
  registerOwner(artifactKind, mutationId) {
    this.registry.register(artifactKind, mutationId, { kind: "owner" }, { kind: "owner", artifactKind, mutationId });
  }
  registerContributed(artifactKind, contributorPluginId, ownerPluginId, metadata, contributorDependsOnOwner) {
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(contributorPluginId, ownerPluginId);
    this.registry.register(artifactKind, metadata.mutationId, { kind: "contributed", pluginId: contributorPluginId }, metadata);
  }
  resolve(artifactKind, mutationId) {
    return this.registry.resolve(artifactKind, mutationId);
  }
}

class ArtifactInferenceRouter {
  registry = new ConflictCheckedRegistry;
  dependsOn = new Map;
  registerOwner(artifactKind, inferenceSchema) {
    this.registry.register(artifactKind, inferenceSchema, { kind: "owner" }, { kind: "owner", artifactKind, inferenceSchema });
  }
  registerContributed(artifactKind, metadata, contributorDependsOnOwner) {
    if (metadata.owner !== metadata.contributor) {
      throw new Error(`[DEBUG] contributed inference owner/contributor mismatch: ${metadata.owner} !== ${metadata.contributor}`);
    }
    if (metadata.artifactKind !== artifactKind) {
      throw new Error(`[DEBUG] contributed inference artifactKind mismatch: ${metadata.artifactKind} !== ${artifactKind}`);
    }
    if (!contributorDependsOnOwner)
      throw new ArtifactContributionNotPermittedError(metadata.contributor, artifactKind);
    this.registry.register(artifactKind, metadata.inferenceSchema, { kind: "contributed", pluginId: metadata.contributor }, metadata);
    this.dependsOn.set(`${artifactKind} ${metadata.inferenceSchema}`, metadata.dependsOn ?? []);
  }
  resolve(artifactKind, inferenceSchema) {
    return this.registry.resolve(artifactKind, inferenceSchema);
  }
  dependencyOrder() {
    const keys = [...this.dependsOn.keys()];
    const indegree = new Map(keys.map((key) => [key, 0]));
    const dependents = new Map;
    for (const key of keys) {
      for (const dependency of this.dependsOn.get(key) ?? []) {
        if (!indegree.has(dependency))
          continue;
        indegree.set(key, (indegree.get(key) ?? 0) + 1);
        const list = dependents.get(dependency) ?? [];
        list.push(key);
        dependents.set(dependency, list);
      }
    }
    const order = [];
    const remaining = new Map(indegree);
    const queue = keys.filter((key) => (indegree.get(key) ?? 0) === 0);
    while (queue.length > 0) {
      queue.sort();
      const key = queue.shift();
      order.push(key);
      for (const dependent of dependents.get(key) ?? []) {
        const next = (remaining.get(dependent) ?? 0) - 1;
        remaining.set(dependent, next);
        if (next === 0)
          queue.push(dependent);
      }
    }
    if (order.length !== keys.length) {
      const leftover = keys.filter((key) => !order.includes(key)).sort();
      throw new Error(`[DEBUG] ArtifactInferenceRouter.dependencyOrder: cycle among ${leftover.join(", ")}`);
    }
    return order;
  }
}
if (undefined) {}
if (undefined) {}
/* ../../../../../../../../../🔨️modules/🔄️machine/🟦️.ts */
var NodeId = (value) => value;
var ActorId = (value) => value;
var ROOT = NodeId(0);

class BitSet {
  #bits;
  constructor(bits = []) {
    this.#bits = new Set(bits);
  }
  set(id2) {
    this.#bits.add(id2);
  }
  clear(id2) {
    this.#bits.delete(id2);
  }
  contains(id2) {
    return this.#bits.has(id2);
  }
  *iterOnes() {
    for (const id2 of [...this.#bits].sort((a, b) => a - b))
      yield NodeId(id2);
  }
  clearAll() {
    this.#bits.clear();
  }
  isEmpty() {
    return this.#bits.size === 0;
  }
  clone() {
    return new BitSet(this.#bits);
  }
  equals(other) {
    if (!(other instanceof BitSet))
      return false;
    if (other.#bits.size !== this.#bits.size)
      return false;
    for (const id2 of this.#bits)
      if (!other.#bits.has(id2))
        return false;
    return true;
  }
}

class Snapshot {
  configuration;
  context;
  status;
  #nodes;
  #history = [];
  constructor(nodes, configuration, context, status = { kind: "running" }) {
    this.#nodes = nodes;
    this.configuration = configuration;
    this.context = context;
    this.status = status;
  }
  matches(stableId) {
    for (const id2 of this.configuration.iterOnes())
      if (this.#nodes[id2].stableId === stableId)
        return true;
    return false;
  }
  historyFor(node) {
    return this.#history.find(([key]) => key === node)?.[1];
  }
  recordHistory(node, value) {
    const entry = this.#history.find(([key]) => key === node);
    if (entry)
      entry[1] = [...value];
    else
      this.#history.push([node, [...value]]);
  }
  historyEntries() {
    return this.#history;
  }
  branchForExploration() {
    const branch = new Snapshot(this.#nodes, this.configuration.clone(), structuredClone(this.context), { kind: "running" });
    for (const [owner, ids] of this.#history)
      branch.recordHistory(owner, ids);
    return branch;
  }
}

class NullInspector {
  observe() {}
}
var MICROSTEP_LIMIT = 1000;
function isDescendant(nodes, a, ancestor) {
  if (a === ancestor)
    return false;
  let cur = nodes[a].parent;
  while (cur !== undefined) {
    if (cur === ancestor)
      return true;
    cur = nodes[cur].parent;
  }
  return false;
}
function isDescendantOrSelf(nodes, a, ancestor) {
  return a === ancestor || isDescendant(nodes, a, ancestor);
}
function depthOf(nodes, id2) {
  let depth = 0;
  let cur = nodes[id2].parent;
  while (cur !== undefined) {
    depth += 1;
    cur = nodes[cur].parent;
  }
  return depth;
}
function isCompoundOrParallel(nodes, id2) {
  const kind = nodes[id2].kind;
  return kind === "compound" || kind === "parallel";
}
function isLeafish(nodes, id2) {
  const kind = nodes[id2].kind;
  return kind === "atomic" || kind === "final";
}
function computeDomain(nodes, source, targets, kind) {
  if (targets.length === 0)
    return source;
  if (kind === "internal" && isCompoundOrParallel(nodes, source) && targets.every((t) => isDescendant(nodes, t, source)))
    return source;
  let anc = nodes[source].parent;
  while (anc !== undefined) {
    if (isCompoundOrParallel(nodes, anc) && targets.every((t) => isDescendantOrSelf(nodes, t, anc)))
      return anc;
    anc = nodes[anc].parent;
  }
  return ROOT;
}
function resolveEffectiveTargets(nodes, targets, snapshot) {
  const out = [];
  for (const t of targets) {
    const kind = nodes[t].kind;
    if (kind === "historyShallow" || kind === "historyDeep") {
      const recorded = snapshot.historyFor(t);
      if (recorded) {
        for (const r of recorded)
          if (!out.includes(r))
            out.push(r);
      } else {
        const fallback = nodes[t].initial;
        if (fallback !== undefined && !out.includes(fallback))
          out.push(fallback);
      }
    } else if (!out.includes(t)) {
      out.push(t);
    }
  }
  return out;
}
function addDescendantStatesToEnter(nodes, state7, snapshot, out) {
  const kind = nodes[state7].kind;
  if (kind === "historyShallow" || kind === "historyDeep") {
    for (const r of resolveEffectiveTargets(nodes, [state7], snapshot))
      addDescendantStatesToEnter(nodes, r, snapshot, out);
    return;
  }
  if (!out.includes(state7))
    out.push(state7);
  if (kind === "compound") {
    const initial = nodes[state7].initial;
    if (initial !== undefined) {
      addDescendantStatesToEnter(nodes, initial, snapshot, out);
      addAncestorStatesToEnter(nodes, initial, state7, snapshot, out);
    }
  } else if (kind === "parallel") {
    for (const child of nodes[state7].children) {
      if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
        addDescendantStatesToEnter(nodes, child, snapshot, out);
    }
  }
}
function addAncestorStatesToEnter(nodes, state7, stopAt, snapshot, out) {
  let anc = nodes[state7].parent;
  while (anc !== undefined && anc !== stopAt) {
    if (!out.includes(anc))
      out.push(anc);
    if (nodes[anc].kind === "parallel") {
      for (const child of nodes[anc].children) {
        if (!out.some((e) => isDescendantOrSelf(nodes, e, child)))
          addDescendantStatesToEnter(nodes, child, snapshot, out);
      }
    }
    anc = nodes[anc].parent;
  }
}
function stateDone(nodes, config, node) {
  const kind = nodes[node].kind;
  if (kind === "final")
    return true;
  if (kind === "compound") {
    for (const child of nodes[node].children)
      if (config.contains(child))
        return stateDone(nodes, config, child);
    return false;
  }
  if (kind === "parallel")
    return nodes[node].children.every((c) => stateDone(nodes, config, c));
  return false;
}
function computeDoneNodes(nodes, config) {
  const out = [];
  for (const id2 of config.iterOnes())
    if (isCompoundOrParallel(nodes, id2) && stateDone(nodes, config, id2))
      out.push(id2);
  return out;
}
function candidatesFor(definition, config, context, event, selector, done) {
  const out = [];
  definition.transitions.forEach((t, i) => {
    if (!config.contains(t.source))
      return;
    const matchesTrigger = selector.kind === "event" && t.trigger.kind === "event" && t.trigger.event === selector.event || selector.kind === "spontaneous" && (t.trigger.kind === "eventless" || t.trigger.kind === "done" && done.includes(t.trigger.node)) || selector.kind === "timer" && t.trigger.kind === "timer" && t.trigger.timer === selector.timer;
    if (!matchesTrigger)
      return;
    if (t.guard !== undefined && !definition.guards[t.guard](context, event))
      return;
    out.push(i);
  });
  return out;
}
function resolveConflicts(nodes, transitions, candidates) {
  const sorted = [...candidates].sort((a, b) => transitions[a].docIndex - transitions[b].docIndex);
  const selected = [];
  outer:
    for (const cand of sorted) {
      const candDomain = computeDomain(nodes, transitions[cand].source, transitions[cand].targets, transitions[cand].kind);
      const toRemove = [];
      for (let i = 0;i < selected.length; i += 1) {
        const sel = selected[i];
        const selDomain = computeDomain(nodes, transitions[sel].source, transitions[sel].targets, transitions[sel].kind);
        if (isDescendantOrSelf(nodes, candDomain, selDomain) || isDescendantOrSelf(nodes, selDomain, candDomain)) {
          if (depthOf(nodes, transitions[cand].source) > depthOf(nodes, transitions[sel].source))
            toRemove.push(i);
          else
            continue outer;
        }
      }
      for (let i = toRemove.length - 1;i >= 0; i -= 1)
        selected.splice(toRemove[i], 1);
      selected.push(cand);
    }
  return selected;
}
function applyTransitions(definition, snapshot, transitionsIdx, event, sink, inspector) {
  const nodes = definition.nodes;
  const exitIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    for (const id2 of snapshot.configuration.iterOnes())
      if (isDescendant(nodes, id2, domain) && !exitIds.includes(id2))
        exitIds.push(id2);
  }
  exitIds.sort((a, b) => depthOf(nodes, b) - depthOf(nodes, a));
  for (const owner of exitIds) {
    for (const child of nodes[owner].children) {
      const childKind = nodes[child].kind;
      if (childKind === "historyShallow") {
        const activeChild = nodes[owner].children.find((c) => snapshot.configuration.contains(c) && nodes[c].kind !== "historyShallow" && nodes[c].kind !== "historyDeep");
        if (activeChild !== undefined)
          snapshot.recordHistory(child, [activeChild]);
      } else if (childKind === "historyDeep") {
        const leaves = [];
        for (const id2 of snapshot.configuration.iterOnes())
          if (isDescendant(nodes, id2, owner) && isLeafish(nodes, id2))
            leaves.push(id2);
        snapshot.recordHistory(child, leaves);
      }
    }
  }
  for (const id2 of exitIds) {
    for (const actionId of nodes[id2].exitActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId] of nodes[id2].timers)
      sink.push({ kind: "cancelTimer", timer: timerId });
    for (const invokeId of nodes[id2].invokes)
      sink.push({ kind: "stopInvoke", invoke: invokeId });
    snapshot.configuration.clear(id2);
  }
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    for (const actionId of t.actions)
      definition.actions[actionId](snapshot.context, event, sink);
  }
  const entryIds = [];
  for (const ti of transitionsIdx) {
    const t = definition.transitions[ti];
    const domain = computeDomain(nodes, t.source, t.targets, t.kind);
    const effectiveTargets = resolveEffectiveTargets(nodes, t.targets, snapshot);
    for (const target of effectiveTargets)
      addDescendantStatesToEnter(nodes, target, snapshot, entryIds);
    for (const target of effectiveTargets)
      addAncestorStatesToEnter(nodes, target, domain, snapshot, entryIds);
  }
  entryIds.sort((a, b) => depthOf(nodes, a) - depthOf(nodes, b));
  for (const id2 of entryIds) {
    snapshot.configuration.set(id2);
    for (const actionId of nodes[id2].entryActions)
      definition.actions[actionId](snapshot.context, event, sink);
    for (const [timerId, delayMs] of nodes[id2].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of nodes[id2].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  inspector.observe({ kind: "microstep", exited: exitIds, entered: entryIds });
}
function finalizeStatus(definition, snapshot) {
  if (snapshot.status.kind === "done")
    return;
  if (stateDone(definition.nodes, snapshot.configuration, ROOT) && definition.makeOutput) {
    snapshot.status = { kind: "done", output: definition.makeOutput(snapshot.context) };
  }
}
function runToCompletion(definition, snapshot, seed, sink, inspector) {
  inspector.observe({ kind: "macrostepStart" });
  const queue = seed ? [seed] : [];
  let microsteps = 0;
  for (;; ) {
    if (microsteps >= MICROSTEP_LIMIT)
      break;
    let selected;
    let eventOwned;
    const trigger = queue.shift();
    if (trigger) {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const selector = trigger.selector.kind === "event" ? { kind: "event", event: trigger.selector.event } : { kind: "timer", timer: trigger.selector.timer };
      selected = candidatesFor(definition, snapshot.configuration, snapshot.context, trigger.event, selector, done);
      eventOwned = trigger.event;
    } else {
      const done = computeDoneNodes(definition.nodes, snapshot.configuration);
      const spontaneous = candidatesFor(definition, snapshot.configuration, snapshot.context, undefined, { kind: "spontaneous" }, done);
      if (spontaneous.length === 0)
        break;
      selected = spontaneous;
      eventOwned = undefined;
    }
    if (selected.length === 0)
      continue;
    const resolved = resolveConflicts(definition.nodes, definition.transitions, selected);
    microsteps += 1;
    const local = [];
    applyTransitions(definition, snapshot, resolved, eventOwned, local, inspector);
    for (const command of local) {
      if (command.kind === "raise")
        queue.push({ selector: { kind: "event", event: command.event.eventId() }, event: command.event });
      inspector.observe({ kind: "commandIssued", command });
      sink.push(command);
    }
  }
  finalizeStatus(definition, snapshot);
  inspector.observe({ kind: "settled", microsteps });
  return { microsteps };
}
function init(machine, input, sink) {
  const definition = machine.definition;
  const snapshot = new Snapshot(definition.nodes, new BitSet, definition.contextFromInput(input));
  const entryIds = [];
  addDescendantStatesToEnter(definition.nodes, ROOT, snapshot, entryIds);
  entryIds.sort((a, b) => depthOf(definition.nodes, a) - depthOf(definition.nodes, b));
  for (const id2 of entryIds) {
    snapshot.configuration.set(id2);
    for (const actionId of definition.nodes[id2].entryActions)
      definition.actions[actionId](snapshot.context, undefined, sink);
    for (const [timerId, delayMs] of definition.nodes[id2].timers)
      sink.push({ kind: "schedule", timer: timerId, delayMs });
    for (const invokeId of definition.nodes[id2].invokes)
      sink.push({ kind: "startInvoke", invoke: invokeId });
  }
  runToCompletion(definition, snapshot, undefined, sink, new NullInspector);
  return snapshot;
}
function macrostep(machine, snapshot, event, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "event", event: event.eventId() }, event }, sink, inspector);
}
function timerElapsed(machine, snapshot, timer, sink, inspector) {
  return runToCompletion(machine.definition, snapshot, { selector: { kind: "timer", timer } }, sink, inspector);
}

class NativeHost {
  #start = Date.now();
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  effects() {
    return this.#effects;
  }
  drainEffects() {
    return this.#effects.splice(0, this.#effects.length);
  }
  startedTasks() {
    return this.#startedTasks;
  }
  dueTimers() {
    const now = this.nowMs();
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.nowMs() + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
  }
  nowMs() {
    return Date.now() - this.#start;
  }
}

class TestHost {
  #clockMs = 0;
  #effects = [];
  #pendingTimers = [];
  #startedTasks = [];
  #cancelledTasks = [];
  effects() {
    return this.#effects;
  }
  startedTasks() {
    return this.#startedTasks;
  }
  cancelledTasks() {
    return this.#cancelledTasks;
  }
  advance(delayMs) {
    this.#clockMs += delayMs;
    const now = this.#clockMs;
    const due = [];
    const remaining = this.#pendingTimers.filter(([actor, timer, at]) => {
      if (at > now)
        return true;
      due.push([actor, timer]);
      return false;
    });
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...remaining);
    return due;
  }
  executeEffect(actor, effect) {
    this.#effects.push([actor, effect]);
  }
  schedule(actor, timer, delayMs) {
    this.#pendingTimers.push([actor, timer, this.#clockMs + delayMs]);
  }
  cancelTimer(actor, timer) {
    const kept = this.#pendingTimers.filter(([a, t]) => !(a === actor && t === timer));
    this.#pendingTimers.length = 0;
    this.#pendingTimers.push(...kept);
  }
  startTask(actor, invoke) {
    this.#startedTasks.push([actor, invoke]);
  }
  cancelTask(actor, invoke) {
    const kept = this.#startedTasks.filter(([a, i]) => !(a === actor && i === invoke));
    this.#startedTasks.length = 0;
    this.#startedTasks.push(...kept);
    this.#cancelledTasks.push([actor, invoke]);
  }
  nowMs() {
    return this.#clockMs;
  }
}
class Actor {
  id;
  snapshot;
  mailbox = [];
  constructor(id2, snapshot) {
    this.id = id2;
    this.snapshot = snapshot;
  }
}

class ActorSystem {
  host;
  #machine;
  #actors = [];
  #nextId = 0;
  constructor(host, machine) {
    this.host = host;
    this.#machine = machine;
  }
  spawnRoot(input) {
    const id2 = ActorId(this.#nextId);
    this.#nextId += 1;
    const buffer = [];
    const snapshot = init(this.#machine, input, buffer);
    this.#actors.push(new Actor(id2, snapshot));
    this.#routeCommands(id2, buffer);
    return id2;
  }
  snapshot(id2) {
    return this.#actors.find((a) => a.id === id2)?.snapshot;
  }
  send(to, event) {
    this.#actors.find((a) => a.id === to)?.mailbox.push(event);
  }
  timerElapsed(to, timer) {
    const actor = this.#actors.find((a) => a.id === to);
    if (!actor)
      return;
    const buffer = [];
    const report = timerElapsed(this.#machine, actor.snapshot, timer, buffer, new NullInspector);
    this.#routeCommands(to, buffer);
    return report;
  }
  drain() {
    const reports = [];
    for (;; ) {
      let progressed = false;
      for (const actor of this.#actors) {
        const event = actor.mailbox.shift();
        if (event === undefined)
          continue;
        progressed = true;
        const buffer = [];
        const report = macrostep(this.#machine, actor.snapshot, event, buffer, new NullInspector);
        this.#routeCommands(actor.id, buffer);
        reports.push(report);
      }
      if (!progressed)
        break;
    }
    return reports;
  }
  #routeCommands(actor, commands) {
    const sends = [];
    const found = this.#actors.find((a) => a.id === actor);
    if (found) {
      for (const command of commands) {
        const pair = routeCommand(this.host, found.snapshot, actor, command);
        if (pair)
          sends.push(pair);
      }
    }
    for (const [to, event] of sends)
      this.send(to, event);
  }
}
function routeCommand(host, snapshot, actor, command) {
  switch (command.kind) {
    case "effect":
      host.executeEffect(actor, command.effect);
      return;
    case "raise":
      return;
    case "send":
      return [command.to, command.event];
    case "emit":
      snapshot.status = { kind: "done", output: command.output };
      return;
    case "startInvoke":
      host.startTask(actor, command.invoke);
      return;
    case "stopInvoke":
      host.cancelTask(actor, command.invoke);
      return;
    case "schedule":
      host.schedule(actor, command.timer, command.delayMs);
      return;
    case "cancelTimer":
      host.cancelTimer(actor, command.timer);
      return;
  }
}
if (undefined) {
  let flatEvent = function(kind, branch = 0) {}, buildTrafficLight = function() {}, buildBranching = function() {};
}
/* ../../../../../../../../../📦️packages/🟦️typescript/🟦️.ts */
if (undefined) {}
/* ../../../../../../🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json */
var _catalog_default2 = {
  version: 1,
  modules: [
    { pluginId: "animate", directoryName: "🎞️animate" },
    { pluginId: "architect", directoryName: "🏛️architect" },
    { pluginId: "block", directoryName: "🧱️block" },
    { pluginId: "cad", directoryName: "📐️cad" },
    { pluginId: "cad-extension-aec-building", directoryName: "🏢️cad-extension-aec-building" },
    { pluginId: "cad-extension-aec-building-energy", directoryName: "🔥️cad-extension-aec-building-energy" },
    { pluginId: "cad-extension-aec-building-structure", directoryName: "🏟️cad-extension-aec-building-structure" },
    { pluginId: "cad-extension-spatial-shape", directoryName: "🔷️cad-extension-spatial-shape" },
    { pluginId: "dag", directoryName: "🕸️dag" },
    { pluginId: "demonstrator", directoryName: "🎪️demonstrator" },
    { pluginId: "draw", directoryName: "🖍️draw" },
    { pluginId: "energy", directoryName: "🔋️energy" },
    { pluginId: "fem", directoryName: "🏗️fem" },
    { pluginId: "flow", directoryName: "🌊️flow" },
    { pluginId: "flow-extension-bim", directoryName: "🏘️flow-extension-bim" },
    { pluginId: "flow-extension-brep", directoryName: "🧊️flow-extension-brep" },
    { pluginId: "flow-extension-dictionary", directoryName: "📚️flow-extension-dictionary" },
    { pluginId: "flow-extension-draw", directoryName: "🎨️flow-extension-draw" },
    { pluginId: "flow-extension-list", directoryName: "📃️flow-extension-list" },
    { pluginId: "flow-extension-logic", directoryName: "🔀️flow-extension-logic" },
    { pluginId: "flow-extension-math", directoryName: "🧮️flow-extension-math" },
    { pluginId: "flow-extension-primitive", directoryName: "🔤️flow-extension-primitive" },
    { pluginId: "flow-extension-text", directoryName: "📝️flow-extension-text" },
    { pluginId: "forms", directoryName: "📋️forms" },
    { pluginId: "gis", directoryName: "🌍️gis" },
    { pluginId: "imperative", directoryName: "📜️imperative" },
    { pluginId: "imperative-extension-control", directoryName: "🎮️imperative-extension-control" },
    { pluginId: "imperative-extension-effect", directoryName: "📣️imperative-extension-effect" },
    { pluginId: "imperative-extension-logic", directoryName: "⚖️imperative-extension-logic" },
    { pluginId: "imperative-extension-math", directoryName: "➕️imperative-extension-math" },
    { pluginId: "imperative-extension-text", directoryName: "🔡️imperative-extension-text" },
    { pluginId: "layout", directoryName: "📏️layout" },
    { pluginId: "lowpoly", directoryName: "💠️lowpoly" },
    { pluginId: "mathematical", directoryName: "➗️mathematical" },
    { pluginId: "norm", directoryName: "📕️norm" },
    { pluginId: "note", directoryName: "🗒️note" },
    { pluginId: "playbook", directoryName: "📖️playbook" },
    { pluginId: "playbook-module-procedural", directoryName: "⚙️playbook-module-procedural" },
    { pluginId: "procedural", directoryName: "🌀️procedural" },
    { pluginId: "process", directoryName: "🏭️process" },
    { pluginId: "process-extension-concrete", directoryName: "🏙️process-extension-concrete" },
    { pluginId: "process-extension-metal", directoryName: "🔩️process-extension-metal" },
    { pluginId: "process-extension-robotic", directoryName: "🤖️process-extension-robotic" },
    { pluginId: "process-extension-wood", directoryName: "🪓️process-extension-wood" },
    { pluginId: "puzzle", directoryName: "🧩️puzzle" },
    { pluginId: "raster", directoryName: "🖨️raster" },
    { pluginId: "reasoning-mindmap", directoryName: "💡️reasoning-mindmap" },
    { pluginId: "remodel", directoryName: "📸️remodel" },
    { pluginId: "s", directoryName: "🪐️s" },
    { pluginId: "sequence", directoryName: "🎬️sequence" },
    { pluginId: "shooting", directoryName: "🎥️shooting" },
    { pluginId: "sourcing", directoryName: "🪵️sourcing" },
    { pluginId: "sourcing-module-beams", directoryName: "🪜️sourcing-module-beams" },
    { pluginId: "sourcing-module-slabs", directoryName: "🧇️sourcing-module-slabs" },
    { pluginId: "sourcing-module-windows", directoryName: "🪟️sourcing-module-windows" },
    { pluginId: "stdio", directoryName: "🗄️stdio" },
    { pluginId: "trinity", directoryName: "🔱️trinity" },
    { pluginId: "vcs", directoryName: "🌿️vcs" },
    { pluginId: "writer", directoryName: "✒️writer" }
  ]
};
/* ../../../../../../🔌️plugin/📇️registry/📦️deployment/📐️schema.json */
var _schema_default = {
  $schema: "http://json-schema.org/draft-07/schema#",
  type: "object",
  additionalProperties: false,
  required: ["version", "modules"],
  definitions: {
    moduleRoutes: {
      type: "object",
      additionalProperties: false,
      required: ["plugin", "extension"],
      properties: {
        plugin: { const: "/🔌️plugin-modules" },
        extension: { const: "/🧩️extension-modules" }
      }
    }
  },
  properties: {
    version: { const: 1 },
    modules: {
      type: "array",
      minItems: 1,
      maxItems: 256,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["pluginId", "directoryName"],
        properties: {
          pluginId: { type: "string", pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$", maxLength: 128 },
          directoryName: { $ref: "semio:installation-directory-v1" }
        }
      }
    }
  }
};
/* ../../../../../../🔌️plugin/📇️registry/📦️deployment/🛣️routes.json */
var _routes_default = {
  plugin: "/🔌️plugin-modules",
  extension: "/🧩️extension-modules"
};
/* ../../../../../../🧩️extension/📐️directory.schema.json */
var _directory_schema_default = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: "semio:installation-directory-v1",
  type: "string",
  pattern: "^(?![📁📂📄])(?:\\p{Extended_Pictographic}\\uFE0F(?:\\u200D\\p{Extended_Pictographic}\\uFE0F)*|[0-9#*]\\uFE0F\\u20E3)[a-z0-9]+(?:-[a-z0-9]+)*$",
  maxLength: 192
};

/* ../../../../../../🧩️extension/🟦️.ts */
var pattern = new RegExp(_directory_schema_default.pattern, "u");
var segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });
function installationDirectoryEmoji(name) {
  if (typeof name !== "string" || [...name].length > _directory_schema_default.maxLength || name !== name.normalize("NFC") || !pattern.test(name))
    throw new Error("Installation directory requires one explicit non-generic emoji and a portable slug");
  return [...segmenter.segment(name)][0].segment.replaceAll("️", "");
}

/* ../../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts */
var idSpec = _schema_default.properties.modules.items.properties.pluginId;
var idPattern = new RegExp(idSpec.pattern, "u");
function parseModuleRoutes(input) {
  if (!input || typeof input !== "object" || Array.isArray(input))
    throw new Error("Invalid module routes");
  const value = input, properties = _schema_default.definitions.moduleRoutes.properties;
  if (Object.keys(value).sort().join(",") !== "extension,plugin" || value.plugin !== properties.plugin.const || value.extension !== properties.extension.const)
    throw new Error("Module routes must match their exact schema authority");
  return Object.freeze({ plugin: value.plugin, extension: value.extension });
}
var MODULE_ROUTES = parseModuleRoutes(_routes_default);
var MODULE_PLUGIN_ROUTE = MODULE_ROUTES.plugin;
var MODULE_EXTENSION_ROUTE = MODULE_ROUTES.extension;
function parseModuleDirectories(input) {
  if (!input || typeof input !== "object" || Array.isArray(input))
    throw new Error("Invalid module deployment catalog");
  const value = input;
  if (Object.keys(value).sort().join(",") !== "modules,version" || value.version !== 1 || !Array.isArray(value.modules) || value.modules.length < 1 || value.modules.length > 256)
    throw new Error("Invalid module deployment catalog fields");
  const ids = new Set, emojis = new Set;
  return Object.freeze(value.modules.map((entry) => {
    if (!entry || typeof entry !== "object" || Array.isArray(entry) || Object.keys(entry).sort().join(",") !== "directoryName,pluginId")
      throw new Error("Invalid module deployment row");
    if (typeof entry.pluginId !== "string" || entry.pluginId.length > idSpec.maxLength || !idPattern.test(entry.pluginId))
      throw new Error("Invalid public module identity");
    const emoji = installationDirectoryEmoji(entry.directoryName);
    if (ids.has(entry.pluginId) || emojis.has(emoji))
      throw new Error("Duplicate module identity or sibling emoji");
    ids.add(entry.pluginId);
    emojis.add(emoji);
    return Object.freeze({ pluginId: entry.pluginId, directoryName: entry.directoryName });
  }));
}
var MODULE_DIRECTORIES = parseModuleDirectories(_catalog_default2);
var MODULE_BRIDGE_FILE = "🌉️bridge.js";
function moduleDirectoryName(pluginId) {
  const row = MODULE_DIRECTORIES.find((entry) => entry.pluginId === pluginId);
  if (!row)
    throw new Error(`No hand-authored module directory for ${JSON.stringify(pluginId)}`);
  return row.directoryName;
}

/* ../../../../../../🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts */
var PLUGIN_HOST_CONFIGS = [
  { pluginId: "s", landingAppId: "home", hostAppId: "studio" }
];
var PLUGIN_BUILD_TARGETS = [
  { pluginId: "animate", packageId: "semio:animate", cratePath: "✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_animate.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:animate.present"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590", coreWasmSha256: "5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590", descriptorSha256: "12a912e82f98d54f405262123150f41035a15234332a1abc971062ac7e973b17" } },
  { pluginId: "architect", packageId: "semio:architect", cratePath: "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_architect.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:data.program"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "2301bc724c96c3f6ea698bc1eba4feb50a0b0b4d1dfdbffa94a912c7e9dab510", coreWasmSha256: "2301bc724c96c3f6ea698bc1eba4feb50a0b0b4d1dfdbffa94a912c7e9dab510", descriptorSha256: "09d0f7320243a4aa38d5c83fa7d0a75ed398756edcb093c848adf515d1c1c4d8" } },
  { pluginId: "block", packageId: "semio:block", cratePath: "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_block.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "cad", packageId: "semio:cad", cratePath: "✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_cad.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.cad"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "64a36cc37cb80d8d0c122af7c22272e1749730a45e2eb18657e435f6614c8823", coreWasmSha256: "64a36cc37cb80d8d0c122af7c22272e1749730a45e2eb18657e435f6614c8823", descriptorSha256: "ff3daed49568aaec15d35de6067f2df0956bf988de1db8baa98560f10063b867" } },
  { pluginId: "dag", packageId: "semio:dag", cratePath: "✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_dag.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:graph.dag"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "55c9da9026706dbcd47277335eda53abf66e3ecf19fd848280a95b7a531f51e2", coreWasmSha256: "55c9da9026706dbcd47277335eda53abf66e3ecf19fd848280a95b7a531f51e2", descriptorSha256: "53d81f2b0927fbc1383cccb1c989a5fe190fd98ea582786bd6ea1846aea5258d" } },
  { pluginId: "demonstrator", packageId: "semio:demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_demonstrator.wasm", role: "plugin", capabilities: [], contributes: [], consumes: ["forms.questionKind", "flow.extension", "process.machines"], dependsOn: ["cad", "gis", "procedural", "process", "puzzle", "sourcing", "stdio"], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "e39095467e06ec3d2fd45543e73bdcfa12d03e4a5a941d9145cd46f570d0ae63", coreWasmSha256: "e39095467e06ec3d2fd45543e73bdcfa12d03e4a5a941d9145cd46f570d0ae63", descriptorSha256: "72e0822284f68c9fd9fa60552db84cd489b1ca9c770adf389dd6e17cb57a2ff3" } },
  { pluginId: "draw", packageId: "semio:draw", cratePath: "✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_draw.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["draw-fsm", "stdio"], activationEvents: ["on-artifact-kind:2d.drawing"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "4bccf647dd64b0d6088e7338a25e7ed1326a412f44660459f0d6c9cab0e79714", coreWasmSha256: "4bccf647dd64b0d6088e7338a25e7ed1326a412f44660459f0d6c9cab0e79714", descriptorSha256: "b9d12f23271b085b41da39d7ba395ea78604cab8006b6b00e1ee39aa5265a1bd" } },
  { pluginId: "energy", packageId: "semio:energy", cratePath: "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_energy.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:data.model"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "1c0f620a5d442096c9683acf7095f470375c8b7efa0821076d8e548b8d706f20", coreWasmSha256: "1c0f620a5d442096c9683acf7095f470375c8b7efa0821076d8e548b8d706f20", descriptorSha256: "383853b475b0308336f8088fe067d27fa2f525b21349d70b080b07aa86ae2ec1" } },
  { pluginId: "fem", packageId: "semio:fem", cratePath: "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_fem.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.fem2d", "on-artifact-kind:computation.fem3d"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13", coreWasmSha256: "924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13", descriptorSha256: "f0c10888f9dc7101c596b0e8b837fcbd439cb031738dd233e767cc8ad59f6fdb" } },
  { pluginId: "flow", packageId: "semio:flow", cratePath: "✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["flow.extension"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.flow"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "b996f5722473bb19e91f3ab4b38cd67bd95cf1852586684e836a260a642eaed2", coreWasmSha256: "b996f5722473bb19e91f3ab4b38cd67bd95cf1852586684e836a260a642eaed2", descriptorSha256: "1996bf86c181d869f1d9839d3b4763146ce18c99da5a3c0cc67470398c10f2d4" } },
  { pluginId: "forms", packageId: "semio:forms", cratePath: "✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_forms.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["forms.questionKind"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:form.dictionary"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "a63d0dfc2619a9e7f05ae83c119717989ff8a32667f4771838c5c5599014b152", coreWasmSha256: "a63d0dfc2619a9e7f05ae83c119717989ff8a32667f4771838c5c5599014b152", descriptorSha256: "8e0b3d00eb48790dd1f31070462adaf925fc00cdc8a664c1865366b6589c0d88" } },
  { pluginId: "gis", packageId: "semio:gis", cratePath: "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_gis.wasm", role: "plugin", capabilities: ["documents.write", "shell.navigate"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "78a180b8fcf22d57778a88b5bc93821e832ebf63bf23d90ae36bfd1e756c27eb", coreWasmSha256: "90a7cd5fc5aa1d0ccb755c7a920ea779d60398e942305ea6bc71478f43d0f15a", descriptorSha256: "5be61f0b3aab6dc86dbb992810d5f4cc5632dd32e27a74d966fa47ba033af06b" } },
  { pluginId: "imperative", packageId: "semio:imperative", cratePath: "✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.procedure"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "32cdff3f114c8390f85c3f7ed928525d25ed52be15b147cbfa58ec64a0e4234f", coreWasmSha256: "32cdff3f114c8390f85c3f7ed928525d25ed52be15b147cbfa58ec64a0e4234f", descriptorSha256: "7dc6bc0885b16f4a552ecdf5e1757da8d336efebcb81b603c87341ae25a66506" } },
  { pluginId: "layout", packageId: "semio:layout", cratePath: "✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_layout.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.layout"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "dfde964f079e83c8f8cc67873cd495448be7a06ac8f6776e8585aef4b4f5b0bc", coreWasmSha256: "dfde964f079e83c8f8cc67873cd495448be7a06ac8f6776e8585aef4b4f5b0bc", descriptorSha256: "66358711ac5cd24af7edebf20ba9e40c3a7d96bb9e28ba19bc9d548b62c026db" } },
  { pluginId: "lowpoly", packageId: "semio:lowpoly", cratePath: "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_lowpoly.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["cad", "stdio"], activationEvents: ["on-artifact-kind:3d.lowpoly"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "95f9ac4920995ae69e8807c90be68082694a15b2466910d3cf257476a8940c02", coreWasmSha256: "95f9ac4920995ae69e8807c90be68082694a15b2466910d3cf257476a8940c02", descriptorSha256: "2e2e5e1e43988b270aa356d10fca3608c594faa7b7f6a47b9c1efa93fbb45751" } },
  { pluginId: "mathematical", packageId: "semio:mathematical", cratePath: "✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_mathematical.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:computation.equation"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "0b801ea2f23f760c1b8b2b24a7f137af965cc5825da11065cac51cd179b14716", coreWasmSha256: "0b801ea2f23f760c1b8b2b24a7f137af965cc5825da11065cac51cd179b14716", descriptorSha256: "824b2c80a380ac3cebb2c39ec5ff9b95282fb98e6888f6c91293f85e0263b227" } },
  { pluginId: "norm", packageId: "semio:norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_norm.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["fem", "stdio"], activationEvents: ["on-artifact-kind:computation.norm.din4108", "on-artifact-kind:computation.norm.din16798", "on-artifact-kind:computation.norm.din18599", "on-artifact-kind:computation.norm.en1990", "on-artifact-kind:computation.norm.en1991", "on-artifact-kind:computation.norm.en1992", "on-artifact-kind:computation.norm.en1993", "on-artifact-kind:computation.norm.en1994", "on-artifact-kind:computation.norm.en1995", "on-artifact-kind:computation.norm.en1996", "on-artifact-kind:computation.norm.en1997", "on-artifact-kind:computation.norm.en1998", "on-artifact-kind:computation.norm.en1999", "on-artifact-kind:computation.norm.iso16757", "on-artifact-kind:computation.norm.vdi3805"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "ee09ede9e0a96f42d31342b2e646edfb17b05f3d63b47315148774eb9f99dbfc", coreWasmSha256: "ee09ede9e0a96f42d31342b2e646edfb17b05f3d63b47315148774eb9f99dbfc", descriptorSha256: "dbca604de90af12da82cb423792a4ced55422e75c1f1baee863caf898f0295c3" } },
  { pluginId: "note", packageId: "semio:note", cratePath: "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_note.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.note"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7", coreWasmSha256: "a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7", descriptorSha256: "1b8c29c800f1fd38f95f6754ec982585b59595a60ddf06fdbbadb6738850a093" } },
  { pluginId: "playbook", packageId: "semio:playbook", cratePath: "✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_playbook.wasm", role: "plugin", capabilities: [], contributes: [], consumes: ["playbook.blockKind"], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "procedural", packageId: "semio:procedural", cratePath: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_procedural.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["forms.questionKind", "flow.extension"], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "42503bf34bf77e69d5e730d75f9b58ce3666f42eadd4b3f810d2d80af69a96bd", coreWasmSha256: "370310791b85c6cc96ea370aa50e34dea530c256abc8b606139b9049064ee71c", descriptorSha256: "932ed0810ec0b2fd27381f847f343bcdacc1e5b683db7c853d8128779b568a79" } },
  { pluginId: "process", packageId: "semio:process", cratePath: "✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_process.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: ["process.machines"], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.process"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "adf1fc2a97ec390e8c2e6f26f474fbcb211e63d5ca7a45e8eb930c6db934abeb", coreWasmSha256: "63e4f7d59977ac763a86abbc6c3e0e51e0cbb4ee3aa0556685d4f8c04729e7b8", descriptorSha256: "5797c6564c2600528f82605534cc116069f5b5c1fffcecd53b46449f3d9526a6" } },
  { pluginId: "puzzle", packageId: "semio:puzzle", cratePath: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_puzzle.wasm", role: "plugin", capabilities: ["documents.write", "ui.dialog", "shell.clipboard"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "9a44199ad3131cd1317895dfe2ae29915c7846cd24a9910c00f6f9357d1be942", coreWasmSha256: "b273db280223bc136ccf3eca82c648f148cb0ca167f77eda27db5a9b17b79914", descriptorSha256: "cbec74394759b25b11065af8bdf0380c51d812d683ebb99af3568b564adfd31d" } },
  { pluginId: "raster", packageId: "semio:raster", cratePath: "✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_raster.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.raster"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6", coreWasmSha256: "9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6", descriptorSha256: "26760a5a3c146b1612a8e8036c877f91a17c13cef425b94a174127df3e33bd94" } },
  { pluginId: "reasoning-mindmap", packageId: "semio:reasoning-mindmap", cratePath: "✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_reasoning_mindmap.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:graph.wires"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "7686a3193c6aeffe74e8e73d76b842112e892e57f9f3aa9ed04d39bc8bc1c2b8", coreWasmSha256: "7686a3193c6aeffe74e8e73d76b842112e892e57f9f3aa9ed04d39bc8bc1c2b8", descriptorSha256: "eb21b2587a19242762803823f748628b1eb1553c783f6281dfee25ac72706f93" } },
  { pluginId: "remodel", packageId: "semio:remodel", cratePath: "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_remodel.wasm", role: "plugin", capabilities: ["documents.write", "ui.dialog"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:3d.remodel"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "77ef3c98d134f1164cdd388911333b0618bcec94fead7c11ad6fdd24abb125b5", coreWasmSha256: "77ef3c98d134f1164cdd388911333b0618bcec94fead7c11ad6fdd24abb125b5", descriptorSha256: "1e1dded5a4979ce72c0ff11f4e12e8336df93784c89c0f53b0ee573b694fbe62" } },
  { pluginId: "s", packageId: "semio:s", cratePath: "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_space.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:space.shome", "on-artifact-kind:space.sspace"], extensionPoints: [], host: { landingAppId: "home", hostAppId: "studio" }, executionMode: "isolated", hashes: { wasmSha256: "762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af", coreWasmSha256: "762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af", descriptorSha256: "df021b9a83bcb48ab858afe4a8f2c2e30d69f8166850ddebb064421109b3fed6" } },
  { pluginId: "sequence", packageId: "semio:sequence", cratePath: "✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_sequence.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["imperative-control", "imperative-effect", "imperative-math", "imperative-text", "stdio"], activationEvents: ["on-artifact-kind:computation.sequence"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "bbcf24176893beb37e0dcdf36f658f52a62b8a5e48163130cd5f02371b2a6a79", coreWasmSha256: "bbcf24176893beb37e0dcdf36f658f52a62b8a5e48163130cd5f02371b2a6a79", descriptorSha256: "5c5ee126f62f14b60a81d95575c85186db47ec9b7712d0e56d5ba6b2a032088a" } },
  { pluginId: "shooting", packageId: "semio:shooting", cratePath: "✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_shooting.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:2d.shooting"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "2e16eed70a875e078501c439d8f05c162163f1193bcaee4f11b41f0b2f2eed01", coreWasmSha256: "2e16eed70a875e078501c439d8f05c162163f1193bcaee4f11b41f0b2f2eed01", descriptorSha256: "ad86c4d9cf0730ae4b512389898962bb9eefd1f631f8543d7fd8143be3276129" } },
  { pluginId: "sourcing", packageId: "semio:sourcing", cratePath: "✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_sourcing.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "c27638455e4eba364a044826adb2e5ad2b679c88601d80757f40e354c4c12298", coreWasmSha256: "81b04b6396cf37bd2fee9119cd802592bbf51a40ceef05c7af4c710801bc9045", descriptorSha256: "fa7ea0be8379f959e0e9b7bbf2c5ae4168a3b29104e8d9d2015c27847632ac28" } },
  { pluginId: "stdio", packageId: "semio:stdio", cratePath: "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_stdio.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: [], activationEvents: [], extensionPoints: [] },
  { pluginId: "trinity", packageId: "semio:trinity", cratePath: "✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_trinity.wasm", role: "plugin", capabilities: [], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: [], extensionPoints: [] },
  { pluginId: "vcs", packageId: "semio:vcs", cratePath: "✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_vcs.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio"], activationEvents: ["on-artifact-kind:vcs.document"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "74771b987f39e483da63efdb21006a3ce511ad5edd1c3bd0de05543bef00d925", coreWasmSha256: "74771b987f39e483da63efdb21006a3ce511ad5edd1c3bd0de05543bef00d925", descriptorSha256: "b702fe11bb1c92bb06226ccce58792ccd37fa01be8313a740e52ea6a48e8329e" } },
  { pluginId: "writer", packageId: "semio:writer", cratePath: "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_writer.wasm", role: "plugin", capabilities: ["documents.write"], contributes: [], consumes: [], dependsOn: ["stdio", "trinity"], activationEvents: ["on-artifact-kind:text.document"], extensionPoints: [], executionMode: "isolated", hashes: { wasmSha256: "6507f654884a1c93e633bfc4cd42b5cebb880925ca4e03ca278bc9ccf191c18e", coreWasmSha256: "6507f654884a1c93e633bfc4cd42b5cebb880925ca4e03ca278bc9ccf191c18e", descriptorSha256: "ced53b5c3f821e2cb2fc847868737e6c695e8e32aa9c2358886df559214e750d" } }
];
var EXTENSION_TARGETS = [
  { pluginId: "cad-extension-aec-building", packageId: "semio:cad-extension-aec-building", cratePath: "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_cad_aec_building.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "af59b52fd8c7f60d5eb1195406a65d4eaf2de59b471fe54ddddd9dd1ec7d70c0", coreWasmSha256: "af59b52fd8c7f60d5eb1195406a65d4eaf2de59b471fe54ddddd9dd1ec7d70c0", descriptorSha256: "4f06e341b211c507f489e3929838512d79015d79a3b8fd97f3c4ef1f3a2ee43e" } },
  { pluginId: "cad-extension-aec-building-energy", packageId: "semio:cad-extension-aec-building-energy", cratePath: "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_cad_aec_building_energy.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "e5b2ff618804be66178d53f4f5302d9e08974e7f982a94163e812ddd7c315722", coreWasmSha256: "e5b2ff618804be66178d53f4f5302d9e08974e7f982a94163e812ddd7c315722", descriptorSha256: "c49d812a1ef6056b2a7a38b886a91d5b2ceb3bce0f498d4e2f7fd5e709b8489f" } },
  { pluginId: "cad-extension-aec-building-structure", packageId: "semio:cad-extension-aec-building-structure", cratePath: "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_cad_aec_building_structure.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "ec7281c5e733b921760a7a365660b0d105442b20c9064168b28168f92bc97fc9", coreWasmSha256: "ec7281c5e733b921760a7a365660b0d105442b20c9064168b28168f92bc97fc9", descriptorSha256: "71cd360006a6b82ab5e42bbce01580600441234af040753100200e3f2f736dc4" } },
  { pluginId: "cad-extension-spatial-shape", packageId: "semio:cad-extension-spatial-shape", cratePath: "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_cad_spatial_shape.wasm", role: "extension", capabilities: [], contributes: [], consumes: [], dependsOn: ["cad"], activationEvents: [], extensionPoints: [], extends: "cad", executionMode: "isolated", hashes: { wasmSha256: "d77ec8ebc85fd286e5cdb3f24d037137461e9293524cbf94d7809a7d58fd98ab", coreWasmSha256: "d77ec8ebc85fd286e5cdb3f24d037137461e9293524cbf94d7809a7d58fd98ab", descriptorSha256: "7919165697487d2cdee0c6e4162a25dc8f6e57d0fa751f6239ed5b6b872de5a5" } },
  { pluginId: "flow-extension-bim", packageId: "semio:flow-extension-bim", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_bim.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-brep", packageId: "semio:flow-extension-brep", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_brep.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow", "stdio"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "a5b648c7575d312ab9c65fad854ef5c33552361e6c72cb61316d006a8694dfc2", coreWasmSha256: "0d14dfa6d5bb5c69b0a39c7f716b4c327f3de3275139db8d2541cc310328d550", descriptorSha256: "da5ccae1e44d4022546d87ea82a30e223a58266c58b0efc9e1635497924251f3" } },
  { pluginId: "flow-extension-dictionary", packageId: "semio:flow-extension-dictionary", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_dictionary.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "a6c38efadee3b569ceb61a618951c6bfecab568bd3819a69acb0bdfff12ff29d", coreWasmSha256: "9be9766880776481c47af78498492ba7d71937ce2bea3595900e153124a7643e", descriptorSha256: "56207a8c59d586c53d6603c80812c5d7be175d123dde5a3e24c2210191a94543" } },
  { pluginId: "flow-extension-draw", packageId: "semio:flow-extension-draw", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_draw.wasm", role: "extension", capabilities: ["flow.extension"], contributes: ["flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow" },
  { pluginId: "flow-extension-list", packageId: "semio:flow-extension-list", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_list.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "b704f249a888288f4d13f4c322371f7eb744dd915545ba7387ce8b24053170ab", coreWasmSha256: "05963254be2f7b930616e3fab999de8886c123fd21d64ea2efec378b0d923b9e", descriptorSha256: "dfe347d736dce2eb6cefe3937b368182fd8fe464c389612d84995bc2ccc7fdfe" } },
  { pluginId: "flow-extension-logic", packageId: "semio:flow-extension-logic", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_logic.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "ec11011c12be2573da2aac46df43d20c709df1f06d29ec5c9d34f778b0aeffda", coreWasmSha256: "c2d98a64566f42468585b89d08dce5a0be8b64b8ee8465678ca520f494de15ba", descriptorSha256: "0effe44b7293cbd4029ec14c974c7a702e27c4e9f1510484b63a5b4c19ef2b2d" } },
  { pluginId: "flow-extension-math", packageId: "semio:flow-extension-math", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_math.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "db18a550fc3efba8c5f356fb671b8a96f62bcdeb811fa9e2d7580bc7d9ef2379", coreWasmSha256: "133e8cbf94e80d6172aa3a0c8a1edddb240e9094bc424cb1799410a1f156c54d", descriptorSha256: "805d0e505a71c67245b511c475ddcdcd66947c631fd8053b091e221595a5cff8" } },
  { pluginId: "flow-extension-primitive", packageId: "semio:flow-extension-primitive", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_primitive.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "0f73a465132b44308438bf12bebee2e8163965d34047d0e088d2b36d71b38424", coreWasmSha256: "7b448976b669c674f6a800668616cb8112132adae92acfe83a503a08f50fbe8c", descriptorSha256: "d0db4f5d79aef0bfd8454b0d76656d421ceff0ccb7d6534624ff9e25c6cd156d" } },
  { pluginId: "flow-extension-text", packageId: "semio:flow-extension-text", cratePath: "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_flow_extension_text.wasm", role: "extension", capabilities: [], contributes: ["flow.extension", "flow.extension"], consumes: [], dependsOn: ["flow"], activationEvents: [], extensionPoints: [], extends: "flow", executionMode: "linked", hashes: { wasmSha256: "c2d238545722d63e064198cf29bb7ebc927db713bfecf947665f5a328e02c1d7", coreWasmSha256: "41ac9ea01f64d68575dca620d2380a89496cb4159ed7b42a5879c7008ed8835a", descriptorSha256: "26a92210dbafe6b41b306076f3beee34e3f756d7164bc96e20b9f2d0dd360003" } },
  { pluginId: "imperative-extension-control", packageId: "semio:imperative-extension-control", cratePath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative_control.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-effect", packageId: "semio:imperative-extension-effect", cratePath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative_effect.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-logic", packageId: "semio:imperative-extension-logic", cratePath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative_logic.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-math", packageId: "semio:imperative-extension-math", cratePath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative_math.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "imperative-extension-text", packageId: "semio:imperative-extension-text", cratePath: "✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_imperative_text.wasm", role: "extension", capabilities: ["imperative.module"], contributes: ["imperative.module"], consumes: [], dependsOn: ["imperative"], activationEvents: [], extensionPoints: [], extends: "imperative" },
  { pluginId: "playbook-module-procedural", packageId: "semio:playbook-module-procedural", cratePath: "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_playbook_procedural.wasm", role: "extension", capabilities: ["playbook.blockKind"], contributes: ["playbook.blockKind"], consumes: [], dependsOn: ["playbook"], activationEvents: [], extensionPoints: [], extends: "playbook" },
  { pluginId: "process-extension-concrete", packageId: "semio:process-extension-concrete", cratePath: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_process_concrete.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-metal", packageId: "semio:process-extension-metal", cratePath: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_process_metal.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-robotic", packageId: "semio:process-extension-robotic", cratePath: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_process_robotic.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "process-extension-wood", packageId: "semio:process-extension-wood", cratePath: "✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_process_wood.wasm", role: "extension", capabilities: ["process.machines"], contributes: ["process.machines"], consumes: [], dependsOn: ["process"], activationEvents: [], extensionPoints: [], extends: "process" },
  { pluginId: "sourcing-module-beams", packageId: "semio:sourcing-module-beams", cratePath: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_sourcing_beams.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" },
  { pluginId: "sourcing-module-slabs", packageId: "semio:sourcing-module-slabs", cratePath: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_sourcing_slabs.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" },
  { pluginId: "sourcing-module-windows", packageId: "semio:sourcing-module-windows", cratePath: "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_sourcing_windows.wasm", role: "extension", capabilities: ["sourcing.module"], contributes: ["sourcing.module"], consumes: [], dependsOn: ["sourcing"], activationEvents: [], extensionPoints: [], extends: "sourcing" }
];
var PROGRAM_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
  pluginId: target.pluginId,
  moduleUrl: `${MODULE_PLUGIN_ROUTE}/${moduleDirectoryName(target.pluginId)}/${MODULE_BRIDGE_FILE}`
}));
var pluginModuleUrl = (pluginId) => `${MODULE_PLUGIN_ROUTE}/${moduleDirectoryName(pluginId)}/${MODULE_BRIDGE_FILE}`;
var extensionModuleUrl = (extensionId) => `${MODULE_EXTENSION_ROUTE}/${moduleDirectoryName(extensionId)}/${MODULE_BRIDGE_FILE}`;

/* ../../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts */
var PLAYGROUND_BUILD_TARGETS = [
  { variant: "aggregator", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.puzzle.puzzle3d@1/*#editor", brand: "entwerfen-mit-bestand-aggregator", aliases: ["mit-bestand", "entwerfen-mit-bestand"], ports: { react: 6023, wgpu: 6123 }, examples: [], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json" }, { kind: "static-dir", route: "/infinite-fixture", root: "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures" }] },
  { variant: "animate", pluginId: "animate", cratePath: "✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust", aliases: [], ports: { react: 6051, wgpu: 6151 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "architect", pluginId: "architect", cratePath: "✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust", aliases: [], ports: { react: 6090, wgpu: 6190 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "aussuchen", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.sourcing.curation@1/*#editor", brand: "entwerfen-mit-bestand-aussuchen", aliases: ["entwerfen-mit-bestand-aussuchen"], ports: { react: 6030, wgpu: 6130 }, examples: [], engines: [], assets: [] },
  { variant: "bearbeiten", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.process.process3d@1/*#editor", brand: "entwerfen-mit-bestand-bearbeiten", aliases: ["entwerfen-mit-bestand-bearbeiten"], ports: { react: 6031, wgpu: 6131 }, examples: [], engines: [], assets: [] },
  { variant: "block2d", pluginId: "block", cratePath: "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust", app: "s.block.block2d@1/*#editor", aliases: ["block 2d"], ports: { react: 6024, wgpu: 6124 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "block3d", pluginId: "block", cratePath: "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust", app: "s.block.block3d@1/*#editor", aliases: ["block 3d"], ports: { react: 6025, wgpu: 6125 }, examples: ["🎬️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json" }] },
  { variant: "block5d", pluginId: "block", cratePath: "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust", app: "s.block.block5d@1/*#editor", aliases: ["block 5d"], ports: { react: 6026, wgpu: 6126 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "cad", pluginId: "cad", cratePath: "✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust", app: "s.cad.cad@1/*#editor", aliases: [], ports: { react: 6020, wgpu: 6120 }, examples: ["🎬️demo-session"], engines: [], assets: [{ kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧫️fixtures" }] },
  { variant: "dag", pluginId: "dag", cratePath: "✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust", aliases: [], ports: { react: 6017, wgpu: 6117 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "demonstrator", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.demonstrator.playground@1/*#editor", aliases: [], ports: { react: 6107, wgpu: 6207 }, examples: [], engines: [], assets: [] },
  { variant: "din16798", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.din16798@1/*#editor", aliases: [], ports: { react: 6092, wgpu: 6192 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "din18599", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.din18599@1/*#editor", aliases: [], ports: { react: 6093, wgpu: 6193 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "din4108", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.din4108@1/*#editor", aliases: [], ports: { react: 6091, wgpu: 6191 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "draw", pluginId: "draw", cratePath: "✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust", aliases: [], ports: { react: 6064, wgpu: 6164 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1990", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1990@1/*#editor", aliases: [], ports: { react: 6094, wgpu: 6194 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1991", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1991@1/*#editor", aliases: [], ports: { react: 6095, wgpu: 6195 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1992", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1992@1/*#editor", aliases: [], ports: { react: 6096, wgpu: 6196 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1993", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1993@1/*#editor", aliases: [], ports: { react: 6097, wgpu: 6197 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1994", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1994@1/*#editor", aliases: [], ports: { react: 6098, wgpu: 6198 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1995", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1995@1/*#editor", aliases: [], ports: { react: 6099, wgpu: 6199 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1996", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1996@1/*#editor", aliases: [], ports: { react: 6100, wgpu: 6200 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1997", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1997@1/*#editor", aliases: [], ports: { react: 6101, wgpu: 6201 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1998", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1998@1/*#editor", aliases: [], ports: { react: 6102, wgpu: 6202 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "en1999", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.en1999@1/*#editor", aliases: [], ports: { react: 6103, wgpu: 6203 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "energy", pluginId: "energy", cratePath: "✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust", app: "s.energy.model@1/*#editor", aliases: [], ports: { react: 6106, wgpu: 6206 }, examples: [], engines: [], assets: [] },
  { variant: "fem2d", pluginId: "fem", cratePath: "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", app: "s.fem.fem2d@1/*#editor", aliases: ["fem 2d"], ports: { react: 6086, wgpu: 6186 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "fem3d", pluginId: "fem", cratePath: "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust", app: "s.fem.fem3d@1/*#editor", aliases: ["fem 3d"], ports: { react: 6087, wgpu: 6187 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "flow", pluginId: "flow", cratePath: "✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust", aliases: [], ports: { react: 6016, wgpu: 6116 }, examples: ["🎬️demo-session"], engines: ["./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings"], assets: [] },
  { variant: "forms", pluginId: "forms", cratePath: "✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust", aliases: [], ports: { react: 6058, wgpu: 6158 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "generation2d", pluginId: "procedural", cratePath: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust", app: "s.procedural.generation2d@1/*#editor", aliases: ["procedural 2d"], ports: { react: 6021, wgpu: 6121 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "generation3d", pluginId: "procedural", cratePath: "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust", app: "s.procedural.generation3d@1/*#editor", aliases: ["procedural 3d"], ports: { react: 6018, wgpu: 6118 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "generator", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.procedural.generation3d@1/*#editor", brand: "entwerfen-mit-bestand-generator", aliases: ["entwerfen-mit-bestand-generator"], ports: { react: 6027, wgpu: 6127 }, examples: [], engines: [], assets: [] },
  { variant: "gis2d", pluginId: "gis", cratePath: "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust", app: "s.gis.gismap@1/*#editor", aliases: ["gis 2d"], ports: { react: 6040, wgpu: 6140 }, examples: ["🎬️demo-session"], engines: ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"], assets: [{ kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" }, { kind: "tile-proxy", route: "/vt", upstream: "https://tiles.openfreemap.org/planet", cache: "openfreemap-vt" }] },
  { variant: "gis3d", pluginId: "gis", cratePath: "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust", app: "s.gis.gisterrain@1/*#editor", aliases: ["gis 3d"], ports: { react: 6083, wgpu: 6183 }, examples: ["🎬️demo-session"], engines: ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"], assets: [{ kind: "tile-proxy", route: "/dem", upstream: "https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png", cache: "terrarium-dem" }] },
  { variant: "imperative", pluginId: "imperative", cratePath: "✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust", aliases: [], ports: { react: 6076, wgpu: 6176 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "iso16757", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.iso16757@1/*#editor", aliases: [], ports: { react: 6104, wgpu: 6204 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "koordinator", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.cad.cad@1/*#editor", brand: "entwerfen-mit-bestand-koordinator", aliases: ["entwerfen-mit-bestand-koordinator"], ports: { react: 6028, wgpu: 6128 }, examples: [], engines: [], assets: [{ kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧫️fixtures" }] },
  { variant: "layout", pluginId: "layout", cratePath: "✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust", aliases: [], ports: { react: 6079, wgpu: 6179 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "lowpoly", pluginId: "lowpoly", cratePath: "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust", aliases: [], ports: { react: 6078, wgpu: 6178 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "mathematical", pluginId: "mathematical", cratePath: "✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust", app: "s.mathematical.equation@1/*#editor", aliases: ["mathematical", "math"], ports: { react: 6084, wgpu: 6184 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "note", pluginId: "note", cratePath: "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust", aliases: [], ports: { react: 6080, wgpu: 6180 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "playbook", pluginId: "playbook", cratePath: "✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust", aliases: [], ports: { react: 6085, wgpu: 6185 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "process3d", pluginId: "process", cratePath: "✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust", app: "s.process.process3d@1/*#editor", aliases: ["process 3d"], ports: { react: 6022, wgpu: 6122 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "puzzle2d", pluginId: "puzzle", cratePath: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust", app: "s.puzzle.puzzle2d@1/*#editor", aliases: ["2d", "puzzle 2d"], ports: { react: 6012, wgpu: 6112 }, examples: ["🎬️demo-session"], engines: ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"], assets: [] },
  { variant: "puzzle3d", pluginId: "puzzle", cratePath: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust", app: "s.puzzle.puzzle3d@1/*#editor", aliases: ["3d", "puzzle 3d"], ports: { react: 6013, wgpu: 6113 }, examples: ["🎬️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json" }, { kind: "static-dir", route: "/infinite-fixture", root: "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures" }] },
  { variant: "puzzle5d", pluginId: "puzzle", cratePath: "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust", app: "s.puzzle.puzzle5d@1/*#editor", aliases: ["5d", "puzzle 5d"], ports: { react: 6014, wgpu: 6114 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "raster", pluginId: "raster", cratePath: "✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust", aliases: [], ports: { react: 6060, wgpu: 6160 }, examples: ["🎬️demo-session"], engines: ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"], assets: [] },
  { variant: "reasoning-wires", pluginId: "reasoning-mindmap", cratePath: "✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust", aliases: ["wires"], ports: { react: 6015, wgpu: 6115 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "remodel", pluginId: "remodel", cratePath: "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust", aliases: [], ports: { react: 6063, wgpu: 6163 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "s", pluginId: "s", cratePath: "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust", aliases: [], ports: { react: 6070, wgpu: 6066 }, userPorts: { react: [6072, 6073], wgpu: [6067, 6068] }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "sequence", pluginId: "sequence", cratePath: "✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust", aliases: [], ports: { react: 6077, wgpu: 6177 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "shooting", pluginId: "shooting", cratePath: "✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust", aliases: [], ports: { react: 6019, wgpu: 6119 }, examples: ["🎬️demo-session"], engines: [], assets: [{ kind: "mesh-collection", route: "/mesh", catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json" }] },
  { variant: "sourcing", pluginId: "sourcing", cratePath: "✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust", app: "s.sourcing.curation@1/*#editor", aliases: ["curation"], ports: { react: 6081, wgpu: 6181 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "trinity-jack", pluginId: "trinity", cratePath: "✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust", app: "s.trinity.jack@1/*#editor", aliases: ["trinity jack"], ports: { react: 6054, wgpu: 6154 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "trinity-rewriting", pluginId: "trinity", cratePath: "✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust", app: "s.trinity.rewriting@1/*#editor", aliases: ["trinity rewriting"], ports: { react: 6056, wgpu: 6156 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "vcs", pluginId: "vcs", cratePath: "✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust", aliases: [], ports: { react: 6075, wgpu: 6175 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "vdi3805", pluginId: "norm", cratePath: "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust", app: "s.norm.vdi3805@1/*#editor", aliases: [], ports: { react: 6105, wgpu: 6205 }, examples: ["🎬️demo-session"], engines: [], assets: [] },
  { variant: "verfolgen", pluginId: "demonstrator", cratePath: "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust", app: "s.gis.gismap@1/*#editor", brand: "entwerfen-mit-bestand-verfolgen", aliases: ["entwerfen-mit-bestand-verfolgen"], ports: { react: 6032, wgpu: 6132 }, examples: [], engines: ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"], assets: [{ kind: "tile-proxy", route: "/osm", upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png", cache: "osm-tiles" }, { kind: "tile-proxy", route: "/vt", upstream: "https://tiles.openfreemap.org/planet", cache: "openfreemap-vt" }] },
  { variant: "writer", pluginId: "writer", cratePath: "✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust", aliases: [], ports: { react: 6062, wgpu: 6162 }, examples: ["🎬️demo-session"], engines: [], assets: [] }
];

/* ../../../../../../🔌️plugin/📇️registry/🟦️.ts */
function toCatalogTarget(target) {
  return { pluginId: target.pluginId, wasmOut: target.wasmOut, role: target.role, contributes: target.contributes, consumes: target.consumes, dependsOn: target.dependsOn };
}
function toPlaygroundCatalogTarget(target) {
  return { variant: target.variant, pluginId: target.pluginId, app: target.app, aliases: target.aliases };
}
function buildPluginCatalog() {
  return {
    plugins: PLUGIN_BUILD_TARGETS.map(toCatalogTarget),
    extensions: EXTENSION_TARGETS.map(toCatalogTarget),
    hosts: PLUGIN_HOST_CONFIGS,
    playgrounds: PLAYGROUND_BUILD_TARGETS.map(toPlaygroundCatalogTarget),
    moduleUrl: pluginModuleUrl,
    extensionModuleUrl
  };
}
var PLUGIN_CATALOG = buildPluginCatalog();

/* ../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout.ts */
var DIAGRAM_UNIT = 48;
var DIAGRAM_LAYOUT_CODEC_KIND = "diagram-directed-layout-v1";
var DIAGRAM_LAYOUT_INGRESS_ITEMS = 64;
var DIAGRAM_LAYOUT_INGRESS_BYTES = 16 * 1024;
var DIAGRAM_LAYOUT_OUTPUT_ITEMS = 128;
var DIAGRAM_LAYOUT_MAX_INPUT_ITEMS = 65536;
var DIAGRAM_LAYOUT_MAX_ID_CHARACTERS = 512;
var DIAGRAM_LAYOUT_MAX_NODE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4;
var DIAGRAM_LAYOUT_MAX_EDGE_BYTES = 64 + DIAGRAM_LAYOUT_MAX_ID_CHARACTERS * 4 * 3;
var DIAGRAM_LAYOUT_MAX_RESERVED_BYTES = 256 * 1024 * 1024;
function diagramLayoutUtf8Bytes(value) {
  let bytes = 0;
  let characters = 0;
  for (let index = 0;index < value.length; index++) {
    characters += 1;
    if (characters > DIAGRAM_LAYOUT_MAX_ID_CHARACTERS)
      throw new Error("Diagram layout id exceeds 512 Unicode characters");
    const code = value.charCodeAt(index);
    if (code <= 127)
      bytes += 1;
    else if (code <= 2047)
      bytes += 2;
    else if (code >= 55296 && code <= 56319 && index + 1 < value.length && value.charCodeAt(index + 1) >= 56320 && value.charCodeAt(index + 1) <= 57343) {
      bytes += 4;
      index += 1;
    } else
      bytes += 3;
  }
  return bytes;
}
function diagramLayoutNodeWireBytes(value) {
  return 64 + diagramLayoutUtf8Bytes(value.id);
}
function diagramLayoutEdgeWireBytes(value) {
  return 64 + diagramLayoutUtf8Bytes(value.id) + diagramLayoutUtf8Bytes(value.source) + diagramLayoutUtf8Bytes(value.target);
}
function diagramLayoutIdentityAdmitted(value, allowEmpty = false) {
  if (typeof value !== "string" || !allowEmpty && value.length === 0)
    return false;
  try {
    diagramLayoutUtf8Bytes(value);
    return true;
  } catch {
    return false;
  }
}
function diagramLayoutCredits(nodeCount, edgeCount) {
  if (!Number.isSafeInteger(nodeCount) || !Number.isSafeInteger(edgeCount) || nodeCount < 0 || edgeCount < 0 || nodeCount + edgeCount > DIAGRAM_LAYOUT_MAX_INPUT_ITEMS)
    return { admitted: false, reason: "items" };
  const inputBytes = nodeCount * DIAGRAM_LAYOUT_MAX_NODE_BYTES + edgeCount * DIAGRAM_LAYOUT_MAX_EDGE_BYTES;
  const outputBytes = nodeCount * 32;
  if (!Number.isSafeInteger(inputBytes) || inputBytes + outputBytes > DIAGRAM_LAYOUT_MAX_RESERVED_BYTES)
    return { admitted: false, reason: "bytes" };
  return { admitted: true, inputBytes, inputItems: nodeCount + edgeCount, outputBytes, outputItems: nodeCount };
}
function asDiagramLayoutSource(values) {
  if ("get" in values && typeof values.get === "function")
    return values;
  const array = values;
  return { get: (index) => array[index], length: array.length };
}
var diagramLayoutLimits = Object.freeze({ maxEdges: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, maxNodes: DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, previewNodes: 128 });
var diagramLayoutFrame = Object.freeze({ fuel: 16384, milliseconds: 6 });
var diagramLayoutPageSize = 128;

class DiagramPagedStore {
  capacity;
  directories = new Array(16);
  count = 0;
  pageHighWater = 0;
  constructor(capacity) {
    this.capacity = capacity;
  }
  get length() {
    return this.count;
  }
  get(index) {
    if (index < 0 || index >= this.count)
      return;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    return this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32]?.[index % diagramLayoutPageSize];
  }
  set(index, value) {
    if (index < 0 || index >= this.capacity)
      throw new Error("Diagram layout page capacity exceeded");
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex] ?? (this.directories[directoryIndex] = new Array(32));
    const page = directory[pageIndex % 32] ?? (directory[pageIndex % 32] = new Array(diagramLayoutPageSize));
    this.pageHighWater = Math.max(this.pageHighWater, pageIndex + 1);
    page[index % diagramLayoutPageSize] = value;
    if (index >= this.count)
      this.count = index + 1;
  }
  push(value) {
    const index = this.count;
    this.set(index, value);
    return index;
  }
  pop() {
    if (this.count === 0)
      return;
    const index = --this.count;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    const page = directory?.[pageIndex % 32];
    const value = page?.[index % diagramLayoutPageSize];
    if (page)
      page[index % diagramLayoutPageSize] = undefined;
    if (index % diagramLayoutPageSize === 0 && directory)
      directory[pageIndex % 32] = undefined;
    return value;
  }
  take(index) {
    if (index < 0 || index >= this.count)
      return;
    const pageIndex = Math.floor(index / diagramLayoutPageSize);
    const page = this.directories[Math.floor(pageIndex / 32)]?.[pageIndex % 32];
    const offset = index % diagramLayoutPageSize;
    const value = page?.[offset];
    if (page)
      page[offset] = undefined;
    return value;
  }
  resetCleared() {
    this.count = 0;
  }
  releaseOnePage() {
    if (this.count > 0) {
      this.pop();
      return false;
    }
    if (this.pageHighWater === 0)
      return true;
    const pageIndex = --this.pageHighWater;
    const directoryIndex = Math.floor(pageIndex / 32);
    const directory = this.directories[directoryIndex];
    if (directory) {
      directory[pageIndex % 32] = undefined;
      if (pageIndex % 32 === 0)
        this.directories[directoryIndex] = undefined;
    }
    return this.pageHighWater === 0;
  }
  releasePageStep() {
    const retained = this.count;
    const limit = Math.max(0, retained - diagramLayoutPageSize);
    while (this.count > limit)
      this.pop();
    if (retained > 0)
      return false;
    return this.releaseOnePage();
  }
}
function finiteLayoutValue(value, fallback) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}
function optionalFiniteLayoutValue(value) {
  return value === undefined || typeof value === "number" && Number.isFinite(value);
}
function resolveLayoutOptions(options) {
  return {
    direction: options.direction ?? "TB",
    nodeHeight: Math.max(1, finiteLayoutValue(options.nodeHeight, DIAGRAM_UNIT)),
    nodeSep: Math.max(0, finiteLayoutValue(options.nodeSep, DIAGRAM_UNIT * 1.04)),
    nodeWidth: Math.max(1, finiteLayoutValue(options.nodeWidth, DIAGRAM_UNIT)),
    rankSep: Math.max(0, finiteLayoutValue(options.rankSep, DIAGRAM_UNIT * 1.67))
  };
}
function nodeLayoutDimension(node, axis, fallback) {
  const measured = node.measured?.[axis];
  const direct = node[axis];
  const style = typeof node.style?.[axis] === "number" ? node.style[axis] : undefined;
  return Math.max(1, finiteLayoutValue(measured, finiteLayoutValue(direct, finiteLayoutValue(style, fallback))));
}
function createLayoutMerge(source) {
  return { left: 0, leftCursor: 0, middle: Math.min(1, source.length), right: Math.min(2, source.length), rightCursor: Math.min(1, source.length), source, target: new DiagramPagedStore(source.capacity), width: 1 };
}
function stepLayoutMerge(merge, compare2) {
  if (merge.source.length < 2 || merge.width >= merge.source.length)
    return true;
  if (merge.left >= merge.source.length) {
    const cleared = merge.source;
    merge.source = merge.target;
    cleared.resetCleared();
    merge.target = cleared;
    merge.width *= 2;
    merge.left = 0;
    merge.leftCursor = 0;
    merge.middle = Math.min(merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.width * 2, merge.source.length);
    return merge.width >= merge.source.length;
  }
  if (merge.leftCursor >= merge.middle && merge.rightCursor >= merge.right) {
    merge.left += merge.width * 2;
    merge.leftCursor = merge.left;
    merge.middle = Math.min(merge.left + merge.width, merge.source.length);
    merge.rightCursor = merge.middle;
    merge.right = Math.min(merge.left + merge.width * 2, merge.source.length);
    return false;
  }
  if (merge.rightCursor >= merge.right)
    merge.target.push(merge.source.take(merge.leftCursor++));
  else if (merge.leftCursor >= merge.middle)
    merge.target.push(merge.source.take(merge.rightCursor++));
  else
    merge.target.push(merge.source.take(compare2(merge.source.get(merge.leftCursor), merge.source.get(merge.rightCursor)) <= 0 ? merge.leftCursor++ : merge.rightCursor++));
  return false;
}
function compareLayoutText(left, right) {
  return left < right ? -1 : left > right ? 1 : 0;
}
function projectLayoutNode(source, x, y) {
  return {
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    connectable: source.connectable,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    dragHandle: source.dragHandle,
    draggable: source.draggable,
    dragging: source.dragging,
    expandParent: source.expandParent,
    extent: source.extent,
    focusable: source.focusable,
    handles: source.handles,
    height: source.height,
    hidden: source.hidden,
    id: source.id,
    initialHeight: source.initialHeight,
    initialWidth: source.initialWidth,
    measured: source.measured,
    origin: source.origin,
    parentId: source.parentId,
    position: { x, y },
    resizing: source.resizing,
    selectable: source.selectable,
    selected: source.selected,
    sourcePosition: source.sourcePosition,
    style: source.style,
    targetPosition: source.targetPosition,
    type: source.type,
    width: source.width,
    zIndex: source.zIndex
  };
}
function projectLayoutEdge(source) {
  return {
    animated: source.animated,
    ariaLabel: source.ariaLabel,
    ariaRole: source.ariaRole,
    className: source.className,
    data: source.data,
    deletable: source.deletable,
    domAttributes: source.domAttributes,
    focusable: source.focusable,
    hidden: source.hidden,
    id: source.id,
    interactionWidth: source.interactionWidth,
    label: source.label,
    labelBgBorderRadius: source.labelBgBorderRadius,
    labelBgPadding: source.labelBgPadding,
    labelBgStyle: source.labelBgStyle,
    labelShowBg: source.labelShowBg,
    labelStyle: source.labelStyle,
    markerEnd: source.markerEnd,
    markerStart: source.markerStart,
    reconnectable: source.reconnectable,
    selectable: source.selectable,
    selected: source.selected,
    source: source.source,
    sourceHandle: source.sourceHandle,
    style: source.style,
    target: source.target,
    targetHandle: source.targetHandle,
    type: source.type,
    zIndex: source.zIndex
  };
}
function pagedLayoutArray(store, length) {
  const target = [];
  const numericIndex = (property) => {
    if (typeof property !== "string" || !/^(0|[1-9]\d*)$/.test(property))
      return;
    const index = Number(property);
    return Number.isSafeInteger(index) && index < length ? index : undefined;
  };
  return new Proxy(target, {
    get(array, property, receiver) {
      if (property === "length")
        return length;
      const index = numericIndex(property);
      return index === undefined ? Reflect.get(array, property, receiver) : store.get(index);
    },
    getOwnPropertyDescriptor(array, property) {
      const index = numericIndex(property);
      return index === undefined ? Reflect.getOwnPropertyDescriptor(array, property) : { configurable: true, enumerable: true, value: store.get(index), writable: false };
    },
    has(array, property) {
      return numericIndex(property) !== undefined || Reflect.has(array, property);
    }
  });
}

class DiagramLayoutPublication {
  sourceNodes;
  sourceEdges;
  descriptor;
  capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  positions = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  closeStage = "positions";
  expectedPosition = 0;
  expectedSequence = 1;
  outputComplete = false;
  faulted = false;
  terminalRetained = false;
  constructor(sourceNodes, sourceEdges, descriptor) {
    this.sourceNodes = sourceNodes;
    this.sourceEdges = sourceEdges;
    this.descriptor = descriptor;
  }
  readInputPage(cursor, maxItems) {
    try {
      if (this.faulted || !Number.isSafeInteger(cursor) || cursor < 0 || cursor > this.sourceNodes.length + this.sourceEdges.length)
        return this.faultPage();
      const limit = Math.max(1, Math.min(DIAGRAM_LAYOUT_INGRESS_ITEMS, Math.floor(finiteLayoutValue(maxItems, 1))));
      if (cursor < this.sourceNodes.length)
        return this.readNodePage(cursor, limit);
      return this.readEdgePage(cursor - this.sourceNodes.length, limit);
    } catch {
      return this.faultPage();
    }
  }
  acceptOutputPage(page) {
    try {
      if (this.faulted || this.outputComplete || !Number.isSafeInteger(page.itemCount) || !Number.isSafeInteger(page.byteLength) || page.itemCount < 0 || page.itemCount > DIAGRAM_LAYOUT_OUTPUT_ITEMS || page.byteLength < 0 || page.byteLength > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.rejectOutput();
      const payload = page.payload;
      if (!payload || payload.kind !== "positions" || payload.generation !== this.descriptor.generation || payload.sequence !== this.expectedSequence || !Array.isArray(payload.values) || payload.values.length !== page.itemCount || page.byteLength !== page.itemCount * 32 || page.complete !== payload.complete || page.itemCount === 0 && this.sourceNodes.length > this.expectedPosition)
        return this.rejectOutput();
      for (let index = 0;index < payload.values.length; index++) {
        const position = payload.values[index];
        if (!position || position.index !== this.expectedPosition || position.index >= this.sourceNodes.length || !Number.isFinite(position.x) || !Number.isFinite(position.y))
          return this.rejectOutput();
        this.positions.set(position.index, { index: position.index, x: position.x, y: position.y });
        const node = this.capturedNodes?.get(position.index);
        if (node)
          node.position = { x: position.x, y: position.y };
        this.expectedPosition += 1;
      }
      const exactComplete = this.expectedPosition === this.sourceNodes.length;
      if (payload.complete !== exactComplete)
        return this.rejectOutput();
      this.expectedSequence += 1;
      this.outputComplete = payload.complete;
      return true;
    } catch {
      return this.rejectOutput();
    }
  }
  acceptTerminal(terminal) {
    this.terminalRetained = true;
    if (terminal.generation !== this.descriptor.generation || terminal.status !== "complete" || this.faulted || !this.outputComplete || this.expectedPosition !== this.sourceNodes.length || this.capturedNodes?.length !== this.sourceNodes.length || this.capturedEdges?.length !== this.sourceEdges.length) {
      this.faulted = true;
      return;
    }
    const nodes = this.capturedNodes;
    const edges = this.capturedEdges;
    this.capturedNodes = undefined;
    this.capturedEdges = undefined;
    return new DiagramLayoutPublishedResult(nodes, edges, this.sourceNodes.length, this.sourceEdges.length);
  }
  closeStep() {
    if (this.closeStage === "positions") {
      if (!this.positions.releasePageStep())
        return false;
      this.closeStage = "edges";
      return false;
    }
    if (this.closeStage === "edges") {
      if (this.capturedEdges && !this.capturedEdges.releasePageStep())
        return false;
      this.capturedEdges = undefined;
      this.closeStage = "nodes";
      return false;
    }
    if (this.closeStage === "nodes") {
      if (this.capturedNodes && !this.capturedNodes.releasePageStep())
        return false;
      this.capturedNodes = undefined;
      this.closeStage = "terminal";
      return false;
    }
    if (this.closeStage === "terminal") {
      this.terminalRetained = false;
      this.closeStage = "complete";
    }
    return true;
  }
  terminalIsEmpty() {
    return !this.terminalRetained;
  }
  readNodePage(offset, limit) {
    const values = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceNodes.length) {
      const index = offset + values.length;
      const source = this.sourceNodes[index];
      if (!source || typeof source.id !== "string")
        return this.faultPage();
      const value = {
        height: source.height,
        id: source.id,
        index,
        measuredHeight: source.measured?.height,
        measuredWidth: source.measured?.width,
        styleHeight: typeof source.style?.height === "number" ? source.style.height : undefined,
        styleWidth: typeof source.style?.width === "number" ? source.style.width : undefined,
        width: source.width
      };
      let valueBytes;
      try {
        valueBytes = diagramLayoutNodeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.faultPage();
      this.capturedNodes.set(index, projectLayoutNode(source, source.position.x, source.position.y));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceNodes.length && this.sourceEdges.length === 0;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "nodes", offset, values } };
  }
  readEdgePage(offset, limit) {
    const values = [];
    let bytes = 0;
    while (values.length < limit && offset + values.length < this.sourceEdges.length) {
      const index = offset + values.length;
      const source = this.sourceEdges[index];
      if (!source || typeof source.id !== "string" || typeof source.source !== "string" || typeof source.target !== "string")
        return this.faultPage();
      const value = { id: source.id, index, source: source.source, target: source.target };
      let valueBytes;
      try {
        valueBytes = diagramLayoutEdgeWireBytes(value);
      } catch {
        return this.faultPage();
      }
      if (values.length > 0 && bytes + valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        break;
      if (valueBytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.faultPage();
      this.capturedEdges.set(index, projectLayoutEdge(source));
      values.push(value);
      bytes += valueBytes;
    }
    const next = offset + values.length;
    const complete = next === this.sourceEdges.length;
    return { byteLength: bytes, complete, itemCount: values.length, payload: { bytes, complete, generation: this.descriptor.generation, kind: "edges", offset, values } };
  }
  faultPage() {
    this.faulted = true;
    return { byteLength: 0, complete: true, itemCount: 0, payload: { generation: this.descriptor.generation, kind: "seal" } };
  }
  rejectOutput() {
    this.faulted = true;
    return false;
  }
}

class DiagramLayoutPublishedResult {
  nodeStore;
  edgeStore;
  nodes;
  edges;
  constructor(nodeStore, edgeStore, nodeCount, edgeCount) {
    this.nodeStore = nodeStore;
    this.edgeStore = edgeStore;
    this.nodes = pagedLayoutArray(nodeStore, nodeCount);
    this.edges = pagedLayoutArray(edgeStore, edgeCount);
  }
  closeStep() {
    if (!this.nodeStore.releasePageStep())
      return false;
    return this.edgeStore.releasePageStep();
  }
}
class DiagramLayoutOwnedResult {
  nodes;
  edges;
  nodeCount;
  edgeCount;
  constructor(nodes, edges, nodeCount, edgeCount) {
    this.nodes = nodes;
    this.edges = edges;
    this.nodeCount = nodeCount;
    this.edgeCount = edgeCount;
  }
  takeNode(index) {
    return this.nodes.take(index);
  }
  takeEdge(index) {
    return this.edges.take(index);
  }
  closeStep() {
    if (this.nodes.length > 0)
      this.nodes.pop();
    else if (this.edges.length > 0)
      this.edges.pop();
    return this.nodes.length === 0 && this.edges.length === 0;
  }
}

class DiagramLayoutJob {
  generation;
  sourceNodes;
  sourceEdges;
  options;
  nodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  queue = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankCross = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankDepth = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankOffset = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  rankSpan = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edgeNext = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  previewPositions = new DiagramPagedStore(diagramLayoutLimits.previewNodes);
  layoutX = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  layoutY = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  admittedEdgeCount = 0;
  queueLength = 0;
  previewLength = 0;
  previewWriteCursor = 0;
  resultTaken = false;
  pendingEdge;
  nodeMerge;
  edgeMerge;
  crossingMerge;
  mergeSpares = new Array(9);
  mergeSpareLength = 0;
  stage = "admit-nodes";
  status = "running";
  cursor = 0;
  secondaryCursor = 0;
  queueCursor = 0;
  activeRankNode = -1;
  unresolvedCursor = 0;
  maxRank = 0;
  totalDepth = 0;
  previewSequence = 0;
  faultReason;
  closeStage = "previews";
  closeCursor = 0;
  closeArray = 0;
  closePrepared = false;
  sourceNodeCount;
  sourceEdgeCount;
  constructor(nodes, edges, options = {}, generation2 = 1) {
    this.generation = generation2;
    this.sourceNodes = asDiagramLayoutSource(nodes);
    this.sourceEdges = asDiagramLayoutSource(edges);
    this.sourceNodeCount = nodes.length;
    this.sourceEdgeCount = edges.length;
    this.options = resolveLayoutOptions(options);
    if (nodes.length > diagramLayoutLimits.maxNodes || edges.length > diagramLayoutLimits.maxEdges)
      this.fail("Diagram layout capacity exceeded");
  }
  static fromBatchTest(nodes, edges, options = {}, generation2 = 1) {
    return new DiagramLayoutJob(nodes, edges, options, generation2);
  }
  static fromOwnedPagedSources(nodes, edges, options = {}, generation2 = 1) {
    return new DiagramLayoutJob(nodes, edges, options, generation2);
  }
  takeResult() {
    if (this.status !== "complete" || this.resultTaken)
      return;
    this.resultTaken = true;
    const result3 = new DiagramLayoutOwnedResult(this.capturedNodes, this.capturedEdges, this.sourceNodeCount, this.sourceEdgeCount);
    this.capturedNodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
    this.capturedEdges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
    return result3;
  }
  get reason() {
    return this.faultReason;
  }
  cancel(generation2 = this.generation) {
    if (generation2 === this.generation && this.status === "running")
      this.status = "cancelled";
  }
  takePreview() {
    if (this.previewLength === 0)
      return;
    const positions = new Array(this.previewLength);
    for (let index = 0;index < this.previewLength; index++) {
      const sourceIndex = (this.previewWriteCursor - this.previewLength + index + diagramLayoutLimits.previewNodes) % diagramLayoutLimits.previewNodes;
      positions[index] = this.previewPositions.take(sourceIndex);
    }
    this.previewPositions.resetCleared();
    this.previewLength = 0;
    this.previewWriteCursor = 0;
    return { generation: this.generation, positions, sequence: this.previewSequence };
  }
  step(work) {
    const fuel = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    if (work.generation !== this.generation)
      this.cancel();
    if (this.status !== "running" || fuel === 0)
      return { consumed: 0, stage: this.stage, status: this.status };
    let remaining = fuel;
    while (remaining > 0 && this.now() < work.deadline && this.status === "running") {
      remaining -= 1;
      this.stepUnit();
    }
    return { consumed: fuel - remaining, stage: this.stage, status: this.status };
  }
  close(work) {
    this.prepareClose();
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && this.now() < work.deadline && this.closeStage !== "complete") {
      remaining -= 1;
      this.closeUnit();
    }
    return this.closeStage === "complete";
  }
  now() {
    return typeof performance === "undefined" ? Date.now() : performance.now();
  }
  fail(reason) {
    this.faultReason = reason;
    this.status = "fault";
  }
  prepareClose() {
    if (this.closePrepared)
      return;
    this.closePrepared = true;
    for (const merge of [this.nodeMerge, this.edgeMerge, this.crossingMerge]) {
      if (!merge)
        continue;
      this.mergeSpares[this.mergeSpareLength++] = merge.source;
      this.mergeSpares[this.mergeSpareLength++] = merge.target;
    }
  }
  advance(stage) {
    this.stage = stage;
    this.cursor = 0;
    this.secondaryCursor = 0;
  }
  stepUnit() {
    if (this.stage === "admit-nodes")
      this.admitNode();
    else if (this.stage === "sort-nodes")
      this.sortNode();
    else if (this.stage === "index-nodes")
      this.indexNode();
    else if (this.stage === "admit-edges")
      this.admitEdge();
    else if (this.stage === "sort-edges")
      this.sortEdge();
    else if (this.stage === "build-graph")
      this.buildGraph();
    else if (this.stage === "assign-ranks")
      this.assignRank();
    else if (this.stage === "crossing")
      this.accumulateCrossing();
    else if (this.stage === "sort-crossing")
      this.sortCrossing();
    else if (this.stage === "measure-ranks")
      this.measureRank();
    else if (this.stage === "position-ranks")
      this.positionRank();
    else if (this.stage === "coordinates")
      this.coordinateNode();
    else if (this.stage === "project")
      this.projectNode();
    else if (this.stage === "project-edges")
      this.projectEdge();
  }
  admitNode() {
    const source = this.sourceNodes;
    if (this.cursor >= source.length) {
      this.nodeMerge = createLayoutMerge(this.nodes);
      this.sourceNodes = undefined;
      this.advance("sort-nodes");
      return;
    }
    const node = source.get(this.cursor);
    if (!diagramLayoutIdentityAdmitted(node.id)) {
      this.fail("Diagram layout node id is invalid");
      return;
    }
    const sourceIndex = this.cursor++;
    const captured = projectLayoutNode(node, node.position.x, node.position.y);
    this.capturedNodes.set(sourceIndex, captured);
    this.nodes.push({
      barycenterCount: 0,
      barycenterSum: 0,
      cross: 0,
      depth: 0,
      height: nodeLayoutDimension(captured, "height", this.options.nodeHeight),
      id: captured.id,
      indegree: 0,
      order: 0,
      outgoingHead: -1,
      outgoingTail: -1,
      processed: false,
      rank: 0,
      sourceIndex,
      width: nodeLayoutDimension(captured, "width", this.options.nodeWidth),
      x: 0,
      y: 0
    });
  }
  sortNode() {
    if (!this.nodeMerge || stepLayoutMerge(this.nodeMerge, (left, right) => compareLayoutText(left.id, right.id))) {
      if (this.nodeMerge) {
        this.nodes = this.nodeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.nodeMerge.target;
      }
      this.nodeMerge = undefined;
      this.advance("index-nodes");
    }
  }
  indexNode() {
    if (this.cursor >= this.nodes.length) {
      this.advance("admit-edges");
      return;
    }
    const node = this.nodes.get(this.cursor);
    if (this.cursor > 0 && this.nodes.get(this.cursor - 1).id === node.id) {
      this.fail("Duplicate Diagram layout node id");
      return;
    }
    node.order = this.cursor;
    this.cursor += 1;
  }
  admitEdge() {
    const source = this.sourceEdges;
    if (this.cursor >= source.length && !this.pendingEdge) {
      this.edgeMerge = createLayoutMerge(this.edges);
      this.sourceEdges = undefined;
      this.advance("sort-edges");
      return;
    }
    if (!this.pendingEdge) {
      const edge = source.get(this.cursor);
      const inputIndex = this.cursor++;
      if (!edge)
        return;
      if (!diagramLayoutIdentityAdmitted(edge.id, true) || !diagramLayoutIdentityAdmitted(edge.source) || !diagramLayoutIdentityAdmitted(edge.target)) {
        this.fail("Diagram layout edge identity is invalid");
        return;
      }
      const captured = projectLayoutEdge(edge);
      this.capturedEdges.set(inputIndex, captured);
      this.pendingEdge = { captured, inputIndex, sourceLookup: { done: false, high: this.nodes.length - 1, low: 0, value: captured.source } };
    }
    const pending2 = this.pendingEdge;
    if (!pending2.sourceLookup.done) {
      this.stepLayoutLookup(pending2.sourceLookup);
      return;
    }
    pending2.targetLookup ??= { done: false, high: this.nodes.length - 1, low: 0, value: pending2.captured.target };
    if (!pending2.targetLookup.done) {
      this.stepLayoutLookup(pending2.targetLookup);
      return;
    }
    const sourceIndex = pending2.sourceLookup.result;
    const targetIndex = pending2.targetLookup.result;
    if (sourceIndex !== undefined && targetIndex !== undefined) {
      const id2 = typeof pending2.captured.id === "string" ? pending2.captured.id : `${pending2.captured.source}:${pending2.captured.target}:${pending2.inputIndex}`;
      this.edges.push({ id: id2, source: sourceIndex, sourceId: pending2.captured.source, sourceIndex: pending2.inputIndex, target: targetIndex, targetId: pending2.captured.target });
      this.admittedEdgeCount += 1;
    }
    this.pendingEdge = undefined;
  }
  stepLayoutLookup(lookup) {
    if (lookup.low > lookup.high) {
      lookup.done = true;
      return;
    }
    const middle = Math.floor((lookup.low + lookup.high) / 2);
    const comparison = compareLayoutText(lookup.value, this.nodes.get(middle).id);
    if (comparison === 0) {
      lookup.done = true;
      lookup.result = middle;
    } else if (comparison < 0)
      lookup.high = middle - 1;
    else
      lookup.low = middle + 1;
  }
  sortEdge() {
    if (!this.edgeMerge || stepLayoutMerge(this.edgeMerge, (left, right) => compareLayoutText(left.sourceId, right.sourceId) || compareLayoutText(left.targetId, right.targetId) || compareLayoutText(left.id, right.id))) {
      if (this.edgeMerge) {
        this.edges = this.edgeMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.edgeMerge.target;
      }
      this.edgeMerge = undefined;
      this.advance("build-graph");
    }
  }
  buildGraph() {
    if (this.cursor >= this.edges.length) {
      if (this.secondaryCursor < this.nodes.length) {
        const node = this.nodes.get(this.secondaryCursor);
        if (node && node.indegree === 0)
          this.queue.set(this.queueLength++, this.secondaryCursor);
        this.secondaryCursor += 1;
        return;
      }
      this.queueCursor = 0;
      this.activeRankNode = -1;
      this.unresolvedCursor = 0;
      this.advance("assign-ranks");
      return;
    }
    const edgeIndex = this.cursor++;
    const edge = this.edges.get(edgeIndex);
    if (edge.source === edge.target)
      return;
    const source = this.nodes.get(edge.source);
    const target = this.nodes.get(edge.target);
    this.edgeNext.set(edgeIndex, -1);
    if (source.outgoingTail < 0)
      source.outgoingHead = edgeIndex;
    else
      this.edgeNext.set(source.outgoingTail, edgeIndex);
    source.outgoingTail = edgeIndex;
    target.indegree += 1;
  }
  assignRank() {
    if (this.activeRankNode >= 0) {
      const active2 = this.nodes.get(this.activeRankNode);
      if (this.secondaryCursor >= 0) {
        const edgeIndex = this.secondaryCursor;
        this.secondaryCursor = this.edgeNext.get(edgeIndex) ?? -1;
        const edge = this.edges.get(edgeIndex);
        const target = this.nodes.get(edge.target);
        if (!target.processed) {
          target.rank = Math.max(target.rank, active2.rank + 1);
          target.indegree = Math.max(0, target.indegree - 1);
          if (target.indegree === 0)
            this.queue.set(this.queueLength++, edge.target);
        }
        return;
      }
      active2.processed = true;
      this.activeRankNode = -1;
      this.secondaryCursor = -1;
      return;
    }
    if (this.queueCursor < this.queueLength) {
      const candidate = this.queue.get(this.queueCursor++);
      if (this.nodes.get(candidate).processed)
        return;
      this.activeRankNode = candidate;
      const active2 = this.nodes.get(candidate);
      this.secondaryCursor = active2.outgoingHead;
      this.maxRank = Math.max(this.maxRank, active2.rank);
      return;
    }
    if (this.unresolvedCursor < this.nodes.length) {
      const candidate = this.unresolvedCursor++;
      if (this.nodes.get(candidate).processed)
        return;
      this.nodes.get(candidate).indegree = 0;
      this.queue.set(this.queueLength++, candidate);
      return;
    }
    this.advance("crossing");
  }
  accumulateCrossing() {
    if (this.cursor >= this.edges.length) {
      this.crossingMerge = createLayoutMerge(this.nodes);
      this.advance("sort-crossing");
      return;
    }
    const edge = this.edges.get(this.cursor++);
    const source = this.nodes.get(edge.source);
    const target = this.nodes.get(edge.target);
    if (source.rank < target.rank) {
      target.barycenterCount += 1;
      target.barycenterSum += source.order;
    }
  }
  sortCrossing() {
    const compare2 = (left, right) => {
      if (left.rank !== right.rank)
        return left.rank - right.rank;
      const leftBarycenter = left.barycenterCount === 0 ? left.order : left.barycenterSum / left.barycenterCount;
      const rightBarycenter = right.barycenterCount === 0 ? right.order : right.barycenterSum / right.barycenterCount;
      return leftBarycenter - rightBarycenter || compareLayoutText(left.id, right.id);
    };
    if (!this.crossingMerge || stepLayoutMerge(this.crossingMerge, compare2)) {
      if (this.crossingMerge) {
        this.nodes = this.crossingMerge.source;
        this.mergeSpares[this.mergeSpareLength++] = this.crossingMerge.target;
      }
      this.crossingMerge = undefined;
      this.advance("measure-ranks");
    }
  }
  measureRank() {
    if (this.cursor >= this.nodes.length) {
      this.advance("position-ranks");
      return;
    }
    const node = this.nodes.get(this.cursor++);
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const rank = node.rank;
    const span = this.rankSpan.get(rank);
    this.rankSpan.set(rank, (span ?? 0) + (span === undefined ? 0 : this.options.nodeSep) + crossSize);
    this.rankDepth.set(rank, Math.max(this.rankDepth.get(rank) ?? 0, depthSize));
  }
  positionRank() {
    if (this.cursor > this.maxRank) {
      this.totalDepth = this.maxRank < 0 ? 0 : (this.rankOffset.get(this.maxRank) ?? 0) + (this.rankDepth.get(this.maxRank) ?? 0);
      this.advance("coordinates");
      return;
    }
    const rank = this.cursor++;
    this.rankOffset.set(rank, rank === 0 ? 0 : (this.rankOffset.get(rank - 1) ?? 0) + (this.rankDepth.get(rank - 1) ?? 0) + this.options.rankSep);
    this.rankCross.set(rank, -(this.rankSpan.get(rank) ?? 0) / 2);
  }
  coordinateNode() {
    if (this.cursor >= this.nodes.length) {
      this.advance("project");
      return;
    }
    const node = this.nodes.get(this.cursor++);
    const horizontal = this.options.direction === "LR" || this.options.direction === "RL";
    const crossSize = horizontal ? node.height : node.width;
    const depthSize = horizontal ? node.width : node.height;
    const cross = (this.rankCross.get(node.rank) ?? 0) + crossSize / 2;
    const forwardDepth = (this.rankOffset.get(node.rank) ?? 0) + depthSize / 2;
    const depth = this.options.direction === "BT" || this.options.direction === "RL" ? this.totalDepth - forwardDepth : forwardDepth;
    this.rankCross.set(node.rank, cross + crossSize / 2 + this.options.nodeSep);
    node.cross = cross;
    node.depth = depth;
    node.x = horizontal ? depth - node.width / 2 : cross - node.width / 2;
    node.y = horizontal ? cross - node.height / 2 : depth - node.height / 2;
    this.layoutX.set(node.sourceIndex, node.x);
    this.layoutY.set(node.sourceIndex, node.y);
  }
  projectNode() {
    if (this.cursor >= this.sourceNodeCount) {
      this.advance("project-edges");
      return;
    }
    const sourceNode = this.capturedNodes.get(this.cursor++);
    if (!sourceNode)
      return;
    const sourceIndex = this.cursor - 1;
    const x = this.layoutX.get(sourceIndex);
    const y = this.layoutY.get(sourceIndex);
    if (x === undefined || y === undefined)
      return;
    sourceNode.position = { x, y };
    this.previewPositions.set(this.previewWriteCursor, { index: sourceIndex, x, y });
    this.previewWriteCursor = (this.previewWriteCursor + 1) % diagramLayoutLimits.previewNodes;
    this.previewLength = Math.min(diagramLayoutLimits.previewNodes, this.previewLength + 1);
    this.previewSequence += 1;
  }
  projectEdge() {
    if (this.cursor >= this.sourceEdgeCount) {
      this.stage = "complete";
      this.status = "complete";
      return;
    }
    this.capturedEdges.get(this.cursor++);
  }
  closeUnit() {
    if (this.closeStage === "previews") {
      if (this.previewLength > 0)
        this.previewPositions.take(--this.previewLength);
      else if (!this.resultTaken && this.capturedNodes.length > 0)
        this.capturedNodes.pop();
      else if (!this.resultTaken && this.capturedEdges.length > 0)
        this.capturedEdges.pop();
      else
        this.closeStage = "edges";
      return;
    }
    if (this.closeStage === "edges") {
      if (this.edges.length > 0)
        this.edges.pop();
      else
        this.closeStage = "nodes";
      return;
    }
    if (this.closeStage === "nodes") {
      const node = this.nodes.get(this.nodes.length - 1);
      if (!node) {
        this.closeStage = "spares";
        return;
      }
      this.nodes.pop();
      return;
    }
    if (this.closeStage === "spares") {
      if (this.closeCursor >= this.mergeSpareLength) {
        this.closeCursor = 0;
        this.closeStage = "indices";
        return;
      }
      if (this.mergeSpares[this.closeCursor].releaseOnePage())
        this.closeCursor += 1;
      return;
    }
    if (this.closeStage === "indices") {
      const store = this.closeIndexStore();
      if (!store) {
        this.closeCursor = 0;
        this.closeArray = 0;
        this.closeStage = "captures";
        return;
      }
      if (store.length > 0)
        store.pop();
      else
        this.closeArray += 1;
      return;
    }
    if (this.closeStage === "captures") {
      this.closeStage = "scalars";
      return;
    }
    if (this.closeStage === "scalars") {
      this.sourceNodes = undefined;
      this.sourceEdges = undefined;
      this.nodeMerge = undefined;
      this.edgeMerge = undefined;
      this.crossingMerge = undefined;
      this.closeStage = "complete";
    }
  }
  closeIndexStore() {
    if (this.closeArray === 0)
      return this.queue;
    if (this.closeArray === 1)
      return this.rankCross;
    if (this.closeArray === 2)
      return this.rankDepth;
    if (this.closeArray === 3)
      return this.rankOffset;
    if (this.closeArray === 4)
      return this.rankSpan;
    if (this.closeArray === 5)
      return this.edgeNext;
    if (this.closeArray === 6)
      return this.layoutX;
    if (this.closeArray === 7)
      return this.layoutY;
    return;
  }
}

class DiagramLayoutWireJob {
  descriptor;
  nodes = new DiagramPagedStore(diagramLayoutLimits.maxNodes);
  edges = new DiagramPagedStore(diagramLayoutLimits.maxEdges);
  nodeReceived = 0;
  edgeReceived = 0;
  job;
  owned;
  resultCursor = 0;
  sequence = 0;
  emptyResultPublished = false;
  cancelled = false;
  ingesting = false;
  faultReason;
  constructor(descriptor) {
    this.descriptor = descriptor;
    const credits = diagramLayoutCredits(descriptor.nodeCount, descriptor.edgeCount);
    if (descriptor.kind !== DIAGRAM_LAYOUT_CODEC_KIND || !Number.isSafeInteger(descriptor.generation) || descriptor.generation < 0 || !Number.isSafeInteger(descriptor.nodeCount) || !Number.isSafeInteger(descriptor.edgeCount) || descriptor.nodeCount < 0 || descriptor.edgeCount < 0 || descriptor.nodeCount > diagramLayoutLimits.maxNodes || descriptor.edgeCount > diagramLayoutLimits.maxEdges || !credits.admitted)
      this.faultReason = "Diagram layout descriptor capacity is invalid";
  }
  get status() {
    if (this.faultReason)
      return "fault";
    if (this.cancelled)
      return "cancelled";
    if (!this.job)
      return "running";
    return this.job.step({ deadline: 0, fuel: 0, generation: this.descriptor.generation }).status;
  }
  get reason() {
    return this.faultReason ?? this.job?.reason;
  }
  ingest(page) {
    if (this.cancelled || this.faultReason || this.ingesting)
      return false;
    this.ingesting = true;
    try {
      if (!page || typeof page !== "object" || Array.isArray(page))
        return this.failIngress("Diagram layout ingress page is invalid");
      const candidate = page;
      const generation2 = candidate.generation;
      const kind = candidate.kind;
      if (!Number.isSafeInteger(generation2) || generation2 !== this.descriptor.generation)
        return this.failIngress("Diagram layout ingress generation is invalid");
      if (kind === "seal")
        return this.sealIngress();
      if (kind !== "nodes" && kind !== "edges")
        return this.failIngress("Diagram layout ingress kind is invalid");
      if (this.job)
        return false;
      const offset = candidate.offset;
      const bytes = candidate.bytes;
      const complete = candidate.complete;
      const values = candidate.values;
      if (!Number.isSafeInteger(offset) || offset < 0 || !Number.isSafeInteger(bytes) || bytes < 0 || bytes > DIAGRAM_LAYOUT_INGRESS_BYTES || complete !== undefined && typeof complete !== "boolean" || !Array.isArray(values) || values.length > DIAGRAM_LAYOUT_INGRESS_ITEMS)
        return this.failIngress("Diagram layout ingress page exceeds its item or byte cap");
      const capturedNodes = kind === "nodes" ? this.captureNodes(values, offset, bytes) : undefined;
      const capturedEdges = kind === "edges" ? this.captureEdges(values, offset, bytes) : undefined;
      if (kind === "nodes" && !capturedNodes || kind === "edges" && !capturedEdges)
        return false;
      const nextNodeReceived = this.nodeReceived + (capturedNodes?.length ?? 0);
      const nextEdgeReceived = this.edgeReceived + (capturedEdges?.length ?? 0);
      const ingressComplete = nextNodeReceived === this.descriptor.nodeCount && nextEdgeReceived === this.descriptor.edgeCount;
      if (complete && !ingressComplete)
        return this.failIngress("Diagram layout ingress completed before its declared counts");
      if (this.cancelled || this.faultReason)
        return false;
      for (let index = 0;index < (capturedNodes?.length ?? 0); index++)
        this.nodes.set(this.nodeReceived + index, capturedNodes[index]);
      for (let index = 0;index < (capturedEdges?.length ?? 0); index++)
        this.edges.set(this.edgeReceived + index, capturedEdges[index]);
      this.nodeReceived = nextNodeReceived;
      this.edgeReceived = nextEdgeReceived;
      if (ingressComplete)
        this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
      return true;
    } catch {
      return this.failIngress("Diagram layout ingress value is invalid");
    } finally {
      this.ingesting = false;
    }
  }
  cancel(generation2 = this.descriptor.generation) {
    if (generation2 !== this.descriptor.generation)
      return;
    this.cancelled = true;
    this.job?.cancel(generation2);
  }
  step(work) {
    if (this.faultReason)
      return { consumed: 0, stage: "complete", status: "fault" };
    if (this.cancelled && !this.job)
      return { consumed: 0, stage: "complete", status: "cancelled" };
    if (!this.job)
      return { consumed: 0, stage: "admit-nodes", status: "running" };
    const result3 = this.job.step(work);
    if (result3.status === "fault")
      this.faultReason = this.job.reason;
    if (result3.status === "complete" && this.resultCursor < this.descriptor.nodeCount)
      return { ...result3, status: "running" };
    return result3;
  }
  takePreviewPage() {
    return;
  }
  takeResultPage() {
    if (!this.job || this.status !== "complete")
      return;
    this.owned ??= this.job.takeResult();
    if (!this.owned)
      return;
    if (this.owned.nodeCount === 0) {
      if (this.emptyResultPublished)
        return;
      this.emptyResultPublished = true;
      this.sequence += 1;
      return { complete: true, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values: [] };
    }
    if (this.resultCursor >= this.owned.nodeCount)
      return;
    const count = Math.min(DIAGRAM_LAYOUT_OUTPUT_ITEMS, this.owned.nodeCount - this.resultCursor);
    const values = new Array(count);
    for (let index = 0;index < count; index++) {
      const sourceIndex = this.resultCursor + index;
      const node = this.owned.takeNode(sourceIndex);
      values[index] = { index: sourceIndex, x: node.position.x, y: node.position.y };
    }
    this.resultCursor += count;
    this.sequence += 1;
    return { complete: this.resultCursor === this.owned.nodeCount, generation: this.descriptor.generation, kind: "positions", sequence: this.sequence, values };
  }
  close(work) {
    let remaining = Math.max(0, Math.floor(finiteLayoutValue(work.fuel, 0)));
    while (remaining > 0 && (typeof performance === "undefined" ? Date.now() : performance.now()) < work.deadline) {
      remaining -= 1;
      if (this.job && !this.job.close({ deadline: work.deadline, fuel: 1 }))
        continue;
      if (this.owned && !this.owned.closeStep())
        continue;
      if (this.nodes.length > 0) {
        this.nodes.pop();
        continue;
      }
      if (this.edges.length > 0) {
        this.edges.pop();
        continue;
      }
      return true;
    }
    return false;
  }
  terminal() {
    const status = this.status;
    if (status === "running")
      return;
    if (status === "complete" && (this.resultCursor < this.descriptor.nodeCount || this.descriptor.nodeCount === 0 && !this.emptyResultPublished))
      return;
    if (status === "fault")
      return { generation: this.descriptor.generation, kind: "terminal", reason: this.reason ?? "Diagram layout fault", status };
    return { generation: this.descriptor.generation, kind: "terminal", status };
  }
  captureNodes(values, offset, declaredBytes) {
    if (offset !== this.nodeReceived || offset + values.length > this.descriptor.nodeCount)
      return this.failCapture("Diagram node ingress offset is invalid");
    let bytes = 0;
    const captured = new Array(values.length);
    for (let index = 0;index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source))
        return this.failCapture("Diagram node ingress value is invalid");
      const candidate = source;
      const value = {
        height: candidate.height,
        id: candidate.id,
        index: candidate.index,
        measuredHeight: candidate.measuredHeight,
        measuredWidth: candidate.measuredWidth,
        styleHeight: candidate.styleHeight,
        styleWidth: candidate.styleWidth,
        width: candidate.width
      };
      if (!Number.isSafeInteger(value.index) || value.index !== offset + index || typeof value.id !== "string" || value.id.length === 0 || !optionalFiniteLayoutValue(value.height) || !optionalFiniteLayoutValue(value.measuredHeight) || !optionalFiniteLayoutValue(value.measuredWidth) || !optionalFiniteLayoutValue(value.styleHeight) || !optionalFiniteLayoutValue(value.styleWidth) || !optionalFiniteLayoutValue(value.width))
        return this.failCapture("Diagram node ingress value is invalid");
      bytes += diagramLayoutNodeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.failCapture("Diagram node ingress exceeds its byte cap");
      if (!value.id)
        return this.failCapture("Diagram node id is empty");
      captured[index] = {
        data: {},
        height: value.height,
        id: value.id,
        measured: { height: value.measuredHeight, width: value.measuredWidth },
        position: { x: 0, y: 0 },
        style: { height: value.styleHeight, width: value.styleWidth },
        width: value.width
      };
    }
    if (bytes !== declaredBytes)
      return this.failCapture("Diagram node ingress byte accounting is invalid");
    return captured;
  }
  captureEdges(values, offset, declaredBytes) {
    if (offset !== this.edgeReceived || offset + values.length > this.descriptor.edgeCount)
      return this.failCapture("Diagram edge ingress offset is invalid");
    let bytes = 0;
    const captured = new Array(values.length);
    for (let index = 0;index < values.length; index++) {
      const source = values[index];
      if (!source || typeof source !== "object" || Array.isArray(source))
        return this.failCapture("Diagram edge ingress value is invalid");
      const candidate = source;
      const value = { id: candidate.id, index: candidate.index, source: candidate.source, target: candidate.target };
      if (!Number.isSafeInteger(value.index) || value.index !== offset + index || typeof value.id !== "string" || typeof value.source !== "string" || typeof value.target !== "string" || value.source.length === 0 || value.target.length === 0)
        return this.failCapture("Diagram edge ingress value is invalid");
      bytes += diagramLayoutEdgeWireBytes(value);
      if (bytes > DIAGRAM_LAYOUT_INGRESS_BYTES)
        return this.failCapture("Diagram edge ingress exceeds its byte cap");
      captured[index] = { id: value.id, source: value.source, target: value.target };
    }
    if (bytes !== declaredBytes)
      return this.failCapture("Diagram edge ingress byte accounting is invalid");
    return captured;
  }
  sealIngress() {
    if (this.job)
      return true;
    if (this.nodeReceived !== this.descriptor.nodeCount || this.edgeReceived !== this.descriptor.edgeCount)
      return this.failIngress("Diagram layout ingress was not complete");
    this.job = DiagramLayoutJob.fromOwnedPagedSources(this.nodes, this.edges, this.descriptor.options, this.descriptor.generation);
    return true;
  }
  failCapture(reason) {
    this.faultReason = reason;
    return;
  }
  failIngress(reason) {
    this.faultReason = reason;
    return false;
  }
}
function createDiagramLayoutWorkerJob(descriptor) {
  return new DiagramLayoutWireJob(descriptor);
}

/* ../../🔌️browser-interactive-job-port/🟦️.ts */
var INTERACTIVE_JOB_SLOT_CAPACITY = 16;
var INTERACTIVE_JOB_INPUT_ITEM_CAPACITY = 65536;
var INTERACTIVE_JOB_INPUT_BYTE_CAPACITY = 256 * 1024 * 1024;
var INTERACTIVE_JOB_PAGE_ITEM_CAPACITY = 128;
var INTERACTIVE_JOB_PAGE_BYTE_CAPACITY = 16 * 1024;
var INTERACTIVE_JOB_UI_BUDGET_MS = 2;
var INTERACTIVE_JOB_OBSERVER_CAPACITY = 32;
var INTERACTIVE_JOB_PORT_ITEM_CAPACITY = 262144;
var INTERACTIVE_JOB_PORT_BYTE_CAPACITY = 256 * 1024 * 1024;

class BrowserInteractiveJobPort {
  lifecycle;
  send;
  quarantineConsumer;
  schedule;
  status = "unavailable";
  slots = new Array(INTERACTIVE_JOB_SLOT_CAPACITY);
  closeCursor = 0;
  closeScheduled = false;
  reservedItems = 0;
  reservedBytes = 0;
  observers = new Array(INTERACTIVE_JOB_OBSERVER_CAPACITY);
  observerCursor = 0;
  observerNotifyScheduled = false;
  statusRevision = 0;
  statusSnapshot = { status: "unavailable", revision: 0 };
  now;
  constructor(lifecycle, send, now, quarantineConsumer, schedule = (callback) => setTimeout(callback, 0)) {
    this.lifecycle = lifecycle;
    this.send = send;
    this.quarantineConsumer = quarantineConsumer;
    this.schedule = schedule;
    this.now = now;
  }
  ready() {
    if (this.status === "unavailable") {
      this.status = "ready";
      this.publishStatus();
    }
  }
  getSnapshot() {
    return this.statusSnapshot;
  }
  observeConsumerTurn(site, durationMs) {
    if (durationMs < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${durationMs.toFixed(3)} ms`);
    return false;
  }
  subscribe(listener) {
    const slot = this.observers.findIndex((entry) => entry === undefined);
    if (slot < 0)
      throw new Error(`interactive job observer slots exceeded ${INTERACTIVE_JOB_OBSERVER_CAPACITY}`);
    this.observers[slot] = listener;
    return () => {
      this.observers[slot] = undefined;
    };
  }
  submit(descriptor, consumer) {
    if (this.status !== "ready" || descriptor.kind.length === 0 || descriptor.kind.length > 64)
      return;
    if (!admittedCount(descriptor.operation) || !admittedCount(descriptor.generation) || !admittedCount(descriptor.inputItems) || !admittedCount(descriptor.inputBytes) || !admittedCount(descriptor.outputItems) || !admittedCount(descriptor.outputBytes) || !admittedCount(descriptor.inputPageItems) || !admittedCount(descriptor.outputPageItems) || !admittedCount(descriptor.pageBytes))
      return;
    if (descriptor.inputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.inputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY || descriptor.outputItems > INTERACTIVE_JOB_INPUT_ITEM_CAPACITY || descriptor.outputBytes > INTERACTIVE_JOB_INPUT_BYTE_CAPACITY)
      return;
    if (descriptor.inputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.outputPageItems > INTERACTIVE_JOB_PAGE_ITEM_CAPACITY || descriptor.pageBytes > INTERACTIVE_JOB_PAGE_BYTE_CAPACITY)
      return;
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY)
      return;
    if (this.slots.some((slot) => slot?.descriptor.operation === descriptor.operation))
      return;
    const index = this.slots.findIndex((slot) => slot === undefined);
    if (index < 0)
      return;
    this.slots[index] = { descriptor, consumer, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, closing: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    try {
      this.send({ kind: "job-submit", lifecycle: this.lifecycle, descriptor });
    } catch {
      this.slots[index] = undefined;
      this.reservedItems -= reservedItems;
      this.reservedBytes -= reservedBytes;
      return;
    }
    return { operation: descriptor.operation, generation: descriptor.generation, cancel: () => this.cancel(descriptor.operation, descriptor.generation) };
  }
  receive(message) {
    if (!message.kind.startsWith("job-"))
      return false;
    if (message.lifecycle !== this.lifecycle || this.status !== "ready")
      return true;
    if (!admittedCount(message.operation) || !admittedCount(message.generation)) {
      this.quarantine("interactive job message identity was invalid");
      return true;
    }
    const index = this.slots.findIndex((slot2) => slot2?.descriptor.operation === message.operation);
    if (index < 0)
      return true;
    const slot = this.slots[index];
    if (message.generation > slot.descriptor.generation) {
      this.quarantine(`interactive job returned future generation ${message.generation}`);
      return true;
    }
    if (message.generation < slot.descriptor.generation)
      return true;
    if (slot.closing)
      return true;
    if (message.kind === "job-input-pull") {
      if (!admittedCount(message.cursor) || message.cursor !== slot.inputCursor || !admittedCount(message.maxItems) || message.maxItems === 0 || message.maxItems > slot.descriptor.inputPageItems) {
        this.quarantine("interactive job pull exceeded fixed credits");
        return true;
      }
      const startedAt2 = this.now();
      let page;
      try {
        page = slot.consumer.readInputPage(message.cursor, Math.min(message.maxItems, slot.descriptor.inputPageItems));
      } catch (error) {
        this.quarantine(`input consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "input consumer"))
        return true;
      if (!this.admitPage(slot, page, true))
        return true;
      slot.inputCursor += page.itemCount;
      try {
        this.send({ kind: "job-input-page", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: message.cursor, page });
      } catch (error) {
        this.quarantine(`input page transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      return true;
    }
    if (message.kind === "job-output-page") {
      if (!this.admitPage(slot, message.page, false))
        return true;
      const startedAt2 = this.now();
      try {
        slot.consumer.onOutputPage(message.page);
      } catch (error) {
        this.quarantine(`output consumer threw: ${error instanceof Error ? error.message : String(error)}`);
        return true;
      }
      if (!this.observe(startedAt2, "output consumer"))
        return true;
      return true;
    }
    if (message.status !== "complete" && message.status !== "cancelled" && message.status !== "fault") {
      this.quarantine("interactive job returned invalid terminal status");
      return true;
    }
    const terminal = { operation: message.operation, generation: message.generation, status: message.status, ...message.detail === undefined ? {} : { detail: message.detail } };
    const startedAt = this.now();
    try {
      slot.consumer.onTerminal(terminal);
    } catch (error) {
      this.quarantine(`terminal consumer threw: ${error instanceof Error ? error.message : String(error)}`);
      slot.closing = true;
      this.scheduleClose();
      return true;
    }
    slot.closing = true;
    if (!this.observe(startedAt, "terminal consumer"))
      return true;
    this.scheduleClose();
    return true;
  }
  close() {
    if (this.status === "closed")
      return;
    this.status = "closed";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  closeStep() {
    if (this.status !== "closed" && this.status !== "quarantined")
      return false;
    return this.drainClosingStep();
  }
  drainClosingStep() {
    while (this.closeCursor < this.slots.length && (!this.slots[this.closeCursor] || !this.slots[this.closeCursor].closing))
      this.closeCursor++;
    if (this.closeCursor === this.slots.length)
      return true;
    const slot = this.slots[this.closeCursor];
    const startedAt = this.now();
    let complete = false;
    try {
      complete = slot.consumer.closeStep();
      if (complete)
        complete = slot.consumer.terminalIsEmpty();
    } catch (error) {
      this.quarantine(`consumer close threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    if (!this.observe(startedAt, "consumer close"))
      return false;
    if (complete) {
      this.releaseSlot(this.closeCursor);
      this.closeCursor++;
    }
    return false;
  }
  quarantineFromOwner() {
    if (this.status === "closed")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
  }
  cancel(operation, generation2) {
    if (this.status !== "ready")
      return false;
    const slot = this.slots.find((candidate) => candidate?.descriptor.operation === operation);
    if (!slot || slot.descriptor.generation !== generation2)
      return false;
    try {
      this.send({ kind: "job-cancel", lifecycle: this.lifecycle, operation, generation: generation2 });
    } catch (error) {
      this.quarantine(`cancel transfer threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
    return true;
  }
  admitPage(slot, page, input) {
    const pageItemLimit = input ? slot.descriptor.inputPageItems : slot.descriptor.outputPageItems;
    if (!admittedCount(page.itemCount) || !admittedCount(page.byteLength) || typeof page.complete !== "boolean" || page.itemCount === 0 && !page.complete || page.itemCount > pageItemLimit || page.byteLength > slot.descriptor.pageBytes) {
      this.quarantine("interactive job page exceeded fixed credits");
      return false;
    }
    const items = (input ? slot.inputItems : slot.outputItems) + page.itemCount;
    const bytes = (input ? slot.inputBytes : slot.outputBytes) + page.byteLength;
    const itemLimit = input ? slot.descriptor.inputItems : slot.descriptor.outputItems;
    const byteLimit = input ? slot.descriptor.inputBytes : slot.descriptor.outputBytes;
    if (items > itemLimit || bytes > byteLimit) {
      this.quarantine("interactive job aggregate credits exhausted");
      return false;
    }
    if (page.complete && items !== itemLimit || !page.complete && items >= itemLimit) {
      this.quarantine("interactive job page completion violated declared item credits");
      return false;
    }
    if (input) {
      slot.inputItems = items;
      slot.inputBytes = bytes;
    } else {
      slot.outputItems = items;
      slot.outputBytes = bytes;
    }
    return true;
  }
  observe(startedAt, site) {
    const duration = this.now() - startedAt;
    if (duration < INTERACTIVE_JOB_UI_BUDGET_MS)
      return true;
    this.quarantine(`${site} took ${duration.toFixed(3)} ms`);
    return false;
  }
  quarantine(detail) {
    if (this.status !== "ready")
      return;
    this.status = "quarantined";
    this.closeCursor = 0;
    for (let index = 0;index < this.slots.length; index++)
      if (this.slots[index])
        this.slots[index].closing = true;
    this.publishStatus();
    this.scheduleClose();
    this.quarantineConsumer(detail);
  }
  notifyObservers() {
    this.observerCursor = 0;
    if (this.observerNotifyScheduled)
      return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  publishStatus() {
    this.statusRevision += 1;
    this.statusSnapshot = { status: this.status, revision: this.statusRevision };
    this.notifyObservers();
  }
  notifyOneObserver() {
    this.observerNotifyScheduled = false;
    while (this.observerCursor < this.observers.length && !this.observers[this.observerCursor])
      this.observerCursor++;
    if (this.observerCursor === this.observers.length)
      return;
    const observer = this.observers[this.observerCursor++];
    const startedAt = this.now();
    try {
      observer();
    } catch (error) {
      this.quarantine(`status observer threw: ${error instanceof Error ? error.message : String(error)}`);
      return;
    }
    if (!this.observe(startedAt, "status observer"))
      return;
    this.observerNotifyScheduled = true;
    this.schedule(() => this.notifyOneObserver());
  }
  releaseSlot(index) {
    const slot = this.slots[index];
    if (!slot)
      return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }
  scheduleClose() {
    if (this.closeScheduled)
      return;
    this.closeScheduled = true;
    this.schedule(() => {
      this.closeScheduled = false;
      this.closeCursor = 0;
      if (!this.drainClosingStep())
        this.scheduleClose();
    });
  }
}
function admittedCount(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

/* ../../📇️interactive-job-registry/🟦️.ts */
var DIAGRAM_DESCRIPTOR = {
  kind: DIAGRAM_LAYOUT_CODEC_KIND,
  inputPageItems: 64,
  outputPageItems: 128,
  pageBytes: 16 * 1024,
  create(descriptor) {
    const payload = descriptor.payload;
    if (payload.kind !== DIAGRAM_LAYOUT_CODEC_KIND || payload.generation !== descriptor.generation)
      return;
    return new DiagramInteractiveWorkerJob(createDiagramLayoutWorkerJob(payload), descriptor.generation);
  }
};
var INTERACTIVE_WORKER_DESCRIPTORS = Object.freeze([DIAGRAM_DESCRIPTOR]);

class DiagramInteractiveWorkerJob {
  job;
  generation;
  constructor(job, generation2) {
    this.job = job;
    this.generation = generation2;
  }
  acceptInput(payload) {
    return this.job.ingest(payload);
  }
  cancel() {
    this.job.cancel(this.generation);
  }
  close(step13) {
    return this.job.close({ deadline: step13.deadlineMs, fuel: step13.fuel });
  }
  step(step13) {
    return this.job.step({ deadline: step13.deadlineMs, fuel: step13.fuel, generation: this.generation }).status;
  }
  takeOutput() {
    const page = this.job.takePreviewPage() ?? this.job.takeResultPage();
    if (!page)
      return;
    return { itemCount: page.values.length, byteLength: page.values.length * 32, payload: page, complete: page.complete };
  }
  terminal() {
    const terminal = this.job.terminal();
    if (!terminal)
      return;
    return terminal.status === "fault" ? { status: "fault", detail: terminal.reason } : { status: terminal.status };
  }
}

class InteractiveWorkerScheduler {
  lifecycle;
  descriptors;
  post;
  schedule;
  now;
  fault;
  slots = new Array(INTERACTIVE_JOB_SLOT_CAPACITY);
  cursor = 0;
  scheduled = false;
  closed = false;
  closeCursor = 0;
  reservedItems = 0;
  reservedBytes = 0;
  constructor(lifecycle, descriptors, post, schedule, now, fault) {
    this.lifecycle = lifecycle;
    this.descriptors = descriptors;
    this.post = post;
    this.schedule = schedule;
    this.now = now;
    this.fault = fault;
  }
  receive(message) {
    try {
      return this.receiveOwned(message);
    } catch (error) {
      return this.protocolFault(`interactive job callback threw: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  receiveOwned(message) {
    if (!message.kind.startsWith("job-"))
      return false;
    if (this.closed || message.lifecycle !== this.lifecycle)
      return true;
    if (message.kind === "job-submit")
      return this.submit(message.descriptor);
    if (!admittedCount2(message.operation) || !admittedCount2(message.generation))
      return this.protocolFault("interactive job message identity was invalid");
    const index = this.find(message.operation);
    if (index < 0)
      return true;
    const slot = this.slots[index];
    if (message.generation > slot.descriptor.generation)
      return this.protocolFault("interactive job future generation");
    if (message.generation < slot.descriptor.generation)
      return true;
    if (message.kind === "job-cancel") {
      slot.job.cancel();
      slot.phase = "closing";
      this.scheduleRun();
      return true;
    }
    if (slot.phase !== "ingress" || message.cursor !== slot.inputCursor)
      return this.protocolFault("interactive job ingress cursor mismatch");
    if (!admitPage2(message.page, slot.descriptor.inputPageItems, slot.descriptor.pageBytes) || message.page.itemCount === 0 && !message.page.complete)
      return this.protocolFault("interactive job input page exceeded fixed credits");
    const items = slot.inputItems + message.page.itemCount;
    const bytes = slot.inputBytes + message.page.byteLength;
    if (items > slot.descriptor.inputItems || bytes > slot.descriptor.inputBytes)
      return this.protocolFault("interactive job input credits exhausted");
    if (!slot.job.acceptInput(message.page.payload))
      return this.protocolFault("interactive job rejected input ownership");
    slot.inputItems = items;
    slot.inputBytes = bytes;
    slot.inputCursor += message.page.itemCount;
    if (message.page.complete) {
      slot.phase = "running";
      this.scheduleRun();
    } else {
      this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: message.operation, generation: message.generation, cursor: slot.inputCursor, maxItems: slot.descriptor.inputPageItems });
    }
    return true;
  }
  close() {
    if (this.closed)
      return;
    this.closed = true;
    this.closeCursor = 0;
  }
  closeStep() {
    try {
      return this.closeOwnedStep();
    } catch (error) {
      this.protocolFault(`interactive job close callback threw: ${error instanceof Error ? error.message : String(error)}`);
      return false;
    }
  }
  closeOwnedStep() {
    for (let scanned = 0;scanned < this.slots.length; scanned++) {
      const index = (this.closeCursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot)
        continue;
      if (slot.phase !== "closing") {
        slot.job.cancel();
        slot.phase = "closing";
        this.closeCursor = (index + 1) % this.slots.length;
        return false;
      }
      if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 }))
        this.releaseSlot(index);
      this.closeCursor = (index + 1) % this.slots.length;
      return false;
    }
    return true;
  }
  submit(descriptor) {
    if (!admitDescriptor(descriptor) || this.find(descriptor.operation) >= 0) {
      this.postTerminal(descriptor, "fault", "interactive job descriptor unavailable or saturated");
      return true;
    }
    const index = this.slots.findIndex((slot) => slot === undefined);
    const factory = this.descriptors.find((candidate) => candidate.kind === descriptor.kind);
    if (factory && (descriptor.inputPageItems !== factory.inputPageItems || descriptor.outputPageItems !== factory.outputPageItems || descriptor.pageBytes !== factory.pageBytes)) {
      this.postTerminal(descriptor, "fault", "interactive job kind limits do not match the static registry");
      return true;
    }
    const reservedItems = descriptor.inputItems + descriptor.outputItems;
    const reservedBytes = descriptor.inputBytes + descriptor.outputBytes;
    if (this.reservedItems + reservedItems > INTERACTIVE_JOB_PORT_ITEM_CAPACITY || this.reservedBytes + reservedBytes > INTERACTIVE_JOB_PORT_BYTE_CAPACITY) {
      this.postTerminal(descriptor, "fault", "interactive job process credits saturated");
      return true;
    }
    const job = factory?.create(descriptor);
    if (index < 0 || !job) {
      this.postTerminal(descriptor, "fault", "interactive job kind unavailable or slots saturated");
      return true;
    }
    this.slots[index] = { descriptor, job, inputCursor: 0, inputItems: 0, inputBytes: 0, outputItems: 0, outputBytes: 0, phase: "ingress", afterPublish: "running", terminalSent: false };
    this.reservedItems += reservedItems;
    this.reservedBytes += reservedBytes;
    this.post({ kind: "job-input-pull", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, cursor: 0, maxItems: descriptor.inputPageItems });
    return true;
  }
  scheduleRun() {
    if (this.scheduled || this.closed)
      return;
    this.scheduled = true;
    this.schedule(() => {
      this.scheduled = false;
      const startedAt = this.now();
      try {
        this.runOne();
      } catch (error) {
        this.protocolFault(`interactive job Worker callback threw: ${error instanceof Error ? error.message : String(error)}`);
      }
      if (this.now() - startedAt >= 8)
        this.protocolFault("interactive job Worker turn exceeded budget");
    });
  }
  runOne() {
    if (this.closed)
      return;
    for (let scanned = 0;scanned < this.slots.length; scanned++) {
      const index = (this.cursor + scanned) % this.slots.length;
      const slot = this.slots[index];
      if (!slot || slot.phase === "ingress")
        continue;
      this.cursor = (index + 1) % this.slots.length;
      if (slot.phase === "running") {
        const status = slot.job.step({ deadlineMs: this.now() + 6, fuel: 16384 });
        if (status !== "running" && status !== "complete" && status !== "cancelled" && status !== "fault")
          return void this.protocolFault("interactive job returned invalid step status");
        if (status === "running" || status === "complete") {
          slot.phase = "publishing";
          slot.afterPublish = status === "running" ? "running" : "closing";
        } else
          slot.phase = "closing";
        this.scheduleRun();
        return;
      }
      if (slot.phase === "publishing") {
        const page = slot.job.takeOutput();
        if (!page)
          slot.phase = slot.afterPublish;
        else {
          if (!admitPage2(page, slot.descriptor.outputPageItems, slot.descriptor.pageBytes))
            return void this.protocolFault("interactive job output page exceeded fixed credits");
          slot.outputItems += page.itemCount;
          slot.outputBytes += page.byteLength;
          if (slot.outputItems > slot.descriptor.outputItems || slot.outputBytes > slot.descriptor.outputBytes)
            return void this.protocolFault("interactive job output credits exhausted");
          this.post({ kind: "job-output-page", lifecycle: this.lifecycle, operation: slot.descriptor.operation, generation: slot.descriptor.generation, page });
          if (page.complete)
            slot.phase = "closing";
          else if (slot.afterPublish === "running")
            slot.phase = "running";
        }
        this.scheduleRun();
        return;
      }
      if (slot.phase === "closing") {
        const terminal = slot.job.terminal() ?? { status: "cancelled" };
        if (terminal.status !== "complete" && terminal.status !== "cancelled" && terminal.status !== "fault")
          return void this.protocolFault("interactive job returned invalid terminal status");
        if (!slot.terminalSent) {
          this.postTerminal(slot.descriptor, terminal.status, terminal.detail);
          slot.terminalSent = true;
          this.scheduleRun();
          return;
        }
        if (slot.job.close({ deadlineMs: this.now() + 6, fuel: 1024 }))
          this.releaseSlot(index);
      }
      this.scheduleRun();
      return;
    }
  }
  find(operation) {
    return this.slots.findIndex((slot) => slot?.descriptor.operation === operation);
  }
  postTerminal(descriptor, status, detail) {
    this.post({ kind: "job-terminal", lifecycle: this.lifecycle, operation: descriptor.operation, generation: descriptor.generation, status, ...detail === undefined ? {} : { detail } });
  }
  releaseSlot(index) {
    const slot = this.slots[index];
    if (!slot)
      return;
    this.reservedItems -= slot.descriptor.inputItems + slot.descriptor.outputItems;
    this.reservedBytes -= slot.descriptor.inputBytes + slot.descriptor.outputBytes;
    this.slots[index] = undefined;
  }
  protocolFault(detail) {
    this.close();
    try {
      this.fault(detail);
    } catch {}
    return true;
  }
}
function admitDescriptor(descriptor) {
  return descriptor.kind.length > 0 && descriptor.kind.length <= 64 && admittedCount2(descriptor.operation) && admittedCount2(descriptor.generation) && admittedCount2(descriptor.inputItems) && admittedCount2(descriptor.inputBytes) && admittedCount2(descriptor.outputItems) && admittedCount2(descriptor.outputBytes) && admittedCount2(descriptor.inputPageItems) && admittedCount2(descriptor.outputPageItems) && admittedCount2(descriptor.pageBytes) && descriptor.inputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.outputItems <= INTERACTIVE_JOB_INPUT_ITEM_CAPACITY && descriptor.inputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.outputBytes <= INTERACTIVE_JOB_INPUT_BYTE_CAPACITY && descriptor.inputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.outputPageItems <= INTERACTIVE_JOB_PAGE_ITEM_CAPACITY && descriptor.pageBytes <= INTERACTIVE_JOB_PAGE_BYTE_CAPACITY;
}
function admitPage2(page, itemCapacity, byteCapacity) {
  return admittedCount2(page.itemCount) && admittedCount2(page.byteLength) && typeof page.complete === "boolean" && page.itemCount <= itemCapacity && page.byteLength <= byteCapacity;
}
function admittedCount2(value) {
  return Number.isSafeInteger(value) && value >= 0;
}

/* ../../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🟦️.ts */
function localInteractionIdentityEquals(left, right) {
  return left.appInstanceId === right.appInstanceId && left.generation === right.generation && left.revision === right.revision && left.documentRevision === right.documentRevision && left.topologyRevision === right.topologyRevision;
}
/* ../../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/📡️transport/🟦️.ts */
var rejections = ["busy", "closed", "generation-exhausted", "source-failed"];
var maximumWireBytes = 4256;
function encodeLocalInteractionUnsigned(text2) {
  if (!/^(0|[1-9][0-9]{0,19})$/.test(text2))
    throw new Error("local-interaction.invalid-u64");
  let value = BigInt(text2);
  if (value > 0xffffffffffffffffn)
    throw new Error("local-interaction.invalid-u64");
  const result3 = [];
  do {
    const byte = Number(value & 127n);
    value >>= 7n;
    result3.push(byte | (value === 0n ? 0 : 128));
  } while (value !== 0n);
  return result3;
}

class Reader {
  bytes;
  offset = 0;
  constructor(bytes) {
    this.bytes = bytes;
    if (bytes.length > maximumWireBytes)
      throw new Error("local-interaction.wire-envelope");
  }
  byte() {
    const value = this.bytes[this.offset++];
    if (value === undefined)
      throw new Error("local-interaction.truncated");
    return value;
  }
  unsigned() {
    let value = 0n;
    for (let index = 0;index < 10; index++) {
      const byte = this.byte();
      if (index === 9 && byte > 1)
        throw new Error("local-interaction.invalid-u64");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if (index !== 0 && byte === 0)
          throw new Error("local-interaction.noncanonical-u64");
        return value.toString();
      }
    }
    throw new Error("local-interaction.invalid-u64");
  }
  hash() {
    let result3 = "";
    for (let index = 0;index < 32; index++)
      result3 += this.byte().toString(16).padStart(2, "0");
    return result3;
  }
  bool() {
    const value = this.byte();
    if (value > 1)
      throw new Error("local-interaction.invalid-bool");
    return value === 1;
  }
  finish() {
    if (this.offset !== this.bytes.length)
      throw new Error("local-interaction.trailing-bytes");
  }
}
function hash(out, value) {
  if (!/^[0-9a-f]{64}$/.test(value))
    throw new Error("local-interaction.invalid-revision");
  for (let index = 0;index < 64; index += 2)
    out.push(Number.parseInt(value.slice(index, index + 2), 16));
}
function token(out, value) {
  out.push(...encodeLocalInteractionUnsigned(value.requestId), ...encodeLocalInteractionUnsigned(value.queryGeneration));
  const identity = value.identity;
  if (!Number.isInteger(identity.appInstanceId) || identity.appInstanceId < 0 || identity.appInstanceId > 4294967295)
    throw new Error("local-interaction.invalid-instance");
  out.push(...encodeLocalInteractionUnsigned(String(identity.appInstanceId)), ...encodeLocalInteractionUnsigned(identity.generation));
  hash(out, identity.revision);
  hash(out, identity.documentRevision);
  hash(out, identity.topologyRevision);
  out.push(...encodeLocalInteractionUnsigned(value.ordinal));
}
function readToken(reader) {
  const requestId = reader.unsigned(), queryGeneration = reader.unsigned();
  const instance = BigInt(reader.unsigned());
  if (instance > 0xffffffffn)
    throw new Error("local-interaction.invalid-instance");
  const identity = { appInstanceId: Number(instance), generation: reader.unsigned(), revision: reader.hash(), documentRevision: reader.hash(), topologyRevision: reader.hash() };
  return { requestId, queryGeneration, identity, ordinal: reader.unsigned() };
}
function encodeLocalInteractionQueryCommand(command) {
  const out = [command.kind === "read" ? 0 : command.kind === "acknowledge" ? 1 : 2];
  if (command.kind === "read")
    out.push(...encodeLocalInteractionUnsigned(command.requestId));
  else
    token(out, command.token);
  return Uint8Array.from(out);
}
function decodeLocalInteractionQueryReply(bytes) {
  const reader = new Reader(bytes), kind = reader.byte();
  let result3;
  if (kind === 0)
    result3 = { kind: "started", token: readToken(reader) };
  else if (kind === 1) {
    const token2 = readToken(reader), terminal = reader.bool(), length = Number(reader.unsigned());
    if (length > 4096)
      throw new Error("local-interaction.page-length");
    const payload = [];
    for (let index = 0;index < length; index++)
      payload.push(reader.byte());
    result3 = { kind: "page", page: { ...token2, terminal, bytes: payload } };
  } else if (kind === 2)
    result3 = { kind: "closed", token: readToken(reader), cancelled: reader.bool() };
  else if (kind === 3) {
    const requestId = reader.unsigned(), code = rejections[reader.byte()];
    if (code === undefined)
      throw new Error("local-interaction.rejection-code");
    result3 = { kind: "rejected", requestId, code };
  } else
    throw new Error("local-interaction.reply-kind");
  reader.finish();
  return result3;
}

/* ../../../../../../../../../🔨️modules/📡️replication/🟦️.ts */
//! 📡️ Replication contract — TypeScript twin of the Rust `protocol` crate.
//!
//! Byte-for-byte identical to `📦️packages/🦀️rust`'s encoders: the 20 frames in `🧫️fixtures/📡️wire`
//! are the shared gate both sides must reproduce. Frame layout is `lane u8`, `frame tag u8`, then
//! fields in declaration order — no length prefix, no per-field tags.
function mutationEnvelopeToWire(envelope, timestamp, codec) {
  const packPayload = (value) => Array.from(codec.encode(value));
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
    dependencies: [...envelope.deps ?? []],
    diff: { schema: envelope.diff.schemaId, payload: packPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: packPayload(envelope.inverse.inverseDiff.payload) },
    timestamp
  };
}
function writeVarintU64(out, value) {
  let remaining = value;
  for (;; ) {
    const byte = remaining & 127;
    remaining = Math.floor(remaining / 128);
    if (remaining === 0) {
      out.push(byte);
      return;
    }
    out.push(byte | 128);
  }
}
function readVarintU64(bytes, pos) {
  let result3 = 0;
  let shift = 1;
  for (let i = 0;i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined)
      throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result3 += (byte & 127) * shift;
    if ((byte & 128) === 0)
      return result3;
    shift *= 128;
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}
function writeStr(out, value) {
  const bytes = new TextEncoder().encode(value);
  writeVarintU64(out, bytes.length);
  for (const byte of bytes)
    out.push(byte);
}
function readStr(bytes, pos) {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len)
    throw new Error("wire str: truncated");
  pos[0] += len;
  return new TextDecoder().decode(slice);
}
function writeBytes(out, value) {
  writeVarintU64(out, value.length);
  for (const byte of value)
    out.push(byte);
}
function readBytes(bytes, pos) {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len)
    throw new Error("wire bytes: truncated");
  pos[0] += len;
  return Array.from(slice);
}
function writeBool(out, value) {
  out.push(value ? 1 : 0);
}
function readBool(bytes, pos) {
  const byte = bytes[pos[0]];
  if (byte === undefined)
    throw new Error("wire bool: truncated");
  pos[0] += 1;
  return byte !== 0;
}
function writeF64(out, value) {
  const buffer = new ArrayBuffer(8);
  new DataView(buffer).setFloat64(0, value, true);
  for (const byte of new Uint8Array(buffer))
    out.push(byte);
}
function readF64(bytes, pos) {
  const slice = bytes.subarray(pos[0], pos[0] + 8);
  if (slice.length !== 8)
    throw new Error("wire f64: truncated");
  pos[0] += 8;
  return new DataView(slice.buffer, slice.byteOffset, 8).getFloat64(0, true);
}
function writeVecBytes(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writeBytes(out, value);
}
function readVecBytes(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  const result3 = [];
  for (let i = 0;i < count; i++)
    result3.push(readBytes(bytes, pos));
  return result3;
}
function presencePresent(value) {
  return value !== undefined && value !== null;
}
function encodePresencePeer(peer) {
  const out = [];
  writeStr(out, peer.actor);
  let flags = 0;
  if (presencePresent(peer.label))
    flags |= 1 << 0;
  if (presencePresent(peer.presencePack))
    flags |= 1 << 1;
  if (presencePresent(peer.userId))
    flags |= 1 << 2;
  if (presencePresent(peer.role))
    flags |= 1 << 3;
  if (presencePresent(peer.dragGhostJson))
    flags |= 1 << 4;
  if (presencePresent(peer.interaction))
    flags |= 1 << 5;
  if (presencePresent(peer.color))
    flags |= 1 << 6;
  if (presencePresent(peer.surface))
    flags |= 1 << 7;
  if (peer.views.length > 0)
    flags |= 1 << 8;
  if (presencePresent(peer.ui))
    flags |= 1 << 9;
  writeVarintU64(out, flags);
  writeVarintU64(out, peer.connectedAtMs ?? 0);
  if (presencePresent(peer.label))
    writeStr(out, peer.label);
  if (presencePresent(peer.presencePack))
    writeBytes(out, peer.presencePack);
  if (presencePresent(peer.userId))
    writeStr(out, peer.userId);
  if (presencePresent(peer.role))
    writeStr(out, peer.role);
  if (presencePresent(peer.dragGhostJson))
    writeStr(out, peer.dragGhostJson);
  if (presencePresent(peer.interaction))
    writePresenceInteraction(out, peer.interaction);
  if (presencePresent(peer.color))
    out.push(peer.color);
  if (presencePresent(peer.surface))
    writeStr(out, peer.surface);
  if (peer.views.length > 0)
    writeVecPresenceWindowView(out, peer.views);
  if (presencePresent(peer.ui))
    writePresenceUi(out, peer.ui);
  return out;
}
function writePresenceInteraction(out, interaction) {
  writeStr(out, interaction.app_id);
  writeVarintU64(out, interaction.domains.length);
  for (const domain of interaction.domains) {
    writeStr(out, domain.domain);
    writeStr(out, domain.granularity);
    writeVecStr(out, domain.selected);
    writeVecStr(out, domain.hovered);
  }
}
function writePresenceViewKind(out, kind) {
  if (kind.kind === "canvas") {
    out.push(0);
    writeF64(out, kind.x);
    writeF64(out, kind.y);
    writeF64(out, kind.zoom);
  } else if (kind.kind === "orbit") {
    out.push(1);
    for (const value of [...kind.position, ...kind.target, ...kind.up])
      writeF64(out, value);
    writeF64(out, kind.fov);
  } else {
    out.push(2);
    writeF64(out, kind.lng);
    writeF64(out, kind.lat);
    writeF64(out, kind.zoom);
    writeF64(out, kind.bearing);
    writeF64(out, kind.pitch);
  }
}
function writePresenceWindowView(out, view) {
  writeStr(out, view.windowId);
  writeStr(out, view.space);
  writePresenceViewKind(out, view.kind);
  writeF64(out, view.size[0]);
  writeF64(out, view.size[1]);
  writeBool(out, presencePresent(view.pointer));
  if (presencePresent(view.pointer))
    for (const value of view.pointer)
      writeF64(out, value);
}
function writeVecPresenceWindowView(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writePresenceWindowView(out, value);
}
function writePresenceUi(out, ui) {
  writeOptStr(out, ui.hoveredPath ?? null);
  writeOptStr(out, ui.focusedPath ?? null);
  writeOptStr(out, ui.pressedPath ?? null);
}
function writeOptStr(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    writeStr(out, value);
}
function writeVecStr(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    writeStr(out, value);
}
function writeVecEnvelope(out, values) {
  writeVarintU64(out, values.length);
  for (const value of values)
    encodeEnvelope(out, value);
}
function encodeHlc(out, hlc) {
  writeVarintU64(out, hlc.actor);
  writeVarintU64(out, hlc.physical_ms);
  writeVarintU64(out, hlc.logical);
}
function encodeEnvelope(out, envelope) {
  writeStr(out, envelope.mutation_id);
  writeStr(out, envelope.document_id);
  writeStr(out, envelope.actor);
  writeVecStr(out, envelope.dependencies);
  writeStr(out, envelope.diff.schema);
  writeBytes(out, envelope.diff.payload);
  writeStr(out, envelope.inverse.schema);
  writeBytes(out, envelope.inverse.payload);
  encodeHlc(out, envelope.timestamp);
}
var ARTIFACT_BOOTSTRAP_FORMAT_VERSION = 1;
var ARTIFACT_BOOTSTRAP_CHUNK_BYTES = 4 * 1024;
var ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES = 64 * 1024 * 1024;
var ARTIFACT_BOOTSTRAP_MAX_CHUNKS = 16 * 1024;
var DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS = Object.freeze({ maxTotalBytes: ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, maxChunks: ARTIFACT_BOOTSTRAP_MAX_CHUNKS, maxChunkBytes: ARTIFACT_BOOTSTRAP_CHUNK_BYTES });
function artifactBootstrapError(message) {
  return new Error(`artifact bootstrap ${message}`);
}
function equalBytes(left, right) {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}
function validateBytes(name, value) {
  if (value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255))
    throw artifactBootstrapError(`${name} contains an invalid byte`);
}
function validateHash(name, value, nonzero) {
  if (value.length !== 32)
    throw artifactBootstrapError(`${name} must contain 32 bytes`);
  validateBytes(name, value);
  if (nonzero && value.every((byte) => byte === 0))
    throw artifactBootstrapError(`${name} must be nonzero`);
}
function validateNatural(name, value, minimum) {
  if (!Number.isSafeInteger(value) || value < minimum)
    throw artifactBootstrapError(`${name} is invalid`);
}
function validateLimits(limits) {
  validateNatural("max total bytes", limits.maxTotalBytes, 1);
  validateNatural("max chunks", limits.maxChunks, 1);
  validateNatural("max chunk bytes", limits.maxChunkBytes, 1);
  if (limits.maxChunkBytes > ARTIFACT_BOOTSTRAP_CHUNK_BYTES)
    throw artifactBootstrapError("max chunk bytes exceeds wire limit");
}
function artifactBootstrapTotal(bootstrap) {
  validateNatural("pack length", bootstrap.pack_length, 1);
  validateNatural("SPR length", bootstrap.spr_length, 1);
  const total = bootstrap.pack_length + bootstrap.spr_length;
  if (!Number.isSafeInteger(total))
    throw artifactBootstrapError("total bytes overflow");
  return total;
}
function validateArtifactBootstrapHeader(bootstrap, inline, limits) {
  validateLimits(limits);
  if (bootstrap.format_version !== ARTIFACT_BOOTSTRAP_FORMAT_VERSION)
    throw artifactBootstrapError(`version ${bootstrap.format_version} is unsupported`);
  validateHash("descriptor hash", bootstrap.descriptor_hash, true);
  validateHash("pack schema hash", bootstrap.pack_schema_hash, true);
  validateHash("pack hash", bootstrap.pack_hash, true);
  validateHash("SPR hash", bootstrap.spr_hash, true);
  validateHash("aggregate hash", bootstrap.aggregate_hash, true);
  validateHash("baseline frontier chain hash", bootstrap.baseline_frontier.chain_hash, false);
  validateHash("required tail frontier chain hash", bootstrap.required_tail_frontier.chain_hash, false);
  const schemaBytes = new TextEncoder().encode(bootstrap.artifact_schema).length;
  const kindBytes = new TextEncoder().encode(bootstrap.artifact_kind).length;
  if (schemaBytes === 0 || schemaBytes > 256)
    throw artifactBootstrapError("artifact schema length is invalid");
  if (kindBytes === 0 || kindBytes > 256)
    throw artifactBootstrapError("artifact kind length is invalid");
  if (bootstrap.baseline_frontier.document_id.length === 0 || bootstrap.baseline_frontier.document_id !== bootstrap.required_tail_frontier.document_id)
    throw artifactBootstrapError("frontier document mismatch");
  validateNatural("baseline head", bootstrap.baseline_frontier.head_edit_ordinal, 0);
  validateNatural("baseline commit", bootstrap.baseline_frontier.last_commit_seq, 0);
  validateNatural("required tail head", bootstrap.required_tail_frontier.head_edit_ordinal, 0);
  validateNatural("required tail commit", bootstrap.required_tail_frontier.last_commit_seq, 0);
  if (bootstrap.required_tail_frontier.head_edit_ordinal < bootstrap.baseline_frontier.head_edit_ordinal || bootstrap.required_tail_frontier.last_commit_seq < bootstrap.baseline_frontier.last_commit_seq)
    throw artifactBootstrapError("required tail frontier precedes baseline");
  const total = artifactBootstrapTotal(bootstrap);
  if (total > limits.maxTotalBytes)
    throw artifactBootstrapError("total bytes exceed assembler budget");
  validateNatural("chunk count", bootstrap.chunk_count, 0);
  if (inline) {
    if (bootstrap.chunk_count !== 0)
      throw artifactBootstrapError("inline pair must declare zero chunks");
  } else {
    if (bootstrap.chunk_count === 0 || bootstrap.chunk_count > limits.maxChunks)
      throw artifactBootstrapError("chunk count exceeds assembler budget");
    if (bootstrap.chunk_count > total || total > bootstrap.chunk_count * limits.maxChunkBytes)
      throw artifactBootstrapError("chunk count cannot cover declared bytes");
  }
  return total;
}
function validateArtifactBootstrap(bootstrap, limits) {
  const inline = bootstrap.inline !== null;
  const total = validateArtifactBootstrapHeader(bootstrap, inline, limits);
  if (bootstrap.inline !== null) {
    if (bootstrap.inline.pack.length !== bootstrap.pack_length || bootstrap.inline.spr.length !== bootstrap.spr_length)
      throw artifactBootstrapError("inline pair lengths do not match metadata");
    validateBytes("inline pack", bootstrap.inline.pack);
    validateBytes("inline SPR", bootstrap.inline.spr);
  }
  return total;
}
async function artifactBootstrapSha256(bytes) {
  const owned = Uint8Array.from(bytes);
  return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", owned.buffer));
}
class ArtifactBootstrapAssembler {
  bootstrap;
  limits;
  deadlineMs;
  #storage;
  #received = 0;
  #nextIndex = 0;
  constructor(bootstrap, expectedDescriptorHash, limits = DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, deadlineMs = null, control) {
    const total = validateArtifactBootstrap(bootstrap, limits);
    validateHash("expected descriptor hash", expectedDescriptorHash, true);
    if (!equalBytes(bootstrap.descriptor_hash, expectedDescriptorHash))
      throw artifactBootstrapError("descriptor mismatch");
    if (control.isCancelled())
      throw artifactBootstrapError("cancelled");
    if (deadlineMs !== null && control.nowMs() >= deadlineMs)
      throw artifactBootstrapError("deadline exceeded");
    this.bootstrap = bootstrap;
    this.limits = limits;
    this.deadlineMs = deadlineMs;
    this.#storage = new Uint8Array(total);
    control.onProgress(this.progress);
    if (bootstrap.inline !== null) {
      this.#storage.set(bootstrap.inline.pack, 0);
      this.#storage.set(bootstrap.inline.spr, bootstrap.pack_length);
      this.#received = total;
      control.onProgress(this.progress);
    }
  }
  get retainedBytes() {
    return this.#storage?.byteLength ?? 0;
  }
  get progress() {
    return { receivedBytes: this.#received, totalBytes: this.bootstrap.pack_length + this.bootstrap.spr_length, receivedChunks: this.#nextIndex, totalChunks: this.bootstrap.chunk_count };
  }
  abort() {
    this.#storage?.fill(0);
    this.#storage = null;
  }
  #fail(message) {
    this.abort();
    throw artifactBootstrapError(message);
  }
  #guard(control) {
    if (control.isCancelled())
      this.#fail("cancelled");
    if (this.deadlineMs !== null && control.nowMs() >= this.deadlineMs)
      this.#fail("deadline exceeded");
    if (this.#storage === null)
      throw artifactBootstrapError("is not active");
  }
  push(chunk, control) {
    this.#guard(control);
    if (this.bootstrap.inline !== null)
      this.#fail("inline transfer cannot accept chunks");
    if (!equalBytes(chunk.descriptor_hash, this.bootstrap.descriptor_hash))
      this.#fail("chunk descriptor mismatch");
    if (chunk.index !== this.#nextIndex)
      this.#fail(`chunk index ${chunk.index} does not equal expected ${this.#nextIndex}`);
    if (chunk.bytes.length === 0 || chunk.bytes.length > this.limits.maxChunkBytes)
      this.#fail("chunk bytes exceed assembler budget");
    try {
      validateBytes("chunk", chunk.bytes);
    } catch {
      this.#fail("chunk contains an invalid byte");
    }
    if (chunk.index >= this.bootstrap.chunk_count)
      this.#fail("chunk index exceeds declared count");
    const end = this.#received + chunk.bytes.length;
    if (end > this.progress.totalBytes)
      this.#fail("chunk bytes exceed declared total");
    this.#storage.set(chunk.bytes, this.#received);
    this.#received = end;
    this.#nextIndex += 1;
    control.onProgress(this.progress);
    return this.progress;
  }
  async finish(done, control) {
    try {
      this.#guard(control);
      if (this.bootstrap.inline === null) {
        if (done === null)
          this.#fail("is incomplete without done frame");
        if (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash))
          this.#fail("done descriptor mismatch");
        if (done.chunk_count !== this.bootstrap.chunk_count)
          this.#fail("done chunk count mismatch");
      } else if (done !== null && (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash) || done.chunk_count !== 0)) {
        this.#fail("inline done metadata mismatch");
      }
      if (this.#nextIndex !== this.bootstrap.chunk_count || this.#received !== this.progress.totalBytes)
        this.#fail("is incomplete");
      const storage = this.#storage;
      const pack = storage.subarray(0, this.bootstrap.pack_length);
      const spr = storage.subarray(this.bootstrap.pack_length);
      const [packHash, sprHash, aggregateHash] = await Promise.all([artifactBootstrapSha256(pack), artifactBootstrapSha256(spr), artifactBootstrapSha256(storage)]);
      this.#guard(control);
      if (!equalBytes(packHash, this.bootstrap.pack_hash))
        this.#fail("pack hash mismatch");
      if (!equalBytes(sprHash, this.bootstrap.spr_hash))
        this.#fail("SPR hash mismatch");
      if (!equalBytes(aggregateHash, this.bootstrap.aggregate_hash))
        this.#fail("aggregate hash mismatch");
      this.#storage = null;
      return { pack, spr };
    } catch (error) {
      this.abort();
      throw error;
    }
  }
}
if (undefined) {}
/* ../../../../../../../🟦️.ts */
var replicationPackCodec = { encode: encodePackValue, decode: decodePackValue };
if (undefined) {}
var JSON_BRIDGE_FIELD_ID = 1;
var PACK_TAG_FALSE = 1;
var PACK_TAG_TRUE = 2;
var PACK_TAG_F64 = 5;
var PACK_TAG_STR = 6;
var PACK_TAG_STR_INLINE = 7;
var PACK_TAG_LIST = 12;
var PACK_TAG_MAP = 16;
var PACK_TAG_VALUE = 17;
var PACK_TAG_NULL = 18;
function packPushBytes(out, bytes) {
  for (let index = 0;index < bytes.length; index++)
    out.push(bytes[index]);
}
function packByteCompare(a, b) {
  const encoder = new TextEncoder;
  const ab = encoder.encode(a);
  const bb = encoder.encode(b);
  const len = Math.min(ab.length, bb.length);
  for (let index = 0;index < len; index++) {
    const diff = ab[index] - bb[index];
    if (diff !== 0)
      return diff;
  }
  return ab.length - bb.length;
}
function packCollectStrings(value, counts) {
  if (typeof value === "string") {
    counts.set(value, (counts.get(value) ?? 0) + 1);
    return;
  }
  if (Array.isArray(value)) {
    for (const item of value)
      packCollectStrings(item, counts);
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value))
      packCollectStrings(item, counts);
  }
}
function packBuildSymbols(value) {
  const counts = new Map;
  packCollectStrings(value, counts);
  const encoder = new TextEncoder;
  const symbols = [];
  for (const [text2, count] of counts)
    if (encoder.encode(text2).length <= 128 || count >= 2)
      symbols.push(text2);
  symbols.sort(packByteCompare);
  return symbols;
}
function packEncodeString(text2, symbolIndex, out) {
  const index = symbolIndex.get(text2);
  if (index !== undefined) {
    out.push(PACK_TAG_STR);
    writeVarintU64(out, index);
    return;
  }
  packEncodeStringInline(text2, out);
}
function packEncodeStringInline(text2, out) {
  const bytes = new TextEncoder().encode(text2);
  out.push(PACK_TAG_STR_INLINE);
  writeVarintU64(out, bytes.length);
  packPushBytes(out, bytes);
}
function packDecodeString(bytes, symbols, pos) {
  const tag = bytes[pos[0]];
  pos[0] += 1;
  if (tag === PACK_TAG_STR) {
    const index = readVarintU64(bytes, pos);
    const symbol = symbols[index];
    if (symbol === undefined)
      throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
    return symbol;
  }
  if (tag === PACK_TAG_STR_INLINE) {
    const len = readVarintU64(bytes, pos);
    const text2 = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
    pos[0] += len;
    return text2;
  }
  throw new Error(`decodePackValue: expected a string tag, found 0x${tag.toString(16)}`);
}
function packEncodeValue(value, symbolIndex, out) {
  if (value === null || value === undefined) {
    out.push(PACK_TAG_NULL);
    return;
  }
  if (typeof value === "boolean") {
    out.push(value ? PACK_TAG_TRUE : PACK_TAG_FALSE);
    return;
  }
  if (typeof value === "number") {
    out.push(PACK_TAG_F64);
    writeF64(out, value === 0 ? 0 : value);
    return;
  }
  if (typeof value === "string") {
    packEncodeString(value, symbolIndex, out);
    return;
  }
  if (Array.isArray(value)) {
    out.push(PACK_TAG_LIST);
    writeVarintU64(out, value.length);
    for (const item of value)
      packEncodeValue(item, symbolIndex, out);
    return;
  }
  if (typeof value === "object") {
    out.push(PACK_TAG_MAP);
    const entries = Object.entries(value).sort((a, b) => packByteCompare(a[0], b[0]));
    writeVarintU64(out, entries.length);
    for (const [key, entryValue2] of entries) {
      packEncodeStringInline(key, out);
      packEncodeValue(entryValue2, symbolIndex, out);
    }
    return;
  }
  throw new Error(`encodePackValue: unsupported JSON value of type ${typeof value}`);
}
function packDecodeValue(bytes, symbols, pos) {
  const tag = bytes[pos[0]];
  pos[0] += 1;
  switch (tag) {
    case PACK_TAG_NULL:
      return null;
    case PACK_TAG_FALSE:
      return false;
    case PACK_TAG_TRUE:
      return true;
    case PACK_TAG_F64:
      return readF64(bytes, pos);
    case PACK_TAG_STR: {
      const index = readVarintU64(bytes, pos);
      const symbol = symbols[index];
      if (symbol === undefined)
        throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
      return symbol;
    }
    case PACK_TAG_STR_INLINE: {
      const len = readVarintU64(bytes, pos);
      const text2 = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
      pos[0] += len;
      return text2;
    }
    case PACK_TAG_LIST: {
      const count = readVarintU64(bytes, pos);
      const items = [];
      for (let i = 0;i < count; i++)
        items.push(packDecodeValue(bytes, symbols, pos));
      return items;
    }
    case PACK_TAG_MAP: {
      const count = readVarintU64(bytes, pos);
      const entries = {};
      for (let i = 0;i < count; i++) {
        const key = packDecodeString(bytes, symbols, pos);
        entries[key] = packDecodeValue(bytes, symbols, pos);
      }
      return entries;
    }
    default:
      throw new Error(`decodePackValue: unrecognized dsl value tag 0x${tag.toString(16)}`);
  }
}
function encodePackValue(value) {
  const symbols = packBuildSymbols(value);
  const symbolIndex = new Map(symbols.map((symbol, index) => [symbol, index]));
  const encoder = new TextEncoder;
  const out = [];
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
  }
  writeVarintU64(out, 1);
  writeVarintU64(out, JSON_BRIDGE_FIELD_ID);
  out.push(PACK_TAG_VALUE);
  packEncodeValue(value, symbolIndex, out);
  return new Uint8Array(out);
}
function decodePackValue(bytes) {
  const pos = [0];
  const decoder = new TextDecoder;
  const symbolCount = readVarintU64(bytes, pos);
  const symbols = [];
  for (let i = 0;i < symbolCount; i++) {
    const len = readVarintU64(bytes, pos);
    symbols.push(decoder.decode(bytes.subarray(pos[0], pos[0] + len)));
    pos[0] += len;
  }
  const fieldCount = readVarintU64(bytes, pos);
  let result3 = null;
  for (let i = 0;i < fieldCount; i++) {
    const fieldId = readVarintU64(bytes, pos);
    const outerTag = bytes[pos[0]];
    pos[0] += 1;
    if (outerTag !== PACK_TAG_VALUE)
      throw new Error(`decodePackValue: unexpected field tag 0x${outerTag.toString(16)} for field ${fieldId}`);
    const value = packDecodeValue(bytes, symbols, pos);
    if (fieldId === JSON_BRIDGE_FIELD_ID)
      result3 = value;
  }
  return result3;
}
var SCENE_PACK_UNIT = Symbol("scene-pack-unit");
function readOptU64(bytes, pos) {
  return readBool(bytes, pos) ? readVarintU64(bytes, pos) : null;
}
function writeOptU8(out, value) {
  writeBool(out, value !== null);
  if (value !== null)
    out.push(value);
}
function writeChildPackEntry(out, entry) {
  writeStr(out, entry.slot);
  writeStr(out, entry.child_id);
  writeStr(out, entry.dialect);
  writeBytes(out, entry.envelope_pack);
}
function readChildPackEntry(bytes, pos) {
  return { slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };
}
function writeVecChildPackEntry(out, entries) {
  writeVarintU64(out, entries.length);
  for (const entry of entries)
    writeChildPackEntry(out, entry);
}
function readVecChildPackEntry(bytes, pos) {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
var APP_COMMAND_TAGS = {
  ConfigCommand: 0,
  Command: 1,
  CommandText: 2,
  ContextMenu: 3,
  ArtifactCommand: 4,
  ApplyEnvelopes: 5,
  LoadDocument: 6,
  ReadDocument: 7,
  LoadConfig: 8,
  ReadConfig: 9,
  MediaIn: 10,
  MediaOut: 11,
  MediaFingerprint: 12,
  PureCommand: 13,
  LoadChildren: 14,
  ReadChildren: 15,
  ReadHistory: 16,
  transactionPrepare: 17,
  transactionCommit: 18,
  transactionRollback: 19,
  transactionUndo: 20,
  transactionRedo: 21,
  openArtifact: 22,
  setDefaultApp: 23,
  clearDefaultApp: 24,
  setMergePolicy: 25,
  resolveConflict: 26,
  readConflicts: 27,
  presence: 28,
  LocalInteractionQuery: 29
};
var APP_FRAME_TAGS = {
  Done: 0,
  Invocation: 1,
  DocumentChanged: 2,
  Document: 3,
  Config: 4,
  ConfigChanged: 5,
  ContextMenu: 6,
  Media: 7,
  MediaFingerprint: 8,
  Error: 9,
  Emit: 10,
  Draft: 11,
  Children: 12,
  Ephemeral: 13,
  HistorySnapshot: 14,
  transactionProposal: 15,
  transactionPrepared: 16,
  transactionCommitted: 17,
  transactionRolledBack: 18,
  MergeReport: 19,
  Conflicts: 20,
  UiPatch: 21,
  UiSnapshotEnd: 22,
  LocalInteractionQuery: 23
};
function encodeAppCommand(cmd) {
  const out = [];
  if ("ConfigCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ConfigCommand);
    writeVarintU64(out, cmd.ConfigCommand.seq);
    writeBytes(out, cmd.ConfigCommand.command);
  } else if ("Command" in cmd) {
    out.push(APP_COMMAND_TAGS.Command);
    writeVarintU64(out, cmd.Command.seq);
    writeBytes(out, cmd.Command.command);
    writeBytes(out, cmd.Command.view_state);
  } else if ("CommandText" in cmd) {
    out.push(APP_COMMAND_TAGS.CommandText);
    writeVarintU64(out, cmd.CommandText.seq);
    writeStr(out, cmd.CommandText.line);
  } else if ("ContextMenu" in cmd) {
    out.push(APP_COMMAND_TAGS.ContextMenu);
    writeVarintU64(out, cmd.ContextMenu.seq);
    writeBytes(out, cmd.ContextMenu.request);
  } else if ("ArtifactCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ArtifactCommand);
    writeVarintU64(out, cmd.ArtifactCommand.seq);
    writeBytes(out, cmd.ArtifactCommand.command);
  } else if ("ApplyEnvelopes" in cmd) {
    out.push(APP_COMMAND_TAGS.ApplyEnvelopes);
    writeVarintU64(out, cmd.ApplyEnvelopes.seq);
    writeVecEnvelope(out, cmd.ApplyEnvelopes.envelopes.map((envelope, index) => mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: 0, logical: index + 1 }, replicationPackCodec)));
  } else if ("LoadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadDocument);
    writeVarintU64(out, cmd.LoadDocument.seq);
    writeBytes(out, cmd.LoadDocument.pack);
    writeBytes(out, cmd.LoadDocument.spr);
  } else if ("ReadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocument);
    writeVarintU64(out, cmd.ReadDocument.seq);
  } else if ("LoadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadConfig);
    writeVarintU64(out, cmd.LoadConfig.seq);
    writeBytes(out, cmd.LoadConfig.pack);
    writeBytes(out, cmd.LoadConfig.spr);
  } else if ("ReadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadConfig);
    writeVarintU64(out, cmd.ReadConfig.seq);
  } else if ("MediaIn" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaIn);
    writeVarintU64(out, cmd.MediaIn.seq);
    writeStr(out, cmd.MediaIn.port);
    writeBytes(out, cmd.MediaIn.descriptor);
    writeBytes(out, cmd.MediaIn.data);
  } else if ("MediaOut" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaOut);
    writeVarintU64(out, cmd.MediaOut.seq);
    writeStr(out, cmd.MediaOut.port);
    writeBytes(out, cmd.MediaOut.request);
  } else if ("MediaFingerprint" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaFingerprint);
    writeVarintU64(out, cmd.MediaFingerprint.seq);
    writeStr(out, cmd.MediaFingerprint.port);
  } else if ("PureCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.PureCommand);
    writeVarintU64(out, cmd.PureCommand.seq);
    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.document);
    writeBytes(out, cmd.PureCommand.document_spr);
    writeBytes(out, cmd.PureCommand.config);
    writeBytes(out, cmd.PureCommand.config_spr);
    writeBytes(out, cmd.PureCommand.draft);
    writeBytes(out, cmd.PureCommand.draft_spr);
  } else if ("LoadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadChildren);
    writeVarintU64(out, cmd.LoadChildren.seq);
    writeVecChildPackEntry(out, cmd.LoadChildren.entries);
  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
  } else if ("ReadHistory" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadHistory);
    writeVarintU64(out, cmd.ReadHistory.seq);
  } else if ("transactionPrepare" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionPrepare);
    writeVarintU64(out, cmd.transactionPrepare.seq);
    writeStr(out, cmd.transactionPrepare.txn_id);
    writeStr(out, cmd.transactionPrepare.mutation_id);
    writeBytes(out, cmd.transactionPrepare.payload);
    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);
    writeStr(out, cmd.transactionPrepare.label);
    writeBytes(out, cmd.transactionPrepare.origin);
  } else if ("transactionCommit" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionCommit);
    writeVarintU64(out, cmd.transactionCommit.seq);
    writeStr(out, cmd.transactionCommit.txn_id);
  } else if ("transactionRollback" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRollback);
    writeVarintU64(out, cmd.transactionRollback.seq);
    writeStr(out, cmd.transactionRollback.txn_id);
  } else if ("transactionUndo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionUndo);
    writeVarintU64(out, cmd.transactionUndo.seq);
    writeStr(out, cmd.transactionUndo.group_id);
  } else if ("transactionRedo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRedo);
    writeVarintU64(out, cmd.transactionRedo.seq);
    writeStr(out, cmd.transactionRedo.group_id);
  } else if ("openArtifact" in cmd) {
    out.push(APP_COMMAND_TAGS.openArtifact);
    writeVarintU64(out, cmd.openArtifact.seq);
    writeStr(out, cmd.openArtifact.artifact_ref);
    out.push(cmd.openArtifact.role);
    writeStr(out, cmd.openArtifact.plugin_id);
    writeStr(out, cmd.openArtifact.app_id);
  } else if ("setDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.setDefaultApp);
    writeVarintU64(out, cmd.setDefaultApp.seq);
    writeStr(out, cmd.setDefaultApp.artifact_kind);
    writeStr(out, cmd.setDefaultApp.standard);
    writeStr(out, cmd.setDefaultApp.subset);
    out.push(cmd.setDefaultApp.role);
    writeStr(out, cmd.setDefaultApp.plugin_id);
    writeStr(out, cmd.setDefaultApp.app_id);
  } else if ("clearDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.clearDefaultApp);
    writeVarintU64(out, cmd.clearDefaultApp.seq);
    writeStr(out, cmd.clearDefaultApp.artifact_kind);
    writeStr(out, cmd.clearDefaultApp.standard);
    writeStr(out, cmd.clearDefaultApp.subset);
    out.push(cmd.clearDefaultApp.role);
  } else if ("setMergePolicy" in cmd) {
    out.push(APP_COMMAND_TAGS.setMergePolicy);
    writeVarintU64(out, cmd.setMergePolicy.seq);
    out.push(cmd.setMergePolicy.policy);
  } else if ("resolveConflict" in cmd) {
    out.push(APP_COMMAND_TAGS.resolveConflict);
    writeVarintU64(out, cmd.resolveConflict.seq);
    writeStr(out, cmd.resolveConflict.conflict_id);
    out.push(cmd.resolveConflict.resolution);
  } else if ("readConflicts" in cmd) {
    out.push(APP_COMMAND_TAGS.readConflicts);
    writeVarintU64(out, cmd.readConflicts.seq);
  } else if ("LocalInteractionQuery" in cmd) {
    out.push(APP_COMMAND_TAGS.LocalInteractionQuery);
    writeVarintU64(out, cmd.LocalInteractionQuery.seq);
    writeBytes(out, Array.from(encodeLocalInteractionQueryCommand(cmd.LocalInteractionQuery.command)));
  } else if ("presence" in cmd) {
    out.push(APP_COMMAND_TAGS.presence);
    writeVarintU64(out, cmd.presence.seq);
    writeOptU8(out, cmd.presence.own_color);
    writeVecBytes(out, cmd.presence.peers);
  } else {
    throw new Error("encodeAppCommand: unrecognized command variant");
  }
  return new Uint8Array(out);
}
function decodeAppFrame(bytes) {
  if (bytes.length === 0)
    throw new Error("decodeAppFrame: empty frame");
  const pos = [1];
  switch (bytes[0]) {
    case APP_FRAME_TAGS.Done:
      return { Done: { in_reply_to: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.Invocation: {
      const in_reply_to = readVarintU64(bytes, pos);
      const output = readBytes(bytes, pos);
      const diagnostics = readBytes(bytes, pos);
      const ui_scope = readBytes(bytes, pos);
      const history_patch = readBytes(bytes, pos);
      const messages = readBytes(bytes, pos);
      return { Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch, messages } };
    }
    case APP_FRAME_TAGS.DocumentChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { DocumentChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.Document: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Document: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.Config: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Config: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.ConfigChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { ConfigChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.ContextMenu:
      return { ContextMenu: { in_reply_to: readVarintU64(bytes, pos), items: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Media: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data2 = readBytes(bytes, pos);
      return { Media: { in_reply_to, port, descriptor, data: data2 } };
    }
    case APP_FRAME_TAGS.MediaFingerprint: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const fingerprint = readBytes(bytes, pos);
      return { MediaFingerprint: { in_reply_to, port, fingerprint } };
    }
    case APP_FRAME_TAGS.Error: {
      const in_reply_to = readOptU64(bytes, pos);
      const fault = readBytes(bytes, pos);
      const report = readBytes(bytes, pos);
      return { Error: { in_reply_to, fault, report } };
    }
    case APP_FRAME_TAGS.Emit:
      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Draft:
      return { Draft: { in_reply_to: readVarintU64(bytes, pos), pack: readBytes(bytes, pos), spr: readBytes(bytes, pos), ops: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.Children:
      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.Ephemeral:
      return {
        Ephemeral: { presence: readBytes(bytes, pos), presence_generation: readVarintU64(bytes, pos), transient_generation: readVarintU64(bytes, pos), interaction: readBytes(bytes, pos) }
      };
    case APP_FRAME_TAGS.HistorySnapshot:
      return { HistorySnapshot: { in_reply_to: readVarintU64(bytes, pos), history_patch: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.transactionProposal: {
      const in_reply_to = readVarintU64(bytes, pos);
      const proposal_id = readStr(bytes, pos);
      const local_ops = readVecBytes(bytes, pos);
      const description = readStr(bytes, pos);
      const coalesce_key = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      return { transactionProposal: { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } };
    }
    case APP_FRAME_TAGS.transactionPrepared: {
      const txn_id = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      const rejection = readBytes(bytes, pos);
      return { transactionPrepared: { txn_id, foreign, rejection } };
    }
    case APP_FRAME_TAGS.transactionCommitted:
      return { transactionCommitted: { txn_id: readStr(bytes, pos), edit_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.transactionRolledBack:
      return { transactionRolledBack: { txn_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.MergeReport:
      return { MergeReport: { in_reply_to: readOptU64(bytes, pos), report: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Conflicts:
      return { Conflicts: { in_reply_to: readOptU64(bytes, pos), conflicts: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.UiPatch: {
      const in_reply_to = readOptU64(bytes, pos);
      const surface = readStr(bytes, pos);
      const kind = readStr(bytes, pos);
      const revision = readVarintU64(bytes, pos);
      const base_revision = readVarintU64(bytes, pos);
      const ops = readBytes(bytes, pos);
      return { UiPatch: { in_reply_to, surface, kind, revision, base_revision, ops } };
    }
    case APP_FRAME_TAGS.UiSnapshotEnd:
      return { UiSnapshotEnd: { revision: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.LocalInteractionQuery: {
      const length = readVarintU64(bytes, pos);
      if (length > 4256 || pos[0] + length !== bytes.length)
        throw new Error("local-interaction.reply-envelope");
      return { LocalInteractionQuery: { reply: decodeLocalInteractionQueryReply(bytes.subarray(pos[0])) } };
    }
    default:
      throw new Error(`decodeAppFrame: unknown tag ${bytes[0]}`);
  }
}
function decodeFaultFromWire(faultBytes, decodePackValue2) {
  try {
    const raw = decodePackValue2(new Uint8Array(faultBytes));
    if (!raw || typeof raw !== "object" || !("message" in raw))
      return null;
    return raw;
  } catch {
    return null;
  }
}
function faultDisplayMessage(faultBytes, decodePackValue2) {
  const fault = decodeFaultFromWire(faultBytes, decodePackValue2);
  if (!fault)
    return "unknown fault";
  const code = typeof fault.code === "string" ? fault.code : String(fault.code);
  return `${code}: ${fault.message}`;
}
function sameLocalInteractionQuery(left, right) {
  return left.requestId === right.requestId && left.queryGeneration === right.queryGeneration && localInteractionIdentityEquals(left.identity, right.identity);
}
function appChannelTransactionReply(command) {
  if ("transactionPrepare" in command)
    return { kind: "prepared", id: command.transactionPrepare.txn_id };
  if ("transactionCommit" in command)
    return { kind: "committed", id: command.transactionCommit.txn_id };
  if ("transactionRollback" in command)
    return { kind: "rolledBack", id: command.transactionRollback.txn_id };
  return null;
}
function appChannelReplySequence(frame) {
  const value = Object.values(frame)[0];
  return value && "in_reply_to" in value && typeof value.in_reply_to === "number" ? value.in_reply_to : null;
}
function appChannelFrameBelongsTo(frame, sequence, transaction) {
  const replySequence = appChannelReplySequence(frame);
  if (replySequence !== null)
    return replySequence === sequence;
  if ("transactionPrepared" in frame)
    return transaction?.kind === "prepared" && transaction.id === frame.transactionPrepared.txn_id;
  if ("transactionCommitted" in frame)
    return transaction?.kind === "committed" && transaction.id === frame.transactionCommitted.txn_id;
  if ("transactionRolledBack" in frame)
    return transaction?.kind === "rolledBack" && transaction.id === frame.transactionRolledBack.txn_id;
  return true;
}

class AppChannelRequestSequence {
  sequence;
  request;
  constructor(sequence = 0, request = 0n) {
    this.sequence = sequence;
    this.request = request;
    if (!Number.isSafeInteger(sequence) || sequence < 0 || request < 0n || request > 0xffffffffffffffffn)
      throw new Error("app-channel.invalid-sequence-owner");
  }
  nextSequence() {
    if (this.sequence === Number.MAX_SAFE_INTEGER)
      throw new Error("app-channel.sequence-exhausted");
    return ++this.sequence;
  }
  nextQuery() {
    if (this.sequence > Number.MAX_SAFE_INTEGER - 2)
      throw new Error("app-channel.sequence-exhausted");
    if (this.request === 0xffffffffffffffffn)
      throw new Error("local-interaction.request-exhausted");
    const sequence = this.sequence + 1;
    this.sequence += 2;
    return { sequence, cancelSequence: this.sequence, request: (++this.request).toString() };
  }
  checkpoint() {
    return { sequence: this.sequence, request: this.request.toString() };
  }
}

class AppChannelClient {
  sequenceOwner;
  localQuery = null;
  disposed = false;
  handle;
  instanceId;
  appId;
  actor;
  outcomeIterator;
  pending = [];
  cachedPack = null;
  cachedSpr = null;
  constructor(handle, sequenceOwner, instanceId, appId, actor = "local") {
    this.sequenceOwner = sequenceOwner;
    this.handle = handle;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
    this.pumpOutcomes();
  }
  async pumpOutcomes() {
    for (;; ) {
      const step13 = await this.outcomeIterator.next();
      if (step13.done) {
        this.finishLocalInteractionQuery(new Error("local-interaction.channel-closed"));
        for (const waiter of this.pending.splice(0))
          waiter.reject(new Error("app-channel.closed"));
        return;
      }
      const outcome = step13.value;
      if (outcome.instanceId !== this.instanceId)
        continue;
      if ("error" in outcome) {
        this.finishLocalInteractionQuery(outcome.error);
        this.pending.shift()?.reject(outcome.error);
        continue;
      }
      const frames = [];
      for (const encoded of outcome.frames) {
        try {
          frames.push(decodeAppFrame(encoded));
        } catch (error) {
          this.cancelLocalInteractionQuery(error);
          for (let index = this.pending.length - 1;index >= 0; index -= 1) {
            if (!this.pending[index].queryReceipt)
              this.pending.splice(index, 1)[0].reject(error);
          }
        }
      }
      const ordinary = [];
      for (const frame of frames) {
        if ("LocalInteractionQuery" in frame)
          this.receiveLocalInteractionQuery(frame.LocalInteractionQuery.reply);
        else
          ordinary.push(frame);
      }
      const correlated = new Set(ordinary.flatMap((frame) => {
        const sequence = appChannelReplySequence(frame);
        return sequence === null ? [] : [sequence];
      }));
      for (let index = 0;index < this.pending.length; ) {
        const waiter = this.pending[index];
        if (!correlated.has(waiter.seq)) {
          index += 1;
          continue;
        }
        this.pending.splice(index, 1);
        const reply = ordinary.filter((frame) => appChannelFrameBelongsTo(frame, waiter.seq, waiter.transaction));
        this.captureDocumentFrames(reply);
        waiter.resolve(reply);
      }
      this.finishDisposal();
    }
  }
  dispose() {
    this.disposed = true;
    for (let index = this.pending.length - 1;index >= 0; index -= 1) {
      if (!this.pending[index].queryReceipt)
        this.pending.splice(index, 1)[0].reject(new Error("app-channel.disposed"));
    }
    if (this.localQuery)
      this.cancelLocalInteractionQuery(new Error("local-interaction.disposed"));
    this.finishDisposal();
  }
  finishDisposal() {
    if (this.disposed && this.localQuery === null && this.pending.length === 0)
      this.outcomeIterator.return?.();
  }
  nextSeq() {
    return this.sequenceOwner.nextSequence();
  }
  captureDocumentFrames(frames) {
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }
  documentPack() {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack, spr: this.cachedSpr } : null;
  }
  sendCommand(command) {
    if (this.disposed)
      return Promise.reject(new Error("app-channel.disposed"));
    return new Promise((resolve, reject) => {
      const seq = Object.values(command)[0].seq;
      this.pending.push({ seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject });
      this.handle.enqueue(this.instanceId, [encodeAppCommand(command)]);
    });
  }
  readLocalInteractionPages(consume, signal) {
    if (this.disposed)
      return Promise.reject(new Error("app-channel.disposed"));
    if (this.localQuery)
      return Promise.reject(new Error("local-interaction.busy"));
    if (signal?.aborted)
      return Promise.reject(new Error("local-interaction.cancelled"));
    let admission2;
    try {
      admission2 = this.sequenceOwner.nextQuery();
    } catch (error) {
      return Promise.reject(error);
    }
    const requestId = admission2.request;
    return new Promise((resolve, reject) => {
      const abort = () => this.cancelLocalInteractionQuery(new Error("local-interaction.cancelled"));
      this.localQuery = { requestId, cancelSequence: admission2.cancelSequence, consume, resolve, reject, signal, abort, token: null, nextOrdinal: 0n, consuming: false, cancelled: false, terminalConsumed: false, failure: null };
      signal?.addEventListener("abort", abort, { once: true });
      this.sendLocalInteractionQuery(admission2.sequence, { kind: "read", requestId });
    });
  }
  sendLocalInteractionQuery(seq, command) {
    this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {}, reject: (error) => this.finishLocalInteractionQuery(error) });
    try {
      this.handle.enqueue(this.instanceId, [encodeAppCommand({ LocalInteractionQuery: { seq, command } })]);
    } catch (error) {
      const index = this.pending.findIndex((waiter) => waiter.seq === seq);
      if (index !== -1)
        this.pending.splice(index, 1);
      this.finishLocalInteractionQuery(error);
    }
  }
  cancelLocalInteractionQuery(error) {
    const query = this.localQuery;
    if (!query || query.cancelled)
      return;
    query.cancelled = true;
    query.failure = error;
    if (query.token)
      this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
  }
  finishLocalInteractionQuery(error = null) {
    const query = this.localQuery;
    if (!query)
      return;
    this.localQuery = null;
    for (let index = this.pending.length - 1;index >= 0; index -= 1) {
      if (this.pending[index].queryReceipt)
        this.pending.splice(index, 1);
    }
    query.signal?.removeEventListener("abort", query.abort);
    const failure = query.failure ?? error;
    if (failure !== null)
      query.reject(failure);
    else if (query.token && query.terminalConsumed)
      query.resolve(query.token.identity);
    else
      query.reject(new Error("local-interaction.incomplete-close"));
    this.finishDisposal();
  }
  receiveLocalInteractionQuery(reply) {
    const query = this.localQuery;
    if (!query)
      return;
    if (reply.kind === "rejected") {
      if (reply.requestId === query.requestId)
        this.finishLocalInteractionQuery(new Error(`local-interaction.${reply.code}`));
      return;
    }
    if (reply.kind === "started") {
      if (reply.token.requestId !== query.requestId || reply.token.identity.appInstanceId !== this.instanceId || query.token !== null)
        return;
      query.token = reply.token;
      if (query.cancelled)
        this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
      return;
    }
    const token2 = reply.kind === "page" ? reply.page : reply.token;
    if (!query.token || !sameLocalInteractionQuery(query.token, token2))
      return;
    if (reply.kind === "closed") {
      if (!query.cancelled && (!query.terminalConsumed || token2.ordinal !== query.token.ordinal))
        return;
      this.finishLocalInteractionQuery(reply.cancelled ? new Error("local-interaction.cancelled") : null);
      return;
    }
    if (query.cancelled || query.consuming || token2.ordinal !== query.nextOrdinal.toString())
      return;
    query.token = { requestId: token2.requestId, queryGeneration: token2.queryGeneration, identity: token2.identity, ordinal: token2.ordinal };
    query.consuming = true;
    Promise.resolve().then(() => query.consume(reply.page)).then(() => {
      if (this.localQuery !== query)
        return;
      query.consuming = false;
      if (query.cancelled)
        return;
      query.terminalConsumed = reply.page.terminal;
      query.nextOrdinal += 1n;
      let seq;
      try {
        seq = this.nextSeq();
      } catch (error) {
        this.cancelLocalInteractionQuery(error);
        return;
      }
      this.sendLocalInteractionQuery(seq, { kind: "acknowledge", token: query.token });
    }, (error) => {
      if (this.localQuery !== query)
        return;
      query.consuming = false;
      this.cancelLocalInteractionQuery(error);
    });
  }
  async command(commandBytes, viewState) {
    return this.sendCommand({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewState)) }
    });
  }
  async configure(config) {
    return this.sendCommand({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }
  async readDocument() {
    return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } });
  }
  async loadDocument(pack, spr) {
    this.cachedPack = pack;
    this.cachedSpr = spr;
    return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });
  }
  async readHistory() {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }
  async openArtifact(artifactRef, role, pluginId = "", appId = "") {
    return this.sendCommand({ openArtifact: { seq: this.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }
  async setDefaultApp(artifactKind, standard, subset, role, pluginId, appId) {
    return this.sendCommand({ setDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }
  async clearDefaultApp(artifactKind, standard, subset, role) {
    return this.sendCommand({ clearDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }
  async contextMenu(request) {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) }
    });
    const errorFrame = frames.find((frame) => ("Error" in frame));
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    }
    const menuFrame = frames.find((frame) => ("ContextMenu" in frame) && frame.ContextMenu.in_reply_to === seq);
    if (!menuFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }
  async applyEnvelopes(envelopes) {
    return this.sendCommand({ ApplyEnvelopes: { seq: this.nextSeq(), envelopes } });
  }
  async setMergePolicy(policy) {
    return this.sendCommand({ setMergePolicy: { seq: this.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }
  async resolveConflict(conflictId, resolution) {
    return this.sendCommand({ resolveConflict: { seq: this.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }
  async readConflicts() {
    return this.sendCommand({ readConflicts: { seq: this.nextSeq() } });
  }
  async pushPresence(ownColor, peers) {
    return this.sendCommand({ presence: { seq: this.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  async transactionPrepareOwner(txnId, mutationId, payload) {
    return this.sendCommand({
      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], label: "", origin: [] }
    });
  }
  async transactionPreparePlanned(txnId, preparedOps, label, origin) {
    return this.sendCommand({
      transactionPrepare: {
        seq: this.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        label,
        origin: Array.from(origin)
      }
    });
  }
  async transactionCommit(txnId) {
    return this.sendCommand({ transactionCommit: { seq: this.nextSeq(), txn_id: txnId } });
  }
  async transactionRollback(txnId) {
    return this.sendCommand({ transactionRollback: { seq: this.nextSeq(), txn_id: txnId } });
  }
  async transactionUndo(groupId) {
    return this.sendCommand({ transactionUndo: { seq: this.nextSeq(), group_id: groupId } });
  }
  async transactionRedo(groupId) {
    return this.sendCommand({ transactionRedo: { seq: this.nextSeq(), group_id: groupId } });
  }
}
if (undefined) {}
if (undefined) {}
if (undefined) {
  let sampleDirectoryEvent = function(seq) {};
}
/* ../../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts */
var SHARD_WORKER_URL = "/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js";
var DEFAULT_SHARD_BUDGET = { fuel: 50000000, wallMs: 100, memoryBytes: 256 * 1024 * 1024, uiNodes: 20000, mailboxLen: 64, maxEffects: 64, maxPatchBytes: 1 << 20 };
function poolConcurrency() {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}
function buildShardClientOptions(overrides) {
  return {
    shardCount: poolConcurrency(),
    createWorker: () => new Worker(SHARD_WORKER_URL, { type: "module" }),
    ...overrides
  };
}
function createPooledActorRuntime(options) {
  const shardClient = new ShardClient(buildShardClientOptions(options));
  shardClient.startWatchdog();
  return { shardClient };
}

/* ../../../../💾️resident/🟦️.ts */
var ledger = new OwnedResidentLedger({ bytes: 33554432, slots: 262144, owners: 262144, control: { bytes: 65536, slots: 1024, owners: 1024 } });
function rendererResidentLedger() {
  return ledger;
}

/* ../../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts */
function coerceWireBytes(raw) {
  if (raw instanceof Uint8Array)
    return raw;
  if (ArrayBuffer.isView(raw) && Object.prototype.toString.call(raw) === "[object Uint8Array]")
    return new Uint8Array(raw.buffer, raw.byteOffset, raw.byteLength);
  if (Array.isArray(raw))
    return Uint8Array.from(raw);
  if (raw && typeof raw === "object") {
    const record = raw;
    if (record.kind === "bytes" && Array.isArray(record.value))
      return Uint8Array.from(record.value);
    if (Array.isArray(record.data))
      return Uint8Array.from(record.data);
  }
  if (typeof raw === "string") {
    const binary = atob(raw);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0;i < binary.length; i++)
      bytes[i] = binary.charCodeAt(i);
    return bytes;
  }
  throw new Error(`[DEBUG] coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.slice(0, 120)}`);
}
function coerceTurnResult(raw) {
  const record = raw && typeof raw === "object" ? raw : {};
  const uiPatches = Array.isArray(record.uiPatches) ? record.uiPatches : [];
  const effects = Array.isArray(record.effects) ? record.effects : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  const commandIngress = record.commandIngress && typeof record.commandIngress === "object" ? record.commandIngress : undefined;
  return { uiPatches, effects, nextWake, commandIngress };
}
function shellFrameBytes(effect, instanceId) {
  if (effect.tag !== "send-message")
    return null;
  const val = effect.val ?? {};
  if (!val.target || val.target.tag !== "shell")
    return null;
  if (Number(val.target.val) !== instanceId)
    return null;
  if (val.payload === undefined)
    return null;
  return coerceWireBytes(val.payload);
}
function decodeWirePatchOps(ops, decodePackValue2) {
  const decoded = [];
  for (const op of ops) {
    const val = op.val ?? {};
    const path = Array.isArray(val.path) ? val.path : [];
    switch (op.tag) {
      case "replace":
        decoded.push({ kind: "Replace", path, node: decodePackValue2(coerceWireBytes(val.node)) });
        break;
      case "insert-child":
        decoded.push({ kind: "InsertChild", path, index: Number(val.index ?? 0), node: decodePackValue2(coerceWireBytes(val.node)) });
        break;
      case "remove-child":
        decoded.push({ kind: "RemoveChild", path, index: Number(val.index ?? 0) });
        break;
      case "set-props":
        decoded.push({ kind: "SetProps", path, props: val.props !== undefined ? decodePackValue2(coerceWireBytes(val.props)) : undefined });
        break;
      default:
        break;
    }
  }
  return decoded;
}
function applyUiPatchToRetained(previous, patch) {
  let node = previous?.node ?? null;
  let sawFullReplace = false;
  for (const op of patch.ops) {
    if (op.kind === "Replace" && op.path.length === 0) {
      node = op.node;
      sawFullReplace = true;
    } else {
      return { surface: previous, desynced: true };
    }
  }
  if (!sawFullReplace && previous && patch.baseRevision !== previous.revision)
    return { surface: previous, desynced: true };
  return { surface: node !== null ? { revision: patch.revision, node } : previous, desynced: false };
}
function wireExtensionInvocation(effect) {
  const value = effect.val;
  const req = value?.req;
  if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn)
    throw new Error("extension.request-id-invalid");
  const params = value?.params;
  if (typeof params?.extensionId !== "string" || !params.extensionId || typeof params.capability !== "string" || !params.capability)
    throw new Error("extension.request-address-invalid");
  const requestJson = new TextDecoder("utf-8", { fatal: true }).decode(coerceWireBytes(params.payload));
  return { invokeExtension: { req, extensionId: params.extensionId, capability: params.capability, requestJson } };
}
function wireEffectToFriendly(effect, decodePackValue2) {
  const val = effect.val ?? {};
  const str = (key) => String(val[key] ?? "");
  const num = (key) => Number(val[key] ?? 0);
  const packField = (key) => val[key] !== undefined ? decodePackValue2(coerceWireBytes(val[key])) : undefined;
  switch (effect.tag) {
    case "invoke-extension":
      return wireExtensionInvocation(effect);
    case "request-sync":
      return "requestSync";
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: str("kind"), params: packField("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "spawn-plugin-instance":
      return { spawnPluginInstance: { req: num("req"), pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId, label: val.label, documentJson: val.documentJson } };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId } };
    default:
      console.warn(`[DEBUG] wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion`);
      return null;
  }
}

/* 🟦️typescript/🐚️plugin-bridge.ts */
var pooledRuntime = null;
function getShardClient() {
  pooledRuntime ??= createPooledActorRuntime({
    residentLedger: rendererResidentLedger(),
    onActorTrap: (actorId, message) => console.error(`[DEBUG] wgpu plugin-bridge: actor ${actorId} trapped: ${message}`),
    onShardLost: (shardIndex, actorIds) => {
      console.error(`[DEBUG] wgpu plugin-bridge: shard ${shardIndex} lost, restoring actors: ${actorIds.join(", ")}`);
      getActivationRegistry().handleShardLost(shardIndex, actorIds);
    }
  });
  return pooledRuntime.shardClient;
}
var sharedActivationRegistry = null;
function getActivationRegistry() {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}
var actorTurnChains = new Map;
function submitTurn(actorId, events, commandPage) {
  getActivationRegistry().touch(actorId);
  const previousSettled = (actorTurnChains.get(actorId) ?? Promise.resolve()).catch(() => {
    return;
  });
  const next = previousSettled.then(() => getShardClient().turn(actorId, events, DEFAULT_SHARD_BUDGET, commandPage));
  actorTurnChains.set(actorId, next);
  return next.then(coerceTurnResult);
}
var retainedWindowByActor = new Map;
function applyRetainedWindowPatches(actorId, uiPatches) {
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? [], decodePackValue);
    const previous = retainedWindowByActor.get(actorId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      console.warn(`[DEBUG] plugin-bridge: actor ${actorId} desynced (unrecognized op shape or stale baseRevision) — keeping the previously retained body`);
      continue;
    }
    if (surface)
      retainedWindowByActor.set(actorId, surface);
  }
}
async function performRender(actorId, instanceId, bodyKey) {
  const result3 = await submitTurn(actorId, [{ kind: "surface-visible", payload: { surface: { instance: instanceId, surface: bodyKey } } }]);
  if (result3.uiPatches.length > 0)
    applyRetainedWindowPatches(actorId, result3.uiPatches);
  return retainedWindowByActor.get(actorId)?.node ?? null;
}
var pendingTurnEffects = new Map;
var nextGlobalInstanceId = 1;
async function performInvocation(client, instanceId, invocation, viewState) {
  const frames = await client.command(encodePackValue(invocation), viewState);
  let output = null;
  let diagnostics = [];
  let uiScope;
  let historyPatch;
  for (const frame of frames) {
    if ("Invocation" in frame) {
      output = decodePackValue(new Uint8Array(frame.Invocation.output));
      const decodedDiagnostics = decodePackValue(new Uint8Array(frame.Invocation.diagnostics));
      diagnostics = Array.isArray(decodedDiagnostics) ? decodedDiagnostics : [];
      uiScope = decodePackValue(new Uint8Array(frame.Invocation.ui_scope));
      const decodedHistoryPatch = decodePackValue(new Uint8Array(frame.Invocation.history_patch));
      historyPatch = decodedHistoryPatch && typeof decodedHistoryPatch === "object" ? decodedHistoryPatch : undefined;
    } else if ("Error" in frame) {
      const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
      if (fault)
        throw new SemioFaultError(fault);
      throw new Error(`invocation failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
    }
  }
  const leftover = pendingTurnEffects.get(instanceId) ?? [];
  pendingTurnEffects.delete(instanceId);
  const requestedEffects = leftover.map((effect) => wireEffectToFriendly(effect, decodePackValue)).filter((effect) => effect !== null);
  return { output, mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] }, diagnostics, requestedEffects, events: [], uiScope, historyPatch };
}
async function loadPluginModule(pluginId, moduleUrl, signal) {
  const manifest = await fetchDescriptorManifest(pluginId, moduleUrl, signal);
  const registry = getActivationRegistry();
  registry.registerManifest({ pluginId, moduleUrl, caps: [] });
  const shardClient = getShardClient();
  const actorIdByInstance = new Map;
  const channelByInstance = new Map;
  const channelRequests = new AppChannelRequestSequence;
  let eventSeq = 0;
  const requireActorId = (instanceId) => {
    const actorId = actorIdByInstance.get(instanceId);
    if (!actorId)
      throw new Error(`[DEBUG] program ${pluginId}: no actor for instance ${instanceId} (createApp not called, or already destroyed)`);
    return actorId;
  };
  const requireChannel = (instanceId) => {
    const client = channelByInstance.get(instanceId);
    if (!client)
      throw new Error(`[DEBUG] program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
    return client;
  };
  const turnOutcomes = createTurnOutcomeBroadcast();
  const runQueuedTurn = async (instanceId, events) => {
    try {
      const actorId = requireActorId(instanceId);
      const results = [];
      for (let commandIndex = 0;commandIndex < events.length; commandIndex += 1) {
        eventSeq += 1;
        const pages = createShardCommandIngressPages({
          owner: BigInt(instanceId),
          generation: 1n,
          commandIndex,
          commandCount: events.length,
          instance: instanceId,
          seq: BigInt(eventSeq),
          command: events[commandIndex]
        });
        for (const commandPage of pages)
          results.push(await submitTurn(actorId, [], commandPage));
        let terminal = results.at(-1)?.commandIngress?.tag;
        for (let continuation = 0;terminal !== "command-complete" && continuation < 1024; continuation += 1) {
          if (terminal === "fault")
            throw new Error(`[DEBUG] plugin ${pluginId}: command ingress fault`);
          if (terminal === "backpressure")
            throw new Error(`[DEBUG] plugin ${pluginId}: command ingress backpressure after serialized submission`);
          const continued = await submitTurn(actorId, []);
          results.push(continued);
          terminal = continued.commandIngress?.tag;
        }
        if (terminal !== "command-complete")
          throw new Error(`[DEBUG] plugin ${pluginId}: command ingress did not complete within 1024 continuations`);
      }
      const result3 = {
        uiPatches: results.flatMap((turn) => turn.uiPatches),
        effects: results.flatMap((turn) => turn.effects),
        nextWake: [...results].reverse().find((turn) => turn.nextWake !== null)?.nextWake ?? null,
        commandIngress: results.at(-1)?.commandIngress
      };
      const outFrames = [];
      const leftover = [];
      for (const effect of result3.effects) {
        const frame = shellFrameBytes(effect, instanceId);
        if (frame)
          outFrames.push(frame);
        else
          leftover.push(effect);
      }
      pendingTurnEffects.set(instanceId, leftover);
      if (result3.uiPatches.length > 0)
        applyRetainedWindowPatches(actorId, result3.uiPatches);
      turnOutcomes.push({ instanceId, frames: outFrames });
    } catch (error) {
      turnOutcomes.push({ instanceId, error });
    }
  };
  const channelHandle = {
    enqueue: (instanceId, events) => {
      runQueuedTurn(instanceId, events);
    },
    outcomes: turnOutcomes.stream
  };
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      const instanceId = nextGlobalInstanceId;
      nextGlobalInstanceId += 1;
      const actorId = `${pluginId}#${instanceId}`;
      actorIdByInstance.set(instanceId, actorId);
      await registry.activate(pluginId, actorId, "manual");
      eventSeq += 1;
      await submitTurn(actorId, [{ kind: "instance-open", payload: { instance: instanceId, appId, actor: "local", config: [], assets: [], capabilities: [], quotas: Array.from(encodePackValue({})) } }]);
      channelByInstance.set(instanceId, new AppChannelClient(channelHandle, channelRequests, instanceId, appId, "local"));
      return instanceId;
    },
    destroyApp: async (instanceId) => {
      const actorId = actorIdByInstance.get(instanceId);
      if (!actorId)
        return;
      channelByInstance.get(instanceId)?.dispose();
      actorIdByInstance.delete(instanceId);
      channelByInstance.delete(instanceId);
      retainedWindowByActor.delete(actorId);
      pendingTurnEffects.delete(instanceId);
      shardClient.dispose(actorId);
    },
    handleAction: (instanceId, actionJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), viewState),
    handleCommand: (instanceId, commandJson, viewState) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), viewState),
    render: (instanceId, bodyKey) => performRender(requireActorId(instanceId), instanceId, bodyKey),
    contextMenu: (instanceId, request) => requireChannel(instanceId).contextMenu(request),
    dispose: () => {
      for (const instanceId of channelByInstance.keys())
        channelByInstance.get(instanceId)?.dispose();
      for (const actorId of actorIdByInstance.values()) {
        retainedWindowByActor.delete(actorId);
        shardClient.dispose(actorId);
      }
      actorIdByInstance.clear();
      channelByInstance.clear();
      turnOutcomes.complete();
    }
  };
}
function viewStateFromContextJson(contextJson) {
  try {
    const parsed = JSON.parse(contextJson);
    return parsed && typeof parsed === "object" && "viewState" in parsed ? parsed.viewState : parsed;
  } catch {
    return;
  }
}
function pluginHandleForBridge(handle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, contextJson) => handle.handleAction(instanceId, actionJson, viewStateFromContextJson(contextJson)).then((result3) => JSON.stringify(result3)),
    handleCommand: (instanceId, commandJson, contextJson) => handle.handleCommand(instanceId, commandJson, viewStateFromContextJson(contextJson)).then((result3) => JSON.stringify(result3)),
    render: (instanceId, bodyKey, viewStateJson) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items))
  };
}
/* ../../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json */
var _catalog_default3 = {
  $schema: "./🧬️catalog.schema.json",
  version: 1,
  collections: [
    {
      catalog: "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json",
      root: "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation",
      output: "🌱️metabolism"
    }
  ],
  entries: [
    {
      url: "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb",
      source: "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb",
      path: "🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb"
    },
    {
      url: "/mesh/🧊️hexagonal-cut-concrete-forest-right.glb",
      source: "♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/▶️hexagonal-cut-concrete-forest-right.glb",
      path: "🏚️abbau-aufbau/👉️hexagonal-cut-concrete-forest-right.glb"
    },
    {
      url: "/mesh/🧊️placeholder.glb",
      source: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧊️placeholder.glb",
      path: "🥽️mesh/🧊️placeholder.glb"
    }
  ]
};
/* ../../../../../../../../../🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json */
var _catalog_default4 = {
  $schema: "./🧬️catalog.schema.json",
  version: 1,
  entries: [
    { url: "/mesh/🧊️base.glb", path: "🧱️bases/⬛️base/🧊️base.glb" },
    { url: "/mesh/🧊️base_collider.glb", path: "🧱️bases/⬛️base/💥️base_collider.glb" },
    { url: "/mesh/🧊️base_blob.glb", path: "🧱️bases/🫧️blob/🧊️base_blob.glb" },
    { url: "/mesh/🧊️base_blob_collider.glb", path: "🧱️bases/🫧️blob/💥️base_blob_collider.glb" },
    { url: "/mesh/🧊️bridge.glb", path: "🌉️bridge/🧊️bridge.glb" },
    { url: "/mesh/🧊️bridge_collider.glb", path: "🌉️bridge/💥️bridge_collider.glb" },
    { url: "/mesh/🧊️capital.glb", path: "🏛️capitals/▫️square/🧊️capital.glb" },
    { url: "/mesh/🧊️capital_collider.glb", path: "🏛️capitals/▫️square/💥️capital_collider.glb" },
    { url: "/mesh/🧊️cylindric-capital.glb", path: "🏛️capitals/🔘️cylindric/🧊️cylindric-capital.glb" },
    { url: "/mesh/🧊️cylindric-capital_collider.glb", path: "🏛️capitals/🔘️cylindric/💥️cylindric-capital_collider.glb" },
    { url: "/mesh/🧊️capsule_J.glb", path: "💊️capsules/🪝️j/🧊️capsule_J.glb" },
    { url: "/mesh/🧊️capsule_J_collider.glb", path: "💊️capsules/🪝️j/💥️capsule_J_collider.glb" },
    { url: "/mesh/🧊️capsule_L.glb", path: "💊️capsules/📐️l/🧊️capsule_L.glb" },
    { url: "/mesh/🧊️capsule_L_collider.glb", path: "💊️capsules/📐️l/💥️capsule_L_collider.glb" },
    { url: "/mesh/🧊️capsule_p.glb", path: "💊️capsules/🅿️p/🧊️capsule_p.glb" },
    { url: "/mesh/🧊️capsule_p_collider.glb", path: "💊️capsules/🅿️p/💥️capsule_p_collider.glb" },
    { url: "/mesh/🧊️capsule_q.glb", path: "💊️capsules/🔎️q/🧊️capsule_q.glb" },
    { url: "/mesh/🧊️capsule_q_collider.glb", path: "💊️capsules/🔎️q/💥️capsule_q_collider.glb" },
    { url: "/mesh/🧊️capsule_s.glb", path: "💊️capsules/🐍️s/🧊️capsule_s.glb" },
    { url: "/mesh/🧊️capsule_s_collider.glb", path: "💊️capsules/🐍️s/💥️capsule_s_collider.glb" },
    { url: "/mesh/🧊️capsule_z.glb", path: "💊️capsules/⚡️z/🧊️capsule_z.glb" },
    { url: "/mesh/🧊️capsule_z_collider.glb", path: "💊️capsules/⚡️z/💥️capsule_z_collider.glb" },
    { url: "/mesh/🧊️capsule_slash.glb", path: "💊️capsules/↗️slash/🧊️capsule_slash.glb" },
    { url: "/mesh/🧊️capsule_slash_collider.glb", path: "💊️capsules/↗️slash/💥️capsule_slash_collider.glb" },
    { url: "/mesh/🧊️capsule_backslash.glb", path: "💊️capsules/↘️backslash/🧊️capsule_backslash.glb" },
    { url: "/mesh/🧊️capsule_backslash_collider.glb", path: "💊️capsules/↘️backslash/💥️capsule_backslash_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_J.glb", path: "🏞️balconies/🪝️j/🧊️capsule-with-balcony_J.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_J_collider.glb", path: "🏞️balconies/🪝️j/💥️capsule-with-balcony_J_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_L.glb", path: "🏞️balconies/📐️l/🧊️capsule-with-balcony_L.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_L_collider.glb", path: "🏞️balconies/📐️l/💥️capsule-with-balcony_L_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_p.glb", path: "🏞️balconies/🅿️p/🧊️capsule-with-balcony_p.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_p_collider.glb", path: "🏞️balconies/🅿️p/💥️capsule-with-balcony_p_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_q.glb", path: "🏞️balconies/🔎️q/🧊️capsule-with-balcony_q.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_q_collider.glb", path: "🏞️balconies/🔎️q/💥️capsule-with-balcony_q_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_s.glb", path: "🏞️balconies/🐍️s/🧊️capsule-with-balcony_s.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_s_collider.glb", path: "🏞️balconies/🐍️s/💥️capsule-with-balcony_s_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_z.glb", path: "🏞️balconies/⚡️z/🧊️capsule-with-balcony_z.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_z_collider.glb", path: "🏞️balconies/⚡️z/💥️capsule-with-balcony_z_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_slash.glb", path: "🏞️balconies/↗️slash/🧊️capsule-with-balcony_slash.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_slash_collider.glb", path: "🏞️balconies/↗️slash/💥️capsule-with-balcony_slash_collider.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_backslash.glb", path: "🏞️balconies/↘️backslash/🧊️capsule-with-balcony_backslash.glb" },
    { url: "/mesh/🧊️capsule-with-balcony_backslash_collider.glb", path: "🏞️balconies/↘️backslash/💥️capsule-with-balcony_backslash_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_J.glb", path: "🥚️ellipsoids/🪝️j/🧊️ellipsoid-capsule_J.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_J_collider.glb", path: "🥚️ellipsoids/🪝️j/💥️ellipsoid-capsule_J_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_L.glb", path: "🥚️ellipsoids/📐️l/🧊️ellipsoid-capsule_L.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_L_collider.glb", path: "🥚️ellipsoids/📐️l/💥️ellipsoid-capsule_L_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_p.glb", path: "🥚️ellipsoids/🅿️p/🧊️ellipsoid-capsule_p.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_p_collider.glb", path: "🥚️ellipsoids/🅿️p/💥️ellipsoid-capsule_p_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_q.glb", path: "🥚️ellipsoids/🔎️q/🧊️ellipsoid-capsule_q.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_q_collider.glb", path: "🥚️ellipsoids/🔎️q/💥️ellipsoid-capsule_q_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_s.glb", path: "🥚️ellipsoids/🐍️s/🧊️ellipsoid-capsule_s.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_s_collider.glb", path: "🥚️ellipsoids/🐍️s/💥️ellipsoid-capsule_s_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_z.glb", path: "🥚️ellipsoids/⚡️z/🧊️ellipsoid-capsule_z.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_z_collider.glb", path: "🥚️ellipsoids/⚡️z/💥️ellipsoid-capsule_z_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_slash.glb", path: "🥚️ellipsoids/↗️slash/🧊️ellipsoid-capsule_slash.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_slash_collider.glb", path: "🥚️ellipsoids/↗️slash/💥️ellipsoid-capsule_slash_collider.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_backslash.glb", path: "🥚️ellipsoids/↘️backslash/🧊️ellipsoid-capsule_backslash.glb" },
    { url: "/mesh/🧊️ellipsoid-capsule_backslash_collider.glb", path: "🥚️ellipsoids/↘️backslash/💥️ellipsoid-capsule_backslash_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_J.glb", path: "📐️trapezoids/🪝️j/🧊️trapezoid-capsule_J.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_J_collider.glb", path: "📐️trapezoids/🪝️j/💥️trapezoid-capsule_J_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_L.glb", path: "📐️trapezoids/📐️l/🧊️trapezoid-capsule_L.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_L_collider.glb", path: "📐️trapezoids/📐️l/💥️trapezoid-capsule_L_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_p.glb", path: "📐️trapezoids/🅿️p/🧊️trapezoid-capsule_p.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_p_collider.glb", path: "📐️trapezoids/🅿️p/💥️trapezoid-capsule_p_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_q.glb", path: "📐️trapezoids/🔎️q/🧊️trapezoid-capsule_q.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_q_collider.glb", path: "📐️trapezoids/🔎️q/💥️trapezoid-capsule_q_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_s.glb", path: "📐️trapezoids/🐍️s/🧊️trapezoid-capsule_s.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_s_collider.glb", path: "📐️trapezoids/🐍️s/💥️trapezoid-capsule_s_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_z.glb", path: "📐️trapezoids/⚡️z/🧊️trapezoid-capsule_z.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_z_collider.glb", path: "📐️trapezoids/⚡️z/💥️trapezoid-capsule_z_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_slash.glb", path: "📐️trapezoids/↗️slash/🧊️trapezoid-capsule_slash.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_slash_collider.glb", path: "📐️trapezoids/↗️slash/💥️trapezoid-capsule_slash_collider.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_backslash.glb", path: "📐️trapezoids/↘️backslash/🧊️trapezoid-capsule_backslash.glb" },
    { url: "/mesh/🧊️trapezoid-capsule_backslash_collider.glb", path: "📐️trapezoids/↘️backslash/💥️trapezoid-capsule_backslash_collider.glb" },
    { url: "/mesh/🧊️tambour.glb", path: "🥁️tambours/🧱️standard/🧊️tambour.glb" },
    { url: "/mesh/🧊️tambour_collider.glb", path: "🥁️tambours/🧱️standard/💥️tambour_collider.glb" },
    { url: "/mesh/🧊️tambour_first-storey.glb", path: "🥁️tambours/🌱️first-storey/🧊️tambour_first-storey.glb" },
    { url: "/mesh/🧊️tambour_first-storey_collider.glb", path: "🥁️tambours/🌱️first-storey/💥️tambour_first-storey_collider.glb" },
    { url: "/mesh/🧊️tambour_single-storey.glb", path: "🥁️tambours/🏠️single-storey/🧊️tambour_single-storey.glb" },
    { url: "/mesh/🧊️tambour_single-storey_collider.glb", path: "🥁️tambours/🏠️single-storey/💥️tambour_single-storey_collider.glb" },
    { url: "/mesh/🧊️tambour_last-storey.glb", path: "🥁️tambours/🏁️last-storey/🧊️tambour_last-storey.glb" },
    { url: "/mesh/🧊️tambour_last-storey_collider.glb", path: "🥁️tambours/🏁️last-storey/💥️tambour_last-storey_collider.glb" },
    { url: "/mesh/🧊️cylindric-tambour.glb", path: "🔋️cylindric-tambours/🧱️standard/🧊️cylindric-tambour.glb" },
    { url: "/mesh/🧊️cylindric-tambour_collider.glb", path: "🔋️cylindric-tambours/🧱️standard/💥️cylindric-tambour_collider.glb" },
    { url: "/mesh/🧊️cylindric-tambour_first-storey.glb", path: "🔋️cylindric-tambours/🌱️first-storey/🧊️cylindric-tambour_first-storey.glb" },
    { url: "/mesh/🧊️cylindric-tambour_first-storey_collider.glb", path: "🔋️cylindric-tambours/🌱️first-storey/💥️cylindric-tambour_first-storey_collider.glb" },
    { url: "/mesh/🧊️cylindric-tambour_single-storey.glb", path: "🔋️cylindric-tambours/🏠️single-storey/🧊️cylindric-tambour_single-storey.glb" },
    { url: "/mesh/🧊️cylindric-tambour_single-storey_collider.glb", path: "🔋️cylindric-tambours/🏠️single-storey/💥️cylindric-tambour_single-storey_collider.glb" },
    { url: "/mesh/🧊️cylindric-tambour_last-storey.glb", path: "🔋️cylindric-tambours/🏁️last-storey/🧊️cylindric-tambour_last-storey.glb" },
    { url: "/mesh/🧊️cylindric-tambour_last-storey_collider.glb", path: "🔋️cylindric-tambours/🏁️last-storey/💥️cylindric-tambour_last-storey_collider.glb" }
  ]
};

/* ../../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🟦️.ts */
function object(value, keys) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error("Mesh catalog object required");
  const row = value;
  if (Object.keys(row).length !== keys.length || keys.some((key) => !(key in row)))
    throw new Error("Mesh catalog fields do not match the schema");
  return row;
}
function path(value, extension = "") {
  if (typeof value !== "string" || !value.endsWith(extension))
    throw new Error("Mesh catalog path required");
  const stem = extension ? value.slice(0, -extension.length) : value;
  if (!stem || stem.split("/").some((part) => !part || /[.\\%?#\u0000-\u001f]/u.test(part)))
    throw new Error(`Unsafe mesh catalog path: ${value}`);
  return value;
}
function publicUrl(value) {
  if (typeof value !== "string" || !value.startsWith("/mesh/"))
    throw new Error("Mesh public URL required");
  const leaf = path(value.slice("/mesh/".length), ".glb");
  if (leaf.includes("/"))
    throw new Error("Mesh public identity must be a single explicit URL key");
  return value;
}
function rows(value) {
  if (!Array.isArray(value))
    throw new Error("Mesh catalog rows required");
  return value;
}
function parseMeshDeliveryCatalog(input, readCatalog) {
  const authority = object(input, ["$schema", "version", "collections", "entries"]);
  if (authority.version !== 1 || typeof authority.$schema !== "string")
    throw new Error("Unsupported mesh delivery schema");
  const result3 = [];
  const urls = new Set;
  const sources = new Set;
  const paths = new Set;
  const catalogs = new Set;
  const admit = (entry) => {
    if (urls.has(entry.url) || sources.has(entry.source) || paths.has(entry.path))
      throw new Error(`Duplicate mesh identity: ${entry.url}`);
    urls.add(entry.url);
    sources.add(entry.source);
    paths.add(entry.path);
    result3.push(Object.freeze(entry));
  };
  for (const value of rows(authority.collections)) {
    const collection = object(value, ["catalog", "root", "output"]);
    const catalogPath = path(collection.catalog, ".json");
    if (catalogs.has(catalogPath))
      throw new Error(`Duplicate mesh source catalog: ${catalogPath}`);
    catalogs.add(catalogPath);
    const root = path(collection.root);
    const output = path(collection.output);
    const source = object(readCatalog(catalogPath), ["$schema", "version", "entries"]);
    if (source.version !== 1 || typeof source.$schema !== "string" || rows(source.entries).length === 0)
      throw new Error("Unsupported mesh source schema");
    for (const value2 of rows(source.entries)) {
      const entry = object(value2, ["url", "path"]);
      const leaf = path(entry.path, ".glb");
      admit({ url: publicUrl(entry.url), source: `${root}/${leaf}`, path: `${output}/${leaf}` });
    }
  }
  for (const value of rows(authority.entries)) {
    const entry = object(value, ["url", "source", "path"]);
    admit({ url: publicUrl(entry.url), source: path(entry.source, ".glb"), path: path(entry.path, ".glb") });
  }
  return Object.freeze(result3);
}
var MESH_DELIVERY_CATALOG = parseMeshDeliveryCatalog(_catalog_default3, (path2) => {
  if (path2 === "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json")
    return _catalog_default4;
  throw new Error(`Unknown mesh source catalog: ${path2}`);
});
var indexes = new WeakMap;
function resolveMeshAsset(url, catalog = MESH_DELIVERY_CATALOG) {
  let index = indexes.get(catalog);
  if (!index) {
    index = new Map(catalog.map((entry2) => [entry2.url, entry2]));
    indexes.set(catalog, index);
  }
  const entry = index.get(url);
  if (!entry)
    throw new Error(`Unknown mesh asset: ${url}`);
  return entry;
}
function meshAssetTransportUrl(url, catalog = MESH_DELIVERY_CATALOG) {
  return url.startsWith("/mesh/") ? `/mesh/${resolveMeshAsset(url, catalog).path}` : url;
}

/* ../../🎞️frame-worker/🟦️.ts */
var WORKER_STEP_BUDGET_MS = 8;
var BOOT_HEARTBEAT_MS = 2;
var PLUGIN_BOOT_CAPACITY = 32;
var PLUGIN_MANIFEST_CODE_UNIT_CAPACITY = 64 * 1024;
var ASSET_RESPONSE_BYTE_CAPACITY = 16 * 1024 * 1024;
var ASSET_RESPONSE_PAGE_BYTES = 16 * 1024;
function ownedStep(stage, callback) {
  const startedAt = performance.now();
  const value = callback();
  const duration = performance.now() - startedAt;
  if (duration >= WORKER_STEP_BUDGET_MS)
    throw new Error(`worker-boot-step-overrun: ${stage} took ${duration.toFixed(3)} ms`);
  return value;
}
async function monitoredSuspension(stage, operation) {
  let lastBeat = performance.now();
  let maximumBlockMs = 0;
  const heartbeat = setInterval(() => {
    const now = performance.now();
    maximumBlockMs = Math.max(maximumBlockMs, now - lastBeat - BOOT_HEARTBEAT_MS);
    lastBeat = now;
  }, BOOT_HEARTBEAT_MS);
  try {
    const result3 = await ownedStep(`${stage}:start`, operation);
    await new Promise((resolve) => setTimeout(resolve, 0));
    if (closed || closing)
      throw new Error(`worker-boot-cancelled: ${stage}`);
    if (maximumBlockMs >= WORKER_STEP_BUDGET_MS)
      throw new Error(`worker-boot-step-overrun: ${stage} blocked the Worker for ${maximumBlockMs.toFixed(3)} ms`);
    return result3;
  } finally {
    clearInterval(heartbeat);
  }
}
async function macrotask() {
  await new Promise((resolve) => setTimeout(resolve, 0));
  if (closed || closing)
    throw new Error("worker-boot-cancelled");
}
var scope = self;
var lifecycle = 0;
var runtime;
var interactiveJobs;
var closed = false;
var closing = false;
var failed = false;
var quarantined;
var lastFrame = { cursor: "default", fullscreen: null };
var pendingFault;
var runtimeCloseComplete = false;
var jobsCloseComplete = false;
var closeOwner2 = "runtime";
var assetPumping = false;
var assetAbort;
scope.onmessage = (event) => void receive(event.data);
async function receive(message) {
  if (message.kind === "boot") {
    await boot(message);
    return;
  }
  if (message.lifecycle !== lifecycle)
    return;
  if (message.kind === "close") {
    if (closed || closing)
      return;
    beginClose();
    return;
  }
  if (closed || closing || failed || quarantined)
    return;
  if (message.kind === "job-submit" || message.kind === "job-input-page" || message.kind === "job-cancel") {
    if (!interactiveJobs) {
      fault("interactive-job-not-ready", "interactive job arrived before Worker boot completed");
      return;
    }
    const startedAt2 = performance.now();
    interactiveJobs.receive(message);
    const duration = performance.now() - startedAt2;
    if (duration >= WORKER_STEP_BUDGET_MS)
      fault("interactive-job-overrun", `interactive job admission turn took ${duration.toFixed(3)} ms`);
    return;
  }
  if (!runtime) {
    fault("worker-not-booted", "frame batch arrived before renderer boot completed");
    return;
  }
  const startedAt = performance.now();
  try {
    runtime.enqueueBatch(JSON.stringify({ replaceable: message.replaceable, lossless: message.lossless }), message.generation);
    const result3 = JSON.parse(runtime.tick(message.timestampMs, message.sequence, message.generation));
    const duration = performance.now() - startedAt;
    lastFrame = { cursor: result3.cursor, fullscreen: result3.fullscreen };
    if (result3.quarantined || duration >= WORKER_STEP_BUDGET_MS)
      quarantined = { code: result3.faultCode ?? "worker-step-overrun", detail: result3.faultDetail ?? `frame step took ${duration.toFixed(3)} ms` };
    post({ kind: "frame", lifecycle, sequence: message.sequence, generation: message.generation, cursor: result3.cursor, fullscreen: result3.fullscreen, requestFrame: result3.requestFrame, progress: result3.progress, workerDurationMs: duration, quarantined: quarantined !== undefined, faultCode: quarantined?.code, faultDetail: quarantined?.detail });
    if (quarantined)
      requestFault(quarantined.code, quarantined.detail);
    else
      scheduleAssetPump();
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
  }
}
async function closeRuntime() {
  for (;; ) {
    const startedAt = performance.now();
    if (closeOwner2 === "runtime" && !runtimeCloseComplete) {
      runtimeCloseComplete = runtime ? runtime.closeStep() : true;
      closeOwner2 = "jobs";
    } else if (!jobsCloseComplete) {
      jobsCloseComplete = interactiveJobs ? interactiveJobs.closeStep() : true;
      closeOwner2 = "runtime";
    } else if (!runtimeCloseComplete) {
      closeOwner2 = "runtime";
    }
    if (performance.now() - startedAt >= WORKER_STEP_BUDGET_MS) {
      pendingFault ??= { code: "worker-close-overrun", detail: "Worker close turn exceeded the Worker budget" };
    }
    if (runtimeCloseComplete && jobsCloseComplete)
      break;
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  if (pendingFault)
    post({ kind: "fault", lifecycle, code: pendingFault.code, detail: pendingFault.detail });
  post({ kind: "closed", lifecycle });
  closed = true;
  scope.close();
}
function beginClose() {
  if (closed || closing)
    return;
  closing = true;
  failed = pendingFault !== undefined;
  runtimeCloseComplete = runtime === undefined;
  jobsCloseComplete = interactiveJobs === undefined;
  assetAbort?.abort();
  assetAbort = undefined;
  if (runtime) {
    try {
      ownedStep("asset-abort", () => runtime.abortAssetResponse());
    } catch (error) {
      pendingFault ??= { code: "asset-abort-fault", detail: error instanceof Error ? error.message : String(error) };
    }
  }
  interactiveJobs?.close();
  closeRuntime();
}
async function boot(message) {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  lifecycle = message.lifecycle;
  try {
    progress("renderer-module", 0.05);
    const bindings = await monitoredSuspension("renderer-module", () => import(message.bindingsModuleUrl));
    if (bindings.default) {
      progress("wasm-instance", 0.15);
      await monitoredSuspension("wasm-instance", () => bindings.default(message.bindingsWasmUrl));
    }
    if (!bindings.semioWgpuWorkerBootstrap)
      throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      bindings.semioWgpuSetAppRole?.(message.appRole);
      if (message.hub)
        bindings.semioWgpuSetHubEnv?.(message.hub.hubUrl, message.hub.user, message.hub.dataDir);
    });
    progress("plugin-graph", 0.25);
    const bootPlan = ownedStep("plugin-graph", () => resolvePlaygroundBoot(PLUGIN_CATALOG, message.pluginVariant));
    if (bootPlan.plugins.length > PLUGIN_BOOT_CAPACITY)
      throw new Error(`plugin-credits: boot plan exceeds ${PLUGIN_BOOT_CAPACITY} plugins`);
    for (const error of bootPlan.dependencyErrors)
      progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins = [];
    for (let index = 0;index < bootPlan.plugins.length; index++) {
      const target = bootPlan.plugins[index];
      progress(`plugin:${target.pluginId}`, 0.3 + 0.3 * (index / Math.max(1, bootPlan.plugins.length)));
      await macrotask();
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl));
      ownedStep(`plugin-manifest:${target.pluginId}`, () => {
        const manifest = JSON.stringify(module.manifest);
        if (manifest.length > PLUGIN_MANIFEST_CODE_UNIT_CAPACITY)
          throw new Error(`plugin-manifest-credits: ${target.pluginId} exceeds ${PLUGIN_MANIFEST_CODE_UNIT_CAPACITY} code units`);
      });
      plugins.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, handle: pluginHandleForBridge(module) })));
    }
    if (plugins.length === 0)
      throw new Error(`no wasm plugin modules found for variant ${message.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => bindings.semioWgpuWorkerBootstrap(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => post({ kind: "wake", lifecycle })));
    while (true) {
      await macrotask();
      const step13 = ownedStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()));
      progress(step13.stage, 0.65 + step13.progress * 0.3);
      if (step13.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell());
        continue;
      }
      if (step13.complete)
        break;
    }
    runtime = ownedStep("renderer-finish", () => bootstrap.finish());
    interactiveJobs = ownedStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)));
    progress("ready", 1);
    post({ kind: "booted", lifecycle });
    scheduleAssetPump();
  } catch (error) {
    fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}
function scheduleAssetPump() {
  if (assetPumping || !runtime || closed || closing || failed || quarantined)
    return;
  assetPumping = true;
  setTimeout(() => void pumpAsset(), 0);
}
async function pumpAsset() {
  try {
    if (!runtime || closed || closing || failed || quarantined)
      return;
    const request = ownedStep("asset-request", () => JSON.parse(runtime.pollAssetRequest()));
    if (!request.available)
      return;
    if (!request.url || request.responseByteCapacity !== ASSET_RESPONSE_BYTE_CAPACITY || request.pageByteCapacity !== ASSET_RESPONSE_PAGE_BYTES) {
      throw new Error("asset-request-protocol: request descriptor did not match fixed Worker credits");
    }
    assetAbort = new AbortController;
    const response = await monitoredSuspension("asset-fetch", () => fetch(meshAssetTransportUrl(request.url), { signal: assetAbort.signal }));
    if (!response.ok || !response.body)
      throw new Error(`asset-fetch-status: ${response.status}`);
    const declaredHeader = ownedStep("asset-response-headers", () => response.headers.get("content-length"));
    const declared = declaredHeader === null ? undefined : Number(declaredHeader);
    if (declared !== undefined && (!Number.isSafeInteger(declared) || declared < 0 || declared > ASSET_RESPONSE_BYTE_CAPACITY))
      throw new Error("asset-response-length: Content-Length exceeded fixed aggregate credits");
    ownedStep("asset-response-reserve", () => runtime.reserveAssetResponse(declared ?? ASSET_RESPONSE_BYTE_CAPACITY));
    const reader = ownedStep("asset-stream-reader", () => response.body.getReader({ mode: "byob" }));
    let received = 0;
    for (;; ) {
      const pageOwner2 = ownedStep("asset-page-owner", () => new Uint8Array(ASSET_RESPONSE_PAGE_BYTES));
      const chunk = await monitoredSuspension("asset-stream-read", () => reader.read(pageOwner2));
      if (chunk.done)
        break;
      const bytes = chunk.value;
      if (bytes.byteLength === 0 || bytes.byteLength > ASSET_RESPONSE_PAGE_BYTES)
        throw new Error("asset-response-page: stream violated fixed BYOB page credits");
      received += bytes.byteLength;
      if (received > (declared ?? ASSET_RESPONSE_BYTE_CAPACITY))
        throw new Error("asset-response-overflow: stream exceeded admitted bytes");
      ownedStep("asset-page", () => runtime.pushAssetResponsePage(bytes));
      await macrotask();
    }
    ownedStep("asset-stream-release", () => reader.releaseLock());
    if (declared !== undefined && received !== declared)
      throw new Error("asset-response-short-read: stream ended before declared bytes");
    ownedStep("asset-seal", () => runtime.sealAssetResponse());
    post({ kind: "wake", lifecycle });
  } catch (error) {
    if (runtime) {
      try {
        ownedStep("asset-abort", () => runtime.abortAssetResponse());
      } catch {}
    }
    if (!closing && !closed)
      fault("asset-stream-fault", error instanceof Error ? error.message : String(error));
  } finally {
    assetAbort = undefined;
    assetPumping = false;
  }
}
function progress(stage, value) {
  if (!closed && !closing && !failed)
    post({ kind: "boot-progress", lifecycle, stage, progress: value });
}
function post(message) {
  scope.postMessage(message);
}
function fault(code, detail) {
  requestFault(code, detail);
}
function requestFault(code, detail) {
  if (closed || pendingFault)
    return;
  pendingFault = { code, detail };
  failed = true;
  beginClose();
}
