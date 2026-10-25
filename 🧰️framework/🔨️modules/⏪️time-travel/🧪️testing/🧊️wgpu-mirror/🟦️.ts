/** 🧊️ The wgpu shell's accessibility mirror and the band words it announces. */
import { evalSafe, markToken, waitUntil } from "../🔬️probe/🟦️.ts";
import { page } from "../🧠️state/🟦️.ts";
import type { Band } from "../🎗️band/📋️readout/🟦️.ts";

//#region 🔖️Wgpu

/** 🖌️ The wgpu shell paints on a canvas and publishes no DOM of its own. What a user (or an assistive technology) can
 * reach is: the ARIA mirror `#semio-wgpu-accessibility` (one element per projected node, `data-node-key` = the node key or
 * chrome control id, `data-window` = its window; a synthetic `click` on an actionable element is an `accessibility-activate`,
 * typing into a mirrored input an `accessibility-value`), the chrome hit registry (`semioWgpuIntrospection.dumpChrome()`:
 * every pointer target with its page rect, and the dispatched-action ledger when `SEMIO_RUNTIME_DIAGNOSTICS` is armed), and
 * the canvas itself for pointer and keyboard input.
 * @see ../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts
 * @see ../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs */
export type MirrorNode = { key: string; window: string; role: string; inputType: string | null; label: string; description: string; disabled: boolean; expanded: string | null; pressed: string | null; selected: string | null; value: string | null; valueNow: string | null; valueMin: string | null; valueMax: string | null; valueText: string | null; actionable: boolean; focused: boolean; live: string | null; busy: boolean; shortcut: string | null; tag: string; invalid: boolean; setSize: number | null; posInSet: number | null; step: string | null; min: string | null; max: string | null; tone: string | null; checked: string | null; readonly: boolean };

/** 🪞️ Every mirrored node in reading order, with the ARIA state the mirror stamps. */
export const mirror = () =>
  evalSafe(
    () =>
      Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).map((el) => {
        const describedBy = el.getAttribute("aria-describedby");
        return {
          key: el.dataset.nodeKey ?? "",
          window: el.dataset.window ?? "",
          role: el.getAttribute("role") ?? el.tagName.toLowerCase(),
          label: el.getAttribute("aria-label") ?? el.textContent ?? "",
          description: describedBy ? (document.getElementById(describedBy)?.textContent ?? "") : "",
          disabled: el.getAttribute("aria-disabled") === "true",
          expanded: el.getAttribute("aria-expanded"),
          pressed: el.getAttribute("aria-pressed"),
          selected: el.getAttribute("aria-selected"),
          value: el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement ? el.value : null,
          valueNow: el.getAttribute("aria-valuenow"),
          valueMin: el.getAttribute("aria-valuemin"),
          valueMax: el.getAttribute("aria-valuemax"),
          valueText: el.getAttribute("aria-valuetext"),
          actionable: el.dataset.actionable === "true",
          focused: el.dataset.focused === "true" || document.activeElement === el,
          live: el.getAttribute("aria-live"),
          busy: el.getAttribute("aria-busy") === "true",
          shortcut: el.getAttribute("aria-keyshortcuts"),
          tag: el.tagName.toLowerCase(),
          inputType: el instanceof HTMLInputElement ? el.type : null,
          invalid: el.getAttribute("aria-invalid") === "true",
          setSize: el.hasAttribute("aria-setsize") ? Number(el.getAttribute("aria-setsize")) : null,
          posInSet: el.hasAttribute("aria-posinset") ? Number(el.getAttribute("aria-posinset")) : null,
          step: el.getAttribute("step"),
          min: el.getAttribute("min"),
          max: el.getAttribute("max"),
          tone: el.getAttribute("data-tone"),
          checked: el.getAttribute("aria-checked"),
          readonly: el.getAttribute("aria-readonly") === "true" || (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) && el.readOnly,
        };
      }),
    [] as MirrorNode[],
  );

/** 🔎️ The mirrored node an authored key names: exact, then a `/`- or `.`-delimited suffix (a chrome control id prefixes
 * the tab id: `shell.panel.tab.<anchor>.<tabId>`), never a bare substring. */
export const mirrorFind = (nodes: MirrorNode[], authored: string) =>
  nodes.find((node) => node.key === authored) ?? nodes.find((node) => !node.key.includes("::") && (node.key.endsWith(`/${authored}`) || node.key.endsWith(`␟${authored}`) || node.key.endsWith(`\u001f${authored}`) || node.key.endsWith(`.${authored}`)));

/** 🧷️ Stamps the mirrored element of `key` (in `window` when given) with a probe token and answers its locator. */
export const mirrorLocator = async (key: string, windowId?: string) => {
  const token = markToken();
  const found = await evalSafe(
    (arg) => {
      const el = Array.from(document.querySelectorAll<HTMLElement>("#semio-wgpu-accessibility [data-node-key]")).find((node) => node.dataset.nodeKey === arg.key && (!arg.windowId || node.dataset.window === arg.windowId));
      el?.setAttribute("data-probe-target", arg.token);
      return Boolean(el);
    },
    false,
    { key, windowId: windowId ?? null, token },
  );
  return found ? page.locator(`[data-probe-target="${token}"]`).first() : null;
};

/** 👆️ Activates one mirrored node the way an assistive technology does: a `click` on its mirror element, which the mirror
 * forwards as `accessibility-activate` to the renderer. */
export const mirrorActivate = async (key: string, windowId?: string) => {
  const target = await mirrorLocator(key, windowId);
  if (!target) return false;
  await target.dispatchEvent("click").catch(() => {});
  return true;
};

