/** 🏃️ Fixture-driven natural-file acceptance through the real OS shell.
 * Each row imports its exact sample, edits the declared native control, proves exported undo/redo,
 * saves with a durable kept UUID acknowledgment, closes, reopens that UUID before and after reload,
 * compares edited content, and verifies refusal preserves both the document and its windows.
 * run --serve <url> --out <ticket generated directory> [--only <artifact,…>] --entry natural-file|authored-example --locale en|de
 * @see ../🧫️fixtures/🔣️.json */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Browser, Download, FileChooser, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "@semio-tech/framework-plugin/build/plan";
import { ensureParityPlaywrightBrowsersPath } from "@semio-tech/framework-os-dev/parity/execution";
import { FAULT, NOISE, awaitBeacon, clickUncovered, dismissIntroduction, fillStagedArgument, mutateUndoRedo, readShell, seatLocale, unfoldActionsRail, windowIds, withDevServe, witness } from "@semio-tech/framework-os-dev/testing/program-matrix";
import { REPO_ROOT, hexBytes, readMatrix, resolveSample, selectEditorProgram, type AcceptanceRow } from "../📇️matrix/🟦️.ts";
import { judgeBytes, persistedContent } from "../🔬️oracles/🟦️.ts";
import { detailsControlSelector } from "./🎯️details/🟦️.ts";
import {validateJourneyOutput,parseEntryMode,parseJourneyLocale,assertActualLocale,assertSeatedLocale,selectJourneyRows,assertOwnedProgram,type EntryMode} from "./🚪️entry/🟦️.ts";

//#region 🔖️Report
export type StepVerdict = Readonly<{ status: "pass" | "fail" | "skipped"; detail: string }>;
export const STEPS = ["open", "render", "edit", "save", "close", "reopen", "reload-reopen", "export", "invalid"] as const;
export type StepName = (typeof STEPS)[number];
export type RowReport = { entry:EntryMode; artifact: string; batch: string; kind: string; steps: Partial<Record<StepName, StepVerdict>>; faults: string[]; pass: boolean };
export type JourneyReport = { entry:EntryMode; actualLocale?:string; baseUrl: string; tag: string; locale: string; started: string; finished?: string; rows: RowReport[]; fatal?: string };

/** 📋️ Markdown matrix: one line per artifact, one column per step. */
export function journeyTable(report: JourneyReport): string {
  const mark = { pass: "ok", fail: "FAIL", skipped: "-" } as const;
  const lines = [`| batch | artifact | ${STEPS.join(" | ")} | verdict |`, `| --- | --- | ${STEPS.map(() => "---").join(" | ")} | --- |`];
  for (const row of report.rows) lines.push(`| ${row.batch} | ${row.artifact} | ${STEPS.map((step) => (row.steps[step] ? mark[row.steps[step]!.status] : "-")).join(" | ")} | ${row.pass ? "PASS" : "FAIL"} |`);
  return `${lines.join("\n")}\n`;
}
//#endregion 🔖️Report

//#region 🔖️Journey
type Program = Readonly<{ pluginId: string; appId: string }>;
type PickerState = { next: Uint8Array | null; name: string; expectedInvalid: boolean };
const ok = (detail: string): StepVerdict => ({ status: "pass", detail });
const no = (detail: string): StepVerdict => ({ status: "fail", detail });
const skipped = (detail: string): StepVerdict => ({ status: "skipped", detail });

