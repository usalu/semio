import { strict as assert } from "node:assert";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { Page } from "playwright";

type Rect = [number, number, number, number];
type Point = [number, number];
type Renderer = "react" | "wgpu";
type Control = { id: string; kind: string; rect: Rect; windowId?: string; pressed?: string };
type PhysicalInput = { seq: number; type: string; trusted: boolean; target: string; key?: string; point?: Point };
type Action = { seq: number; action: string; controller?: string; outcome?: string };
type State = { ready: string | null; error: string | null; controls: Control[]; surfaces: string[]; fullscreen: boolean; generation: number; selected: string[]; example: string | null; role: string | null };
type Step = { name: string; operation: string; key?: string; value?: string; children?: string[]; modifiers?: string[]; labels?: Record<"en" | "de", string[]>; delta?: Point; settleMs?: number };
type Definition = { schemaVersion: number; physicalInputTypes: string[]; viewport: { width: number; height: number; deviceScaleFactor: number }; geometryControls: string[]; sceneKind: string; command: { id: string; panel: string; children: string[] }; steps: Step[] };
type Detail = Record<string, unknown>;
type Side = { before: State; after: State; actions: Action[]; inputs: PhysicalInput[]; detail: Detail; error: string | null; screenshot: string; observed: boolean };
type Receipt = { name: string; renderers: Partial<Record<Renderer, Side>>; differences: string[] };
const operations = new Set(["boot", "click", "close-panel", "disclose", "disclose-open", "disclose-close", "select-open", "select-activate", "settings-close", "pane", "action-scroll", "split", "cap", "close-all", "reopen", "scene-drag", "scene-wheel", "scene-click", "chord", "palette-pointer", "palette-keyboard", "fullscreen", "example", "popup-dismiss"]);
const center = (r: Rect): Point => [r[0] + r[2] / 2, r[1] + r[3] / 2];
const match = (id: string, key: string): boolean => id === key || [".", "/", "::"].some(separator => id.endsWith(separator + key));
const contains = (r: Rect, at: Point): boolean => at[0] >= r[0] && at[0] <= r[0] + r[2] && at[1] >= r[1] && at[1] <= r[1] + r[3];
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const mod = process.platform === "darwin" ? "Meta" : "Control";
const fullChord = process.platform === "darwin" ? "Control+Meta+f" : "F11";

/** 🧬️ Admits the authored neutral journey before any browser is started. */
export function parseShellInteractionJourney(value: unknown): Definition {
  assert(value && typeof value === "object");
  const d = value as Definition;
  assert(Object.keys(d).every(k => ["schemaVersion", "physicalInputTypes", "viewport", "geometryControls", "sceneKind", "command", "steps"].includes(k)));
  assert.equal(d.schemaVersion, 1);
  assert.deepEqual(d.physicalInputTypes, ["pointerdown", "pointerup", "keydown", "keyup", "wheel"]);
  assert.equal(d.steps.length, 57);
  assert.equal(new Set(d.steps.map(s => s.name)).size, 57);
  assert.equal(d.viewport.deviceScaleFactor, 1);
  assert(Number.isInteger(d.viewport.width) && Number.isInteger(d.viewport.height) && d.viewport.width >= 320 && d.viewport.width <= 3840 && d.viewport.height >= 240 && d.viewport.height <= 2160);
  assert(Object.keys(d.viewport).every(k => ["width", "height", "deviceScaleFactor"].includes(k)));
  assert(Object.keys(d.command).every(k => ["id", "panel", "children"].includes(k)));
  const key = (v: unknown): boolean => typeof v === "string" && [...v].length > 0 && [...v].length <= 512;
  const keys = (v: unknown): boolean => Array.isArray(v) && v.length > 0 && v.length <= 64 && new Set(v).size === v.length && v.every(key);
  assert(keys(d.geometryControls) && key(d.sceneKind) && key(d.command.id) && key(d.command.panel) && keys(d.command.children));
  for (const s of d.steps) {
    assert(/^[a-z][a-z0-9-]{0,127}$/.test(s.name) && operations.has(s.operation));
    assert(Object.keys(s).every(k => ["name", "operation", "key", "value", "children", "modifiers", "labels", "delta", "settleMs"].includes(k)));
    if (s.key !== undefined) assert(key(s.key));
    if (s.value !== undefined) assert(key(s.value));
    if (s.children !== undefined) assert(keys(s.children));
    if (s.modifiers !== undefined) assert(Array.isArray(s.modifiers) && s.modifiers.length > 0 && s.modifiers.length <= 4 && new Set(s.modifiers).size === s.modifiers.length && s.modifiers.every(k => ["Alt", "Shift", "Control", "Meta"].includes(k)));
    if (s.labels !== undefined) assert(Object.keys(s.labels).length === 2 && keys(s.labels.en) && keys(s.labels.de));
    if (["click", "close-panel", "disclose", "disclose-open", "disclose-close", "select-open", "select-activate", "settings-close", "pane", "action-scroll", "cap", "chord", "popup-dismiss"].includes(s.operation)) assert(s.key && [...s.key].length <= 512);
    if (["disclose", "disclose-open", "disclose-close", "pane", "action-scroll"].includes(s.operation)) assert(s.children?.length && new Set(s.children).size === s.children.length);
    if (["select-open", "select-activate", "action-scroll", "scene-drag", "scene-click"].includes(s.operation)) assert(s.value);
    if (["scene-drag", "scene-wheel"].includes(s.operation)) assert(s.delta?.length === 2 && s.delta.every(v => Number.isFinite(v) && Math.abs(v) <= 2000));
    if (s.settleMs !== undefined) assert(Number.isInteger(s.settleMs) && s.settleMs >= 0 && s.settleMs <= 15000);
  }
  return d;
}

