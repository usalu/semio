type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, readActorBytePage } = dependencies;
  type ActorBytePage = any;

  const { it, expect, vi } = vitest;

  it("ActorBytePage matches shared vectors and Node Buffer for every fixed word", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const { default: schema } = await import("../../🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    expect(ajv.getSchema(`${schema.$id}#/$defs/PageFixture`)!(fixture)).toBe(true);
    const validate = ajv.getSchema(`${schema.$id}#/$defs/Page`)!;
    expect(ACTOR_BYTE_PAGE_BYTES).toBe(fixture.maximumBytes);
    for (const row of fixture.vectors) {
      const backing = new Uint8Array(row.length + 13).fill(255);
      const input = backing.subarray(7, 7 + row.length);
      for (let index = 0; index < input.length; index++) input[index] = (index * fixture.bytePattern.multiplier + fixture.bytePattern.addend) % fixture.bytePattern.modulus;
      const original = input.slice();
      const page = createActorBytePage(input);
      const oracle = Buffer.alloc(fixture.maximumBytes, fixture.canonicalTailByte);
      oracle.set(input);
      const json = JSON.parse(JSON.stringify(page, (_key, value) => typeof value === "bigint" ? value.toString() : value));
      expect(validate(json)).toBe(true);
      expect(Object.keys(page)).toHaveLength(fixture.blockCount + 1);
      expect(page.length).toBe(row.length);
      expect(Object.isFrozen(page)).toBe(true);
      expect(page.block00.word0.toString()).toBe(row.firstWord);
      const lastWord = Math.max(0, Math.ceil(row.length / 8) - 1);
      for (let blockIndex = 0; blockIndex < fixture.blockCount; blockIndex++) {
        const block = Reflect.get(page, `block${blockIndex.toString().padStart(2, "0")}`);
        expect(Object.keys(block)).toHaveLength(fixture.wordsPerBlock);
        expect(Object.isFrozen(block)).toBe(true);
        for (let wordIndex = 0; wordIndex < fixture.wordsPerBlock; wordIndex++) {
          const word = Reflect.get(block, `word${wordIndex}`);
          expect(word).toBe(oracle.readBigUInt64LE((blockIndex * 8 + wordIndex) * 8));
          if (blockIndex * 8 + wordIndex === lastWord) expect(word.toString()).toBe(row.lastUsedWord);
        }
      }
      expect(readActorBytePage(page)).toEqual(original);
      input.fill(0); expect(readActorBytePage(page)).toEqual(original);
      expect(validate({ ...json, block64: json.block00 })).toBe(false);
      expect(validate({ ...json, unknown: "retained-wrapper" })).toBe(false);
      expect(validate({ ...json, block00: { ...json.block00, word8: "0" } })).toBe(false);
      for (const value of fixture.invalidWords) expect(validate({ ...json, block00: { ...json.block00, word0: value } })).toBe(false);
    }
  });

  it("ActorBytePage rejects invalid selected fields and nonzero padding without invoking getters", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    for (const length of [...fixture.invalidLengths, NaN, Infinity, "1", null, undefined]) {
      const page = { ...createActorBytePage(new Uint8Array()), length };
      expect(() => readActorBytePage(page as ActorBytePage)).toThrow();
    }
    for (const value of [-1n, 0x10000000000000000n, 1, "1", null, undefined]) {
      const page = createActorBytePage(new Uint8Array(8));
      expect(() => readActorBytePage({ ...page, block00: { ...page.block00, word0: value } } as ActorBytePage)).toThrow();
    }
    const maximum = createActorBytePage(new Uint8Array(8).fill(255));
    expect(maximum.block00.word0).toBe(0xffffffffffffffffn);
    expect(Buffer.from(readActorBytePage(maximum)).readBigUInt64LE()).toBe(0xffffffffffffffffn);
    for (const row of fixture.padding) {
      const page = structuredClone(createActorBytePage(new Uint8Array(row.length)));
      const block = Reflect.get(page, `block${Math.floor(row.byteOffset / 64).toString().padStart(2, "0")}`);
      Reflect.set(block, `word${Math.floor(row.byteOffset % 64 / 8)}`, BigInt(row.value) << BigInt(row.byteOffset % 8 * 8));
      if (row.accepted) expect(readActorBytePage(page)[row.byteOffset]).toBe(row.value);
      else expect(() => readActorBytePage(page)).toThrow("actor-byte-page.padding");
    }
    let reads = 0;
    for (const key of ["length", "block00", "block63"]) {
      const page = structuredClone(createActorBytePage(new Uint8Array()));
      Reflect.deleteProperty(page, key); expect(() => readActorBytePage(page)).toThrow();
      Object.defineProperty(page, key, { get() { reads++; throw new Error("unowned getter"); } });
      expect(() => readActorBytePage(page)).toThrow();
    }
    const page = structuredClone(createActorBytePage(new Uint8Array()));
    Reflect.deleteProperty(page.block00, "word7"); expect(() => readActorBytePage(page)).toThrow();
    Object.defineProperty(page.block00, "word7", { get() { reads++; throw new Error("unowned getter"); } });
    expect(() => readActorBytePage(page)).toThrow(); expect(reads).toBe(0);
    expect(() => readActorBytePage(Object.create(createActorBytePage(new Uint8Array())))).toThrow();
    expect(() => createActorBytePage(new Uint8Array(fixture.maximumBytes + 1))).toThrow();
    expect(() => createActorBytePage([] as unknown as Uint8Array)).toThrow();
  });

  it("ActorBytePage performs only fixed selected reads and at most one payload allocation", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const input = new Uint8Array(fixture.maximumBytes).fill(255);
    const page = createActorBytePage(input);
    let enumerations = 0; let unknownReads = 0;
    const wrapper = Object.assign({}, page, { retainedUnknown: new Uint8Array(8192) });
    Object.defineProperty(wrapper, "foreign", { get() { unknownReads++; throw new Error("unknown wrapper"); } });
    const selected = new Proxy(wrapper, { ownKeys() { enumerations++; throw new Error("unbounded enumeration"); } });
    const allocations: number[] = [];
    const original = Uint8Array;
    const constructor = new Proxy(original, { construct(target, argumentsList, newTarget) {
      if (typeof argumentsList[0] !== "number") throw new Error("unexpected whole-buffer construction");
      allocations.push(argumentsList[0]); return Reflect.construct(target, argumentsList, newTarget);
    } });
    let result: Uint8Array;
    vi.stubGlobal("Uint8Array", constructor);
    try { result = readActorBytePage(selected); } finally { vi.unstubAllGlobals(); }
    expect(result!).toEqual(input);
    expect(allocations).toEqual([fixture.ownership.copyMaximumBytes]);
    expect(enumerations).toBe(0); expect(unknownReads).toBe(0);
    expect(wrapper.retainedUnknown.byteLength).toBe(8192);
    expect(readActorBytePage(page).buffer).not.toBe(input.buffer);
    expect(fixture.ownership).toEqual({ confersAuthority: false, buildsPageArray: false, readsUnknownKeys: false, copyMaximumBytes: 4096 });
  });

}
