/** 🤏️ The learner's hand on the pets: a press on a pet with nothing interactive beneath it is the pets' own — nothing beneath it acts, it is followed wherever the pointer goes and ends in a release or a cancellation — while a control under a pet keeps its press and its click, text selection works beside and around the pets, Escape calls a held pet off, and the hand leaves nothing behind.
 *
 * The browser is played by `@testing-library/user-event`, which models what a real pointer and keyboard do: a
 * cancelled `pointerdown` suppresses the compatible `mousedown` and with it focus and the start of a text selection,
 * but not the click, which goes to the nearest common ancestor of where the press began and ended; a press that is not
 * cancelled focuses what it lands on and selects text as it moves. The hand's verdicts are held against what that
 * model lets the page see.
 *
 * The layer's part — what of the hand reaches the stage, in the stage's own pixels, when, and what the layer does with
 * the frame's verdict (the grabbing cursor, the touch pads over grounded pets, letting go) — runs against a fake core:
 * `advance` notes every event it is handed and `frameOf` shows the actors a case puts on stage, so these cases hold
 * the shell's contract whatever the stage does with the events.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🤏️grasp/🟦️.ts
 * @see ../../🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx
 * @see https://testing-library.com/docs/user-event/pointer
 * @see https://www.w3.org/TR/pointerevents3/#the-pointerdown-event
 */

import { cleanup, render } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRef } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { menagerieIssues, type ActorFrame, type Footing, type Frame, type LiftFrame, type Menagerie, type Species, type Stage, type StageEvent } from "@semio-tech/pets";
import { PET_CONTROLS, PET_CURSOR, PET_HELD, PET_PRESS_CONTROLS, PetLayer, STIR_MILLISECONDS, fixtureId, watchGrasp, type Grasp, type Grasped, type GraspOptions, type PetLayerHandle } from "@semio-tech/pets-react";

const fake = vi.hoisted(() => ({ heard: [] as StageEvent[], steps: [] as (readonly StageEvent[])[], actors: [] as ActorFrame[], held: null as string | null, rate: 0 as Frame["rate"], lifts: [] as LiftFrame[] }));

vi.mock("@semio-tech/pets", async (original) => {
  const real = await original<Record<string, unknown>>();
  return {
    ...real,
    openStage: (seed: number) => ({ seed, tick: 0 }),
    advance: (_menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]) => {
      fake.heard.push(...events);
      fake.steps.push(events);
      return { ...stage, tick: stage.tick + events.reduce((ticks, event) => ticks + (event.kind === "ticked" ? event.ticks : 0), 0) };
    },
    frameOf: (_menagerie: Menagerie, stage: Stage): Frame => ({ tick: stage.tick, actors: fake.actors, rate: fake.rate, wake: null, ladders: [], particles: [], lifts: fake.lifts, puffs: [], held: fake.held }),
  };
});

const css = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/🎨️.css"), "utf8");

//#region 🔖️Page
const BODY = { x: 100, y: 200, width: 40, height: 48 } as const;
const ON = { clientX: 120, clientY: 230 } as const;
const OFF = { clientX: 400, clientY: 230 } as const;

/** 🐾️ Whether a position lies in the body of the one pet of these pages. */
function onPet(x: number, y: number): boolean {
  return x >= BODY.x && x <= BODY.x + BODY.width && y >= BODY.y && y <= BODY.y + BODY.height;
}

/** 🏗️ A page with words beside the pet and, under it, a card, a button, a label with its box and a drag grip of the host; positions are given with every event, since jsdom lays nothing out. */
function page(): { readonly words: HTMLElement; readonly card: HTMLElement; readonly button: HTMLButtonElement; readonly label: HTMLLabelElement; readonly box: HTMLInputElement; readonly grip: HTMLElement } {
  document.body.innerHTML =
    '<main><p id="words">Words beside the pet that a learner may select.</p><section id="card">Card text under the pet</section>' +
    '<button id="go" type="button"><span id="go-label">Go</span></button><label id="agree"><input id="tick" type="checkbox"> Agree</label><span id="grip" data-quiz-grip="">⠿</span></main>';
  const find = <T extends HTMLElement>(id: string): T => document.getElementById(id) as T;
  return { words: find("words"), card: find("card"), button: find("go"), label: find("agree"), box: find("tick"), grip: find("grip") };
}

/** 🖱️ A pointer event with the fields the hand reads, built on `MouseEvent` because jsdom has no `PointerEvent`. */
function pointer(type: string, init: MouseEventInit & { readonly pointerId?: number; readonly pointerType?: string; readonly isPrimary?: boolean } = {}): MouseEvent {
  const event = new MouseEvent(type, { bubbles: true, cancelable: true, button: 0, ...init });
  Object.defineProperty(event, "pointerId", { value: init.pointerId ?? 7 });
  Object.defineProperty(event, "pointerType", { value: init.pointerType ?? "mouse" });
  Object.defineProperty(event, "isPrimary", { value: init.isPrimary ?? true });
  return event;
}

/** 📒️ Records every listener added to a target and not removed again, with its options. */
function ledger(target: EventTarget): { readonly open: () => string[]; readonly options: () => Record<string, unknown> } {
  const open: { readonly type: string; readonly listener: unknown; readonly capture: boolean; readonly options: unknown }[] = [];
  const capture = (options: unknown): boolean => (typeof options === "boolean" ? options : ((options as AddEventListenerOptions | undefined)?.capture ?? false));
  const add = target.addEventListener.bind(target);
  const remove = target.removeEventListener.bind(target);
  vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | AddEventListenerOptions) => {
    open.push({ type, listener, capture: capture(options), options });
    add(type, listener, options);
  });
  vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | EventListenerOptions) => {
    const index = open.findIndex((entry) => entry.type === type && entry.listener === listener && entry.capture === capture(options));
    if (index >= 0) open.splice(index, 1);
    remove(type, listener, options);
  });
  return { open: () => open.map((entry) => entry.type).sort(), options: () => Object.fromEntries(open.map((entry) => [entry.type, entry.options])) };
}

