// @vitest-environment jsdom
/** 🎯️ The React half of the grip-focus law. Tab, Shift+Tab and Ctrl/⌘+Arrow walk the grips of a viewport exactly as the native shell does, and the focused grip is marked for the stylesheet's focus cross.
 *
 * 🧫️ Read from `🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json` — the SAME file the Rust law reads (`🛂️manifest/🧱️app-surface/📱️shell-hooks/🎯️grip-walk/🧪️tests`), so the walk rules cannot drift between the implementations. */
import { describe, expect, it } from "vitest";
import { createGripFocusRegistryV1, gripWalkOfKeyV1, handleGripFocusKeyV1, walkGripsV1, type GripRefV1, type GripStepV1, type GripWalkV1 } from "../../🧱️elements/🏛️ShellHost/🎯️grip-focus/🟦️.ts";
import fixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json" with { type: "json" };

const named: Readonly<Record<string, readonly GripRefV1[]>> = { grips: fixture.grips, single: fixture.single };
const stepOf = (value: unknown): GripStepV1 => (typeof value === "string" ? (value as "leave" | "nothing") : { to: (value as { to: string }).to });

describe("🎯️ grip focus", () => {
  for (const row of fixture.walks) {
    it(`walks: ${row.id}`, () => {
      expect(walkGripsV1(row.grips === null ? [] : named[row.grips]!, row.current, row.how as GripWalkV1)).toEqual(stepOf(row.step));
    });
  }

  it("keeps one focus per domain and drops it when the grip disappears, like the fixture sequence", () => {
    const registry = createGripFocusRegistryV1();
    for (const [index, raw] of fixture.registry.entries()) {
      const op = raw as Readonly<{ op: string; domain: string; grips?: string | readonly GripRefV1[]; how?: string; step?: unknown; expect?: string | null }>;
      const grips = typeof op.grips === "string" ? named[op.grips]! : (op.grips ?? []);
      if (op.op === "note") registry.note(op.domain, grips);
      else if (op.op === "walk") expect(registry.walk(op.domain, op.how as GripWalkV1), `step ${index}`).toEqual(stepOf(op.step));
      else expect(registry.focused(op.domain), `step ${index}`).toBe(op.expect);
    }
  });

  it("maps the keys of the native shell hook: Enter starts, Tab walks, Ctrl/⌘+Arrow cycles, Escape ends", () => {
    const key = (init: Partial<{ key: string; ctrlKey: boolean; metaKey: boolean; altKey: boolean; shiftKey: boolean }> & { key: string }) => ({ ctrlKey: false, metaKey: false, altKey: false, shiftKey: false, ...init });
    expect(gripWalkOfKeyV1(key({ key: "Enter" }), false)).toBe("start");
    expect(gripWalkOfKeyV1(key({ key: "Enter" }), true)).toBeNull();
    expect(gripWalkOfKeyV1(key({ key: "Tab" }), false)).toBeNull();
    expect(gripWalkOfKeyV1(key({ key: "Tab" }), true)).toBe("forward");
    expect(gripWalkOfKeyV1(key({ key: "Tab", shiftKey: true }), true)).toBe("backward");
    for (const [arrow, how] of [["ArrowRight", "forwardInEntity"], ["ArrowDown", "forwardInEntity"], ["ArrowLeft", "backwardInEntity"], ["ArrowUp", "backwardInEntity"]] as const) {
      expect(gripWalkOfKeyV1(key({ key: arrow, ctrlKey: true }), true)).toBe(how);
      expect(gripWalkOfKeyV1(key({ key: arrow, metaKey: true }), true)).toBe(how);
      expect(gripWalkOfKeyV1(key({ key: arrow }), true), "a plain arrow stays a nudge").toBeNull();
      expect(gripWalkOfKeyV1(key({ key: arrow, ctrlKey: true }), false), "no focus, no cycling").toBeNull();
    }
    expect(gripWalkOfKeyV1(key({ key: "Escape" }), true)).toBe("end");
    expect(gripWalkOfKeyV1(key({ key: "Escape" }), false)).toBeNull();
  });

  it("walks the grips a viewport states in the DOM and marks the focused one", () => {
    document.body.innerHTML = `<div data-grip-domain="norm.dom" id="host" tabindex="0">${fixture.grips.map((grip) => `<span data-grip-id="${grip.id}" data-grip-entity="${grip.entity}"></span>`).join("")}</div>`;
    const host = document.getElementById("host")!;
    const registry = createGripFocusRegistryV1();
    const press = (key: string, init: Partial<KeyboardEventInit> = {}) => handleGripFocusKeyV1({ key, ctrlKey: false, metaKey: false, altKey: false, shiftKey: false, ...init, target: host }, registry);
    const marked = () => [...host.querySelectorAll("[data-grip-focused]")].map((element) => (element as HTMLElement).dataset.gripId);
    expect(press("Tab"), "Tab belongs to the shell until a grip has the focus").toBe(false);
    expect(press("Enter")).toBe(true);
    expect(marked()).toEqual(["a.length"]);
    expect(press("ArrowRight", { ctrlKey: true })).toBe(true);
    expect(marked()).toEqual(["a.height"]);
    expect(press("Tab")).toBe(true);
    expect(marked()).toEqual(["a.angle"]);
    for (const _ of [0, 1, 2]) press("Tab");
    expect(marked()).toEqual(["b.height"]);
    expect(press("Tab"), "past the last grip the shell's Tab order continues").toBe(false);
    expect(marked()).toEqual([]);
    expect(press("Enter")).toBe(true);
    expect(press("Escape")).toBe(true);
    expect(marked()).toEqual([]);
    expect(host.querySelector("[aria-current]")).toBeNull();
    expect(handleGripFocusKeyV1({ key: "Enter", ctrlKey: false, metaKey: false, altKey: false, shiftKey: false, target: document.body }, registry), "outside a viewport nothing is the walk's").toBe(false);
  });
});
