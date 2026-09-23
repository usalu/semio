// #region 🧲️Header
// 💻️ 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts
// Specs: End-to-end acceptance coverage for semio-tech play — every app of the grid boots AND shows content.
// Summary: The overview must list one card per authored pane. Then, for every pane, deep-links to
// `/#<paneId>` (which boots exactly that pane immediately), waits for its own `FrameworkOsShell` to report
// an outcome through the per-shell `data-shell-ready`/`data-shell-error`/`data-shell-not-found` beacon,
// requires "ready", lets the boot example announcement settle, requires the pane's error boundary to stay
// silent and fails on any page error, non-404 console error or refused input. It then requires VISIBLE
// content: the curated example's own rendered label in the pane's chrome, and a main window that actually
// paints — a non-uniform pixel census of the largest canvas, or a non-empty main region for a DOM window.
// Across the whole run no asset request may be answered by the SPA fallback. The overview button must
// return to the overview.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, test, type Page } from "@playwright/test";
// #endregion 🔌️Adapters

//#region 🪪️PlayPanes
type PlayPane = { readonly variant: string; readonly label: string; readonly example?: string; readonly exampleLabel?: string };

/** @emoji 🪪️ Reads the pane catalog as data instead of importing app modules, which drag the React runtime into Node's loader. */
function playPanes(): readonly PlayPane[] {
  const catalog = JSON.parse(readFileSync(join(import.meta.dirname, "..", "..", "🔨️modules", "🧩️runtime", "🔣️.json"), "utf8")) as { readonly groups: readonly { readonly panes: readonly PlayPane[] }[] };
  return catalog.groups.flatMap(group => group.panes);
}
//#endregion 🪪️PlayPanes

/** @emoji ⏱️ Cold wasm plugin boots can be slow — generous so the suite reports real defects, not infrastructure latency. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 180_000;

/** @emoji 🕊️ Quiet window after the network settles: the shell announces the boot example after "ready",
 * and a refused announcement is only logged once the guest answers. */
const BOOT_SETTLE_MS = 2_000;

/** @emoji 📄️ A DOM main region with neither text nor this many elements rendered nothing a reader sees. */
const MINIMUM_MAIN_REGION_ELEMENTS = 10;

/** @emoji 🖼️ The smallest edge an image asset may have and still be the CONTENT of the pixel layer it
 * fills. `raster`'s composite declares a 1024×1024 `pixel` layer whose `imageKey` resolves to the 2×2
 * `semio-emblem` placeholder, decodes nothing (`📓️raster.md`: `atob` is never called) and paints an empty
 * canvas — while "the document has a layer and the scene has an asset" reads perfectly healthy. Eight
 * pixels a side is far below any authored demo and far above a placeholder. */
const MINIMUM_IMAGE_ASSET_EDGE = 8;

/** @emoji ⏳️ A retained lane can land after the boot settle — `energy`'s 4 542-character `data-meshes-json`
 * was still empty 2.5 s after `data-shell-ready` and full at 4 s — so the witness is polled rather than
 * read once. A painted pane answers on the first read (≈20 ms); only a genuinely empty one pays the wait. */
const PAINT_WITNESS_DEADLINE_MS = 4_000;
const PAINT_WITNESS_POLL_MS = 250;

/** @emoji 🔇️ Drops resource 404s and the repo's `[DEBUG] `-prefixed temporary diagnostics. */
function significantConsoleErrors(messages: readonly string[]): string[] {
  return messages.filter(text => !/Failed to load resource:.*\b40[0-9]\b/i.test(text) && !text.startsWith("[DEBUG] "));
}

/** @emoji 👂️ Collects page errors, console errors and refused inputs — a refused input is the shell's
 * "The input could not be delivered" notice, logged only as a warning. */
