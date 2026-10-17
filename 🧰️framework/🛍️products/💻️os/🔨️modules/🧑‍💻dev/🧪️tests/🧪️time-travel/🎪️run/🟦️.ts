/** 🎪️ One locale pass and the whole run with its markdown report. */
import { join } from "node:path";
import { ensureParityPlaywrightBrowsersPath } from "../../../⚖️parity/🏃️execution/🟦️.ts";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { writeFileSync } from "node:fs";
import { context, scenario, setCurrentLocale, setCurrentStep, setNavigations, setExpectedReloads, serveOrigin, renderer, universalRoute, wgpuSeedTerminology, lastBootFault, setWgpuSeedTerminology, page, navigations, probeReloadAt, setInvalidatedBy, invalidatedBy, setMod, mod, explore, OUT, stamp, cancelled, only, expectedReloads, setBrowser, locales, browser, chordLocales, reloadSelected, base, longHistory } from "@semio-tech/framework-time-travel-ts/testing/state";
import type { Locale } from "@semio-tech/framework-time-travel-ts/testing/state";
import { step9 } from "../📟️console-digest/🟦️.ts";
import { stepU } from "../🌐️universal/🟦️.ts";
import { log, verdict, note, verdicts, emit, notes, timeline } from "@semio-tech/framework-time-travel-ts/testing/report";
import { openPage, DESKTOP, dumpJson, shot, evalSafe, sleep } from "@semio-tech/framework-time-travel-ts/testing/page";
import { navigationCause, consoleRows, pageErrors, hardFaults } from "@semio-tech/framework-time-travel-ts/testing/console";
import { historyInventory, openHistory, readHistory, treeWindow } from "@semio-tech/framework-time-travel-ts/testing/history";
import { mirror, mirrorFind, mirrorLocator, chromeHits } from "@semio-tech/framework-time-travel-ts/testing/wgpu-mirror";
import type { MirrorNode } from "@semio-tech/framework-time-travel-ts/testing/wgpu-mirror";
import { attachFolder, pickSyncFolder } from "@semio-tech/framework-time-travel-ts/testing/history-session";
import { closePanels, openTab, INSPECTION_TAB, presenceText } from "@semio-tech/framework-time-travel-ts/testing/chrome";
import { prepareChord, band, shownNotices, finalizeDialog } from "@semio-tech/framework-time-travel-ts/testing/band";
import { ariaFindings, ariaRoots } from "@semio-tech/framework-time-travel-ts/testing/aria-oracle";
import { editor } from "@semio-tech/framework-time-travel-ts/testing/editor";

/** 📟️ The steps every run carries whatever its scenario: the universal journey and the console digest. */
const SHARED_STEPS: Readonly<Record<number, (context: object) => Promise<void>>> = { 9: step9, 24: stepU };

/** 🧭️ The order of one locale pass: the scenario's steps, then the universal journey, then the console digest. */
const runOrder = (): readonly number[] => [...(scenario?.order ?? []), 24, 9];

/** 🔎️ The step function of `step`: the scenario's own, else a shared one. */
const stepFor = (step: number): ((context: object) => Promise<void>) => scenario?.steps[step] ?? SHARED_STEPS[step]!;

/** 🔢️ Every step number the report lists, ascending. */
const stepNumbers = (): readonly number[] => [...new Set([...(scenario?.stepNumbers ?? []), 9, 24])].sort((a, b) => a - b);
//#region 🔖️Main

/** 🌎️ One locale pass: a fresh desktop context whose `navigator.language` selects the UI locale, one tab, every step of
 * {@link ORDER} (the reload check before the fresh-context steps), or the `--explore` inventory. Ctrl-C stops after the running
 * step. */
