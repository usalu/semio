/** 🖊️ The draft editor of a history session: its number controls, text inputs and focus. */
import { scenario, renderer, page } from "../🧠️state/🟦️.ts";
import { mirror, mirrorFind, mirrorFocus, mirrorLocator, mirrorAwait } from "../🧊️wgpu-mirror/🟦️.ts";
import { evalSafe, markToken, sleep, shot } from "../🔬️probe/🟦️.ts";
import { scrollHistory, openHistory, pressAuthored, revealHistory } from "../📜️history/🟦️.ts";
import { band, prepareChord } from "../🎗️band/🟦️.ts";
import { resolveDomId, inside } from "../🧱️chrome/🟦️.ts";
import { log } from "../🧾️report/🟦️.ts";

//#region 🔖️Band

type Editor = { heading: string | null; status: string | null; numbers: Record<string, StepperRead | null>; targets: { id: string; chips: string[]; useSelection: boolean; text: string } | null; inputs: string[] };

type StepperRead = { id: string; tag: string; stepper: boolean; value: string | null; step: string | null; min: string | null; max: string | null; plus: boolean; minus: boolean; role: string | null };

/** 🔢️ The number inputs of the mutation under edit that the scenario asks the editor read to carry. */
const editorNumbers = (): readonly string[] => scenario?.editorNumbers ?? [];

/** ✏️ The draft editor section: its heading, the Rust band status line, the scenario's number controls and the targets list. */
export const editor = (): Promise<Editor | null> => (renderer === "wgpu" ? wgpuEditor(editorNumbers()) : reactEditor(editorNumbers()));

/** 🖊️ The draft editor through the wgpu mirror; a number input is a `spinbutton` (or `slider`) whose value the mirror
 * carries as the input value / `aria-valuenow`. */
const wgpuEditor = async (pointers: readonly string[]): Promise<Editor | null> => {
  const nodes = await mirror();
  const number = (pointer: string): StepperRead | null => {
    const node = mirrorFind(nodes, `framework.history.editor.input.${pointer}`);
    if (!node) return null;
    const steppers = nodes.filter((other) => other.key.startsWith(`${node.key}::`) || other.key.startsWith(`${node.key}.`));
    return { id: node.key, tag: node.tag, stepper: node.role === "spinbutton" || (node.tag === "input" && node.inputType === "number"), value: node.value ?? node.valueNow, step: node.step, min: node.min, max: node.max, plus: steppers.some((other) => /increase|erhöhen|\+/i.test(other.label)), minus: steppers.some((other) => /decrease|verringern|−|-/i.test(other.label)), role: node.role };
  };
  const targets = mirrorFind(nodes, "framework.history.editor.input.targets.row");
  const chips = nodes.filter((node) => /framework\.history\.editor\.input\.targets\.chip\.\d+\.row$/.test(node.key)).map((node) => node.label);
  return {
    heading: mirrorFind(nodes, "framework.history.editor.target")?.label ?? null,
    status: mirrorFind(nodes, "framework.history.timeTravel.status")?.label ?? null,
    numbers: Object.fromEntries(pointers.map((pointer) => [pointer, number(pointer)])),
    targets: targets ? { id: targets.key, chips, useSelection: Boolean(mirrorFind(nodes, "framework.history.editor.input.targets.useSelection") ?? mirrorFind(nodes, "framework.history.editor.input.targets.useSelection.row")), text: `${targets.label} ${targets.description}`.trim() } : null,
    inputs: nodes.filter((node) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(node.key)).map((node) => `${node.key.replace(/^.*framework\.history\.editor\.input/, "")}=${node.label.slice(0, 60)}`),
  };
};

