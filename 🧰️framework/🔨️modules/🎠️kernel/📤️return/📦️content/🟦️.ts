//#region 🧬️ContentFramingContract
import type { ActorUiPatchReceipt } from "../../../🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts";
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
  const { it, expect, vi } = import.meta.vitest;
  const factory = async () => new (await import("./🟦️.ts")).KernelReturnContentFraming();
  const oracle = async () => {
    const name = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(name);
    const encode = (module.default ?? module).encodeUIntBuffer;
    const uint = (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
    const frame = (tag: number, body: Uint8Array): Buffer => Buffer.concat([Buffer.from([tag]), uint(body.length), body]);
    return { uint, frame };
  };
  const collect = async (bytes: Uint8Array, pageBytes = 4096) => {
    const cursor = await factory();
    const records: { tag: number; length: bigint; bytes: number[] }[] = [];
    for (let page = 0; page < bytes.length; page += pageBytes) {
      for (const byte of bytes.subarray(page, page + pageBytes)) {
        const kind = cursor.push(byte);
        if (kind === "header") records.push({ tag: cursor.tag, length: cursor.length, bytes: [] });
        else if (kind === "body") records.at(-1)!.bytes.push(byte);
      }
    }
    cursor.finish();
    return { cursor, records };
  };

  it("KernelReturnContentFraming matches the shared stream and independent frame encoding at every split", async () => {
    const { default: wire } = await import("./🧬️wire/🔣️.json");
    const { default: schema } = await import("./🧬️schema/🔣️.json");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { default: fixtureSchema } = await import("./🧪️schema/🔣️.json");
    const { default: pageSchema } = await import("../../../🎭️actor/📄️page/🧬️schema.json");
    const { default: lifetimeSchema } = await import("../../../🎭️actor/🚪️lifetime/🧬️schema.json");
    const { default: patchSchema } = await import("../../../🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(pageSchema).addSchema(lifetimeSchema).addSchema(patchSchema);
    expect(ajv.compile(schema)(wire)).toBe(true);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
    const { frame } = await oracle();
    const chunks = fixture.recordVectors.map(row => {
      const body = Buffer.from(row.bodyHex, "hex");
      expect(frame(row.tag, body).toString("hex")).toBe(row.frameHex);
      return frame(row.tag, body);
    });
    const bytes = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), ...chunks]);
    const expected = fixture.recordVectors.map(row => ({ tag: row.tag, length: BigInt(row.bodyHex.length / 2), bytes: [...Buffer.from(row.bodyHex, "hex")] }));
    for (const size of fixture.crossPage.pageBytes) expect((await collect(bytes, size)).records).toEqual(expected);
    for (let split = 0; split <= bytes.length; split++) {
      const cursor = await factory();
      for (const byte of bytes.subarray(0, split)) cursor.push(byte);
      for (const byte of bytes.subarray(split)) cursor.push(byte);
      cursor.finish();
      expect(cursor.complete).toBe(true);
      expect(cursor.metadata).toEqual({ status: "idle", nextWake: null, fuelUsed: 1n, effectCount: 0n, presenceCount: 0n });
    }
  });

  it("KernelReturnContentFraming rejects every shared section-order violation and exact counted bodies", async () => {
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { frame, uint } = await oracle();
    const magic = Buffer.from(fixture.magicHex, "hex");
    const frames = new Map(fixture.recordVectors.map(row => [row.tag, Buffer.from(row.frameHex, "hex")]));
    for (const tags of fixture.negativeSequences) await expect(collect(Buffer.concat([magic, ...tags.map(tag => frames.get(tag)!)]))).rejects.toThrow(/return-content/);
    for (const status of [0, 1, 2, 3]) {
      const metadata = frame(0, Buffer.concat([Buffer.from([status, 1]), uint(0xffffffffffffffffn), uint(0xffffffffffffffffn), uint(2), uint(1)]));
      const effects = [frame(5, Buffer.from(fixture.invocation.effectBodyHex, "hex")), frame(5, Buffer.of(0))];
      const presence = frame(6, Buffer.of(0));
      const tail = [frames.get(7)!, ...(status >= 2 ? [frame(8, Buffer.of(0))] : []), frames.get(9)!];
      const valid = Buffer.concat([magic, metadata, ...effects, presence, ...tail]);
      const result = await collect(valid, 1);
      expect(result.cursor.metadata?.effectCount).toBe(2n);
      expect(result.cursor.metadata?.presenceCount).toBe(1n);
      expect(Buffer.from(result.records[1]!.bytes).toString("hex")).toBe(fixture.invocation.effectBodyHex);
      await expect(collect(Buffer.concat([magic, metadata, effects[0]!, presence, ...tail]))).rejects.toThrow();
      await expect(collect(Buffer.concat([magic, metadata, ...effects, ...tail]))).rejects.toThrow();
      await expect(collect(Buffer.concat([magic, metadata, ...effects, presence, frame(6, Buffer.of(0)), ...tail]))).rejects.toThrow();
      const wrongTail = [frames.get(7)!, ...(status < 2 ? [frame(8, Buffer.of(0))] : []), frames.get(9)!];
      await expect(collect(Buffer.concat([magic, metadata, ...effects, presence, ...wrongTail]))).rejects.toThrow();
    }
    const metadata = frames.get(0)!;
    const lifecycle = frame(1, Buffer.from("0101070201", "hex"));
    await collect(Buffer.concat([magic, metadata, lifecycle, frames.get(7)!, frames.get(9)!]));
    await expect(collect(Buffer.concat([magic, metadata, lifecycle, lifecycle, frames.get(7)!, frames.get(9)!]))).rejects.toThrow();
  });

  it("KernelReturnContentFraming never allocates a body from an announced u64 length", async () => {
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { frame, uint } = await oracle();
    const prefix = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), frame(0, Buffer.of(0, 0, 0, 1, 0)), Buffer.of(5), uint(0xffffffffffffffffn)]);
    const cursor = await factory();
    const allocations: unknown[] = [];
    const original = Uint8Array;
    vi.stubGlobal("Uint8Array", new Proxy(original, { construct(target, args, newTarget) { allocations.push(args[0]); return Reflect.construct(target, args, newTarget); } }));
    try {
      for (const byte of prefix) cursor.push(byte);
      expect(cursor.length).toBe(0xffffffffffffffffn);
      expect(cursor.remaining).toBe(0xffffffffffffffffn);
      expect(cursor.push(7)).toBe("body");
      expect(cursor.remaining).toBe(0xfffffffffffffffen);
      expect(allocations).toEqual([]);
      expect(() => cursor.finish()).toThrow(/truncated/);
    } finally { vi.unstubAllGlobals(); }
  });

  it("KernelReturnContentFraming preserves large Unicode and opaque operation bytes across raw pages", async () => {
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { frame, uint } = await oracle();
    const surface = Buffer.concat([Buffer.from(fixture.uiBegin.surface), Buffer.alloc(fixture.crossPage.largeSurfaceBytes, 97)]);
    const beginBody = Buffer.concat([Buffer.of(1, 7, 2, 3), uint(surface.length), surface, Buffer.of(0, 1, 1)]);
    const uiBegin = frame(2, beginBody);
    const payload = Buffer.alloc(8193);
    for (let index = 0; index < payload.length; index++) payload[index] = (index * 37 + 11) % 256;
    const operation = Buffer.concat([Buffer.of(1, 7), uint(payload.length), payload]);
    const frames = fixture.recordVectors.map(row => row.tag === 2 ? uiBegin : row.tag === 3 ? frame(3, operation) : Buffer.from(row.frameHex, "hex"));
    const bytes = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), ...frames]);
    const copy = Buffer.from(bytes);
    for (const size of fixture.crossPage.pageBytes) {
      const { records } = await collect(bytes, size);
      expect(Buffer.from(records[1]!.bytes)).toEqual(beginBody);
      expect(Buffer.from(records[2]!.bytes)).toEqual(operation);
    }
    expect(bytes).toEqual(copy);
  });

  it("KernelReturnContentFraming rejects truncation, noncanonical lengths, invalid counts and sticky faults", async () => {
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { frame, uint } = await oracle();
    const magic = Buffer.from(fixture.magicHex, "hex");
    const valid = Buffer.concat([magic, ...fixture.recordVectors.map(row => Buffer.from(row.frameHex, "hex"))]);
    for (let length = 0; length < valid.length; length++) await expect(collect(valid.subarray(0, length))).rejects.toThrow();
    for (const tail of [0, 9, 255]) await expect(collect(Buffer.concat([valid, Buffer.of(tail)]))).rejects.toThrow();
    for (const suffix of ["008000", "0080808080808080808002", "0080808080808080808080", "0a00", "0000"]) await expect(collect(Buffer.concat([magic, Buffer.from(suffix, "hex")]))).rejects.toThrow();
    const tail = fixture.recordVectors.filter(row => row.tag >= 2).map(row => Buffer.from(row.frameHex, "hex"));
    for (const body of [Buffer.of(4, 0, 1, 0, 0), Buffer.of(0, 2, 1, 0, 0), Buffer.of(0, 0, 0x80, 0, 0, 0), Buffer.of(0, 0, 1, 0, 0, 0)]) await expect(collect(Buffer.concat([magic, frame(0, body), ...tail]))).rejects.toThrow();
    const begin = fixture.recordVectors.find(row => row.tag === 2)!;
    const prefix = Buffer.concat([magic, Buffer.from(fixture.recordVectors[0]!.frameHex, "hex")]);
    const invalidBegin = Buffer.concat([Buffer.from(begin.bodyHex, "hex").subarray(0, -1), uint(1154)]);
    await expect(collect(Buffer.concat([prefix, frame(2, invalidBegin), ...tail.slice(1)]))).rejects.toThrow();
    const cursor = await factory();
    expect(() => cursor.push(0)).toThrow(/return-content/);
    expect(cursor.failure).not.toBeNull();
    expect(() => cursor.push(0x73)).toThrow(/return-content/);
    expect(cursor.complete).toBe(false);
  });
}
//#endregion 🧪️ContentFramingLaws

