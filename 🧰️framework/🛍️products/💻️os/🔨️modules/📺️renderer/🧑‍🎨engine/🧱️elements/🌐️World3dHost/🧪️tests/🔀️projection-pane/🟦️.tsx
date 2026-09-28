// @vitest-environment jsdom

import Ajv2020 from "ajv/dist/2020.js";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { FlowProvider } from "@semio-tech/ui-react";
import { WorldProjectionKindSwitch, worldProjectionDefaults, worldProjectionTemplateSelectionId } from "@semio-tech/infinite-world-r3f";
import { afterEach, describe, expect, it, vi } from "vitest";
import schema from "../../../../🧬️schema/🔀️projection-pane/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🔀️projection-pane/🔣️.json" with { type: "json" };

afterEach(cleanup);

describe("World projection retained Tree contract", () => {
  it("validates the neutral pane taxonomy", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("mounts the actual React Tree ids, hierarchy, selection, and row activation", () => {
    const onSpecChange = vi.fn();
    const view = render(<FlowProvider inline="rtl" block="up"><WorldProjectionKindSwitch id={fixture.paneId} spec={worldProjectionDefaults("threePoint")} onSpecChange={onSpecChange} /></FlowProvider>);
    expect(view.getByRole(fixture.body.role)).toBeTruthy();
    expect(document.querySelectorAll("[role='treeitem']")).toHaveLength(fixture.rows.length);
    expect(view.getByRole("tree").getAttribute("dir")).toBe(fixture.body.inline);
    expect([...document.querySelectorAll("[role='treeitem']")].map((row) => row.id)).toEqual(fixture.rows.map((row) => row.id));
    for (const row of fixture.rows) {
      const item = document.getElementById(row.id);
      expect(item, row.id).not.toBeNull();
      expect(item?.getAttribute("role")).toBe("treeitem");
      expect(item?.getAttribute("aria-selected")).toBe(String(row.selected));
      expect(item?.querySelector("button")?.hasAttribute("aria-expanded") ?? false).toBe(row.disclosure);
      expect(item?.textContent).toContain(row.label);
    }

    const orthographic = document.getElementById(fixture.rows[1]!.id)!;
    fireEvent.click(orthographic);
    expect(onSpecChange).toHaveBeenCalledTimes(1);
    for (const [spec] of onSpecChange.mock.calls) expect(worldProjectionTemplateSelectionId(spec)).toBe("orthographic");
  });
});