function bodies(state: State): Map<string, Rect> {
  return new Map(state.surfaces.filter(s => s.startsWith("window:")).flatMap(s => {
    const id = s.slice(7); const row = state.controls.filter(c => c.id === id).sort((a, b) => b.rect[2] * b.rect[3] - a.rect[2] * a.rect[3])[0];
    return row ? [[id, row.rect] as [string, Rect]] : [];
  }));
}
const tabs = (state: State): Control[] => state.controls.filter(c => /^mode-dock-tab-(?:root|[0-9])/.test(c.id) || /^dock\.tab\.[^.]*\.[^.]+$/.test(c.id));
function bodyUnion(rows: Map<string, Rect>): Rect {
  const r = [...rows.values()]; const x = Math.min(...r.map(r => r[0])); const y = Math.min(...r.map(r => r[1]));
  return [x, y, Math.max(...r.map(r => r[0] + r[2])) - x, Math.max(...r.map(r => r[1] + r[3])) - y];
}

class Driver {
  private restore?: Map<string, Rect>;
  private actionWindow?: string;
  readonly logs: string[] = [];
  constructor(readonly renderer: Renderer, private readonly page: Page, private readonly definition: Definition, private readonly locale: "en" | "de") {
    page.on("console", m => this.logs.push(`${m.type()} ${m.text()}`));
    page.on("pageerror", e => this.logs.push(`pageerror ${e.message}`));
  }
  async inputs(): Promise<PhysicalInput[]> {
    return this.page.evaluate(() => (window as unknown as { __semioPhysicalJourneyInputs?: PhysicalInput[] }).__semioPhysicalJourneyInputs ?? []);
  }
  async actions(): Promise<Action[]> {
    return this.page.evaluate(async renderer => {
      const host = window as unknown as { __semioInputLedger?: { recent: { inputSeq: number; action: string; controllerId?: string; outcome?: { kind: string } }[] }; semioWgpuIntrospection?: { dumpChrome(): Promise<string> } };
      if (renderer === "react") return (host.__semioInputLedger?.recent ?? []).map(r => ({ seq: r.inputSeq, action: r.action, controller: r.controllerId, outcome: r.outcome?.kind }));
      const chrome = JSON.parse(await host.semioWgpuIntrospection!.dumpChrome()) as { actions: { seq: number; action: string; controllerId: string }[] };
      return chrome.actions.map(r => ({ seq: r.seq, action: r.action, controller: r.controllerId, outcome: "dispatched" }));
    }, this.renderer);
  }
  async observe(): Promise<State> {
    return this.page.evaluate(async renderer => {
      const fullscreen = document.fullscreenElement !== null;
      const ready = document.documentElement.getAttribute("data-semio-os-ready"); const error = document.documentElement.getAttribute("data-semio-os-error");
      const host = window as unknown as { semioWgpuIntrospection?: { dumpChrome(): Promise<string> } };
      if (renderer === "wgpu") {
        const c = JSON.parse(await host.semioWgpuIntrospection!.dumpChrome()) as { generation: number; hits: { controlId: string; kind: string; rect: Rect; windowId?: string }[]; surfaces: { level: string; id: string }[] };
        return { ready, error, fullscreen, generation: c.generation, controls: c.hits.map(h => ({ id: h.controlId, kind: h.kind, rect: h.rect, windowId: h.windowId })), surfaces: [...new Set(c.surfaces.map(s => s.level === "dialog" ? "dialog" : `${s.level}:${s.id}`))].sort(), selected: [], example: null, role: null };
      }
      const counts = new Map<string, number>();
      const controls = Array.from(document.querySelectorAll<HTMLElement>('[id],button,[role="tab"],[role="option"],[data-slot]')).map(e => {
        const base = e.getAttribute("data-slot") ?? e.getAttribute("role") ?? e.tagName.toLowerCase(); const count = (counts.get(base) ?? 0) + 1; counts.set(base, count);
        const r = e.getBoundingClientRect();
        return { id: e.id || (count === 1 ? base : `${base}#${count}`), kind: base, rect: [r.x, r.y, r.width, r.height] as Rect, windowId: e.closest("[data-window-id]")?.getAttribute("data-window-id") ?? undefined, pressed: e.getAttribute("aria-pressed") ?? e.getAttribute("data-state") ?? undefined };
      }).filter(c => c.rect[2] > 0 && c.rect[3] > 0);
      const surfaces = [...Array.from(document.querySelectorAll("[data-window-id]")).map(e => `window:${e.getAttribute("data-window-id")}`), ...Array.from(document.querySelectorAll('[data-level="panel"]')).filter(e => e.id.startsWith("framework.panelTab.")).map(e => `panel:${e.id.slice(19)}`), ...(document.querySelector('[data-level="dialog"]') ? ["dialog"] : [])];
      return { ready, error, fullscreen, generation: 0, controls, surfaces: [...new Set(surfaces)].sort(), selected: Array.from(document.querySelectorAll('[aria-selected="true"]')).map(e => e.id || e.getAttribute("data-tab-id") || e.textContent || ""), example: document.querySelector('[data-slot="select-value"]')?.textContent?.trim() ?? null, role: document.querySelector("[data-role]")?.getAttribute("data-role") ?? null };
    }, this.renderer);
  }
  async wait(predicate: (state: State) => unknown, reason: string, timeout = 15000): Promise<State> {
    const deadline = Date.now() + timeout;
    do { const state = await this.observe(); if (state.error) throw new Error(state.error); if (await predicate(state)) return state; await this.page.waitForTimeout(150); } while (Date.now() < deadline);
    throw new Error(reason);
  }
  async boot(): Promise<Detail> {
    await this.page.waitForFunction(() => document.querySelector("[data-semio-os-ready],[data-semio-os-error]") !== null, null, { timeout: 240000 });
    let prior = ""; let quiet = 0;
    for (let tick = 0; tick < 240; tick++) {
      const state = await this.observe(); const actions = await this.actions(); const signature = JSON.stringify([state.controls, state.surfaces, actions.at(-1)?.seq]);
      quiet = signature === prior ? quiet + 1 : 0; prior = signature;
      if (state.error) throw new Error(state.error);
      if (tick >= 10 && quiet >= 3 && state.controls.length > 4) {
        if (this.renderer === "wgpu") await this.page.locator("canvas").first().focus();
        return { booted: true, ready: state.ready, quietTicks: quiet, controls: state.controls.length };
      }
      await this.page.waitForTimeout(1000);
    }
    throw new Error("Accepted chrome and action journal did not become quiet");
  }
  async resolve(key: string): Promise<{ row: Control; resolved: string }> {
    const state = await this.observe(); const rows = key === "framework.settings" && this.renderer === "wgpu" ? state.controls.filter(c => c.id !== key || c.kind === "Toggle") : state.controls;
    const exact = rows.find(c => c.id === key); const suffix = rows.find(c => c.id.endsWith("." + key)); const included = rows.find(c => c.id.includes(key));
    const row = exact ?? suffix ?? included; assert(row, `No physical target ${key}`);
    return { row, resolved: exact ? "exact" : suffix ? "suffix" : "contains" };
  }
  async hit(row: Control, rect = row.rect): Promise<void> {
    if (this.renderer !== "react") return;
    const obstruction = await this.page.evaluate(({ id, at }) => {
      const hit = document.elementFromPoint(at[0], at[1]); const exact = Array.from(document.querySelectorAll("[id]")).filter(e => e.id === id); const slot = id.replace(/#\d+$/, "");
      const candidates = exact.length ? exact : Array.from(document.querySelectorAll("[data-slot]")).filter(e => e.getAttribute("data-slot") === slot);
      return candidates.some(e => e === hit || e.contains(hit)) ? null : hit?.closest("[id]")?.id || hit?.tagName || "outside viewport";
    }, { id: row.id, at: center(rect) });
    assert.equal(obstruction, null, `Physical target ${row.id} covered by ${obstruction}`);
  }
  async click(key: string): Promise<Detail> { const { row, resolved } = await this.resolve(key); await this.hit(row); await this.page.mouse.click(...center(row.rect)); return { resolved, id: row.id, rect: row.rect, clicked: true }; }
  async chord(key: string): Promise<void> { await this.page.keyboard.press(key.replace(/^mod/, mod)); }
  async drag(from: Point, to: Point, button: "left" | "middle" | "right" = "left", hold = 0, modifiers: string[] = []): Promise<void> {
    for (const key of modifiers) await this.page.keyboard.down(key);
    try {
      await this.page.mouse.move(...from); await this.page.mouse.down({ button }); if (hold) await this.page.waitForTimeout(hold);
      for (let n = 1; n <= 8; n++) await this.page.mouse.move(from[0] + (to[0] - from[0]) * n / 8, from[1] + (to[1] - from[1]) * n / 8);
    } finally { await this.page.mouse.up({ button }); for (const key of modifiers.toReversed()) await this.page.keyboard.up(key); }
  }
  async cap(cap: string): Promise<Detail> {
    const tab = tabs(await this.observe())[0]; assert(tab, "No dock tab"); await this.page.mouse.move(...center(tab.rect)); await this.page.waitForTimeout(400);
    const before = await this.observe(); const id = tab.windowId ?? tab.id.match(/^dock\.tab\.[^.]*\.(.+)$/)?.[1]; assert(id, "Dock tab lacks concrete owner");
    const control = before.controls.find(c => new RegExp(`(^|[.\\-])${cap}$`, "i").test(c.id) && (c.windowId === id || c.id === `${tab.id}.${cap}`)); assert(control, `No ${cap} cap for ${id}`);
    const previous = bodies(before); await this.hit(control); await this.page.mouse.click(...center(control.rect));
    let after: State;
    if (cap === "close") { after = await this.wait(s => !s.surfaces.includes(`window:${id}`) && !tabs(s).some(t => t.windowId === id || t.id === tab.id), "Closed window retained body/tab"); this.restore = undefined; }
    else if (this.restore) { const restore = this.restore; after = await this.wait(s => bodies(s).size === restore.size && [...restore].every(([id, r]) => bodies(s).get(id)?.every((v, i) => Math.abs(v - r[i]!) <= 3)), "Unfocus failed to restore concrete bodies"); this.restore = undefined; }
    else { assert(previous.size >= 2, "Focus needs two visible bodies"); const union = bodyUnion(previous); after = await this.wait(s => bodies(s).size === 1 && bodies(s).get(id)?.every((v, i) => Math.abs(v - union[i]!) <= 3), "Focus failed to expand owner and conceal siblings"); this.restore = previous; }
    return { resolved: "contains", cap, windowId: id, beforeBodies: Object.fromEntries(previous), afterBodies: Object.fromEntries(bodies(after)), physicalOutcome: true };
  }
  async pane(step: Step): Promise<Detail> {
    const before = await this.observe(); const row = before.controls.find(c => c.id.endsWith("." + step.key)); assert(row, `Missing pane ${step.key}`);
    const body = [...bodies(before)].find(([, r]) => contains(r, center(row.rect))); assert(body, "Pane has no concrete visible window");
    const children = (s: State): string[] => step.children!.filter(k => s.controls.some(c => match(c.id, k) && contains(bodies(s).get(body[0])!, center(c.rect))));
    const prior = children(before); assert(prior.length === 0 || prior.length === step.children!.length, "Partial pane children"); const opening = prior.length === 0;
    await this.hit(row); await this.page.mouse.click(...center(row.rect)); const after = await this.wait(s => children(s).length === (opening ? step.children!.length : 0), "Pane children did not change");
    if (opening && step.key === "engagement.toggle") this.actionWindow = body[0];
    return { resolved: "suffix", windowId: body[0], opening, required: step.children, visibleChildren: children(after), physicalOutcome: true };
  }
  async actionScroll(step: Step): Promise<Detail> {
    assert(this.actionWindow, "Actions needs preceding pane owner");
    const visible = (s: State, k: string): Control | undefined => { const r = bodies(s).get(this.actionWindow!); return r ? s.controls.find(c => match(c.id, k) && contains(r, center(c.rect))) : undefined; };
    const before = await this.observe(); const first = visible(before, step.children![0]!); const second = visible(before, step.children![1]!); assert(first && second, "Missing initial action rows");
    const pitch = second.rect[1] - first.rect[1]; assert([first.rect[3], second.rect[3], pitch].every(v => Math.abs(v - 24) <= 1), "Action row pitch differs from24px"); assert(!visible(before, step.key!), "Target initially onscreen");
    await this.page.mouse.move(...center(second.rect)); await this.page.mouse.wheel(0, 720); const shown = await this.wait(s => visible(s, step.key!), "Scroll did not reveal authored target"); const target = visible(shown, step.key!)!;
    const cursor = (await this.actions()).at(-1)?.seq ?? 0; await this.hit(target); await this.page.mouse.click(...center(target.rect));
    await this.wait(async () => (await this.actions()).some(a => a.seq > cursor && a.action === step.value), "Revealed row did not dispatch its own action");
    await this.page.mouse.move(...center(target.rect)); await this.page.mouse.wheel(0, -720); await this.wait(s => { const row = visible(s, step.children![0]!); return row && Math.abs(row.rect[1] - first.rect[1]) <= 1 && !visible(s, step.key!); }, "Scroll failed to restore clipped rows");
    return { windowId: this.actionWindow, targetKey: step.key, dispatched: true, restored: true, physicalOutcome: true };
  }
  async select(step: Step, activate: boolean): Promise<Detail> {
    if (!activate) await this.click(step.key!);
    const option = async (): Promise<Rect | null> => this.renderer === "wgpu" ? (await this.observe()).controls.find(c => c.id === step.value && c.kind === "Button")?.rect ?? null : this.page.evaluate(value => { const e = Array.from(document.querySelectorAll('[role="option"][data-value]')).find(e => e.getAttribute("data-value") === value); if (!e) return null; const r = e.getBoundingClientRect(); return [r.x, r.y, r.width, r.height] as Rect; }, step.value!);
    await this.wait(async () => await option(), "Select did not publish requested option"); const rect = await option(); assert(rect);
    if (!activate) return { resolved: "exact", controlKey: step.key, optionValue: step.value, popupOpen: true, optionRect: rect };
    await this.page.mouse.click(...center(rect)); await this.wait(async () => !(await option()), "Select option remained visible");
    const field = step.key === "framework.settings.appearance" ? "appearance" : "locale";
    const applied = await this.page.waitForFunction(({ renderer, key, value, field }) => {
      if (renderer === "react") { const root = Array.from(document.querySelectorAll<HTMLElement>('.semio-scope[data-shell-id]')).find(e => e.getBoundingClientRect().width > 0); return (field === "appearance" ? root?.dataset.uiAppearance : root?.lang || document.documentElement.lang) === value; }
      return Array.from(document.querySelectorAll('[role="combobox"][data-node-key]')).some(e => matchKey(e.getAttribute("data-node-key"), key) && e.getAttribute("aria-valuetext") === value);
      function matchKey(candidate: string | null, key: string): boolean { return candidate === key || candidate?.endsWith("/" + key) === true; }
    }, { renderer: this.renderer, key: step.key!, value: step.value!, field }, { timeout: 15000 }); await applied.dispose();
    return { resolved: "exact", controlKey: step.key, optionValue: step.value, popupOpen: false, appliedSettings: { [field]: step.value }, physicalOutcome: true };
  }
  async split(): Promise<Detail> {
    const before = await this.observe(); const gutter = before.controls.find(c => /resizable-handle|separator|PanelResize|DockSplit/i.test(c.id + " " + c.kind)); assert(gutter, "No split gutter");
    const from = center(gutter.rect); const axis = gutter.rect[3] > gutter.rect[2] ? 0 : 1; const to: Point = [...from]; to[axis] += 120; const previous = bodies(before); assert(previous.size >= 2);
    await this.drag(from, to); const after = await this.wait(s => { const r = s.controls.find(c => c.kind === gutter.kind && Math.abs(center(c.rect)[axis] - to[axis]) <= 4 && Math.abs(center(c.rect)[1 - axis]! - from[1 - axis]!) <= 4); const now = bodies(s); return r && now.size === previous.size && [...previous].every(([id]) => now.has(id)) && [...previous].some(([id, r]) => Math.abs(now.get(id)![axis + 2]! - r[axis + 2]!) >= 100); }, "Divider did not move120px and resize live siblings");
    return { resolved: "contains", beforeBodies: Object.fromEntries(previous), afterBodies: Object.fromEntries(bodies(after)), physicalOutcome: true };
  }
  async reopen(): Promise<Detail> {
    let state = await this.observe(); const count = tabs(state).length; const previous = new Set(state.surfaces);
    const template = (s: State): Control | undefined => s.controls.find(c => c.id.includes("framework.display.windows.") && c.id.endsWith(".kind") && !c.id.startsWith("tree.drag."));
    const section = (s: State): Control | undefined => s.controls.find(c => /(?:^|[./])framework\.display\.windows\.[^.]+$/.test(c.id) && !/\.(?:panel|empty|scroll)$/.test(c.id));
    if (!template(state)) { await this.click("framework.category.display"); state = await this.wait(s => template(s) || section(s), "Display lacks template section"); }
    if (!template(state)) { const row = section(state)!; await this.page.mouse.click(...center(row.rect)); state = await this.wait(s => template(s), "Disclosure lacks window template"); }
    const row = template(state)!; const handleId = "tree.drag.transfer." + row.id.replace(/^tree\.label\./, "");
    const handle = this.renderer === "wgpu" ? state.controls.find(c => c.id === handleId)?.rect : await this.page.evaluate(id => { const e = document.getElementById(id)?.querySelector('[data-slot="drag-handle"]'); if (!e) return null; const r = e.getBoundingClientRect(); return [r.x, r.y, r.width, r.height] as Rect; }, row.id);
    assert(handle && handle[2] > 0, "Window template lacks transfer handle"); await this.hit(row, handle); await this.drag(center(handle), [this.definition.viewport.width / 2, this.definition.viewport.height / 2], "left", 650);
    state = await this.wait(s => tabs(s).length === count + 1, "Transfer did not create exactly one window"); const id = state.surfaces.find(s => s.startsWith("window:") && !previous.has(s))?.slice(7); assert(id);
    await this.wait(s => s.controls.some(c => this.renderer === "wgpu" ? c.kind === this.definition.sceneKind && c.windowId === id : c.id === id && c.kind === "window"), "Created tab lacks live scene body");
    if (this.renderer === "react") await this.page.waitForFunction(id => Array.from(document.querySelectorAll("[data-viewport-camera-json]")).some(e => e.getAttribute("data-window-instance-id") === id && e.querySelector("canvas")), id);
    return { resolved: "exact", windowId: id, beforeTabs: count, afterTabs: tabs(state).length, liveSceneBody: true, physicalOutcome: true };
  }
  async scenes(): Promise<{ surfaceId: string; windowId: string; camera: unknown; selected: unknown }[]> {
    return this.page.evaluate(async renderer => {
      if (renderer === "wgpu") {
        const host = window as unknown as { semioWgpuIntrospection: { dumpMeshStats(): Promise<string>; dumpChrome(): Promise<string> } };
        const [meshJson, chromeJson] = await Promise.all([host.semioWgpuIntrospection.dumpMeshStats(), host.semioWgpuIntrospection.dumpChrome()]);
        const value = JSON.parse(meshJson) as { surfaces: { surfaceId: string; rect: Rect; camera: unknown; liveCamera: unknown; selected: unknown }[] };
        const chrome = JSON.parse(chromeJson) as { hits: { kind: string; windowId?: string; rect: Rect }[] };
        return value.surfaces.map(s => {
          const owner = chrome.hits.find(h => h.kind === "World3d" && h.windowId && h.rect.every((v, i) => Math.abs(v - s.rect[i]!) <= 1));
          if (!owner?.windowId) throw new Error("Public scene camera lacks accepted physical window owner");
          return { surfaceId: s.surfaceId, windowId: owner.windowId, camera: s.liveCamera ?? s.camera, selected: s.selected };
        });
      }
      return Array.from(document.querySelectorAll<HTMLElement>("[data-viewport-camera-json][data-selection-json][data-window-instance-id]")).filter(e => e.getBoundingClientRect().width > 0).map(e => ({ surfaceId: e.dataset.surfaceId ?? "", windowId: e.dataset.windowInstanceId!, camera: JSON.parse(e.dataset.viewportCameraJson ?? "null"), selected: JSON.parse(e.dataset.selectionJson ?? "null") }));
    }, this.renderer);
  }
  async scenePoint(): Promise<Point> {
    if (this.renderer === "wgpu") { const s = await this.observe(); const row = s.controls.filter(c => c.kind === this.definition.sceneKind && s.surfaces.includes("window:" + c.windowId)).sort((a, b) => b.rect[2] * b.rect[3] - a.rect[2] * a.rect[3])[0]; assert(row, "No live accepted scene"); return [row.rect[0] + row.rect[2] * .7, row.rect[1] + row.rect[3] * .45]; }
    return this.page.evaluate(() => { const e = Array.from(document.querySelectorAll('[data-slot="pane-host"],[data-slot="window-body"]')).sort((a, b) => b.getBoundingClientRect().width * b.getBoundingClientRect().height - a.getBoundingClientRect().width * a.getBoundingClientRect().height)[0]; if (!e) throw new Error("No live scene body"); const r = e.getBoundingClientRect(); return [r.x + r.width * .7, r.y + r.height * .45] as Point; });
  }
  async palette(keyboard: boolean): Promise<Detail> {
    const selector = '[id="ui.search.input"],[data-node-key="ui.search.input"]';
    const focus = async (query?: string): Promise<void> => { await this.page.waitForFunction(({ selector, query }) => { const input = document.querySelector(selector); return input instanceof HTMLInputElement && document.activeElement === input && (query === undefined || input.value === query); }, { selector, query }); };
    await this.chord("mod+p"); await this.wait(s => s.surfaces.includes("dialog") && s.controls.some(c => c.id === "ui.search.input"), "Palette lacks accepted dialog/input"); await focus("");
    const selection = async (): Promise<string | null> => this.page.evaluate(selector => { const input = document.querySelector(selector); const list = document.getElementById(input?.getAttribute("aria-controls") ?? ""); const id = input?.getAttribute("aria-activedescendant"); const active = id ? document.getElementById(id) : null; return input?.getAttribute("role") === "combobox" && list?.getAttribute("role") === "listbox" && list.contains(active) && active?.getAttribute("aria-selected") === "true" ? id ?? null : null; }, selector);
    let keyboardNavigation: Detail | undefined;
    if (keyboard) { const first = await selection(); assert(first); await this.chord("ArrowDown"); await this.wait(async () => (await selection()) !== first, "ArrowDown did not advance selection"); const next = await selection(); assert(next && next !== first); await this.chord("ArrowUp"); await this.wait(async () => (await selection()) === first, "ArrowUp failed to restore selection"); keyboardNavigation = { first, next, restored: true }; }
    const label = await this.page.evaluate(id => { const e = Array.from(document.querySelectorAll('[role="option"][data-command-item-id]')).find(e => e.getAttribute("data-command-item-id") === id); return e?.getAttribute("aria-label") ?? e?.textContent?.trim() ?? null; }, this.definition.command.id); assert(label?.endsWith("…"), "Command lacks localized staged label"); const query = label.slice(0, -1); await this.page.keyboard.type(query);
    const filtered = async (): Promise<Control> => {
      const result = await this.page.evaluate(async ({ id, renderer }) => {
        const rows = Array.from(document.querySelectorAll('[role="option"][data-command-item-id]')).filter(e => e.getAttribute("data-command-item-id") === id); if (rows.length !== 1) return null; const row = rows[0]!;
        if (renderer === "react") { const r = row.getBoundingClientRect(); return { id: row.id, kind: "option", rect: [r.x, r.y, r.width, r.height] as Rect }; }
        const host = window as unknown as { semioWgpuIntrospection: { dumpAccessibility(): Promise<string>; dumpChrome(): Promise<string> } }; const ax = JSON.parse(await host.semioWgpuIntrospection.dumpAccessibility()) as { windows: { nodes: { key: string; rect: Rect }[] }[] }; const chrome = JSON.parse(await host.semioWgpuIntrospection.dumpChrome()) as { hits: { controlId: string; rect: Rect }[] }; const node = ax.windows.flatMap(w => w.nodes).find(n => n.key === row.getAttribute("data-node-key")); const hit = chrome.hits.find(h => h.controlId.startsWith("ui.search.item.") && node?.rect.every((v, i) => Math.abs(v - h.rect[i]!) < .01)); return hit ? { id: hit.controlId, kind: "option", rect: hit.rect } : null;
      }, { id: this.definition.command.id, renderer: this.renderer }); assert(result && result.rect[2] > 0, "Filtered canonical command lacks real physical hit"); return result;
    };
    await this.page.waitForFunction(({ selector, query, label }) => { const e = document.querySelector(selector); const rows = Array.from(document.querySelectorAll('[role="option"][data-command-item-id]')); return e instanceof HTMLInputElement && e.value === query && document.activeElement === e && rows.length === 1 && (rows[0]!.getAttribute("aria-label") ?? rows[0]!.textContent?.trim()) === label; }, { selector, query, label });
    const reopen = async (): Promise<void> => { await this.chord("mod+p"); await focus(query); await filtered(); };
    let pointerDismissals: Detail | undefined;
    if (keyboard) { await this.chord("Escape"); await this.wait(s => !s.surfaces.includes("dialog") && !s.controls.some(c => c.id === "ui.search.input"), "Escape did not dismiss palette"); await reopen(); }
    else {
      const state = await this.observe(); const dialog = state.controls.find(c => c.id === "ui.search.dialog" || c.kind === "dialog-content"); assert(dialog); const interior: Rect = [dialog.rect[0] + 5, dialog.rect[1] + dialog.rect[3] * .5, 0, 0]; await this.hit(dialog, interior); await this.page.mouse.click(...center(interior)); await this.page.waitForTimeout(400); assert((await this.observe()).surfaces.includes("dialog"), "Interior dismissed palette");
      const row = await filtered(); const input = (await this.resolve("ui.search.input")).row; await this.page.mouse.move(...center(row.rect)); await this.page.mouse.down(); await this.page.waitForTimeout(200); await this.page.mouse.move(...center(input.rect)); await this.page.mouse.up(); await this.page.waitForTimeout(400); assert((await this.observe()).surfaces.includes("dialog"), "Drag release activated different target");
      const close = (await this.observe()).controls.find(c => c.id === "ui.search.close" || c.kind === "dialog-close"); assert(close); await this.click(close.id); await this.wait(s => !s.surfaces.includes("dialog"), "Close did not dismiss palette"); await reopen(); await this.page.mouse.click(4, 4); await this.wait(s => !s.surfaces.includes("dialog"), "Outside did not dismiss palette"); await reopen(); pointerDismissals = { interiorRetained: true, dragReleaseCancelled: true, closePreservedQuery: true, outsidePreservedQuery: true };
    }
    const target = await filtered(); if (keyboard) await this.chord("Enter"); else { await this.hit(target); await this.page.mouse.click(...center(target.rect)); }
    const command = this.definition.command; await this.wait(s => !s.surfaces.includes("dialog") && !s.controls.some(c => c.id === "ui.search.input" || c.id.startsWith("ui.search.item.")) && s.surfaces.includes("panel:" + command.panel) && command.children.every(k => s.controls.some(c => match(c.id, k))), "Command failed to retire palette and publish Execute/Reset form");
    return { activated: { commandId: command.id, label }, keyboardNavigation, pointerDismissals, dialogRetired: true, panel: command.panel, physicalOutcome: true };
  }
  async run(step: Step): Promise<Detail> {
    if (step.operation === "boot") return { booted: true };
    if (step.operation === "click") return this.click(step.key!);
    if (step.operation === "close-panel") { if (!(await this.observe()).surfaces.includes("panel:" + step.key)) return { alreadyClosed: true, physicalOutcome: true }; const d = await this.click(step.key!); await this.wait(s => !s.surfaces.includes("panel:" + step.key), "Panel stayed open"); return { ...d, physicalOutcome: true }; }
    if (step.operation.startsWith("disclose")) { const has = (s: State, k: string): boolean => s.controls.some(c => match(c.id, k)); const before = await this.observe(); const closing = step.operation === "disclose-close"; if (step.operation === "disclose-open") assert(step.children!.every(k => !has(before, k)), "Closed disclosure exposes child hits"); await this.click(step.key!); await this.wait(s => step.children!.every(k => has(s, k) !== closing), "Disclosure failed child publication"); if (step.labels) for (const label of step.labels[this.locale]) await this.page.getByRole("treeitem", { name: label, exact: true }).first().waitFor({ state: "attached" }); return { children: step.children, visible: !closing, physicalOutcome: true }; }
    if (step.operation === "settings-close") { await this.click(step.key!); await this.wait(s => !s.surfaces.some(s => s.startsWith("panel:framework.settings.") || this.definition.steps.some(t => t.operation === "disclose" && s === "panel:" + t.key)), "Settings panel remained visible"); return { physicalOutcome: true }; }
    if (step.operation === "select-open" || step.operation === "select-activate") return this.select(step, step.operation === "select-activate");
    if (step.operation === "pane") return this.pane(step);
    if (step.operation === "action-scroll") return this.actionScroll(step);
    if (step.operation === "split") return this.split();
    if (step.operation === "cap") return this.cap(step.key!);
    if (step.operation === "close-all") { let count = 0; for (; count < 16 && tabs(await this.observe()).length; count++) await this.cap("close"); assert.equal(tabs(await this.observe()).length, 0); return { closed: count, remaining: 0, physicalOutcome: true }; }
    if (step.operation === "reopen") return this.reopen();
    if (step.operation.startsWith("scene-")) { const beforeScene = await this.scenes(); assert(beforeScene.length && beforeScene.every(s => s.camera), "Live scene lacks public camera state"); const at = await this.scenePoint(); if (step.operation === "scene-drag") await this.drag(at, [at[0] + step.delta![0], at[1] + step.delta![1]], step.value as "left" | "middle" | "right", 0, step.modifiers); else if (step.operation === "scene-wheel") { await this.page.mouse.move(...at); await this.page.mouse.wheel(...step.delta!); } else await this.page.mouse.click(...at, { button: step.value as "left" | "right" }); await this.page.waitForTimeout(300); const afterScene = await this.scenes(); const cameraChanged = !same(beforeScene.map(s => s.camera), afterScene.map(s => s.camera)); if (step.operation !== "scene-click") assert(cameraChanged, "Physical camera gesture did not change published live viewport"); return { at, gesture: step.operation, beforeScene, afterScene, cameraChanged }; }
    if (step.operation === "popup-dismiss") {
      const popup = (s: State): boolean => s.controls.some(c => this.renderer === "wgpu" ? /DropdownItem/.test(c.kind) : c.kind === "select-item" || c.kind === "option");
      if (!popup(await this.observe())) { await this.click(step.key!); await this.wait(popup, "Picker did not reopen for actual dismissal"); }
      const before = await this.observe(); await this.chord("Escape"); const after = await this.wait(s => !popup(s), "Escape retained picker options");
      assert(same(before.surfaces, after.surfaces) && before.example === after.example, "Picker dismissal changed app or selected example");
      return { popupRetired: true, selectionRetained: true, physicalOutcome: true };
    }
    if (step.operation === "chord") { await this.chord(step.key!); return { chord: step.key }; }
    if (step.operation === "fullscreen") { const before = (await this.observe()).fullscreen; await this.chord(fullChord); await this.wait(s => s.fullscreen !== before, "Fullscreen physical shortcut did not change public state"); return { entered: !before, physicalOutcome: true }; }
    if (step.operation.startsWith("palette-")) return this.palette(step.operation === "palette-keyboard");
    if (step.operation === "example") { const before = await this.observe(); const options = this.renderer === "wgpu" ? before.controls.filter(c => /DropdownItem|ContextMenu/.test(c.kind)).map(c => ({ text: c.id, rect: c.rect })) : await this.page.evaluate(() => Array.from(document.querySelectorAll('[role="option"],[data-slot="select-item"]')).map(e => { const r = e.getBoundingClientRect(); return { text: e.textContent?.trim() ?? "", rect: [r.x, r.y, r.width, r.height] as Rect }; }).filter(o => o.rect[2] > 0 && o.rect[3] > 0)); const target = options.find(o => o.text !== before.example && !/^no /i.test(o.text)); assert(target, "Example picker lacks alternate entry"); await this.page.mouse.click(...center(target.rect)); return { picked: target.text }; }
    throw new Error("Unimplemented authored operation " + step.operation);
  }
  async settle(before: State, cursor: number, minimum: number): Promise<void> {
    const started = Date.now(); let signature = ""; let quietSince = started;
    do { await this.page.waitForTimeout(250); const state = await this.observe(); const journal = await this.actions(); const current = JSON.stringify([state.controls, state.surfaces, state.fullscreen, journal.at(-1)?.seq]); if (current !== signature) { signature = current; quietSince = Date.now(); } if (Date.now() - started >= minimum && Date.now() - quietSince >= 1000 && (this.renderer === "react" || state.generation > before.generation) && (journal.some(a => a.seq > cursor) || !same(before.controls, state.controls) || !same(before.surfaces, state.surfaces) || before.fullscreen !== state.fullscreen)) return; } while (Date.now() - started < 15000);
  }
  async screenshot(path: string): Promise<void> { await this.page.screenshot({ path, fullPage: true }); }
}

/** ⚖️ Compares actual paired journals, surface transitions and physical outcomes. */
export function shellInteractionDifferences(receipt: Receipt, definition: Definition): string[] {
  const a = receipt.renderers.react; const b = receipt.renderers.wgpu; if (!a || !b) return ["unpaired baseline"];
  const differences: string[] = [];
  if (a.error || b.error) differences.push(`runtime error react=${a.error} wgpu=${b.error}`);
  const verbs = (s: Side): string[] => [...new Set(s.actions.map(a => a.action))].sort(); if (!same(verbs(a), verbs(b))) differences.push("dispatched action sets differ");
  const delta = (s: Side): unknown => [s.after.surfaces.filter(k => !s.before.surfaces.includes(k)), s.before.surfaces.filter(k => !s.after.surfaces.includes(k)), s.before.fullscreen !== s.after.fullscreen]; if (!same(delta(a), delta(b))) differences.push("accepted surface/fullscreen transitions differ");
  for (const side of [a, b]) if (!side.inputs || side.inputs.some(i => i.trusted !== true) || (receipt.name !== "boot" && !side.detail.alreadyClosed && side.inputs.length === 0)) differences.push("missing trusted physical input");
  if (!a.observed || !b.observed) differences.push("missing published consequence");
  for (const key of ["cap", "opening", "required", "visibleChildren", "targetKey", "dispatched", "restored", "physicalOutcome", "closed", "remaining", "beforeTabs", "afterTabs", "liveSceneBody", "activated", "appliedSettings", "pointerDismissals", "popupRetired", "selectionRetained", "cameraChanged"]) if (!same(a.detail[key], b.detail[key])) differences.push(`${key} differs`);
  if (receipt.name !== "window-reopen" && !same(a.detail.windowId, b.detail.windowId)) differences.push("physical target owner differs");
  const rect = (label: string, x?: Rect, y?: Rect): void => { if (!x || !y || x.some((v, i) => Math.abs(v - y[i]!) > 1)) differences.push(`${label} exceeds1 CSS pixel or is missing`); };
  if (["boot", "dismiss-tour"].includes(receipt.name)) for (const key of definition.geometryControls) { const x = a.after.controls.find(c => c.id === key)?.rect; const y = b.after.controls.find(c => c.id === key)?.rect; if (key === "s-presence-peers" && (!x || !y)) continue; if (x || y) rect(key, x, y); }
  for (const phase of ["beforeBodies", "afterBodies"]) { const x = a.detail[phase] as Record<string, Rect> | undefined; const y = b.detail[phase] as Record<string, Rect> | undefined; for (const key of new Set([...Object.keys(x ?? {}), ...Object.keys(y ?? {})])) rect(`${phase}/${key}`, x?.[key], y?.[key]); }
  return differences;
}

/** 🚶️ Runs all57 authored physical steps against fresh public renderer state. */
export async function runShellInteractionJourney(segments: readonly string[]): Promise<void> {
  const options = new Map<string, string>(); for (let i = 0; i < segments.length; i += 2) { const key = segments[i]!; const value = segments[i + 1]; assert(["--react-serve", "--wgpu-serve", "--renderer", "--locale", "--output"].includes(key) && value && !options.has(key), "parity journey requires unique named option/value pairs"); options.set(key, value); }
  const renderer = options.get("--renderer"); const locale = options.get("--locale"); const output = options.get("--output"); assert(["react", "wgpu", "paired"].includes(renderer!) && ["en", "de"].includes(locale!) && output, "parity journey requires --renderer react|wgpu|paired --locale en|de --output directory and corresponding live serve URLs");
  const targets: Renderer[] = renderer === "paired" ? ["react", "wgpu"] : [renderer as Renderer]; for (const r of targets) assert(options.get(`--${r}-serve`), `Missing ${r} serve URL`);
  const authored = (await import("../../⚖️parity/🧫️fixtures/🚶️shell-interaction/🔣️.json")).default;
  const definition = parseShellInteractionJourney(authored); const out = resolve(output); mkdirSync(out, { recursive: true }); const { chromium } = await import("playwright"); const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] }); const context = await browser.newContext({ viewport: definition.viewport, deviceScaleFactor: 1, locale: locale === "de" ? "de-DE" : "en-US" }); const drivers = new Map<Renderer, Driver>(); const receipts: Receipt[] = [];
  const flush = (): void => { writeFileSync(resolve(out, "steps.json"), JSON.stringify({ schemaVersion: 1, renderer, locale, viewport: definition.viewport, receipts }, null, 2)); writeFileSync(resolve(out, "parity.md"), ["# Actual Shell Interaction Journey", "", `Mode ${renderer}; explicit locale ${locale}; ${definition.viewport.width}×${definition.viewport.height}, DPR1.`, "", "| Step | React | WGPU | Paired Result |", "| --- | --- | --- | --- |", ...receipts.map(r => `| ${r.name} | ${r.renderers.react?.error ?? (r.renderers.react ? "observed" : "absent")} | ${r.renderers.wgpu?.error ?? (r.renderers.wgpu ? "observed" : "absent")} | ${r.differences.join("; ") || "match"} |`), ""].join("\n")); for (const [r, d] of drivers) writeFileSync(resolve(out, `${r}-console.log`), d.logs.join("\n") + "\n"); };
  try {
    for (const r of targets) { const page = await context.newPage(); const d = new Driver(r, page, definition, locale as "en" | "de"); drivers.set(r, d); await page.addInitScript(types => {
        localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
        const host = window as unknown as { __semioPhysicalJourneyInputs: PhysicalInput[] }; host.__semioPhysicalJourneyInputs = []; let seq = 0;
        for (const type of types) window.addEventListener(type, event => {
          const e = event as KeyboardEvent & PointerEvent; const target = e.target instanceof Element ? e.target.closest("[id],[data-node-key]") : null;
          host.__semioPhysicalJourneyInputs.push({ seq: ++seq, type: e.type, trusted: e.isTrusted, target: target?.id || target?.getAttribute("data-node-key") || "", ...(typeof e.key === "string" ? { key: e.key } : {}), ...(typeof e.clientX === "number" ? { point: [e.clientX, e.clientY] as Point } : {}) });
          if (host.__semioPhysicalJourneyInputs.length > 1024) host.__semioPhysicalJourneyInputs.shift();
        }, { capture: true });
      }, definition.physicalInputTypes); await page.goto(options.get(`--${r}-serve`)!, { waitUntil: "domcontentloaded" }); await d.boot(); }
    for (const step of definition.steps) {
      const receipt: Receipt = { name: step.name, renderers: {}, differences: [] };
      for (const [r, d] of drivers) { const before = await d.observe(); const cursor = (await d.actions()).at(-1)?.seq ?? 0; const inputCursor = (await d.inputs()).at(-1)?.seq ?? 0; let detail: Detail = {}; let error: string | null = null; try { detail = await d.run(step); await d.settle(before, cursor, step.settleMs ?? 1600); } catch (e) { error = String(e); } const after = await d.observe(); const actions = (await d.actions()).filter(a => a.seq > cursor); const inputs = (await d.inputs()).filter(i => i.seq > inputCursor); if (!error && (inputs.some(i => !i.trusted) || (step.operation !== "boot" && !detail.alreadyClosed && inputs.length === 0))) error = "Missing trusted physical input witness"; const screenshot = `${r}-${String(receipts.length + 1).padStart(2, "0")}-${step.name}.png`; await d.screenshot(resolve(out, screenshot)); const observed = step.operation === "boot" || detail.physicalOutcome === true || actions.length > 0 || !same([before.controls, before.surfaces, before.fullscreen, before.selected, before.example, before.role], [after.controls, after.surfaces, after.fullscreen, after.selected, after.example, after.role]); if (!error && !observed) error = "Missing published consequence"; receipt.renderers[r] = { before, after, actions, inputs, detail, error, screenshot, observed }; console.log(`[DEBUG] actual-shell-journey ${r} ${step.name} observed=${observed} actions=${actions.length} trustedInputs=${inputs.length} error=${error ?? "none"}`); }
      receipt.differences = renderer === "paired" ? shellInteractionDifferences(receipt, definition) : ["unpaired baseline"]; receipts.push(receipt); flush();
      if (Object.values(receipt.renderers).some(s => s.error)) throw new Error(`Actual shell journey failed at ${step.name}; evidence retained`);
    }
    if (renderer === "paired") assert(receipts.every(r => r.differences.length === 0), "Paired shell journey has recorded differences");
    console.log(`[DEBUG] actual-shell-journey completed ${receipts.length}/57 ${renderer === "paired" ? "paired" : "baseline"}`);
  } finally { flush(); await browser.close(); }
}