async function drive(page: Page, row: AcceptanceRow, locale: "en"|"de", entry:EntryMode, outDir: string, faults: string[], refusals: string[], downloads: Download[], choosers: PickerState, steps: Partial<Record<StepName, StepVerdict>> = {}, signal: AbortSignal = new AbortController().signal): Promise<Partial<Record<StepName, StepVerdict>>> {
  const faultMark = faults.length;
  const notices = (): Promise<string[]> => page.evaluate(() => (window as unknown as { __stdioAcceptanceNotices: string[] }).__stdioAcceptanceNotices ?? []);
  const programs = await page.evaluate(() => ((window as unknown as { __semioOsCatalogProbe?: { programs: Program[] } }).__semioOsCatalogProbe?.programs ?? []) as Program[]);
  const program = selectEditorProgram(row, programs);
  const openProgram = async (): Promise<string | null> => {
    const before = await windowIds(page);
    await dismissIntroduction(page);
    await page.evaluate(() => (document.activeElement instanceof HTMLElement ? document.activeElement.blur() : undefined));
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    if ((await input.count()) === 0) return "command palette never opened";
    await input.fill(row.artifact);
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program!.pluginId}.${program!.appId}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) { await page.keyboard.press("Escape"); return `no palette row spawn.${program!.pluginId}.${program!.appId}`; }
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }));
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      signal.throwIfAborted();
      if ((await windowIds(page)).some((id) => !before.includes(id))) { await page.waitForTimeout(3_500); return null; }
      await page.waitForTimeout(250);
    }
    return `no new window; notices ${JSON.stringify((await notices()).slice(-2))}`;
  };
  const freshWindows = async (before: readonly string[]): Promise<string[]> => (await windowIds(page)).filter((id) => !before.includes(id));
  const bodyRendered = async (ids: readonly string[]): Promise<string | null> => {
    const painted = await page.evaluate((owned) => [...document.querySelectorAll('[data-slot="window-body"]')].some((body) => {
      const owner = body.closest('[data-slot="window"]')?.id;
      if (owner === null || owner === undefined || !owned.includes(owner)) return false;
      const copy = body.cloneNode(true) as HTMLElement;
      copy.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"]').forEach((node) => node.remove());
      return (copy.textContent ?? "").trim().length > 0 || copy.querySelector("canvas, svg") !== null;
    }), ids);
    return painted ? null : `no owned document body painted (${ids.join(",")})`;
  };
  const closeNewest = async (ids: readonly string[]): Promise<string[]> => {
    for (const id of ids) await page.evaluate((windowId) => { const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId); const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]'); if (button instanceof HTMLElement) button.click(); }, id);
    const deadline = Date.now() + 35_000;
    while (Date.now() < deadline && (await windowIds(page)).some((id) => ids.includes(id))) { signal.throwIfAborted(); await page.waitForTimeout(250); }
    return (await windowIds(page)).filter((id) => ids.includes(id));
  };
  const railIds = async (): Promise<string[]> => (await unfoldActionsRail(page), (await readShell(page)).actions.map((id) => id.replace(/^action\./u, "")));
  const spawnedPrograms = (): Promise<(Program & { id: string })[]> => page.evaluate(() => (window as unknown as { __semioOsCatalogProbe?: { spawned: (Program & { id: string })[] } }).__semioOsCatalogProbe?.spawned ?? []);
  const focusDocument = async (ids: readonly string[]): Promise<void> => {
    await page.evaluate((owned) => (Array.from(document.querySelectorAll('[data-slot="mode-dock-tab"]')).find((tab) => owned.includes(tab.getAttribute("data-window-id") ?? "")) as HTMLElement | undefined)?.click(), ids);
    await page.waitForTimeout(250);
  };
  const activeKeptDocument = (): Promise<string | null> => page.evaluate(() => document.querySelector('[data-semio-local-document-id][aria-current="true"]')?.getAttribute("data-semio-local-document-id") ?? null);
  const keptDocument = (id: string) => page.locator(`[data-semio-local-document-id=${JSON.stringify(id)}]`).first();
  const reopenKept = async (id: string): Promise<string[]> => {
    const button = keptDocument(id);
    await button.waitFor({ state: "visible", timeout: 120_000 });
    const before = new Set(await windowIds(page));
    const seen = (await notices()).length;
    await button.click();
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      signal.throwIfAborted();
      const events = (await notices()).slice(seen);
      if ((await activeKeptDocument()) === id && events.some((notice) => notice.startsWith("shell.documentTransfer.file-opened|"))) {
        const ids = [...new Set(await windowIds(page))].filter((windowId) => !before.has(windowId));
        if (ids.length === 0) throw new Error(`kept document ${id} opened without windows`);
        await focusDocument(ids);
        await page.waitForTimeout(750);
        assertOwnedProgram(ids,await spawnedPrograms(),program!);
        await page.locator(`[data-semio-local-document-id=${JSON.stringify(id)}][data-semio-local-persisted="true"]`).first().waitFor({state:"visible",timeout:120_000});
        if(await activeKeptDocument()!==id)throw new Error("reopened document UUID changed before persistence acknowledgment");
        return ids;
      }
      if (events.some((notice) => notice.startsWith("shell.openFile.") || notice.startsWith("shell.documentTransfer.file-open-failed|"))) throw new Error(`kept document ${id} refused: ${events.join(" | ")}`);
      await page.waitForTimeout(250);
    }
    throw new Error(`kept document ${id} was not reopened and acknowledged`);
  };

  const pickNatural = async (bytes: Uint8Array, name: string): Promise<string[]> => {
    let rail = await railIds();
    let bootstrap: string[] = [];
    if (!rail.includes("openArtifactFile")) {
      const before = await windowIds(page);
      const error = await openProgram();
      if (error !== null) throw new Error(error);
      bootstrap = await freshWindows(before);
      rail = await railIds();
    }
    if (!rail.includes("openArtifactFile")) throw new Error("no openArtifactFile action in the owner editor");
    const before = await windowIds(page);
    const beforeOwners = new Set((await spawnedPrograms()).map((entry) => entry.id));
    const seen = (await notices()).length;
    choosers.next = bytes;
    choosers.name = name;
    await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]');
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      signal.throwIfAborted();
      const opened = await freshWindows(before);
      const events = (await notices()).slice(seen);
      if (opened.length > 0 && events.some((notice) => notice.startsWith("shell.documentTransfer.file-opened|"))) {
        await closeNewest(bootstrap);
        await focusDocument(opened);
        await page.waitForTimeout(750);
        let correctOwner = false;
        const ownerDeadline = Date.now() + 10_000;
        while (!correctOwner && Date.now() < ownerDeadline) {
          signal.throwIfAborted();
          correctOwner = (await spawnedPrograms()).some((owner) => !beforeOwners.has(owner.id) && owner.pluginId === program!.pluginId && owner.appId === program!.appId);
          if (!correctOwner) await page.waitForTimeout(100);
        }
        assertOwnedProgram(opened,await spawnedPrograms(),program!);
        if (!correctOwner) throw new Error(`natural file opened outside declared owner ${program!.appId}`);
        return opened;
      }
      if (events.some((notice) => notice.startsWith("shell.openFile.") || notice.startsWith("shell.documentTransfer.file-open-failed|"))) throw new Error(`natural open refused: ${events.join(" | ")}`);
      await page.waitForTimeout(250);
    }
    throw new Error("natural file open produced no new document and file-opened acknowledgment");
  };
  const naturalExport = async (stage: string): Promise<Uint8Array> => {
    const rail = await railIds();
    if (!rail.includes("saveArtifactFile")) throw new Error("no saveArtifactFile action in the owner editor");
    const cursor = downloads.length;
    const seen = (await notices()).length;
    await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.saveArtifactFile"]');
    const deadline = Date.now() + 60_000;
    while (Date.now() < deadline && (downloads.length === cursor || !(await notices()).slice(seen).some((notice) => notice.startsWith("shell.documentTransfer.file-saved|")))) {
      signal.throwIfAborted();
      if ((await notices()).slice(seen).some((notice) => notice.startsWith("shell.documentTransfer.file-save-failed|"))) throw new Error("natural file save refused");
      await page.waitForTimeout(250);
    }
    const download = downloads[cursor];
    if (download === undefined || !(await notices()).slice(seen).some((notice) => notice.startsWith("shell.documentTransfer.file-saved|"))) throw new Error("natural file export lacked download or file-saved acknowledgment");
    mkdirSync(join(outDir, "exports"), { recursive: true });
    const path = join(outDir, "exports", `${row.artifact}-${stage}${row.extension}`);
    await download.saveAs(path);
    const bytes = new Uint8Array(readFileSync(path));
    const verdict = judgeBytes(row.export.format, bytes);
    if (!verdict.ok) throw new Error(`natural export refused by ${row.export.format} oracle: ${verdict.detail}`);
    return bytes;
  };

  if (program === undefined) { steps.open = no(`no #editor program of ${row.kind} in the shell catalog probe (surface ${row.surface})`); return steps; }
  let opened:string[];
  let entryDetail:string;
  if(entry==="natural-file"){
    for (const [index,payload] of (row.validPayloads??[]).entries()){
      const bytes=hexBytes(payload);const arbitrary=await pickNatural(bytes,`arbitrary-${index}${row.extension}`);const exported=await naturalExport(`arbitrary-${index}`);
      if(persistedContent(exported,"bytes")!==persistedContent(bytes,"bytes"))throw new Error(`valid arbitrary payload ${index} did not round-trip byte-for-byte`);
      if((await closeNewest(arbitrary)).length>0)throw new Error(`valid arbitrary payload ${index} did not close cleanly`);
    }
    const sample=resolveSample(row,bytes=>judgeBytes(row.oracle,bytes).ok);
    if(sample===null){steps.open=no("no pinned natural-file sample accepted by independent oracle");return steps;}
    opened=await pickNatural(new Uint8Array(readFileSync(sample)),`sample${row.extension}`);
    entryDetail=`natural-file imported ${sample}`;
  }else{
    const example=row.authoredExample;if(!example)throw new Error("no declared authored example");
    const before=await windowIds(page);const error=await openProgram();if(error!==null)throw new Error(error);
    opened=await freshWindows(before);await focusDocument(opened);
    assertOwnedProgram(opened,await spawnedPrograms(),program);
    const control=()=>page.locator(opened.map(id=>`[data-slot="window"][id=${JSON.stringify(id)}] [data-slot="window-body"] ${detailsControlSelector(example.field,"text")}`).join(",")).first();
    const picker=page.locator('[id="playground.navbar.fixture"]').first();await picker.waitFor({state:"visible",timeout:120_000});
    const executeExample=async()=>{
      if(!(await railIds()).includes("setActiveExample"))throw new Error("no declared rendered example action");
      await clickUncovered(page,'[data-slot="window-action-pane"] [id="action.setActiveExample"]');
      const result=await fillStagedArgument(page,"exampleId",example.id,"@liveId");if(result!==`exampleId=${example.id}`)throw new Error(`exact authored example argument refused: ${result}`);
      const execute=page.locator('[data-slot="window-action-pane"] [id$="setActiveExample.execute"]').first();await execute.waitFor({state:"visible",timeout:15_000});await execute.click();
    };
    await picker.click();const option=page.locator(`[role="option"][data-value=${JSON.stringify(example.id)}]`).first();await option.waitFor({state:"visible",timeout:15_000});
    if((await option.textContent())?.trim()!==example.label[locale])throw new Error("authored example option did not use the actual requested locale");
    await option.click();
    const settled=async()=>{opened=(await windowIds(page)).filter(id=>!before.includes(id));return opened.length>0&&(await control().count())>0&&(await control().inputValue())===example.baseline;};
    const awaitBaseline=async()=>{const deadline=Date.now()+120_000;while(Date.now()<deadline&&!await settled()){signal.throwIfAborted();await page.waitForTimeout(250);}if(!await settled())throw new Error("authored example did not restore its exact typed baseline");};
    await executeExample();await awaitBaseline();
    await control().waitFor({state:"visible",timeout:120_000});
    const draft=control();const beforeProbe=witness(await readShell(page));await draft.fill("Authored entry probe");await draft.press("Enter");await draft.blur();
    const deadline=Date.now()+30_000;while(Date.now()<deadline&&((await draft.inputValue())!=="Authored entry probe"||witness(await readShell(page)).edits<=beforeProbe.edits)){signal.throwIfAborted();await page.waitForTimeout(250);}
    if((await draft.inputValue())!=="Authored entry probe"||witness(await readShell(page)).edits<=beforeProbe.edits)throw new Error("authored entry probe did not commit");
    await executeExample();await awaitBaseline();
    assertOwnedProgram(opened,await spawnedPrograms(),program);
    if((await picker.textContent())?.trim()!==example.label[locale])throw new Error("authored example picker did not retain its localized selected label");
    entryDetail=`authored-example ${example.id} (${example.documentSchema}) from ${example.source}; exact typed baseline ${JSON.stringify(example.baseline)}; mandatory committed probe replaced by explicit rendered action`;

  }
  const documentId=await activeKeptDocument();
  if(documentId===null||documentId.length===0)throw new Error("document did not attach a kept-document UUID");
  steps.open=ok(`${entryDetail} into ${program.pluginId} ${program.appId}; kept UUID ${documentId}; actual locale ${locale}`);
  const unpainted = await bodyRendered(opened);
  steps.render = unpainted === null && faults.length === faultMark ? ok("body painted, no fault") : no(unpainted ?? faults.slice(faultMark).slice(-2).join(" | "));
  const baseline = persistedContent(await naturalExport("baseline"), row.persistence.comparison);
  let editedProof: string | null = null;

  const ownedControl = (selector: string) => page.locator(opened.map((id) => `[data-slot="window"][id=${JSON.stringify(id)}] [data-slot="window-body"] ${selector}`).join(",")).first();
  if (row.edit.mode === "rendered" && (row.edit.verb === "textEdit" || row.edit.verb === "set-page")) {
    const control = ownedControl(row.edit.control ?? ".semio-text-editor-host textarea:not([data-read-only])");
    const before = witness(await readShell(page));
    if ((await control.count()) === 0) steps.edit = no("no writable native text editor input");
    else {
      await control.focus();
      await control.press(process.platform === "darwin" ? "Meta+a" : "Control+a");
      await control.evaluate((element, value) => {
        const clipboard = new DataTransfer();
        clipboard.setData("text/plain", value);
        element.dispatchEvent(new ClipboardEvent("paste", { clipboardData: clipboard, bubbles: true, cancelable: true }));
      }, row.edit.value ?? "");
      const surface = control.locator("xpath=ancestor::*[contains(concat(' ', normalize-space(@class), ' '), ' semio-text-editor-host ')][1]");
      const apply = surface.getByRole("button", { name: /^(?:Apply|Anwenden)$/u }).last();
      if ((await apply.count()) > 0) await apply.click();
      const deadline = Date.now() + 30_000;
      let after = witness(await readShell(page));
      while (after.edits <= before.edits && Date.now() < deadline) { signal.throwIfAborted(); await page.waitForTimeout(250); after = witness(await readShell(page)); }
      const exactValue = (await control.inputValue()) === row.edit.value;
      steps.edit = after.edits > before.edits && exactValue ? ok(`${row.edit.verb}: native text paste committed the exact fixture value through the host revision/splice binding`) : no(`text edit was not committed exactly; edit count ${before.edits} -> ${after.edits}; exact value=${exactValue}`);
    }
  }
  else if (row.edit.mode === "rendered") {
    const selector = row.edit.path === "details" && row.edit.field && row.edit.valueKind ? detailsControlSelector(row.edit.field, row.edit.valueKind) : row.edit.control;
    if (selector === undefined) throw new Error("rendered edit has no deterministic native control");
    const control = ownedControl(selector);
    const originalValue = await control.inputValue();
    const before = witness(await readShell(page));
    await control.click({ force: true });
    await control.fill(row.edit.value ?? "");
    await control.press("Enter");
    await control.blur();
    const deadline = Date.now() + 30_000;
    let after = witness(await readShell(page));
    while (after.edits <= before.edits && Date.now() < deadline) { signal.throwIfAborted(); await page.waitForTimeout(250); after = witness(await readShell(page)); }
    const actual = await control.inputValue();
    const exact = row.edit.valueKind === "number" ? Number(actual) === Number(row.edit.value) : actual === row.edit.value;
    steps.edit = exact && actual !== originalValue && after.edits > before.edits ? ok(`${row.edit.verb}: native scalar changed from ${JSON.stringify(originalValue)} to the exact fixture value ${JSON.stringify(actual)}`) : no(`native field did not commit the expected change; ${JSON.stringify(originalValue)} -> ${JSON.stringify(actual)}; expected ${JSON.stringify(row.edit.value)}; edits ${before.edits} -> ${after.edits}`);
  }
  else {
    const key = `${program.pluginId}/${/^s\.[^.]+\.([^@]+)@/u.exec(program.appId)?.[1] ?? row.artifact}`;
    if (row.edit.mode !== "verb" || row.edit.verb === undefined || !(await railIds()).includes(row.edit.verb)) throw new Error("declared mutation verb is absent; unrelated actions are never scanned");
    const verbs = { [key]: row.edit.verb };
    const args = row.edit.verb && row.edit.args ? { [`${key}.${row.edit.verb}`]: row.edit.args } : {};
    const result = await mutateUndoRedo(page, refusals, key, verbs, args, {}, "@liveId", 1);
    steps.edit = result.mutation !== null && "mutated" in result && result.mutated ? ok(`${result.mutation}: ${JSON.stringify(result.mutationDetail ?? "applied and undo/redo differ")}`) : no(`${result.mutationDetail ?? "no verb moved the document"} (rail ${result.railRows} rows)`);
  }
  if (steps.edit?.status === "pass") {
    editedProof = persistedContent(await naturalExport("mutation"), row.persistence.comparison);
    await railIds();
    await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
    await page.waitForTimeout(1_000);
    const undone = persistedContent(await naturalExport("undo"), row.persistence.comparison);
    await railIds();
    await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.redo"]');
    await page.waitForTimeout(1_000);
    const redone = persistedContent(await naturalExport("redo"), row.persistence.comparison);
    steps.edit = undone === baseline && redone === editedProof && editedProof !== baseline ? ok(`${steps.edit.detail}; exported undo restores baseline and redo restores edited content`) : no("exported content failed edit/undo/redo round trip");
  }
  if (steps.edit?.status !== "pass") { steps.save = skipped("no proven edit to persist"); return steps; }
  const saved = await naturalExport("edited");
  const persisted = await keptDocument(documentId).getAttribute("data-semio-local-persisted") === "true";
  steps.save = persisted && await activeKeptDocument() === documentId ? ok(`Savefile acknowledged a fresh durable archive for the same kept UUID ${documentId}`) : no(`Savefile did not retain a durable current kept UUID ${documentId}`);
  const expected = persistedContent(saved, row.persistence.comparison);
  steps.export = expected !== baseline && expected === editedProof ? ok(`edited natural file differs from baseline and matches the reversible edit; ${saved.length} bytes; ${row.persistence.comparison} fingerprint ${expected}`) : no("saved export did not retain the proven reversible edit");
  if (steps.save?.status !== "pass" || steps.export.status !== "pass") return steps;
  const stuck = await closeNewest(opened);
  steps.close = stuck.length === 0 ? ok("dock tab closed") : no(`still open: ${stuck.join(",")}`);
  if (stuck.length > 0) return steps;
  const reopened = await reopenKept(documentId);
  const reopenPainted = await bodyRendered(reopened);
  const reopenedValue = persistedContent(await naturalExport("reopened"), row.persistence.comparison);
  if(entry==="authored-example"){const selector=detailsControlSelector(row.authoredExample!.field,"text");const control=page.locator(reopened.map(id=>`[data-slot="window"][id=${JSON.stringify(id)}] ${selector}`).join(",")).first();if(await control.inputValue()!==row.edit.value)throw new Error("reopened authored document did not retain its exact edited field");}
  steps.reopen = reopenPainted === null && reopenedValue === expected ? ok(`reopened kept UUID ${documentId}; edited content fingerprint ${expected}`) : no(reopenPainted ?? "reopened kept document lost the saved edit");

  await page.reload({ waitUntil: "commit" });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  assertSeatedLocale(await seatLocale(page, locale),locale);
  assertActualLocale(await page.evaluate(()=>document.documentElement.lang||null),locale);
  if (!beacon?.startsWith("ready")) {
    steps["reload-reopen"] = no(`shell did not become ready after reload: ${beacon}`);
    return steps;
  }
  const retained = await reopenKept(documentId);
  await focusDocument(retained);
  const reloadPainted = await bodyRendered(retained);
  const reloadValue = persistedContent(await naturalExport("reloaded"), row.persistence.comparison);
  if(entry==="authored-example"){const selector=detailsControlSelector(row.authoredExample!.field,"text");const control=page.locator(retained.map(id=>`[data-slot="window"][id=${JSON.stringify(id)}] ${selector}`).join(",")).first();if(await control.inputValue()!==row.edit.value)throw new Error("reloaded authored document did not retain its exact edited field");}
  steps["reload-reopen"] = reloadPainted === null && reloadValue === expected && await activeKeptDocument() === documentId ? ok(`same persisted UUID ${documentId} reopened after reload with edited content fingerprint ${expected}`) : no(reloadPainted ?? "restored document identity or saved edit changed");

  const beforeInvalid = [...new Set(await windowIds(page))].sort();
  await railIds();
  {
    choosers.next = hexBytes(row.invalid.bytes);
    choosers.name = `invalid${row.invalid.extension ?? row.extension}`;
    const seen = (await notices()).length;
    const faulted = faults.length;
    choosers.expectedInvalid = true;
    await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]');
    const deadline = Date.now() + 30_000;
    while ((await notices()).length === seen && Date.now() < deadline) { signal.throwIfAborted(); await page.waitForTimeout(250); }
    choosers.expectedInvalid = false;
    const raised = (await notices()).slice(seen);
    const pageErrors = faults.slice(faulted).filter((line) => line.startsWith("pageerror"));
    const afterInvalid = [...new Set(await windowIds(page))].sort();
    const refused = raised.some((notice) => row.invalid.kind === "unsupported-format" ? notice.startsWith("shell.openFile.format-unresolved|") : notice.startsWith("shell.documentTransfer.file-open-failed|") || notice.startsWith("shell.openFile.format-unresolved|")) && !raised.some((notice) => notice.startsWith("shell.documentTransfer.file-opened|"));
    const uncorrupted = persistedContent(await naturalExport("after-refusal"), row.persistence.comparison) === expected;
    steps.invalid = choosers.next === null && refused && pageErrors.length === 0 && uncorrupted && JSON.stringify(beforeInvalid) === JSON.stringify(afterInvalid) ? ok(`${row.invalid.kind} natural file refused without changing document content or windows: ${JSON.stringify(raised.slice(0, 2))}`) : no(`${row.invalid.kind} open did not refuse cleanly: ${JSON.stringify(raised)}; ${pageErrors.length} page errors; content preserved=${uncorrupted}`);
  }
  await closeNewest(retained).catch(() => undefined);
  return steps;
}

