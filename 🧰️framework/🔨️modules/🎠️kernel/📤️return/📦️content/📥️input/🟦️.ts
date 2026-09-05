//#region 📥️OwnedInputContract
import { OwnedShardReturn, OwnedShardReturnPage, type ShardActorActivationLease } from "../../../../🎭️actor/📮️shard-client/🟦️.ts";
import { actorInstanceLifetimeEquals, type ActorInstanceLifetime } from "../../../../🎭️actor/🚪️lifetime/🟦️.ts";
import type { ActorUiPatchReceipt } from "../../../../🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import type { OwnedUiInstance } from "../../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { OwnedUiOperationPayloadBuilder, OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } from "../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts";
import { OwnedUiResidentPayload, OwnedUiResidentPayloadSourceRelease, OwnedUiResidentBuilderRetirement, type OwnedUiResidentInstance } from "../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts";
import { KernelReturnContentFraming, KernelReturnUiOperationHeader, type KernelReturnUiFieldName } from "../🟦️.ts";

export type KernelReturnInputGrant = { readonly maxItems: number; readonly maxBytes: number };
export type KernelReturnInputStep = { readonly kind: "pending" | "ready" | "complete" | "blocked" | "rejected"; readonly items: number; readonly bytes: number };
export type KernelReturnInputFieldValue = { readonly operation: number; readonly opcode: number; readonly node: bigint | null; readonly name: KernelReturnUiFieldName; readonly byteLength: bigint; readonly receipt: ActorUiPatchReceipt };
type InputOwner = { readonly source: OwnedShardReturn; readonly owner: OwnedUiInstance; readonly activation: ShardActorActivationLease; readonly lifetime: ActorInstanceLifetime; readonly framing: KernelReturnContentFraming; content: OwnedKernelReturnContent | null; page: OwnedShardReturnPage | null; cursor: number; operation: number; header: KernelReturnUiOperationHeader | null; field: OwnedKernelReturnInputField | null; failure: string | null; fault: unknown; closing: boolean };
const MINT = Object.freeze({});
const NO_INPUT_FAULT = Symbol("return-input.no-fault");
const BUILDER_CONSUMED = Symbol("return-input.builder-consumed");
const result = (kind: KernelReturnInputStep["kind"], bytes = 0): KernelReturnInputStep => ({ kind, items: bytes ? 1 : 0, bytes });
const payloadGrant = (grant: KernelReturnInputGrant): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= 64;
function inputRejected(state: InputOwner, code: string, bytes: number): KernelReturnInputStep { state.failure = code; return result("rejected", bytes); }
function inputFault(state: InputOwner, error: unknown, bytes: number): KernelReturnInputStep { if (state.fault === NO_INPUT_FAULT) state.fault = error; else if (!Object.is(state.fault, error)) throw error; return inputRejected(state, "return-input.fault", bytes); }
let mintField: (state: InputOwner, value: KernelReturnInputFieldValue, page: OwnedShardReturnPage, start: number) => OwnedKernelReturnInputField;
let mintFragment: (field: OwnedKernelReturnInputField, page: OwnedShardReturnPage, start: number, offset: bigint, length: number) => OwnedKernelReturnInputFragment;
let fieldBuilder: (field: OwnedKernelReturnInputField) => OwnedUiOperationPayloadBuilder | null;
let fieldReadable: (field: OwnedKernelReturnInputField) => boolean;
let recordFieldRelease: (field: OwnedKernelReturnInputField, fragment: OwnedKernelReturnInputFragment, release: OwnedKernelReturnInputRelease) => boolean;
let fieldOwnsRelease: (field: unknown, release: OwnedKernelReturnInputRelease) => boolean;
let retainFieldFault: (field: OwnedKernelReturnInputField, error: unknown) => void;
let mintRelease: (fragment: OwnedKernelReturnInputFragment, proof: object, kind: "copied" | "cancelled") => OwnedKernelReturnInputRelease;
let installFragmentRelease: (fragment: OwnedKernelReturnInputFragment, release: OwnedKernelReturnInputRelease) => void;
let detachFragmentField: (fragment: OwnedKernelReturnInputFragment) => void;
let clearFragmentRelease: (fragment: OwnedKernelReturnInputFragment, release: OwnedKernelReturnInputRelease) => void;
let releaseKind: (release: OwnedKernelReturnInputRelease) => "copied" | "cancelled";
let detachInputRelease: (release: OwnedKernelReturnInputRelease) => void;
let settleInputRelease: (release: OwnedKernelReturnInputRelease) => void;
let installFieldFragment: (field: OwnedKernelReturnInputField, fragment: OwnedKernelReturnInputFragment) => void;
let mintPayloadDetachment: (field: OwnedKernelReturnInputField) => OwnedKernelReturnPayloadDetachment;
let installPayloadDetachment: (field: OwnedKernelReturnInputField, observation: OwnedKernelReturnPayloadDetachment) => void;
let ownsPayloadDetachment: (field: unknown, observation: OwnedKernelReturnPayloadDetachment) => boolean;
let payloadDetachmentPhase: (observation: OwnedKernelReturnPayloadDetachment) => "unbound" | "bound" | "detached" | "settled";
let bindPayloadDetachment: (observation: OwnedKernelReturnPayloadDetachment, payload: OwnedUiResidentPayload) => void;
let detachPayload: (observation: OwnedKernelReturnPayloadDetachment, payload: OwnedUiResidentPayload, proof: OwnedUiResidentPayloadSourceRelease) => void;
let settlePayload: (observation: OwnedKernelReturnPayloadDetachment, proof: unknown) => boolean;
//#endregion 📥️OwnedInputContract