/** 🎯️ Focuses one mirrored node (the mirror forwards `accessibility-focus`, so keyboard input then reaches that node). */
export const mirrorFocus = async (key: string) => {
  const target = await mirrorLocator(key);
  if (!target) return false;
  await target.focus().catch(() => {});
  return true;
};

/** 🧲️ The chrome hit registry: every pointer target with its page rect (CSS px). */
export const chromeHits = async () => {
  const json = await evalSafe(async () => (await (window as unknown as { semioWgpuIntrospection?: { dumpChrome: () => Promise<string> } }).semioWgpuIntrospection?.dumpChrome()) ?? "", "");
  try {
    return (JSON.parse(json || "{}") as { hits?: { controlId: string; kind: string; rect: [number, number, number, number]; windowId?: string; action?: string }[]; actions?: { seq: number; controllerId: string; action: string; origin: string; args?: string }[] });
  } catch {
    return {};
  }
};

/** 🖱️ Presses a chrome control with the real pointer at the centre of its registered hit rect. */
const pointerPress = async (authored: string) => {
  const hits = (await chromeHits()).hits ?? [];
  const hit = hits.find((row) => row.controlId === authored) ?? hits.find((row) => row.controlId.endsWith(`.${authored}`) || row.controlId.endsWith(`/${authored}`));
  if (!hit) return null;
  await page.mouse.click(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
  return hit.controlId;
};

/** ☝️ Presses `authored` through the mirror (activation), else with the pointer on its chrome hit rect. */
/** 🪂️ Waits until the mirror holds the node `authored` names (and, with `enabled`, until it is not `aria-disabled`): a panel's
 * nodes reach the mirror up to ~3 s after the press that opens it, and a control enabled by typing follows its field by a refresh
 * or two (measured on the first live wgpu run). Answers the node, or null after `timeoutMs`. */
export const mirrorAwait = async (authored: string, timeoutMs = 6000, enabled = false) => {
  const found = await waitUntil(async () => mirrorFind(await mirror(), authored) ?? null, (node) => node !== null && (!enabled || !node.disabled), timeoutMs, 300);
  return found.ok ? found.value : null;
};

export const wgpuPress = async (authored: string) => {
  const node = mirrorFind(await mirror(), authored);
  if (node && (await mirrorActivate(node.key, node.window))) return `mirror:${node.key}`;
  const hit = await pointerPress(authored);
  return hit ? `pointer:${hit}` : null;
};

/** 🎮️ Band controls as the wgpu shell names them (`⏪️time-travel` `TimeTravelControl::control_id`). */
export const WGPU_BAND_CONTROLS: readonly (readonly [string, string])[] = [
  ["nextProblem", "shell.time-travel.next-problem"],
  ["accept", "ui.timeTravel.accept"],
  ["discard", "ui.timeTravel.discard"],
  ["cancelReplay", "shell.time-travel.cancel-replay"],
  ["rerun", "shell.time-travel.rerun"],
  ["finalize", "shell.time-travel.finalize"],
  ["back", "shell.time-travel.back"],
  ["exit", "ui.timeTravel.exit"],
];

/** 🪧️ Stage and review words of the band's announced message (`TimeTravelLabel`, en and de), in match order. */
export const BAND_WORDS: readonly (readonly [string, RegExp])[] = [
  ["replaying", /Replaying later mutations|Spätere Mutationen werden neu angewendet/],
  ["reviewing", /Reviewing the edited history|Bearbeiteter Verlauf wird geprüft/],
  ["choosing", /Choose how to finalize|Art des Abschlusses wählen/],
  ["finalizing", /Finalizing the history edit|Verlaufsbearbeitung wird abgeschlossen/],
  ["editing", /Editing a mutation|Mutation wird bearbeitet/],
];

export const REVIEW_WORDS: readonly (readonly [string, RegExp])[] = [
  ["ready", /Ready to finalize|Bereit zum Abschließen/],
  ["blocked", /Errors must be fixed or withdrawn before finalizing|Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden/],
  ["needsReplay", /Replay needed|Neu anwenden nötig/],
  ["noChanges", /No changes: showing the current history|Keine Änderungen: aktueller Verlauf wird angezeigt/],
];

/** 📣️ The band as the wgpu shell announces it: the live `shell.time-travel.status` node (its message joins stage · target ·
 * progress · review · worst · fault · accepted) and the band controls the mirror carries. */
export const wgpuBand = async (): Promise<Band | null> => {
  const nodes = await mirror();
  const status = nodes.find((node) => node.key === "shell.time-travel.status");
  if (!status) return null;
  const meter = nodes.find((node) => node.key === "shell.time-travel.progress") ?? status;
  const text = status.label;
  const controls = WGPU_BAND_CONTROLS.map(([control, key]) => [control, nodes.find((node) => node.key === key)] as const)
    .filter((entry): entry is readonly [string, MirrorNode] => Boolean(entry[1]))
    .map(([control, node]) => ({ control, disabled: node.disabled, title: node.description || null, keys: node.shortcut, text: node.label, native: false, id: node.key }));
  return {
    stage: BAND_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? "",
    role: status.role,
    live: status.live,
    generation: null,
    text,
    review: REVIEW_WORDS.find(([, pattern]) => pattern.test(text))?.[0] ?? null,
    outcome: /Worst outcome: (\w+)|Schwerstes Ergebnis: (\w+)/.exec(text)?.slice(1).find(Boolean)?.toLowerCase() ?? null,
    fault: null,
    target: text,
    progress: meter.valueNow !== null && meter.valueMax !== null ? `${meter.valueNow}/${meter.valueMax}` : null,
    controls,
  };
};
//#endregion 🔖️Wgpu