/** 🧭️ Runs the selected rows against the shell at `baseUrl`. */
export async function runJourney(options: Readonly<{ baseUrl: string; tag: string; locale: "en"|"de"; entry:EntryMode; only: readonly string[]; batches: readonly string[]; outDir: string; headed: boolean; signal: AbortSignal }>): Promise<JourneyReport> {
  const outDir=validateJourneyOutput(REPO_ROOT,options.outDir,options.tag,selectJourneyRows(readMatrix().artifacts,options.entry,options.only));
  mkdirSync(outDir, { recursive: true });
  const report: JourneyReport = { entry:options.entry, baseUrl: options.baseUrl, tag: options.tag, locale: options.locale, started: new Date().toISOString(), rows: [] };
  const flush = (): void => writeFileSync(join(outDir, "journey.json"), JSON.stringify(report, null, 1));
  ensureRepositoryBrowserPathV1(repoRoot,process.env);
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const scratch = resolve(outDir, "browser-tmp");
  mkdirSync(scratch, { recursive: true });
  const browser: Browser = await chromium.launch({ headless: !options.headed, args: ["--use-angle=swiftshader"], env: { ...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch } });
  const abort = (): void => { void browser.close(); };
  options.signal.addEventListener("abort", abort, { once: true });
  if (options.signal.aborted) abort();
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
    await page.addInitScript(() => {
      const events: string[] = [];
      (window as unknown as { __stdioAcceptanceNotices: string[] }).__stdioAcceptanceNotices = events;
      const observed = new WeakMap<Element, string>();
      new MutationObserver(() => {
        for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
          const text = `${element.getAttribute("data-notice-code") ?? ""}|${(element.textContent ?? "").trim()}`;
          if (observed.get(element) !== text) { observed.set(element, text); events.push(text); }
        }
      }).observe(document, { childList: true, subtree: true, attributes: true, characterData: true });
    });
    page.setDefaultNavigationTimeout(180_000);
    const faults: string[] = [];
    const refusals: string[] = [];
    const downloads: Download[] = [];
    const choosers: PickerState = { next: null, name: "", expectedInvalid: false };
    page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
    page.on("download", (download) => downloads.push(download));
    page.on("filechooser", (chooser: FileChooser) => {
      const payload = choosers.next;
      choosers.next = null;
      void chooser.setFiles(payload === null ? [] : [{ name: choosers.name, mimeType: "application/octet-stream", buffer: Buffer.from(payload) }]);
    });
    page.on("console", (message) => {
      const text = message.text();
      if (/WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u.test(text)) return;
      if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
      if (choosers.expectedInvalid && /^\[os-shell\].*(?:file-open|picked-file|natural-file)/u.test(text) && !/unreachable|trapped|\btrap\b|panicked|Uncaught|pageerror/iu.test(text)) return;
      if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 300));
    });
    await page.goto(options.baseUrl, { waitUntil: "commit", timeout: 300_000 });
    const beacon = await awaitBeacon(page, Date.now() + 300_000);
    if (!beacon?.startsWith("ready")) throw new Error(`shell never became ready: ${beacon}`);
    await dismissIntroduction(page);
    assertSeatedLocale(await seatLocale(page, options.locale),options.locale);
    const actualLocale=await page.evaluate(()=>document.documentElement.lang||null);assertActualLocale(actualLocale,options.locale);report.actualLocale=actualLocale!;
    const rows=selectJourneyRows(readMatrix().artifacts,options.entry,options.only).filter(row=>options.batches.length===0||options.batches.includes(row.batch));
    for (const row of rows) {
      if (options.signal.aborted) { report.fatal = "cancelled"; break; }
      const faultMark = faults.length;
      const entry: RowReport = { entry:options.entry, artifact: row.artifact, batch: row.batch, kind: row.kind, steps: {}, faults: [], pass: false };
      try {
        entry.steps = await drive(page, row, options.locale, options.entry, outDir, faults, refusals, downloads, choosers, entry.steps, options.signal);
      } catch (error) {
        entry.steps[STEPS.find((step) => entry.steps[step] === undefined) ?? "open"] = no(`journey threw: ${String(error).split("\n")[0]!.slice(0, 200)}`);
      }
      choosers.expectedInvalid = false;
      entry.faults = faults.slice(faultMark).slice(0, 5);
      entry.pass = entry.faults.length === 0 && STEPS.every((step) => entry.steps[step]?.status === "pass");
      if (!entry.pass) await page.screenshot({ path: join(outDir, `${row.artifact}-failed.png`), fullPage: true }).catch(() => undefined);
      report.rows.push(entry);
      flush();
      console.log(`[artifact-acceptance] ${options.entry}/${row.batch}/${row.artifact}: ${entry.pass ? "PASS" : "FAIL"} ${STEPS.map((step) => `${step}=${entry.steps[step]?.status ?? "-"}`).join(" ")}`);
    }
  } catch (error) {
    report.fatal = options.signal.aborted ? "cancelled" : String(error).split("\n")[0]!.slice(0, 300);
  } finally {
    options.signal.removeEventListener("abort", abort);
    await browser.close();
  }
  if (options.signal.aborted) report.fatal = "cancelled";
  report.finished = new Date().toISOString();
  flush();
  writeFileSync(join(outDir, "table.md"), journeyTable(report));
  return report;
}

