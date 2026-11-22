// #region 🧲️Header
// 💻️ 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🖼️visible-content/🟦️.ts
// Specs: Shared visible-content assertions of the Play acceptance suite (dev and published release).
// Summary: Pixel census of a canvas, framework fault markers inside a pane, empty DOM window bodies and
// the SPA-fallback classification of an asset answer. Pure functions are unit-tested by `🧪️playvisiblecontent`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { PNG } from "pngjs";

import type { Page } from "../../../🔨️modules/🧪️e2e/📦️release/🎭️fixture/🟦️.ts";
// #endregion 🔌️Adapters

/** 📦️ Media, font and document paths that must be answered by a FILE. A dev server's SPA fallback happily
 * answers `GET /🖼️assets/🖼️bauteilbörse.png` with `index.html` and HTTP 200, so a missing figure looks like a
 * loaded one to everything except the eye (ticket 26/09/19 `📓️status.md`, 2026-09-22 04:10). */
export const MEDIA_ASSET_PATH = /\.(png|jpe?g|gif|webp|avif|svg|ico|bmp|glb|gltf|obj|stl|ply|3dm|mp3|mp4|webm|wav|ogg|woff2?|ttf|otf|pdf)$/i;

/** 🧩️ Code, data and style paths of the published release: a `.wasm`, `.js` or `.json` answered with
 * `index.html` is a missing module that the SPA fallback turned into a parse error. */
export const CODE_ASSET_PATH = /\.(wasm|m?js|cjs|json|css|map|webmanifest|txt|xml)$/i;

/** 🏚️ Describes an asset answered by a document (`text/html`), or `undefined` when the answer is fine. */
export function spaFallbackAnswer(url: string, contentType: string, status: number, patterns: readonly RegExp[]): string | undefined {
  if (!/^https?:/i.test(url)) return undefined;
  let pathname: string;
  try { pathname = new URL(url).pathname; } catch { return undefined; }
  let decoded: string;
  try { decoded = decodeURIComponent(pathname); } catch { decoded = pathname; }
  if (!patterns.some(pattern => pattern.test(decoded))) return undefined;
  return contentType.toLowerCase().startsWith("text/html") ? `${pathname} → ${status} ${contentType}` : undefined;
}

/** 🎨️ Counts the distinct RGB colours of the interior (the central half, every second pixel) of a PNG. The
 * inset keeps window chrome and borders out of the census. */
export function distinctInteriorColors(png: Buffer): number {
  const image = PNG.sync.read(png);
  const colors = new Set<number>();
  const inset = Math.max(1, Math.floor(Math.min(image.width, image.height) / 4));
  for (let y = inset; y < image.height - inset; y += 2) for (let x = inset; x < image.width - inset; x += 2) {
    const index = (y * image.width + x) * 4;
    colors.add((image.data[index]! << 16) | (image.data[index + 1]! << 8) | image.data[index + 2]!);
  }
  return colors.size;
}

/** 📐️ A canvas narrower than this on either edge is a decoration, not a window the reader looks at. */
export const MINIMUM_CANVAS_EDGE = 32;

/** 🚨️ The framework's own failure surfaces, each rendered by the named component:
 * `WindowFaultStatus` (`data-semio-window-fault`, plugin instance faulted), `canvas` error view
 * (`data-semio-os-shell-error`), `FaultBoundary` (`data-shell-fault-boundary`), `ErrorView`
 * (`data-slot="error-view"`), `LayeredPaneBoundary` (`data-layered-pane-error`), `ShellRouteNotFoundPage`
 * (`data-shell-route-not-found`), host bootstrap failure (`data-semio-bootstrap-status`, always `role="alert"`),
 * execution target failure (`role="alert"` variant of `data-semio-execution-target-status`), directory
 * bootstrap fault, `UnknownComponentView` (`data-unknown-component`), `MediaTransportHost` invalid state and
 * the refusal notices of Slider, Stepper and Interpreter inputs (rendered only while a refusal exists). */
export const FRAMEWORK_FAULT_SELECTOR = [
  "[data-semio-window-fault]",
  "[data-semio-os-shell-error]",
  "[data-shell-fault-boundary]",
  '[data-slot="error-view"]',
  "[data-layered-pane-error]",
  "[data-shell-route-not-found]",
  "[data-semio-bootstrap-status]",
  '[data-semio-execution-target-status][role="alert"]',
  '[data-directory-bootstrap="fault"]',
  '[data-directory-bootstrap="locale-missing"]',
  "[data-unknown-component]",
  '[data-media-state="invalid"]',
  '[data-slot="input-refusal"]',
  '[data-slot="slider-refusal"]',
  '[data-slot="stepper-refusal"]',
].join(", ");

