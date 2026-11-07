//#region 📨️ActorInputWire
/** 📨️ Encodes the one `reactor.admit-actor-input` payload as the repo's container-less pack value (`store::pack_rt::encode_wire_value`).
 *
 * Language-agnostic owner: `🔌️plugin/🧬️schema/🎟️actor-input/🧬️schema/🔣️.json` declares the decoded form and `🧫️fixtures/🔣️.json`
 * pins the exact bytes of every case; the defining Rust owner is `ActorInputWire`. Self-contained on purpose: the generated component
 * bridge and the document actor worker embed it by source, so it holds no import. Every number is an exact unsigned integer
 * (`TAG_UINT`), never `TAG_F64`, because the guest decodes through `FromValue`, whose unsigned arm refuses a float. */
export function createActorInputWireV1() {
  const text = new TextEncoder();
  const compare = (left: Uint8Array, right: Uint8Array): number => {
    const length = Math.min(left.length, right.length);
    for (let at = 0; at < length; at++) if (left[at] !== right[at]) return left[at]! - right[at]!;
    return left.length - right.length;
  };
  const varint = (out: number[], value: number | bigint): void => {
    let rest = BigInt(value);
    for (;;) {
      const byte = Number(rest & 0x7fn);
      rest >>= 7n;
      if (rest === 0n) { out.push(byte); return; }
      out.push(byte | 0x80);
    }
  };
  const collect = (value: unknown, counts: Map<string, number>): void => {
    if (typeof value === "string") counts.set(value, (counts.get(value) ?? 0) + 1);
    else if (Array.isArray(value)) for (const item of value) collect(item, counts);
    else if (value !== null && typeof value === "object" && !ArrayBuffer.isView(value)) for (const item of Object.values(value)) collect(item, counts);
  };
  const inline = (out: number[], value: string): void => {
    const bytes = text.encode(value);
    out.push(0x07);
    varint(out, bytes.length);
    for (const byte of bytes) out.push(byte);
  };
  const write = (value: unknown, index: Map<string, number>, out: number[]): void => {
    if (value === null || value === undefined) { out.push(0x12); return; }
    if (typeof value === "boolean") { out.push(value ? 0x02 : 0x01); return; }
    if (typeof value === "string") {
      const at = index.get(value);
      if (at === undefined) inline(out, value);
      else { out.push(0x06); varint(out, at); }
      return;
    }
    if (value instanceof Uint8Array) {
      out.push(0x0c);
      varint(out, value.length);
      for (const byte of value) { out.push(0x04); varint(out, byte); }
      return;
    }
    if (Array.isArray(value)) {
      out.push(0x0c);
      varint(out, value.length);
      for (const item of value) write(item, index, out);
      return;
    }
    const exact = typeof value === "bigint" ? value : typeof value === "number" && Number.isSafeInteger(value) ? BigInt(value) : undefined;
    if (exact !== undefined) {
      if (exact < 0n || exact > 0xffffffffffffffffn) throw new Error("actor-input.integer");
      out.push(0x04);
      varint(out, exact);
      return;
    }
    if (typeof value === "object") {
      const entries = Object.entries(value).map(([key, item]) => [text.encode(key), key, item] as const).sort((left, right) => compare(left[0], right[0]));
      out.push(0x10);
      varint(out, entries.length);
      for (const [, key, item] of entries) { inline(out, key); write(item, index, out); }
      return;
    }
    throw new Error("actor-input.value");
  };
  const lift = (value: unknown): unknown => {
    if (value === undefined || value === null) return null;
    if (value instanceof Uint8Array) return value;
    if (Array.isArray(value)) return value.map(lift);
    if (typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, lift(item)]));
    return value;
  };
  return {
    /** 📤️ The pack bytes of one decoded wire value. */
    encode(value: unknown): Uint8Array {
      const counts = new Map<string, number>();
      collect(value, counts);
      const interned = [...counts].filter(([symbol, count]) => text.encode(symbol).length <= 128 || count >= 2).map(([symbol]) => text.encode(symbol)).sort(compare);
      const index = new Map(interned.map((bytes, at) => [new TextDecoder().decode(bytes), at] as const));
      const out: number[] = [];
      varint(out, interned.length);
      for (const bytes of interned) { varint(out, bytes.length); for (const byte of bytes) out.push(byte); }
      varint(out, 1);
      varint(out, 1);
      out.push(0x11);
      write(value, index, out);
      return new Uint8Array(out);
    },
    /** 🎟️ A jco-lowered WIT `event` (`{ tag, val }`, camelCase fields) as a wire value: absent optionals become null, bytes stay bytes. */
    event: lift,
  };
}

/** 🧵️ Emits the complete lexical scope so compiler identifier changes cannot split the wire linkage. */
export function actorInputWireSourceV1(): string {
  return "const actorInputWire=(" + createActorInputWireV1.toString() + ")();";
}
//#endregion 📨️ActorInputWire
