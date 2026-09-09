type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { KernelReturnContentFraming } = dependencies;

  const { it, expect, vi } = vitest;
  const factory = async () => new (await import("../../🟦️.ts")).KernelReturnContentFraming();
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
    const { default: wire } = await import("../../🔌️wire/🔣️.json");
    const { default: schema } = await import("../../🧬️schema/🔣️.json");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: pageSchema } = await import("../../../../../🎭️actor/📃️page/🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../../../🎭️actor/🚪️lifetime/🧬️schema/🔣️.json");
    const { default: patchSchema } = await import("../../../../../🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json");
    const { default: valueSchema } = await import("../../../../../🌱️value/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(pageSchema).addSchema(lifetimeSchema).addSchema(patchSchema);
    expect(ajv.addSchema(schema).getSchema(`${schema.$id}#/$defs/Content`)!(wire)).toBe(true);
    expect(ajv.getSchema(`${schema.$id}#/$defs/ContentFixture`)!(fixture)).toBe(true);
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
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
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
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
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
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
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
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
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

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { KernelReturnUiOperationHeader } = dependencies;

  const { it, expect, vi } = vitest;
  const factory = async (length: bigint) => new (await import("../../🟦️.ts")).KernelReturnUiOperationHeader(length);
  const oracle = async () => {
    const name = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(name);
    const encode = (module.default ?? module).encodeUIntBuffer;
    return (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
  };
  it("KernelReturnUiOperationHeader selects all eleven grammar fields without reading payload", async () => {
    const { default: fixture } = await import("../../📥️input/🧫️fixtures/🔣️.json");
    const { default: schema } = await import("../../📥️input/🧬️schema/🔣️.json");
    const { default: wire } = await import("../../🔌️wire/🔣️.json");
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
    const { default: fixture } = await import("../../📥️input/🧫️fixtures/🔣️.json");
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
    const { default: fixture } = await import("../../📥️input/🧫️fixtures/🔣️.json");
    const { default: shared } = await import("../../🧫️fixtures/🔣️.json");
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
