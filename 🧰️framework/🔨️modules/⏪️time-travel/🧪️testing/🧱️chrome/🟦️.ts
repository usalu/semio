/** 🪟️ Shell chrome: DOM ids, dock panels, the boot wait and the tour dialog. */
import { page, renderer, currentLocale, setLastBootFault, universalRoute, scenario } from "../🧠️state/🟦️.ts";
import type { Point } from "../🧠️state/🟦️.ts";
import { evalSafe, sleep, waitUntil } from "../📄️page/🟦️.ts";
import { mirrorFind, mirror, wgpuPress } from "../🧊️wgpu-mirror/🟦️.ts";
import { log } from "../🧾️report/🟦️.ts";
import { COPY } from "../🌍️copy/🟦️.ts";

//#region 🔖️Arguments

export const HISTORY_TAB = "framework.panel.history";

export const INSPECTION_TAB = "framework.panel.inspection";
//#endregion 🔖️Arguments
//#region 🔖️Board

export const inside = (point: Point, box: { x: number; y: number; width: number; height: number }, margin: number) => point.x > box.x + margin && point.x < box.x + box.width - margin && point.y > box.y + margin && point.y < box.y + box.height - margin;
//#endregion 🔖️Board
//#region 🔖️Chrome

/** 🪪️ The live DOM id of an authored id: exact, or namespaced `…/<authored>` (panel bodies, window measures). */
export const resolveDomId = (authored: string) =>
  evalSafe(
    (target) => {
      for (const el of Array.from(document.querySelectorAll("[id]"))) if (el.id === target || el.id.endsWith(`/${target}`) || el.id.endsWith(`␟${target}`)) return el.id;
      return null;
    },
    null as string | null,
    authored,
  );

export const byId = (id: string) => page.locator(`[id="${id.replace(/"/g, '\\"')}"]`).first();

/** 🗝️ The live handle of an authored key whichever renderer answers: React's DOM id, wgpu's mirror key. */
export const presentKey = async (authored: string) => (renderer === "wgpu" ? (mirrorFind(await mirror(), authored)?.key ?? null) : resolveDomId(authored));

/** 🚥️ The per-window time-travel indicators as `stage-or-role:accessible name` (React `[data-semio-time-travel-indicator]`,
 * wgpu `framework.window.<id>.timeTravel.indicator`, a `note`). */
export const indicators = async () =>
  renderer === "wgpu"
    ? (await mirror()).filter((node) => /framework\.window\..+\.timeTravel\.indicator$/.test(node.key)).map((node) => `${node.role}:${node.label}`)
    : evalSafe(() => Array.from(document.querySelectorAll("[data-semio-time-travel-indicator]")).map((el) => `${el.getAttribute("data-semio-time-travel-indicator")}:${el.getAttribute("aria-label")}`), [] as string[]);

/** 👥️ The presence roster's accessible text (React `#s-presence-peers`, wgpu the `s-presence-peers` status node). */
export const presenceText = async () =>
  renderer === "wgpu"
    ? (mirrorFind(await mirror(), "s-presence-peers")?.label ?? null)
    : evalSafe(() => {
        const el = document.getElementById("s-presence-peers");
        return el ? `${el.getAttribute("aria-label") ?? ""} ${(el as HTMLElement).innerText}`.trim() : null;
      }, null as string | null);

/** 🔡️ The visible text of an authored key: React's `innerText`, wgpu's accessible name and description. */
export const textOfKey = async (authored: string) => {
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), authored);
    return node ? `${node.label} ${node.description}`.trim() : "";
  }
  const id = await resolveDomId(authored);
  return id ? await byId(id).innerText().catch(() => "") : "";
};

/** 🗃️ The dock panels that are open, by tab id. wgpu: a panel is open when the mirror carries its body — nodes whose
 * `data-window` IS the tab id (`framework.panel.history`, `s-sync-status`), the tab being the chrome button of that key.
 * `aria-pressed` is not the witness: on B2 five chrome buttons read `pressed=true` at boot while no panel is shown
 * (`wgpuPressedTabsWithoutPanel` judges that on its own). */
