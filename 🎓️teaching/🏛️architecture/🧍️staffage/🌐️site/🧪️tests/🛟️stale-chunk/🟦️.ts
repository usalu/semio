/** 🛟️ Stale-chunk recovery after a redeploy: one reload per minute for a failed hashed chunk, never a loop, never without storage.
 * @see ./🥒️.feature — the scenarios
 * @see ../../🛟️recovery/🟦️.ts — the recovery */
import { expect, it } from "vitest";
import { STALE_CHUNK_RELOAD_KEY, recoverStaleChunks } from "../../🛟️recovery/🟦️.ts";

const page = (storage: Pick<Storage, "getItem" | "setItem">, now: number) => {
  const target = new EventTarget();
  let reloads = 0;
  recoverStaleChunks({ target, storage: () => storage, reload: () => void reloads++, now: () => now });
  const fail = (): boolean => { const event = new Event("vite:preloadError", { cancelable: true }); target.dispatchEvent(event); return event.defaultPrevented; };
  return { fail, reloads: () => reloads };
};
const memory = (): Pick<Storage, "getItem" | "setItem"> => { const items = new Map<string, string>(); return { getItem: (key) => items.get(key) ?? null, setItem: (key, value) => void items.set(key, value) }; };

it("reloads once on the first failed chunk and swallows the failure", () => {
  const storage = memory();
  const tab = page(storage, 1_000_000);
  expect(tab.fail()).toBe(true);
  expect(tab.reloads()).toBe(1);
  expect(storage.getItem(STALE_CHUNK_RELOAD_KEY)).toBe("1000000");
});

it("lets a repeated failure within a minute of the reload reach the page", () => {
  const storage = memory();
  storage.setItem(STALE_CHUNK_RELOAD_KEY, "1000000");
  const tab = page(storage, 1_030_000);
  expect(tab.fail()).toBe(false);
  expect(tab.reloads()).toBe(0);
  expect(page(storage, 1_061_000).fail()).toBe(true);
});

it("never reloads when session storage throws", () => {
  const tab = page({ getItem: () => { throw new Error("denied"); }, setItem: () => { throw new Error("denied"); } }, 1);
  expect(tab.fail()).toBe(false);
  expect(tab.reloads()).toBe(0);
});
