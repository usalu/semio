/** 🫥️ The pet layer is decoration and behaves like it: one static, hidden element that no assistive technology and no
 * keyboard ever meets (judged by `@testing-library`'s accessibility tree and the role model of `aria-query`) and that
 * a fine pointer never hits — a press reaches the pets only through the hand's listeners on the window, never a
 * press on a control —, in which the cast of a scene appears on the top edges of the page's surfaces, keeps living
 * outside React, stands still when told to, runs nothing while the document is hidden or forced colours are active,
 * and leaves no listener, observer, frame, timer or element behind when it goes. Nothing is ever written to the
 * console. What the hand hands the stage is held in `🤏️pet-handling`.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx
 * @see ../🤏️pet-handling/🟦️.tsx
 * @see ../../🎯️targets/⚛️react/🎨️.css
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html
 */

import { cleanup, getRoles, isInaccessible, render, within } from "@testing-library/react";
import { roles } from "aria-query";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { StrictMode } from "react";
import { afterEach, beforeEach, describe, expect, it, vi, type MockInstance } from "vitest";
import { menagerieIssues, type Channel, type Clip, type Menagerie, type Species, type Track } from "@semio-tech/pets";
import { PET_EDGE_INSET, PET_HOME_SCENE, PET_LAYER_Z_INDEX, PetLayer, petCapacity, petCast, petScale } from "@semio-tech/pets-react";

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");

//#region 🔖️Menagerie
function track(bone: string, channel: Channel, values: readonly number[]): Track {
  return { bone, channel, keys: values.map((value, index) => ({ at: index / (values.length - 1), value })) };
}

function clip(id: string, seconds: number, loop: boolean, tracks: readonly Track[]): Clip {
  return { id, seconds, loop, tracks };
}

function specimen(id: string, body: string): Species {
  return {
    id,
    name: { en: id, de: id },
    thing: { en: "test rig", de: "Prüfgerüst" },
    grounds: [],
    size: { width: 40, height: 44 },
    palette: { body, accent: "#f7f3e3", detail: "#001117" },
    bones: [
      { id: "root", x: 0, y: 0 },
      { id: "body", parent: "root", x: 0, y: -22 },
    ],
    parts: [
      { id: "foot", bone: "root", shape: { kind: "line", x1: -6, y1: -6, x2: -6, y2: 0 }, fill: "none", stroke: "ink", strokeWidth: 3 },
      { id: "trunk", bone: "body", shape: { kind: "ellipse", cx: 0, cy: 0, rx: 18, ry: 20 }, fill: "body", stroke: "ink" },
    ],
    face: {
      eyes: [
        { id: "eye-left", bone: "body", x: -7, y: -4, radius: 4, pupil: 1.8 },
        { id: "eye-right", bone: "body", x: 7, y: -4, radius: 4, pupil: 1.8 },
      ],
      mouth: { bone: "body", x: 0, y: 7, width: 8 },
    },
    clips: [
      clip("idle", 3, true, [track("body", "scaleY", [1, 1.04, 1])]),
      clip("walk", 0.6, true, [track("body", "y", [0, -2, 0])]),
      clip("wiggle", 0.8, false, [track("body", "rotation", [0, 8, -8, 0])]),
      clip("doze", 4, true, [track("body", "scaleY", [1, 0.96, 1])]),
      clip("land", 0.3, false, [track("body", "scaleY", [1, 0.85, 1])]),
    ],
    repertoire: { idle: ["idle"], walk: ["walk"], fidget: ["wiggle"], greet: ["wiggle"], cuddle: ["wiggle"], squabble: ["wiggle"], sulk: ["idle"], sleep: ["doze"], land: ["land"], hang: ["idle"], tumble: ["land"], purr: ["doze"], dizzy: ["wiggle"], shrug: ["wiggle"], push: ["walk"] },
    locomotion: { gait: "walk", speed: 40 },
    temperament: { energy: 0.6, sociability: 0.6, curiosity: 0.6 },
    states: [{ id: "resting", name: { en: "Resting", de: "In Ruhe" } }],
    tricks: [],
    purr: { clip: "doze" },
    emitters: [],
    gear: [],
    grip: 40,
    reach: 10,
    mood: "content",
  };
}

const MENAGERIE: Menagerie = {
  schema: "semio.pets.menagerie/v1",
  id: "specimens",
  title: { en: "Specimens", de: "Exemplare" },
  species: [specimen("alba", "#1e9b8d"), specimen("bruno", "#fa9500"), specimen("carla", "#ff344f"), specimen("dora", "#7b61ff")],
  bonds: [{ between: ["alba", "bruno"], affinity: 0.5 }],
  casts: [
    { scene: "home", core: ["alba", "bruno"], rotation: ["carla"] },
    { scene: "garden", core: ["dora"], rotation: [] },
    { scene: "parade", core: ["alba"], rotation: ["bruno", "carla", "dora"] },
  ],
  chemistry: [],
};

