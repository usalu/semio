//#region 📥️OwnedInputContract
import { OwnedShardReturn, OwnedShardReturnPage, type ShardActorActivationLease } from "../../../../🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import { actorInstanceLifetimeEquals, type ActorInstanceLifetime } from "../../../../🎭️actor/🚪️lifetime/🟦️component.ts";
import type { ActorUiPatchReceipt } from "../../../../🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts";
import type { OwnedUiInstance } from "../../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts";
import { OwnedUiOperationPayloadBuilder, OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } from "../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts";
import { KernelReturnContentFraming, KernelReturnUiOperationHeader, type KernelReturnUiFieldName } from "../🟦️component.ts";

export type KernelReturnInputGrant = { readonly maxItems: number; readonly maxBytes: number };
export type KernelReturnInputStep = { readonly kind: "pending" | "ready" | "complete" | "blocked" | "rejected"; readonly items: number; readonly bytes: number };
export type KernelReturnInputFieldValue = { readonly operation: number; readonly opcode: number; readonly node: bigint | null; readonly name: KernelReturnUiFieldName; readonly byteLength: bigint; readonly receipt: ActorUiPatchReceipt };
type InputOwner = { readonly source: OwnedShardReturn; readonly owner: OwnedUiInstance; readonly activation: ShardActorActivationLease; readonly lifetime: ActorInstanceLifetime; readonly framing: KernelReturnContentFraming; content: OwnedKernelReturnContent | null; page: OwnedShardReturnPage | null; cursor: number; operation: number; header: KernelReturnUiOperationHeader | null; field: OwnedKernelReturnInputField | null; failure: string | null; closing: boolean };
const MINT = Object.freeze({});
const result = (kind: KernelReturnInputStep["kind"], bytes = 0): KernelReturnInputStep => ({ kind, items: bytes ? 1 : 0, bytes });
let mintField: (state: InputOwner, value: KernelReturnInputFieldValue, page: OwnedShardReturnPage, start: number) => OwnedKernelReturnInputField;
let mintFragment: (field: OwnedKernelReturnInputField, page: OwnedShardReturnPage, start: number, offset: bigint, length: number) => OwnedKernelReturnInputFragment;
let fieldBuilder: (field: OwnedKernelReturnInputField) => OwnedUiOperationPayloadBuilder | null;
let fieldReadable: (field: OwnedKernelReturnInputField) => boolean;
let recordFieldRelease: (field: OwnedKernelReturnInputField, fragment: OwnedKernelReturnInputFragment, release: OwnedKernelReturnInputRelease) => boolean;
let mintRelease: (fragment: OwnedKernelReturnInputFragment, proof: object, kind: "copied" | "cancelled") => OwnedKernelReturnInputRelease;
//#endregion 📥️OwnedInputContract

//#region 📦️CapturedContent
/** 📦️ A strongly registered parser owns the original captured page and every selected field. */
export class OwnedKernelReturnContent {
  readonly #state: InputOwner;
  constructor(source: OwnedShardReturn, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime) {
    if (!OwnedShardReturn.matchesOwner(source, owner, activation, lifetime)) throw new Error("return-input.owner");
    this.#state = { source, owner, activation, lifetime: Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime }), framing: new KernelReturnContentFraming(), content: this, page: null, cursor: 0, operation: 0, header: null, field: null, failure: null, closing: false };
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
  advance(grant: KernelReturnInputGrant): KernelReturnInputStep {
    const state = this.#state;
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || state.closing) return result("blocked");
    if (state.failure !== null) return result("rejected");
    if (state.field !== null) return result("ready");
    try {
      const page = state.source.page;
      if (page === null) return result("blocked");
      if (!OwnedShardReturnPage.matchesOwner(page, state.owner, state.activation, state.lifetime)) throw new Error("return-input.page-owner");
      if (state.page === null) { if (page.receipt.pageSequence !== 1n) throw new Error("return-input.page-sequence"); state.page = page; }
      if (state.page !== page) throw new Error("return-input.page-pending");
      if (state.cursor >= page.receipt.length) { if (page.receipt.final) state.framing.finish(); return result("blocked"); }
      const byte = page.byteAt(state.cursor);
      const kind = state.framing.push(byte); state.cursor++;
      if (kind === "header" && state.framing.tag === 3) state.header = new KernelReturnUiOperationHeader(state.framing.length);
      if (kind === "body" && state.framing.tag === 2 && state.framing.remaining === 0n) {
        const receipt = state.framing.uiReceipt;
        if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state.lifetime)) throw new Error("return-input.patch-lifetime");
      }
      if (kind === "body" && state.framing.tag === 3 && state.header !== null) {
        state.header.push(byte);
        const fields = state.header.value;
        if (fields !== null) {
          const receipt = state.framing.uiReceipt;
          if (!receipt || !actorInstanceLifetimeEquals(receipt.lifetime, state.lifetime)) throw new Error("return-input.patch-lifetime");
          if (fields.field !== null) state.field = mintField(state, Object.freeze({ operation: state.operation, opcode: fields.opcode, node: fields.node, name: fields.field, byteLength: fields.payloadLength, receipt }), page, state.cursor);
          state.operation++; state.header = null;
        }
      }
      return result(state.field ? "ready" : "pending", 1);
    } catch (error) { state.failure = error instanceof Error ? error.message : "return-input.fault"; return result("rejected", 1); }
  }
  beginClose(): void { const state = this.#state; state.closing = true; state.field?.beginClose(); }
}
//#endregion 📦️CapturedContent

