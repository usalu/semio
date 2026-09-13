/** 🧩️ Semantic parity structure owner. */

import { ProbeRunResult } from "../🔬️probe/🟦️.ts";



//#region 🔬️ParityScript
/** 🔬️wgpu↔React UI-parity verification harness — structural DOM/retained-tree comparison, per-region
 * pixel diffing, and a boot-triage ladder, driven per catalog playground. Ticket:
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/`. */

//#region 🔖️ParityTypes
type ParityRenderer = "react" | "wgpu";

type ParityRect = readonly [number, number, number, number];

type ParityColor = readonly [number, number, number, number];

type ParityNode = {
  readonly path: string;
  readonly kind: string;
  readonly rect: ParityRect;
  readonly text: string | null;
  readonly color: ParityColor | null;
  readonly bg: ParityColor | null;
  readonly fontSize: number | null;
  readonly fontWeight: number | null;
  readonly visible: boolean;
  readonly state: { readonly hovered: boolean; readonly disabled: boolean; readonly selected: boolean };
};

type ParityDump = {
  readonly viewport: { readonly w: number; readonly h: number; readonly dpr: number };
  readonly focusPath: string | null;
  readonly nodes: readonly ParityNode[];
};

type ParityMismatchAxis = "topology" | "text" | "rect" | "color" | "bg" | "fontSize" | "focus";

type ParityMismatch = { readonly path: string; readonly axis: ParityMismatchAxis; readonly react: unknown; readonly wgpu: unknown };

type StructuralResult = { readonly status: "PASS" | "FAIL"; readonly nodeCount: number; readonly mismatches: readonly ParityMismatch[] };

/** 🪜️Boot-triage ladder status — evaluated before any structural/pixel comparison, never conflated with a mismatch.
 * `STALE-BRIDGE` (terra-parity-rebaseline): a variant activation failing because its on-disk jco bridge
 * (`🔌️plugin-modules/<variant>/semio_s_plugin_*.js`) is still the pre-H2 `runSerialized` shape and has no
 * `createActorApi` export — see `🔖️Triage`'s `parityClassifyStaleBridge`. Distinct from every other rung:
 * those are architecture/runtime defects, this is "the fleet hasn't regenerated this bridge yet", which is
 * the EXPECTED state for most of the 58 variants until `sdk-green` lands (📌️important.md, §"48 materialised
 * plugin bridges"). A sweep that reports this as a bare FAIL is not measuring anything real. */
type BootStatus = "PASS" | "SERVER-FAIL" | "BOOT-TIMEOUT" | "ENV-FAIL" | "DUMP-EMPTY" | "BLANK-PAINT" | "STALE-BRIDGE";

type PixelRegionResult = { readonly path: string; readonly ratio: number; readonly threshold: number; readonly diffPng?: string };

type ParityPlaygroundReport = {
  readonly variant: string;
  readonly boot: { readonly react: BootStatus; readonly wgpu: BootStatus; readonly detail?: string };
  readonly structural?: StructuralResult;
  readonly pixel?: { readonly status: "PASS" | "FAIL"; readonly regions: readonly PixelRegionResult[] };
  /** 🎬️See `🔖️ProbeCatalog` — behavioral (interaction-driven) parity, distinct from the static
   * `structural`/`pixel` end-state checks above. Optional: only populated once boot passed (a probe
   * can't drive a page that never finished booting). */
  readonly behavioral?: ProbeRunResult;
  readonly durationMs: number;
};

//#endregion 🔖️ParityTypes

//#region 🔖️StructuralDump
/** 🌳️DOM-side structural walk — every element carrying `data-ui-path` (see `framework/os/renderer/js/react/index.tsx`
 * region `🔖️UiInterpreter`) is one matched node. `text` is only captured for non-container kinds since
 * `textContent` on a container aggregates all descendant text, which would false-positive against wgpu's
 * per-node (non-aggregated) text field. */
const PARITY_CONTAINER_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "externalSlot"]);