export const openPanelTabIds = async () => {
  if (renderer !== "wgpu") return reactOpenPanelTabIds();
  const nodes = await mirror();
  const windows = new Set(nodes.map((node) => node.window));
  return nodes.filter((node) => node.window === "shell.chrome" && node.role === "button" && !node.key.includes("::") && windows.has(node.key)).map((node) => node.key);
};

/** 🫥️ The wgpu chrome's panel tabs that read `aria-pressed=true` while the mirror carries no body for them. */
export const wgpuPressedTabsWithoutPanel = async () => {
  const nodes = await mirror();
  const windows = new Set(nodes.map((node) => node.window));
  return nodes.filter((node) => node.window === "shell.chrome" && node.role === "button" && node.pressed === "true" && /(^|\.)panel\./.test(node.key) && !windows.has(node.key)).map((node) => `${node.key}=${node.label}`);
};

const reactOpenPanelTabIds = () => evalSafe(() => Array.from(document.querySelectorAll('[data-slot="panel"]')).filter((el) => (el as HTMLElement).offsetParent !== null).map((el) => el.id.replace(/^framework\.panelTab\./, "")), [] as string[]);

/** 🍔️ The phone layout's one toggle for the merged mobile panel (`ui.mobilePanel.toggle`, a navbar Toggle). */
const MOBILE_PANEL_TOGGLE = "ui.mobilePanel.toggle";

/** 🪧 Opens (never toggles) the panel tab `id`; at phone width the tab lives in the merged mobile panel, so its toggle opens
 * first. */