/** 🚨️ Names every framework fault surface mounted inside the pane, with its text, or an empty list. */
export async function readFrameworkFaults(page: Page, variant: string): Promise<readonly string[]> {
  return page.evaluate(({ id, selector }) => {
    const pane = document.querySelector(`[data-layered-pane="${id}"]`);
    if (!pane) return [];
    return [...pane.querySelectorAll(selector)].map(element => {
      const marker = element.getAttributeNames().find(name => name.startsWith("data-")) ?? element.tagName.toLowerCase();
      return `${marker}: ${(element.textContent ?? "").trim().slice(0, 200)}`;
    });
  }, { id: variant, selector: FRAMEWORK_FAULT_SELECTOR });
}

/** 🕳️ Names every visible DOM window body of the pane that shows neither text nor `minimumElements`
 * elements and holds no canvas. Windows that mount a retained 3D/raster surface are judged by their own
 * witness and canvas census; windows with no layout box (inactive tabs) are not visible and are skipped. */
export async function readEmptyDomWindows(page: Page, variant: string, minimumElements: number): Promise<readonly string[]> {
  return page.evaluate(({ id, minimum }) => {
    const pane = document.querySelector(`[data-layered-pane="${id}"]`);
    if (!pane) return [];
    const area = (element: Element): number => { const box = element.getBoundingClientRect(); return box.width * box.height; };
    const empty: string[] = [];
    [...pane.querySelectorAll('[data-slot="window-body"]')].forEach((body, index) => {
      if (area(body) === 0 || body.querySelector("canvas") || body.querySelector("[data-meshes-json], [data-instances-json], [data-layers-json], [data-assets-json]")) return;
      const content = body.querySelector(':scope > [data-slot="pane-host-root"]');
      if (!content || area(content) === 0) { empty.push(`(window body ${index}) mounted no visible content host`); return; }
      const clone = content.cloneNode(true) as HTMLElement;
      clone.querySelectorAll('[data-slot="pane-host"], [role="status"][aria-busy="true"], [hidden], [aria-hidden="true"]').forEach(element => element.remove());
      const text = (clone.textContent ?? "").trim().length, elements = clone.querySelectorAll("*").length;
      if (text === 0 && elements < minimum) empty.push(`(window body ${index}) ${text} content characters, ${elements} content elements`);
    });
    return empty;
  }, { id: variant, minimum: minimumElements });
}

/** 🖼️ Distinct interior colours per visible canvas of the pane, largest first. An empty list means the pane
 * mounts no canvas and owes no pixel census. */
export async function censusPaneCanvases(page: Page, variant: string): Promise<readonly { readonly index: number; readonly colors: number }[]> {
  const pane = page.locator(`[data-layered-pane="${variant}"]`);
  const sizes = await pane.locator("canvas").evaluateAll((canvases, edge) => canvases.map((canvas, index) => {
    const box = canvas.getBoundingClientRect(), style = getComputedStyle(canvas);
    return { index, area: style.visibility === "hidden" || style.display === "none" || box.width < edge || box.height < edge ? 0 : box.width * box.height };
  }), MINIMUM_CANVAS_EDGE);
  const census: { index: number; colors: number }[] = [];
  for (const { index } of sizes.filter(size => size.area > 0).sort((a, b) => b.area - a.area)) census.push({ index, colors: distinctInteriorColors(await pane.locator("canvas").nth(index).screenshot()) });
  return census;
}

/** ⏳️ Polls the canvas census until some canvas is non-uniform (at least `minimumColors` distinct colours) or
 * the deadline passes. Returns `undefined` when the pane mounts no canvas, otherwise the final census. */
export async function awaitNonUniformCanvas(page: Page, variant: string, minimumColors: number, deadlineMs: number, pollMs: number): Promise<{ readonly painted: boolean; readonly census: readonly { readonly index: number; readonly colors: number }[] } | undefined> {
  const deadline = Date.now() + deadlineMs;
  let census = await censusPaneCanvases(page, variant);
  if (census.length === 0) return undefined;
  while (!census.some(row => row.colors >= minimumColors) && Date.now() < deadline) {
    await page.waitForTimeout(pollMs);
    census = await censusPaneCanvases(page, variant);
    if (census.length === 0) return undefined;
  }
  return { painted: census.some(row => row.colors >= minimumColors), census };
}
