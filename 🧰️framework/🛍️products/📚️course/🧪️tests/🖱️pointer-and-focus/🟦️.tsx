/** 🖱️ The pointer, keyboard focus and drag the client shares (design §16) — measured on the nearest anchor, an item or a card, also while pressed, and over every viewport gap, with `lodash`'s `clamp` as the third-party oracle of the cursor's fraction; the cursor vectors are shared JSON.
 *
 * @see ../🧰️presence-client-kit/🟦️.tsx — the sockets and fixtures
 * @see ../🧫️fixtures/📡️presence-client/🔣️.json
 * @see ../../🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
 */

import { fireEvent, render, screen } from "@testing-library/react";
import clamp from "lodash/clamp";
import { describe, expect, it, vi } from "vitest";
import { cursorProblem, learnerTag } from "@semio-tech/course";
import { CoursePresence, cursorAt, viewportCursor, usePresencePointer } from "@semio-tech/course-react";
import vectors from "../🧫️fixtures/📡️presence-client/🔣️.json";
import { ME, box, pointer } from "../🧰️presence-client-kit/🟦️.tsx";

describe("🖱️ pointer and focus", () => {
  function Harness(props: { readonly presence: CoursePresence }) {
    usePresencePointer(props.presence);
    return (
      <div>
        <section data-presence-anchor="task:u-values">
          <ul>
            <li data-course-item="glazing" data-presence-anchor="item:glazing">
              <span data-course-grip="">⠿</span>
              <button type="button">Single glazing</button>
            </li>
          </ul>
        </section>
        <button type="button">Outside</button>
      </div>
    );
  }

  for (const vector of vectors.cursors) {
    it(`measures a cursor: ${vector.id}`, () => {
      const anchor = document.createElement("div");
      anchor.dataset.presenceAnchor = "home:board";
      box(anchor, vector.box);
      const cursor = cursorAt(anchor, vector.point[0]!, vector.point[1]!);
      expect(cursor ?? null).toEqual(vector.expected === null ? null : { anchor: "home:board", ...vector.expected });
    });
  }

  for (const vector of vectors.viewportCursors) {
    it(`tracks every viewport gap: ${vector.id}`, () => {
      const width = window.innerWidth;
      const height = window.innerHeight;
      Object.defineProperties(window, { innerWidth: { configurable: true, value: vector.viewport[0] }, innerHeight: { configurable: true, value: vector.viewport[1] } });
      try {
        expect(viewportCursor(vector.point[0]!, vector.point[1]!)).toEqual(vector.expected);
        expect([vector.expected.x, vector.expected.y]).toEqual(vector.point.map((value, index) => Math.round(clamp(value / vector.viewport[index]!, 0, 1) * 10_000) / 10_000));
        expect(cursorProblem({ tag: learnerTag(ME), cursor: vector.expected })).toBeUndefined();
      } finally {
        Object.defineProperties(window, { innerWidth: { configurable: true, value: width }, innerHeight: { configurable: true, value: height } });
      }
    });
  }

  it("keeps pointing across captured drags, clears cancel and blur, and never shares observer interactions", () => {
    const presence = { point: vi.fn(), focusOn: vi.fn(), drag: vi.fn() };

    const rendered = render(<Harness presence={presence as unknown as CoursePresence} />);
    const item = document.querySelector('[data-presence-anchor="item:glazing"]')!;
    const outside = screen.getByRole("button", { name: "Outside" });
    box(item, { left: 100, top: 80, width: 200, height: 40 });
    Object.defineProperty(document, "elementFromPoint", { configurable: true, value: vi.fn(() => outside) });
    const grip = item.querySelector("[data-course-grip]")!;
    pointer(grip, "pointerdown", 110, 100, 1);
    pointer(grip, "pointermove", 10, 10, 1);
    expect(presence.point).toHaveBeenLastCalledWith(viewportCursor(10, 10));
    pointer(grip, "pointercancel", 10, 10);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    fireEvent.keyDown(document, { key: "Tab" });
    item.querySelector("button")!.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith("item:glazing");
    window.dispatchEvent(new Event("blur"));
    expect(presence.point).toHaveBeenLastCalledWith(undefined);
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    rendered.unmount();
    function Disabled() { usePresencePointer(presence as unknown as CoursePresence, false); return <button type="button">Observer</button>; }
    render(<Disabled />);
    presence.point.mockClear();
    pointer(screen.getByRole("button", { name: "Observer" }), "pointermove", 500, 300);
    expect(presence.point).not.toHaveBeenCalled();
    Reflect.deleteProperty(document, "elementFromPoint");
  });
  it("names the nearest anchor under the pointer — an item before its card — also while pressed, the item a grip drags until release, and keyboard focus only", () => {
    const presence = { point: vi.fn(), focusOn: vi.fn(), drag: vi.fn() };
    render(<Harness presence={presence as unknown as CoursePresence} />);
    box(document.querySelector('[data-presence-anchor="task:u-values"]')!, { left: 100, top: 50, width: 200, height: 100 });
    const item = document.querySelector('[data-presence-anchor="item:glazing"]')!;
    box(item, { left: 100, top: 80, width: 200, height: 40 });
    const button = screen.getByRole("button", { name: "Single glazing" });
    const outside = screen.getByRole("button", { name: "Outside" });
    const grip = item.querySelector("[data-course-grip]")!;
    pointer(button, "pointermove", 150, 100);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "item:glazing", x: 0.25, y: 0.5 });
    pointer(document.querySelector("section")!, "pointermove", 150, 60);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "task:u-values", x: 0.25, y: 0.1 });
    pointer(button, "pointermove", 160, 100, 1);
    expect(presence.point).toHaveBeenLastCalledWith({ anchor: "item:glazing", x: 0.3, y: 0.5 });
    pointer(outside, "pointermove", 10, 10);
    expect(presence.point).toHaveBeenLastCalledWith(viewportCursor(10, 10));
    pointer(grip, "pointerdown", 110, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith("glazing");
    pointer(grip, "pointerup", 110, 100);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    pointer(grip, "pointerdown", 110, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith("glazing");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    pointer(button, "pointerdown", 150, 100, 1);
    expect(presence.drag).toHaveBeenLastCalledWith(undefined);
    button.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    fireEvent.keyDown(document, { key: "Tab" });
    outside.focus();
    button.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith("item:glazing");
    outside.focus();
    expect(presence.focusOn).toHaveBeenLastCalledWith(undefined);
    for (const call of presence.point.mock.calls) if (call[0] !== undefined) expect(cursorProblem({ tag: learnerTag(ME), cursor: call[0], drag: { item: "glazing" } })).toBeUndefined();
  });
});