export const openTab = async (id: string) => {
  if ((await openPanelTabIds()).includes(id)) return true;
  if (renderer === "wgpu") {
    let via = await wgpuPress(id);
    if (via === null && (await wgpuPress(MOBILE_PANEL_TOGGLE)) !== null) {
      await sleep(700);
      via = await wgpuPress(id);
    }
    return via !== null && (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
  }
  const tab = page.locator(`[data-slot="panel-tab-button"][id="${id}"], [id="${id}"]`).first();
  const toggle = page.locator(`[id="${MOBILE_PANEL_TOGGLE}"]`).first();
  if (!(await tab.count().catch(() => 0)) && (await toggle.count().catch(() => 0))) {
    await toggle.click({ timeout: 3000 }).catch(() => {});
    await sleep(700);
  }
  if (!(await tab.count().catch(() => 0))) return false;
  await tab.click({ timeout: 4000 }).catch(() => {});
  return (await waitUntil(openPanelTabIds, (ids) => ids.includes(id), 8000)).ok;
};

/** 🔒️ Closes every open dock panel (they overlay the board, and a pointer gesture there lands on the panel), and the merged
 * mobile panel when it is open. */
export const closePanels = async () => {
  for (const id of await openPanelTabIds()) {
    if (renderer === "wgpu") await wgpuPress(id);
    else await page.locator(`[data-slot="panel-tab-button"][id="${id}"]`).first().click({ timeout: 3000 }).catch(() => {});
    await sleep(300);
  }
  if (renderer === "wgpu") {
    if (mirrorFind(await mirror(), MOBILE_PANEL_TOGGLE)?.pressed === "true") await wgpuPress(MOBILE_PANEL_TOGGLE);
  } else {
    const toggle = page.locator(`[id="${MOBILE_PANEL_TOGGLE}"][aria-pressed="true"], [id="${MOBILE_PANEL_TOGGLE}"][data-state="on"]`).first();
    if (await toggle.count().catch(() => 0)) await toggle.click({ timeout: 3000 }).catch(() => {});
  }
  return openPanelTabIds();
};

/** 🙈️ Dismisses a boot tour dialog by its Skip/Close button (en or de), scoped to dialogs only. */
const dismissTour = async () => {
  if (renderer === "wgpu") {
    if (mirrorFind(await mirror(), "ui.introduction.skip") && (await wgpuPress("ui.introduction.skip"))) log("dismissed tour ui.introduction.skip");
    return;
  }
  for (const pattern of COPY[currentLocale].tourButtons) {
    const button = page.locator('[role="dialog"] button', { hasText: pattern }).first();
    if (await button.count().catch(() => 0)) {
      await button.click({ timeout: 2000 }).catch(() => {});
      await sleep(600);
      log(`dismissed tour ${pattern}`);
    }
  }
};

/** 🥾️ Waits for `windows` board windows (three on a desktop, one at phone width) and a canvas, dismissing a tour on the way. */
export const waitForBoot = async (label: string, polls = 100, windows = 3) => {
  for (let index = 0; index < polls; index++) {
    await sleep(3000);
    const shape = renderer === "wgpu" ? await wgpuBootShape() : await evalSafe(() => ({ windows: document.querySelectorAll('[data-slot="window"]').length, canvases: document.querySelectorAll("canvas").length, dialogs: document.querySelectorAll('[role="dialog"]').length, chrome: true, body: document.body?.innerText.slice(0, 160) ?? "" }), { windows: 0, canvases: 0, dialogs: 0, chrome: false, body: "" });
    if (renderer === "wgpu" && shape.canvases === 0) {
      const fault = await evalSafe(() => (document.body?.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 300), "");
      if (/boot-failed|renderer fault/u.test(fault)) {
        setLastBootFault(fault);
        log(`${label} wgpu shell refused to boot: ${fault}`);
        return false;
      }
    }
    if (shape.dialogs) await dismissTour();
    if ((universalRoute !== null ? shape.windows >= 1 : shape.windows >= windows && shape.canvases >= 1) && shape.chrome) {
      log(`${label} booted after ${((index + 1) * 3).toFixed(0)} s: ${JSON.stringify(shape).slice(0, 200)}`);
      await dismissTour();
      return true;
    }
    if (index % 5 === 4) log(`${label} waiting… ${JSON.stringify(shape).slice(0, 200)}`);
  }
  return false;
};

/** 🌅️ The wgpu boot shape: the introspection shim is attached only once the frame worker booted, its structure dump
 * names the live windows, and the ARIA mirror holds nodes once the first frame was projected — the shell counts as booted
 * for a person only once the mirror carries the chrome (`shell.chrome`, measured on B2 a few seconds after the boards). */
const wgpuBootShape = async () => {
  const shape = await evalSafe(
    async () => {
      const introspection = (window as unknown as { semioWgpuIntrospection?: { dumpStructure: () => Promise<string> } }).semioWgpuIntrospection;
      let windowIds: string[] = [];
      try {
        windowIds = introspection ? ((JSON.parse((await introspection.dumpStructure()) || "{}") as { windowIds?: string[] }).windowIds ?? []) : [];
      } catch {
        windowIds = [];
      }
      const mirrorRoot = document.getElementById("semio-wgpu-accessibility");
      return { booted: Boolean(introspection), windowIds, mirrorNodes: mirrorRoot?.querySelectorAll("[data-node-key]").length ?? 0, chromeNodes: mirrorRoot?.querySelectorAll('[data-window="shell.chrome"]').length ?? 0, canvases: document.querySelectorAll("canvas").length };
    },
    { booted: false, windowIds: [] as string[], mirrorNodes: 0, chromeNodes: 0, canvases: 0 },
  );
  const boards = shape.windowIds.filter((id) => id.startsWith(scenario?.bootWindowPrefix ?? "")).length;
  return { windows: boards, canvases: shape.canvases, dialogs: mirrorFind(await mirror(), "ui.introduction.skip") ? 1 : 0, chrome: shape.chromeNodes > 0, body: JSON.stringify(shape).slice(0, 160) };
};

/** 🎟️ Marks the first element `find` answers with `data-probe-target=<token>` in page, so Playwright can click the
 * exact element a structural search found. */
export const markToken = (() => {
  let next = 0;
  return () => `probe-${++next}`;
})();
//#endregion 🔖️Chrome