const HOMELESS: Menagerie = { ...MENAGERIE, id: "homeless", casts: [{ scene: "garden", core: ["dora"], rotation: [] }] };
//#endregion 🔖️Menagerie

//#region 🔖️Page
interface Box {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

const VIEWPORT = { width: 1200, height: 800 } as const;
const CARDS = { first: { left: 100, top: 300, width: 400, height: 200 }, second: { left: 650, top: 420, width: 400, height: 200 } } as const;
const WORDS: Box = { left: 650, top: 380, width: 200, height: 36 };
const BUTTON: Box = { left: 100, top: 744, width: 120, height: 40 };
const HALF = 20;

function place<T extends Element>(element: T, box: Box): T {
  element.getBoundingClientRect = () => ({ ...box, x: box.left, y: box.top, right: box.left + box.width, bottom: box.top + box.height, toJSON: () => box }) as DOMRect;
  return element;
}

/** 🏗️ A page of two cards whose top edges carry pets, a paragraph hovering over the left part of the second card and a button near the floor. */
function page(): { readonly first: HTMLElement; readonly second: HTMLElement; readonly button: HTMLButtonElement } {
  Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: VIEWPORT.width });
  Object.defineProperty(document.documentElement, "clientHeight", { configurable: true, value: VIEWPORT.height });
  const first = place(document.createElement("article"), CARDS.first);
  const second = place(document.createElement("article"), CARDS.second);
  first.setAttribute("data-pet-surface", "");
  second.setAttribute("data-pet-surface", "");
  const words = place(document.createElement("p"), WORDS);
  const button = place(document.createElement("button"), BUTTON);
  document.body.append(first, words, second, button);
  return { first, second, button };
}

function pointer(type: string, x: number, y: number, init: MouseEventInit = {}): MouseEvent {
  const event = new MouseEvent(type, { bubbles: type !== "pointerleave", cancelable: true, clientX: x, clientY: y, button: 0, ...init });
  Object.defineProperty(event, "pointerType", { value: "mouse" });
  Object.defineProperty(event, "isPrimary", { value: true });
  return event;
}

interface Standing {
  readonly species: string;
  readonly x: number;
  readonly y: number;
  readonly flip: number;
  readonly scale: number;
  readonly opacity: number;
}

/** 🧍️ Where every pet of a layer stands, read back from what was painted. */
function standing(layer: Element): Standing[] {
  return [...layer.querySelectorAll<SVGSVGElement>("svg.pet")].map((pet) => {
    const match = /^translate\((-?[\d.]+)px, (-?[\d.]+)px\) scale\((-?[\d.]+), (-?[\d.]+)\)$/.exec(pet.style.transform);
    expect(match, pet.style.transform).not.toBeNull();
    return { species: pet.getAttribute("data-pet")!, x: Number(match![1]), y: Number(match![2]), flip: Number(match![3]), scale: Number(match![4]), opacity: Number(pet.style.opacity) };
  });
}

/** 👀️ How far every pet looks to the right of the page (negative: to the left), in pixels of its pupils. */
function looks(layer: Element): number[] {
  const pets = standing(layer);
  return [...layer.querySelectorAll<SVGSVGElement>("svg.pet")].map((pet, index) => Number(pet.querySelector(".pet-pupil")!.getAttribute("cx")) * Math.sign(pets[index]!.flip));
}

function layerOf(container: Element): HTMLElement {
  const layers = container.querySelectorAll<HTMLElement>(".pet-layer");
  expect(layers).toHaveLength(1);
  return layers[0]!;
}
//#endregion 🔖️Page

//#region 🔖️Ledgers
type Listening = { readonly type: string; readonly listener: unknown; readonly capture: boolean };

/** 📒️ Everything listening on a target that was added since the ledger opened and not removed again; calls go through. */
function ledger(target: EventTarget): () => string[] {
  const open: Listening[] = [];
  const capture = (options: unknown): boolean => (typeof options === "boolean" ? options : ((options as AddEventListenerOptions | undefined)?.capture ?? false));
  const add = target.addEventListener.bind(target);
  const remove = target.removeEventListener.bind(target);
  vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | AddEventListenerOptions) => {
    open.push({ type, listener, capture: capture(options) });
    add(type, listener, options);
  });
  vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | EventListenerOptions) => {
    const index = open.findIndex((entry) => entry.type === type && entry.listener === listener && entry.capture === capture(options));
    if (index >= 0) open.splice(index, 1);
    remove(type, listener, options);
  });
  return () => open.map((entry) => entry.type).sort();
}

/** 🔭️ Counts the observers alive: constructed, observing and not yet disconnected. */
function observers(): { resize: number; mutation: number } {
  const live = { resize: 0, mutation: 0 };
  class Resizes {
    observe(): void {
      live.resize += 1;
    }
    unobserve(): void {}
    disconnect(): void {
      live.resize -= 1;
    }
  }
  const Native = MutationObserver;
  class Mutations extends Native {
    override observe(target: Node, options?: MutationObserverInit): void {
      live.mutation += 1;
      super.observe(target, options);
    }
    override disconnect(): void {
      live.mutation -= 1;
      super.disconnect();
    }
  }
  vi.stubGlobal("ResizeObserver", Resizes);
  vi.stubGlobal("MutationObserver", Mutations);
  return live;
}
//#endregion 🔖️Ledgers

