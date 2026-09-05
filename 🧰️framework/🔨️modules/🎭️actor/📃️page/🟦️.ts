//#region 📄️BytePageStorage
type Digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;
type BlockIndex = `${0 | 1 | 2 | 3 | 4 | 5}${Digit}` | `6${0 | 1 | 2 | 3}`;
export type ActorBytePageBlock = { readonly [key in `word${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7}`]: bigint };
export type ActorBytePage = { readonly length: number } & { readonly [key in `block${BlockIndex}`]: ActorBytePageBlock };
export const ACTOR_BYTE_PAGE_BYTES = 4096;

/** 📄️ Copies one admitted byte view into neutral fixed storage; this mints no transport authority. */
export function createActorBytePage(bytes: Uint8Array): ActorBytePage {
  if (!(bytes instanceof Uint8Array) || bytes.length > ACTOR_BYTE_PAGE_BYTES) throw new Error("actor-byte-page.input");
  const page: Record<string, number | ActorBytePageBlock> = { length: bytes.length };
  for (let blockIndex = 0; blockIndex < 64; blockIndex++) {
    const block: Record<string, bigint> = {};
    for (let wordIndex = 0; wordIndex < 8; wordIndex++) {
      let word = 0n;
      const start = blockIndex * 64 + wordIndex * 8;
      for (let byteIndex = 0; byteIndex < 8; byteIndex++) word |= BigInt(bytes[start + byteIndex] ?? 0) << BigInt(byteIndex * 8);
      block[`word${wordIndex}`] = word;
    }
    page[`block${blockIndex.toString().padStart(2, "0")}`] = Object.freeze(block) as ActorBytePageBlock;
  }
  return Object.freeze(page) as ActorBytePage;
}

/** 📥️ Reads fixed own data fields only; provenance, unknown wrappers and retirement remain caller-owned. */
export function readActorBytePage(page: ActorBytePage): Uint8Array {
  const field = (owner: unknown, key: string): unknown => {
    if (owner === null || typeof owner !== "object") throw new Error("actor-byte-page.field");
    const descriptor = Object.getOwnPropertyDescriptor(owner, key);
    if (!descriptor || !("value" in descriptor)) throw new Error("actor-byte-page.field");
    return descriptor.value;
  };
  const length = field(page, "length");
  if (typeof length !== "number" || !Number.isInteger(length) || length < 0 || length > ACTOR_BYTE_PAGE_BYTES) throw new Error("actor-byte-page.length");
  const bytes = new Uint8Array(length);
  for (let blockIndex = 0; blockIndex < 64; blockIndex++) {
    const block = field(page, `block${blockIndex.toString().padStart(2, "0")}`);
    for (let wordIndex = 0; wordIndex < 8; wordIndex++) {
      const word = field(block, `word${wordIndex}`);
      if (typeof word !== "bigint" || word < 0n || word > 0xffffffffffffffffn) throw new Error("actor-byte-page.word");
      const start = blockIndex * 64 + wordIndex * 8;
      for (let byteIndex = 0; byteIndex < 8; byteIndex++) {
        const byte = Number(word >> BigInt(byteIndex * 8) & 255n);
        const offset = start + byteIndex;
        if (offset < length) bytes[offset] = byte;
        else if (byte !== 0) throw new Error("actor-byte-page.padding");
      }
    }
  }
  return bytes;
}
//#endregion 📄️BytePageStorage

//#region 🧪️BytePageLaws
if (import.meta.vitest) {
  const { it, expect, vi } = import.meta.vitest;

  it("ActorBytePage matches shared vectors and Node Buffer for every fixed word", async () => {
    const { default: fixture } = await import("./🧫️fixture/🔣️.json");
    const { default: schema } = await import("./🧬️schema.json");
    const { default: fixtureSchema } = await import("./📐️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(schema);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
    const validate = ajv.compile(schema);
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
    const { default: fixture } = await import("./🧫️fixture/🔣️.json");
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
    const { default: fixture } = await import("./🧫️fixture/🔣️.json");
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
//#endregion 🧪️BytePageLaws
