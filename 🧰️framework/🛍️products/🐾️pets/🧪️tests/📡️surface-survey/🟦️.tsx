/** 📡️ What the pet layer sees of a page: the survey turns element boxes into the `surveyed` event of the stage (shared
 * vectors, expected by a second reading of the design written in Python), names every surface once and for good, reads
 * without writing, knows when a measurement went stale and turns the pointer into stage events without ever touching
 * an event. Which elements count as controls is judged against the WAI-ARIA role model of `aria-query`, queried through
 * `@testing-library`'s own role lookup.
 *
 * @see ../../🧫️fixtures/📡️surface-survey/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts
 * @see https://www.w3.org/TR/wai-aria-1.2/#widget_roles
 */

import { getRoles } from "@testing-library/react";
import { roles } from "aria-query";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { StageEvent } from "@semio-tech/pets";
import { FIXTURE_ELEMENTS, FLOOR, FOCUS_MARGIN, KEEPOUT_MARGIN, PET_CONTROLS, PET_KEEPOUTS, PET_PRESS_CONTROLS, PET_PROPS, PET_SURFACES, SURFACE_WIDTH, SURVEY_ATTRIBUTES, fixtureId, stageBox, surfaceId, survey, wallId, watchPointer, watchSurvey, type SurveyHost } from "@semio-tech/pets-react";
import vectors from "../../🧫️fixtures/📡️surface-survey/🔣️.json";

interface Box {
  readonly left: number;
  readonly top: number;
  readonly width: number;
  readonly height: number;
}

interface SceneNode {
  readonly key: string;
  readonly tag: string;
  readonly attributes: Readonly<Record<string, string>>;
  readonly box?: Box;
  readonly overflow?: string;
  readonly children: readonly SceneNode[];
}

interface Scene {
  readonly name: string;
  readonly viewport: { readonly width: number; readonly height: number };
  readonly frame?: Box;
  readonly focus?: string;
  readonly pointer?: { readonly x: number; readonly y: number };
  readonly options?: { readonly surfaces?: string; readonly keepoutsBesidesDefaults?: string; readonly walls?: string; readonly props?: string };
  readonly nodes: readonly SceneNode[];
  readonly expected: {
    readonly kind: string;
    readonly width: number;
    readonly height: number;
    readonly surfaces: readonly { readonly element: string; readonly x0: number; readonly x1: number; readonly y: number }[];
    readonly keepouts: readonly { readonly x: number; readonly y: number; readonly width: number; readonly height: number }[];
    readonly walls: readonly { readonly element: string; readonly side: 1 | -1; readonly x: number; readonly y0: number; readonly y1: number }[];
    readonly fixtures?: readonly { readonly element: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number }[];
  };
}

const scenes = vectors.scenes as readonly Scene[];

/** 📦️ Gives an element the box a layout engine would have given it (jsdom lays nothing out). */
function place<T extends Element>(element: T, box: Box): T {
  element.getBoundingClientRect = () => ({ ...box, x: box.left, y: box.top, right: box.left + box.width, bottom: box.top + box.height, toJSON: () => box }) as DOMRect;
  return element;
}

/** 🪟️ Sizes the viewport the way a browser reports it without scrollbars. */
function viewport(width: number, height: number): void {
  Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: width });
  Object.defineProperty(document.documentElement, "clientHeight", { configurable: true, value: height });
}

function build(nodes: readonly SceneNode[], parent: Element, built: Map<string, HTMLElement>): void {
  for (const node of nodes) {
    const element = document.createElement(node.tag);
    for (const [name, value] of Object.entries(node.attributes)) element.setAttribute(name, value);
    if (node.box !== undefined) place(element, node.box);
    if (node.overflow !== undefined) {
      element.style.overflowX = node.overflow;
      element.style.overflowY = node.overflow;
    }
    built.set(node.key, element);
    parent.append(element);
    build(node.children, element, built);
  }
}

function card(box: Box, name = "data-pet-surface"): HTMLElement {
  const element = place(document.createElement("article"), box);
  element.setAttribute(name, "");
  document.body.append(element);
  return element;
}

