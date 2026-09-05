//#region 🚪️InstanceLifecycleWire
export type ActorInstanceLifetime = { readonly activationGeneration: bigint; readonly instanceId: number; readonly guestLifetime: bigint };
export type ActorInstanceOpenRequest = { readonly kind: "open"; readonly activationGeneration: bigint; readonly instanceId: number; readonly requestSequence: number };
export type ActorInstanceCloseRequest = { readonly kind: "close"; readonly lifetime: ActorInstanceLifetime; readonly requestSequence: number };
export type ActorInstanceLifecycleReceipt =
  | { readonly kind: "captured"; readonly lifetime: ActorInstanceLifetime; readonly requestSequence: number }
  | { readonly kind: "accepted" | "retired"; readonly lifetime: ActorInstanceLifetime; readonly requestSequence: number; readonly closeGeneration: bigint };
export type ActorInstanceLifecycleAck = { readonly kind: "ack"; readonly receipt: ActorInstanceLifecycleReceipt };
export type ActorInstanceLifecycleWire = ActorInstanceOpenRequest | ActorInstanceCloseRequest | ActorInstanceLifecycleReceipt | ActorInstanceLifecycleAck;

export const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = 44;

/** 📤️ Encodes fixed lifecycle authority and exact receipt ACKs in canonical unsigned LEB128. */
export function encodeActorInstanceLifecycle(value: ActorInstanceLifecycleWire): Uint8Array {
  const body = value.kind === "ack" ? value.receipt : value;
  const tag = value.kind === "ack"
    ? body.kind === "captured" ? 5 : body.kind === "accepted" ? 6 : body.kind === "retired" ? 7 : -1
    : body.kind === "open" ? 0 : body.kind === "captured" ? 1 : body.kind === "close" ? 2 : body.kind === "accepted" ? 3 : body.kind === "retired" ? 4 : -1;
  if (tag === -1) throw new Error("actor-lifecycle.tag");
  const generation = body.kind === "open" ? body.activationGeneration : body.lifetime.activationGeneration;
  const instance = body.kind === "open" ? body.instanceId : body.lifetime.instanceId;
  const validGeneration = (field: bigint): boolean => typeof field === "bigint" && field > 0n && field <= 0xffffffffffffffffn;
  if (!validGeneration(generation) || !Number.isInteger(instance) || instance < 0 || instance > 0xffffffff || !Number.isSafeInteger(body.requestSequence) || body.requestSequence <= 0) throw new Error("actor-lifecycle.invalid-authority");
  if (body.kind !== "open" && !validGeneration(body.lifetime.guestLifetime)) throw new Error("actor-lifecycle.invalid-guest-lifetime");
  if ((body.kind === "accepted" || body.kind === "retired") && !validGeneration(body.closeGeneration)) throw new Error("actor-lifecycle.invalid-close-generation");
  const output = new Uint8Array(ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES);
  let length = 0;
  const put = (initial: bigint) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  output[length++] = tag;
  put(generation); put(BigInt(instance));
  if (body.kind !== "open") put(body.lifetime.guestLifetime);
  put(BigInt(body.requestSequence));
  if (body.kind === "accepted" || body.kind === "retired") put(body.closeGeneration);
  return output.slice(0, length);
}

/** 📥️ Rejects trailing, truncated, noncanonical, overflowed, and zero lifecycle authority before dispatch. */
export function decodeActorInstanceLifecycle(bytes: Uint8Array): ActorInstanceLifecycleWire {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES) throw new Error("actor-lifecycle.envelope");
  const kind = bytes[0];
  if (kind === undefined || kind > 7) throw new Error("actor-lifecycle.tag");
  let offset = 1;
  const get = (maximum: bigint, nonzero: boolean) => {
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined) throw new Error("actor-lifecycle.truncated");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if ((index !== 0 && byte === 0) || value > maximum || (nonzero && value === 0n)) throw new Error("actor-lifecycle.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-lifecycle.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const guestLifetime = kind === 0 ? null : get(0xffffffffffffffffn, true);
  const requestSequence = Number(get(BigInt(Number.MAX_SAFE_INTEGER), true));
  let value: ActorInstanceLifecycleWire;
  if (guestLifetime === null) value = { kind: "open", activationGeneration, instanceId, requestSequence };
  else {
    const lifetime = { activationGeneration, instanceId, guestLifetime };
    if (kind === 2) value = { kind: "close", lifetime, requestSequence };
    else {
      const receipt: ActorInstanceLifecycleReceipt = kind === 1 || kind === 5
        ? { kind: "captured", lifetime, requestSequence }
        : { kind: kind === 3 || kind === 6 ? "accepted" : "retired", lifetime, requestSequence, closeGeneration: get(0xffffffffffffffffn, true) };
      value = kind >= 5 ? { kind: "ack", receipt } : receipt;
    }
  }
  if (offset !== bytes.length) throw new Error("actor-lifecycle.trailing");
  return value;
}

