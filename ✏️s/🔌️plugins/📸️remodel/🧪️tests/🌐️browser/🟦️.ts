import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { checkScriptInvocation, type ScriptInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { prepareRepositoryBrowserV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts";
import { browserLaunchOptionsV1, PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import { awaitBeacon, dismissIntroduction, seatLocale, unfoldActionsRail } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { initialRemodelBrowserReportV1, remodelEditorPresentV1, remodelAcceptanceSatisfiedV1, serializeRemodelBrowserReportV1, parseRemodelBrowserArgumentsV1, remodelImageInputsV1, remodelHostReadyV1 } from "./📋️report/🟦️.ts";

/** 📸️ Inspects a supplied live host and records unfinished Remodel acceptance scenarios honestly. */
export async function runRemodelBrowserAcceptanceV1(invocation: ScriptInvocation, repoRoot: string, segments: readonly string[]): Promise<void> {
  checkScriptInvocation(invocation);
  const signal = invocation.control.signal;
  signal.throwIfAborted();
  const { host, output, locale } = parseRemodelBrowserArgumentsV1(segments);
  const report = initialRemodelBrowserReportV1(host);
  await mkdir(dirname(output), { recursive: true });
  try {
    await prepareRepositoryBrowserV1(repoRoot, invocation);
    signal.throwIfAborted();
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    const browser = await chromium.launch(browserLaunchOptionsV1());
    const stop = (): void => { void browser.close().catch(() => undefined); };
    signal.addEventListener("abort", stop, { once: true });
    try {
      signal.throwIfAborted();
      const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale });
      page.on("pageerror", error => { if (report.faults.length < 32) report.faults.push(String(error).slice(0, 500)); });
      const progress = setInterval(() => console.log("[remodel] Waiting for the supplied host and editor readiness"), 10_000);
      let beacon: string | null;
      try {
        await page.goto(host, { waitUntil: "commit", timeout: 180_000 });
        beacon = await awaitBeacon(page, Date.now() + 180_000);
      } finally { clearInterval(progress); }
      signal.throwIfAborted();
      if (!remodelHostReadyV1(beacon)) throw Error("Host did not report ready: " + String(beacon));
      await dismissIntroduction(page);
      const observed = await page.locator('[data-slot="surface-role-chip"]').evaluateAll(elements => elements.map(element => ({ app: element.getAttribute("data-app-id"), role: element.getAttribute("data-role") })));
      const contents = await page.locator('[data-slot="window-body"]').count();
      const row = report.scenarios.find(item => item.id === "identity")!;
      row.evidence.push(JSON.stringify({ beacon, identities: observed, windowBodies: contents }));
      const editorObserved = remodelEditorPresentV1(observed);
      if (!editorObserved) row.status = "unsupported";
      if (editorObserved) {
        const modes = report.scenarios.find(item => item.id === "locales-and-modes")!;
        for (const locale of ["en", "de"] as const) {
          signal.throwIfAborted();
          const seated = await seatLocale(page, locale);
          modes.evidence.push("locale " + seated);
          if (!new RegExp("^" + locale + "=" + locale + "(?:-|$)", "iu").test(seated)) { modes.status = "unsupported"; break; }
          for (const id of ["capture", "model", "analyze"]) {
            signal.throwIfAborted();
            const button = page.locator('[id="playground.navbar.modes.' + id + '"]').first();
            if (await button.count() === 0) { modes.status = "unsupported"; break; }
            await button.click();
            await page.waitForFunction(mode => document.getElementById("playground.navbar.modes." + mode)?.getAttribute("aria-pressed") === "true", id, { timeout: 15_000 });
            const windows = await page.locator('[data-slot="mode-dock-tab"] [role="tab"]').allTextContents();
            modes.evidence.push(JSON.stringify({ locale, mode: id, pressed: await button.getAttribute("aria-pressed"), windows }));
            if (windows.length === 0) { modes.status = "failed"; break; }
          }
          if (modes.status !== "unexecuted") break;
        }
        const examples = report.scenarios.find(item => item.id === "examples")!;
        for (const id of ["demo", "demo-session", "synthetic-orbit"]) {
          signal.throwIfAborted();
          const picker = page.locator('[id="playground.navbar.fixture"][role="combobox"]').first();
          if (await picker.count() === 0) { examples.status = "unsupported"; break; }
          await picker.click();
          const exact = page.locator('[role="option"][data-value="' + id + '"]');
          if (await exact.count() !== 1) { examples.status = "unsupported"; await page.keyboard.press("Escape"); break; }
          const label = (await exact.textContent())?.trim() ?? "";
          await exact.click();
          await page.waitForFunction(text => (document.getElementById("playground.navbar.fixture")?.textContent ?? "").includes(text), label, { timeout: 20_000 });
          const shown = (await picker.textContent())?.trim() ?? "";
          examples.evidence.push(JSON.stringify({ selected: id, label, shown, visible: (await page.locator('[data-slot="window-body"]').allTextContents()).join(" ").slice(0, 2000) }));
          if (!label || !shown.includes(label)) { examples.status = "failed"; break; }
        }
        signal.throwIfAborted();
        const mediaTab = page.locator('[data-slot="panel-tab-button"][id="remodeling.media"]').first();
        if (await mediaTab.count() && await mediaTab.isVisible()) {
          if (await mediaTab.getAttribute("aria-selected") !== "true") await mediaTab.click();
          const orbit = page.locator('[data-ui-node-key="remodeling-media.stream.synthetic-orbit"]').first();
          if (await orbit.waitFor({ state: "visible", timeout: 20_000 }).then(() => true).catch(() => false)) {
            examples.evidence.push(JSON.stringify({ syntheticOrbitStream: await orbit.innerText() }));
          }
        }
        const calibrationTab = page.locator('[data-slot="panel-tab-button"][id="remodeling.calibration"]').first();
        if (await calibrationTab.count() && await calibrationTab.isVisible()) {
          if (await calibrationTab.getAttribute("aria-selected") !== "true") await calibrationTab.click();
          const camera = page.locator('[data-ui-node-key="remodeling-calibration.camera.synthetic-cam"]').first();
          if (await camera.waitFor({ state: "visible", timeout: 20_000 }).then(() => true).catch(() => false)) {
            examples.evidence.push(JSON.stringify({ syntheticCamera: await camera.innerText() }));
          }
        }
        signal.throwIfAborted();
        const images = report.scenarios.find(item => item.id === "import-images")!;
        await unfoldActionsRail(page);
        const importAction = page.locator('[data-slot="window-action-pane"] [id="action.importFrames"]').first();
        if (await importAction.count() === 0) images.status = "unsupported";
        else {
          const awaiting = page.waitForEvent("filechooser", { timeout: 15_000 }).catch(() => null);
          await importAction.click();
          let chooser = await Promise.race([awaiting, page.waitForTimeout(1_000).then(() => null)]);
          if (!chooser) {
            signal.throwIfAborted();
            const execute = page.locator('[data-slot="window-action-pane"] [id$=".action.importFrames.execute"], [data-slot="window-action-pane"] [id="action.importFrames.execute"]').first();
            if (await execute.count()) await execute.click();
            chooser = await awaiting;
          }
          signal.throwIfAborted();
          if (!chooser) { images.status = "unsupported"; images.evidence.push("File chooser was not observed within 15 seconds"); }
          else if (!chooser.isMultiple()) { images.status = "unsupported"; images.evidence.push("File chooser did not allow multiple inputs"); }
          else {
            const inputs = remodelImageInputsV1(repoRoot);
            await chooser.setFiles(inputs);
            images.evidence.push(JSON.stringify({ picker: "multiple", selected: inputs.map(path => path.split(/[\\/]/u).at(-1)) }));
          }
        }
        signal.throwIfAborted();
        const lifecycle = report.scenarios.find(item => item.id === "run-controls-and-abort")!;
        const reconstruction = page.locator('[data-slot="panel-tab-button"][id="tool.reconstruction"]').first();
        if (await reconstruction.count() && await reconstruction.isVisible()) {
          if (await reconstruction.getAttribute("aria-selected") !== "true") await reconstruction.click();
          lifecycle.evidence.push(JSON.stringify({ tool: "reconstruction", selected: await reconstruction.getAttribute("aria-selected") }));
        } else lifecycle.status = "unsupported";
      }
    } finally {
      signal.removeEventListener("abort", stop);
      await browser.close().catch(error => { if (!signal.aborted) throw error; });
    }
    signal.throwIfAborted();
  } catch (error) {
    if (report.faults.length < 32) report.faults.push(String(error).slice(0, 500));
    if (signal.aborted) throw signal.reason;
    throw error;
  } finally {
    await writeFile(output, serializeRemodelBrowserReportV1(report), "utf8");
  }
  if (!remodelAcceptanceSatisfiedV1(report)) process.exitCode = 1;
}