//#region 📦️CapturedContent
/** 📦️ A strongly registered parser owns the original captured page and every selected field. */
export class OwnedKernelReturnContent {
  readonly #state: InputOwner;
  constructor(source: OwnedShardReturn, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime) {
    if (!OwnedShardReturn.matchesOwner(source, owner, activation, lifetime)) throw new Error("return-input.owner");
    this.#state = { source, owner, activation, lifetime: Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime }), framing: new KernelReturnContentFraming(), content: this, page: null, cursor: 0, operation: 0, header: null, field: null, failure: null, fault: NO_INPUT_FAULT, closing: false };
    if (!source.bindContent(this)) throw new Error("return-input.content-owned");
    Object.freeze(this);
  }
  static matches(content: unknown, source: OwnedShardReturn, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): content is OwnedKernelReturnContent {
    if (content === null || typeof content !== "object" || !(#state in content)) return false;
    const state = content.#state;
    return state.source === source && state.owner === owner && state.activation === activation && actorInstanceLifetimeEquals(state.lifetime, lifetime);
  }
  get field(): OwnedKernelReturnInputField | null { return this.#state.field; }
  get failure(): string | null { return this.#state.failure; }
  /** 🧯️ Identifies the original retained fault without inspecting or releasing its contents. */
  static matchesFault(content: unknown, fault: unknown): boolean { return content !== null && typeof content === "object" && #state in content && content.#state.fault !== NO_INPUT_FAULT && Object.is(content.#state.fault, fault); }
  advance(grant: KernelReturnInputGrant): KernelReturnInputStep {
    const state = this.#state;
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || state.closing) return result("blocked");
    if (state.failure !== null) return result("rejected");
    if (state.field !== null) return result("ready");
    try {
      const page = state.source.page;
      if (page === null) return result("blocked");
      if (!OwnedShardReturnPage.matchesOwner(page, state.owner, state.activation, state.lifetime)) return inputRejected(state, "return-input.page-owner", 1);
      if (state.page === null) { if (page.receipt.pageSequence !== 1n) return inputRejected(state, "return-input.page-sequence", 1); state.page = page; }
      if (state.page !== page) return inputRejected(state, "return-input.page-pending", 1);
      if (state.cursor >= page.receipt.length) { if (page.receipt.final) state.framing.finish(); return result("blocked"); }
      const byte = page.byteAt(state.cursor);
      const kind = state.framing.push(byte); state.cursor++;
      if (kind === "header" && state.framing.tag === 3) state.header = new KernelReturnUiOperationHeader(state.framing.length);
      if (kind === "body" && state.framing.tag === 2 && state.framing.remaining === 0n) {
        const receipt = state.framing.uiReceipt;
        if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state.lifetime)) return inputRejected(state, "return-input.patch-lifetime", 1);
      }
      if (kind === "body" && state.framing.tag === 3 && state.header !== null) {
        state.header.push(byte);
        const fields = state.header.value;
        if (fields !== null) {
          const receipt = state.framing.uiReceipt;
          if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state.lifetime)) return inputRejected(state, "return-input.patch-lifetime", 1);
          if (fields.field !== null) state.field = mintField(state, Object.freeze({ operation: state.operation, opcode: fields.opcode, node: fields.node, name: fields.field, byteLength: fields.payloadLength, receipt }), page, state.cursor);
          state.operation++; state.header = null;
        }
      }
      return result(state.field ? "ready" : "pending", 1);
    } catch (error) { return inputFault(state, error, 1); }
  }
  beginClose(): void { const state = this.#state; state.closing = true; state.field?.beginClose(); }
}
//#endregion 📦️CapturedContent

//#region 🏷️PrivateField
/** 🏷️ Only the captured content parser can select this exact operation field and its admitted consumer. */
export class OwnedKernelReturnInputField {
  readonly #state: InputOwner;
  readonly #value: KernelReturnInputFieldValue;
  #builder: OwnedUiOperationPayloadBuilder | OwnedUiResidentBuilderRetirement | typeof BUILDER_CONSUMED | null = null;
  #fragment: OwnedKernelReturnInputFragment | null = null;
  #release: OwnedKernelReturnInputRelease | null = null;
  #consumed = 0n;
  #advanced = 0;
  #start: number;
  #complete = false;
  #closing = false;
  #payload: OwnedUiResidentPayload | null = null;
  #payloadDetachment: OwnedKernelReturnPayloadDetachment | null = null;
  private constructor(mint: object, state: InputOwner, value: KernelReturnInputFieldValue, page: OwnedShardReturnPage, start: number) {
    if (mint !== MINT) throw new Error("return-input.private-field");
    this.#state = state; this.#value = value; this.#start = start; state.field = this;
    mintPayloadDetachment(this);
    const available = page.receipt.length - start;
    this.#fragment = mintFragment(this, page, start, 0n, Number(value.byteLength < BigInt(available) ? value.byteLength : BigInt(available)));
    Object.freeze(this);
  }
  static {
    mintField = (state, value, page, start) => new OwnedKernelReturnInputField(MINT, state, value, page, start);
    fieldBuilder = field => OwnedUiOperationPayloadBuilder.hasBrand(field.#builder) ? field.#builder : null;
    fieldReadable = field => !field.#closing && !field.#state.closing && field.#state.failure === null;
    installFieldFragment = (field, fragment) => { if (field.#fragment !== null) throw new Error("return-input.fragment-owned"); field.#fragment = fragment; };
    installPayloadDetachment = (field, observation) => { if (field.#payloadDetachment !== null) throw new Error("return-input.payload-observation-owned"); field.#payloadDetachment = observation; };
    ownsPayloadDetachment = (field, observation) => field !== null && typeof field === "object" && #payloadDetachment in field && field.#payloadDetachment === observation;
    fieldOwnsRelease = (field, release) => field !== null && typeof field === "object" && #release in field && field.#release === release && field.#fragment !== null;
    retainFieldFault = (field, error) => { inputFault(field.#state, error, 0); };
    recordFieldRelease = (field, fragment, release) => {
      if (field.#fragment !== fragment || field.#release !== null || field.#state.field !== field) return false;
      field.#release = release;
      if (releaseKind(release) === "cancelled") field.#closing = true;
      return true;
    };
  }
  static matchesOwner(field: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): field is OwnedKernelReturnInputField {
    if (field === null || typeof field !== "object" || !(#state in field)) return false;
    const state = field.#state;
    return state.field === field && state.owner === owner && state.activation === activation && actorInstanceLifetimeEquals(state.lifetime, lifetime) && OwnedShardReturn.matchesOwner(state.source, owner, activation, lifetime);
  }
  /** 🪢️ Identifies the actual private binding even when its caller faulted before recording success. */
  static matchesBuilder(field: unknown, builder: unknown): builder is OwnedUiOperationPayloadBuilder {
    return field !== null && typeof field === "object" && #builder in field && OwnedUiOperationPayloadBuilder.hasBrand(builder) && field.#builder === builder;
  }
  /** 🪢️ Observes the same private proof held after the original binding was detached. */
  static matchesBuilderDetached(field: unknown, proof: unknown): boolean { return field !== null && typeof field === "object" && #builder in field && field.#builder !== null && field.#builder !== BUILDER_CONSUMED && !OwnedUiOperationPayloadBuilder.hasBrand(field.#builder) && field.#builder === proof; }
  /** 🧵️ Settled observation still requires the original UI parent to hold its exact field/proof pair. */
  static matchesBuilderSettled(field: unknown, proof: unknown): boolean { return field !== null && typeof field === "object" && #builder in field && field.#builder === BUILDER_CONSUMED && OwnedUiResidentBuilderRetirement.matchesSourceBinding(proof, field); }
  detachBuilder(builder: OwnedUiOperationPayloadBuilder, proof: OwnedUiResidentBuilderRetirement, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    if (this.#state.field !== this || this.#release !== null || this.#builder !== null && this.#builder !== builder || !OwnedUiResidentBuilderRetirement.matchesBody(proof, builder, this)) return result("rejected");
    this.#closing = true; this.#builder = proof; return result("pending", 64);
  }
  settleBuilder(proof: OwnedUiResidentBuilderRetirement, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    if (this.#state.field !== this || this.#builder !== proof || !OwnedUiResidentBuilderRetirement.matchesDetached(proof, this)) return result("rejected");
    this.#builder = BUILDER_CONSUMED; return result("complete", 64);
  }
  static matchesResidentPayload(field: unknown, payload: unknown): payload is OwnedUiResidentPayload { return field !== null && typeof field === "object" && #payload in field && field.#payload !== null && field.#payload === payload; }
  installResidentPayload(payload: OwnedUiResidentPayload, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    const observation = this.#payloadDetachment; const state = this.#state;
    if (!observation || !fieldReadable(this) || state.field !== this || !OwnedUiResidentPayload.matchesField(payload, this) || !OwnedUiResidentPayload.matchesOwner(payload, state.owner, state.activation, state.lifetime)) return result("rejected");
    if (payloadDetachmentPhase(observation) === "bound" && this.#payload === payload) return result("ready");
    if (payloadDetachmentPhase(observation) !== "unbound" || this.#payload !== null) return result("rejected");
    bindPayloadDetachment(observation, payload); this.#payload = payload; return result("ready", 64);
  }
  residentPayload(scope: OwnedUiResidentInstance): OwnedUiResidentPayload | null { const payload = this.#payload; return payload !== null && OwnedUiResidentPayload.matchesScope(payload, scope) ? payload : null; }
  get residentPayloadDetachment(): OwnedKernelReturnPayloadDetachment | null { const observation = this.#payloadDetachment; if (!observation) return null; const phase = payloadDetachmentPhase(observation); return phase === "detached" || phase === "settled" ? observation : null; }
  detachResidentPayload(payload: OwnedUiResidentPayload, proof: OwnedUiResidentPayloadSourceRelease, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    const observation = this.#payloadDetachment;
    if (!observation || !OwnedUiResidentPayloadSourceRelease.matches(proof, payload, this)) return result("rejected");
    const phase = payloadDetachmentPhase(observation);
    if (!(phase === "bound" && this.#payload === payload || phase === "unbound" && this.#payload === null)) return result("rejected");
    detachPayload(observation, payload, proof); this.#payload = null; return result("pending", 64);
  }
  settleResidentPayload(detachment: OwnedKernelReturnPayloadDetachment, sourceDetachedProof: OwnedUiResidentPayloadSourceRelease, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    if (detachment !== this.#payloadDetachment || this.#payload !== null || !settlePayload(detachment, sourceDetachedProof)) return result("rejected");
    return result("complete", 64);
  }
  get value(): KernelReturnInputFieldValue { return this.#value; }
  get owner(): OwnedUiInstance { return this.#state.owner; }
  get activation(): ShardActorActivationLease { return this.#state.activation; }
  get lifetime(): ActorInstanceLifetime { return this.#state.lifetime; }
  get fragment(): OwnedKernelReturnInputFragment | null { return this.#fragment; }
  get consumed(): bigint { return this.#consumed; }
  get complete(): boolean { return this.#complete; }
  bind(builder: OwnedUiOperationPayloadBuilder): boolean {
    if (!fieldReadable(this) || this.#builder !== null && this.#builder !== builder || !OwnedUiOperationPayloadBuilder.matchesField(builder, this)) return false;
    this.#builder = builder; return true;
  }
  /** 🧶️ Detaches only the original range evidence after consumption or permanent close. */
  detachInputRelease(release: OwnedKernelReturnInputRelease, proof: unknown, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    const fragment = this.#fragment;
    if (!fragment || this.#release !== release || !OwnedKernelReturnInputRelease.matches(release, fragment, proof) || !uiMatchesRelease(proof, release)) return result("rejected");
    if (releaseKind(release) === "copied" && !this.#closing && !this.#state.closing && (this.#advanced !== fragment.length || this.#consumed !== fragment.offset + BigInt(fragment.length))) return result("rejected");
    detachFragmentField(fragment); detachInputRelease(release); return result("pending", 64);
  }
  /** 🧵️ Keeps next-range admission fenced until the original UI evidence detaches. */
  settleInputRelease(release: OwnedKernelReturnInputRelease, proof: unknown, grant: KernelReturnInputGrant): KernelReturnInputStep {
    if (!payloadGrant(grant)) return result("blocked");
    const fragment = this.#fragment;
    if (!fragment || !OwnedKernelReturnInputRelease.matchesSourceDetached(release, this, proof) || !uiMatchesSourceDetached(proof, release)) return result("rejected");
    clearFragmentRelease(fragment, release); this.#fragment = null; this.#release = null; this.#advanced = 0; this.#start = this.#state.cursor; settleInputRelease(release); return result("complete", 64);
  }
  /** 📏️ Advances only a privately released range; field completion never acknowledges the containing page. */
  advance(grant: KernelReturnInputGrant, builder: unknown): KernelReturnInputStep {
    const state = this.#state;
    if (!OwnedUiOperationPayloadBuilder.hasBrand(builder) || this.#builder !== builder) return result("rejected");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || this.#closing || state.closing) return result("blocked");
    if (state.failure !== null) return result("rejected");
    if (this.#complete) return result("complete");
    const fragment = this.#fragment;
    if (!fragment || !this.#release || releaseKind(this.#release) !== "copied") return result("blocked");
    let bytes = 0;
    try {
      const page = state.page;
      if (state.field !== this || !page || state.source.page !== page || state.cursor !== this.#start + this.#advanced || fragment.offset + BigInt(this.#advanced) !== this.#consumed) return inputRejected(state, "return-input.continuation-owner", bytes);
      if (this.#advanced < fragment.length) {
        if (state.cursor >= page.receipt.length || state.framing.tag !== 3 || state.framing.remaining === 0n) return inputRejected(state, "return-input.continuation-range", bytes);
        bytes = 1;
        if (state.framing.push(page.byteAt(state.cursor)) !== "body") return inputRejected(state, "return-input.continuation-framing", bytes);
        state.cursor++; this.#advanced++; this.#consumed++;
      }
      if (this.#advanced < fragment.length) return result("pending", bytes);
      if (this.#consumed === this.#value.byteLength) {
        if (state.framing.remaining !== 0n) return inputRejected(state, "return-input.continuation-trailing", bytes);
        this.#complete = true;
      }
      if (this.#consumed > this.#value.byteLength) return inputRejected(state, "return-input.continuation-overflow", bytes);
      return result(this.#complete ? "complete" : "pending", bytes);
    } catch (error) { return inputFault(state, error, bytes); }
  }
  beginClose(): void { this.#closing = true; }
}
//#endregion 🏷️PrivateField

//#region 💾️PayloadAssociation
/** 🪢️ One original observation retains temporary UI aliases only until exact two-way settlement. */
export class OwnedKernelReturnPayloadDetachment {
  #payload: OwnedUiResidentPayload | null = null;
  #proof: OwnedUiResidentPayloadSourceRelease | null = null;
  #phase: "unbound" | "bound" | "detached" | "settled" = "unbound";
  private constructor(mint: object, field: OwnedKernelReturnInputField) { if (mint !== MINT) throw new Error("return-input.private-payload-detachment"); installPayloadDetachment(field, this); Object.freeze(this); }
  static {
    mintPayloadDetachment = field => new OwnedKernelReturnPayloadDetachment(MINT, field);
    payloadDetachmentPhase = observation => observation.#phase;
    bindPayloadDetachment = (observation, payload) => { observation.#payload = payload; observation.#phase = "bound"; };
    detachPayload = (observation, payload, proof) => { observation.#payload = payload; observation.#proof = proof; observation.#phase = "detached"; };
    settlePayload = (observation, proof) => {
      const payload = observation.#payload;
      if (observation.#phase !== "detached" || payload === null || observation.#proof !== proof || !OwnedUiResidentPayloadSourceRelease.matchesDetached(proof, payload) || !OwnedUiResidentPayload.matchesSourceDetachment(payload, observation)) return false;
      observation.#payload = null; observation.#proof = null; observation.#phase = "settled"; return true;
    };
  }
  static matchesOwner(observation: unknown, field: unknown): observation is OwnedKernelReturnPayloadDetachment { return observation !== null && typeof observation === "object" && #phase in observation && ownsPayloadDetachment(field, observation); }
  static matches(observation: unknown, field: unknown, payload: unknown): observation is OwnedKernelReturnPayloadDetachment { return OwnedKernelReturnPayloadDetachment.matchesOwner(observation, field) && observation.#phase === "detached" && observation.#payload !== null && observation.#payload === payload; }
  static matchesSettled(observation: unknown, payload: unknown): observation is OwnedKernelReturnPayloadDetachment { return observation !== null && typeof observation === "object" && #phase in observation && observation.#phase === "settled" && observation.#payload === null && observation.#proof === null && OwnedUiResidentPayload.matchesSourceDetachment(payload, observation); }
}
//#endregion 💾️PayloadAssociation

//#region 📄️PrivateFragment
/** 📄️ One selected raw-page range stays owned until the exact private copied or detached proof arrives. */
export class OwnedKernelReturnInputFragment {
  #field: OwnedKernelReturnInputField | null;
  #page: OwnedShardReturnPage | null;
  readonly #start: number;
  readonly #offset: bigint;
  readonly #length: number;
  #release: OwnedKernelReturnInputRelease | null = null;
  private constructor(mint: object, field: OwnedKernelReturnInputField, page: OwnedShardReturnPage, start: number, offset: bigint, length: number) {
    if (mint !== MINT) throw new Error("return-input.private-fragment");
    this.#field = field; this.#page = page; this.#start = start; this.#offset = offset; this.#length = length; installFieldFragment(field, this); Object.freeze(this);
  }
  static {
    mintFragment = (field, page, start, offset, length) => new OwnedKernelReturnInputFragment(MINT, field, page, start, offset, length);
    installFragmentRelease = (fragment, release) => {
      if (!fragment.#field || fragment.#release !== null || !recordFieldRelease(fragment.#field, fragment, release)) throw new Error("return-input.release-owned");
      fragment.#release = release; fragment.#page = null;
    };
    detachFragmentField = fragment => { fragment.#field = null; fragment.#page = null; };
    clearFragmentRelease = (fragment, release) => { if (fragment.#release !== release || fragment.#field !== null || fragment.#page !== null) throw new Error("return-input.release-detachment"); fragment.#release = null; };
  }
  static matches(fragment: unknown, field: object): fragment is OwnedKernelReturnInputFragment { return fragment !== null && typeof fragment === "object" && #field in fragment && fragment.#field === field; }
  get field(): OwnedKernelReturnInputField | null { return this.#field; }
  get offset(): bigint { return this.#offset; }
  get length(): number { return this.#length; }
  byteAt(index: number, builder: unknown): number {
    if (!this.#field || fieldBuilder(this.#field) !== builder || !OwnedUiOperationPayloadBuilder.matchesField(builder, this.#field)) throw new Error("return-input.builder");
    if (!fieldReadable(this.#field) || this.#page === null || !Number.isSafeInteger(index) || index < 0 || index >= this.#length) throw new Error("return-input.fragment-read");
    return this.#page.byteAt(this.#start + index);
  }
  release(proof: unknown): OwnedKernelReturnInputRelease | null {
    if (this.#release) return OwnedKernelReturnInputRelease.matches(this.#release, this, proof) ? this.#release : null;
    const field = this.#field; if (!field) return null;
    const builder = fieldBuilder(field);
    if (!builder) return null;
    const copied = OwnedUiOperationInputCopied.matches(proof, this, field, builder, this.#offset, this.#length);
    const cancelled = !copied && OwnedUiOperationInputCancelled.matches(proof, this, field, builder, this.#offset, this.#length);
    if (!copied && !cancelled) return null;
    try { return mintRelease(this, proof as object, copied ? "copied" : "cancelled"); }
    catch (error) { retainFieldFault(field, error); throw error; }
  }
}
//#endregion 📄️PrivateFragment

//#region 🧾️PrivateRelease
function uiMatchesRelease(proof: unknown, release: OwnedKernelReturnInputRelease): boolean { return OwnedUiOperationInputCopied.matchesRelease(proof, release) || OwnedUiOperationInputCancelled.matchesRelease(proof, release); }
function uiMatchesSourceDetached(proof: unknown, release: OwnedKernelReturnInputRelease): boolean { return OwnedUiOperationInputCopied.matchesSourceDetached(proof, release) || OwnedUiOperationInputCancelled.matchesSourceDetached(proof, release); }
/** 🧾️ This local receipt certifies one exact raw reader detachment, not page ACK or UI publication. */
export class OwnedKernelReturnInputRelease {
  #fragment: OwnedKernelReturnInputFragment | null;
  #proof: object | null;
  readonly #kind: "copied" | "cancelled";
  #phase: "issued" | "sourceDetached" | "settled" = "issued";
  private constructor(mint: object, fragment: OwnedKernelReturnInputFragment, proof: object, kind: "copied" | "cancelled") { if (mint !== MINT) throw new Error("return-input.private-release"); this.#fragment = fragment; this.#proof = proof; this.#kind = kind; installFragmentRelease(fragment, this); Object.freeze(this); }
  static {
    mintRelease = (fragment, proof, kind) => new OwnedKernelReturnInputRelease(MINT, fragment, proof, kind);
    releaseKind = release => release.#kind;
    detachInputRelease = release => { release.#fragment = null; release.#proof = null; release.#phase = "sourceDetached"; };
    settleInputRelease = release => { release.#phase = "settled"; };
  }
  static matches(receipt: unknown, fragment: object, proof: unknown): receipt is OwnedKernelReturnInputRelease { return receipt !== null && typeof receipt === "object" && #fragment in receipt && receipt.#phase === "issued" && receipt.#fragment === fragment && receipt.#proof === proof; }
  static matchesSourceDetached(receipt: unknown, field: unknown, proof: unknown): receipt is OwnedKernelReturnInputRelease { return receipt !== null && typeof receipt === "object" && #phase in receipt && receipt.#phase === "sourceDetached" && receipt.#fragment === null && receipt.#proof === null && fieldOwnsRelease(field, receipt) && uiMatchesRelease(proof, receipt); }
  static matchesSettled(receipt: unknown, proof: unknown): receipt is OwnedKernelReturnInputRelease { return receipt !== null && typeof receipt === "object" && #phase in receipt && receipt.#phase === "settled" && receipt.#fragment === null && receipt.#proof === null && uiMatchesRelease(proof, receipt); }
  get kind(): "copied" | "cancelled" { return this.#kind; }
}
//#endregion 🧾️PrivateRelease