/** 🖱️ A pointer event with the fields the feed reads, whether or not this DOM has a `PointerEvent` of its own. */
function pointer(type: string, init: MouseEventInit & { readonly pointerType?: string; readonly isPrimary?: boolean }): MouseEvent {
  const event = new MouseEvent(type, { bubbles: type !== "pointerleave", cancelable: true, ...init });
  Object.defineProperty(event, "pointerType", { value: init.pointerType ?? "mouse" });
  Object.defineProperty(event, "isPrimary", { value: init.isPrimary ?? true });
  return event;
}

/** 📒️ Records what is listening on a target: every `addEventListener` not yet undone by a matching `removeEventListener`. */
function ledger(target: EventTarget): { readonly open: () => string[]; readonly options: () => unknown[]; readonly close: () => void } {
  const open: { readonly type: string; readonly listener: unknown; readonly capture: boolean }[] = [];
  const seen: unknown[] = [];
  const capture = (options: unknown): boolean => (typeof options === "boolean" ? options : ((options as AddEventListenerOptions | undefined)?.capture ?? false));
  const add = vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: unknown, options?: unknown) => {
    open.push({ type, listener, capture: capture(options) });
    seen.push(options);
  });
  const remove = vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: unknown, options?: unknown) => {
    const index = open.findIndex((entry) => entry.type === type && entry.listener === listener && entry.capture === capture(options));
    if (index >= 0) open.splice(index, 1);
  });
  return {
    open: () => open.map((entry) => entry.type).sort(),
    options: () => seen,
    close: () => {
      add.mockRestore();
      remove.mockRestore();
    },
  };
}

/** 🏠️ The window as the watchers see it, with whatever observers a case hands in; calls reach the window as it is at that moment. */
function hostWith(observers: Pick<SurveyHost, "ResizeObserver" | "MutationObserver"> = {}): SurveyHost {
  return {
    document,
    addEventListener: ((...args: Parameters<Window["addEventListener"]>) => window.addEventListener(...args)) as Window["addEventListener"],
    removeEventListener: ((...args: Parameters<Window["removeEventListener"]>) => window.removeEventListener(...args)) as Window["removeEventListener"],
    ...observers,
  };
}

afterEach(() => {
  document.body.replaceChildren();
  Reflect.deleteProperty(document.documentElement, "clientWidth");
  Reflect.deleteProperty(document.documentElement, "clientHeight");
  vi.restoreAllMocks();
});

