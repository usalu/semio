//#region 🚪️InstanceCloseWire
export type ActorInstanceLifetime = { readonly activationGeneration: bigint; readonly instanceId: number };
export type ActorInstanceCloseRequest = { readonly kind: "close"; readonly lifetime: ActorInstanceLifetime; readonly requestSequence: number };
export type ActorInstanceCloseReceipt = { readonly kind: "accepted" | "retired"; readonly lifetime: ActorInstanceLifetime; readonly requestSequence: number; readonly closeGeneration: bigint };
export type ActorInstanceCloseWire = ActorInstanceCloseRequest | ActorInstanceCloseReceipt;

export const ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES = 34;

/** 📤️ Encodes only fixed-width close authority in canonical unsigned LEB128. */
export function encodeActorInstanceClose(value: ActorInstanceCloseWire): Uint8Array {
  const generation = value.lifetime.activationGeneration;
  const instance = value.lifetime.instanceId;
  if (generation <= 0n || generation > 0xffffffffffffffffn || !Number.isInteger(instance) || instance < 0 || instance > 0xffffffff || !Number.isSafeInteger(value.requestSequence) || value.requestSequence <= 0) throw new Error("actor-close.invalid-authority");
  if (value.kind !== "close" && (value.closeGeneration <= 0n || value.closeGeneration > 0xffffffffffffffffn)) throw new Error("actor-close.invalid-close-generation");
  const output = new Uint8Array(ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES);
  let length = 0;
  const put = (initial: bigint) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  output[length++] = value.kind === "close" ? 0 : value.kind === "accepted" ? 1 : 2;
  put(generation); put(BigInt(instance)); put(BigInt(value.requestSequence));
  if (value.kind !== "close") put(value.closeGeneration);
  return output.slice(0, length);
}