function collectErrors(page: Page): { readonly pageErrors: string[]; readonly consoleErrors: string[]; readonly refusedInputs: string[] } {
  const pageErrors: string[] = [], consoleErrors: string[] = [], refusedInputs: string[] = [];
  page.on("pageerror", error => pageErrors.push(`${error.name}: ${error.message}`));
  page.on("console", message => {
    if (message.type() === "error") consoleErrors.push(message.text());
    else if (/^input #\d+ .* refused: /.test(message.text())) refusedInputs.push(message.text());
  });
  return { pageErrors, consoleErrors, refusedInputs };
}

//#region 🖼️VisibleContent
/** @emoji 📦️ Request paths that must be answered by a FILE. A dev server's SPA fallback happily answers
 * `GET /🖼️assets/🖼️bauteilbörse.png` with `index.html` and HTTP 200, so a missing figure looks like a
 * loaded one to everything except the eye — exactly how `animate`'s missing deck figure survived audit #1
 * and a whole strict acceptance run (ticket 26/09/19 `📓️status.md`, 2026-09-22 04:10). */
const ASSET_PATH = /\.(png|jpe?g|gif|webp|avif|svg|ico|bmp|glb|gltf|obj|stl|ply|3dm|mp3|mp4|webm|wav|ogg|woff2?|ttf|otf|pdf)$/i;

/** @emoji 🏚️ Collects every asset request the server answered with a document instead of the asset. */
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

/** @emoji 🪟️ One mounted window's verdict on whether it PAINTED anything, read from what its host
 * publishes rather than from pixels. */
type WindowWitness = { readonly surface: string; readonly kind: "world3d" | "paint2d" | "dom"; readonly painted: boolean; readonly detail: string };
type PaintWitness = { readonly painted: boolean; readonly detail: string };

/** @emoji 🔬️ Asks every window of the pane, in its OWN terms, whether it painted — evaluated inside the
 * page so one round trip answers for the whole pane (≈20 ms).
 *
 * Pixels cannot answer this. An element screenshot is the PAGE screenshot clipped to the element box, so
 * the window chrome that overlaps a full-bleed viewport (header, gutters, overlays) lands inside the crop
 * and a blank `raster` composite censuses 118–316 distinct colours; and a `drawImage` readback of the live
 * canvas returns transparent for every WebGPU surface in this host (`raster`'s composite reports
 * `getContext("webgpu")`), painted or not. Both routes were measured on :6033 and both are false greens.
 *
 * So each window is asked what it retains:
 * — a 3D world window publishes its retained geometry on its own surface element
 *   (`data-meshes-json` / `data-instances-json`, written by `🌐️World3dHost`) — it must hold at least one
 *   mesh or instance;
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
    const pane = document.querySelector(`[data-play-pane="${id}"]`);
    if (!pane) return [];
    const area = (element: Element): number => { const box = element.getBoundingClientRect(); return box.width * box.height; };
    const size = (text: string | null): number => {
      if (typeof text !== "string" || text.length === 0) return 0;
      try { const value = JSON.parse(text); return Array.isArray(value) ? value.length : value && typeof value === "object" ? Object.keys(value).length : 0; } catch { return 0; }
    };
    // 🪟️ A pane whose windows are plain documents (the `norm` codes, `stdio-json`, `block2d`, `home`)
    // mounts no `[data-surface-id]` host at all — its content is the window body itself.
    const surfaces = [...pane.querySelectorAll("[data-surface-id]")];
    if (surfaces.length === 0) {
      return [...pane.querySelectorAll('[data-slot="window-body"]')].sort((a, b) => area(b) - area(a)).map((body, index) => {
        const text = (body instanceof HTMLElement ? body.innerText : body.textContent ?? "").trim().length;
        const elements = body.querySelectorAll("*").length;
        return { surface: `(window body ${index})`, kind: "dom" as const, painted: text > 0 || elements >= minimumElements, detail: `${text} characters, ${elements} elements` };
      });
    }
    return surfaces.sort((a, b) => area(b) - area(a)).map((element): { surface: string; kind: "world3d" | "paint2d" | "dom"; painted: boolean; detail: string } => {
      const surface = element.getAttribute("data-surface-id") ?? "(unnamed)";
      const meshes = element.getAttribute("data-meshes-json"), instances = element.getAttribute("data-instances-json");
      if (meshes !== null || instances !== null) {
        return { surface, kind: "world3d", painted: size(meshes) + size(instances) > 0, detail: `${size(meshes)} meshes, ${size(instances)} instances` };
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
      const text = (region instanceof HTMLElement ? region.innerText : region.textContent ?? "").trim().length;
      const elements = region.querySelectorAll("*").length;
      return { surface, kind: "dom", painted: text > 0 || elements >= minimumElements, detail: `window: ${text} characters, ${elements} elements` };
    });
  }, { id: variant, minimumElements: MINIMUM_MAIN_REGION_ELEMENTS, minimumEdge: MINIMUM_IMAGE_ASSET_EDGE });
}

/** @emoji 🖼️ Polls {@link readPaintWitnesses} until some window paints or the deadline passes, and renders
 * the verdict of every window so a failure names the exact empty one. */
async function paintWitness(page: Page, variant: string): Promise<PaintWitness> {
  const deadline = Date.now() + PAINT_WITNESS_DEADLINE_MS;
  let witnesses = await readPaintWitnesses(page, variant);
  while (!witnesses.some((witness) => witness.painted) && Date.now() < deadline) {
    await page.waitForTimeout(PAINT_WITNESS_POLL_MS);
    witnesses = await readPaintWitnesses(page, variant);
  }
  const detail = witnesses.length === 0 ? "the pane mounted no window at all" : witnesses.map((witness) => `${witness.surface} [${witness.kind}] ${witness.painted ? "paints" : "EMPTY"} — ${witness.detail}`).join("; ");
  return { painted: witnesses.some((witness) => witness.painted), detail };
}
//#endregion 🖼️VisibleContent

type ShellOutcome = "ready" | "error" | "notFound";

/** @emoji 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome. */
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

test.describe("semio-tech play", () => {
  test("lists one overview card per app", async ({ page }) => {
    const errors = collectErrors(page);
    const fallbackAssets = collectSpaFallbackAssets(page);
    const panes = playPanes();
    await page.goto("./");
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-play-pane-card]")).toHaveCount(panes.length);
    await expect(page.locator('[data-slot="play-app-count"]')).toHaveText(`${panes.length} apps`);
    for (const pane of panes) await expect(page.locator(`[data-play-pane-card][data-pane-id="${pane.variant}"]`)).toHaveAttribute("aria-label", new RegExp(`^Open ${pane.label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}: `));
    expect(errors.pageErrors).toEqual([]);
    expect(significantConsoleErrors(errors.consoleErrors)).toEqual([]);
    expect(fallbackAssets, "assets answered by the SPA fallback").toEqual([]);
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
      await expect(page.locator(`[data-play-pane="${pane.variant}"] [data-play-pane-error]`)).toHaveCount(0);
      await expect(page.locator("[data-play-overview-button]")).toBeVisible();
      expect(errors.pageErrors).toEqual([]);
      expect(significantConsoleErrors(errors.consoleErrors)).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);

      // 🏷️ The chrome must SAY which document it opened, in the words that document publishes. The trigger
      // renders the active example's `label.native.en` and nothing else (`🧪️NavbarExampleSelect/🟦️.ts`),
      // and the catalog pins that same text as `exampleLabel`, so this compares label to label — never the
      // kebab-case id to prose, the mistake `📓️default-example.md` retracted. A pane whose app declares no
      // `setActiveExample` has no trigger at all and is covered by the play unit gate instead
      // (`🧪️tests/🧪️playpanedefaults`: "names every pane whose curated example cannot reach its app").
      const fixture = page.locator(`[data-play-pane="${pane.variant}"] [id="playground.navbar.fixture"]`);
      if (pane.exampleLabel !== undefined && (await fixture.count()) > 0) {
        await expect(fixture, `${pane.variant} chrome must name its curated example ${pane.example}`).toHaveText(pane.exampleLabel);
      }

      // 🖼️ …and a window must SHOW something. A shell that reports ready over an unpainted viewport is
      // the defect class `📓️audit-visual-2.md` had to find by eye on 69 screenshots.
      const witness = await paintWitness(page, pane.variant);
      expect(witness.painted, `${pane.variant} paints nothing — ${witness.detail}`).toBe(true);
      expect(fallbackAssets, `${pane.variant} assets answered by the SPA fallback`).toEqual([]);

      await page.locator("[data-play-overview-button]").click();
      await expect(page.locator("[data-play-overview]")).toBeVisible();
    });
  }
});