const reactEditor = (pointers: readonly string[]) =>
  evalSafe(
    (pointers) => {
      const find = (authored: string) => Array.from(document.querySelectorAll("[id]")).find((el) => el.id === authored || el.id.endsWith(`/${authored}`) || el.id.endsWith(`␟${authored}`)) as HTMLElement | undefined;
      const number = (pointer: string) => {
        const el = find(`framework.history.editor.input.${pointer}`);
        if (!el) return null;
        const input = (el.tagName === "INPUT" ? el : el.querySelector("input")) as HTMLInputElement | null;
        const group = el.closest('[data-slot="stepper-group"]') ?? el.querySelector('[data-slot="stepper-group"]');
        return {
          id: el.id,
          tag: el.tagName.toLowerCase(),
          stepper: input?.getAttribute("data-stepper-input") === "true",
          value: input?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? null,
          step: input?.getAttribute("step") ?? null,
          min: input?.getAttribute("min") ?? null,
          max: input?.getAttribute("max") ?? null,
          plus: Boolean(group?.querySelector('[data-slot="stepper-plus"]')),
          minus: Boolean(group?.querySelector('[data-slot="stepper-minus"]')),
          role: el.getAttribute("role") ?? el.querySelector('[role="slider"]')?.getAttribute("role") ?? null,
        };
      };
      const targetsEl = find("framework.history.editor.input.targets.row");
      const chipLabel = (el: Element) => ((el.querySelector('[data-slot="tree-label"] span') ?? el.querySelector('[data-slot="tree-label"]') ?? el) as HTMLElement).innerText.replace(/\s+/g, " ").trim();
      const chips = Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.targets\.chip\.\d+\.row$/.test(el.id)).map(chipLabel);
      return {
        heading: find("framework.history.editor.target")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        status: find("framework.history.timeTravel.status")?.innerText.replace(/\s+/g, " ").trim() ?? null,
        numbers: Object.fromEntries(pointers.map((pointer) => [pointer, number(pointer)])),
        targets: targetsEl ? { id: targetsEl.id, chips, useSelection: Boolean(find("framework.history.editor.input.targets.useSelection") ?? find("framework.history.editor.input.targets.useSelection.row")), text: targetsEl.innerText.replace(/\s+/g, " ").trim().slice(0, 200) } : null,
        inputs: Array.from(document.querySelectorAll("[id]")).filter((el) => /framework\.history\.editor\.input\.[^/]*\.row$/.test(el.id)).map((el) => `${el.id.replace(/^.*framework\.history\.editor\.input/, "")}=${(el as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 60)}`),
      };
    },
    null as Editor | null,
    pointers,
  );

/** 📗️ {@link editor} after revealing the editor's inputs section (`framework.history.editor.inputs`). */
export const readEditor = async () => {
  await openHistory();
  if (!(await revealHistory("framework.history.editor.inputs"))) await revealHistory("framework.history.editor");
  return editor();
};

/** 🪤 What the editor's number control at `pointer` is the moment before it is typed into: the band stage, the field's own state
 * and whether another element covers its centre. */
const diagnoseTyping = async (pointer: string) => ({
  stage: (await band())?.stage ?? null,
  field: await evalSafe(
    (target) => {
      const host = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === target || el.id.endsWith(`/${target}`) || el.id.endsWith(`␟${target}`));
      const field = (host?.tagName === "INPUT" ? host : host?.querySelector("input")) as HTMLInputElement | null;
      if (!field) return null;
      const rect = field.getBoundingClientRect();
      const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
      return { id: host?.id ?? "", disabled: field.disabled, readOnly: field.readOnly, value: field.value, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)], covered: top && top !== field && !field.contains(top) ? `${top.tagName.toLowerCase()}#${top.id}[${top.getAttribute("data-slot") ?? ""}]` : null };
    },
    null as { id: string; disabled: boolean; readOnly: boolean; value: string; rect: number[]; covered: string | null } | null,
    `framework.history.editor.input.${pointer}`,
  ),
});

/** 💯️ Types `value` into the editor's number control at `pointer` (one `fill` = one input event = one draft), then
 * Enter to commit and blur; answers the value the field held before Enter. */