const flag = (segments: readonly string[], name: string): string | undefined => { const value = segments[segments.indexOf(name) + 1]; return segments.includes(name) && value !== undefined && !value.startsWith("--") ? value : undefined; };

/** 🚪️ CLI entry: `run --serve <url> …`; exits non-zero unless every selected row passes every step. */
export async function runJourneyCli(segments: readonly string[]): Promise<void> {
  const serveUrl = flag(segments, "--serve");
  if (!serveUrl) throw new Error("usage: run --serve <url> --entry natural-file|authored-example --locale en|de --out <dir> [--only <artifact,…>] [--batch A,B] [--tag <t>] [--headed]");
  const locale=parseJourneyLocale(flag(segments,"--locale"));
  const entry=parseEntryMode(flag(segments,"--entry"));
  const output=flag(segments,"--out");if(!output)throw new Error("--out must name the ticket generated directory");
  const plannedRows=selectJourneyRows(readMatrix().artifacts,entry,(flag(segments,"--only")??"").split(",").filter(Boolean));
  validateJourneyOutput(REPO_ROOT,output,flag(segments,"--tag")??`${entry}-${locale}`,plannedRows);
  const controller = new AbortController();
  process.once("SIGINT", () => controller.abort());
  process.once("SIGTERM", () => controller.abort());
  await withDevServe(REPO_ROOT, "stdio-artifact-acceptance", { serveUrl, locale, signal: controller.signal, startedAt: new Date() }, async (baseUrl) => {
    const report = await runJourney({ baseUrl, tag: flag(segments, "--tag") ?? `${entry}-${locale}`, locale, entry, only: (flag(segments, "--only") ?? "").split(",").filter(Boolean), batches: (flag(segments, "--batch") ?? "").split(",").filter(Boolean), outDir: resolve(output), headed: segments.includes("--headed"), signal: controller.signal });
    const failed = report.rows.filter((row) => !row.pass).map((row) => row.artifact);
    console.log(`[artifact-acceptance] ${report.rows.length - failed.length}/${report.rows.length} artifacts pass${report.fatal ? `; fatal: ${report.fatal}` : ""}${failed.length ? `; failing: ${failed.join(", ")}` : ""}`);
    if (report.fatal || failed.length > 0 || report.rows.length === 0) process.exitCode = 1;
  });
}

if (import.meta.main) await runJourneyCli(process.argv.slice(2).filter((segment, index) => !(index === 0 && segment === "run")));
//#endregion 🔖️Journey
