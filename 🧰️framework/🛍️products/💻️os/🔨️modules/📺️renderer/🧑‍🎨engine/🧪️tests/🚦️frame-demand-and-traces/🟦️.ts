/**
 * 🚦️ TypeScript twin of `🧪️tests/🚦️frame-demand-and-traces/🦀️.rs`.
 *
 * Re-derives the frame-demand verdict and both `[TRACE]` lines from the SAME oracle
 * (`🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json`) without looking at the Rust.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json");

const ORDER = ["invalidated", "first-frame", "input-queued", "frame-ready", "live-build", "presentation-pending", "surface-resize", "snapshot-retained", "component-close", "cursor-wake", "runtime-pending"] as const;

type DemandRow = { readonly id: string; readonly set: readonly string[]; readonly demanded: boolean; readonly reasons: string; readonly admittedLine: string };
type ActionRow = { readonly id: string; readonly controller: string; readonly action: string; readonly line: string };
type Fixture = { readonly demand: readonly DemandRow[]; readonly actions: readonly ActionRow[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

const reasons = (set: readonly string[]): string => {
  const ordered = ORDER.filter((name) => set.includes(name));
  return ordered.length === 0 ? "none" : ordered.join("+");
};

describe("frame demand and traces", () => {
  for (const row of fixture.demand) {
    it(`${row.id}: demand, reasons and admitted line`, () => {
      expect(row.set.length > 0).toBe(row.demanded);
      expect(reasons(row.set)).toBe(row.reasons);
      expect(`[TRACE] frame build admitted generation=7 reason=${reasons(row.set)}`).toBe(row.admittedLine);
    });
  }
  for (const row of fixture.actions) {
    it(`${row.id}: one input action line per action`, () => {
      expect(`[TRACE] frame input action controller=${row.controller} action=${row.action}`).toBe(row.line);
    });
  }
});