export const typeEditorNumber = async (pointer: string, value: number) => {
  await revealHistory("framework.history.editor.inputs");
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null };
    await field.fill(String(value), { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await mirrorFocus(node.key);
    await page.keyboard.press("Enter").catch(() => {});
    return { present: true, typed };
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return { present: false, typed: null as string | null };
  const host = page.locator(`[id="${id}"]`).first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : host.locator("input").first();
  const before = await diagnoseTyping(pointer);
  const refused: string[] = [];
  const reason = (action: string) => (error: unknown) => void refused.push(`${action}: ${String(error).split("\n").slice(0, 6).join(" | ").slice(0, 500)}`);
  await input.click({ timeout: 4000 }).catch(reason("click"));
  await input.fill(String(value), { timeout: 4000 }).catch(reason("fill"));
  const typed = await input.inputValue({ timeout: 2000 }).catch(() => null);
  await input.press("Enter", { timeout: 4000 }).catch(reason("enter"));
  if (!refused.length) return { present: true, typed };
  const state = await input.evaluate((el) => {
    const field = el as HTMLInputElement;
    const rect = field.getBoundingClientRect();
    const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
    return { disabled: field.disabled, readOnly: field.readOnly, rect: [Math.round(rect.left), Math.round(rect.top), Math.round(rect.width), Math.round(rect.height)], covered: top && top !== field && !field.contains(top) ? `${top.tagName.toLowerCase()}#${top.id}[${top.getAttribute("data-slot") ?? ""}]` : null };
  }).catch(() => null);
  log(`typeEditorNumber ${pointer} refused ${JSON.stringify({ refused, state, before }).slice(0, 1800)}`);
  await shot(`type-${pointer}-refused`);
  return { present: true, typed };
};

/** ⬆️ Presses one key in the editor's number control at `pointer` (ArrowUp/ArrowDown step by the control's step). */
export const keyEditorNumber = async (pointer: string, key: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    if (!node || !(await mirrorFocus(node.key))) return false;
    await page.keyboard.press(key).catch(() => {});
    return true;
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) {
    log(`keyEditorNumber ${pointer} ${key}: no such control in the document (band ${(await band())?.stage ?? "none"})`);
    return false;
  }
  const host = page.locator(`[id="${id}"]`).first();
  const thumb = host.locator('[role="slider"]').first();
  const input = (await host.evaluate((el) => el.tagName === "INPUT").catch(() => false)) ? host : (await thumb.count().catch(() => 0)) ? thumb : host.locator("input").first();
  await input.focus().catch(() => {});
  await input.press(key).catch(() => {});
  return true;
};

type NumberControlRead = { id: string; dial: boolean; role: string | null; valueNow: number | null; valueText: string | null; min: number | null; max: number | null; ticks: { snap: number; left: number | null }[]; refusal: string | null; invalid: boolean; rowText: string };

/** 🎚️ One editor number control as a person perceives it: whether it is a dial, the thumb's announced value (display units)
 * and value text, the travel, every detent tick (its stored snap and, on a track, its axis position in %), the visible
 * refusal and the row's text. wgpu answers from the mirror (`role=slider|spinbutton`, `aria-value*`); its ticks are painted
 * only, so `ticks` is empty there. */
