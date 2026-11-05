/** 🌐️ Exercises the served Draw guest through real pointer input and independently parses its export. */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import Ajv from "ajv";
import { assertPdfDifference, assertPdfPainting } from "./🔮️oracle.mjs";

const root = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(root, "📐️.json"), "utf8")));
assert(validate(fixture), JSON.stringify(validate.errors));
const args = process.argv.slice(2);
const flag = name => args[args.indexOf(name) + 1];
for (const name of ["--serve", "--locale", "--output"]) assert(args.includes(name) && flag(name) && !flag(name).startsWith("--"), `${name} required`);
assert(fixture.locales.includes(flag("--locale")), "Explicit supported locale required");
const output = resolve(flag("--output"));
mkdirSync(output, { recursive: true });
const folder = join(output, "document");
assert(join(folder, ".semio", "canonical-bootstrap.db").length <= 256, "Generated persistence path exceeds 256 characters");
assert(!existsSync(folder), "Acceptance requires a fresh document folder; choose a new output directory");
mkdirSync(folder, { recursive: true });
const url = new URL(flag("--serve"));
url.searchParams.set("plugin", "draw");
const report = { url: url.href, locale: flag("--locale"), steps: [], errors: [] };
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const cancel = signal => {
  report.failure = `Cancelled by ${signal}`;
  process.exitCode = 1;
  void browser.close().catch(() => undefined);
};
const interrupt = () => cancel("SIGINT");
const terminate = () => cancel("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
const context = await browser.newContext({ locale: report.locale, viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
const page = await context.newPage();
const errors = report.errors;
const workerObservers = [];
page.on("worker", worker => {
  workerObservers.push(worker.evaluate(() => {
    self.addEventListener("error", event => console.error("Draw worker error", event.message));
    self.addEventListener("unhandledrejection", event => console.error("Draw worker rejection", String(event.reason)));
  }).catch(error => errors.push(`Worker observation failed: ${error}`)));
});
page.on("pageerror", error => errors.push(String(error)));
page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
const state = () => page.evaluate(() => {
  const status = document.body.innerText.match(/(\d+) (?:layers?|Ebenen?) · (\d+) (?:selected|ausgewählt)/);
  return { ready: document.documentElement.dataset.semioOsReady, error: document.documentElement.dataset.semioOsError, layers: status ? Number(status[1]) : null, locale: status ? (status[0].includes("ausgewählt") ? "de" : "en") : null };
});
const waitLayers = async expected => {
  await page.waitForFunction(count => {
    const status = document.body.innerText.match(/(\d+) (?:layers?|Ebenen?) · (\d+) (?:selected|ausgewählt)/);
    return status && Number(status[1]) === count;
  }, expected, { timeout: 30000 });
};
const click = id => page.locator(`[id="${id}"]`).first().click({ timeout: 15000 });
const openFolderCard = async () => {
  const panel = page.locator('[data-slot="panel"][id="framework.panelTab.s-sync-status"]').first();
  if (!(await panel.isVisible())) await click("s-sync-status");
  const utility = page.locator('[id="framework.sync.folder"]').first();
  if (!(await utility.isVisible())) await click("ui.utilities.group.sync");
  await utility.click({ timeout: 15000 });
  await page.locator('[id="framework.sync.folder.path"]').waitFor({ state: "visible", timeout: 15000 });
};
const waitSaved = async () => {
  await page.waitForFunction(() => [...document.querySelectorAll('[data-semio-sync-status]')].some(element => /(?:^|·)\s*(?:saved|gespeichert)(?:\s*·|$)/i.test(element.textContent ?? "")), undefined, { timeout: 60000 });
};
const note = async step => {
  await Promise.all(workerObservers);
  assert.equal(errors.length, 0, errors.join("\n"));
  const current = await state();
  assert.equal(current.ready, fixture.ready, JSON.stringify(current));
  assert.equal(current.error, undefined, JSON.stringify(current));
  assert.equal(current.locale, report.locale, "Draw must render the explicitly requested language");
  report.steps.push({ step, ...current });
  writeFileSync(join(output, "report.json"), JSON.stringify(report, null, 2));
  if (step === "rectangle" || step === "reload") await page.screenshot({ path: join(output, `${step}.png`) });
  console.log(`[draw-browser] ${step}`);
};
const exportPdf = async name => {
  await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click({ timeout: 15000 });
  await click("action.exportDocument");
  await page.locator('[id="format"][role="combobox"]').first().click({ timeout: 15000 });
  await page.getByRole("option", { name: "PDF", exact: true }).click({ timeout: 15000 });
  console.log("[draw-browser] waiting for PDF export");
  const [download] = await Promise.all([
    page.waitForEvent("download", { timeout: 60000 }),
    page.locator('[id^="framework.window."][id$=".action.exportDocument.execute"]').first().click({ timeout: 15000 }),
  ]);
  const pdfPath = join(output, name);
  await download.saveAs(pdfPath);
  assert.equal(await download.failure(), null);
  const pixels = await assertPdfPainting(readFileSync(pdfPath), fixture.export.pages);
  await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click({ timeout: 15000 });
  return pixels;
};
await page.addInitScript(faults => {
  const NativeWorker = globalThis.Worker;
  globalThis.Worker = class extends NativeWorker {
    constructor(...arguments_) {
      super(...arguments_);
      this.addEventListener("message", event => {
        if (faults.includes(event.data?.kind)) console.error("Draw guest fault", JSON.stringify(event.data));
      });
      this.addEventListener("error", event => console.error("Draw worker error", event.message));
      this.addEventListener("messageerror", () => console.error("Draw worker message error"));
    }
  };
}, fixture.guestFaults);
try {
  console.log("[draw-browser] opening default Draw session");
  await page.goto(url.href, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  assert.equal((await state()).ready, fixture.ready, JSON.stringify(await state()));
  const canvas = page.locator('[data-surface-id="drawing.play.composite"] canvas').first();
  await canvas.waitFor({ state: "visible", timeout: 30000 });
  await page.waitForFunction(minimum => {
    const status = document.body.innerText.match(/(\d+) (?:layers?|Ebenen?) · \d+ (?:selected|ausgewählt)/);
    return status && Number(status[1]) >= minimum;
  }, fixture.minimumBootLayers, { timeout: 30000 });
  const before = (await state()).layers;
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  assert((await picker.innerText()).includes(fixture.example.label), "Default Draw example must be selected");
  await note("boot");
  await picker.click({ timeout: 15000 });
  await page.getByRole("option", { name: fixture.example.label, exact: true }).waitFor({ state: "visible", timeout: 15000 });
  assert.equal(await page.getByRole("option").count(), 2, "Draw advertises Demo and the framework's blank option");
  await page.getByRole("option").filter({ hasNotText: fixture.example.label }).click({ timeout: 15000 });
  await waitLayers(0);
  await note("blank");
  await picker.click({ timeout: 15000 });
  await page.getByRole("option", { name: fixture.example.label, exact: true }).click({ timeout: 15000 });
  await waitLayers(before);
  assert((await picker.innerText()).includes(fixture.example.label));
  await note("example");
  await openFolderCard();
  await page.locator('[id="framework.sync.folder.path"]').fill(folder);
  await page.getByRole("button", { name: /^(attach|verbinden)$/i }).click({ timeout: 15000 });
  await waitSaved();
  await page.keyboard.press("Escape");
  await note("attach");
  const eventPath = join(folder, ".semio", "events.semio");
  const beforePixels = await exportPdf("before.pdf");
  await openFolderCard();
  await waitSaved();
  await page.keyboard.press("Escape");
  const beforeEventBytes = readFileSync(eventPath).byteLength;
  await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().click({ timeout: 15000 });
  await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().click({ timeout: 15000 });
  await click(fixture.utility);
  const rect = await canvas.boundingBox();
  assert(rect && rect.width > 0 && rect.height > 0, "Nonempty Draw canvas required");
  const [x0, y0] = fixture.gesture.from, [x1, y1] = fixture.gesture.to;
  await page.mouse.move(rect.x + rect.width * x0, rect.y + rect.height * y0);
  await page.mouse.down();
  await page.mouse.move(rect.x + rect.width * x1, rect.y + rect.height * y1, { steps: fixture.gesture.steps });
  await page.mouse.up();
  await waitLayers(before + fixture.layerDelta);
  await note("rectangle");
  const mod = process.platform === "darwin" ? "Meta" : "Control";
  await page.keyboard.press(`${mod}+z`);
  await waitLayers(before);
  await note("undo");
  await page.keyboard.press(`${mod}+Shift+z`);
  await waitLayers(before + fixture.layerDelta);
  await note("redo");
  await openFolderCard();
  await waitSaved();
  report.persistence = { folder, beforeEventBytes, eventLogBytes: readFileSync(eventPath).byteLength };
  assert(report.persistence.eventLogBytes - beforeEventBytes >= fixture.persistence.minimumEventGrowthBytes, "Current rectangle/history events must grow the fresh folder log");
  await page.keyboard.press("Escape");
  await note("persisted");
  report.export = assertPdfDifference(beforePixels, await exportPdf("drawing.pdf"));
  await note("export");
  console.log("[draw-browser] reloading persisted document");
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  assert.equal((await state()).ready, fixture.ready, JSON.stringify(await state()));
  assert.equal((await state()).error, undefined, JSON.stringify(await state()));
  await click(fixture.persistence.reconnect);
  await waitLayers(before + fixture.layerDelta);
  await note("reload");
  assert.deepEqual(report.steps.map(row => row.step), fixture.steps);
} catch (error) {
  report.failure ??= String(error);
  await page.screenshot({ path: join(output, "failure.png") }).catch(() => undefined);
  throw error;
} finally {
  writeFileSync(join(output, "report.json"), JSON.stringify(report, null, 2));
  await browser.close();
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
}