//#region 🧪️OperationHeaderLaws
if (import.meta.vitest) {
  const { it, expect, vi } = import.meta.vitest;
  const factory = async (length: bigint) => new (await import("./🟦️.ts")).KernelReturnUiOperationHeader(length);
  const oracle = async () => {
    const name = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(name);
    const encode = (module.default ?? module).encodeUIntBuffer;
    return (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
  };
  it("KernelReturnUiOperationHeader selects all eleven grammar fields without reading payload", async () => {
    const { default: fixture } = await import("./📥️input/🧪️fixture/🔣️.json");
    const { default: schema } = await import("./📥️input/🧪️schema/🔣️.json");
    const { default: wire } = await import("./🧬️wire/🔣️.json");
    const { default: Ajv } = await import("ajv");
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const uint = await oracle();
    for (const row of fixture.vectors) {
      const payload = Buffer.from(row.payloadHex, "hex");
      const header = Buffer.concat([Buffer.of(row.opcode), ...(row.node === null ? [] : [uint(BigInt(row.node))]), ...(row.field !== null && row.field !== "children" ? [uint(payload.length)] : [])]);
      expect(header.toString("hex")).toBe(row.headerHex);
      expect(Buffer.concat([header, payload]).toString("hex")).toBe(row.bodyHex);
      expect(wire.uiOperations.find(operation => operation.opcode === row.opcode)?.name).toBe(row.name);
      for (let split = 0; split <= header.length; split++) {
        const cursor = await factory(BigInt(header.length + payload.length));
        for (const byte of header.subarray(0, split)) cursor.push(byte);
        for (const byte of header.subarray(split)) cursor.push(byte);
        cursor.finish();
        expect(cursor.value).toEqual({ opcode: row.opcode, node: row.node === null ? null : BigInt(row.node), field: row.field, payloadLength: BigInt(payload.length), headerLength: header.length });
        expect(Object.isFrozen(cursor.value)).toBe(true);
        expect(cursor.failure).toBeNull();
        expect(() => cursor.push(payload[0] ?? 0)).toThrow(/header-complete/);
      }
    }
  });
  it("KernelReturnUiOperationHeader rejects malformed authority-free prefixes with sticky faults", async () => {
    const { default: fixture } = await import("./📥️input/🧪️fixture/🔣️.json");
    for (const row of fixture.invalid) {
      const cursor = await factory(BigInt(row.length));
      expect(() => { for (const byte of Buffer.from(row.headerHex, "hex")) cursor.push(byte); cursor.finish(); }).toThrow();
      expect(cursor.failure).not.toBeNull();
      expect(() => cursor.push(0)).toThrow();
    }
    for (const length of [-1n, 0n, 1n, 0x10000000000000000n]) await expect(factory(length)).rejects.toThrow(/length/);
    for (const byte of [-1, 256, 1.5, NaN]) {
      const cursor = await factory(2n);
      expect(() => cursor.push(byte)).toThrow(/byte/);
    }
    for (const row of fixture.vectors) {
      const header = Buffer.from(row.headerHex, "hex");
      for (let end = 0; end < header.length; end++) {
        const cursor = await factory(BigInt(row.bodyHex.length / 2));
        for (const byte of header.subarray(0, end)) cursor.push(byte);
        expect(() => cursor.finish()).toThrow(/truncated/);
      }
    }
  });
  it("KernelReturnUiOperationHeader retains children count and refuses allocation from u64 length", async () => {
    const { default: fixture } = await import("./📥️input/🧪️fixture/🔣️.json");
    const { default: shared } = await import("./🧪️fixture/🔣️.json");
    const uint = await oracle();
    const children = shared.scalarOperationVectors.find(row => row.name === "setChildren")!;
    const encoded = Buffer.concat([Buffer.of(children.opcode), uint(BigInt(children.node)), uint(children.children!.length), ...children.children!.map(node => uint(BigInt(node)))]);
    expect(encoded.toString("hex")).toBe(children.hex);
    const childCursor = await factory(BigInt(encoded.length));
    let read = 0;
    while (childCursor.value === null) childCursor.push(encoded[read++]!);
    expect(encoded.subarray(read).toString("hex")).toBe(fixture.vectors.find(row => row.opcode === 4)!.payloadHex);
    const cursor = await factory(BigInt(fixture.large.length));
    const bytes = Buffer.from(fixture.large.headerHex, "hex");
    const allocations: unknown[] = [];
    const original = Uint8Array;
    vi.stubGlobal("Uint8Array", new Proxy(original, { construct(target, args, newTarget) { allocations.push(args[0]); return Reflect.construct(target, args, newTarget); } }));
    try {
      for (const byte of bytes) cursor.push(byte);
      cursor.finish();
      expect(cursor.value?.payloadLength).toBe(BigInt(fixture.large.payloadLength));
      expect(allocations.length).toBe(fixture.large.allocations);
    } finally { vi.unstubAllGlobals(); }
  });
}
//#endregion 🧪️OperationHeaderLaws