/** 🪪️ Compares the captured guest lifetime as well as worker activation and numeric instance. */
export function actorInstanceLifetimeEquals(left: ActorInstanceLifetime, right: ActorInstanceLifetime): boolean {
  return left.activationGeneration === right.activationGeneration && left.instanceId === right.instanceId && left.guestLifetime === right.guestLifetime;
}

/** 🪞️ Requires ACK identity to equal the original receipt, including its phase and generation. */
export function actorInstanceLifecycleReceiptEquals(left: ActorInstanceLifecycleReceipt, right: ActorInstanceLifecycleReceipt): boolean {
  return left.kind === right.kind && actorInstanceLifetimeEquals(left.lifetime, right.lifetime) && left.requestSequence === right.requestSequence && (left.kind === "captured" || right.kind !== "captured" && left.closeGeneration === right.closeGeneration);
}

/** 🔓️ Correlates guest-issued capture with the exact pending open request. */
export function actorInstanceCapturedReceiptMatches(request: ActorInstanceOpenRequest, receipt: ActorInstanceLifecycleReceipt): boolean {
  return receipt.kind === "captured" && request.activationGeneration === receipt.lifetime.activationGeneration && request.instanceId === receipt.lifetime.instanceId && request.requestSequence === receipt.requestSequence;
}

/** 📨️ Binds the receipt's wire identity only; native descendant terminal authority is not manufactured here. */
export function actorInstanceCloseReceiptMatches(request: ActorInstanceCloseRequest, accepted: ActorInstanceLifecycleReceipt | null, receipt: ActorInstanceLifecycleReceipt): boolean {
  return receipt.kind !== "captured" && actorInstanceLifetimeEquals(request.lifetime, receipt.lifetime) && request.requestSequence === receipt.requestSequence && (accepted === null ? receipt.kind === "accepted" : accepted.kind === "accepted" && actorInstanceLifetimeEquals(accepted.lifetime, receipt.lifetime) && accepted.requestSequence === receipt.requestSequence && accepted.closeGeneration === receipt.closeGeneration);
}

