/** 🚪️ Neutral program retirement admission laws. */
import { expect, it } from "vitest";
import Ajv from "ajv";
import { LRUCache } from "lru-cache";
import pLimit from "p-limit";
import fixture from "./🔣️.json";
import { admitProgramRetirementV1 } from "../../🧱️elements/🏛️ShellHost/🚪️program-retirement/🟦️.ts";
it("validates neutral admission vectors", () => expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true));
for (const row of fixture.cases) it(row.id, async () => {
  const abort = new AbortController(), events: string[] = [];
  let current = true, released = 0;
  const result = await admitProgramRetirementV1({
    owners: row.owners.map((decision, index) => ({ decision, index })), signal: abort.signal, current: () => current,
    quiesce: async () => row.quiet,
    seal: owner => { events.push("seal:" + owner.index); return () => { released++; }; },
    persist: async owner => { events.push("persist:" + owner.index); if (row.stale) current = false; if (row.cancel) abort.abort(); if (owner.decision !== "saved") throw new Error("write-failed"); },
    confirmDiscard: async owner => { events.push("confirm:" + owner.index); return owner.decision === "discard"; },
  });
  expect(result !== null).toBe(row.approved); expect(released).toBe(row.released);
  expect(events.filter(event => event.startsWith("confirm:"))).toHaveLength(row.cancel ? 0 : row.owners.filter((decision, index) => decision !== "saved" && row.owners.slice(0, index).every(previous => previous !== "keep")).length);
  expect(events).not.toContain("destroy");
  if (result !== null) { expect(result.current()).toBe(true); result.release(); result.release(); expect(released).toBe(row.owners.length); expect(result.current()).toBe(false); }
  const serial = pLimit(1), oracle: string[] = [];
  let allowed = row.quiet;
  await Promise.all(row.owners.map((decision, index) => serial(async () => { if (!allowed) return; oracle.push("persist:" + index); allowed = !row.stale && !row.cancel && decision !== "keep"; })));
  expect(events.filter(event => event.startsWith("persist:"))).toEqual(oracle);
});
it("seals all owners before any archive capture and releases earlier seals on a quiesce refusal", async () => {
  const events: string[] = [];
  const result = await admitProgramRetirementV1({owners: [1, 2], signal: new AbortController().signal, current: () => true,
    quiesce: async owner => owner === 1, seal: owner => { events.push("seal:" + owner); return () => { events.push("release:" + owner); }; },
    persist: async owner => { events.push("persist:" + owner); }, confirmDiscard: async () => false });
  expect(result).toBeNull(); expect(events).toEqual(["seal:1", "release:1"]);
});

it("seals remain scoped to one loaded handle and survive Keep then reseal", async () => {
  const { createSealedInstanceLedgerV1 } = await import("../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts");
  const ledger = createSealedInstanceLedgerV1(1), old = {}, fresh = {};
  ledger.seal(old, "stdio", 1); expect(ledger.sealed(fresh, "stdio", 1)).toBe(false);
  ledger.unseal(old, "stdio", 1); ledger.seal(old, "stdio", 1); expect(ledger.sealed(old, "stdio", 1)).toBe(true);
  ledger.forget(old); expect(ledger.size()).toBe(0);
});

it("matches neutral handle-isolation and resealing laws against independent LRU storage", async () => {
  const { createSealedInstanceLedgerV1 } = await import("../../🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts");
  expect(new Ajv().validate(fixture.ledgerSchema, fixture.ledgerCases)).toBe(true);
  for (const row of fixture.ledgerCases) {
    const ledger = createSealedInstanceLedgerV1(row.slots), oracle = new LRUCache<string, boolean>({ max: row.slots });
    const handles = new Map<string, object>();
    const handle = (id: string): object => { if (!handles.has(id)) handles.set(id, {}); return handles.get(id)!; };
    for (const command of row.commands) {
      const [operation, owner, plugin, id] = command as [string, string, string, number], key = owner + ":" + plugin + ":" + id;
      if (operation === "seal") { ledger.seal(handle(owner), plugin, id); if (!oracle.has(key)) oracle.set(key, true); }
      if (operation === "unseal") { ledger.unseal(handle(owner), plugin, id); oracle.delete(key); }
      if (operation === "forget") { ledger.forget(handle(owner)); for (const key of oracle.keys()) if (key.startsWith(owner + ":")) oracle.delete(key); }
    }
    for (const query of row.queries) {
      const [owner, plugin, id, expected] = query as [string, string, number, boolean];
      expect(ledger.sealed(handle(owner), plugin, id)).toBe(expected); expect(oracle.has(owner + ":" + plugin + ":" + id)).toBe(expected);
    }
  }
});