/** 📥️ Rejects trailing, truncated, noncanonical, overflowed, and zero close authority before dispatch. */
export function decodeActorInstanceClose(bytes: Uint8Array): ActorInstanceCloseWire {
  if (bytes.length === 0 || bytes.length > ACTOR_INSTANCE_CLOSE_MAXIMUM_BYTES) throw new Error("actor-close.envelope");
  const kind = bytes[0];
  if (kind !== 0 && kind !== 1 && kind !== 2) throw new Error("actor-close.tag");
  let offset = 1;
  const get = (maximum: bigint, nonzero: boolean) => {
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined) throw new Error("actor-close.truncated");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if ((index !== 0 && byte === 0) || value > maximum || (nonzero && value === 0n)) throw new Error("actor-close.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-close.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const requestSequence = Number(get(BigInt(Number.MAX_SAFE_INTEGER), true));
  const lifetime = { activationGeneration, instanceId };
  const value: ActorInstanceCloseWire = kind === 0 ? { kind: "close", lifetime, requestSequence } : { kind: kind === 1 ? "accepted" : "retired", lifetime, requestSequence, closeGeneration: get(0xffffffffffffffffn, true) };
  if (offset !== bytes.length) throw new Error("actor-close.trailing");
  return value;
}

/** 🪪️ Compares fixed activation authority, never a fresh actor-name lookup. */
export function actorInstanceLifetimeEquals(left: ActorInstanceLifetime, right: ActorInstanceLifetime): boolean {
  return left.activationGeneration === right.activationGeneration && left.instanceId === right.instanceId;
}

/** 📨️ Binds the receipt's wire identity only; native descendant terminal authority is not manufactured here. */
export function actorInstanceCloseReceiptMatches(request: ActorInstanceCloseRequest, accepted: ActorInstanceCloseReceipt | null, receipt: ActorInstanceCloseReceipt): boolean {
  return actorInstanceLifetimeEquals(request.lifetime, receipt.lifetime) && request.requestSequence === receipt.requestSequence && (accepted === null ? receipt.kind === "accepted" : accepted.kind === "accepted" && actorInstanceLifetimeEquals(accepted.lifetime, receipt.lifetime) && accepted.requestSequence === receipt.requestSequence && accepted.closeGeneration === receipt.closeGeneration);
}

//#region 🧪️WireLaws
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;
  it("actor instance close fault publication fixture preserves watchdog and terminal-outcome precedence", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fault.fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧪️fault.schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(validate({ ...fixture, callbackLimitUs: 8001 })).toBe(false);
    expect(validate({ ...fixture, owners: { ...fixture.owners, forgottenPayloads: 1 } })).toBe(false);
    const name = "lodash-es/gte.js";
    const module: unknown = await import(name);
    const greater: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof greater !== "function") throw new Error("invalid independent elapsed comparison oracle");
    for (const row of fixture.callbacks) expect(greater(row.elapsedUs, fixture.callbackLimitUs) ? "fault" : row.candidate).toBe(row.published);
    for (const row of fixture.clocks) {
      const [start, preflight, finish] = row.samples;
      const entered = typeof start === "number" && typeof preflight === "number" && preflight >= start;
      expect(entered).toBe(row.workEntered);
      expect(!entered || typeof finish !== "number" || finish < preflight || greater(finish - start, fixture.callbackLimitUs) ? "fault" : "complete").toBe(row.published);
    }
    for (const row of fixture.terminalPump) expect(row.faulted ? "fault" : row.blocked ? "external-wait" : row.complete ? "complete" : "ready").toBe(row.status);
  });

  it("actor instance close native value fixture accounts exact descendant text and independent cloned structure", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧪️fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧪️schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(validate({ ...fixture, grants: [0, 4096] })).toBe(false);
    expect(validate({ ...fixture, ownership: { ...fixture.ownership, nestedRootTerminalBeforeDescendants: true } })).toBe(false);
    const name = "lodash-es/cloneDeepWith.js";
    const module: unknown = await import(name);
    const clone: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof clone !== "function") throw new Error("invalid independent descendant traversal oracle");
    for (const row of fixture.cases) {
      const measured = { textBytes: 0, pages: 0, collections: 0 };
      const stack: unknown[] = [row.value];
      while (stack.length) {
        const value = stack.pop();
        if (typeof value === "string") measured.textBytes += new TextEncoder().encode(value).length;
        else if (value && typeof value === "object") {
          const entries = Object.entries(value);
          measured.pages += entries.length;
          measured.collections += Number(entries.length !== 0);
          for (const [key, child] of entries) {
            if (!Array.isArray(value)) measured.textBytes += new TextEncoder().encode(key).length;
            stack.push(child);
          }
        }
      }
      const oracle = { textBytes: 0, pages: 0, collections: 0 };
      const copied: unknown = clone(row.value, (value: unknown, key: unknown, parent: unknown) => {
        if (parent && typeof parent === "object" && !Array.isArray(parent)) oracle.textBytes += Buffer.byteLength(String(key), "utf8");
        if (typeof value === "string") oracle.textBytes += Buffer.byteLength(value, "utf8");
        else if (value && typeof value === "object") {
          const size = Object.keys(value).length;
          oracle.pages += size;
          oracle.collections += Number(size !== 0);
        }
        return undefined;
      });
      expect(copied).toEqual(row.value);
      expect(measured).toEqual(oracle);
      expect(measured).toEqual({ textBytes: row.textBytes, pages: row.pages, collections: row.collections });
    }
  });

  it("actor instance close fixed-list fixture preserves ordered payload handoff", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧪️fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧪️schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, capacity: 5 })).toBe(false);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    expect(validate({ ...fixture, ownership: { ...fixture.ownership, releaseWithPayloadAccepted: true } })).toBe(false);
    expect(validate({ ...fixture, cases: [...fixture.cases, { name: "overflow", values: [1, 2, 3, 4, 5], popped: [] }] })).toBe(false);
    const moduleName = "lodash-es/reverse.js";
    const module: unknown = await import(moduleName);
    const reverse: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof reverse !== "function") throw new Error("invalid independent sequence oracle");
    for (const row of fixture.cases) {
      const remaining: unknown[] = [...row.values];
      const moved: unknown[] = [];
      while (remaining.length) moved.push(remaining.pop());
      expect(moved).toEqual(row.popped);
      expect(reverse([...row.values])).toEqual(row.popped);
      expect(JSON.parse(JSON.stringify(row.values))).toEqual(row.values);
    }
  });

  it("actor typed descendant fixture covers the exact component and patch rosters", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, componentVariants: fixture.componentVariants.slice(1) })).toBe(false);
    expect(validate({ ...fixture, patchVariants: [...fixture.patchVariants, "invented"] })).toBe(false);
    expect(validate({ ...fixture, ownership: { ...fixture.ownership, arenaContentionAdvances: true } })).toBe(false);
    expect(validate({ ...fixture, document: { ...fixture.document, terminalDescendantsRetired: false } })).toBe(false);
    const encode = new TextEncoder();
    const bytes = (value: unknown): number => typeof value === "string" ? encode.encode(value).length : Array.isArray(value) ? value.reduce((sum, child) => sum + bytes(child), 0) : value && typeof value === "object" ? Object.entries(value).reduce((sum, [key, child]) => sum + encode.encode(key).length + bytes(child), 0) : 0;
    const moduleName = "lodash-es/toPairs.js";
    const module: unknown = await import(moduleName);
    const pairs: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof pairs !== "function") throw new Error("invalid independent object traversal oracle");
    const pending: unknown[] = [fixture.document.value];
    let oracleBytes = 0;
    while (pending.length) {
      const value = pending.pop();
      if (typeof value === "string") oracleBytes += Buffer.byteLength(value);
      else if (Array.isArray(value)) pending.push(...value);
      else if (value && typeof value === "object") {
        const entries: unknown = pairs(value);
        if (!Array.isArray(entries)) throw new Error("invalid independent object entries");
        for (const entry of entries) {
          if (!Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== "string") throw new Error("invalid independent object pair");
          oracleBytes += Buffer.byteLength(entry[0]);
          pending.push(entry[1]);
        }
      }
    }
    expect(bytes(fixture.document.value)).toBe(fixture.document.valueTextBytes);
    expect(oracleBytes).toBe(fixture.document.valueTextBytes);
    const components = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️components.json", import.meta.url), "utf8"));
    const componentSchema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️components.schema.json", import.meta.url), "utf8"));
    const validateComponents = new Ajv({ strict: true }).compile(componentSchema);
    expect(validateComponents(components)).toBe(true);
    expect(validateComponents({ ...components, cases: components.cases.slice(1) })).toBe(false);
    expect(validateComponents({ ...components, cases: components.cases.map((row: { component: object }, index: number) => index ? row : { ...row, component: { ...row.component, extra: 1 } }) })).toBe(false);
    expect(components.cases.map((row: { component: { type: string } }) => row.component.type)).toEqual(fixture.componentVariants);
    const enumFields = new Set(["type", "role", "kind", "trigger", "placement"]);
    const valueFields = new Set(["props", "args", "input", "dragData", "dataAttributes"]);
    const semanticBytes = (value: unknown, key = "", raw = false): number => {
      if (typeof value === "string") return !raw && enumFields.has(key) ? 0 : Buffer.byteLength(value);
      if (Array.isArray(value)) return !raw && key === "bytes" ? value.length : value.reduce((sum, child) => sum + semanticBytes(child, "", raw), 0);
      if (!value || typeof value !== "object") return 0;
      const entries: unknown = pairs(value);
      if (!Array.isArray(entries)) throw new Error("invalid typed component oracle");
      return entries.reduce((sum: number, entry: unknown) => {
        if (!Array.isArray(entry) || entry.length !== 2 || typeof entry[0] !== "string") throw new Error("invalid typed component pair");
        return sum + (raw ? Buffer.byteLength(entry[0]) : 0) + semanticBytes(entry[1], entry[0], raw || valueFields.has(entry[0]));
      }, 0);
    };
    for (const row of components.cases) expect(semanticBytes(row.component), row.component.type).toBe(row.bytes);
  });

  it("actor arena handback fixture preserves exact fair obligations across word boundaries", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧪️fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧪️schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, slots: 255 })).toBe(false);
    expect(validate({ ...fixture, expectedOrder: fixture.expectedOrder.slice(1) })).toBe(false);
    expect(validate({ ...fixture, emptyReadyBitConsumesNewOwner: true })).toBe(false);
    expect(validate({ ...fixture, rejectedObligationRetained: false })).toBe(false);
    const counts = new Map<number, number>();
    for (const owner of fixture.obligations) counts.set(owner.slot, (counts.get(owner.slot) ?? 0) + 1);
    const moduleName = "lodash-es/groupBy.js";
    const module: unknown = await import(moduleName);
    const groupBy: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof groupBy !== "function") throw new Error("invalid independent handback grouping oracle");
    const groups: unknown = groupBy(fixture.obligations, "slot");
    if (!groups || typeof groups !== "object") throw new Error("invalid independent handback groups");
    for (const [slot, count] of counts) {
      const group: unknown = Reflect.get(groups, String(slot));
      if (!Array.isArray(group)) throw new Error("invalid independent handback group");
      expect(group.length).toBe(count);
    }
    const order: number[] = [];
    let cursor = fixture.start;
    while (counts.size) {
      const slots = [...counts.keys()].sort((a, b) => a - b);
      const slot = slots.find(slot => slot >= cursor) ?? slots[0];
      const count = counts.get(slot)!;
      if (count === 1) counts.delete(slot); else counts.set(slot, count - 1);
      order.push(slot);
      cursor = (slot + 1) % fixture.slots;
    }
    expect(order).toEqual(fixture.expectedOrder);
  });

  it("actor instance close wire matches strict shared fixtures and an independent LEB128 encoder", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema.json", import.meta.url), "utf8"));
    const fixtureSchema = JSON.parse(readFileSync(new URL("./🧪️schema.json", import.meta.url), "utf8"));
    const ajv = new Ajv({ strict: true });
    ajv.addSchema(schema);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
    const validate = ajv.compile(schema);
    for (const invalid of ["0", "-1", "01", "18446744073709551616"]) expect(validate({ ...fixture.vectors[0].value, lifetime: { activationGeneration: invalid, instanceId: 7 } })).toBe(false);
    expect(validate({ ...fixture.vectors[0].value, unexpected: true })).toBe(false);
    const dropModule = "lodash-es/drop.js";
    const importedDrop: unknown = await import(dropModule);
    if (!importedDrop || typeof importedDrop !== "object") throw new Error("invalid ordered owner oracle module");
    const drop: unknown = Reflect.get(importedDrop, "default");
    if (typeof drop !== "function") throw new Error("invalid ordered owner oracle interface");
    expect(fixture.publishedClose.complete.map((_value: boolean, index: number) => drop(fixture.publishedClose.owners, index + 1))).toEqual(fixture.publishedClose.remaining);
    expect(fixture.publishedClose.remaining.map((_value: unknown, index: number) => index >= fixture.publishedClose.owners.length)).toEqual(fixture.publishedClose.complete);
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported: unknown = await import(oracleModule);
    if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
    const oracle: unknown = Reflect.get(imported, "default");
    if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
    const encoder: unknown = Reflect.get(oracle, "encodeUIntBuffer");
    if (typeof encoder !== "function") throw new Error("missing LEB128 oracle encoder");
    const u64 = (value: bigint): number[] => {
      const input = Buffer.alloc(8); input.writeBigUInt64LE(value);
      const output: unknown = encoder(input);
      if (!(output instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
      return Array.from(output);
    };
    for (const row of fixture.vectors) {
      const lifetime = { ...row.value.lifetime, activationGeneration: BigInt(row.value.lifetime.activationGeneration) };
      const value: ActorInstanceCloseWire = row.value.kind === "close" ? { ...row.value, lifetime } : { ...row.value, lifetime, closeGeneration: BigInt(row.value.closeGeneration) };
      const bytes = encodeActorInstanceClose(value);
      const expected = [value.kind === "close" ? 0 : value.kind === "accepted" ? 1 : 2, ...u64(lifetime.activationGeneration), ...u64(BigInt(lifetime.instanceId)), ...u64(BigInt(value.requestSequence)), ...(value.kind === "close" ? [] : u64(value.closeGeneration))];
      expect(Array.from(bytes)).toEqual(expected);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(decodeActorInstanceClose(bytes)).toEqual(value);
      expect(() => decodeActorInstanceClose(Uint8Array.from([...bytes, 0]))).toThrow();
      for (let length = 0; length < bytes.length; length += 1) expect(() => decodeActorInstanceClose(bytes.subarray(0, length))).toThrow();
    }
    for (const bytes of [[3, 1, 7, 9], [0, 0, 7, 9], [0, 0x81, 0, 7, 9], [0, ...Array(10).fill(255), 7, 9], [0, 1, 7, 0]]) expect(() => decodeActorInstanceClose(Uint8Array.from(bytes))).toThrow();
  });

  it("actor instance close receipts reject reused IDs and premature terminal messages", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture.json", import.meta.url), "utf8"));
    const prior = { ...fixture.reopen.prior, activationGeneration: BigInt(fixture.reopen.prior.activationGeneration) };
    const current = { ...fixture.reopen.current, activationGeneration: BigInt(fixture.reopen.current.activationGeneration) };
    const request: ActorInstanceCloseRequest = { kind: "close", lifetime: current, requestSequence: 9 };
    const accepted: ActorInstanceCloseReceipt = { kind: "accepted", lifetime: current, requestSequence: 9, closeGeneration: 13n };
    expect(actorInstanceLifetimeEquals(prior, current)).toBe(fixture.reopen.oldRequestAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, lifetime: prior })).toBe(fixture.reopen.oldReceiptAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, kind: "retired" })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, null, accepted)).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired" })).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", closeGeneration: 12n })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", requestSequence: 8 })).toBe(false);
  });

  it("actor instance close worker activation preserves the new generation across a delayed dispose", async () => {
    const { readFileSync } = await import("node:fs");
    const { shardWorkerSource } = await import("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture.json", import.meta.url), "utf8"));
    const prior = BigInt(fixture.reopen.prior.activationGeneration);
    const current = BigInt(fixture.reopen.current.activationGeneration);
    const { Worker } = await import("node:worker_threads");
    type Message = { readonly kind: string; readonly requestId?: string; readonly ok?: boolean; readonly value?: unknown; readonly error?: string };
    const pending = new Map<string, { resolve: (value: Message) => void; reject: (error: Error) => void }>();
    const worker = new Worker("const { parentPort } = require('node:worker_threads'); const self = { postMessage: value => parentPort.postMessage(value), addEventListener: (_, callback) => parentPort.on('message', data => callback({ data })) }; const WebAssembly = { Suspending: function(){}, promising: function(){} };\n" + shardWorkerSource(), { eval: true });
    worker.on("message", (message: Message) => {
      if (message.kind !== "result" || !message.requestId) return;
      const waiting = pending.get(message.requestId);
      if (waiting) { pending.delete(message.requestId); waiting.resolve(message); }
    });
    worker.on("error", (error) => { for (const waiting of pending.values()) waiting.reject(error); pending.clear(); });
    const moduleUrl = "data:text/javascript," + encodeURIComponent("export async function createActorApi(actorId, activationGeneration) { return { poll: async () => ({ actorId, activationGeneration }) }; }");
    const send = (data: unknown, requestId: string): Promise<Message> => new Promise((resolve, reject) => { pending.set(requestId, { resolve, reject }); worker.postMessage(data); });
    try {
      expect(await send({ kind: "activate", requestId: "a1", actorId: "same", activationGeneration: prior, moduleUrl, assets: [] }, "a1")).toMatchObject({ ok: true });
      expect((await send({ kind: "turn", requestId: "t1", actorId: "same", events: [], budget: {} }, "t1")).value).toEqual({ actorId: "same", activationGeneration: prior });
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
      expect(await send({ kind: "activate", requestId: "a2", actorId: "same", activationGeneration: current, moduleUrl, assets: [] }, "a2")).toMatchObject({ ok: true });
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
      expect((await send({ kind: "turn", requestId: "t2", actorId: "same", events: [], budget: {} }, "t2")).value).toEqual({ actorId: "same", activationGeneration: current });
      expect((await send({ kind: "activate", requestId: "old", actorId: "old", activationGeneration: prior, moduleUrl, assets: [] }, "old")).ok).toBe(false);
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: current });
    } finally {
      await worker.terminate();
    }
  });
}
//#endregion 🧪️WireLaws
//#endregion 🚪️InstanceCloseWire
