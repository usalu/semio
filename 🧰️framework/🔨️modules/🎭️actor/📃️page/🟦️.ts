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
  const { registerTests1 } = await import("./🧪️tests/🧪️actorbytepage-matches-shared-vectors-and-node-buffer-for-every-fixed-wor/🟦️.ts");
  await registerTests1(import.meta.vitest, { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, readActorBytePage }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️BytePageLaws