export const readNumberControl = async (pointer: string): Promise<NumberControlRead | null> => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  const authored = `framework.history.editor.input.${pointer}`;
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = mirrorFind(nodes, authored);
    const row = mirrorFind(nodes, `${authored}.row`);
    if (!node) return null;
    const number = (text: string | null) => (text === null || text === "" || !Number.isFinite(Number(text)) ? null : Number(text));
    return { id: node.key, dial: /dial/i.test(`${node.description} ${node.role}`), role: node.role, valueNow: number(node.valueNow), valueText: node.valueText, min: number(node.valueMin ?? node.min), max: number(node.valueMax ?? node.max), ticks: [], refusal: node.invalid ? node.description || null : null, invalid: node.invalid, rowText: `${row?.label ?? ""} ${row?.description ?? ""} ${node.invalid ? node.description : ""}`.trim() };
  }
  return evalSafe(
    (target) => {
      const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`) || el.id.endsWith(`␟${suffix}`));
      const host = find(target);
      if (!host) return null;
      const row = find(`${target}.row`) ?? host.parentElement ?? host;
      const thumb = host.querySelector<HTMLElement>('[role="slider"]') ?? (host.getAttribute("role") === "slider" ? host : null);
      const field = (host.tagName === "INPUT" ? host : host.querySelector("input")) as HTMLInputElement | null;
      const number = (text: string | null | undefined) => (text === null || text === undefined || text === "" || !Number.isFinite(Number(text)) ? null : Number(text));
      const alert = row.querySelector<HTMLElement>('[role="alert"]');
      return {
        id: host.id,
        dial: Boolean(host.querySelector('[data-slot="slider-dial"]')),
        role: thumb?.getAttribute("role") ?? field?.getAttribute("role") ?? field?.type ?? null,
        valueNow: number(thumb?.getAttribute("aria-valuenow") ?? field?.value),
        valueText: thumb?.getAttribute("aria-valuetext") ?? field?.getAttribute("aria-valuetext") ?? null,
        min: number(thumb?.getAttribute("aria-valuemin") ?? field?.min),
        max: number(thumb?.getAttribute("aria-valuemax") ?? field?.max),
        ticks: Array.from(host.querySelectorAll<HTMLElement>('[data-slot="slider-tick"]')).map((tick) => ({ snap: Number(tick.getAttribute("data-snap")), left: tick.style?.left ? parseFloat(tick.style.left) : null })),
        refusal: alert?.textContent?.trim() || null,
        invalid: Boolean(row.querySelector('[aria-invalid="true"]') ?? host.querySelector('[data-invalid="true"]')),
        rowText: row.innerText.replace(/\s+/g, " ").trim().slice(0, 300),
      };
    },
    null as NumberControlRead | null,
    authored,
  );
};

/** 🪡️ Types `text` into a slider's typed readout the way a person does (double-click the readout, type, Enter); wgpu types
 * into the mirrored control. Answers what the field held before Enter. */
export const typeSliderText = async (pointer: string, text: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  const authored = `framework.history.editor.input.${pointer}`;
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const node = nodes.find((other) => other.key.endsWith(`${authored}::editor`)) ?? mirrorFind(nodes, authored);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null, via: "absent" };
    await field.fill(text, { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await field.press("Enter", { timeout: 4000 }).catch(() => page.keyboard.press("Enter").catch(() => {}));
    return { present: true, typed: typed as string | null, via: `mirror ${node.role}` };
  }
  const token = markToken();
  const field = page.locator(`[data-probe-row="${token}"] [data-slot="slider-content"] input[type="number"]`).first();
  let attempts = 0;
  for (; attempts < 3; attempts++) {
    const readout = await evalSafe(
      (arg) => {
        const find = (suffix: string) => Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((el) => el.id === suffix || el.id.endsWith(`/${suffix}`) || el.id.endsWith(`␟${suffix}`));
        const row = find(`${arg.target}.row`) ?? find(arg.target)?.parentElement ?? null;
        for (const stale of Array.from(document.querySelectorAll("[data-probe-row], [data-probe-target]"))) if (stale.getAttribute("data-probe-row") === arg.token || stale.getAttribute("data-probe-target") === arg.token) {
          stale.removeAttribute("data-probe-row");
          stale.removeAttribute("data-probe-target");
        }
        const value = row?.querySelector<HTMLElement>('[data-slot="slider-value"]');
        value?.setAttribute("data-probe-target", arg.token);
        row?.setAttribute("data-probe-row", arg.token);
        return Boolean(value) || Boolean(row?.querySelector('[data-slot="slider-content"] input[type="number"]'));
      },
      false,
      { target: authored, token },
    );
    if (!readout) return { present: false, typed: null as string | null, via: "no-readout" };
    if (await field.count().catch(() => 0)) break;
    await evalSafe(
      (mark) => {
        const host = window as unknown as { __probeReadout?: { t: number; editing: boolean; focus: string }[] };
        const states: { t: number; editing: boolean; focus: string }[] = (host.__probeReadout = []);
        const start = performance.now();
        const sample = () => {
          const row = document.querySelector(`[data-probe-row="${mark}"]`);
          const active = document.activeElement as HTMLElement | null;
          let named: HTMLElement | null = active;
          while (named && !named.id) named = named.parentElement;
          const state = { t: Math.round(performance.now() - start), editing: Boolean(row?.querySelector('[data-slot="slider-content"] input[type="number"]')), focus: active ? `${active.tagName.toLowerCase()}[${active.getAttribute("data-slot") ?? active.getAttribute("role") ?? ""}]@${named?.id.slice(-70) ?? ""}` : "none" };
          const last = states[states.length - 1];
          if (!last || last.editing !== state.editing || last.focus !== state.focus) states.push(state);
          if (performance.now() - start < 2500) setTimeout(sample, 20);
        };
        sample();
        return true;
      },
      false,
      token,
    );
    await page.locator(`[data-probe-target="${token}"]`).first().dblclick({ timeout: 4000 }).catch(() => {});
    await sleep(700);
    if (await field.count().catch(() => 0)) break;
  }
  const opening = await evalSafe(() => ((window as unknown as { __probeReadout?: { t: number; editing: boolean; focus: string }[] }).__probeReadout ?? []).slice(0, 24), [] as { t: number; editing: boolean; focus: string }[]);
  if (!(await field.count().catch(() => 0))) return { present: true, typed: null as string | null, via: `readout did not stay open for typing after ${attempts} double-clicks`, opening };
  await field.fill(text, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  const bounds = await field.evaluate((el) => ({ min: el.getAttribute("min"), max: el.getAttribute("max") }), undefined, { timeout: 3000 }).catch(() => null);
  await field.press("Enter", { timeout: 4000 }).catch(() => {});
  const life = await evalSafe(
    async (mark) => {
      const states: { t: number; editing: boolean; invalid: boolean; alert: string | null }[] = [];
      const start = performance.now();
      while (performance.now() - start < 1000) {
        const row = document.querySelector(`[data-probe-row="${mark}"]`);
        const editor = row?.querySelector<HTMLInputElement>('[data-slot="slider-content"] input[type="number"]') ?? null;
        const state = { t: Math.round(performance.now() - start), editing: Boolean(editor), invalid: editor?.getAttribute("aria-invalid") === "true" || Boolean(row?.querySelector('[data-invalid="true"]')), alert: row ? (row.querySelector('[role="alert"]')?.textContent?.trim() ?? null) : "row-unmounted" };
        const last = states[states.length - 1];
        if (!last || last.editing !== state.editing || last.invalid !== state.invalid || last.alert !== state.alert) states.push(state);
        await new Promise((resolveTick) => setTimeout(resolveTick, 40));
      }
      return states;
    },
    [] as { t: number; editing: boolean; invalid: boolean; alert: string | null }[],
    token,
  );
  return { present: true, typed, via: attempts ? `readout (double-click ${attempts + 1})` : "readout", bounds, life, opening: opening.slice(0, 8) };
};

/** 🫳️ Presses "Use selection" of the editor's reference input at `pointer`: the first row of its windowed reference row,
 * the button `framework.history.editor.input.<pointer>.useSelection` (its row `….useSelection.row`), on both renderers. */
export const pressUseSelection = async (pointer: string) => {
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") await mirrorAwait(`framework.history.editor.input.${pointer}.useSelection`, 40000);
  const pressed = await pressAuthored(`framework.history.editor.input.${pointer}.useSelection`);
  return pressed.present && pressed.disabled !== true ? (pressed.id ?? "useSelection") : null;
};

type FocusRead = { key: string | null; tag: string; isBand: boolean; inBand: boolean; inDialog: boolean; inHistory: boolean };

/** 🧿️ Where keyboard focus is: React's active element (its own id, else the nearest ancestor id that names a framework or
 * dialog control), whether it IS the band region and whether it sits in the band, the prompt or the History panel; wgpu's
 * focused mirror node (the band is its live `shell.time-travel.status` node). */
export const focusRead = async (): Promise<FocusRead> => {
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => other.focused);
    const band = node?.key === "shell.time-travel.status";
    return { key: node?.key ?? null, tag: node?.tag ?? "", isBand: band, inBand: band || /^(ui\.timeTravel|shell\.time-travel)\./.test(node?.key ?? ""), inDialog: /^shell\.dialog\./.test(node?.key ?? ""), inHistory: /framework\.history/.test(node?.key ?? "") };
  }
  return evalSafe(
    () => {
      const active = document.activeElement as HTMLElement | null;
      let node: HTMLElement | null = active;
      while (node && !/framework\.|ui\.dialog/.test(node.id)) node = node.parentElement;
      return { key: node?.id ?? (active?.id || null), tag: active?.tagName.toLowerCase() ?? "", isBand: Boolean(active?.hasAttribute("data-semio-time-travel")), inBand: Boolean(active?.closest("[data-semio-time-travel]")), inDialog: Boolean(active?.closest('[role="dialog"]')), inHistory: Boolean(active?.closest('[id*="framework.panel.history"], [id*="framework.history"]')) };
    },
    { key: null, tag: "", isBand: false, inBand: false, inDialog: false, inHistory: false } as FocusRead,
  );
};

export const focusIsOnAnEditorInput = (focus: FocusRead) => Boolean(focus.key && /framework\.history\.editor\.input\./.test(focus.key) && !focus.key.endsWith(".row"));

/** 👀️ Whether the editor's first input is revealed WITHOUT the probe scrolling: React — inside the window viewport and the
 * top element at its centre (nothing covers it); wgpu — mirrored (the windowed body projects only what is on screen). */
export const editorRevealed = async () => {
  if (renderer === "wgpu") {
    const node = (await mirror()).find((other) => /framework\.history\.editor\.input\.[^.]+$/.test(other.key));
    return { revealed: Boolean(node), first: node?.key ?? null };
  }
  return evalSafe(
    () => {
      const input = Array.from(document.querySelectorAll<HTMLElement>('[id*="/framework.history.editor.input."]')).find((el) => !el.id.endsWith(".row"));
      if (!input) return { revealed: false, first: null as string | null };
      const rect = input.getBoundingClientRect();
      const x = rect.left + Math.min(rect.width / 2, 20);
      const y = rect.top + rect.height / 2;
      const inside = rect.width > 0 && rect.height > 0 && rect.top >= 0 && rect.bottom <= innerHeight && rect.left >= 0 && rect.right <= innerWidth;
      const top = inside ? document.elementFromPoint(x, y) : null;
      return { revealed: inside && Boolean(top && (input.contains(top) || top.contains(input))), first: input.id as string | null };
    },
    { revealed: false, first: null as string | null },
  );
};

/** 🖋️ Types `text` into the editor's text input at `pointer` (`input(Text)` commits on blur): fill, then Tab away — wgpu fills
 * the mirrored field and hands focus back to the canvas. Answers what the field held before it lost focus. */
export const typeEditorText = async (pointer: string, text: string) => {
  await revealHistory("framework.history.editor.inputs");
  await revealHistory(`framework.history.editor.input.${pointer}.row`);
  if (renderer === "wgpu") {
    const node = mirrorFind(await mirror(), `framework.history.editor.input.${pointer}`);
    const field = node ? await mirrorLocator(node.key, node.window) : null;
    if (!node || !field) return { present: false, typed: null as string | null };
    await field.fill(text, { force: true, timeout: 4000 }).catch(() => {});
    const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
    await prepareChord();
    return { present: true, typed };
  }
  const id = await resolveDomId(`framework.history.editor.input.${pointer}`);
  if (!id) return { present: false, typed: null as string | null };
  const host = page.locator(`[id="${id}"]`).first();
  const field = (await host.evaluate((el) => el.tagName === "INPUT" || el.tagName === "TEXTAREA").catch(() => false)) ? host : host.locator("input, textarea").first();
  await field.click({ timeout: 4000 }).catch(() => {});
  await field.fill(text, { timeout: 4000 }).catch(() => {});
  const typed = await field.inputValue({ timeout: 2000 }).catch(() => null);
  await field.press("Tab").catch(() => {});
  return { present: true, typed };
};
//#endregion 🔖️Band