/** 📌️ Gives the document element the pointer capture jsdom lacks and records what was asked of it. */
function capturing(): { readonly set: ReturnType<typeof vi.fn>; readonly released: ReturnType<typeof vi.fn>; readonly held: () => number | null } {
  const root = document.documentElement;
  let holder: number | null = null;
  const set = vi.fn((id: number) => {
    holder = id;
  });
  const released = vi.fn((id: number) => {
    if (holder === id) holder = null;
  });
  Object.assign(root, { setPointerCapture: set, releasePointerCapture: released, hasPointerCapture: (id: number) => holder === id });
  return { set, released, held: () => holder };
}
//#endregion 🔖️Page

let hand: Grasp | null = null;
let heard: Grasped[] = [];

/** 🫴️ Lends the pets of the page a hand; what it tells lands in `heard`. */
function lend(options: GraspOptions = {}): Grasp {
  hand = watchGrasp(window, (event) => heard.push(event), { takes: onPet, now: () => 0, ...options });
  return hand;
}

const pressed = (): Grasped[] => heard.filter((event) => event.kind !== "stirred");

beforeEach(() => {
  heard = [];
});

afterEach(() => {
  hand?.stop();
  hand = null;
  document.body.innerHTML = "";
  document.getSelection()?.removeAllRanges();
  for (const name of ["setPointerCapture", "releasePointerCapture", "hasPointerCapture"]) Reflect.deleteProperty(document.documentElement, name);
  vi.restoreAllMocks();
});

