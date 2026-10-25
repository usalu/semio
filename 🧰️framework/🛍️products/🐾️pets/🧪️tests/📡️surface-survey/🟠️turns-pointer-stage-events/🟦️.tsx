/** 🟠️ The turns pointer stage events tests of the 📡️ surface survey suite. */
import { expect, it } from "vitest";
import type { StageEvent } from "@semio-tech/pets";
import { FLOOR, PET_CONTROLS, PET_PRESS_CONTROLS, surfaceId, survey, wallId, watchPointer } from "@semio-tech/pets-react";
import { card, ledger, place, pointer, viewport } from "../🟣️harness/🟦️.tsx";

/** 🟡 Registers the turns pointer stage events tests of the 📡️ surface survey suite. */
export function registerTurnsPointerStageEventsTests(): void {
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
}
