/** 📃️ The browser pages of a run: evaluation, polling, screenshots, devices and fresh peers. */
import { join } from "node:path";
import { writeFileSync } from "node:fs";
import { page, invalidatedBy, OUT, base, currentLocale, currentStep, scenario, setContext, browser, setPage, context, renderer, wgpuSeedTerminology, setNavigations, navigations, probeReloadAt, setInvalidatedBy } from "../🧠️state/🟦️.ts";
import type { Device, Peer } from "../🧠️state/🟦️.ts";
import { log } from "../🧾️report/🟦️.ts";
import { band, bandTrace, pageNotices, installBandTrace, installNoticeTrace, harvestNotices } from "../🎗️band/🟦️.ts";
import { editor } from "../🖊️editor/🟦️.ts";
import { openPanelTabIds, waitForBoot } from "../🧱️chrome/🟦️.ts";
import { treeWindow, historyInventory, readHistory } from "../📜️history/🟦️.ts";
import { consoleRows, listen, navigationCause } from "../🎧️console/🟦️.ts";
import { routeUrl } from "../🧭️arguments/🟦️.ts";

//#region 🔖️Page

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

/** 🩺️ One diagnosis of the live page under `tag`: a screenshot plus the band, the band trace, the editor, the open panels, the
 * history inventory and rows, the shown notices and the step's last console lines — taken where a verdict fails for a reason the
 * verdict's own detail cannot tell. */
export const diagnose = async (tag: string) => {
  await shot(tag);
  return dumpJson(tag, { band: await band(), trace: (await bandTrace()).slice(-30), editor: await editor(), panels: await openPanelTabIds(), commands: await treeWindow("framework.history.commands"), inventory: (await historyInventory()).filter((row) => !/entry\.|mutation\./.test(String(row.id))), rows: (await readHistory()).slice(0, 60), notices: await pageNotices(), ...(await scenario?.diagnostics?.()), console: consoleRows.filter((row) => row.locale === currentLocale && row.step === currentStep).slice(-80) });
};

/** 🧰️ Writes one diagnostic JSON beside the screenshots and returns its path. */
export const dumpJson = (tag: string, value: unknown) => {
  const path = join(OUT, `${base}-${currentLocale}-s${currentStep}-${tag}.json`);
  writeFileSync(path, JSON.stringify(value, null, 1));
  return path;
};

/** 🖥️ The desktop device every locale pass starts on: three board windows side by side. */
export const DESKTOP: Device = { key: "desktop", viewport: { width: 1600, height: 1000 }, isMobile: false, hasTouch: false, windows: 3 };

/** 📱️ A phone below `UI_MOBILE_MAX_WIDTH_PX` (767): one window, the merged mobile panel. */
export const PHONE: Device = { key: "mobile", viewport: { width: 375, height: 812 }, isMobile: true, hasTouch: true, windows: 1 };

/** 📒️ A tablet in the `📱️device` band 768…1023 px: dock anchors, touch. */
export const TABLET: Device = { key: "tablet", viewport: { width: 768, height: 1024 }, isMobile: true, hasTouch: true, windows: 1 };

/** 🌱️ Opens a fresh browser context and its one tab for `device` in the current locale — the run's console listeners on, the
 * wgpu diagnostics armed, the playground route loaded — and makes it the page every helper drives; answers whether it booted. */
export const openPage = async (device: Device, label: string, polls = 60): Promise<boolean> => {
  setContext(await browser.newContext({ viewport: device.viewport, isMobile: device.isMobile, hasTouch: device.hasTouch, deviceScaleFactor: device.isMobile ? 2 : 1, locale: currentLocale === "de" ? "de-DE" : "en-US", acceptDownloads: false }));
  setPage(await context.newPage());
  if (renderer === "wgpu") await page.addInitScript((seed) => {
    try {
      globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
      if (seed && !globalThis.localStorage?.getItem("semio.os.config")) globalThis.localStorage?.setItem("semio.os.config", JSON.stringify({ preferences: { "os.config.ui-preferences": JSON.stringify({ version: 1, events: [{ mutation: "setTerminology", terminology: "native" }] }) } }));
    } catch {
      return;
    }
  }, wgpuSeedTerminology);
  listen(page);
  await page.goto(routeUrl(), { waitUntil: "domcontentloaded", timeout: 90000 }).catch((error) => log(`${label} goto failed ${String(error).split("\n")[0]}`));
  const booted = await waitForBoot(label, polls, device.windows);
  if (booted) {
    await installBandTrace();
    await installNoticeTrace();
  }
  return booted;
};

/** 🔀️ Makes `peer` the page every helper drives. */
export const use = (peer: Peer) => {
  setContext(peer.context);
  setPage(peer.page);
};

/** 🪟️ Runs `body` on fresh pages it opens through {@link openPage}, then closes every one of them and restores the locale's main
 * page — a step in its own document never disturbs the main flow. */
export const onFreshPages = async (body: (open: (device: Device, label: string) => Promise<Peer | null>) => Promise<void>) => {
  const main: Peer = { context, page };
  const opened: Peer[] = [];
  try {
    await body(async (device, label) => {
      const booted = await openPage(device, label);
      opened.push({ context, page });
      const fresh = page;
      fresh.on("framenavigated", (frame) => {
        if (frame !== fresh.mainFrame()) return;
        setNavigations(navigations + 1);
        const cause = Date.now() - probeReloadAt < 8000 ? "expected (the probe reloaded it)" : navigationCause();
        if (!cause.startsWith("expected")) setInvalidatedBy(invalidatedBy ?? cause);
        log(`${label} main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)} — ${cause}`);
      });
      return booted ? { context, page } : null;
    });
  } finally {
    for (const peer of opened) {
      use(peer);
      await harvestNotices();
      await peer.context.close().catch(() => {});
    }
    use(main);
  }
};
//#endregion 🔖️Page