describe("📡️ surface survey", () => {
  it("ships vectors for every rule of the survey", () => {
    expect(scenes.length).toBeGreaterThanOrEqual(10);
    expect(vectors.margins).toEqual({ surfaceWidth: SURFACE_WIDTH, keepout: KEEPOUT_MARGIN, focus: FOCUS_MARGIN, fixtureElements: FIXTURE_ELEMENTS });
    expect(PET_SURFACES).toBe("[data-pet-surface]");
    expect(PET_PROPS).toBe("[data-pet-prop]");
    expect(scenes.filter((scene) => (scene.expected.fixtures ?? []).length > 0).length).toBeGreaterThanOrEqual(3);
  });

  for (const scene of scenes) {
    it(`surveys: ${scene.name}`, () => {
      viewport(scene.viewport.width, scene.viewport.height);
      const built = new Map<string, HTMLElement>();
      build(scene.nodes, document.body, built);
      const frame = scene.frame === undefined ? undefined : place(document.createElement("div"), scene.frame);
      if (frame !== undefined) document.body.append(frame);
      if (scene.focus !== undefined) {
        built.get(scene.focus)!.focus();
        expect(document.activeElement).toBe(built.get(scene.focus));
      }
      const before = document.body.outerHTML;
      const surveyed = survey(document, {
        ...(scene.options?.surfaces === undefined ? {} : { surfaces: scene.options.surfaces }),
        ...(scene.options?.keepoutsBesidesDefaults === undefined ? {} : { keepouts: `${PET_KEEPOUTS}, ${scene.options.keepoutsBesidesDefaults}` }),
        ...(scene.options?.walls === undefined ? {} : { walls: scene.options.walls }),
        ...(scene.options?.props === undefined ? {} : { props: scene.options.props }),
        ...(scene.pointer === undefined ? {} : { pointer: scene.pointer }),
        frame,
      });
      expect(surveyed).toEqual({
        ...scene.expected,
        fixtures: (scene.expected.fixtures ?? []).map(({ element, ...fixture }) => ({ id: fixtureId(built.get(element)!), ...fixture })),
        surfaces: scene.expected.surfaces.map(({ element, ...edge }) => ({ id: element === "floor" ? FLOOR : surfaceId(built.get(element)!), ...edge })),
        walls: scene.expected.walls.map(({ element, side, ...wall }) => ({ id: wallId(built.get(element)!, side), surface: surfaceId(built.get(element)!), side, ...wall })),
      });
      expect(document.body.outerHTML).toBe(before);
    });
  }

  it("measures inside a root only, in the pixels of the stage", () => {
    viewport(1000, 700);
    const inside = document.createElement("section");
    document.body.append(inside);
    const mine = place(document.createElement("article"), { left: 100, top: 300, width: 200, height: 100 });
    mine.setAttribute("data-pet-surface", "");
    inside.append(mine);
    const theirs = card({ left: 500, top: 300, width: 200, height: 100 });
    expect(survey(inside).surfaces.map((surface) => surface.id)).toEqual([surfaceId(mine), FLOOR]);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([surfaceId(mine), surfaceId(theirs), FLOOR]);
    expect(stageBox(document)).toEqual({ x: 0, y: 0, width: 1000, height: 700 });
    expect(stageBox(document, place(document.createElement("div"), { left: 10, top: 20, width: 300, height: 200 }))).toEqual({ x: 10, y: 20, width: 300, height: 200 });
  });

  it("lets the page hide its own overflow without losing a surface: that clip is the viewport's", () => {
    viewport(1000, 700);
    for (const element of [document.documentElement, document.body]) {
      element.style.overflowX = "hidden";
      element.style.overflowY = "hidden";
    }
    const inside = card({ left: 100, top: 300, width: 200, height: 100 });
    expect(getComputedStyle(document.body).overflowY).toBe("hidden");
    expect(survey(document).surfaces).toEqual([
      { id: surfaceId(inside), x0: 100, x1: 300, y: 300 },
      { id: FLOOR, x0: 0, x1: 1000, y: 700 },
    ]);
    document.documentElement.removeAttribute("style");
    document.body.removeAttribute("style");
  });

  it("sees nothing from a root that is itself behind the scenes, and everything again once it is not", () => {
    viewport(1000, 700);
    const backstage = document.createElement("section");
    backstage.setAttribute("inert", "");
    const root = document.createElement("div");
    backstage.append(root);
    document.body.append(backstage);
    const inside = place(document.createElement("article"), { left: 100, top: 300, width: 200, height: 100 });
    inside.setAttribute("data-pet-surface", "");
    const words = place(document.createElement("p"), { left: 400, top: 300, width: 200, height: 40 });
    const folded = document.createElement("div");
    folded.setAttribute("hidden", "");
    folded.append(place(document.createElement("p"), { left: 400, top: 400, width: 200, height: 40 }));
    const after = place(document.createElement("p"), { left: 400, top: 500, width: 200, height: 40 });
    root.append(inside, words, folded, after);
    const floor = { id: FLOOR, x0: 0, x1: 1000, y: 700 };
    expect(survey(root)).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [floor], keepouts: [], walls: [], fixtures: [] });
    expect(survey(document)).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [floor], keepouts: [], walls: [], fixtures: [] });
    backstage.removeAttribute("inert");
    const walls = [
      { id: wallId(inside, -1), surface: surfaceId(inside), side: -1, x: 100, y0: 300, y1: 400 },
      { id: wallId(inside, 1), surface: surfaceId(inside), side: 1, x: 300, y0: 300, y1: 400 },
    ];
    const shown = { kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: surfaceId(inside), x0: 100, x1: 300, y: 300 }, floor], keepouts: [{ x: 396, y: 296, width: 208, height: 48 }, { x: 396, y: 496, width: 208, height: 48 }, { x: 96, y: 300, width: 208, height: 100 }], walls, fixtures: [] };
    expect(survey(root)).toEqual(shown);
    expect(survey(document)).toEqual(shown);
    folded.remove();
    expect(survey(root)).toEqual(shown);
  });

  it("falls back to the window's own size where the page reports none", () => {
    expect(stageBox(document)).toEqual({ x: 0, y: 0, width: window.innerWidth, height: window.innerHeight });
    expect(window.innerWidth).toBeGreaterThan(0);
  });

  it("names every surface once and for good, however the page is rearranged", () => {
    viewport(1200, 800);
    const first = card({ left: 0, top: 100, width: 200, height: 100 });
    const second = card({ left: 300, top: 100, width: 200, height: 100 });
    const names = survey(document).surfaces.map((surface) => surface.id);
    expect(names).toEqual([surfaceId(first), surfaceId(second), FLOOR]);
    expect(new Set(names).size).toBe(3);
    document.body.prepend(second);
    place(second, { left: 600, top: 400, width: 250, height: 100 });
    expect(survey(document).surfaces).toEqual([
      { id: names[1], x0: 600, x1: 850, y: 400 },
      { id: names[0], x0: 0, x1: 200, y: 100 },
      { id: FLOOR, x0: 0, x1: 1200, y: 800 },
    ]);
    first.remove();
    const third = card({ left: 0, top: 500, width: 200, height: 100 });
    const later = survey(document).surfaces.map((surface) => surface.id);
    expect(later).toEqual([names[1], surfaceId(third), FLOOR]);
    expect(names).not.toContain(surfaceId(third));
    document.body.append(first);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([names[1], later[1], names[0], FLOOR]);
  });

  it("counts as controls exactly what the WAI-ARIA role model of aria-query calls a widget", () => {
    const widget = (role: string): boolean => {
      const definition = roles.get(role as never);
      return definition !== undefined && !definition.abstract && definition.superClass.some((chain) => (chain as readonly string[]).includes("widget"));
    };
    const named = [...PET_CONTROLS.matchAll(/\[role="([a-z]+)"\]/g)].map((match) => match[1]!);
    expect(named.length).toBeGreaterThanOrEqual(9);
    for (const role of named) expect(widget(role), role).toBe(true);
    const host = document.createElement("div");
    host.innerHTML =
      '<button>b</button><a href="#x">a</a><a>plain</a><input type="text"><input type="checkbox"><input type="radio"><input type="range"><select><option>o</option></select><textarea></textarea>' +
      "<div>d</div><p>p</p><h2>h</h2><ul><li>l</li></ul><section>s</section><span>t</span><img alt=\"i\">" +
      named.map((role) => `<span role="${role}">r</span>`).join("") +
      '<span role="note">n</span><span role="img">m</span><span role="status">u</span>';
    document.body.append(host);
    const judged = new Map<Element, boolean>();
    for (const [role, elements] of Object.entries(getRoles(host))) for (const element of elements) judged.set(element, (judged.get(element) ?? false) || widget(role));
    const sample = [...host.querySelectorAll("*")].filter((element) => element.tagName !== "OPTION");
    expect(sample.length).toBeGreaterThanOrEqual(25);
    for (const element of sample) expect(element.matches(PET_CONTROLS), element.outerHTML).toBe(judged.get(element) ?? false);
    const beyond = document.createElement("div");
    beyond.innerHTML = '<summary>s</summary><div tabindex="0">t</div><div tabindex="-1">u</div><div contenteditable="true">c</div><div contenteditable="false">f</div>';
    expect([...beyond.children].map((element) => element.matches(PET_CONTROLS))).toEqual([true, true, false, true, false]);
  });

  it("tells when a survey went stale, at once for what moves and soon for what changed, and leaves nothing behind", async () => {
    const live = { resize: 0, mutation: 0 };
    const resized: (() => void)[] = [];
    const observed: unknown[] = [];
    class Resizes {
      constructor(callback: () => void) {
        resized.push(callback);
      }
      observe(target: Element): void {
        live.resize += 1;
        observed.push(target);
      }
      unobserve(): void {}
      disconnect(): void {
        live.resize -= 1;
      }
    }
    class Mutations extends MutationObserver {
      override observe(target: Node, options?: MutationObserverInit): void {
        live.mutation += 1;
        observed.push(options);
        super.observe(target, options);
      }
      override disconnect(): void {
        live.mutation -= 1;
        super.disconnect();
      }
    }
    const pane = document.createElement("div");
    const layer = document.createElement("div");
    const button = document.createElement("button");
    document.body.append(pane, layer, button);
    const page = ledger(document);
    const view = ledger(window);
    const host = hostWith({ ResizeObserver: Resizes as unknown as typeof ResizeObserver, MutationObserver: Mutations });
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(host, document, stale, { ignore: layer });
    expect(live).toEqual({ resize: 1, mutation: 1 });
    expect(observed).toEqual([document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: [...SURVEY_ATTRIBUTES] }]);
    expect(SURVEY_ATTRIBUTES).toEqual(["class", "hidden", "inert", "open"]);
    expect(page.open()).toEqual(["focusin", "focusout", "scroll", "transitionend"]);
    expect(view.open()).toEqual(["resize"]);
    for (const options of [...page.options(), ...view.options()]) expect(options).toEqual({ capture: true, passive: true });
    page.close();
    view.close();
    stop();
    expect(live).toEqual({ resize: 0, mutation: 0 });

    const end = watchSurvey(host, document, stale, { ignore: layer });
    const scroll = new Event("scroll", { cancelable: true });
    pane.dispatchEvent(scroll);
    expect(stale.mock.calls).toEqual([[true]]);
    expect(scroll.defaultPrevented).toBe(false);
    window.dispatchEvent(new Event("resize"));
    resized.at(-1)!();
    expect(stale.mock.calls).toEqual([[true], [true], [true]]);
    stale.mockClear();
    button.focus();
    expect(stale.mock.calls.length).toBeGreaterThanOrEqual(1);
    expect(stale.mock.calls.every(([urgent]) => urgent === false)).toBe(true);
    stale.mockClear();
    button.blur();
    pane.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale.mock.calls).toEqual([[false], [false]]);
    stale.mockClear();
    const pet = layer.appendChild(document.createElementNS("http://www.w3.org/2000/svg", "g"));
    pet.dispatchEvent(new Event("transitionend", { bubbles: true }));
    layer.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale, "the end of a transition inside the layer moves no surface").not.toHaveBeenCalled();
    document.body.dispatchEvent(new Event("transitionend", { bubbles: true }));
    expect(stale.mock.calls).toEqual([[false]]);
    pet.remove();
    await new Promise((resolve) => setTimeout(resolve, 0));
    stale.mockClear();

    const settle = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));
    pane.append(document.createElement("p"));
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    stale.mockClear();
    for (const [name, value] of [["class", "open"], ["hidden", ""], ["inert", ""], ["open", ""]] as const) {
      pane.setAttribute(name, value);
      await settle();
      expect(stale.mock.calls, name).toEqual([[false]]);
      stale.mockClear();
    }
    pane.setAttribute("title", "nothing that moves a box");
    pane.setAttribute("style", "color: red");
    layer.append(document.createElementNS("http://www.w3.org/2000/svg", "svg"));
    layer.firstElementChild!.setAttribute("class", "pet");
    await settle();
    expect(stale).not.toHaveBeenCalled();
    layer.append(document.createElement("i"));
    pane.append(document.createElement("i"));
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    stale.mockClear();
    layer.setAttribute("inert", "");
    await settle();
    expect(stale.mock.calls).toEqual([[false]]);
    layer.removeAttribute("inert");
    await settle();
    stale.mockClear();

    end();
    pane.dispatchEvent(new Event("scroll"));
    window.dispatchEvent(new Event("resize"));
    button.focus();
    pane.append(document.createElement("p"));
    await settle();
    expect(stale).not.toHaveBeenCalled();
    expect(live).toEqual({ resize: 0, mutation: 0 });
  });

  it("passes over the greeting of a resize observer about a root that already has a size", () => {
    viewport(1000, 700);
    const notify: (() => void)[] = [];
    class Resizes {
      constructor(callback: () => void) {
        notify.push(callback);
      }
      observe(): void {}
      unobserve(): void {}
      disconnect(): void {}
    }
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(hostWith({ ResizeObserver: Resizes as unknown as typeof ResizeObserver }), document, stale);
    notify[0]!();
    expect(stale).not.toHaveBeenCalled();
    notify[0]!();
    notify[0]!();
    expect(stale.mock.calls).toEqual([[true], [true]]);
    stop();
  });

  it("watches a part of the page by its own element and works without observers", () => {
    const part = document.createElement("section");
    document.body.append(part);
    const stale = vi.fn<(urgent: boolean) => void>();
    const stop = watchSurvey(hostWith(), part, stale);
    window.dispatchEvent(new Event("resize"));
    expect(stale.mock.calls).toEqual([[true]]);
    stop();
  });

  it("turns the pointer into stage events without touching an event", () => {
    const page = ledger(window);
    const root = ledger(document.documentElement);
    const quiet = watchPointer(window, () => {});
    expect(page.open()).toEqual(["pointercancel", "pointerdown", "pointermove", "pointerup"]);
    expect(root.open()).toEqual(["pointerleave"]);
    for (const options of [...page.options(), ...root.options()]) expect(options).toEqual({ passive: true });
    quiet();
    expect([...page.open(), ...root.open()]).toEqual([]);
    page.close();
    root.close();

    const events: StageEvent[] = [];
    const stop = watchPointer(window, (event) => events.push(event), { origin: () => ({ x: 10, y: 20 }) });
    const plain = document.createElement("div");
    const button = document.createElement("button");
    const inner = document.createElement("span");
    button.append(inner);
    document.body.append(plain, button);
    const sent: MouseEvent[] = [];
    const send = (target: EventTarget, event: MouseEvent): void => {
      sent.push(event);
      target.dispatchEvent(event);
    };

    send(plain, pointer("pointermove", { clientX: 310, clientY: 220 }));
    send(inner, pointer("pointermove", { clientX: 320, clientY: 230 }));
    expect(events).toEqual([
      { kind: "pointed", x: 300, y: 200, over: "free" },
      { kind: "pointed", x: 310, y: 210, over: "control" },
    ]);
    events.length = 0;

    send(plain, pointer("pointerdown", { clientX: 120, clientY: 90, button: 0 }));
    send(plain, pointer("pointermove", { clientX: 130, clientY: 95, buttons: 1 }));
    send(plain, pointer("pointerup", { clientX: 130, clientY: 95, button: 0 }));
    send(plain, pointer("pointermove", { clientX: 131, clientY: 95 }));
    expect(events, "a press the pets did not take keeps the pointer busy until it ends").toEqual([
      { kind: "pointed", x: 110, y: 70, over: "free" },
      { kind: "pointed", x: 120, y: 75, over: "control" },
      { kind: "pointed", x: 121, y: 75, over: "free" },
    ]);
    events.length = 0;

    send(inner, pointer("pointerdown", { clientX: 120, clientY: 90, button: 0 }));
    send(inner, pointer("pointerup", { clientX: 120, clientY: 90, button: 0 }));
    send(plain, pointer("pointerdown", { clientX: 120, clientY: 90, button: 2 }));
    send(plain, pointer("pointermove", { clientX: 120, clientY: 90, buttons: 2 }));
    send(plain, pointer("pointerdown", { clientX: 120, clientY: 90, button: 0, isPrimary: false }));
    send(plain, pointer("pointermove", { clientX: 120, clientY: 90, buttons: 1, isPrimary: false }));
    send(plain, pointer("pointerdown", { clientX: 500, clientY: 90, button: 0 }));
    send(plain, pointer("pointermove", { clientX: 500, clientY: 91 }));
    expect(events).toEqual([
      { kind: "pointed", x: 110, y: 70, over: "control" },
      { kind: "pointed", x: 110, y: 70, over: "free" },
      { kind: "pointed", x: 110, y: 70, over: "free" },
      { kind: "pointed", x: 110, y: 70, over: "free" },
      { kind: "pointed", x: 110, y: 70, over: "free" },
      { kind: "pointed", x: 490, y: 70, over: "free" },
      { kind: "pointed", x: 490, y: 71, over: "free" },
    ]);
    events.length = 0;

    const label = document.createElement("label");
    const grip = document.createElement("span");
    grip.setAttribute("data-grip", "");
    document.body.append(label, grip);
    send(label, pointer("pointermove", { clientX: 20, clientY: 30 }));
    send(grip, pointer("pointermove", { clientX: 20, clientY: 30 }));
    expect(events.map((event) => (event.kind === "pointed" ? event.over : event.kind)), "a label hands its press on to its control").toEqual(["control", "free"]);
    events.length = 0;

    send(plain, pointer("pointerleave", {}));
    send(document.documentElement, pointer("pointerleave", { pointerType: "touch" }));
    expect(events).toEqual([]);
    send(document.documentElement, pointer("pointerleave", {}));
    send(document.documentElement, pointer("pointerleave", { pointerType: "pen" }));
    expect(events).toEqual([{ kind: "unpointed" }, { kind: "unpointed" }]);
    events.length = 0;

    send(plain, pointer("pointerup", { clientX: 120, clientY: 90, button: 0 }));
    send(plain, pointer("pointerup", { clientX: 120, clientY: 90, button: 0, pointerType: "pen" }));
    send(plain, pointer("pointercancel", {}));
    expect(events).toEqual([]);
    send(plain, pointer("pointerdown", { clientX: 120, clientY: 90, button: 0, pointerType: "touch" }));
    send(plain, pointer("pointerup", { clientX: 120, clientY: 90, button: 0, pointerType: "touch" }));
    send(plain, pointer("pointerdown", { clientX: 500, clientY: 90, button: 0, pointerType: "touch" }));
    send(plain, pointer("pointercancel", { pointerType: "touch" }));
    expect(events).toEqual([{ kind: "pointed", x: 110, y: 70, over: "free" }, { kind: "unpointed" }, { kind: "pointed", x: 490, y: 70, over: "free" }, { kind: "unpointed" }]);
    events.length = 0;

    for (const event of sent) expect(event.defaultPrevented, event.type).toBe(false);
    stop();
    send(plain, pointer("pointermove", { clientX: 1, clientY: 1 }));
    send(plain, pointer("pointerdown", { clientX: 120, clientY: 90, button: 0 }));
    send(plain, pointer("pointerup", { clientX: 120, clientY: 90, button: 0, pointerType: "touch" }));
    send(document.documentElement, pointer("pointerleave", {}));
    expect(events).toEqual([]);
  });

  it("measures from the viewport's corner when the layer tells it nothing", () => {
    const events: StageEvent[] = [];
    const stop = watchPointer(window, (event) => events.push(event));
    document.body.dispatchEvent(pointer("pointerdown", { clientX: 5, clientY: 6, button: 0 }));
    expect(events).toEqual([{ kind: "pointed", x: 5, y: 6, over: "free" }]);
    stop();
  });

  it("asks the host which elements are controls at every event", () => {
    expect(PET_PRESS_CONTROLS).toBe(`${PET_CONTROLS}, label`);
    const events: StageEvent[] = [];
    let controls = `${PET_PRESS_CONTROLS}, [data-grip]`;
    const stop = watchPointer(window, (event) => events.push(event), { controls: () => controls });
    const grip = document.createElement("span");
    grip.setAttribute("data-grip", "");
    const button = document.createElement("button");
    document.body.append(grip, button);
    grip.dispatchEvent(pointer("pointermove", { clientX: 5, clientY: 6 }));
    controls = "[data-grip]";
    button.dispatchEvent(pointer("pointermove", { clientX: 5, clientY: 6 }));
    grip.dispatchEvent(pointer("pointermove", { clientX: 5, clientY: 6 }));
    expect(events.map((event) => (event.kind === "pointed" ? event.over : event.kind))).toEqual(["control", "free", "control"]);
    stop();
  });

  it("names the sides of a surface as walls of their own, once and for good, and leaves the sides of other elements to the host", () => {
    viewport(1000, 700);
    const card = place(document.createElement("article"), { left: 100, top: 300, width: 200, height: 100 });
    card.setAttribute("data-pet-surface", "");
    const rail = place(document.createElement("aside"), { left: 600, top: 100, width: 50, height: 400 });
    rail.setAttribute("data-rail", "");
    document.body.append(card, rail);
    const walls = survey(document).walls;
    expect(walls).toEqual([
      { id: wallId(card, -1), surface: surfaceId(card), side: -1, x: 100, y0: 300, y1: 400 },
      { id: wallId(card, 1), surface: surfaceId(card), side: 1, x: 300, y0: 300, y1: 400 },
    ]);
    expect(new Set([...walls.map((wall) => wall.id), surfaceId(card), surfaceId(rail)]).size).toBe(4);
    expect(wallId(card, -1)).toBe(wallId(card, -1));
    place(card, { left: 140, top: 320, width: 200, height: 100 });
    expect(survey(document, { walls: "[data-rail]" }).walls.map((wall) => [wall.id, wall.x])).toEqual([
      [wallId(card, -1), 140],
      [wallId(card, 1), 340],
      [wallId(rail, -1), 600],
      [wallId(rail, 1), 650],
    ]);
    expect(survey(document, { walls: "[data-rail]" }).surfaces.map((surface) => surface.id)).toEqual([surfaceId(card), FLOOR]);
  });

  it("names every fixture once and for good, never sees one inside the stage's own element and reads nothing but its key", () => {
    viewport(1000, 700);
    const row = place(document.createElement("li"), { left: 100, top: 300, width: 400, height: 30 });
    row.setAttribute("data-pet-prop", "heating/u-values");
    row.setAttribute("data-correct", "true");
    row.setAttribute("data-value", "0.24");
    const layer = place(document.createElement("div"), { left: 0, top: 0, width: 1000, height: 700 });
    const inside = place(document.createElement("div"), { left: 600, top: 300, width: 200, height: 30 });
    inside.setAttribute("data-pet-prop", "heating");
    layer.append(inside);
    document.body.append(row, layer);
    const fixture = { id: fixtureId(row), key: "heating/u-values", x: 100, y: 300, width: 400, height: 30 };
    expect(survey(document, { frame: layer }).fixtures).toEqual([fixture]);
    expect(survey(document).fixtures.map((each) => each.id)).toEqual([fixtureId(row), fixtureId(inside)]);
    place(row, { left: 120, top: 340, width: 400, height: 30 });
    expect(survey(document, { frame: layer, pointer: null }).fixtures).toEqual([{ ...fixture, x: 120, y: 340 }]);
    expect(survey(document, { frame: layer, pointer: { x: 520, y: 370 } }).fixtures).toEqual([]);
    expect(survey(document, { frame: layer, props: "[data-other]" }).fixtures).toEqual([]);
    expect(fixtureId(row)).not.toBe(surfaceId(row));
    const reads = vi.spyOn(Element.prototype, "getAttribute");
    const dataset = vi.spyOn(HTMLElement.prototype, "dataset", "get");
    survey(document, { frame: layer });
    expect(reads.mock.calls.map(([name]) => name)).toContain("data-pet-prop");
    expect(reads.mock.calls.map(([name]) => name).filter((name) => name === "data-correct" || name === "data-value")).toEqual([]);
    expect(dataset).not.toHaveBeenCalled();
  });

  it("never looks into the stage's own element, and asks every element it walks once whether it is anything at all", () => {
    viewport(1000, 700);
    const outside = card({ left: 100, top: 300, width: 200, height: 100 });
    const plain = place(document.createElement("div"), { left: 600, top: 100, width: 100, height: 100 });
    const layer = place(document.createElement("div"), { left: 0, top: 0, width: 1000, height: 700 });
    const drawn = place(document.createElement("article"), { left: 500, top: 300, width: 200, height: 100 });
    drawn.setAttribute("data-pet-surface", "");
    drawn.append(place(document.createElement("p"), { left: 510, top: 310, width: 100, height: 20 }), place(document.createElement("button"), { left: 510, top: 340, width: 100, height: 20 }));
    layer.append(drawn);
    document.body.append(plain, layer);
    const page = { kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: surfaceId(outside), x0: 100, x1: 300, y: 300 }, { id: FLOOR, x0: 0, x1: 1000, y: 700 }], keepouts: [{ x: 96, y: 300, width: 208, height: 100 }], walls: [{ id: wallId(outside, -1), surface: surfaceId(outside), side: -1, x: 100, y0: 300, y1: 400 }, { id: wallId(outside, 1), surface: surfaceId(outside), side: 1, x: 300, y0: 300, y1: 400 }], fixtures: [] };
    expect(survey(document, { frame: layer, walls: "article" })).toEqual(page);
    expect(survey(document).surfaces.map((surface) => surface.id)).toEqual([surfaceId(outside), surfaceId(drawn), FLOOR]);
    const backstage = document.createElement("section");
    backstage.setAttribute("inert", "");
    document.body.append(backstage);
    const asked = vi.spyOn(Element.prototype, "matches");
    expect(survey(document, { frame: layer, walls: "article" })).toEqual(page);
    const askedOf = (element: Element): number => asked.mock.contexts.filter((context) => context === element).length;
    expect([layer, drawn, ...drawn.children].map(askedOf)).toEqual([0, 0, 0, 0]);
    expect(askedOf(plain)).toBe(1);
    expect(askedOf(outside)).toBeGreaterThan(1);
    expect(survey(layer, { frame: layer })).toEqual({ kind: "surveyed", width: 1000, height: 700, surfaces: [{ id: FLOOR, x0: 0, x1: 1000, y: 700 }], keepouts: [], walls: [], fixtures: [] });
  });
});