//#region 🏷️PrivateField
/** 🏷️ Only the captured content parser can select this exact operation field and its admitted consumer. */
export class OwnedKernelReturnInputField {
  readonly #state: InputOwner;
  readonly #value: KernelReturnInputFieldValue;
  #builder: OwnedUiOperationPayloadBuilder | null = null;
  #fragment: OwnedKernelReturnInputFragment | null;
  #release: OwnedKernelReturnInputRelease | null = null;
  #consumed = 0n;
  #advanced = 0;
  #start: number;
  #complete = false;
  #closing = false;
  private constructor(mint: object, state: InputOwner, value: KernelReturnInputFieldValue, page: OwnedShardReturnPage, start: number) {
    if (mint !== MINT) throw new Error("return-input.private-field");
    this.#state = state; this.#value = value; this.#start = start;
    const available = page.receipt.length - start;
    this.#fragment = mintFragment(this, page, start, 0n, Number(value.byteLength < BigInt(available) ? value.byteLength : BigInt(available)));
    Object.freeze(this);
  }
  static {
    mintField = (state, value, page, start) => new OwnedKernelReturnInputField(MINT, state, value, page, start);
    fieldBuilder = field => field.#builder;
    fieldReadable = field => !field.#closing && !field.#state.closing && field.#state.failure === null;
    recordFieldRelease = (field, fragment, release) => {
      if (field.#fragment !== fragment || field.#release !== null || field.#state.field !== field) return false;
      field.#release = release;
      if (release.kind === "cancelled") field.#closing = true;
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
    return field !== null && typeof field === "object" && #builder in field && field.#builder !== null && field.#builder === builder;
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
  /** 📏️ Advances only a privately released range; field completion never acknowledges the containing page. */
  advance(grant: KernelReturnInputGrant, builder: unknown): KernelReturnInputStep {
    const state = this.#state;
    if (this.#builder === null || this.#builder !== builder) return result("rejected");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 1 || this.#closing || state.closing) return result("blocked");
    if (state.failure !== null) return result("rejected");
    if (this.#complete) return result("complete");
    const fragment = this.#fragment;
    if (!fragment || this.#release?.kind !== "copied") return result("blocked");
    let bytes = 0;
    try {
      const page = state.page;
      if (state.field !== this || !page || state.source.page !== page || state.cursor !== this.#start + this.#advanced || fragment.offset + BigInt(this.#advanced) !== this.#consumed) throw new Error("return-input.continuation-owner");
      if (this.#advanced < fragment.length) {
        if (state.cursor >= page.receipt.length || state.framing.tag !== 3 || state.framing.remaining === 0n) throw new Error("return-input.continuation-range");
        bytes = 1;
        if (state.framing.push(page.byteAt(state.cursor)) !== "body") throw new Error("return-input.continuation-framing");
        state.cursor++; this.#advanced++; this.#consumed++;
      }
      if (this.#advanced < fragment.length) return result("pending", bytes);
      if (this.#consumed === this.#value.byteLength) {
        if (state.framing.remaining !== 0n) throw new Error("return-input.continuation-trailing");
        this.#complete = true;
      }
      if (this.#consumed > this.#value.byteLength) throw new Error("return-input.continuation-overflow");
      this.#fragment = null; this.#release = null; this.#advanced = 0;
      return result(this.#complete ? "complete" : "pending", bytes);
    } catch (error) { state.failure = error instanceof Error ? error.message : "return-input.continuation-fault"; return result("rejected", bytes); }
  }
  beginClose(): void { this.#closing = true; }
}
//#endregion 🏷️PrivateField

//#region 📄️PrivateFragment
/** 📄️ One selected raw-page range stays owned until the exact private copied or detached proof arrives. */
export class OwnedKernelReturnInputFragment {
  readonly #field: OwnedKernelReturnInputField;
  #page: OwnedShardReturnPage | null;
  readonly #start: number;
  readonly #offset: bigint;
  readonly #length: number;
  #release: OwnedKernelReturnInputRelease | null = null;
  private constructor(mint: object, field: OwnedKernelReturnInputField, page: OwnedShardReturnPage, start: number, offset: bigint, length: number) {
    if (mint !== MINT) throw new Error("return-input.private-fragment");
    this.#field = field; this.#page = page; this.#start = start; this.#offset = offset; this.#length = length; Object.freeze(this);
  }
  static { mintFragment = (field, page, start, offset, length) => new OwnedKernelReturnInputFragment(MINT, field, page, start, offset, length); }
  static matches(fragment: unknown, field: object): fragment is OwnedKernelReturnInputFragment { return fragment !== null && typeof fragment === "object" && #field in fragment && fragment.#field === field; }
  get field(): OwnedKernelReturnInputField { return this.#field; }
  get offset(): bigint { return this.#offset; }
  get length(): number { return this.#length; }
  byteAt(index: number, builder: unknown): number {
    if (fieldBuilder(this.#field) !== builder || !OwnedUiOperationPayloadBuilder.matchesField(builder, this.#field)) throw new Error("return-input.builder");
    if (!fieldReadable(this.#field) || this.#page === null || !Number.isSafeInteger(index) || index < 0 || index >= this.#length) throw new Error("return-input.fragment-read");
    return this.#page.byteAt(this.#start + index);
  }
  release(proof: unknown): OwnedKernelReturnInputRelease | null {
    if (this.#release) return OwnedKernelReturnInputRelease.matches(this.#release, this, proof) ? this.#release : null;
    const builder = fieldBuilder(this.#field);
    if (!builder) return null;
    const copied = OwnedUiOperationInputCopied.matches(proof, this, this.#field, builder, this.#offset, this.#length);
    const cancelled = !copied && OwnedUiOperationInputCancelled.matches(proof, this, this.#field, builder, this.#offset, this.#length);
    if (!copied && !cancelled) return null;
    const receipt = mintRelease(this, proof as object, copied ? "copied" : "cancelled");
    if (!recordFieldRelease(this.#field, this, receipt)) return null;
    this.#page = null; this.#release = receipt; return receipt;
  }
}
//#endregion 📄️PrivateFragment

//#region 🧾️PrivateRelease
/** 🧾️ This local receipt certifies one exact raw reader detachment, not page ACK or UI publication. */
export class OwnedKernelReturnInputRelease {
  readonly #fragment: OwnedKernelReturnInputFragment;
  readonly #proof: object;
  readonly #kind: "copied" | "cancelled";
  private constructor(mint: object, fragment: OwnedKernelReturnInputFragment, proof: object, kind: "copied" | "cancelled") { if (mint !== MINT) throw new Error("return-input.private-release"); this.#fragment = fragment; this.#proof = proof; this.#kind = kind; Object.freeze(this); }
  static { mintRelease = (fragment, proof, kind) => new OwnedKernelReturnInputRelease(MINT, fragment, proof, kind); }
  static matches(receipt: unknown, fragment: object, proof: unknown): receipt is OwnedKernelReturnInputRelease { return receipt !== null && typeof receipt === "object" && #fragment in receipt && receipt.#fragment === fragment && receipt.#proof === proof; }
  get kind(): "copied" | "cancelled" { return this.#kind; }
}
//#endregion 🧾️PrivateRelease