describe("🤏️ pet handling", () => {
  it("counts as controls what the survey does and labels, which hand their press on", () => {
    expect(PET_PRESS_CONTROLS.startsWith(PET_CONTROLS)).toBe(true);
    expect(PET_PRESS_CONTROLS.slice(PET_CONTROLS.length)).toBe(", label");
    expect(PET_CURSOR).toBe("data-pet-cursor");
  });

  it("takes a press on a pet with nothing interactive beneath it: nothing beneath it acts, and the click it leads to is swallowed", async () => {
    const { card } = page();
    const { set, released, held } = capturing();
    const seen = { document: vi.fn(), card: vi.fn(), mouse: vi.fn(), click: vi.fn(), page: vi.fn() };
    document.addEventListener("pointerdown", seen.document, true);
    card.addEventListener("pointerdown", seen.card);
    card.addEventListener("mousedown", seen.mouse);
    card.addEventListener("click", seen.click);
    document.addEventListener("click", seen.page, true);
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer({ keys: "[MouseLeft>]", target: card, coords: ON });
    expect(heard).toEqual([{ kind: "stirred" }, { kind: "pressed", x: 120, y: 230, pointer: "mouse" }]);
    expect(set).toHaveBeenCalledTimes(1);
    const id = set.mock.calls[0]![0] as number;
    expect(held()).toBe(id);
    expect(document.activeElement).toBe(document.body);
    await user.pointer({ keys: "[/MouseLeft]", target: card, coords: ON });
    expect(pressed()).toEqual([
      { kind: "pressed", x: 120, y: 230, pointer: "mouse" },
      { kind: "released", x: 120, y: 230 },
    ]);
    expect(released).toHaveBeenCalledWith(id);
    expect(held()).toBeNull();
    for (const [name, spy] of Object.entries(seen)) expect(spy, name).not.toHaveBeenCalled();
    expect(document.getSelection()?.isCollapsed ?? true).toBe(true);

    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: OFF }, { keys: "[/MouseLeft]" }]);
    expect(pressed()).toHaveLength(2);
    for (const [name, spy] of Object.entries(seen)) expect(spy, `${name} hears a press beside the pet`).toHaveBeenCalledTimes(1);
  });

  it("leaves a press on a control under a pet to the control, which receives its click and the focus", async () => {
    const { button, label, box, grip } = page();
    const clicked = vi.fn();
    button.addEventListener("click", clicked);
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: document.getElementById("go-label")!, coords: ON }, { keys: "[/MouseLeft]" }]);
    expect(clicked).toHaveBeenCalledTimes(1);
    expect(document.activeElement).toBe(button);
    await user.pointer([{ keys: "[MouseLeft>]", target: label, coords: ON }, { keys: "[/MouseLeft]" }]);
    expect(box.checked).toBe(true);
    const gripped = vi.fn();
    grip.addEventListener("pointerdown", gripped);
    hand!.stop();
    lend({ controls: () => `${PET_PRESS_CONTROLS}, [data-quiz-grip]` });
    await user.pointer([{ keys: "[MouseLeft>]", target: grip, coords: ON }, { keys: "[/MouseLeft]" }]);
    expect(gripped).toHaveBeenCalledTimes(1);
    expect(pressed()).toEqual([]);
    hand!.stop();
    lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: grip, coords: ON }, { keys: "[/MouseLeft]" }]);
    expect(gripped, "a grip the host does not name is no control").toHaveBeenCalledTimes(1);
    expect(pressed().map((event) => event.kind)).toEqual(["pressed", "released"]);
  });

  it("never breaks a text selection: text beside a pet selects, a selection holds back the next press, and a taken press selects nothing", async () => {
    const { words, card } = page();
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: words, offset: 0 }, { offset: 12 }, { keys: "[/MouseLeft]" }]);
    expect(document.getSelection()!.toString()).toBe("Words beside");
    expect(pressed()).toEqual([]);
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON, offset: 3 }, { keys: "[/MouseLeft]" }]);
    expect(pressed(), "the press goes to the selection").toEqual([]);
    expect(document.getSelection()!.isCollapsed).toBe(true);
    const caret = { node: document.getSelection()!.anchorNode, offset: document.getSelection()!.anchorOffset };
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON, offset: 9 }, { coords: { clientX: 160, clientY: 260 }, offset: 14 }, { keys: "[/MouseLeft]" }]);
    expect(pressed().map((event) => event.kind)).toEqual(["pressed", "dragged", "released"]);
    expect({ node: document.getSelection()!.anchorNode, offset: document.getSelection()!.anchorOffset }).toEqual(caret);
    expect(document.getSelection()!.isCollapsed).toBe(true);
  });

  it("follows a held press wherever the pointer goes, captured on the document element, and lets it go where it ends", async () => {
    const { card } = page();
    const { set, held } = capturing();
    const user = userEvent.setup({ delay: null });
    const grasp = lend({ origin: () => ({ x: 10, y: 20 }), takes: (x, y) => onPet(x + 10, y + 20) });
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { coords: { clientX: 130, clientY: 240 } }, { coords: { clientX: 900, clientY: -40 } }]);
    expect(held()).toBe(set.mock.calls[0]![0]);
    grasp.frame(true);
    expect(document.documentElement.getAttribute(PET_CURSOR)).toBe("grabbing");
    await user.pointer({ keys: "[/MouseLeft]", coords: { clientX: 905, clientY: -38 } });
    expect(pressed()).toEqual([
      { kind: "pressed", x: 110, y: 210, pointer: "mouse" },
      { kind: "dragged", x: 120, y: 220 },
      { kind: "dragged", x: 890, y: -60 },
      { kind: "released", x: 895, y: -58 },
    ]);
    expect(held()).toBeNull();
    grasp.frame(false);
    expect(document.documentElement.hasAttribute(PET_CURSOR)).toBe(false);
  });

  it("tells a quick press and release on a pet as they are: whether it was a click is the stage's to decide", async () => {
    const { card } = page();
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { keys: "[/MouseLeft]" }]);
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { keys: "[/MouseLeft]" }]);
    expect(pressed()).toEqual([
      { kind: "pressed", x: 120, y: 230, pointer: "mouse" },
      { kind: "released", x: 120, y: 230 },
      { kind: "pressed", x: 120, y: 230, pointer: "mouse" },
      { kind: "released", x: 120, y: 230 },
    ]);
  });

  it("calls a held pet off with Escape, keeps that Escape and the click after it from the page, and leaves every other Escape to the page", async () => {
    const { card } = page();
    const keys = vi.fn();
    const clicks = vi.fn();
    document.addEventListener("keydown", keys);
    card.addEventListener("click", clicks);
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { coords: { clientX: 150, clientY: 260 } }]);
    const prevented: boolean[] = [];
    window.addEventListener("keydown", (event) => prevented.push(event.defaultPrevented));
    await user.keyboard("{Escape}");
    expect(pressed()).toEqual([{ kind: "pressed", x: 120, y: 230, pointer: "mouse" }, { kind: "dragged", x: 150, y: 260 }, { kind: "cancelled" }]);
    expect(keys).not.toHaveBeenCalled();
    await user.pointer({ keys: "[/MouseLeft]", coords: { clientX: 150, clientY: 260 } });
    expect(pressed()).toHaveLength(3);
    expect(clicks, "the click of a called-off press is swallowed too").not.toHaveBeenCalled();
    await user.keyboard("{Escape}");
    expect(keys).toHaveBeenCalledTimes(1);
    expect(keys.mock.calls[0]![0].defaultPrevented).toBe(false);
    expect(pressed()).toHaveLength(3);
  });

  it("calls a press off when the browser takes the pointer over, capture is lost, the window loses focus or the host asks — and never twice", () => {
    const { card, button } = page();
    lend();
    const press = (id: number): void => void card.dispatchEvent(pointer("pointerdown", { ...ON, pointerId: id }));
    for (const [id, end] of [
      [1, () => window.dispatchEvent(pointer("pointercancel", { pointerId: 1 }))],
      [2, () => document.documentElement.dispatchEvent(pointer("lostpointercapture", { pointerId: 2 }))],
      [3, () => window.dispatchEvent(new FocusEvent("blur"))],
      [4, () => hand!.cancel()],
    ] as const) {
      press(id);
      window.dispatchEvent(pointer("pointercancel", { pointerId: id + 100 }));
      button.dispatchEvent(new FocusEvent("blur"));
      end();
      end();
      window.dispatchEvent(pointer("pointerup", { ...ON, pointerId: id }));
    }
    expect(pressed()).toEqual(Array.from({ length: 4 }, () => [{ kind: "pressed", x: 120, y: 230, pointer: "mouse" }, { kind: "cancelled" }]).flat());
  });

  it("takes no press that is not the pets': another button, a modifier, a second finger, a miss, or play that is not permitted", async () => {
    const { card } = page();
    const user = userEvent.setup({ delay: null });
    let permitted = false;
    lend({ takes: (x, y) => permitted && onPet(x, y) });
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { keys: "[/MouseLeft]" }]);
    permitted = true;
    await user.pointer([{ keys: "[MouseRight>]", target: card, coords: ON }, { keys: "[/MouseRight]" }]);
    await user.keyboard("{Shift>}");
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { keys: "[/MouseLeft]" }]);
    await user.keyboard("{/Shift}");
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: OFF }, { keys: "[/MouseLeft]" }]);
    expect(pressed()).toEqual([]);
    await user.pointer([{ keys: "[TouchA>]", target: card, coords: OFF }, { keys: "[TouchB>]", pointerName: "TouchB", target: card, coords: ON }, { keys: "[/TouchB]", pointerName: "TouchB" }, { keys: "[/TouchA]", pointerName: "TouchA" }]);
    expect(pressed(), "a second finger is no press of its own").toEqual([]);
    const early = pointer("pointerdown", ON);
    early.preventDefault();
    card.dispatchEvent(early);
    expect(pressed(), "a press somebody already cancelled").toEqual([]);
  });

  it("taps with a finger like with a mouse, and swallows the tap's click", async () => {
    const { card } = page();
    const clicks = vi.fn();
    card.addEventListener("click", clicks);
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer([{ keys: "[TouchA>]", target: card, coords: ON }, { keys: "[/TouchA]", target: card, coords: ON }]);
    expect(pressed()).toEqual([
      { kind: "pressed", x: 120, y: 230, pointer: "touch" },
      { kind: "released", x: 120, y: 230 },
    ]);
    expect(clicks).not.toHaveBeenCalled();
    expect(document.documentElement.hasAttribute(PET_CURSOR)).toBe(false);
  });

  it("swallows no click from the keyboard, also after a press that ended where no click followed", async () => {
    const { card, button } = page();
    const clicked = vi.fn();
    button.addEventListener("click", clicked);
    const user = userEvent.setup({ delay: null });
    lend();
    card.dispatchEvent(pointer("pointerdown", ON));
    document.documentElement.dispatchEvent(pointer("pointerup", { clientX: 2000, clientY: 230 }));
    button.focus();
    await user.keyboard("{Enter}");
    expect(clicked).toHaveBeenCalledTimes(1);
    expect(pressed().map((event) => event.kind)).toEqual(["pressed", "released"]);
  });

  it("lets go quietly when the stage let go of the pet by itself, and still swallows the click", async () => {
    const { card } = page();
    const clicks = vi.fn();
    card.addEventListener("click", clicks);
    const { held } = capturing();
    const user = userEvent.setup({ delay: null });
    const grasp = lend();
    await user.pointer([{ keys: "[MouseLeft>]", target: card, coords: ON }, { coords: { clientX: 150, clientY: 250 } }]);
    grasp.frame(true);
    grasp.frame(false);
    expect(held()).toBeNull();
    await user.pointer([{ coords: { clientX: 170, clientY: 250 } }, { keys: "[/MouseLeft]" }]);
    expect(pressed()).toEqual([{ kind: "pressed", x: 120, y: 230, pointer: "mouse" }, { kind: "dragged", x: 150, y: 250 }]);
    expect(clicks).not.toHaveBeenCalled();
  });

  it("calls an open press off for a new one", () => {
    const { card } = page();
    lend();
    card.dispatchEvent(pointer("pointerdown", { ...ON, pointerId: 1 }));
    card.dispatchEvent(pointer("pointerdown", { clientX: 125, clientY: 235, pointerId: 2, pointerType: "pen" }));
    expect(pressed()).toEqual([{ kind: "pressed", x: 120, y: 230, pointer: "mouse" }, { kind: "cancelled" }, { kind: "pressed", x: 125, y: 235, pointer: "pen" }]);
  });

  it("shows the hand on the document element by an attribute, never by a class: grab over a pet that can be picked up, grabbing while one is held", async () => {
    const { card, button } = page();
    const root = document.documentElement;
    const classes = root.className;
    const user = userEvent.setup({ delay: null });
    let here = true;
    const grasp = lend({ takes: (x, y) => here && onPet(x, y) });
    await user.pointer({ target: card, coords: ON });
    expect(root.getAttribute(PET_CURSOR)).toBe("grab");
    expect([card.style.getPropertyValue("cursor"), card.style.getPropertyPriority("cursor")]).toEqual(["grab", "important"]);
    await user.pointer({ target: button, coords: { clientX: 121, clientY: 231 } });
    expect(root.hasAttribute(PET_CURSOR), "nothing over a control").toBe(false);
    expect([card.hasAttribute("style"), button.hasAttribute("style")]).toEqual([false, false]);
    await user.pointer({ target: card, coords: OFF });
    expect(root.hasAttribute(PET_CURSOR)).toBe(false);
    await user.pointer({ target: card, coords: ON });
    expect(root.getAttribute(PET_CURSOR)).toBe("grab");
    here = false;
    grasp.frame(false);
    expect(root.hasAttribute(PET_CURSOR), "the pet walked away from the resting pointer").toBe(false);
    here = true;
    grasp.frame(false);
    await user.pointer({ keys: "[MouseLeft>]", target: card, coords: ON });
    expect(root.getAttribute(PET_CURSOR)).toBe("grab");
    expect(root.hasAttribute(PET_HELD)).toBe(false);
    grasp.frame(true);
    expect(root.getAttribute(PET_CURSOR)).toBe("grabbing");
    expect(root.getAttribute(PET_HELD), "while a pet is held the stylesheet shows the hand on every element").toBe("");
    expect(card.hasAttribute("style")).toBe(false);
    await user.pointer({ keys: "[/MouseLeft]" });
    grasp.frame(false);
    expect(root.getAttribute(PET_CURSOR)).toBe("grab");
    expect(root.hasAttribute(PET_HELD)).toBe(false);
    await user.pointer([{ keys: "[TouchA>]", target: card, coords: OFF }, { coords: ON }, { keys: "[/TouchA]" }]);
    grasp.stop();
    expect(root.hasAttribute(PET_CURSOR)).toBe(false);
    expect(root.className).toBe(classes);
    expect(document.querySelectorAll("[style]")).toHaveLength(0);
  });

  it("shows grab on the one element under the pointer and gives that element's own cursor back exactly, so a pointer passing a pet restyles one element, not the page", async () => {
    const { card, words } = page();
    words.style.setProperty("cursor", "help");
    const before = document.body.innerHTML;
    const user = userEvent.setup({ delay: null });
    lend();
    await user.pointer({ target: words, coords: ON });
    expect([words.style.getPropertyValue("cursor"), words.style.getPropertyPriority("cursor")]).toEqual(["grab", "important"]);
    await user.pointer({ target: card, coords: ON });
    expect([words.style.getPropertyValue("cursor"), words.style.getPropertyPriority("cursor"), card.style.getPropertyValue("cursor")]).toEqual(["help", "", "grab"]);
    await user.pointer({ target: card, coords: OFF });
    expect(document.body.innerHTML).toBe(before);
    const rules = css.replace(/\/\*[\s\S]*?\*\//gu, "");
    expect(rules, "no rule names the mark that changes whenever the pointer passes a pet").not.toContain(PET_CURSOR);
    expect(document.documentElement.hasAttribute(PET_HELD)).toBe(false);
  });

  it("tells the stage that the learner stirs, at most once a second, and that the page scrolls", async () => {
    const { card, words } = page();
    let time = 0;
    const user = userEvent.setup({ delay: null });
    lend({ now: () => time });
    await user.keyboard("a");
    await user.keyboard("b");
    time = STIR_MILLISECONDS - 1;
    await user.pointer([{ keys: "[MouseLeft>]", target: words, coords: OFF }, { keys: "[/MouseLeft]" }]);
    time = STIR_MILLISECONDS;
    card.dispatchEvent(new WheelEvent("wheel", { bubbles: true }));
    time = 2 * STIR_MILLISECONDS;
    words.dispatchEvent(new InputEvent("input", { bubbles: true }));
    time = 3 * STIR_MILLISECONDS;
    await user.pointer([{ keys: "[TouchA>]", target: words, coords: OFF }, { keys: "[/TouchA]" }]);
    time = 4 * STIR_MILLISECONDS;
    await user.pointer({ target: words, coords: ON });
    expect(heard).toEqual([{ kind: "stirred" }, { kind: "stirred" }, { kind: "stirred" }, { kind: "stirred" }]);
    heard = [];
    card.dispatchEvent(new Event("scroll"));
    document.dispatchEvent(new Event("scroll"));
    expect(heard).toEqual([{ kind: "scrolled" }, { kind: "scrolled" }]);
  });

  it("cancels the start of a text selection, a native drag and a context menu only while it holds a press", () => {
    const { card } = page();
    lend();
    const attempt = (type: string): boolean => {
      const event = new Event(type, { bubbles: true, cancelable: true });
      card.dispatchEvent(event);
      return event.defaultPrevented;
    };
    expect(["selectstart", "dragstart", "contextmenu"].map(attempt)).toEqual([false, false, false]);
    card.dispatchEvent(pointer("pointerdown", ON));
    expect(["selectstart", "dragstart", "contextmenu"].map(attempt)).toEqual([true, true, true]);
    window.dispatchEvent(pointer("pointerup", ON));
    expect(["selectstart", "dragstart", "contextmenu"].map(attempt)).toEqual([false, false, false]);
  });

  it("listens in the capture phase of the window and the document, cancels only where it must, and leaves nothing behind — also mid-press", () => {
    const { card } = page();
    const { held } = capturing();
    const view = ledger(window);
    const doc = ledger(document);
    const grasp = lend();
    expect(view.open()).toEqual(["blur", "click", "contextmenu", "dragstart", "input", "keydown", "lostpointercapture", "pointercancel", "pointerdown", "pointermove", "pointerup", "selectstart", "wheel"]);
    expect(doc.open()).toEqual(["scroll"]);
    const active = { capture: true, passive: false };
    const passive = { capture: true, passive: true };
    expect(view.options()).toEqual({
      pointerdown: active,
      pointermove: passive,
      pointerup: passive,
      pointercancel: passive,
      lostpointercapture: passive,
      keydown: active,
      wheel: passive,
      input: passive,
      blur: { capture: false, passive: true },
      click: active,
      selectstart: active,
      dragstart: active,
      contextmenu: active,
    });
    expect(doc.options()).toEqual({ scroll: passive });
    card.dispatchEvent(pointer("pointermove", ON));
    card.dispatchEvent(pointer("pointerdown", ON));
    grasp.frame(true);
    expect(held()).toBe(7);
    expect(document.documentElement.getAttribute(PET_CURSOR)).toBe("grabbing");
    grasp.stop();
    grasp.stop();
    expect(view.open()).toEqual([]);
    expect(doc.open()).toEqual([]);
    expect(held()).toBeNull();
    expect(document.documentElement.hasAttribute(PET_CURSOR)).toBe(false);
    const told = heard.length;
    card.dispatchEvent(pointer("pointermove", { clientX: 130, clientY: 240 }));
    window.dispatchEvent(pointer("pointerup", ON));
    grasp.cancel();
    grasp.frame(true);
    expect(heard).toHaveLength(told);
    expect(document.documentElement.hasAttribute(PET_CURSOR)).toBe(false);
  });
});