const REACT_DOM_DUMP_SCRIPT = `(() => {
  const CONTAINER_KINDS = new Set(${JSON.stringify([...PARITY_CONTAINER_KINDS])});
  function parseColor(str) {
    const m = /rgba?\\(([^)]+)\\)/.exec(str || "");
    if (!m) return null;
    const parts = m[1].split(",").map((s) => parseFloat(s.trim()));
    if (parts.length < 3) return null;
    return [Math.round(parts[0]), Math.round(parts[1]), Math.round(parts[2]), parts[3] === undefined ? 1 : parts[3]];
  }
  function nearestPath(el) {
    let cur = el;
    while (cur) {
      const p = cur.getAttribute && cur.getAttribute("data-ui-path");
      if (p) return p;
      cur = cur.parentElement;
    }
    return null;
  }
  const nodes = [];
  document.querySelectorAll("[data-ui-path]").forEach((el) => {
    const rect = el.getBoundingClientRect();
    const style = getComputedStyle(el);
    const path = el.getAttribute("data-ui-path");
    const kind = (path.split("/").pop() || "").replace(/\\[.*/, "").replace(/^#.*/, "");
    nodes.push({
      path,
      kind,
      rect: [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)],
      text: CONTAINER_KINDS.has(kind) ? null : (el.textContent || "").replace(/\\s+/g, " ").trim() || null,
      color: parseColor(style.color),
      bg: parseColor(style.backgroundColor),
      fontSize: parseFloat(style.fontSize) || null,
      fontWeight: parseFloat(style.fontWeight) || null,
      visible: rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none",
      state: {
        hovered: el.matches(":hover"),
        disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true",
        selected: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-pressed") === "true",
      },
    });
  });
  const active = document.activeElement;
  return JSON.stringify({
    viewport: { w: window.innerWidth, h: window.innerHeight, dpr: window.devicePixelRatio },
    focusPath: active ? nearestPath(active) : null,
    nodes,
  });
})()`;

async function dumpReactStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(REACT_DOM_DUMP_SCRIPT);
  return JSON.parse(json as unknown as string) as ParityDump;
}

/** 🧊️Calls the wasm-bindgen introspection hooks exposed by the renderer's `🗣️Interpreter/🎯️targets/🧊️wgpu`
 * region `🔬️IntrospectionExports`. They read `UI_ENGINE`, a thread-local that lives inside the dedicated
 * `semio-frame-worker`, so the UI isolate reaches them over the transport's introspection message pair and
 * `🚀️browser-boot/🟦️.ts` publishes that async shim as `window.semioWgpuIntrospection` once the Worker
 * reports `booted`. Deliberately NOT `window.wasmBindings`: Trunk publishes a second, never-booted
 * UI-thread instantiation of the same module under that name, whose `dumpStructure` traps on an
 * uninitialised engine and appears seconds before the Worker is ready. Returns an empty dump (never throws)
 * when the hooks aren't present yet, so triage can distinguish "not booted" from "no hooks" via
 * `DUMP-EMPTY`. */
async function dumpWgpuStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(async () => await (window as unknown as { semioWgpuIntrospection?: { dumpStructure?: () => Promise<string> } }).semioWgpuIntrospection?.dumpStructure?.());
  if (!json) return { viewport: { w: 0, h: 0, dpr: 1 }, focusPath: null, nodes: [] };
  return JSON.parse(json) as ParityDump;
}

async function dumpWgpuFrameStats(page: import("playwright").Page): Promise<{ readonly drawCalls: number; readonly quads: number; readonly glyphs: number } | null> {
  const json = await page.evaluate(async () => await (window as unknown as { semioWgpuIntrospection?: { dumpFrameStats?: () => Promise<string> } }).semioWgpuIntrospection?.dumpFrameStats?.());
  if (!json) return null;
  const stats = JSON.parse(json) as { readonly drawCalls: number; readonly quadCount: number; readonly glyphCount: number };
  return { drawCalls: stats.drawCalls, quads: stats.quadCount, glyphs: stats.glyphCount };
}

//#endregion 🔖️StructuralDump

//#region 🔖️StructuralCompare
const PARITY_RECT_TOLERANCE_PX = 1.5;

const PARITY_COLOR_TOLERANCE = 3;

const PARITY_FONT_SIZE_TOLERANCE_PX = 0.5;

/** 🎨️Scene canvases are rasterized by two different pipelines — structural comparison covers only
 * their rect (placement), never their internal text/color, which is a pixel/behavioral-probe concern. */
const PARITY_SCENE_LEAF_KINDS = new Set(["componentScene", "image"]);

function parityNormalizeText(s: string | null): string | null {
  return s === null ? null : s.normalize("NFC").replace(/\s+/g, " ").trim();
}

function parityColorClose(a: ParityColor | null, b: ParityColor | null): boolean {
  if (a === null || b === null) return a === b;
  return Math.abs(a[0] - b[0]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[1] - b[1]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[2] - b[2]) <= PARITY_COLOR_TOLERANCE;
}

