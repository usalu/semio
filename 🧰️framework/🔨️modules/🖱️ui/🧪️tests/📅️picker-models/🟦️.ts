/** 📅️ The picker models in TypeScript: the same vectors the Rust models answer, checked against the browser's own `Date` and `Intl`.
 *
 * 🧫️ Read from `🖱️ui/🧪️tests/🧫️fixtures/📅️picker-models/🔣️.json`.
 */
import { describe, expect, it } from "vitest";
import fixture from "../🧫️fixtures/📅️picker-models/🔣️.json" with { type: "json" };
import { contextMenuAnchorPointV1 } from "../../🧱️elements/🖱️ContextMenu/🔘️trigger/🟦️.tsx";

const day = (text: string) => new Date(`${text}T00:00:00Z`);

describe("📅️ picker models", () => {
  for (const c of fixture.rowMenu.anchor) {
    it(`row menu anchor ${c.row.join(",")}`, () => {
      const [x, y, w, h] = c.row as [number, number, number, number];
      const point = contextMenuAnchorPointV1({ left: x, top: y, right: x + w, bottom: y + h });
      expect(point.x).toBeCloseTo(c.expected[0]!, 3);
      expect(point.y).toBeCloseTo(c.expected[1]!, 3);
    });
  }
  for (const c of fixture.date.iso.filter((entry) => "epochDay" in entry)) {
    it(`date ${c.text}`, () => {
      expect(Math.floor(day(c.text).getTime() / 86_400_000)).toBe(c.epochDay);
      expect((day(c.text).getUTCDay() + 6) % 7).toBe(c.weekdayFromMonday);
    });
  }
  for (const locale of fixture.date.locales) {
    it(`locale ${locale.id} months and display follow Intl`, () => {
      const names = [...Array(12)].map((_, month) => new Intl.DateTimeFormat(locale.intl, { timeZone: "UTC", month: "long" }).format(new Date(Date.UTC(2026, month, 1))));
      expect(names).toEqual(locale.months);
      for (const entry of locale.format) expect(new Intl.DateTimeFormat(locale.intl, { timeZone: "UTC", year: "numeric", month: "2-digit", day: "2-digit" }).format(day(entry.iso))).toBe(entry.text);
    });
  }
  for (const c of fixture.colour.values.filter((entry) => /^#[0-9a-fA-F]{6}$/.test(entry.text))) {
    it(`colour value ${c.text} is the DOM colour value (lower case)`, () => {
      expect(c.text.toLowerCase()).toBe(c.expected);
    });
  }
});