const CONSOLE = ["log", "info", "warn", "error", "debug", "trace"] as const;
let spoken: MockInstance[] = [];

beforeEach(() => {
  vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "requestAnimationFrame", "cancelAnimationFrame", "performance"] });
  spoken = CONSOLE.map((method) => vi.spyOn(console, method).mockImplementation(() => {}));
});

afterEach(() => {
  cleanup();
  for (const [index, spy] of spoken.entries()) expect(spy.mock.calls, `console.${CONSOLE[index]}`).toEqual([]);
  expect(vi.getTimerCount()).toBe(0);
  document.body.replaceChildren();
  document.head.replaceChildren();
  Reflect.deleteProperty(document.documentElement, "clientWidth");
  Reflect.deleteProperty(document.documentElement, "clientHeight");
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  vi.useRealTimers();
});

describe("🫥️ decorative layer", () => {
  it("is built on a menagerie the core accepts", () => {
    expect(menagerieIssues(MENAGERIE)).toEqual([]);
    expect(menagerieIssues(HOMELESS)).toEqual([]);
  });

  it("renders one static element hidden from assistive technology and lets a cast appear in it", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    const layer = layerOf(container);
    expect(layer.tagName).toBe("DIV");
    expect(layer.getAttribute("aria-hidden")).toBe("true");
    expect(layer.getAttributeNames().sort()).toEqual(["aria-hidden", "class", "style"]);
    expect(layer.className).toBe("pet-layer");
    expect(layer.style.zIndex).toBe(String(PET_LAYER_Z_INDEX));
    expect(layer.childElementCount).toBe(0);
    vi.advanceTimersByTime(3000);
    const pets = standing(layer);
    expect(pets.map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(layer.childElementCount).toBe(3);
    for (const pet of pets) {
      expect(Math.abs(pet.flip), pet.species).toBe(1);
      expect(pet.scale, pet.species).toBe(1);
      expect(pet.opacity, pet.species).toBeGreaterThan(0);
    }
    expect(layerOf(container)).toBe(layer);
  });

  it("exposes nothing to assistive technology and nothing to the keyboard", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const nodes = [layer, ...layer.querySelectorAll("*")];
    expect(nodes.length).toBeGreaterThan(20);
    expect(getRoles(layer)).toEqual({});
    expect(Object.values(getRoles(container)).flat()).toEqual([container]);
    for (const node of nodes) expect(isInaccessible(node), node.nodeName).toBe(true);
    const known = [...roles.keys()].filter((role) => roles.get(role)?.abstract !== true);
    expect(known.length).toBeGreaterThan(60);
    for (const role of known) expect(within(container).queryAllByRole(role), role).toEqual([]);
    for (const node of nodes) {
      expect((node as HTMLElement | SVGElement).tabIndex, node.nodeName).toBe(-1);
      for (const name of ["tabindex", "role", "id", "title", "aria-label", "aria-labelledby", "aria-live", "href", "contenteditable"]) expect(node.hasAttribute(name), `${node.nodeName} ${name}`).toBe(false);
      expect(node.matches("a, button, input, select, textarea, summary, details, iframe, object, embed, audio, video, title, desc, style, script, text, foreignObject"), node.nodeName).toBe(false);
    }
    for (const pet of layer.querySelectorAll("svg")) expect(pet.getAttribute("focusable")).toBe("false");
    expect(layer.textContent).toBe("");
  });

  it("never takes pointer events without its stylesheet either: a running show says so on the layer itself", () => {
    page();
    expect(document.head.querySelector("style")).toBeNull();
    const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(layer.style.pointerEvents).toBe("none");
    expect(layer.querySelectorAll("svg.pet").length).toBe(3);
    for (const node of layer.querySelectorAll("*")) expect(getComputedStyle(node).pointerEvents, node.nodeName).toBe("none");
    unmount();
    expect(layer.getAttribute("style") ?? "").toBe("");
  });

  it("shows nothing of a menagerie the core rejects, so nothing the schema forbids reaches the page", () => {
    page();
    const forged: Menagerie = { ...MENAGERIE, species: MENAGERIE.species.map((species, index) => (index === 0 ? { ...species, palette: { ...species.palette, body: "url(https://example.org/pixel)" } } : species)) };
    expect(menagerieIssues(forged).map((issue) => issue.code)).toEqual(["out-of-range"]);
    const { container } = render(<PetLayer menagerie={forged} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(5000);
    const layer = layerOf(container);
    expect(layer.childElementCount).toBe(0);
    expect(layer.getAttributeNames().sort()).toEqual(["aria-hidden", "class"]);
    expect(vi.getTimerCount()).toBe(0);
  });

  it("takes no pointer events where the pointer is fine: the stylesheet says so for the layer and everything in it, and only a touch pad would take them", () => {
    page();
    const sheet = document.createElement("style");
    sheet.textContent = css;
    document.head.append(sheet);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(getComputedStyle(layer).pointerEvents).toBe("none");
    expect(getComputedStyle(layer).position).toBe("fixed");
    const pets = layer.querySelectorAll("svg.pet");
    expect(pets.length).toBe(3);
    for (const node of layer.querySelectorAll("*")) expect(getComputedStyle(node).pointerEvents, node.nodeName).toBe("none");
    expect(css).toMatch(/@media \(forced-colors: active\) \{\s*\.pet-layer \{\s*display: none;/);
    expect(css).toMatch(/@media print \{\s*\.pet-layer \{\s*display: none;/);
    expect(layer.querySelector(".pet-pad, .pet-pads")).toBeNull();
    const rules = [...css.replace(/\/\*[\s\S]*?\*\//g, "").matchAll(/([^{}]+)\{([^}]*)\}/g)];
    expect(rules.filter(([, , body]) => /pointer-events:\s*(?!none)\S/.test(body!)).map(([, selector]) => selector!.trim())).toEqual([".pet-pad"]);
    expect(css).not.toContain("animation-name:");
  });

  it("never transitions: the stylesheet says so for the layer and everything in it, also under a host that shortens every transition for reduced motion", () => {
    page();
    const host = document.createElement("style");
    host.textContent = ".host * { transition-duration: 0.01ms !important; }";
    const sheet = document.createElement("style");
    sheet.textContent = css;
    document.head.append(sheet, host);
    const { container } = render(
      <div className="host">
        <p>text</p>
        <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />
      </div>,
    );
    vi.advanceTimersByTime(3000);
    const layer = container.querySelector<HTMLElement>(".pet-layer")!;
    expect(getComputedStyle(container.querySelector("p")!).transitionDuration).toBe("0.01ms");
    expect(getComputedStyle(container.querySelector("p")!).transitionProperty).not.toBe("none");
    expect(getComputedStyle(layer).transitionProperty).toBe("none");
    const nodes = [...layer.querySelectorAll("*")];
    expect(nodes.length).toBeGreaterThan(3);
    for (const node of nodes) expect(getComputedStyle(node).transitionProperty, node.nodeName).toBe("none");
    expect(css).toMatch(/\.pet-layer,\s*\.pet-layer \* \{\s*transition-property: none !important;/);
    host.remove();
    sheet.remove();
  });

  it("stands its pets on the top edges of the surfaces and the floor, never over what is kept free", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} capacity={3} />);
    vi.advanceTimersByTime(100);
    const pets = standing(layerOf(container));
    expect(pets).toHaveLength(3);
    const grounds = [
      { y: CARDS.first.top, x0: CARDS.first.left, x1: CARDS.first.left + CARDS.first.width },
      { y: CARDS.second.top, x0: WORDS.left + WORDS.width + 4, x1: CARDS.second.left + CARDS.second.width },
      { y: VIEWPORT.height, x0: 0, x1: VIEWPORT.width },
    ];
    for (const pet of pets) {
      const ground = grounds.find((candidate) => candidate.y === pet.y);
      expect(ground, `${pet.species} at ${pet.x}, ${pet.y}`).toBeDefined();
      expect(pet.x - HALF, pet.species).toBeGreaterThanOrEqual(ground!.x0);
      expect(pet.x + HALF, pet.species).toBeLessThanOrEqual(ground!.x1);
      if (pet.y === VIEWPORT.height) expect(pet.x + HALF <= BUTTON.left - 4 || pet.x - HALF >= BUTTON.left + BUTTON.width + 4, `${pet.species} in front of the button`).toBe(true);
      expect(pet.opacity, pet.species).toBe(1);
    }
  });

  it("finds a place for every pet on any stage it is seeded for", () => {
    page();
    const places = new Set<string>();
    for (let seed = 1; seed <= 24; seed++) {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={seed} />);
      vi.advanceTimersByTime(100);
      const pets = standing(layerOf(container));
      expect(pets, `seed ${seed}`).toHaveLength(3);
      for (const pet of pets) {
        expect([CARDS.first.top, CARDS.second.top, VIEWPORT.height], `seed ${seed} ${pet.species}`).toContain(pet.y);
        expect(pet.x - HALF, `seed ${seed} ${pet.species} at the left edge`).toBeGreaterThanOrEqual(PET_EDGE_INSET);
        expect(pet.x + HALF, `seed ${seed} ${pet.species} at the right edge`).toBeLessThanOrEqual(VIEWPORT.width - PET_EDGE_INSET);
        if (pet.y === CARDS.second.top) expect(pet.x - HALF, `seed ${seed} ${pet.species} in front of the words`).toBeGreaterThanOrEqual(WORDS.left + WORDS.width + 4);
        if (pet.y === VIEWPORT.height) expect(pet.x + HALF <= BUTTON.left - 4 || pet.x - HALF >= BUTTON.left + BUTTON.width + 4, `seed ${seed} ${pet.species} in front of the button`).toBe(true);
        places.add(`${pet.y}`);
      }
      unmount();
    }
    expect([...places].sort()).toEqual([`${CARDS.first.top}`, `${CARDS.second.top}`, `${VIEWPORT.height}`].sort());
  });

  it("paints a still stage once and schedules nothing while the pointer keeps away from the pets", () => {
    page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const timers = vi.spyOn(window, "setTimeout");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    expect(frames).toHaveBeenCalledTimes(1);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const painted = layer.innerHTML;
    expect(standing(layer)).toHaveLength(3);
    expect(vi.getTimerCount()).toBe(0);
    const writes = vi.spyOn(Element.prototype, "setAttribute");
    window.dispatchEvent(pointer("pointermove", 900, 100));
    window.dispatchEvent(pointer("pointerdown", 900, 120));
    window.dispatchEvent(pointer("pointerup", 900, 120));
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(120_000);
    expect(layer.innerHTML).toBe(painted);
    expect(writes).not.toHaveBeenCalled();
    expect(frames).toHaveBeenCalledTimes(1);
    expect(timers).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  it("keeps a pet of a still stage whole while the pointer rests on it, and wakes nothing for the pointer: no frame, no write, no timer", () => {
    page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const timers = vi.spyOn(window, "setTimeout");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const painted = layer.innerHTML;
    const before = standing(layer);
    const writes = vi.spyOn(Element.prototype, "setAttribute");
    window.dispatchEvent(pointer("pointermove", before[0]!.x, before[0]!.y - 10));
    vi.advanceTimersByTime(60_000);
    expect(standing(layer)).toEqual(before);
    expect(layer.innerHTML).toBe(painted);
    expect(frames).toHaveBeenCalledTimes(1);
    expect(writes).not.toHaveBeenCalled();
    expect(timers).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  it("keeps a pet of a living stage whole while the pointer rests on it: a pet on its perch is never see-through", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(1000);
    const layer = layerOf(container);
    const [first] = standing(layer);
    expect(first!.opacity).toBe(1);
    window.dispatchEvent(pointer("pointermove", first!.x, first!.y - 20));
    for (let wait = 0; wait < 7; wait++) {
      vi.advanceTimersByTime(100);
      expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
    }
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    vi.advanceTimersByTime(600);
    expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
  });

  it("follows a still stage's surfaces when the page moves, in one frame, and rests again", () => {
    const { first } = page();
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} />);
    vi.advanceTimersByTime(100);
    const layer = layerOf(container);
    const before = standing(layer);
    const riders = before.filter((pet) => pet.y === CARDS.first.top);
    place(first, { ...CARDS.first, left: CARDS.first.left + 30, top: CARDS.first.top + 50 });
    window.dispatchEvent(new Event("resize"));
    expect(frames).toHaveBeenCalledTimes(2);
    vi.advanceTimersByTime(100);
    const after = standing(layer);
    for (const rider of riders) {
      const moved = after.find((pet) => pet.species === rider.species)!;
      expect([moved.x, moved.y], rider.species).toEqual([rider.x + 30, rider.y + 50]);
    }
    for (const pet of before.filter((each) => each.y !== CARDS.first.top)) expect(after.find((each) => each.species === pet.species), pet.species).toEqual(pet);
    expect(frames).toHaveBeenCalledTimes(2);
    expect(vi.getTimerCount()).toBe(0);
  });

  it("keeps a calm stage alive outside React and lets its pets look at the pointer", () => {
    page();
    let renders = 0;
    function Host(): ReturnType<typeof PetLayer> {
      renders += 1;
      return <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />;
    }
    const { container } = render(<Host />);
    vi.advanceTimersByTime(1000);
    const layer = layerOf(container);
    expect(standing(layer).map((pet) => pet.opacity)).toEqual([1, 1, 1]);
    const event = pointer("pointermove", VIEWPORT.width + 2000, 300);
    window.dispatchEvent(event);
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => look > 1)).toBe(true);
    window.dispatchEvent(pointer("pointermove", -2000, 300));
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => look < -1)).toBe(true);
    document.documentElement.dispatchEvent(pointer("pointerleave", 0, 0));
    vi.advanceTimersByTime(1000);
    expect(looks(layer).every((look) => Math.abs(look) < 0.7)).toBe(true);
    const [first] = standing(layer);
    const button = document.querySelector("button")!;
    const heard = vi.fn();
    document.addEventListener("pointerdown", heard);
    const missed = pointer("pointerdown", first!.x, first!.y - 10);
    button.dispatchEvent(missed);
    window.dispatchEvent(pointer("pointerup", first!.x, first!.y - 10));
    const press = pointer("pointerdown", first!.x, first!.y - 10);
    document.body.dispatchEvent(press);
    window.dispatchEvent(pointer("pointerup", first!.x, first!.y - 10));
    vi.advanceTimersByTime(300);
    document.removeEventListener("pointerdown", heard);
    expect([missed.defaultPrevented, press.defaultPrevented], "a control under a pet keeps its press; a press on the pet is the pets'").toEqual([false, true]);
    expect(heard.mock.calls.map(([seen]) => seen)).toEqual([missed]);
    const seen = new Set<string>();
    for (let second = 0; second < 30; second++) {
      vi.advanceTimersByTime(1000);
      seen.add(layer.innerHTML);
    }
    expect(seen.size).toBeGreaterThan(10);
    expect(standing(layer)).toHaveLength(3);
    expect(vi.getTimerCount()).toBe(1);
    expect(event.defaultPrevented).toBe(false);
    expect(renders).toBe(1);
  });

  it("turns changed props into events of the same stage instead of rendering anew", () => {
    page();
    const { container, rerender } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const alba = layer.querySelector('[data-pet="alba"]');
    const carla = layer.querySelector('[data-pet="carla"]');
    expect(alba).not.toBeNull();
    expect(carla).not.toBeNull();
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} zIndex={12} scale={0.5} />);
    vi.advanceTimersByTime(100);
    expect(layerOf(container)).toBe(layer);
    expect(layer.style.zIndex).toBe("12");
    expect(layer.querySelector('[data-pet="alba"]')).toBe(alba);
    expect(standing(layer).map((pet) => pet.scale)).toEqual([0.5, 0.5, 0.5]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} capacity={2} />);
    vi.advanceTimersByTime(20_000);
    expect(layer.style.zIndex).toBe(String(PET_LAYER_Z_INDEX));
    const fewer = standing(layer)
      .map((pet) => pet.species)
      .sort();
    expect(fewer).toHaveLength(2);
    expect(fewer[1]).toBe("carla");
    expect(["alba", "bruno"]).toContain(fewer[0]);
    expect(layer.querySelector('[data-pet="carla"]')).toBe(carla);
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} />);
    vi.advanceTimersByTime(20_000);
    expect(standing(layer).map((pet) => pet.species)).toEqual(["dora"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="somewhere-else" mode="calm" seed={7} />);
    vi.advanceTimersByTime(20_000);
    expect(standing(layer).map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="somewhere-else" mode="still" seed={7} />);
    vi.advanceTimersByTime(1000);
    const frozen = layer.innerHTML;
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(60_000);
    expect(layer.innerHTML).toBe(frozen);
    rerender(<PetLayer menagerie={MENAGERIE} scene="somewhere-else" mode="lively" seed={7} />);
    expect(vi.getTimerCount()).toBe(1);
  });

  it("tells its host who is on stage, in the order of the menagerie, only when that changes, and nobody once it is gone", () => {
    page();
    const heard: string[] = [];
    const listen = (species: readonly string[]): void => {
      heard.push(species.join(" "));
    };
    const { rerender, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} onCast={listen} />);
    expect(heard).toEqual([]);
    vi.advanceTimersByTime(3000);
    expect(heard).toEqual(["alba bruno carla"]);
    vi.advanceTimersByTime(30_000);
    expect(heard).toEqual(["alba bruno carla"]);
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} onCast={listen} />);
    vi.advanceTimersByTime(20_000);
    expect(heard.at(-1)).toBe("dora");
    for (let index = 1; index < heard.length; index++) expect(heard[index]).not.toBe(heard[index - 1]);
    const other: string[] = [];
    rerender(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} onCast={(species) => void other.push(species.join(" "))} />);
    vi.advanceTimersByTime(100);
    expect(other).toEqual(["dora"]);
    const told = heard.length;
    unmount();
    expect(other).toEqual(["dora", ""]);
    expect(heard).toHaveLength(told);
  });

  it("lets a host make the pets' time pass faster or slower, held between an eighth and eight times the wall clock", () => {
    page();
    const faded = (tempo: number | undefined, milliseconds: number): number => {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="garden" mode="calm" seed={7} tempo={tempo} />);
      vi.advanceTimersByTime(milliseconds);
      const opacity = standing(layerOf(container))[0]!.opacity;
      unmount();
      return opacity;
    };
    const real = faded(undefined, 100);
    expect(real).toBeGreaterThan(0.2);
    expect(real).toBeLessThan(0.5);
    expect(faded(1, 100)).toBe(real);
    expect(faded(4, 100)).toBe(1);
    expect(faded(0.25, 100)).toBeLessThan(real);
    expect(Math.abs(faded(0.25, 400) - real)).toBeLessThanOrEqual(0.125);
    expect(faded(1000, 50)).toBe(faded(8, 50));
    expect(faded(0, 400)).toBe(faded(0.125, 400));
  });

  it("lets the rotation of a cast take turns every minute or two, never on a still or a quiet stage", () => {
    page();
    const company = (layer: Element): string[] =>
      standing(layer)
        .map((pet) => pet.species)
        .sort();
    const lively = render(<PetLayer menagerie={MENAGERIE} scene="parade" mode="calm" seed={7} capacity={2} />);
    const layer = layerOf(lively.container);
    vi.advanceTimersByTime(2000);
    const opening = company(layer);
    expect(opening).toHaveLength(2);
    expect(opening).toContain("alba");
    const turns = new Set<string>(opening);
    vi.advanceTimersByTime(55_000);
    expect(company(layer)).toEqual(opening);
    for (let second = 0; second < 200; second += 5) {
      vi.advanceTimersByTime(5000);
      const now = company(layer);
      expect(now).toContain("alba");
      expect(now.length).toBeLessThanOrEqual(3);
      for (const species of now) turns.add(species);
    }
    expect(turns.size).toBeGreaterThanOrEqual(3);
    lively.unmount();

    for (const [mode, quiet] of [["still", false], ["calm", true]] as const) {
      const resting = render(<PetLayer menagerie={MENAGERIE} scene="parade" mode={mode} quiet={quiet} seed={7} capacity={2} />);
      vi.advanceTimersByTime(2000);
      const cast = company(layerOf(resting.container));
      expect(cast, mode).toEqual(opening);
      for (let second = 0; second < 130; second += 10) {
        vi.advanceTimersByTime(10_000);
        expect(company(layerOf(resting.container)), mode).toEqual(cast);
      }
      resting.unmount();
    }
  });

  it("shows nobody for a scene without a cast and without a home", () => {
    page();
    expect(petCast(MENAGERIE, "garden")?.core).toEqual(["dora"]);
    expect(petCast(MENAGERIE, "nowhere")?.scene).toBe(PET_HOME_SCENE);
    expect(petCast(HOMELESS, "nowhere")).toBeNull();
    const { container } = render(<PetLayer menagerie={HOMELESS} scene="nowhere" mode="calm" seed={7} />);
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    expect(vi.getTimerCount()).toBe(0);
  });

  it("holds fewer and smaller pets on a narrow stage", () => {
    expect([0, 375, 767, 768, 1023, 1024, 1920].map(petCapacity)).toEqual([2, 2, 2, 4, 4, 6, 6]);
    expect([375, 767, 768, 1920].map(petScale)).toEqual([0.8, 0.8, 1, 1]);
    page();
    Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: 600 });
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={7} />);
    vi.advanceTimersByTime(100);
    const pets = standing(layerOf(container));
    expect(pets).toHaveLength(2);
    expect(pets.map((pet) => pet.species).sort()[1]).toBe("carla");
    expect(pets.map((pet) => pet.scale)).toEqual([0.8, 0.8]);
    for (const pet of pets) {
      expect([CARDS.first.top, VIEWPORT.height], `${pet.species} stands on an edge of the page, in the pixels of the page`).toContain(pet.y);
      const [left, right] = pet.y === CARDS.first.top ? [CARDS.first.left, CARDS.first.left + CARDS.first.width] : [0, 600];
      expect(pet.x - HALF * 0.8, pet.species).toBeGreaterThanOrEqual(left - 0.01);
      expect(pet.x + HALF * 0.8, pet.species).toBeLessThanOrEqual(right + 0.01);
    }
    Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: VIEWPORT.width });
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(100);
    const wide = standing(layerOf(container));
    expect(wide.map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(wide.map((pet) => pet.scale)).toEqual([1, 1, 1]);
  });

  it("runs neither frames nor timers while the document is hidden and resumes without catching up", () => {
    page();
    let state: DocumentVisibilityState = "visible";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(vi.getTimerCount()).toBe(1);
    state = "hidden";
    document.dispatchEvent(new Event("visibilitychange"));
    expect(vi.getTimerCount()).toBe(0);
    const hidden = layer.innerHTML;
    window.dispatchEvent(pointer("pointermove", 500, 100));
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(600_000);
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.innerHTML).toBe(hidden);
    state = "visible";
    document.dispatchEvent(new Event("visibilitychange"));
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(3000);
    expect(standing(layer)).toHaveLength(3);
    expect(layer.innerHTML).not.toBe(hidden);
  });

  it("waits where it is while a dialog makes the whole page inert, instead of falling off surfaces that seem gone", async () => {
    const { first, second } = page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    const before = standing(layer);
    expect(before).toHaveLength(3);
    expect(before.some((pet) => pet.y === CARDS.first.top || pet.y === CARDS.second.top)).toBe(true);
    for (const element of [first, second, container]) element.setAttribute("inert", "");
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(0);
    const waiting = layer.innerHTML;
    window.dispatchEvent(new Event("resize"));
    window.dispatchEvent(pointer("pointermove", 500, 100));
    await vi.advanceTimersByTimeAsync(120_000);
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.innerHTML).toBe(waiting);
    expect(standing(layer)).toEqual(before);
    for (const element of [first, second, container]) element.removeAttribute("inert");
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(1);
    await vi.advanceTimersByTimeAsync(48);
    expect(standing(layer).map(({ species, x, y }) => ({ species, x, y }))).toEqual(before.map(({ species, x, y }) => ({ species, x, y })));
    await vi.advanceTimersByTimeAsync(3000);
    expect(layer.innerHTML).not.toBe(waiting);
  });

  it("starts nothing in a hidden document until it is shown", () => {
    page();
    let state: DocumentVisibilityState = "hidden";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    expect(vi.getTimerCount()).toBe(0);
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    state = "visible";
    document.dispatchEvent(new Event("visibilitychange"));
    vi.advanceTimersByTime(3000);
    expect(standing(layerOf(container))).toHaveLength(3);
  });

  it("stays empty and runs nothing under forced colours", () => {
    page();
    const queries: string[] = [];
    vi.stubGlobal("matchMedia", (query: string) => {
      queries.push(query);
      return { matches: query === "(forced-colors: active)", media: query, addEventListener: () => {}, removeEventListener: () => {} };
    });
    const frames = vi.spyOn(window, "requestAnimationFrame");
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="lively" seed={7} />);
    vi.advanceTimersByTime(5000);
    expect(queries).toContain("(forced-colors: active)");
    expect(layerOf(container).childElementCount).toBe(0);
    expect(frames).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  it("leaves no listener, observer, frame, timer or element behind when it goes", () => {
    page();
    const before = document.body.innerHTML;
    const { container, rerender, unmount } = render(<i />);
    const live = observers();
    const listening = [ledger(window), ledger(document), ledger(document.documentElement)];
    rerender(<PetLayer menagerie={MENAGERIE} scene="home" mode="lively" seed={7} />);
    vi.advanceTimersByTime(5000);
    const layer = layerOf(container);
    expect(layer.childElementCount).toBe(3);
    const hand = ["blur", "click", "contextmenu", "dragstart", "input", "keydown", "lostpointercapture", "pointercancel", "pointerdown", "pointermove", "pointerup", "selectstart", "wheel"];
    expect(listening.map((open) => open())).toEqual([
      ["pagehide", "pageshow", "pointercancel", "pointerdown", "pointermove", "pointerup", "resize", ...hand].sort(),
      ["focusin", "focusout", "scroll", "scroll", "transitionend", "visibilitychange"],
      ["pointerleave"],
    ]);
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(vi.getTimerCount()).toBe(1);
    rerender(<i />);
    expect(listening.map((open) => open())).toEqual([[], [], []]);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
    expect(layer.childElementCount).toBe(0);
    expect(layer.style.zIndex).toBe("");
    expect(layer.isConnected).toBe(false);
    unmount();
    container.remove();
    expect(document.body.innerHTML).toBe(before);
    window.dispatchEvent(pointer("pointermove", 10, 10));
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(5000);
    expect(vi.getTimerCount()).toBe(0);
  });

  it("sets up once under StrictMode's double mount and tears down as cleanly", () => {
    page();
    const { container, rerender } = render(
      <StrictMode>
        <i />
      </StrictMode>,
    );
    const live = observers();
    const listening = [ledger(window), ledger(document), ledger(document.documentElement)];
    const frames = vi.spyOn(window, "requestAnimationFrame");
    rerender(
      <StrictMode>
        <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />
      </StrictMode>,
    );
    expect(frames).toHaveBeenCalledTimes(2);
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(standing(layer).map((pet) => pet.species).sort()).toEqual(["alba", "bruno", "carla"]);
    expect(layer.childElementCount).toBe(3);
    expect(listening.map((open) => open().length)).toEqual([20, 6, 1]);
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(vi.getTimerCount()).toBe(1);
    rerender(
      <StrictMode>
        <i />
      </StrictMode>,
    );
    expect(listening.map((open) => open())).toEqual([[], [], []]);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
  });

  it("gives every mount a stage of its own unless it is seeded, and the same stage for the same seed", () => {
    page();
    const play = (seed: number | undefined): string => {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={seed} />);
      vi.advanceTimersByTime(30_000);
      const painted = standing(layerOf(container))
        .map((pet) => `${pet.species}@${pet.x},${pet.y}`)
        .sort()
        .join(" ");
      unmount();
      return painted;
    };
    expect(play(7)).toBe(play(7));
    const random = vi.spyOn(Math, "random");
    play(undefined);
    expect(random).toHaveBeenCalled();
  });

  it("ends the show silently when something inside it fails", () => {
    page();
    const live = observers();
    const { container } = render(
      <PetLayer
        menagerie={MENAGERIE}
        scene="home"
        mode="calm"
        seed={7}
        glances={() => {
          throw new Error("a host function failed");
        }}
      />,
    );
    vi.advanceTimersByTime(5000);
    expect(layerOf(container).childElementCount).toBe(0);
    expect(live).toEqual({ resize: 0, mutation: 0 });
    expect(vi.getTimerCount()).toBe(0);
  });

  it("hands other things worth a look to the stage in its own pixels", () => {
    page();
    const glances = vi.fn(() => [{ x: 5000, y: 300 }]);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} glances={glances} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(glances).toHaveBeenCalled();
    expect(looks(layer)).toHaveLength(3);
    expect(looks(layer).every((look) => look > 0.5)).toBe(true);
  });
});