/** 🎨️React's dump reports sRGB `rgb()` CSS values as 0–255 ints; wgpu's `Theme` colors are LINEAR-space
 * 0–1 floats (see `framework/os/renderer/wgpu/rs/lib.rs`'s `🔬️IntrospectionVisualFields` doc comment) —
 * comparing them raw would treat every color as a mismatch. Converts wgpu's linear floats to sRGB
 * 0–255 ints so both sides land in the same space before `parityColorClose`'s byte-scale tolerance applies. */
function parityLinearToSrgbColor(c: ParityColor | null): ParityColor | null {
  if (c === null) return null;
  const toByte = (channel: number): number => {
    const clamped = Math.min(1, Math.max(0, channel));
    const srgb = clamped <= 0.0031308 ? clamped * 12.92 : 1.055 * Math.pow(clamped, 1 / 2.4) - 0.055;
    return Math.round(srgb * 255);
  };
  return [toByte(c[0]), toByte(c[1]), toByte(c[2]), c[3]];
}

function compareParityStructural(reactDump: ParityDump, wgpuDump: ParityDump): StructuralResult {
  const mismatches: ParityMismatch[] = [];
  const reactByPath = new Map(reactDump.nodes.map((n) => [n.path, n]));
  const wgpuByPath = new Map(wgpuDump.nodes.map((n) => [n.path, n]));
  const allPaths = new Set([...reactByPath.keys(), ...wgpuByPath.keys()]);
  for (const path of allPaths) {
    const r = reactByPath.get(path);
    const w = wgpuByPath.get(path);
    if (!r || !w) {
      mismatches.push({ path, axis: "topology", react: r?.kind ?? null, wgpu: w?.kind ?? null });
      continue;
    }
    const isSceneLeaf = PARITY_SCENE_LEAF_KINDS.has(r.kind);
    if (!isSceneLeaf && parityNormalizeText(r.text) !== parityNormalizeText(w.text)) {
      mismatches.push({ path, axis: "text", react: r.text, wgpu: w.text });
    }
    const [rx, ry, rw, rh] = r.rect;
    const [wx, wy, ww, wh] = w.rect;
    if (Math.abs(rx - wx) > PARITY_RECT_TOLERANCE_PX || Math.abs(ry - wy) > PARITY_RECT_TOLERANCE_PX || Math.abs(rw - ww) > PARITY_RECT_TOLERANCE_PX || Math.abs(rh - wh) > PARITY_RECT_TOLERANCE_PX) {
      mismatches.push({ path, axis: "rect", react: r.rect, wgpu: w.rect });
    }
    const wColorSrgb = parityLinearToSrgbColor(w.color);
    const wBgSrgb = parityLinearToSrgbColor(w.bg);
    if (!isSceneLeaf && !parityColorClose(r.color, wColorSrgb)) mismatches.push({ path, axis: "color", react: r.color, wgpu: wColorSrgb });
    if (!isSceneLeaf && !parityColorClose(r.bg, wBgSrgb)) mismatches.push({ path, axis: "bg", react: r.bg, wgpu: wBgSrgb });
    if (r.fontSize !== null && w.fontSize !== null && Math.abs(r.fontSize - w.fontSize) > PARITY_FONT_SIZE_TOLERANCE_PX) {
      mismatches.push({ path, axis: "fontSize", react: r.fontSize, wgpu: w.fontSize });
    }
  }
  if (parityNormalizeText(reactDump.focusPath) !== parityNormalizeText(wgpuDump.focusPath)) {
    mismatches.push({ path: "$focus", axis: "focus", react: reactDump.focusPath, wgpu: wgpuDump.focusPath });
  }
  return { status: mismatches.length === 0 ? "PASS" : "FAIL", nodeCount: allPaths.size, mismatches: mismatches.slice(0, 200) };
}

export { BootStatus, PARITY_COLOR_TOLERANCE, PARITY_CONTAINER_KINDS, PARITY_FONT_SIZE_TOLERANCE_PX, PARITY_RECT_TOLERANCE_PX, PARITY_SCENE_LEAF_KINDS, ParityColor, ParityDump, ParityMismatch, ParityMismatchAxis, ParityNode, ParityPlaygroundReport, ParityRect, ParityRenderer, PixelRegionResult, REACT_DOM_DUMP_SCRIPT, StructuralResult, compareParityStructural, dumpReactStructure, dumpWgpuFrameStats, dumpWgpuStructure, parityColorClose, parityLinearToSrgbColor, parityNormalizeText };
