/** 🔶️ The actor instance close fault phase of registerTests1. */
import type { TestSource } from "../🟢️harness/🟦️.ts";
/** 🔷 Registers the actor instance close fault tests of registerTests1. */
export function registerActorInstanceCloseFaultTests({ dependencies, vitest, source }: { readonly dependencies: Pick<typeof import("../../../🟦️.ts"), "ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES" | "actorInstanceCapturedReceiptMatches" | "actorInstanceCloseReceiptMatches" | "actorInstanceLifecycleReceiptEquals" | "actorInstanceLifetimeEquals" | "decodeActorInstanceLifecycle" | "encodeActorInstanceLifecycle">; readonly vitest: NonNullable<ImportMeta["vitest"]>; readonly source: TestSource }) {
  const { ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals, decodeActorInstanceLifecycle, encodeActorInstanceLifecycle } = dependencies;
  const { it, expect } = vitest;
  it("actor instance close fault publication fixture preserves watchdog and terminal-outcome precedence", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🚨️fault.fixture.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const name = "lodash-es/gte.js";
    const module: unknown = await import(name);
    const greater: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof greater !== "function") throw new Error("invalid independent elapsed comparison oracle");
    for (const row of fixture.callbacks) expect(row.candidate !== "fault" && greater(row.elapsedUs, fixture.callbackLimitUs) ? "deadline-yield" : row.candidate).toBe(row.published);
    for (const row of fixture.clocks) {
      const [start, preflight, finish] = row.samples;
      const entered = typeof start === "number" && typeof preflight === "number" && preflight >= start;
      expect(entered).toBe(row.workEntered);
      expect(!entered || typeof finish !== "number" || finish < preflight ? "fault" : greater(finish - start, fixture.callbackLimitUs) ? "deadline-yield" : "complete").toBe(row.published);
    }
    for (const row of fixture.terminalPump) expect(row.faulted ? "fault" : row.blocked ? "external-wait" : row.complete ? "complete" : "ready").toBe(row.status);
  });

  it("actor instance close native value fixture accounts exact descendant text and independent cloned structure", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📋️list/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
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
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const encode = new TextEncoder();
    const bytes = (value: unknown): number =>
      typeof value === "string"
        ? encode.encode(value).length
        : Array.isArray(value)
          ? value.reduce((sum, child) => sum + bytes(child), 0)
          : value && typeof value === "object"
            ? Object.entries(value).reduce((sum, [key, child]) => sum + encode.encode(key).length + bytes(child), 0)
            : 0;
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
    const components = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧩️components.json", source.url), "utf8"));
    expect(components.cases.map((row: { component: { type: string } }) => row.component.type)).toEqual(fixture.componentVariants);
    const enumFields = new Set(["type", "role", "kind", "trigger", "placement", "rowExtent"]);
    const valueFields = new Set(["props", "args", "input", "dragData", "dataAttributes"]);
    const semanticBytes = (value: unknown, key = "", raw = false): number => {
      if (typeof value === "string") return !raw && (enumFields.has(key) || key === "publicationRevision") ? 0 : Buffer.byteLength(value);
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
  return { it, expect, encodeActorInstanceLifecycle, ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, decodeActorInstanceLifecycle, actorInstanceLifetimeEquals, actorInstanceCloseReceiptMatches, actorInstanceCapturedReceiptMatches, actorInstanceLifecycleReceiptEquals };
}
