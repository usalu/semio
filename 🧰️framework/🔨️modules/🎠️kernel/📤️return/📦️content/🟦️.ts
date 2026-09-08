//#region 🧬️ContentFramingContract
import type { ActorUiPatchReceipt } from "../../../🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
export type KernelReturnContentMetadata = {
  readonly status: "idle" | "moreWork" | "checkpointReady" | "faulted";
  readonly nextWake: bigint | null;
  readonly fuelUsed: bigint;
  readonly effectCount: bigint;
  readonly presenceCount: bigint;
};
export type KernelReturnContentByte = "prefix" | "header" | "body";
const CONTENT_MAGIC = [0x73, 0x72, 0x74, 1] as const;
const CONTENT_STATUS = ["idle", "moreWork", "checkpointReady", "faulted"] as const;
function contentFault(reason: string): never { throw new Error(`return-content.${reason}`); }

class ContentUnsigned {
  #value = 0n;
  #count = 0;
  get complete(): boolean { return this.#count === 0; }
  push(byte: number): bigint | null {
    if (this.#count === 9 && byte > 1) return contentFault("integer-overflow");
    this.#value |= BigInt(byte & 127) << BigInt(this.#count * 7);
    if (byte & 128) { this.#count++; return null; }
    if (this.#count && byte === 0) return contentFault("noncanonical-integer");
    const value = this.#value;
    this.#value = 0n; this.#count = 0;
    return value;
  }
}
//#endregion 🧬️ContentFramingContract

//#region 🗂️ContentSections
class ContentSections {
  readonly #unsigned = new ContentUnsigned();
  #section = -1;
  #field = 0;
  #status = 0;
  #nextWake: bigint | null = null;
  #fuelUsed = 0n;
  #effects = 0n;
  #presence = 0n;
  #operations = 0n;
  #surface = 0n;
  #metadata: KernelReturnContentMetadata | null = null;
  #patchActivation = 0n;
  #patchInstance = 0;
  #patchGuest = 0n;
  #patchSequence = 0n;
  #uiReceipt: ActorUiPatchReceipt | null = null;
  get metadata(): KernelReturnContentMetadata | null { return this.#metadata; }
  get uiReceipt(): ActorUiPatchReceipt | null { return this.#uiReceipt; }
  begin(tag: number, length: bigint): void {
    const beforeEffects = this.#section === 0 || this.#section === 1 || this.#section === 4 || this.#section === 5;
    const beforePresence = beforeEffects || this.#section === 6;
    let allowed = false;
    switch (tag) {
      case 0: allowed = this.#section === -1; if (length < 5n || length > 42n) contentFault("metadata-length"); break;
      case 1: allowed = this.#section === 0; if (length === 0n || length > 44n) contentFault("lifecycle-length"); break;
      case 2: allowed = this.#section === 0 || this.#section === 1; if (length < 9n) contentFault("ui-begin-length"); break;
      case 3: allowed = (this.#section === 2 || this.#section === 3) && this.#operations > 0n; if (length < 2n) contentFault("ui-operation-length"); break;
      case 4: allowed = (this.#section === 2 || this.#section === 3) && this.#operations === 0n; break;
      case 5: allowed = beforeEffects && this.#effects > 0n; break;
      case 6: allowed = beforePresence && this.#effects === 0n && this.#presence > 0n; break;
      case 7: allowed = beforePresence && this.#effects === 0n && this.#presence === 0n; break;
      case 8: allowed = this.#section === 7 && this.#status >= 2; break;
      case 9: allowed = this.#section === (this.#status >= 2 ? 8 : 7); break;
    }
    if (!allowed) contentFault("section-order");
    if (tag === 4 || tag === 9) { if (length !== 0n) contentFault("empty-record-length"); }
    else if (length === 0n) contentFault("empty-body");
    this.#section = tag; this.#field = 0;
  }
  byte(byte: number): void {
    if (this.#section === 0) this.#metadataByte(byte);
    else if (this.#section === 2) this.#uiBeginByte(byte);
    else if (this.#section === 3 && this.#field === 0) {
      if (byte > 10) contentFault("ui-opcode");
      this.#field = 1;
    }
  }
  #metadataByte(byte: number): void {
    if (this.#field === 0) { if (byte > 3) contentFault("status"); this.#status = byte; this.#field = 1; return; }
    if (this.#field === 1) { if (byte > 1) contentFault("next-wake-option"); this.#field = byte === 0 ? 3 : 2; return; }
    if (this.#field > 5) contentFault("metadata-trailing");
    const value = this.#unsigned.push(byte);
    if (value === null) return;
    switch (this.#field++) {
      case 2: this.#nextWake = value; break;
      case 3: this.#fuelUsed = value; break;
      case 4: this.#effects = value; break;
      case 5: this.#presence = value; break;
    }
  }
  #uiBeginByte(byte: number): void {
    if (this.#surface > 0n) { this.#surface--; return; }
    if (this.#field > 7) contentFault("ui-begin-trailing");
    const value = this.#unsigned.push(byte);
    if (value === null) return;
    const field = this.#field++;
    if ((field === 0 || field === 2 || field === 3) && value === 0n) contentFault("patch-authority");
    if (field === 1 && value > 0xffffffffn) contentFault("instance-overflow");
    if (field === 0) this.#patchActivation = value;
    if (field === 1) this.#patchInstance = Number(value);
    if (field === 2) this.#patchGuest = value;
    if (field === 3) this.#patchSequence = value;
    if (field === 4) { if (value === 0n) contentFault("surface-length"); this.#surface = value; }
    if (field === 7) { if (value > 1153n) contentFault("operation-count"); this.#operations = value; }
  }
  end(): void {
    if (!this.#unsigned.complete) contentFault("truncated-integer");
    switch (this.#section) {
      case 0:
        if (this.#field !== 6) contentFault("truncated-metadata");
        this.#metadata = Object.freeze({ status: CONTENT_STATUS[this.#status]!, nextWake: this.#nextWake, fuelUsed: this.#fuelUsed, effectCount: this.#effects, presenceCount: this.#presence });
        break;
      case 2:
        if (this.#field !== 8 || this.#surface !== 0n) contentFault("truncated-ui-begin");
        this.#uiReceipt = Object.freeze({ lifetime: Object.freeze({ activationGeneration: this.#patchActivation, instanceId: this.#patchInstance, guestLifetime: this.#patchGuest }), patchSequence: this.#patchSequence });
        break;
      case 3: this.#operations--; break;
      case 5: this.#effects--; break;
      case 6: this.#presence--; break;
    }
  }
}
//#endregion 🗂️ContentSections

//#region 📤️ContentFraming
/** 📤️ One-byte framing and counted section validation; semantic bodies and raw input ownership remain with their exact consumers. */
export class KernelReturnContentFraming {
  readonly #unsigned = new ContentUnsigned();
  readonly #sections = new ContentSections();
  #phase: "magic" | "tag" | "length" | "body" | "done" = "magic";
  #magic = 0;
  #tag = -1;
  #length = 0n;
  #remaining = 0n;
  #failure: string | null = null;
  constructor() { Object.freeze(this); }
  get tag(): number { return this.#tag; }
  get length(): bigint { return this.#length; }
  get remaining(): bigint { return this.#remaining; }
  get complete(): boolean { return this.#phase === "done" && this.#failure === null; }
  get failure(): string | null { return this.#failure; }
  get metadata(): KernelReturnContentMetadata | null { return this.#sections.metadata; }
  get uiReceipt(): ActorUiPatchReceipt | null { return this.#sections.uiReceipt; }
  #end(): void { this.#sections.end(); this.#phase = this.#tag === 9 ? "done" : "tag"; }
  push(byte: number): KernelReturnContentByte {
    if (this.#failure !== null) throw new Error(this.#failure);
    try {
      if (!Number.isInteger(byte) || byte < 0 || byte > 255) contentFault("byte");
      switch (this.#phase) {
        case "magic":
          if (byte !== CONTENT_MAGIC[this.#magic++]) contentFault("magic");
          if (this.#magic === CONTENT_MAGIC.length) this.#phase = "tag";
          return "prefix";
        case "tag":
          if (byte > 9) contentFault("record-tag");
          this.#tag = byte; this.#phase = "length"; return "prefix";
        case "length": {
          const length = this.#unsigned.push(byte);
          if (length === null) return "prefix";
          this.#sections.begin(this.#tag, length);
          this.#length = length; this.#remaining = length; this.#phase = "body";
          if (length === 0n) this.#end();
          return "header";
        }
        case "body":
          this.#sections.byte(byte); this.#remaining--;
          if (this.#remaining === 0n) this.#end();
          return "body";
        case "done": return contentFault("trailing");
      }
    } catch (error) {
      this.#failure = error instanceof Error ? error.message : "return-content.fault";
      throw error;
    }
  }
  finish(): void {
    if (this.#failure !== null) throw new Error(this.#failure);
    if (!this.complete) { this.#failure = "return-content.truncated"; throw new Error(this.#failure); }
  }
}
//#endregion 📤️ContentFraming

//#region 🏷️OperationFieldHeader
export type KernelReturnUiFieldName = "node" | "component" | "layout" | "activity" | "children" | "style" | "accessibility" | "bindings" | "menu";
export type KernelReturnUiOperationFields = { readonly opcode: number; readonly node: bigint | null; readonly field: KernelReturnUiFieldName | null; readonly payloadLength: bigint; readonly headerLength: number };
const UI_FIELDS = ["node", "component", "layout", "activity", "children", "style", "accessibility", "bindings", "menu", null, null] as const;

/** 🏷️ Selects one grammar-owned field range without reading or allocating its payload. */
export class KernelReturnUiOperationHeader {
  readonly #unsigned = new ContentUnsigned();
  #remaining: bigint;
  #phase: "opcode" | "node" | "length" | "done" = "opcode";
  #opcode = -1;
  #node: bigint | null = null;
  #count = 0;
  #value: KernelReturnUiOperationFields | null = null;
  #failure: string | null = null;
  constructor(bodyLength: bigint) {
    if (typeof bodyLength !== "bigint" || bodyLength < 2n || bodyLength > 0xffffffffffffffffn) contentFault("ui-header-length");
    this.#remaining = bodyLength; Object.freeze(this);
  }
  get value(): KernelReturnUiOperationFields | null { return this.#value; }
  get failure(): string | null { return this.#failure; }
  #complete(): void {
    this.#value = Object.freeze({ opcode: this.#opcode, node: this.#node, field: UI_FIELDS[this.#opcode]!, payloadLength: this.#remaining, headerLength: this.#count });
    this.#phase = "done";
  }
  push(byte: number): void {
    if (this.#failure !== null) throw new Error(this.#failure);
    try {
      if (!Number.isInteger(byte) || byte < 0 || byte > 255) contentFault("byte");
      if (this.#phase === "done") contentFault("ui-header-complete");
      this.#remaining--; this.#count++;
      if (this.#phase === "opcode") {
        if (byte > 10) contentFault("ui-opcode");
        this.#opcode = byte; this.#phase = byte === 0 ? "length" : "node";
      } else {
        const value = this.#unsigned.push(byte);
        if (value !== null && this.#phase === "node") {
          this.#node = value;
          if (this.#opcode >= 9) { if (this.#remaining !== 0n) contentFault("ui-scalar-trailing"); this.#complete(); }
          else if (this.#opcode === 4) { if (this.#remaining === 0n) contentFault("ui-children-count"); this.#complete(); }
          else this.#phase = "length";
        } else if (value !== null) {
          if (value !== this.#remaining) contentFault("ui-payload-length");
          this.#complete();
        }
      }
      if (this.#remaining === 0n && this.#value === null) contentFault("ui-header-truncated");
    } catch (error) { this.#failure = error instanceof Error ? error.message : "return-content.ui-header-fault"; throw error; }
  }
  finish(): void {
    if (this.#failure !== null) throw new Error(this.#failure);
    if (this.#value === null) { this.#failure = "return-content.ui-header-truncated"; throw new Error(this.#failure); }
  }
}
//#endregion 🏷️OperationFieldHeader

//#region 🧪️ContentFramingLaws
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts");
  await registerTests1(import.meta.vitest, { KernelReturnContentFraming }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️ContentFramingLaws

//#region 🧪️OperationHeaderLaws
if (import.meta.vitest) {
  const { registerTests2 } = await import("./🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts");
  await registerTests2(import.meta.vitest, { KernelReturnUiOperationHeader }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️OperationHeaderLaws