//#region 🔖️Stage
/** 🧬️ A species of two bones whose frames the fake core may show. */
function specimen(id: string): Species {
  return {
    id,
    name: { en: id, de: id },
    thing: { en: "test rig", de: "Prüfgerüst" },
    grounds: [],
    size: { width: 40, height: 44 },
    palette: { body: "#1e9b8d", accent: "#f7f3e3", detail: "#001117" },
    bones: [
      { id: "root", x: 0, y: 0 },
      { id: "body", parent: "root", x: 0, y: -22 },
    ],
    parts: [{ id: "trunk", bone: "body", shape: { kind: "ellipse", cx: 0, cy: 0, rx: 18, ry: 20 }, fill: "body", stroke: "ink" }],
    face: { eyes: [{ id: "eye", bone: "body", x: 0, y: -4, radius: 4, pupil: 1.8 }], mouth: { bone: "body", x: 0, y: 7, width: 8 } },
    clips: [
      { id: "idle", seconds: 3, loop: true, tracks: [] },
      { id: "walk", seconds: 0.6, loop: true, tracks: [] },
      { id: "wiggle", seconds: 0.8, loop: false, tracks: [] },
    ],
    repertoire: { idle: ["idle"], walk: ["walk"], fidget: ["wiggle"], greet: ["wiggle"], cuddle: ["wiggle"], squabble: ["wiggle"], sulk: ["idle"], sleep: ["idle"], land: ["wiggle"], hang: ["idle"], tumble: ["wiggle"], purr: ["idle"], dizzy: ["wiggle"], shrug: ["wiggle"], push: ["walk"] },
    locomotion: { gait: "walk", speed: 40 },
    temperament: { energy: 0.6, sociability: 0.6, curiosity: 0.6 },
    states: [{ id: "resting", name: { en: "Resting", de: "In Ruhe" } }],
    tricks: [],
    purr: { clip: "idle" },
    emitters: [],
    gear: [],
    grip: 40,
    reach: 10,
    mood: "content",
  };
}

