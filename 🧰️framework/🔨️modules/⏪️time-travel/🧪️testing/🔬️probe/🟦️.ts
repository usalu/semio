import { join } from "node:path";
import { page, invalidatedBy, OUT, base, currentLocale, currentStep } from "../🧠️state/🟦️.ts";
import { log } from "../🧾️report/🟦️.ts";

/** 🔬️ Page probes every browser-driven check shares: guarded evaluation, polling, sleeping and screenshots; a leaf, so band, chrome, editor, history and mirror never import the page driver that wires them. */
/** 🛟️ One `page.evaluate` that never takes the run down (a reload race answers `fallback`). */
export const evalSafe = async <T, A = undefined>(fn: (arg: A) => T | Promise<T>, fallback: T, arg?: A): Promise<T> => {
  const began = Date.now();
  try {
    return (await page.evaluate(fn as (value: unknown) => T | Promise<T>, arg as unknown)) as T;
  } catch {
    return fallback;
  } finally {
    if (Date.now() - began > 5000) log(`slow page read: ${Date.now() - began} ms in ${String(fn).replace(/\s+/g, " ").slice(0, 140)}`);
  }
};

export const sleep = (ms: number) => page.waitForTimeout(ms);

/** ⏳️ Polls `read` until `settled` holds or `timeoutMs` passes; answers the last value and the wait. */
export const waitUntil = async <T,>(read: () => Promise<T>, settled: (value: T) => boolean, timeoutMs = 20000, everyMs = 250) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < timeoutMs && invalidatedBy === null) {
    await sleep(everyMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};

/** 📸️ One screenshot of the current step. */
export const shot = async (tag: string) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.png`);
  await page.screenshot({ path }).catch(() => {});
  return path;
};

/** 🏷️ Mints a fresh `probe-<n>` token a check stamps onto a DOM node to find it again after the page re-rendered. */
export const markToken = (() => {
  let next = 0;
  return () => `probe-${++next}`;
})();
