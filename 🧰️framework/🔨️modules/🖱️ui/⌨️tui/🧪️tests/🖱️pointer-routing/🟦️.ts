/** 🖱️ Independent oracle for the terminal pointer routing table.
 *
 * The Rust engine, the Rust painter and this file share nothing but `🧪️tests/🧫️fixtures/🖱️pointer-routing/🔣️.json`.
 * Here the table is read twice: the golden frame rows must show every declared chip, close glyph and maximize
 * glyph at its declared column (so the declared geometry is what a person sees), and a small independent model of
 * the routing rules re-derives the signals of every `chrome` case from that geometry alone.
 */
import cliTruncate from "cli-truncate";
import stringWidth from "string-width";
import { describe, expect, it } from "vitest";
import fixture from "../🧫️fixtures/🖱️pointer-routing/🔣️.json" with { type: "json" };

type Chip = { window: string; index: number; corner: string; x: number; right: number; drag: number; close: number; maximize: number };
type Window = { window: string; x: number; right: number; textRow: number; active: number };
type Event = { t: string; b?: string; x: number; y: number };

const chips = fixture.chips as Chip[];
const windows = fixture.windows as Window[];
const rows = fixture.frame.rows.map((row) => [...row]);
const firstRow = fixture.frame.first;

const windowAt = (x: number, y: number): Window | undefined => windows.find((w) => x >= w.x && x <= w.right && y >= firstRow && y <= firstRow + 2);

const identities = (window: string): string[] => fixture.scene.layout.stacks.find((stack) => stack.windows.some((tab) => tab.id === window))!.windows.map((tab) => tab.id);

function insertionAt(window: string, source: string, corner: string, x: number): number {
  const remaining = chips.filter((chip) => chip.window === window && chip.corner === corner && identities(window)[chip.index] !== source).sort((a, b) => a.index - b.index);
  const next = remaining.findIndex((chip) => x < chip.x + Math.floor((chip.right - chip.x + 1) / 2));
  return next < 0 ? remaining.length : next;
}

/** 🧭️ Replays one case against the declared geometry: left-only activation, tab indices on every control, tab drags. */
function model(events: Event[]): string[] {
  const signals: string[] = [];
  let focusedWindow: string | undefined;
  let press: { window: string; source: string; x: number; y: number; moved: boolean } | undefined;
  const focus = (window: string) => {
    if (focusedWindow !== window) {
      focusedWindow = window;
      signals.push(`${window}:WindowFocus`);
    }
  };
  const proposal = (event: Event): { source: string; target: string; corner: string; index: number } | undefined => {
    if (!press?.moved) return;
    const window = windowAt(event.x, event.y);
    const chip = chips.find((chip) => chip.window === window?.window && event.x >= chip.x && event.x <= chip.right);
    if (!window || !chip || event.y !== window.textRow) return;
    const ids = identities(window.window);
    const target = ids[window.active] !== press.source ? ids[window.active] : ids.find((id) => id !== press!.source) ?? press.source;
    return { source: press.source, target, corner: chip.corner, index: insertionAt(window.window, press.source, chip.corner, event.x) };
  };
  const preview = (event: Event) => {
    const projected = proposal(event);
    if (projected) signals.push(`${press!.window}:WindowDockPreview(${projected.source}->${projected.target},${projected.corner},${projected.index})`);
    return projected;
  };
  for (const event of events) {
    const window = windowAt(event.x, event.y);
    if (event.t === "down" && window && event.y === window.textRow) {
      const chip = chips.find((c) => c.window === window.window && event.x > c.x && event.x < c.right);
      const button = event.b ?? "left";
      if (chip && event.x === chip.close) {
        if (button === "left") signals.push(`${window.window}:WindowClose(${chip.index})`);
      } else if (chip && event.x === chip.maximize) {
        if (button === "left") signals.push(`${window.window}:WindowMaximize(${chip.index})`);
      } else if (chip) {
        if (button === "left") {
          focus(window.window);
          signals.push(`${window.window}:WindowTabActivated(${chip.index})`);
          press = { window: window.window, source: identities(window.window)[chip.index], x: event.x, y: event.y, moved: false };
        } else if (button === "right") {
          signals.push(`${window.window}:ContextMenu(${event.x},${event.y},${chip.index})`);
        }
      } else if (button === "left" || button === "right") {
        focus(window.window);
      }
    } else if (event.t === "drag" && press && (event.b ?? "left") === "left") {
      press.moved ||= Math.abs(event.x - press.x) + Math.abs(event.y - press.y) >= 2;
      preview(event);
    } else if (event.t === "up" && press && (event.b ?? "left") === "left") {
      const projected = preview(event);
      if (projected) signals.push(`${press.window}:WindowDocked(${projected.source}->${projected.target},${projected.corner},${projected.index})`);
      press = undefined;
    }
  }
  return signals;
}