const runLocale = async (locale: Locale) => {
  setCurrentLocale(locale);
  setCurrentStep(0);
  setNavigations(0);
  setExpectedReloads(0);
  log(`navigating to ${serveOrigin} renderer=${renderer} locale=${locale}`);
  let booted = await openPage(DESKTOP, "boot", universalRoute !== null ? 30 : 100);
  if (renderer === "wgpu") {
    setCurrentStep(1);
    if (!wgpuSeedTerminology) {
      const refused = !booted && /terminology authority/u.test(lastBootFault);
      verdict("wgpu-boots-in-a-fresh-browser-profile", booted, { fault: lastBootFault || null, reading: "a fresh browser profile must boot with its explicitly admitted host locale and the native terminology seed; no stored UI preference is required" });
      if (refused) {
        await context.close().catch(() => {});
        setWgpuSeedTerminology(true);
        booted = await openPage(DESKTOP, "boot-with-a-stored-terminology", 100);
      }
    }
    if (wgpuSeedTerminology) note("wgpu-terminology-preference-seeded", { stored: 'localStorage["semio.os.config"].preferences["os.config.ui-preferences"] = {version: 1, events: [{mutation: "setTerminology", terminology: "native"}]}', reading: "diagnostic continuation uses an explicitly stored native terminology after the recorded fresh-profile boot failure; it does not grant fresh-profile success" });
    setCurrentStep(0);
  }
  page.on("framenavigated", (frame) => {
    if (frame !== page.mainFrame()) return;
    setNavigations(navigations + 1);
    const cause = Date.now() - probeReloadAt < 8000 ? "expected (the probe reloaded it)" : navigationCause();
    if (!cause.startsWith("expected")) setInvalidatedBy(invalidatedBy ?? cause);
    log(`main frame navigated (#${navigations}) → ${frame.url().slice(0, 120)} — ${cause}`);
  });
  setMod((await page.evaluate(() => navigator.platform).catch(() => "MacIntel")).includes("Mac") ? "Meta" : "Control");
  setCurrentStep(1);
  if (!verdict("boot", booted, { mod })) {
    const mismatch = consoleRows.filter((row) => row.locale === locale && /plugin\.channel-mismatch|speaks app channel \d+, the host app channel \d+/u.test(row.text)).map((row) => row.text.slice(0, 300));
    if (mismatch.length) verdict("no-plugin-is-refused-for-a-channel-mismatch", false, { lines: mismatch.slice(0, 6), reading: "the serve's plugin component speaks another app channel than its host (S4-BUMP handshake): a stale build — coordinator action: re-describe / re-activate" });
    dumpJson("boot-failure", { inventory: await historyInventory(), console: consoleRows.filter((row) => row.locale === locale).slice(-80) });
    await shot("boot-failure");
    await context.close().catch(() => {});
    return;
  }
  if (explore && renderer === "wgpu") {
    const structure = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection?.dumpStructure()) ?? "", "");
    const before = await mirror();
    await openHistory();
    const history = await mirror();
    await attachFolder(join(OUT, `folder-explore-${stamp}-${locale}`));
    const afterSync = await mirror();
    await closePanels();
    await openTab(INSPECTION_TAB);
    const inspection = await mirror();
    await closePanels();
    await prepareChord();
    await page.keyboard.press(`${mod}+p`).catch(() => {});
    await sleep(1500);
    const searchInput = mirrorFind(await mirror(), "ui.search.input");
    if (searchInput) await (await mirrorLocator(searchInput.key, searchInput.window))?.fill(scenario?.paletteQuery[locale] ?? "", { force: true, timeout: 4000 }).catch(() => {});
    await sleep(1200);
    const palette = await mirror();
    await shot("explore-palette");
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    const line = (node: MirrorNode) => `${node.window}|${node.key}|${node.role}|${node.label.slice(0, 60)}${node.description ? ` — ${node.description.slice(0, 40)}` : ""}`;
    const windowIds = (() => {
      try {
        return (JSON.parse(structure || "{}") as { windowIds?: string[] }).windowIds;
      } catch {
        return null;
      }
    })();
    dumpJson("explore", { windowIds, ...(await scenario?.exploreInventory?.()), chrome: await chromeHits(), mirrorBefore: before.map(line), mirrorHistory: history.map(line), mirrorAfterSync: afterSync.map(line), mirrorInspection: inspection.map(line), mirrorPalette: palette.map(line), band: await band(), presence: await presenceText(), aria: await ariaFindings(ariaRoots("session")) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  if (explore) {
    await openHistory();
    const syncIds = () => evalSafe(() => Array.from(document.querySelectorAll("[id]")).filter((el) => /sync/i.test(el.id)).map((el) => `${el.tagName.toLowerCase()}#${el.id}[${el.getAttribute("data-slot") ?? el.getAttribute("role") ?? ""}] vis=${(el as HTMLElement).offsetParent !== null} ${(el as HTMLElement).innerText?.replace(/\s+/g, " ").trim().slice(0, 60) ?? ""}`), [] as string[]);
    const syncBefore = await syncIds();
    await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(1000);
    const syncOpen = await syncIds();
    const picked = await pickSyncFolder();
    log(`sync folder pick: ${picked}`);
    await sleep(1000);
    const syncFolder = await syncIds();
    const popovers = await evalSafe(() => Array.from(document.querySelectorAll("[data-radix-popper-content-wrapper], [role=dialog]")).map((el) => (el as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 200)), [] as string[]);
    await shot("explore-sync");
    await page.keyboard.press("Escape").catch(() => {});
    dumpJson("explore", { syncBefore, syncOpen, syncFolder, popovers, inventory: await historyInventory(), rows: await readHistory(), band: await band(), ...(await scenario?.diagnostics?.()), aria: await ariaFindings(ariaRoots("session")), tabs: await evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((el) => `${el.id}:${(el as HTMLElement).innerText.trim()}`), [] as string[]) });
    await shot("explore");
    await context.close().catch(() => {});
    return;
  }
  const ctx: object = scenario?.context() ?? {};
  const blocked = scenario ? await scenario.prepare(ctx) : null;
  for (const step of runOrder()) {
    if (cancelled.aborted) {
      note("cancelled-before-the-step", { step });
      break;
    }
    if (invalidatedBy !== null) {
      note("run-invalidated-before-the-step", { step, by: invalidatedBy, reading: "the page was reloaded under the run by something other than the probe (a served file saved without the serve lock, or the page reloading itself); every verdict from the step that was running on is void and the batch must be run again" });
      break;
    }
    await scenario?.beforeStep(step, ctx, blocked);
    if (only && !only.has(step)) continue;
    if (blocked !== null && step !== 9 && !blocked.exempt.includes(step)) {
      setCurrentStep(step);
      verdict(blocked.name, false, { prerequisite: blocked.prerequisite });
      continue;
    }
    setCurrentStep(step);
    const navigationsBefore = navigations;
    const reloadsBefore = expectedReloads;
    const noticesBefore = (await shownNotices()).length;
    log(`step ${step} begins`);
    try {
      await stepFor(step)(ctx);
    } catch (error) {
      verdict("step-threw", false, { error: String(error).slice(0, 400) });
    }
    const stepNotices = (await shownNotices()).slice(noticesBefore);
    if (stepNotices.length) note("notices-first-shown-in-this-step", { notices: stepNotices });
    if (navigations - navigationsBefore > expectedReloads - reloadsBefore) note("page-reloaded-under-the-step", { navigations: navigations - navigationsBefore, reading: "the dev serve reloaded the page (Vite reconnect / supervisor recycle); verdicts after that point measure a fresh document" });
    const results = verdicts.filter((row) => row.locale === locale && row.step === step);
    if (results.some((row) => !row.ok)) {
      const dialog = await finalizeDialog();
      if (!dialog) await openHistory();
      dumpJson("failure", { inventory: await historyInventory(), rows: await readHistory(), commands: await treeWindow("framework.history.commands"), band: await band(), editor: await editor(), dialog, ...(await scenario?.diagnostics?.()), console: consoleRows.filter((row) => row.locale === locale && row.step === step).slice(-60) });
      await shot("failure");
    }
    await shot("end");
  }
  await scenario?.afterSteps(ctx, blocked);
  emit({ kind: "context", ctx });
  await context.close().catch(() => {});
};