//#region 🧪️WireLaws
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;
  it("actor instance close fault publication fixture preserves watchdog and terminal-outcome precedence", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🚨️fault.fixture.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧯️fault.schema.json", import.meta.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📐️schema/🔣️.json", import.meta.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/📐️schema/🔣️.json", import.meta.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/📐️schema/🔣️.json", import.meta.url), "utf8"));
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
    const components = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧩️components.json", import.meta.url), "utf8"));
    const componentSchema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧬️components.schema.json", import.meta.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/📐️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, slots: 255 })).toBe(false);
    expect(validate({ ...fixture, expectedOrder: fixture.expectedOrder.slice(1) })).toBe(false);
    expect(validate({ ...fixture, emptyReadyBitConsumesNewOwner: true })).toBe(false);
    expect(validate({ ...fixture, rejectedObligationRetained: false })).toBe(false);
    expect(validate({ ...fixture, aliasCounter: { ...fixture.aliasCounter, afterReturn: "0" } })).toBe(false);
    const counter = fixture.aliasCounter;
    const independent = Buffer.alloc(8);
    independent.writeBigUInt64LE(BigInt(counter.before));
    let carry = 1;
    for (let index = 0; index < independent.length; index++) {
      const next = independent[index] + carry;
      independent[index] = next & 255;
      carry = next >>> 8;
    }
    expect(independent.readBigUInt64LE().toString()).toBe(counter.afterReturn);
    expect((BigInt(counter.afterReturn) - 1n).toString()).toBe(counter.afterConsume);
    expect((2n ** 64n - 1n).toString()).toBe(counter.maximum);
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

  it("actor patch storage separates physical placement from semantic retirement grants", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🩹️patch/📐️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    for (const invalid of [
      { ...fixture, logicalCapacity: 128 },
      { ...fixture, placedBytes: [0, 4096, fixture.native64.operationBytes - 1, fixture.native64.operationBytes, fixture.native64.operationBytes] },
      { ...fixture, allocationBeforeAdmission: true },
      { ...fixture, emptyPageStillCharged: false },
      { ...fixture, cancelMovesPayload: true },
      { ...fixture, unplaced: { ...fixture.unplaced, allocationBytes: fixture.native64.operationBytes } },
    ]) expect(validate(invalid)).toBe(false);
    const chunkModule = "lodash-es/chunk.js";
    const module: unknown = await import(chunkModule);
    const chunk: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof chunk !== "function") throw new Error("invalid independent patch paging oracle");
    const pages: unknown = chunk(fixture.operations, 1);
    if (!Array.isArray(pages) || !pages.every(Array.isArray)) throw new Error("invalid independent patch pages");
    expect(pages.map(page => page.length)).toEqual(fixture.pageLengths);
    expect(pages.slice().reverse().map(page => page[0].id)).toEqual(fixture.retirementOrder);
    const native = fixture.native64;
    expect(fixture.logicalCapacity * native.descriptorBytes).toBe(native.directoryBytes);
    expect(native.directoryBytes + native.operationBytes).toBe(native.firstBackingBytes);
    expect(fixture.placementGrants.map((grant: number) => grant >= native.operationBytes ? native.operationBytes : 0)).toEqual(fixture.placedBytes);
    expect(native.directoryBytes).toBeLessThanOrEqual(fixture.physicalGrant);
    expect(native.firstPayloadBytes).toBeLessThanOrEqual(fixture.physicalGrant);
    expect(native.operationBytes).toBeGreaterThan(Math.max(...fixture.semanticGrants));
    expect(Buffer.byteLength("é".repeat(256))).toBe(fixture.unplaced.semanticBytes);
    expect(new TextEncoder().encode("é".repeat(256)).length).toBe(fixture.unplaced.semanticBytes);
  });

  it("actor instance close wire matches strict shared fixtures and an independent LEB128 encoder", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema.json", import.meta.url), "utf8"));
    const fixtureSchema = JSON.parse(readFileSync(new URL("./📐️schema/🔣️.json", import.meta.url), "utf8"));
    const ajv = new Ajv({ strict: true });
    ajv.addSchema(schema);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
    const validate = ajv.compile(schema);
    for (const invalid of ["0", "-1", "01", "18446744073709551616"]) {
      expect(validate({ ...fixture.vectors[0].value, activationGeneration: invalid })).toBe(false);
      expect(validate({ ...fixture.vectors[1].value, lifetime: { ...fixture.vectors[1].value.lifetime, guestLifetime: invalid } })).toBe(false);
    }
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
      const value: ActorInstanceLifecycleWire = JSON.parse(JSON.stringify(row.value), (key, field) => ["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(field) : field);
      const bytes = encodeActorInstanceLifecycle(value);
      const body = value.kind === "ack" ? value.receipt : value;
      const tag = value.kind === "ack" ? { captured: 5, accepted: 6, retired: 7 }[value.receipt.kind] : { open: 0, captured: 1, close: 2, accepted: 3, retired: 4 }[value.kind];
      const expected = body.kind === "open"
        ? [tag, ...u64(body.activationGeneration), ...u64(BigInt(body.instanceId)), ...u64(BigInt(body.requestSequence))]
        : [tag, ...u64(body.lifetime.activationGeneration), ...u64(BigInt(body.lifetime.instanceId)), ...u64(body.lifetime.guestLifetime), ...u64(BigInt(body.requestSequence)), ...("closeGeneration" in body ? u64(body.closeGeneration) : [])];
      expect(Array.from(bytes)).toEqual(expected);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(bytes.length).toBeLessThanOrEqual(ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES);
      expect(decodeActorInstanceLifecycle(bytes)).toEqual(value);
      expect(() => decodeActorInstanceLifecycle(Uint8Array.from([...bytes, 0]))).toThrow();
      for (let length = 0; length < bytes.length; length += 1) expect(() => decodeActorInstanceLifecycle(bytes.subarray(0, length))).toThrow();
    }
    for (const bytes of [[8, 1, 7, 9], [0, 0, 7, 9], [0, 0x81, 0, 7, 9], [0, ...Array(10).fill(255), 7, 9], [0, 1, 7, 0], [1, 1, 7, 0, 9], [3, 1, 7, 13, 9, 0], [0, 1, 7, ...u64(9007199254740992n)]]) expect(() => decodeActorInstanceLifecycle(Uint8Array.from(bytes))).toThrow();
    for (const activationGeneration of [0n, -1n, 18446744073709551616n, 1, "1"]) expect(() => encodeActorInstanceLifecycle({ kind: "open", activationGeneration, instanceId: 7, requestSequence: 8 } as ActorInstanceOpenRequest)).toThrow();
    for (const guestLifetime of [0n, -1n, 18446744073709551616n, 13, "13"]) expect(() => encodeActorInstanceLifecycle({ kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime }, requestSequence: 8 } as ActorInstanceLifecycleReceipt)).toThrow();
    expect(() => encodeActorInstanceLifecycle({ kind: "ack", receipt: { kind: "open", activationGeneration: 1n, instanceId: 7, requestSequence: 8 } } as unknown as ActorInstanceLifecycleWire)).toThrow();
  });

  it("actor instance close receipts reject reused IDs and premature terminal messages", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json", import.meta.url), "utf8"));
    const prior = { ...fixture.reopen.prior, activationGeneration: BigInt(fixture.reopen.prior.activationGeneration), guestLifetime: BigInt(fixture.reopen.prior.guestLifetime) };
    const current = { ...fixture.reopen.current, activationGeneration: BigInt(fixture.reopen.current.activationGeneration), guestLifetime: BigInt(fixture.reopen.current.guestLifetime) };
    const request: ActorInstanceCloseRequest = { kind: "close", lifetime: current, requestSequence: 9 };
    const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: current, requestSequence: 9, closeGeneration: 13n };
    expect(actorInstanceLifetimeEquals(prior, current)).toBe(fixture.reopen.oldRequestAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, lifetime: prior })).toBe(fixture.reopen.oldReceiptAccepted);
    expect(actorInstanceCloseReceiptMatches(request, null, { ...accepted, kind: "retired" })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, null, accepted)).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired" })).toBe(true);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", closeGeneration: 12n })).toBe(false);
    expect(actorInstanceCloseReceiptMatches(request, accepted, { ...accepted, kind: "retired", requestSequence: 8 })).toBe(false);
    const open: ActorInstanceOpenRequest = { kind: "open", activationGeneration: current.activationGeneration, instanceId: current.instanceId, requestSequence: 8 };
    const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: current, requestSequence: 8 };
    expect(actorInstanceCapturedReceiptMatches(open, captured)).toBe(true);
    expect(actorInstanceCapturedReceiptMatches(open, accepted)).toBe(false);
    expect(actorInstanceCapturedReceiptMatches(open, { ...captured, requestSequence: 7 })).toBe(false);
    expect(actorInstanceCapturedReceiptMatches(open, { ...captured, lifetime: { ...current, activationGeneration: current.activationGeneration + 1n } })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(captured, { ...captured })).toBe(true);
    expect(actorInstanceLifecycleReceiptEquals(captured, { ...captured, lifetime: prior })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(accepted, { ...accepted, kind: "retired" })).toBe(false);
    expect(actorInstanceLifecycleReceiptEquals(accepted, { ...accepted, closeGeneration: 14n })).toBe(false);
  });

  it("actor instance close worker activation preserves the new generation across a delayed dispose", async () => {
    const { readFileSync } = await import("node:fs");
    // 🚧️ Specifier held in a variable, and `@vite-ignore`d, so no bundler follows it. This whole
    // block is `import.meta.vitest`-only and Node-only (`node:worker_threads` below), but Vite
    // statically analyses dynamic imports regardless of the guard: following this one pulls
    // `🟦️.ts` — a BUILD-time module importing `node:child_process`,
    // `node:fs` and `typescript`, plus repo-lib's `🔍️discovery` — into the browser worker bundle,
    // which fails the storybook preview build and the os/dev server alike with
    // `"node:url" doesn't have a matching export named "fileURLToPath"`.
    const materializeSpecifier = "../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts";
    const { shardWorkerSource } = await import(/* @vite-ignore */ materializeSpecifier);
    const fixture = JSON.parse(readFileSync(new URL("./🧪️fixture/🔣️.json", import.meta.url), "utf8"));
    const prior = BigInt(fixture.reopen.prior.activationGeneration);
    const current = prior + 1n;
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
      expect((await send({ kind: "turn", requestId: "t1", actorId: "same", activationGeneration: prior, events: [], budget: {} }, "t1")).value).toEqual({ actorId: "same", activationGeneration: prior });
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
      expect(await send({ kind: "activate", requestId: "a2", actorId: "same", activationGeneration: current, moduleUrl, assets: [] }, "a2")).toMatchObject({ ok: true });
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: prior });
      expect((await send({ kind: "turn", requestId: "t2", actorId: "same", activationGeneration: current, events: [], budget: {} }, "t2")).value).toEqual({ actorId: "same", activationGeneration: current });
      expect((await send({ kind: "activate", requestId: "old", actorId: "old", activationGeneration: prior, moduleUrl, assets: [] }, "old")).ok).toBe(false);
      worker.postMessage({ kind: "dispose", actorId: "same", activationGeneration: current });
    } finally {
      await worker.terminate();
    }
  });
}
//#endregion 🧪️WireLaws
//#endregion 🚪️InstanceLifecycleWire