describe("🖱️ pointer routing table", () => {
  it("declares the contract the table answers to", () => {
    for (const [name, value] of Object.entries(fixture.contract)) {
      expect(value, name).toBe(true);
    }
  });

  it("shows every declared chip, grip, close glyph and maximize glyph where the table says", () => {
    const text = rows[1];
    const hairline = rows[2];
    for (const chip of chips) {
      expect(text[chip.x], `left wall of ${chip.window} chip ${chip.index}`).toBe("│");
      expect(text[chip.right], `right wall of ${chip.window} chip ${chip.index}`).toBe("│");
      expect(chip.drag, "the grip replaces leading padding without shifting controls").toBe(chip.x + 1);
      expect(text[chip.drag], `grip of ${chip.window} chip ${chip.index}`).toBe("⠿");
      expect(text[chip.maximize], `fullscreen glyph of ${chip.window} chip ${chip.index}`).toBe("⤢");
      expect(text[chip.close], `close glyph of ${chip.window} chip ${chip.index}`).toBe("✕");
      expect(text[chip.close - 1], "a space precedes the close glyph").toBe(" ");
    }
    for (const window of windows) {
      const active = chips.find((c) => c.window === window.window && c.index === window.active)!;
      expect(hairline[active.x + 1], "the seam opens under the active tab").toBe(" ");
    }
  });

  it("derives source-excluded corner insertion from the remaining chip midpoints", () => {
    const held = chips.find((chip) => chip.window === "w1" && chip.index === 1)!;
    const source = identities(held.window)[held.index];
    expect(insertionAt(held.window, source, held.corner, 23)).toBe(1);
    expect(insertionAt(held.window, source, held.corner, 35)).toBe(2);
    expect(insertionAt(held.window, source, held.corner, 18)).toBe(1);
    const original = identities(held.window);
    const remaining = original.filter((id) => id !== source);
    const unchanged = [...remaining];
    unchanged.splice(insertionAt(held.window, source, held.corner, 18), 0, source);
    expect(unchanged).toEqual(original);
  });

  it("paints rows exactly as wide as the terminal under an independent width model", () => {
    for (const row of fixture.frame.rows) expect(stringWidth(row), row).toBe(fixture.scene.size[0]);
  });

  it("elides labels the way cli-truncate does, on narrow and wide text alike", () => {
    for (const { text, max, out } of fixture.elision) {
      expect(cliTruncate(text, max, { position: "end" }), `${text} @ ${max}`).toBe(out);
      expect(stringWidth(out)).toBeLessThanOrEqual(max);
    }
  });

  it("keeps the chips of one window from overlapping and inside its walls", () => {
    for (const window of windows) {
      const own = chips.filter((c) => c.window === window.window).sort((a, b) => a.x - b.x);
      for (let i = 1; i < own.length; i += 1) expect(own[i].x).toBeGreaterThan(own[i - 1].right);
      const last = own[own.length - 1];
      expect(last.right).toBeLessThanOrEqual(window.right);
    }
  });

  for (const scenario of fixture.cases) {
    if ((scenario as { model?: string }).model !== "chrome") continue;
    it(`derives the signals of: ${scenario.name}`, () => {
      expect(model(scenario.events as Event[])).toEqual(scenario.signals.filter((signal) => !signal.startsWith("w1.") && !signal.startsWith("w2.")));
    });
  }

  it("lists every non-chrome case with an expectation for signals, focus and hover", () => {
    for (const scenario of fixture.cases) {
      expect(scenario.signals, scenario.name).toBeInstanceOf(Array);
      expect("focus" in scenario && "hovered" in scenario, scenario.name).toBe(true);
    }
    expect(fixture.cases.length).toBeGreaterThanOrEqual(20);
  });
});
