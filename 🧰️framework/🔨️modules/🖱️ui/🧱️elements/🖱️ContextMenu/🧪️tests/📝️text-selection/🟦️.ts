import { describe, expect, it } from "vitest";
import schema from "../../../../🧬️schema/📝️text-selection-menu/🔣️.json" with { type: "json" };
import { buildTextSelectionContextMenuItems } from "../../📝️text-selection/🟦️.ts";
import { uiDataLabel } from "../../../🎗️UiLabel/🟦️.tsx";
import { configurationDe, configurationEn } from "../../../../🌐️i18n/⚙️configuration/🟦️.ts";

const labels = { cut: uiDataLabel("cut"), copy: uiDataLabel("copy"), paste: uiDataLabel("paste"), selectAll: uiDataLabel("selectAll") };
const actions = { cut: () => {}, copy: () => {}, paste: () => {}, selectAll: () => {} };

describe("text-selection context menu rows (shared schema)", () => {
  it("offers every row for an editable selection, in schema order", () => {
    const items = buildTextSelectionContextMenuItems({ editable: true, hasSelection: true }, labels, actions);
    expect(items.map((item) => item.id)).toEqual(schema.rows.map((row) => row.id));
  });
  it("offers only Copy and Select all for a readable target and disables Copy without a selection", () => {
    const items = buildTextSelectionContextMenuItems({ editable: false, hasSelection: false }, labels, actions);
    expect(items.filter((item) => !item.separator).map((item) => item.id)).toEqual(["text-copy", "text-select-all"]);
    expect(items.find((item) => item.id === "text-copy")?.disabled).toBe(true);
    expect(items.find((item) => item.id === "text-select-all")?.disabled).toBeUndefined();
  });
  it("carries the shell's en and de label texts equal to the i18n configuration", () => {
    for (const row of schema.rows) {
      if (!row.labelKey || !row.label) continue;
      const path = row.labelKey.replace(/^ui\./u, "").split(".");
      const read = (tree: unknown): string => path.reduce<any>((node, key) => node[key], tree).label.normal;
      expect(row.label.en).toBe(read(configurationEn));
      expect(row.label.de).toBe(read(configurationDe));
    }
  });
});