const STAGED: Menagerie = { schema: "semio.pets.menagerie/v1", id: "hands", title: { en: "Hands", de: "Hände" }, species: [specimen("alba"), specimen("bruno")], bonds: [], casts: [{ scene: "home", core: ["alba", "bruno"], rotation: [] }], chemistry: [] };

/** 🧍️ An actor as the fake core shows it: feet at (`x`, `y`) in stage pixels, its solid box 48 × 52 about them, reaching 4 px below the feet. */
function actor(species: string, x: number, y: number, footing: Footing = "perch"): ActorFrame {
  return { species, x, y, facing: 1, activity: "idle", opacity: 1, bones: [], eyes: [{ x: 0, y: 0, lid: 0 }], footing, state: "resting", mood: "content", intensity: 0, spirits: 0, tilt: 0, pivot: { x: 0, y: -40 }, tools: [], body: { x: x - 24, y: y - 48, width: 48, height: 52 } };
}

/** 🗺️ A page of one card under the layer, in a viewport of 1200 × 800. */
function stagePage(): { readonly card: HTMLElement; readonly button: HTMLButtonElement } {
  Object.defineProperty(document.documentElement, "clientWidth", { configurable: true, value: 1200 });
  Object.defineProperty(document.documentElement, "clientHeight", { configurable: true, value: 800 });
  const card = document.createElement("article");
  card.setAttribute("data-pet-surface", "");
  card.getBoundingClientRect = () => ({ left: 100, top: 300, width: 400, height: 200, x: 100, y: 300, right: 500, bottom: 500, toJSON: () => null }) as DOMRect;
  const button = document.createElement("button");
  document.body.append(card, button);
  return { card, button };
}