type RunSummary = { verdicts: number; passed: number; failed: number; uncaught: number; hard: number; failing: string[]; mdPath: string };

/** 🎪️ The whole run: one headless Chromium (the repository's pinned browser build), every locale, the markdown report;
 * answers the counts the acceptance record carries. */
export const runProbe = async (): Promise<RunSummary> => {
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  setBrowser(await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--enable-features=Vulkan,UseSkiaRenderer"] }));
  try {
    for (const locale of locales) {
      if (cancelled.aborted) break;
      await runLocale(locale);
    }
  } finally {
    await browser.close().catch(() => {});
  }
  const stepLines: string[] = [];
  for (const locale of locales) {
    for (const step of stepNumbers()) {
      const rows = verdicts.filter((row) => row.locale === locale && row.step === step);
      if (!rows.length) continue;
      const failed = rows.filter((row) => !row.ok);
      stepLines.push(`STEP ${locale}/${step} ${failed.length ? "FAIL" : "PASS"} (${rows.length - failed.length}/${rows.length})${failed.length ? ` — ${failed.map((row) => row.name).join(", ")}` : ""}`);
    }
  }
  const passed = verdicts.filter((row) => row.ok).length;
  const summary = `time-travel probe renderer=${renderer} PASS=${passed} FAIL=${verdicts.length - passed} uncaught=${pageErrors.length} hard=${hardFaults.length} locales=${locales.join(",")} chords=${[...chordLocales].join(",")} only=${only ? `${[...only].join(",")}${reloadSelected ? ",reload" : ""}` : "all"}${cancelled.aborted ? " cancelled" : ""}${invalidatedBy !== null ? ` INVALIDATED by ${invalidatedBy.slice(0, 260)}` : ""}`;
  console.log(summary);
  for (const line of stepLines) console.log(line);
  emit({ kind: "summary", summary, steps: stepLines });
  const mdPath = join(OUT, `${base}.md`);
  writeFileSync(
    mdPath,
    [
      `# ${base} (renderer ${renderer}, serve ${serveOrigin}, locales ${locales.join(",")}, chords ${[...chordLocales].join(",")}, only ${only ? `${[...only].join(",")}${reloadSelected ? ",reload" : ""}` : "all"}, long history ≥ ${longHistory})`,
      "",
      summary,
      "",
      "## Steps",
      ...stepLines,
      "",
      "## Verdicts",
      ...verdicts.map((row) => `- ${row.ok ? "PASS" : "FAIL"} ${row.locale}/${row.step} ${row.name} ${JSON.stringify(row.detail).slice(0, 700)}`),
      "",
      "## Notes",
      ...(notes.length ? notes.map((row) => `- ${row}`) : ["(none)"]),
      "",
      "## Uncaught page errors",
      ...(pageErrors.length ? pageErrors.map((row) => `- ${row.locale}/${row.step} ${row.text.slice(0, 600)}`) : ["(none)"]),
      "",
      "## Hard faults",
      ...(hardFaults.length ? hardFaults.slice(0, 40).map((row) => `- ${row.locale}/${row.step} ${row.text}`) : ["(none)"]),
      "",
      "## Timeline",
      ...timeline,
      "",
      "## Console (errors and warnings, last 200)",
      ...consoleRows.filter((row) => row.type === "error" || row.type === "warning" || row.type === "pageerror" || row.type === "http").slice(-200).map((row) => `${row.t}s ${row.locale}/${row.step} ${row.source} ${row.type}: ${row.text.slice(0, 400)}`),
    ].join("\n"),
  );
  return { verdicts: verdicts.length, passed, failed: verdicts.length - passed, uncaught: pageErrors.length, hard: hardFaults.length, failing: [...new Set(verdicts.filter((row) => !row.ok).map((row) => `${row.locale}/${row.step} ${row.name}`))], mdPath };
};
//#endregion 🔖️Main
