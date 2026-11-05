/** 💾️ Generic byte archive acknowledgment through actual readback. */
import Ajv from "ajv";
import { Buffer } from "node:buffer";
import { afterEach, expect, it, vi } from "vitest";
import { awaitVerifiedReadbackV1, type VerifiedReadbackListenerV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/💾️acknowledgment/🟦️.ts";
import fixture from "./🧫️fixtures/📦️readback.json";

afterEach(() => vi.useRealTimers());
it("uses the language-neutral byte corpus and independent Buffer comparison oracle", async () => {
  expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true);
  for (const row of fixture.cases) {
    let listener!: VerifiedReadbackListenerV1<Uint8Array>;
    let settled = false;
    const unsubscribe = vi.fn(), probe = vi.fn();
    const expected = new Uint8Array(row.expected);
    const result = awaitVerifiedReadbackV1<Uint8Array>({
      signal: new AbortController().signal, current: () => true, timeoutMs: 1000,
      subscribe: next => { listener = next; return unsubscribe; }, submit: () => {}, probe,
      accepts: actual => actual.length === expected.length && actual.every((byte, index) => byte === expected[index]),
    }).then(value => { settled = true; return value; });
    listener.status(true); await Promise.resolve();
    expect(settled, "persisted status is not content proof").toBe(false);
    for (const readback of row.older) {
      const value = new Uint8Array(readback);
      expect(Buffer.compare(Buffer.from(value), Buffer.from(expected))).not.toBe(0);
      listener.archive(value); await Promise.resolve(); expect(settled).toBe(false);
    }
    expect(Buffer.compare(Buffer.from(row.expected), Buffer.from(expected))).toBe(0);
    listener.archive(expected);
    expect(await result).toEqual(expected);
    expect(unsubscribe).toHaveBeenCalledTimes(1);
    expect(probe).toHaveBeenCalledTimes(2);
    listener.status(true);
    expect(probe).toHaveBeenCalledTimes(2);
  }
});
for (const reason of ["abort", "retired", "timeout", "submit", "probe"] as const) it("cleans up byte verification after " + reason, async () => {
  vi.useFakeTimers();
  const abort = new AbortController(), unsubscribe = vi.fn();
  let listener!: VerifiedReadbackListenerV1<Uint8Array>, current = true;
  const result = awaitVerifiedReadbackV1<Uint8Array>({
    signal: abort.signal, current: () => current, timeoutMs: 10,
    accepts: () => true, subscribe: next => { listener = next; return unsubscribe; },
    submit: () => { if (reason === "submit") throw Error("submit"); },
    probe: () => { if (reason === "probe") throw Error("probe"); },
  });
  if (reason === "abort") abort.abort();
  if (reason === "retired") { current = false; listener.archive(new Uint8Array([1])); }
  if (reason === "timeout") await vi.advanceTimersByTimeAsync(10);
  expect(await result).toBeNull();
  expect(unsubscribe).toHaveBeenCalledTimes(1);
  expect(vi.getTimerCount()).toBe(0);
});
it("ignores pre-submit snapshots and releases a synchronously aborted subscription", async () => {
  const abort = new AbortController(), unsubscribe = vi.fn(), submit = vi.fn(), accepts = vi.fn(() => true);
  const result = awaitVerifiedReadbackV1<Uint8Array>({
    signal: abort.signal, current: () => true, timeoutMs: 1000, accepts,
    subscribe: listener => { listener.archive(new Uint8Array([1])); abort.abort(); return unsubscribe; },
    submit, probe: () => {},
  });
  expect(await result).toBeNull();
  expect(submit).not.toHaveBeenCalled(); expect(accepts).not.toHaveBeenCalled();
  expect(unsubscribe).toHaveBeenCalledTimes(1);
});
