import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createFrameworkSettingsPanelTab, themeAlphaInput, themeNumberInputRow, themeTextInputRow, type ConflictsHostApi } from "../../🟦️.tsx";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import type { ReactElement } from "react";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("ChromePanels theme inputs", () => {
  it("renders and lazily commits canonical text values", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeTextInputRow("theme.spacing.compact", "compact", "0.25rem", commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25rem");
    fireEvent.change(input, { target: { value: "0.5rem" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith("0.5rem");
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits canonical scalar and list numbers", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const row = themeNumberInputRow("theme.metrics.padding", "padding", [1, 2.5], commit);
    const { getByRole } = render(<>{row.control}</>);
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("1, 2.5");
    fireEvent.change(input, { target: { value: "3, 4.5" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith([3, 4.5]);
    expect(consoleError).not.toHaveBeenCalled();
  });

  it("renders and lazily commits clamped appearance alpha", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const commit = vi.fn();
    const { getByRole } = render(themeAlphaInput("theme.appearance.alpha", 0.25, commit));
    const input = getByRole("textbox") as HTMLInputElement;
    expect(input.value).toBe("0.25");
    fireEvent.change(input, { target: { value: "1.2" } });
    fireEvent.blur(input);
    expect(commit).toHaveBeenCalledWith(1);
    expect(consoleError).not.toHaveBeenCalled();
  });
});

describe("Inline Tree resolution controls", () => {
  it("keeps both labeled resolution buttons available before selecting the conflict", () => {
    const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "..");
    const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🎛️inline-tree-controls", "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🎛️inline-tree-controls", "🔣️.json"), "utf8"));
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const resolve = vi.fn();
    const host: ConflictsHostApi = {
      conflicts: [{ id: fixture.conflict.id, kind: { kind: "quarantined", envelopes: [] }, status: "open", messages: [], actors: [], timestamp: { actor: 1, physical_ms: 0, logical: 0 } }],
      locale: "en",
      selectedConflictId: null,
      kindLabel: () => "Quarantined",
      messageText: () => fixture.conflict.message,
      onSelect: vi.fn(),
      onResolve: resolve,
      currentDocumentText: "",
    };
    const panel = createFrameworkSettingsPanelTab(() => null, undefined, () => host);
    const leaf = panel.children!.find((child) => child.id === "framework.settings.conflicts")!;
    const tree = leaf.trees![0]!.tree.resolveTree!() as { sections: { items: { id: string; control: ReactElement; items?: unknown[] }[] }[] };
    const rows = tree.sections[0]!.items;
    expect(rows).toHaveLength(fixture.expected.rowCount);
    expect(rows[0]!.id).toBe(fixture.rowId);
    expect(rows[0]!.items).toBeUndefined();
    const { container } = render(<>{rows[0]!.control}</>);
    const buttons = Array.from(container.querySelectorAll("button"));
    expect(buttons).toHaveLength(fixture.controls.length);
    for (const [index, expected] of fixture.controls.entries()) {
      const button = buttons[index]!;
      expect(button.id).toBe(`${fixture.rowId}.${expected.suffix}`);
      expect(button.textContent).toContain(expected.label);
      fireEvent.click(button);
      expect(resolve).toHaveBeenLastCalledWith(fixture.conflict.id, expected.suffix);
    }
    expect(container.querySelector(".flex.items-center")?.children).toHaveLength(2);
  });
});
