/** 🧰️ The shared decorative layer test fixtures: the stylesheet, the menagerie, the page geometry, the pointer and the event ledgers. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, vi } from "vitest";
import { type Channel, type Clip, type Menagerie, type Species, type Track } from "@semio-tech/pets";

export const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../🎯️targets/⚛️react/🎨️.css"), "utf8");

//#region 🔖️Menagerie
function track(bone: string, channel: Channel, values: readonly number[]): Track {
  return { bone, channel, keys: values.map((value, index) => ({ at: index / (values.length - 1), value })) };
}

export function clip(id: string, seconds: number, loop: boolean, tracks: readonly Track[]): Clip {
  return { id, seconds, loop, tracks };
}

export function specimen(id: string, body: string): Species {
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

export const MENAGERIE: Menagerie = {
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

export const HOMELESS: Menagerie = { ...MENAGERIE, id: "homeless", casts: [{ scene: "garden", core: ["dora"], rotation: [] }] };
//#endregion 🔖️Menagerie

//#region 🔖️Page
interface Box {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

export const VIEWPORT = { width: 1200, height: 800 } as const;

export const CARDS = { first: { left: 100, top: 300, width: 400, height: 200 }, second: { left: 650, top: 420, width: 400, height: 200 } } as const;

export const WORDS: Box = { left: 650, top: 380, width: 200, height: 36 };

export const BUTTON: Box = { left: 100, top: 744, width: 120, height: 40 };

export const HALF = 20;

export function place<T extends Element>(element: T, box: Box): T {
  element.getBoundingClientRect = () => ({ ...box, x: box.left, y: box.top, right: box.left + box.width, bottom: box.top + box.height, toJSON: () => box }) as DOMRect;
  return element;
}

/** 🏗️ A page of two cards whose top edges carry pets, a paragraph hovering over the left part of the second card and a button near the floor. */
export function page(): { readonly first: HTMLElement; readonly second: HTMLElement; readonly button: HTMLButtonElement } {
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

export function pointer(type: string, x: number, y: number, init: MouseEventInit = {}): MouseEvent {
  const event = new MouseEvent(type, { bubbles: type !== "pointerleave", cancelable: true, clientX: x, clientY: y, button: 0, ...init });
  Object.defineProperty(event, "pointerType", { value: "mouse" });
  Object.defineProperty(event, "isPrimary", { value: true });
  return event;
}

export interface Standing {
  readonly species: string;
  readonly x: number;
  readonly y: number;
  readonly flip: number;
  readonly scale: number;
  readonly opacity: number;
}

/** 🧍️ Where every pet of a layer stands, read back from what was painted. */
export function standing(layer: Element): Standing[] {
  return [...layer.querySelectorAll<SVGSVGElement>("svg.pet")].map((pet) => {
    const match = /^translate\((-?[\d.]+)px, (-?[\d.]+)px\) scale\((-?[\d.]+), (-?[\d.]+)\)$/.exec(pet.style.transform);
    expect(match, pet.style.transform).not.toBeNull();
    return { species: pet.getAttribute("data-pet")!, x: Number(match![1]), y: Number(match![2]), flip: Number(match![3]), scale: Number(match![4]), opacity: Number(pet.style.opacity) };
  });
}

/** 👀️ How far every pet looks to the right of the page (negative: to the left), in pixels of its pupils. */
export function looks(layer: Element): number[] {
  const pets = standing(layer);
  return [...layer.querySelectorAll<SVGSVGElement>("svg.pet")].map((pet, index) => Number(pet.querySelector(".pet-pupil")!.getAttribute("cx")) * Math.sign(pets[index]!.flip));
}

export function layerOf(container: Element): HTMLElement {
  const layers = container.querySelectorAll<HTMLElement>(".pet-layer");
  expect(layers).toHaveLength(1);
  return layers[0]!;
}
//#endregion 🔖️Page

//#region 🔖️Ledgers
type Listening = { readonly type: string; readonly listener: unknown; readonly capture: boolean };

/** 📒️ Everything listening on a target that was added since the ledger opened and not removed again; calls go through. */
export function ledger(target: EventTarget): () => string[] {
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
export function observers(): { resize: number; mutation: number } {
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
