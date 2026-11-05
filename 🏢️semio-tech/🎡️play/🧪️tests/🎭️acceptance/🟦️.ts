// #region 🧲️Header
// 💻️ 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts
// Specs: End-to-end acceptance coverage for every Play editor, curated example command and viewer round trip.
// Summary: The overview must list one card per authored pane. Then, for every pane, deep-links to
// `/#<paneId>` (which boots exactly that pane immediately), waits for its own `FrameworkOsShell` to report
// an outcome through the per-shell `data-shell-ready`/`data-shell-error`/`data-shell-not-found` beacon,
// requires "ready", lets the boot example announcement settle, requires the pane's error boundary to stay
// silent and fails on any page error, console error, HTTP error or refused input. It then requires VISIBLE
// content: the curated example's own rendered label in the pane's chrome, and a main window that actually
// paints — a non-uniform pixel census of the largest canvas, or a non-empty main region for a DOM window.
// Across the whole run no asset request may be answered by the SPA fallback. The overview button must
// return to the overview after clearing and restoring its curated document and switching viewer/editor roles.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { PNG } from "pngjs";
import Ajv2020 from "ajv/dist/2020.js";
import generation3dWorkflow from "./🧫️fixtures/🧊️generation3d/🔣️.json";
import formsWorkflow from "./🧫️fixtures/📋️forms/🔣️.json";
import formsWorkflowSchema from "./🧫️fixtures/📋️forms/🧬️schema/🔣️.json";
import cancellationWorkflow from "./🧫️fixtures/📋️forms/🌐️cancellation.json";
import cancellationWorkflowSchema from "./🧫️fixtures/📋️forms/🌐️cancellation.schema.json";

import { expect, test, type Page } from "../../🔨️modules/🧪️e2e/📦️release/🎭️fixture/🟦️.ts";
// #endregion 🔌️Adapters

const validateFormsWorkflow = new Ajv2020({ strict: true }).compile(formsWorkflowSchema);
if (!validateFormsWorkflow(formsWorkflow)) throw new Error(`Invalid Forms browser scenario: ${JSON.stringify(validateFormsWorkflow.errors)}`);
const validateCancellationWorkflow = new Ajv2020({ strict: true }).compile(cancellationWorkflowSchema);
if (!validateCancellationWorkflow(cancellationWorkflow)) throw new Error(`Invalid Forms cancellation scenario: ${JSON.stringify(validateCancellationWorkflow.errors)}`);