/** 📜️ What the fake core heard of a kind, or of the hand's kinds when none is named. */
function told(...kinds: string[]): StageEvent[] {
  const wanted = kinds.length > 0 ? kinds : ["pressed", "dragged", "released", "cancelled", "played"];
  return fake.heard.filter((event) => wanted.includes(event.kind));
}
//#endregion 🔖️Stage

describe("🫳️ the layer's hand", () => {
  const spoken = ["log", "info", "warn", "error", "debug", "trace"] as const;
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "requestAnimationFrame", "cancelAnimationFrame", "performance"] });
    for (const method of spoken) vi.spyOn(console, method).mockImplementation(() => {});
    fake.heard = [];
    fake.steps = [];
    fake.actors = [actor("alba", 100, 150), actor("bruno", 300, 150)];
    fake.held = null;
    fake.rate = 0;
    fake.lifts = [];
  });

  afterEach(() => {
    cleanup();
    for (const method of spoken) expect(vi.mocked(console[method]).mock.calls, `console.${method}`).toEqual([]);
    expect(vi.getTimerCount()).toBe(0);
    Reflect.deleteProperty(document.documentElement, "clientWidth");
    Reflect.deleteProperty(document.documentElement, "clientHeight");
    document.head.replaceChildren();
    vi.unstubAllGlobals();
    vi.useRealTimers();
  });

  it("stages a menagerie the core accepts", () => {
    expect(menagerieIssues(STAGED)).toEqual([]);
  });

  it("tells the stage at once what the learner permits, and again whenever that changes", () => {
    stagePage();
    const { rerender } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    expect(fake.heard.slice(0, 3)).toEqual([
      { kind: "tuned", mode: "calm" },
      { kind: "hushed", quiet: false },
      { kind: "permitted", play: true, mischief: true },
    ]);
    vi.advanceTimersByTime(100);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} play={false} />);
    vi.advanceTimersByTime(100);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} play={false} mischief={false} />);
    vi.advanceTimersByTime(100);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="lively" seed={1} play={false} mischief={false} />);
    vi.advanceTimersByTime(100);
    expect(told("permitted")).toEqual([
      { kind: "permitted", play: true, mischief: true },
      { kind: "permitted", play: false, mischief: true },
      { kind: "permitted", play: false, mischief: false },
    ]);
  });

  it("hands the stage a press on a pet's body, the latest drag of each frame and the release, in the stage's own pixels", () => {
    const { card } = stagePage();
    render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} scale={2} />);
    vi.advanceTimersByTime(100);
    const down = pointer("pointerdown", { clientX: 200, clientY: 260 });
    card.dispatchEvent(down);
    expect(down.defaultPrevented).toBe(true);
    for (const [x, y] of [[204, 262], [210, 270], [220, 280]] as const) card.dispatchEvent(pointer("pointermove", { clientX: x, clientY: y, buttons: 1 }));
    vi.advanceTimersByTime(20);
    card.dispatchEvent(pointer("pointermove", { clientX: 240, clientY: 300, buttons: 1 }));
    window.dispatchEvent(pointer("pointerup", { clientX: 240, clientY: 300 }));
    vi.advanceTimersByTime(20);
    expect(told()).toEqual([
      { kind: "pressed", x: 100, y: 130, pointer: "mouse" },
      { kind: "dragged", x: 110, y: 140 },
      { kind: "dragged", x: 120, y: 150 },
      { kind: "released", x: 120, y: 150 },
    ]);
    const miss = pointer("pointerdown", { clientX: 400, clientY: 260 });
    card.dispatchEvent(miss);
    window.dispatchEvent(pointer("pointerup", { clientX: 400, clientY: 260 }));
    vi.advanceTimersByTime(20);
    expect(miss.defaultPrevented).toBe(false);
    expect(told()).toHaveLength(4);
  });

  it("leaves a press on a control under a pet to the control, and takes nothing on a still stage or while play is off", () => {
    const { card, button } = stagePage();
    const clicked = vi.fn();
    button.addEventListener("click", clicked);
    const { rerender } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    const onControl = pointer("pointerdown", { clientX: 100, clientY: 130 });
    button.dispatchEvent(onControl);
    button.dispatchEvent(pointer("pointerup", { clientX: 100, clientY: 130 }));
    button.click();
    expect(onControl.defaultPrevented).toBe(false);
    expect(clicked).toHaveBeenCalledTimes(1);
    for (const [mode, play] of [["still", true], ["calm", false]] as const) {
      rerender(<PetLayer menagerie={STAGED} scene="home" mode={mode} seed={1} play={play} />);
      vi.advanceTimersByTime(100);
      const press = pointer("pointerdown", { clientX: 100, clientY: 130 });
      card.dispatchEvent(press);
      window.dispatchEvent(pointer("pointerup", { clientX: 100, clientY: 130 }));
      expect(press.defaultPrevented, mode).toBe(false);
    }
    vi.advanceTimersByTime(100);
    expect(told()).toEqual([]);
  });

  it("shows the grabbing hand while the frame holds a pet and calls the held pet off when play is switched off, the stage goes still, the document hides or the layer goes", () => {
    const { card } = stagePage();
    const root = document.documentElement;
    const press = (): void => {
      card.dispatchEvent(pointer("pointerdown", { clientX: 100, clientY: 130 }));
      fake.held = "alba";
      card.dispatchEvent(pointer("pointermove", { clientX: 120, clientY: 120, buttons: 1 }));
      vi.advanceTimersByTime(20);
      expect(root.getAttribute(PET_CURSOR)).toBe("grabbing");
    };
    const { rerender, unmount } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    press();
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} play={false} />);
    fake.held = null;
    vi.advanceTimersByTime(20);
    expect(root.hasAttribute(PET_CURSOR)).toBe(false);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(20);
    press();
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="still" seed={1} />);
    fake.held = null;
    vi.advanceTimersByTime(20);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(20);
    press();
    let state: DocumentVisibilityState = "hidden";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    document.dispatchEvent(new Event("visibilitychange"));
    state = "visible";
    fake.held = null;
    document.dispatchEvent(new Event("visibilitychange"));
    vi.advanceTimersByTime(20);
    press();
    unmount();
    expect(root.hasAttribute(PET_CURSOR)).toBe(false);
    expect(told().map((event) => event.kind)).toEqual(["pressed", "dragged", "cancelled", "pressed", "dragged", "cancelled", "pressed", "dragged", "cancelled", "pressed", "dragged"]);
  });

  it("hands the stage the deeds a host asks for through its handle, and nothing once it is gone", () => {
    stagePage();
    const hand = createRef<PetLayerHandle>();
    const { unmount } = render(<PetLayer ref={hand} menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    hand.current!.play("alba", "trick");
    hand.current!.play("bruno", "toss");
    vi.advanceTimersByTime(20);
    expect(told("played")).toEqual([
      { kind: "played", species: "alba", deed: "trick" },
      { kind: "played", species: "bruno", deed: "toss" },
    ]);
    const handle = hand.current!;
    unmount();
    expect(hand.current).toBeNull();
    handle.play("alba", "hello");
    expect(told("played")).toHaveLength(2);
  });

  it("puts a touch pad over every grounded pet on a device with a coarse pointer while play is permitted, and nowhere else", () => {
    const { card } = stagePage();
    const sheet = document.createElement("style");
    sheet.textContent = css;
    document.head.append(sheet);
    let coarse = false;
    vi.stubGlobal("matchMedia", (query: string) => ({
      get matches() {
        return query === "(pointer: coarse)" && coarse;
      },
      media: query,
      addEventListener: () => {},
      removeEventListener: () => {},
    }));
    fake.actors = [actor("alba", 100, 150), actor("bruno", 300, 100, "air")];
    const { container, rerender, unmount } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} scale={2} />);
    vi.advanceTimersByTime(100);
    const layer = container.querySelector(".pet-layer")!;
    expect(layer.querySelectorAll(".pet-pad")).toHaveLength(0);
    coarse = true;
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="lively" seed={1} scale={2} />);
    vi.advanceTimersByTime(100);
    const pads = [...layer.querySelectorAll<HTMLElement>(".pet-pad")];
    expect(pads).toHaveLength(1);
    expect([pads[0]!.style.transform, pads[0]!.style.width, pads[0]!.style.height]).toEqual(["translate(152px, 204px)", "96px", "96px"]);
    expect(getComputedStyle(pads[0]!).pointerEvents).toBe("auto");
    expect(pads[0]!.tabIndex).toBe(-1);
    expect(pads[0]!.closest("[aria-hidden='true']")).toBe(layer);
    for (const node of layer.querySelectorAll("svg, svg *")) expect(getComputedStyle(node).pointerEvents).toBe("none");
    const touch = pointer("pointerdown", { clientX: 200, clientY: 250, pointerType: "touch" });
    pads[0]!.dispatchEvent(touch);
    expect(touch.defaultPrevented).toBe(true);
    window.dispatchEvent(pointer("pointerup", { clientX: 200, clientY: 250, pointerType: "touch" }));
    fake.actors = [actor("alba", 100, 150, "hand"), actor("bruno", 300, 150)];
    vi.advanceTimersByTime(20);
    expect([...layer.querySelectorAll<HTMLElement>(".pet-pad")].map((pad) => pad.style.transform)).toEqual(["translate(552px, 204px)"]);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="lively" seed={1} scale={2} play={false} />);
    vi.advanceTimersByTime(100);
    expect(layer.querySelectorAll(".pet-pad")).toHaveLength(0);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="still" seed={1} scale={2} />);
    vi.advanceTimersByTime(100);
    expect(layer.querySelectorAll(".pet-pad")).toHaveLength(0);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} scale={2} />);
    vi.advanceTimersByTime(100);
    expect(layer.querySelectorAll(".pet-pad")).toHaveLength(1);
    expect(told().map((event) => (event.kind === "pressed" ? event.pointer : event.kind))).toEqual(["touch", "released"]);
    unmount();
    expect(layer.childElementCount).toBe(0);
    expect(card.isConnected).toBe(true);
  });

  it("permits the stage mischief only while the primary pointer is fine, and tells it again whenever that changes", () => {
    stagePage();
    let fine = false;
    const listeners = new Set<() => void>();
    vi.stubGlobal("matchMedia", (query: string) => ({
      get matches() {
        return query === "(pointer: fine)" ? fine : false;
      },
      media: query,
      addEventListener: (_: string, listener: () => void) => query === "(pointer: fine)" && listeners.add(listener),
      removeEventListener: (_: string, listener: () => void) => query === "(pointer: fine)" && listeners.delete(listener),
    }));
    const { rerender, unmount } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    fine = true;
    for (const listener of listeners) listener();
    vi.advanceTimersByTime(100);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} play={false} />);
    vi.advanceTimersByTime(100);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} play={false} mischief={false} />);
    vi.advanceTimersByTime(100);
    expect(told("permitted")).toEqual([
      { kind: "permitted", play: true, mischief: false },
      { kind: "permitted", play: true, mischief: true },
      { kind: "permitted", play: false, mischief: true },
      { kind: "permitted", play: false, mischief: false },
    ]);
    expect(listeners.size).toBe(1);
    unmount();
    expect(listeners.size).toBe(0);
  });

  it("tells the stage of a fixture the learner took back before the survey of the same step, which no longer has it under the focus — so the stage throws the pet off instead of ending the prank quietly", () => {
    const { card } = stagePage();
    const row = document.createElement("div");
    row.setAttribute("data-pet-prop", "quiz/task");
    row.tabIndex = 0;
    row.textContent = "A task";
    row.getBoundingClientRect = () => ({ left: 120, top: 320, width: 200, height: 28, x: 120, y: 320, right: 320, bottom: 348, toJSON: () => null }) as DOMRect;
    card.append(row);
    fake.lifts = [{ fixture: fixtureId(row), dx: 24, dy: 0, tilt: 0, opacity: 1 }];
    render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    expect(told("surveyed").at(-1)).toMatchObject({ fixtures: [{ id: fixtureId(row), key: "quiz/task" }] });
    expect(row.style.getPropertyValue("opacity")).toBe("0");
    fake.lifts = [];
    row.focus();
    window.dispatchEvent(new Event("resize"));
    vi.advanceTimersByTime(100);
    expect(row.style.getPropertyValue("opacity")).toBe("");
    const step = fake.steps.find((events) => events.some((event) => event.kind === "reclaimed"));
    expect(step?.filter((event) => event.kind === "reclaimed" || event.kind === "surveyed").map((event) => (event.kind === "surveyed" ? { kind: event.kind, fixtures: event.fixtures.length } : event))).toEqual([
      { kind: "reclaimed", fixture: fixtureId(row) },
      { kind: "surveyed", fixtures: 0 },
    ]);
  });

  it("measures the page every eight ticks while pets move, but not because a pet is held", () => {
    const { card } = stagePage();
    const reads = vi.spyOn(card, "getBoundingClientRect");
    fake.rate = 64;
    render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(200);
    const count = (milliseconds: number): number => {
      const before = reads.mock.calls.length;
      vi.advanceTimersByTime(milliseconds);
      return reads.mock.calls.length - before;
    };
    const moving = count(1000);
    fake.held = "alba";
    vi.advanceTimersByTime(100);
    const holding = count(1000);
    expect(moving).toBeGreaterThanOrEqual(7);
    expect(holding).toBeLessThanOrEqual(2);
  });

  it("hands the stage one stir and one scroll at a time, and none on a still stage", () => {
    const { card } = stagePage();
    const { rerender } = render(<PetLayer menagerie={STAGED} scene="home" mode="calm" seed={1} />);
    vi.advanceTimersByTime(100);
    for (let key = 0; key < 3; key++) card.dispatchEvent(new KeyboardEvent("keydown", { key: "a", bubbles: true }));
    card.dispatchEvent(new Event("scroll"));
    card.dispatchEvent(new Event("scroll"));
    vi.advanceTimersByTime(20);
    expect(told("stirred", "scrolled")).toEqual([{ kind: "stirred" }, { kind: "scrolled" }]);
    vi.advanceTimersByTime(STIR_MILLISECONDS);
    rerender(<PetLayer menagerie={STAGED} scene="home" mode="still" seed={1} />);
    vi.advanceTimersByTime(100);
    card.dispatchEvent(new KeyboardEvent("keydown", { key: "b", bubbles: true }));
    card.dispatchEvent(new Event("scroll"));
    vi.advanceTimersByTime(100);
    expect(told("stirred", "scrolled")).toHaveLength(2);
  });
});