/** ⌨️ Replaces an editor value through the attached real session and waits for its authored echo. */
async function replaceEditorText(page: Page, editor: ReturnType<Page["locator"]>, value: string): Promise<void> {
  await expect(editor.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
  await editor.focus();
  await editor.press("ControlOrMeta+A");
  if (value.length === 0) await editor.press("Backspace");
  else await page.keyboard.insertText(value);
  await expect(editor).toHaveValue(value);
}

/** 📝️ Authors a real catalogue question and edits its complete paged source through the visible control. */
async function authorFormsFlow(page: Page, scope: ReturnType<Page["locator"]>, kind: string, source: string, label: string): Promise<string> {
  const builder = scope.locator("[data-surface-id=\"forms.play.blueprint\"]");
  await builder.locator("[data-block-id=\"name\"] button[aria-pressed]").click();
  const cards = builder.locator("[data-block-id]"), before = await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")));
  await builder.locator(`[data-block-kind="${kind}"]`).click();
  await expect(cards).toHaveCount(before.length + 1);
  const id = (await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")))).find(id => !before.includes(id))!;
  await builder.locator(`[data-block-id="${id}"] button[aria-pressed]`).click();
  const document = scope.getByRole("textbox", { name: label, exact: true });
  await expect(document.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
  await document.focus();
  await document.press("ControlOrMeta+A");
  await page.keyboard.insertText(source);
  await expect(document).toHaveValue(source);
  return id;
}


/** 📏️ Measures retained public mesh coordinates independently of provider parameters or progress state. */
async function worldMeshHeights(scope: ReturnType<Page["locator"]>): Promise<number[]> {
  const encoded = await scope.locator("[data-meshes-json]").first().getAttribute("data-meshes-json");
  if (encoded === null) return [];
  const heights = (JSON.parse(encoded) as { data: string }[]).map(mesh => {
    const { positions } = JSON.parse(mesh.data) as { positions: number[] };
    expect(positions.length).toBeGreaterThan(0);
    expect(positions.length % 3).toBe(0);
    expect(positions.every(Number.isFinite)).toBe(true);
    const z = positions.filter((_value, index) => index % 3 === 2);
    return Math.round((Math.max(...z) - Math.min(...z)) * 100_000) / 100_000;
  });
  return [...new Set(heights)].sort((a, b) => a - b);
}

/** 🖼️ Compares the same interior pixel coordinates using the independent PNG decoder. */
function changedInteriorPixels(before:Buffer,after:Buffer):number{
 const a=PNG.sync.read(before),b=PNG.sync.read(after);expect([b.width,b.height]).toEqual([a.width,a.height]);const inset=Math.max(1,Math.floor(Math.min(a.width,a.height)/4));let changed=0;
 for(let y=inset;y<a.height-inset;y++)for(let x=inset;x<a.width-inset;x++){const index=(y*a.width+x)*4;if(a.data[index]!==b.data[index]||a.data[index+1]!==b.data[index+1]||a.data[index+2]!==b.data[index+2])changed++;}
 return changed;
}

//#region 🪪️PlayPanes
type PlayPane = { readonly variant: string; readonly label: string; readonly example?: string; readonly exampleLabel?: string };

/** 🪪️ Reads the pane catalog as data instead of importing app modules, which drag the React runtime into Node's loader. */
function playPanes(): readonly PlayPane[] {
  const catalog = JSON.parse(readFileSync(join(import.meta.dirname, "..", "..", "🔨️modules", "🧩️runtime", "🔣️.json"), "utf8")) as { readonly groups: readonly { readonly panes: readonly PlayPane[] }[] };
  return catalog.groups.flatMap(group => group.panes);
}

type SurfaceRole = "editor" | "viewer";
const roleContract = JSON.parse(readFileSync(join(import.meta.dirname, "🧫️fixtures/🔀️roles/🔣️.json"), "utf8")) as {
  readonly initialRole: SurfaceRole;
  readonly dismissControlId: string;
  readonly settlement: { readonly controlId: string; readonly busy: boolean; readonly beforePaint: boolean };
  readonly steps: readonly { readonly role: SurfaceRole; readonly controlId: string; readonly painted: boolean }[];
};
//#endregion 🪪️PlayPanes

/** ⏱️ Cold wasm plugin boots can be slow — generous so the suite reports real defects, not infrastructure latency. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 240_000;

/** 🕊️ Quiet window after the network settles: the shell announces the boot example after "ready",
 * and a refused announcement is only logged once the guest answers. */
const BOOT_SETTLE_MS = 2_000;

/** 📄️ A DOM main region with neither text nor this many elements rendered nothing a reader sees. */
const MINIMUM_MAIN_REGION_ELEMENTS = 10;

/** 🖼️ The smallest edge an image asset may have and still be the CONTENT of the pixel layer it
 * fills. `raster`'s composite declares a 1024×1024 `pixel` layer whose `imageKey` resolves to the 2×2
 * `semio-emblem` placeholder, decodes nothing (`📓️raster.md`: `atob` is never called) and paints an empty
 * canvas — while "the document has a layer and the scene has an asset" reads perfectly healthy. Eight
 * pixels a side is far below any authored demo and far above a placeholder. */
const MINIMUM_IMAGE_ASSET_EDGE = 8;

/** ⏳️ A retained lane can land after the boot settle — `energy`'s 4 542-character `data-meshes-json`
 * was still empty 2.5 s after `data-shell-ready` and full at 4 s — so the witness is polled rather than
 * read once. A painted pane answers on the first read (≈20 ms); only a genuinely empty one pays the wait. */
const PAINT_WITNESS_DEADLINE_MS = 4_000;
const PAINT_WITNESS_POLL_MS = 250;

/** 👂️ Collects page errors, console errors and refused inputs — a refused input is the shell's
 * "The input could not be delivered" notice, logged only as a warning. */
function collectErrors(page: Page): { readonly pageErrors: string[]; readonly consoleErrors: string[]; readonly refusedInputs: string[]; readonly resourceErrors: string[]; readonly stop: () => void } {
  const pageErrors: string[] = [], consoleErrors: string[] = [], refusedInputs: string[] = [], resourceErrors: string[] = [];
  let collecting = true;
  const stop = () => { collecting = false; };
  page.on("close", stop);
  page.on("pageerror", error => { if (collecting) pageErrors.push(`${error.name}: ${error.message}`); });
  page.on("console", message => {
    if (!collecting) return;
    if (message.type() === "error") consoleErrors.push(message.text());
    else if (/^input #\d+ .* refused: /.test(message.text())) refusedInputs.push(message.text());
  });
  page.on("response", response => {
    if (collecting && response.status() >= 400) resourceErrors.push(`${response.status()} ${response.url()}`);
  });
  page.on("requestfailed", request => {
    if (collecting && !page.isClosed()) resourceErrors.push(`${request.failure()?.errorText ?? "network request failed"} ${request.method()} ${request.url()}`);
  });
  return { pageErrors, consoleErrors, refusedInputs, resourceErrors, stop };
}

//#region 🖼️VisibleContent
/** 📦️ Request paths that must be answered by a FILE. A dev server's SPA fallback happily answers
 * `GET /🖼️assets/🖼️bauteilbörse.png` with `index.html` and HTTP 200, so a missing figure looks like a
 * loaded one to everything except the eye — exactly how `animate`'s missing deck figure survived audit #1
 * and a whole strict acceptance run (ticket 26/09/19 `📓️status.md`, 2026-09-22 04:10). */
const ASSET_PATH = /\.(png|jpe?g|gif|webp|avif|svg|ico|bmp|glb|gltf|obj|stl|ply|3dm|mp3|mp4|webm|wav|ogg|woff2?|ttf|otf|pdf)$/i;

/** 🏚️ Collects every asset request the server answered with a document instead of the asset. */
function collectSpaFallbackAssets(page: Page): string[] {
  const answered: string[] = [];
  page.on("response", response => {
    const url = response.url();
    if (!/^https?:/i.test(url)) return;
    let pathname: string;
    try { pathname = new URL(url).pathname; } catch { return; }
    if (!ASSET_PATH.test(decodeURIComponent(pathname))) return;
    const type = response.headers()["content-type"] ?? "";
    if (type.toLowerCase().startsWith("text/html")) answered.push(`${pathname} → ${response.status()} ${type}`);
  });
  return answered;
}

/** 🪟️ One mounted window's verdict on whether it PAINTED anything, read from what its host
 * publishes rather than from pixels. */
type WindowWitness = { readonly surface: string; readonly kind: "world3d" | "paint2d" | "dom"; readonly painted: boolean; readonly detail: string };
type PaintWitness = { readonly painted: boolean; readonly detail: string };

/** 🔬️ Asks every window of the pane, in its OWN terms, whether it painted — evaluated inside the
 * page so one round trip answers for the whole pane (≈20 ms).
 *
 * Pixels alone cannot answer this. An element screenshot is the PAGE screenshot clipped to the element box, so
 * the window chrome that overlaps a full-bleed viewport (header, gutters, overlays) lands inside the crop
 * and a blank `raster` composite censuses 118–316 distinct colours; and a `drawImage` readback of the live
 * canvas returns transparent for every WebGPU surface in this host (`raster`'s composite reports
 * `getContext("webgpu")`), painted or not. Both routes were measured on :6033 and both are false greens.
 *
 * So each window is asked what it retains:
 * — a 3D world window publishes its retained geometry on its own surface element
 *   (`data-meshes-json` / `data-instances-json`, written by `🌐️World3dHost`) — it must hold at least one
 *   mesh or instance, a draw receipt from its current mounted geometry and varying interior
 *   pixels in a physical canvas screenshot;
 * — a raster window publishes its layer forest and every texture's decoded extent on its own surface
 *   element (`data-layers-json` / `data-assets-json`, written by `🖌️Paint2dHost`) — it must hold at
 *   least one VISIBLE layer whose image asset is at least `minimumEdge` pixels a side, which is what
 *   separates a composite that draws from a layer over a 2×2 placeholder;
 * — every other window is judged by its own region's text and element count.
 *
 * The pane passes when ANY of its windows paints: `sourcing` and `energy` mount a 3D viewport next to
 * tables, and `📓️audit-visual-2.md` calls `sourcing`'s viewport correctly near-empty while the pane is
 * full of content, so a rule keyed to the largest window alone would red a healthy pane. */
async function readPaintWitnesses(page: Page, variant: string): Promise<readonly WindowWitness[]> {
  return page.evaluate(({ id, minimumElements, minimumEdge }) => {
    const pane = document.querySelector(`[data-layered-pane="${id}"]`);
    if (!pane) return [];
    const area = (element: Element): number => { const box = element.getBoundingClientRect(); return box.width * box.height; };
    const size = (text: string | null): number => {
      if (typeof text !== "string" || text.length === 0) return 0;
      try { const value = JSON.parse(text); return Array.isArray(value) ? value.length : value && typeof value === "object" ? Object.keys(value).length : 0; } catch { return 0; }
    };
    const domWitness = (region: Element, surface: string): WindowWitness => {
      const content = region.querySelector(':scope > [data-slot="pane-host-root"]');
      if (!content || area(content) === 0) return { surface, kind: "dom", painted: false, detail: "the window mounted no visible content host" };
      const clone = content.cloneNode(true) as HTMLElement;
      clone.querySelectorAll('[data-slot="pane-host"], [role="status"][aria-busy="true"], [hidden], [aria-hidden="true"]').forEach((element) => element.remove());
      const text = (clone.textContent ?? "").trim().length;
      const elements = clone.querySelectorAll("*").length;
      return { surface, kind: "dom", painted: text > 0 || elements >= minimumElements, detail: `${text} content characters, ${elements} content elements` };
    };
    const surfaces = [...pane.querySelectorAll("[data-surface-id]")];
    if (surfaces.length === 0) {
      return [...pane.querySelectorAll('[data-slot="window-body"]')].sort((a, b) => area(b) - area(a)).map((body, index) => domWitness(body, `(window body ${index})`));
    }
    return surfaces.sort((a, b) => area(b) - area(a)).map((element): { surface: string; kind: "world3d" | "paint2d" | "dom"; painted: boolean; detail: string } => {
      const surface = element.getAttribute("data-surface-id") ?? "(unnamed)";
      const meshes = element.getAttribute("data-meshes-json"), instances = element.getAttribute("data-instances-json");
      if (meshes !== null || instances !== null) {
        return { surface, kind: "world3d", painted: area(element) > 0 && size(meshes) + size(instances) > 0 && (id !== "generation3d" || [...element.querySelectorAll<HTMLCanvasElement>("canvas")].some(canvas => canvas.dataset.worldGeometryRevision === (element as HTMLElement).dataset.worldGeometryRevision && Number(canvas.dataset.worldGeometryVertices) > 0)), detail: `${size(meshes)} meshes, ${size(instances)} instances; drawn geometry=${[...element.querySelectorAll<HTMLCanvasElement>("canvas")].map(canvas => canvas.dataset.worldGeometryDrawn ?? "none").join(",")}` };
      }
      const layersJson = element.getAttribute("data-layers-json"), assetsJson = element.getAttribute("data-assets-json");
      if (layersJson !== null || assetsJson !== null || element.classList.contains("semio-paint-2d-canvas-surface")) {
        if (layersJson === null || assetsJson === null) return { surface, kind: "paint2d", painted: false, detail: "the paint surface published no data-layers-json/data-assets-json witness" };
        const assets = (() => { try { return JSON.parse(assetsJson ?? "{}") as Record<string, { width?: number | null; height?: number | null }>; } catch { return {}; } })();
        const layers = (() => { try { const value = JSON.parse(layersJson ?? "[]"); return (Array.isArray(value) ? value : []) as readonly Record<string, unknown>[]; } catch { return []; } })();
        const drawn = layers.filter((layer) => layer.visible !== false && typeof layer.imageKey === "string").map((layer) => {
          const asset = assets[layer.imageKey as string];
          return { layer: String(layer.id ?? layer.imageKey), key: String(layer.imageKey), w: asset?.width ?? null, h: asset?.height ?? null, present: asset !== undefined };
        });
        const painted = drawn.filter((row) => row.w !== null && row.h !== null && row.w >= minimumEdge && row.h >= minimumEdge);
        const shown = drawn.map((row) => `${row.layer}→${row.key} ${!row.present ? "(missing asset)" : row.w !== null && row.h !== null ? `${row.w}×${row.h}` : "(no readable extent)"}`).join(", ") || "no visible image layer";
        return { surface, kind: "paint2d", painted: painted.length > 0, detail: `${layers.length} layers, ${Object.keys(assets).length} assets: ${shown}` };
      }
      const region = element.closest('[data-slot="window-body"]') ?? element;
      return domWitness(region, surface);
    });
  }, { id: variant, minimumElements: MINIMUM_MAIN_REGION_ELEMENTS, minimumEdge: MINIMUM_IMAGE_ASSET_EDGE });
}

/** 🖼️ Polls {@link readPaintWitnesses} until some window paints or the deadline passes, and renders
 * the verdict of every window so a failure names the exact empty one. */
async function paintWitness(page: Page, variant: string): Promise<PaintWitness> {
  const qualifies = (witness: WindowWitness) => witness.painted && (variant !== "generation3d" || witness.kind === "world3d");
  const deadline = Date.now() + PAINT_WITNESS_DEADLINE_MS;
  let witnesses = await readPaintWitnesses(page, variant);
  while (!witnesses.some(qualifies) && Date.now() < deadline) {
    await page.waitForTimeout(PAINT_WITNESS_POLL_MS);
    witnesses = await readPaintWitnesses(page, variant);
  }
  for (const witness of witnesses) {
    if (variant !== "generation3d" || witness.kind !== "world3d" || !witness.painted) continue;
    const canvas = page.locator(`[data-layered-pane="${variant}"] [data-surface-id="${witness.surface}"] canvas`).first();
    const image = PNG.sync.read(await canvas.screenshot());
    const colors = new Set<number>();
    const inset = Math.max(1, Math.floor(Math.min(image.width, image.height) / 4));
    for (let y = inset; y < image.height - inset; y += 2) for (let x = inset; x < image.width - inset; x += 2) {
      const index = (y * image.width + x) * 4;
      colors.add((image.data[index]! << 16) | (image.data[index + 1]! << 8) | image.data[index + 2]!);
    }
    expect(colors.size, `${variant}: actual GPU geometry draw must produce varying interior canvas pixels`).toBeGreaterThanOrEqual(generation3dWorkflow.gpu.minimumInteriorColors);
  }
  const detail = witnesses.length === 0 ? "the pane mounted no window at all" : witnesses.map((witness) => `${witness.surface} [${witness.kind}] ${witness.painted ? "paints" : "EMPTY"} — ${witness.detail}`).join("; ");
  return { painted: witnesses.some(qualifies), detail };
}
//#endregion 🖼️VisibleContent

type ShellOutcome = "ready" | "error" | "notFound";

/** 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome. */
async function waitForPaneShellOutcome(page: Page, paneId: string): Promise<ShellOutcome> {
  await page.waitForFunction(id => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
    return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
  }, paneId, { timeout: SHELL_READY_TIMEOUT_MS });
  return page.evaluate(id => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement;
    return el.dataset.shellReady !== undefined ? "ready" : el.dataset.shellError !== undefined ? "error" : "notFound";
  }, paneId) as Promise<ShellOutcome>;
}

/** 🎓️ Closes an app's first-use tour through its own published control before exercising its document. */
async function dismissPaneIntroduction(page: Page, pane: PlayPane): Promise<void> {
  const shell = page.locator(`[data-shell-id="${pane.variant}"]`);
  const dismiss = shell.locator(`[id="${roleContract.dismissControlId}"]`);
  if (await dismiss.count() === 0) return;
  await dismiss.click();
  await expect(shell).not.toHaveAttribute("data-introduction-active", "true");
}

/** 🧪️ Delivers a blank example command and restores the curated document through the actual picker. */
async function exerciseCuratedExample(page: Page, pane: PlayPane): Promise<void> {
  if (pane.exampleLabel === undefined) return;
  const picker = page.locator(`[data-layered-pane="${pane.variant}"] [id="playground.navbar.fixture"]`);
  await expect(picker).toBeVisible();
  await expect(picker).toHaveText(pane.exampleLabel);
  await picker.click();
  const blank = page.getByRole("option").first();
  const blankLabel = (await blank.innerText()).trim();
  await blank.click();
  await expect(picker).toHaveText(blankLabel);
  await page.waitForTimeout(BOOT_SETTLE_MS);
  await picker.click();
  await page.getByRole("option", { name: pane.exampleLabel, exact: true }).click();
  await expect(picker).toHaveText(pane.exampleLabel);
  await page.waitForTimeout(BOOT_SETTLE_MS);
  const witness = await paintWitness(page, pane.variant);
  expect(witness.painted, `${pane.variant} restoring its curated example paints nothing — ${witness.detail}`).toBe(true);
}

/** 🚦️ Awaits the role transaction's guest refresh before inspecting its published window bodies. */
async function waitForSurfaceSettlement(page: Page, pane: PlayPane): Promise<void> {
  const shell = page.locator(`[data-shell-id="${pane.variant}"]`);
  const group = shell.locator(`[id="${roleContract.settlement.controlId}"]`);
  await expect(group).toBeVisible();
  await expect.poll(async () => await group.getAttribute("aria-busy") === "true", { timeout: SHELL_READY_TIMEOUT_MS }).toBe(roleContract.settlement.busy);
  expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
  await expect(shell).not.toHaveAttribute("data-shell-error");
  await expect(page.locator(`[data-layered-pane="${pane.variant}"] [data-layered-pane-error]`)).toHaveCount(0);
}

/** 🔀️ Reopens the same dialect on each surface and verifies the active application independently. */
async function exerciseSurfaceRoundTrip(page: Page, pane: PlayPane): Promise<void> {
  const shell = page.locator(`[data-shell-id="${pane.variant}"]`);
  const chip = shell.locator('[data-slot="surface-role-chip"]');
  await expect(chip).toHaveAttribute("data-role", roleContract.initialRole);
  const appId = await chip.getAttribute("data-app-id");
  expect(appId).toMatch(/#editor$/);
  const dialect = appId!.slice(0, appId!.lastIndexOf("#"));
  for (const step of roleContract.steps) {
    const control = shell.locator(`[id="${step.controlId}"]`);
    await expect(control).toBeVisible();
    await expect(control).toBeEnabled();
    await control.click();
    await expect(control).toHaveAttribute("aria-pressed", "true");
    await expect(chip).toHaveAttribute("data-role", step.role);
    await expect(chip).toHaveAttribute("data-app-id", `${dialect}#${step.role}`);
    await waitForSurfaceSettlement(page, pane);
    await expect(chip).toHaveAttribute("data-app-id", `${dialect}#${step.role}`);
    await dismissPaneIntroduction(page, pane);
    const witness = await paintWitness(page, pane.variant);
    expect(witness.painted, `${pane.variant} ${step.role} paints nothing — ${witness.detail}`).toBe(step.painted);
  }
}

test.describe("semio-tech play", () => {
  test.use({locale:"en-US"});
  test("lists one overview card per app", async ({ page }) => {
    const errors = collectErrors(page);
    const fallbackAssets = collectSpaFallbackAssets(page);
    const panes = playPanes();
    await page.goto(".");
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-layered-card]")).toHaveCount(panes.length);
    await expect(page.locator('[data-slot="play-app-count"]')).toHaveText(`${panes.length} apps`);
    for (const pane of panes) await expect(page.locator(`[data-layered-card="${pane.variant}"] [data-overview-card]`)).toHaveAttribute("aria-label", new RegExp(`^Open ${pane.label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}: `));
    errors.stop();
    expect(errors.pageErrors).toEqual([]);
    expect(errors.consoleErrors).toEqual([]);
    expect(errors.resourceErrors).toEqual([]);
    expect(fallbackAssets, "assets answered by the SPA fallback").toEqual([]);
  });

  for (const language of generation3dWorkflow.locales) test.describe(`3d ${language.browser}`, () => {
  test.use({locale:language.browser});
  test("edit", async ({page},testInfo) => {
    test.setTimeout(TEST_TIMEOUT_MS);
    const errors = collectErrors(page);
    const variant = generation3dWorkflow.pane;
    const pane = playPanes().find(pane => pane.variant === variant)!;
    await page.goto(`./#${variant}`);
    expect(await waitForPaneShellOutcome(page, variant)).toBe("ready");
    await dismissPaneIntroduction(page, pane);
    const graph = page.locator(`[data-layered-pane="${variant}"] ${generation3dWorkflow.graph}`).first();
    await expect(graph).toBeVisible();
    const host = JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!) as {widgets: {kind:string;id:string;value?:number;max?:number;min?:number}[];synapses:{from:string;to:string;toPort:string}[]};
    const slider = host.widgets.find(widget => widget.kind === "inputSlider" && typeof widget.value === "number" && typeof widget.max === "number" && widget.value + generation3dWorkflow.increment <= widget.max);
    expect(slider, "curated generator must expose a real editable geometry parameter").toBeDefined();
    await graph.focus();
    for (let step = 0; step <= host.widgets.length; step++) {
      if ((await graph.getAttribute("data-diagram-focused-node")) === slider!.id) break;
      await graph.press(generation3dWorkflow.focusKey);
    }
    await expect(graph).toHaveAttribute("data-diagram-focused-node", slider!.id);
    await graph.press(generation3dWorkflow.selectKey);
    const input = page.locator(`[data-layered-pane="${variant}"] [data-ui-node-key="${generation3dWorkflow.controlKey}"]`);
    await expect(input).toBeVisible();
    await expect(input).toHaveAttribute("aria-label", language.valueLabel);
    await expect(input).toHaveValue(String(slider!.value));
    const world = page.locator(`[data-layered-pane="${variant}"] [data-meshes-json]`).first();
    const beforeMeshes = await world.getAttribute("data-meshes-json");
    expect((await paintWitness(page,variant)).painted).toBe(true);
    const canvas=world.locator("canvas").first();
    const beforePixels=await canvas.screenshot({path:testInfo.outputPath("before.png")});
    const changed = slider!.value! + generation3dWorkflow.increment;
    await input.fill(String(changed));
    await input.press(generation3dWorkflow.commitKey);
    const currentValue = async () => JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!).widgets.find((widget:{id:string})=>widget.id===slider!.id).value;
    await expect.poll(currentValue).toBe(changed);
    await expect.poll(()=>world.getAttribute("data-meshes-json")).not.toBe(beforeMeshes);
    expect((await paintWitness(page, variant)).painted).toBe(true);
    const changedPixels=await canvas.screenshot({path:testInfo.outputPath("edit.png")});
    expect(changedInteriorPixels(beforePixels,changedPixels),"the committed geometry edit must change physical interior canvas pixels").toBeGreaterThanOrEqual(generation3dWorkflow.gpu.minimumChangedInteriorPixels);
    await graph.focus();
    await graph.press(generation3dWorkflow.history.undo);
    await expect.poll(currentValue).toBe(slider!.value);
    await expect.poll(()=>world.getAttribute("data-meshes-json")).toBe(beforeMeshes);
    expect((await paintWitness(page,variant)).painted).toBe(true);
    const undoPixels=await canvas.screenshot({path:testInfo.outputPath("undo.png")});
    expect(changedInteriorPixels(changedPixels,undoPixels),"undo must visibly restore geometry").toBeGreaterThanOrEqual(generation3dWorkflow.gpu.minimumChangedInteriorPixels);
    await graph.press(generation3dWorkflow.history.redo);
    await expect.poll(currentValue).toBe(changed);
    expect((await paintWitness(page, variant)).painted).toBe(true);
    const redoPixels=await canvas.screenshot({path:testInfo.outputPath("redo.png")});
    expect(changedInteriorPixels(undoPixels,redoPixels),"redo must visibly repeat the edit").toBeGreaterThanOrEqual(generation3dWorkflow.gpu.minimumChangedInteriorPixels);
    const connection = host.synapses.find(wire => wire.from === slider!.id && host.widgets.some(widget => widget.id === wire.to && widget.kind === "neuron"));
    expect(connection, "selected geometry parameter must expose its actual connected operator").toBeDefined();
    await graph.focus();
    for (let step = 0; step <= host.widgets.length; step++) {
      if ((await graph.getAttribute("data-diagram-focused-node")) === connection!.to) break;
      await graph.press(generation3dWorkflow.focusKey);
    }
    await expect(graph).toHaveAttribute("data-diagram-focused-node", connection!.to);
    await graph.press(generation3dWorkflow.selectKey);
    const source = page.locator(`[data-layered-pane="${variant}"] [data-ui-node-key="${generation3dWorkflow.sourceNavigation.inputPrefix}${connection!.toPort}${generation3dWorkflow.sourceNavigation.controlSuffix}"]`);
    await expect(source).toBeVisible();
    const beforeInspection = JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!);
    await source.click();
    await expect.poll(async()=>JSON.parse((await graph.getAttribute("data-selection-json"))!)[generation3dWorkflow.sourceNavigation.selectionField]).toEqual([slider!.id]);
    await expect(input).toBeVisible();
    await expect(input).toHaveValue(String(changed));
    const afterInspection = JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!);
    for (const field of generation3dWorkflow.sourceNavigation.documentUnchanged) expect(afterInspection[field]).toEqual(beforeInspection[field]);
    errors.stop();
    expect(errors.pageErrors).toEqual([]);
    expect(errors.consoleErrors).toEqual([]);
    expect(errors.refusedInputs).toEqual([]);
  });

  test("analyze",async({page})=>{
    test.setTimeout(TEST_TIMEOUT_MS);const errors=collectErrors(page),variant=generation3dWorkflow.pane,pane=playPanes().find(pane=>pane.variant===variant)!;
    await page.goto(`./#${variant}`);expect(await waitForPaneShellOutcome(page,variant)).toBe("ready");await dismissPaneIntroduction(page,pane);expect((await paintWitness(page,variant)).painted).toBe(true);
    const graph=page.locator(`[data-layered-pane="${variant}"] ${generation3dWorkflow.graph}`).first();
    const snapshot=async()=>JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!) as {widgets:{id:string;kind:string;neuronKind?:string}[];synapses:{from:string;to:string;toPort:string}[]};
    const before=await snapshot();let sourceId:string|undefined;let button=page.locator(`[data-layered-pane="${variant}"] [data-ui-node-key$="${generation3dWorkflow.analysis.buttonSuffix}"]`).first();
    await graph.focus();for(let step=0;step<=before.widgets.length;step++){await graph.press(generation3dWorkflow.focusKey);await graph.press(generation3dWorkflow.selectKey);if(await button.isVisible()){sourceId=(await graph.getAttribute("data-diagram-focused-node"))!;break;}}
    expect(sourceId,"curated actual inference must offer a compatible successful shape output").toBeDefined();await expect(button).toBeEnabled();await button.click();
    await expect.poll(async()=>{const current=await snapshot();return current.widgets.length;}).toBe(before.widgets.length+1);
    const created=await snapshot(),added=created.widgets.filter(widget=>!before.widgets.some(original=>original.id===widget.id));expect(added).toHaveLength(1);expect(added[0]!.neuronKind).toBe(generation3dWorkflow.analysis.operator);
    const wires=created.synapses.filter(wire=>!before.synapses.some(original=>JSON.stringify(original)===JSON.stringify(wire)));expect(wires).toHaveLength(1);expect(wires[0]).toMatchObject({from:sourceId,to:added[0]!.id,toPort:generation3dWorkflow.analysis.targetPort});
    for(const original of before.widgets)expect(created.widgets.find(widget=>widget.id===original.id)).toEqual(original);for(const original of before.synapses)expect(created.synapses).toContainEqual(original);
    await graph.focus();for(let step=0;step<=created.widgets.length;step++){if((await graph.getAttribute("data-diagram-focused-node"))===added[0]!.id)break;await graph.press(generation3dWorkflow.focusKey);}await expect(graph).toHaveAttribute("data-diagram-focused-node",added[0]!.id);await graph.press(generation3dWorkflow.selectKey);
    const volume=page.locator(`[data-layered-pane="${variant}"] [data-ui-node-key="${generation3dWorkflow.analysis.outputKey}"]`);await expect(volume).toBeVisible();const actualVolume=async()=>{const text=await volume.textContent();if(text===null||/(stale|pending|error|veraltet|Fehler)/i.test(text))return null;const match=text.match(/:\s*(-?\d+(?:\.\d+)?(?:e[+-]?\d+)?)/i);return match?Number(match[1]):null;};await expect.poll(actualVolume).not.toBeNull();const measured=await actualVolume();expect(Number.isFinite(measured)).toBe(true);expect(measured!).toBeGreaterThanOrEqual(generation3dWorkflow.analysis.minimumVolume);
    await graph.focus();await graph.press(generation3dWorkflow.history.undo);await expect.poll(snapshot).toEqual(before);await graph.press(generation3dWorkflow.history.redo);await expect.poll(snapshot).toEqual(created);
    errors.stop();expect(errors.pageErrors).toEqual([]);expect(errors.consoleErrors).toEqual([]);expect(errors.refusedInputs).toEqual([]);expect(errors.resourceErrors).toEqual([]);
  });


  test("cancel",async({page})=>{
    test.setTimeout(TEST_TIMEOUT_MS);
    const errors=collectErrors(page),variant=generation3dWorkflow.pane,pane=playPanes().find(pane=>pane.variant===variant)!,law=generation3dWorkflow.cancel;
    await page.goto(`./#${variant}`);expect(await waitForPaneShellOutcome(page,variant)).toBe("ready");await dismissPaneIntroduction(page,pane);expect((await paintWitness(page,variant)).painted).toBe(true);
    const scope=page.locator(`[data-layered-pane="${variant}"]`),graph=scope.locator(generation3dWorkflow.graph).first();
    const snapshot=async()=>JSON.parse((await graph.getAttribute("data-host-snapshot-json"))!) as {widgets:{neuronKind?:string}[];synapses:unknown[]};
    await scope.locator(`[id="${law.examplePicker}"]`).click();await page.getByRole("option",{name:language.cancelExampleLabel,exact:true}).click();
    await expect.poll(async()=>(await snapshot()).widgets.some(widget=>widget.neuronKind===law.sourceOperator)).toBe(true);
    const status=scope.locator('[data-slot="world-compute-status"][aria-busy="true"]').first();
    const button=status.getByRole("button",{name:language.cancelLabel,exact:true});
    let identity="",started=0,firstDone=0,latestDone=0,observations=0;
    await expect.poll(async()=>{
      if(!await button.isVisible())return false;
      const [run,generation,done,total]=await Promise.all([button.getAttribute("data-cancel-run-id"),button.getAttribute("data-cancel-generation"),status.getAttribute("data-compute-units-done"),status.getAttribute("data-compute-units-total")]);
      if(run===null||generation===null||done===null||total===null||! /^[1-9]\d*$/.test(run)||BigInt(run)>0xffffffffffffffffn||! /^\d+$/.test(generation)||Number(generation)>0xffffffff)return false;
      const next=`${run}:${generation}`,completed=Number(done),extent=Number(total);
      if(!Number.isSafeInteger(completed)||!Number.isSafeInteger(extent)||completed<0||extent<=completed)return false;
      if(next!==identity){identity=next;started=Date.now();firstDone=completed;observations=0;}latestDone=completed;observations++;
      return observations>=law.minimumObservations&&completed>firstDone&&Date.now()-started>=law.minimumElapsedMilliseconds;
    },{timeout:SHELL_READY_TIMEOUT_MS,intervals:[50,100,150]}).toBe(true);
    const [run,generation]=identity.split(":");await expect(button).toHaveAttribute("data-cancel-action",law.action);await expect(button).toHaveAttribute("data-cancel-run-id",run!);await expect(button).toHaveAttribute("data-cancel-generation",generation!);
    const before=await snapshot();await button.click();
    const terminal=scope.locator(`[data-slot="world-compute-status"][data-compute-phase="${law.terminalPhase}"]`).first();await expect(terminal).toBeVisible();await expect(terminal).not.toHaveAttribute("aria-busy","true");await expect(terminal.locator('[data-slot="world-compute-cancel"]')).toHaveCount(0);
    const after=await snapshot();for(const field of law.documentUnchanged)expect(after[field as keyof typeof after]).toEqual(before[field as keyof typeof before]);
    errors.stop();expect(errors.pageErrors).toEqual([]);expect(errors.consoleErrors).toEqual([]);expect(errors.refusedInputs).toEqual([]);expect(errors.resourceErrors).toEqual([]);
  });

  });

  for (const language of formsWorkflow.locales) test.describe(`forms ${language.browser}`, () => {
    test.use({ locale: language.browser });

    test("authoring", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`);
      const builder = scope.locator('[data-surface-id="forms.play.blueprint"]');
      const original = builder.locator('[data-block-id="name"]');
      await expect(original).toBeVisible();
      await original.locator('button[aria-pressed]').click();
      const label = scope.locator(`[data-ui-node-key="${formsWorkflow.controls.questionLabel}"]`);
      await expect(label).toBeVisible();
      await label.fill(formsWorkflow.questionLabel);
      await label.press("Enter");
      await expect(original).toContainText(formsWorkflow.questionLabel);
      const title = scope.locator(`[data-ui-node-key="${formsWorkflow.controls.title}"]`);
      await expect(title).toHaveAttribute("aria-label", language.title);
      await title.fill(formsWorkflow.title);
      await title.press("Enter");
      const cards = builder.locator('[data-block-id]'), before = await cards.count();
      const originalIds = await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")));
      await builder.locator(`[data-block-kind="${formsWorkflow.addedKind}"]`).click();
      await expect(cards).toHaveCount(before + 1);
      const addedId = (await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")))).find(id => !originalIds.includes(id));
      expect(addedId).toBeTruthy();
      const added = builder.locator(`[data-block-id="${addedId}"]`);
      await added.getByRole("button", { name: /^(Delete|Löschen):/ }).click();
      await expect(cards).toHaveCount(before);
      for (const step of roleContract.steps) {
        await scope.locator(`[id="${step.controlId}"]`).click();
        await waitForSurfaceSettlement(page, pane);
        await dismissPaneIntroduction(page, pane);
        if (step.role === "viewer") {
          await expect(scope.getByRole("slider", { name: formsWorkflow.extension.label, exact: true })).toBeDisabled();
          const run = scope.getByRole("button", { name: language.run, exact: true });
          await expect(run).toBeEnabled();
          await run.click();
          await expect.poll(async () => {
            const meshes = await scope.locator("[data-meshes-json]").first().getAttribute("data-meshes-json");
            return meshes === null ? 0 : (JSON.parse(meshes) as unknown[]).length;
          }).toBeGreaterThan(0);
        }
      }
      await expect(title).toHaveValue(formsWorkflow.title);
      await expect(builder.locator('[data-block-id="name"]')).toContainText(formsWorkflow.questionLabel);
      await expect(builder.locator(`[data-block-id="${addedId}"]`)).toHaveCount(0);
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("source refusal", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const fixture = JSON.parse(readFileSync(join(import.meta.dirname, "../../../..", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📄️value-source/🔣️.json"), "utf8")) as { encodingBoundaries: { unit: string; repeat: number }; refusalLocales: { locale: string; message: string; discard: string }[] };
      const labels = fixture.refusalLocales.find(row => row.locale === language.browser.slice(0, 2))!;
      const accepted = fixture.encodingBoundaries.unit.repeat(fixture.encodingBoundaries.repeat), refused = accepted + "x";
      const flow = JSON.parse(readFileSync(join(import.meta.dirname, "../../../..", formsWorkflow.flowFixture), "utf8")), source = JSON.stringify(flow.snapshot);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`), id = await authorFormsFlow(page, scope, formsWorkflow.flowKind, source, language.flowDocument);
      const editor = scope.getByRole("textbox", { name: language.flowDocument, exact: true }), host = editor.locator('xpath=ancestor::*[contains(@class,"semio-text-editor-host")][1]');
      await replaceEditorText(page, editor, refused);
      await expect(host.getByRole("alert")).toHaveText(labels.message);
      await expect(editor).toHaveAttribute("aria-invalid", "true");
      await expect(editor).toHaveValue(refused);
      await replaceEditorText(page, editor, accepted);
      await expect(host.getByRole("alert")).toHaveCount(0);
      await expect(editor).toHaveValue(accepted);
      const builder = scope.locator('[data-surface-id="forms.play.blueprint"]');
      await builder.locator('[data-block-id="name"] button[aria-pressed]').click();
      await builder.locator(`[data-block-id="${id}"] button[aria-pressed]`).click();
      await expect(editor.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
      await expect(editor).toHaveValue(accepted);
      await replaceEditorText(page, editor, source);
      await expect(scope.locator(`[data-ui-node-key="forms-try.${id}"]`).getByRole("slider", { name: "Amount", exact: true })).toHaveAttribute("aria-valuenow", "2");
      await replaceEditorText(page, editor, refused);
      await expect(host.getByRole("alert")).toHaveText(labels.message);
      await host.getByRole("button", { name: labels.discard, exact: true }).click();
      await expect(editor).toHaveValue(source);
      await expect(host.getByRole("alert")).toHaveCount(0);
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("long answer", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const fixtureRoot = join(import.meta.dirname, "../../../..", "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🧫️fixtures");
      const fixture = JSON.parse(readFileSync(join(fixtureRoot, "📝️answer.json"), "utf8")) as { answer: { unit: string; repeat: number; bytes: number; sha256: string }; kinds: string[] };
      const schema = JSON.parse(readFileSync(join(fixtureRoot, "📝️answer.schema.json"), "utf8"));
      expect(new Ajv2020({ strict: true }).validate(schema, fixture)).toBe(true);
      const answer = fixture.answer.unit.repeat(fixture.answer.repeat);
      expect(Buffer.byteLength(answer)).toBe(fixture.answer.bytes);
      expect(answer).toBe("ä🧱".repeat(800));
      expect(Buffer.byteLength(answer)).toBeGreaterThan(4096);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`), builder = scope.locator("[data-surface-id=\"forms.play.blueprint\"]");
      const ids: string[] = [];
      for (const kind of fixture.kinds) {
        await builder.locator("[data-block-id=\"name\"] button[aria-pressed]").click();
        const cards = builder.locator("[data-block-id]"), before = await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")));
        await builder.locator(`[data-block-kind="${kind}"]`).click();
        await expect(cards).toHaveCount(before.length + 1);
        const id = (await cards.evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")))).find(id => !before.includes(id))!;
        ids.push(id);
        const input = scope.locator(`[data-ui-node-key="forms-try.${id}.input"]`).getByRole("textbox");
        await replaceEditorText(page, input, answer);
        await builder.locator("[data-block-id=\"name\"] button[aria-pressed]").click();
        await expect(input).toHaveValue(answer);
      }
      await scope.locator(`[data-ui-node-key="${formsWorkflow.controls.next}"]`).click();
      await scope.locator(`[data-ui-node-key="${formsWorkflow.controls.submit}"]`).click();
      await expect(scope.locator(`[data-ui-node-key="${formsWorkflow.controls.submitted}"]`)).toHaveText(language.submitted);
      await scope.locator("[id=\"playground.navbar.modes.responses\"]").click();
      for (const id of ids) {
        const saved = scope.locator(`[data-ui-node-key="${id}.answer"]`).getByRole("textbox");
        await expect(saved.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
        await expect(saved).toHaveValue(answer);
        await expect(saved).toHaveJSProperty("readOnly", true);
        await expect(saved).not.toBeEditable();
      }
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("flow isolation", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      const flow = JSON.parse(readFileSync(join(import.meta.dirname, "../../../..", formsWorkflow.flowFixture), "utf8")) as { snapshot: { widgets: { id: string; value?: number; text?: string }[] }; parameters: { amount: number; message: string } };
      const source = JSON.stringify(flow.snapshot, null, 2);
      expect(Buffer.byteLength(source)).toBeGreaterThan(600);
      const initial = flow.snapshot.widgets.find(widget => widget.id === "amount")!.value!;
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`), builder = scope.locator('[data-surface-id="forms.play.blueprint"]');
      const ids: string[] = [];
      for (const value of [flow.parameters.amount, initial]) {
        await builder.locator('[data-block-id="name"] button[aria-pressed]').click();
        const before = await builder.locator('[data-block-id]').evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")));
        await builder.locator(`[data-block-kind="${formsWorkflow.flowKind}"]`).click();
        await expect(builder.locator('[data-block-id]')).toHaveCount(before.length + 1);
        const id = (await builder.locator('[data-block-id]').evaluateAll(elements => elements.map(element => element.getAttribute("data-block-id")))).find(id => !before.includes(id))!;
        expect(id).toBeTruthy();
        ids.push(id);
        await builder.locator(`[data-block-id="${id}"] button[aria-pressed]`).click();
        const document = scope.getByRole("textbox", { name: language.flowDocument, exact: true });
        await expect(document.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
        await expect(document).toBeEditable();
        await document.focus();
        await document.press("ControlOrMeta+A");
        await page.keyboard.insertText(source);
        await expect(document).toHaveValue(source);
        const field = scope.locator(`[data-ui-node-key="forms-try.${id}"]`), amount = field.getByRole("slider", { name: "Amount", exact: true });
        await expect(amount).toHaveAttribute("aria-valuenow", String(initial));
        await amount.focus();
        for (let current = initial; current < value; current++) {
          await amount.press("ArrowRight");
          await expect(amount).toHaveAttribute("aria-valuenow", String(current + 1));
        }
        await expect(amount).toHaveAttribute("aria-valuenow", String(value));
        const message = field.getByRole("textbox", { name: "Message", exact: true });
        await expect(message.locator("..")).toHaveAttribute("data-text-editor-ready", "true");
        await expect(message).toHaveValue(flow.snapshot.widgets.find(widget => widget.id === "message")!.text!);
        if (value === flow.parameters.amount) {
          await message.focus();
          await message.press("ControlOrMeta+A");
          await page.keyboard.insertText(flow.parameters.message);
          await expect(message).toHaveValue(flow.parameters.message);
        }
        await field.getByRole("button", { name: language.run, exact: true }).click();
        const output = field.locator('[data-ui-node-key="embedded-flow.output.answer"]');
        await expect(output).toContainText("answer");
        await expect(output.getByText(String(value), { exact: true })).toBeVisible();
      }
      await expect(scope.locator(`[data-ui-node-key="forms-try.${ids[0]}"]`).getByRole("textbox", { name: "Message", exact: true })).toHaveValue(flow.parameters.message);
      expect(new Set(ids).size).toBe(2);
      await expect(scope.locator(`[data-ui-node-key="forms-try.${ids[0]}"] [data-ui-node-key="embedded-flow.output.answer"]`).getByText(String(flow.parameters.amount), { exact: true })).toBeVisible();
      await expect(scope.locator(`[data-ui-node-key="forms-try.${ids[1]}"] [data-ui-node-key="embedded-flow.output.answer"]`).getByText(String(initial), { exact: true })).toBeVisible();
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("windows", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      const flow = JSON.parse(readFileSync(join(import.meta.dirname, "../../../..", formsWorkflow.flowFixture), "utf8"));
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`);
      const id = await authorFormsFlow(page, scope, formsWorkflow.flowKind, JSON.stringify(flow.snapshot), language.flowDocument);
      const fieldKey = `forms-try.${id}`, initialField = scope.locator(`[data-ui-node-key="${fieldKey}"]`);
      const firstId = await initialField.locator("xpath=ancestor::*[@data-slot=\"window\"][1]").getAttribute("id");
      expect(firstId).toBeTruthy();
      const first = scope.locator(`[data-slot="window"][id="${firstId}"]`), firstField = first.locator(`[data-ui-node-key="${fieldKey}"]`);
      const firstSlider = firstField.getByRole("slider", { name: "Amount", exact: true });
      await firstSlider.focus();
      for (let value = 3; value <= 7; value++) { await firstSlider.press("ArrowRight"); await expect(firstSlider).toHaveAttribute("aria-valuenow", String(value)); }
      await firstField.getByRole("button", { name: language.run, exact: true }).click();
      await expect(firstField.locator("[data-ui-node-key=\"embedded-flow.output.answer\"]").getByText("7", { exact: true })).toBeVisible();
      const ids = cancellationWorkflow.windowUi;
      await scope.locator(`[id="${ids.tab}"]`).click();
      await scope.locator(`[id="${ids.group}"]`).click();
      const template = scope.locator(`[id="${ids.template}"]`);
      await expect(template).toHaveAttribute("draggable", "true");
      const box = await first.boundingBox();
      expect(box).not.toBeNull();
      await template.dragTo(first, { targetPosition: { x: box!.width - 3, y: box!.height / 2 } });
      await expect(scope.locator(`[data-slot="window"]:has([data-ui-node-key="${fieldKey}"])`)).toHaveCount(2);
      const second = scope.locator(`[data-slot="window"]:not([id="${firstId}"]):has([data-ui-node-key="${fieldKey}"])`), secondField = second.locator(`[data-ui-node-key="${fieldKey}"]`);
      const secondSlider = secondField.getByRole("slider", { name: "Amount", exact: true });
      await expect(secondSlider).toHaveAttribute("aria-valuenow", "2");
      await secondSlider.focus();
      for (let value = 3; value <= 4; value++) { await secondSlider.press("ArrowRight"); await expect(secondSlider).toHaveAttribute("aria-valuenow", String(value)); }
      await secondField.getByRole("button", { name: language.run, exact: true }).click();
      await expect(secondField.locator("[data-ui-node-key=\"embedded-flow.output.answer\"]").getByText("4", { exact: true })).toBeVisible();
      await expect(firstSlider).toHaveAttribute("aria-valuenow", "7");
      await expect(firstField.locator("[data-ui-node-key=\"embedded-flow.output.answer\"]").getByText("7", { exact: true })).toBeVisible();
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("cancel", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      const source = JSON.stringify(cancellationWorkflow.snapshot), labels = cancellationWorkflow.locales[language.browser as keyof typeof cancellationWorkflow.locales];
      expect(Buffer.byteLength(source)).toBeLessThanOrEqual(16_384);
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`);
      const id = await authorFormsFlow(page, scope, cancellationWorkflow.questionKind, source, language.flowDocument);
      const field = scope.locator(`[data-ui-node-key="forms-try.${id}"]`), run = field.getByRole("button", { name: language.run, exact: true }), cancel = field.getByRole("button", { name: labels.cancel, exact: true });
      await expect(cancel).toBeDisabled();
      await run.click();
      await expect(run).toBeDisabled();
      await expect(cancel).toBeEnabled();
      await cancel.click();
      await expect(field.locator("[data-ui-node-key=\"embedded-flow.status\"]")).toContainText(labels.stopped);
      await expect(run).toBeEnabled();
      await expect(cancel).toBeDisabled();
      await run.click();
      const heights = cancellationWorkflow.snapshot.widgets.filter(widget => widget.id.endsWith("-height")).map(widget => Number("value" in widget ? widget.value : NaN)).sort((a, b) => a - b);
      expect(heights).toHaveLength(4);
      await expect.poll(() => worldMeshHeights(field), { timeout: 60_000 }).toEqual(heights);
      await expect(run).toBeEnabled();
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });

    test("submission", async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page), pane = playPanes().find(pane => pane.variant === formsWorkflow.pane)!;
      await page.goto(`./#${pane.variant}`);
      expect(await waitForPaneShellOutcome(page, pane.variant)).toBe("ready");
      await dismissPaneIntroduction(page, pane);
      const scope = page.locator(`[data-layered-pane="${pane.variant}"]`);
      const control = (key: string) => scope.locator(`[data-ui-node-key="${key}"]`);
      const name = control(formsWorkflow.controls.name).getByRole("textbox"), next = control(formsWorkflow.controls.next);
      await expect(name).toBeVisible();
      await replaceEditorText(page, name, "");
      await expect(next).toBeDisabled();
      await replaceEditorText(page, name, formsWorkflow.answer);
      await expect(next).toHaveAccessibleName(language.next);
      await expect(next).toBeEnabled();
      await next.click();
      const height = scope.getByRole("slider", { name: formsWorkflow.extension.label, exact: true });
      await expect(height).toHaveAttribute("aria-valuenow", String(formsWorkflow.extension.initial));
      await scope.getByRole("button", { name: language.run, exact: true }).click();
      await expect.poll(() => worldMeshHeights(scope), { timeout: 60_000 }).toEqual([formsWorkflow.extension.initial]);
      const initialGeometry = await scope.locator("[data-meshes-json]").first().getAttribute("data-meshes-json");
      await height.focus();
      await height.press(formsWorkflow.extension.key);
      await expect(height).toHaveAttribute("aria-valuenow", String(formsWorkflow.extension.changed));
      await scope.getByRole("button", { name: language.run, exact: true }).click();
      await expect.poll(() => worldMeshHeights(scope), { timeout: 60_000 }).toEqual([formsWorkflow.extension.changed]);
      await expect(scope.locator("[data-meshes-json]").first()).not.toHaveAttribute("data-meshes-json", initialGeometry!);
      const submit = control(formsWorkflow.controls.submit);
      await expect(submit).toHaveAccessibleName(language.submit);
      await expect(submit).toBeEnabled();
      await submit.click();
      await expect(control(formsWorkflow.controls.submitted)).toHaveText(language.submitted);
      await control(formsWorkflow.controls.respondAgain).click();
      await expect(name).toHaveValue("Column A");
      await scope.locator('[id="playground.navbar.modes.responses"]').click();
      const responses = control("forms-responses.items");
      await expect(responses).toContainText(`${language.responses} (1)`);
      await expect(responses).toContainText(formsWorkflow.answer);
      await expect(responses).toContainText(String(formsWorkflow.extension.changed));
      await scope.locator('[id="playground.navbar.modes.blueprint"]').click();
      await expect(name).toBeVisible();
      await scope.locator('[id="playground.navbar.modes.responses"]').click();
      await expect(responses).toContainText(`${language.responses} (1)`);
      await expect(responses).toContainText(formsWorkflow.answer);
      await expect(responses).toContainText(String(formsWorkflow.extension.changed));
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
    });
  });

  test.describe("language", () => {
    test.use({locale:"fr-FR"});
    test("choose", async ({page}) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      await page.goto("./#generation3d");
      await expect(page.locator('[data-slot="play-language-choice"]')).toBeVisible();
      await expect(page.getByRole("button", {name:"English", exact:true})).toBeVisible();
      await page.getByRole("button", {name:"Deutsch", exact:true}).click();
      expect(await waitForPaneShellOutcome(page, "generation3d")).toBe("ready");
      await expect(page.locator('[data-slot="play-language-choice"]')).toHaveCount(0);
      await expect(page.locator("html")).toHaveAttribute("lang", "de");
    });
  });

  for (const pane of playPanes()) {
    test(`boots ${pane.label} (${pane.variant})`, async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page);
      const fallbackAssets = collectSpaFallbackAssets(page);
      await page.goto(`./#${pane.variant}`);
      const outcome = await waitForPaneShellOutcome(page, pane.variant);
      const detail = await page.evaluate(id => (document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null)?.dataset.shellError ?? "", pane.variant);
      expect(outcome, `${pane.variant} shell outcome ${detail}`).toBe("ready");
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(BOOT_SETTLE_MS);
      await expect(page.locator(`[data-layered-pane="${pane.variant}"] [data-layered-pane-error]`)).toHaveCount(0);
      await expect(page.locator("[data-layered-overview-button]")).toBeVisible();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);

      const fixture = page.locator(`[data-layered-pane="${pane.variant}"] [id="playground.navbar.fixture"]`);
      if (pane.exampleLabel !== undefined && (await fixture.count()) > 0) {
        await expect(fixture, `${pane.variant} chrome must name its curated example ${pane.example}`).toHaveText(pane.exampleLabel);
      }

      const witness = await paintWitness(page, pane.variant);
      expect(witness.painted, `${pane.variant} paints nothing — ${witness.detail}`).toBe(true);
      await dismissPaneIntroduction(page, pane);
      await exerciseCuratedExample(page, pane);
      await exerciseSurfaceRoundTrip(page, pane);
      await page.locator("[data-layered-overview-button]").click();
      await expect(page.locator("[data-layered-overlay]")).toBeVisible();
      errors.stop();
      expect(errors.pageErrors).toEqual([]);
      expect(errors.consoleErrors).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      expect(errors.resourceErrors).toEqual([]);
      expect(fallbackAssets, `${pane.variant} assets answered by the SPA fallback`).toEqual([]);
    });
  }
});
